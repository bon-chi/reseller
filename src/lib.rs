#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate chrono;
extern crate serde;
extern crate serde_json;
extern crate bcrypt;

use diesel::prelude::*;
use dotenv::dotenv;
use bcrypt::{DEFAULT_COST, hash, verify};
use std::env;

use self::models::{Reseller, NewReseller, ResellerComment, NewResellerComment};

pub mod models;
pub mod schema;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url).expect(&format!(
        "Error connecting to {}",
        database_url
    ))
}

pub fn create_reseller<'a>(
    conn: &MysqlConnection,
    seller_id: &'a str,
    name: Option<&'a str>,
) -> Reseller {
    use schema::resellers;
    let new_reseller = NewReseller { seller_id, name };
    diesel::insert_into(resellers::table)
        .values(&new_reseller)
        .execute(conn)
        .expect("Error saving new reseller");
    resellers::table
        .order(resellers::id.desc())
        .first(conn)
        .unwrap()
}

pub fn create_reseller_comment<'a>(
    conn: &MysqlConnection,
    reseller_id: i32,
    comment: &'a str,
    user_name: Option<&'a str>,
    pass: String,
) -> ResellerComment {
    use schema::reseller_comments;
    let new_reseller_comment = NewResellerComment {
        reseller_id,
        comment,
        pass: hash(&pass.as_str(), DEFAULT_COST).unwrap(),
        user_name,
    };
    diesel::insert_into(reseller_comments::table)
        .values(&new_reseller_comment)
        .execute(conn)
        .expect("Error saveing new reseller comment");
    reseller_comments::table
        .order(reseller_comments::id.desc())
        .first(conn)
        .unwrap()
}

pub fn resellers(
    conn: &MysqlConnection,
    seller_ids: Vec<String>,
) -> Result<Vec<Reseller>, diesel::result::Error> {
    use schema::resellers;
    resellers::table
        .filter(resellers::dsl::seller_id.eq_any(seller_ids))
        .order(resellers::id.asc())
        .load::<Reseller>(conn)
}

pub fn commented_resellers(
    conn: &MysqlConnection,
    seller_ids: Vec<String>,
) -> Result<Vec<Reseller>, diesel::result::Error> {
    use schema::{resellers, reseller_comments};
    use schema::resellers::columns::*;
    use schema::reseller_comments::dsl::*;
    resellers::table
        .inner_join(reseller_comments)
        .filter(resellers::dsl::seller_id.eq_any(seller_ids))
        .order(resellers::id.asc())
        .select((
            resellers::id,
            seller_id,
            name,
            resellers::created_at,
            resellers::updated_at,
        ))
        .load::<Reseller>(conn)
}
