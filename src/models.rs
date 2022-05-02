use super::schema::rustaceans;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Queryable, AsChangeset)]
pub struct Rustacean {
    pub id: i32,
    pub name: String,
    pub email: String,
    // #[serde(skip_deserializing)]
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[table_name = "rustaceans"]
pub struct NewRustacean {
    pub name: String,
    pub email: String,
}

#[derive(Deserialize)]
pub struct UpdatedRustacean {
    pub id: i32,
    pub name: String,
    pub email: String,
}
