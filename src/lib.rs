#[macro_use]
extern crate diesel;
extern crate serde;

pub mod models;
pub mod schema;
pub mod repositories;

mod auth;
pub use auth::BasicAuth;
