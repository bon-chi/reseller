use schema::{resellers, reseller_comments};

use chrono::{DateTime, Utc, NaiveDateTime};
use diesel::mysql;

#[derive(Queryable, Debug)]
pub struct Reseller {
    pub id: i32,
    pub seller_id: String,
    pub name: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "resellers"]
pub struct NewReseller<'a> {
    pub seller_id: &'a str,
    pub name: Option<&'a str>,
}

#[derive(Queryable)]
pub struct ResellerComment {
    pub id: i64,
    pub reseller_id: i32,
    pub comment: String,
    pub user_name: Option<String>,
    pub pass: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "reseller_comments"]
pub struct NewResellerComment<'a> {
    pub reseller_id: i32,
    pub comment: &'a str,
    pub user_name: Option<&'a str>,
    pub pass: String,
}
