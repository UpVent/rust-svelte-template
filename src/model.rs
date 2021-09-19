use diesel;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

#[derive(Queryable)]
pub struct Article {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub published: bool,
}
