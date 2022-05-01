#[macro_use]
extern crate diesel;

pub mod models;
pub mod repositories;
pub mod schema;

mod auth;
pub use auth::BasicAuth;
