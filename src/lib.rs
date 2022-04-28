#[macro_use]
extern crate diesel;
extern crate serde;

mod auth;

pub mod models;
pub mod schema;

pub use auth::BasicAuth;
