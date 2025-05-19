use serde::{Serialize, Deserialize};
use diesel::{prelude::Insertable, Queryable, AsChangeset};
use chrono::NaiveDateTime;
use crate::schema::rustaceans;

#[derive(Serialize, Deserialize, Queryable, AsChangeset)]
pub struct Rustacean {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub name: String,
    pub email: String,
    #[serde(skip_deserializing)]
    pub created_at: NaiveDateTime,
}


#[derive(Deserialize, Insertable)]
#[diesel(table_name = rustaceans)]
pub struct NewRustacean {
    pub name: String,
    pub email: String,
}

