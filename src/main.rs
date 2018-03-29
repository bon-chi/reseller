extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate dotenv_codegen;
extern crate gotham;
#[macro_use]
extern crate gotham_derive;
extern crate futures;
extern crate hyper;
extern crate mime;
extern crate reseller;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use gotham::handler::{HandlerFuture, IntoHandlerError};
use gotham::http::response::create_response;
use gotham::router::Router;
use gotham::router::builder::*;
use gotham::state::{FromState, State};

use futures::{future, Future, Stream};
use hyper::{Body, Response, StatusCode};

use self::diesel::prelude::*;
use self::models::*;
use self::reseller::*;

fn main() {
    let addr = dotenv!("ADDR");
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, router())
}

fn router() -> Router {
    build_simple_router(|route| {
        // GET /api/resellers?seller_ids=xxx
        route
            .get("/api/resellers")
            .with_query_string_extractor::<QueryStringExtractor>()
            .to(detect_resellers_handler);
        // GET /api/comments/seller_id?offset=0&limit=10
        // POST /api/comment
        route
            .post("/api/comment")
            // .with_path_extractor::<QueryStringCommentExtractor>()
            .to(post_comment_handler);
    })
}

fn detect_resellers_handler(mut state: State) -> (State, Response) {
    println!("hoge");
    let query_param = QueryStringExtractor::take_from(&mut state);
    // println!("{:?}", query_param.seller_ids);
    let connection = establish_connection();
    let resellers = resellers(&connection, query_param.seller_ids);
    // let resellers = Resellers { seller_ids: query_param.seller_ids };
    // println!("{:?}", resellers);
    let reseller_ids = resellers
        .unwrap_or(Vec::new())
        .into_iter()
        .map(|r| r.seller_id);
    let seller_ids: SellerIds = SellerIds(reseller_ids.collect());
    println!("{:?}", serde_json::to_string(&seller_ids));
    // println!("{:?}", reseller_ids);
    let res = create_response(
        &state,
        StatusCode::Ok,
        Some((
            serde_json::to_vec(&seller_ids).expect("serialized seller_ids"),
            mime::APPLICATION_JSON,
        )),
    );
    (state, res)
}

/// Extracts the elements of the POST request and prints them
fn post_handler(mut state: State) -> Box<HandlerFuture> {
    let f = Body::take_from(&mut state).concat2().then(|full_body| {
        match full_body {
            Ok(valid_body) => {
                println!("{:?}", valid_body);
                // let body = Body::from(valid_body);
                // println!("{:?}", body);
                let comment: Comment =
                    serde_json::from_str(&String::from_utf8(valid_body.to_vec()).unwrap()).unwrap();
                // println!("comment: {:?}", comment);
                // add_comment(comment);
                // let body_content = String::from_utf8(valid_body.to_vec()).unwrap();
                // println!("Body: {}", body_content);
                let res = create_response(&state, StatusCode::Ok, None);
                future::ok((state, res))
            }
            Err(e) => return future::err((state, e.into_handler_error())),
        }
    });

    Box::new(f)
}

#[derive(Serialize, Deserialize, Debug)]
struct Comment {
    comment: String,
    seller_id: String,
    pass: String,
}

fn add_comment(comment: Comment) {
    use schema::{reseller_comments, resellers};
    let conn = establish_connection();
    let reseller = resellers::table
        .filter(resellers::dsl::seller_id.eq(&comment.seller_id))
        .load::<Reseller>(&conn)
        .unwrap();
    if reseller.len() == 0 {
        let new_reseller = create_reseller(&conn, &comment.seller_id, None);
        let _r_comment: ResellerComment = create_reseller_comment(
            &conn,
            new_reseller.id,
            &comment.comment,
            None,
            comment.pass,
        );
        println!("add0");
    } else {
        create_reseller_comment(
            &conn,
            reseller.first().unwrap().id,
            &comment.comment,
            None,
            comment.pass,
        );
        println!("add");
    }
}

fn post_comment_handler(mut state: State) -> Box<HandlerFuture> {
    // let query_param = QueryStringCommentExtractor::take_from(&mut state);
    let connection = establish_connection();
    let f = Body::take_from(&mut state).concat2().then(|full_body| {
        match full_body {
            Ok(valid_body) => {
                // let body_content = String::from_utf8(valid_body.to_vec()).unwrap();
                // println!("Body: {}", body_content);
                let comment: Comment =
                    serde_json::from_str(&String::from_utf8(valid_body.to_vec()).unwrap()).unwrap();
                println!("post comment: {:?}", comment);
                add_comment(comment);
                let res = create_response(&state, StatusCode::Ok, None);
                future::ok((state, res))
            }
            Err(e) => return future::err((state, e.into_handler_error())),
        }
    });
    Box::new(f)
}

#[derive(Deserialize, StateData, StaticResponseExtender)]
struct QueryStringExtractor {
    seller_ids: Vec<String>,
}
#[derive(Serialize, Debug)]
struct Resellers {
    seller_ids: Vec<String>,
}
#[derive(Deserialize, Serialize, Debug)]
struct SellerIds(Vec<String>);

#[derive(Deserialize, StateData, StaticResponseExtender)]
struct QueryStringCommentExtractor {
    seller_id: String,
}

fn db() {
    println!("Hello, world!");
    use self::schema::resellers::dsl::*;
    let connection = establish_connection();
    let results = resellers
        .limit(5)
        .load::<Reseller>(&connection)
        .expect("Error loading resellers");
    println!("Displaying {} resellers", results.len());
    for reseller in results {
        println!("{}", reseller.seller_id);
        println!("-----------\n");
        println!("{:?}", reseller.name);
    }

    use self::schema::reseller_comments::dsl::*;
    let comments = reseller_comments
        .limit(5)
        .load::<ResellerComment>(&connection)
        .expect("Error loading reseller_comments");
    println!("Displaying {} reseller_comments", comments.len());
    for reseller_comment in comments {
        println!("{}", reseller_comment.reseller_id);
        println!("-----------\n");
        println!("{:?}", reseller_comment.comment);
    }

    // let new_reseller = create_reseller(&connection, "a", Some("a"));
    // println!(
    //     "Saved reseller seller_id: {}, name: {:?}",
    //     new_reseller.seller_id,
    //     new_reseller.name
    // );

    // let r_comment: ResellerComment = create_reseller_comment(&connection, 1, "comment");
    // println!(
    //     "Saved comment reseller_id: {}, comment: {}",
    //     r_comment.reseller_id,
    //     r_comment.comment
    // );
}

pub fn say_hello(state: State) -> (State, Response) {
    let res = create_response(
        &state,
        StatusCode::Ok,
        Some((String::from("Hello World!").into_bytes(), mime::TEXT_PLAIN)),
    );

    (state, res)
}
#[cfg(test)]
mod tests {
    use super::*;
    use gotham::test::TestServer;

    #[test]
    fn receive_detect_resellers_hadler_response() {
        add_comment(Comment {
            comment: "comment1".to_string(),
            seller_id: "1".to_string(),
            pass: String::from("password"),
        });
        add_comment(Comment {
            comment: "comment2".to_string(),
            seller_id: "2".to_string(),
            pass: String::from("password"),
        });
        let test_server = TestServer::new(router()).unwrap();
        let response = test_server
            .client()
            .get("http://localhost/api/resellers?seller_ids=1&seller_ids=2")
            .perform()
            .unwrap();

        assert_eq!(response.status(), StatusCode::Ok);

        let body = response.read_body().unwrap();
        let expected_seller_ids = SellerIds(vec!["1".to_string(), "2".to_string()]);
        let expected_body =
            serde_json::to_string(&expected_seller_ids).expect("serialized seller ids");
        assert_eq!(&body[..], expected_body.as_bytes());
    }
    #[test]
    fn receive_post_comment_handler_response() {
        let comment = Comment {
            comment: "test comment".to_string(),
            seller_id: "1".to_string(),
            pass: String::from("password"),
        };
        let test_server = TestServer::new(router()).unwrap();
        let response = test_server
            .client()
            .post(
                "http://localhost/api/comment",
                serde_json::to_string(&comment).unwrap(),
                mime::APPLICATION_JSON,
            )
            .perform()
            .unwrap();

        assert_eq!(response.status(), StatusCode::Ok);
    }
}
