#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel_migrations;

use rocket::fairing::AdHoc;
use rocket::http::Status;
use rocket::Build;
use rocket_app::models::*;
use rocket_app::repositories::*;
use rocket_app::BasicAuth;

use rocket::{
    response::status,
    serde::json::{json, Json, Value},
};
use rocket_sync_db_pools::database;

embed_migrations!();

#[database("mysql_db")]
struct DbConnection(diesel::MysqlConnection);

#[get("/rustaceans")]
async fn get_rustaceans(
    _auth: BasicAuth,
    conn: DbConnection,
) -> Result<Value, status::Custom<Value>> {
    conn.run(|c| {
        RustaceanRepository::load_all(c)
            .map(|rustacean| json!(rustacean))
            .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

#[get("/rustaceans/<id>")]
async fn view_rustacean(
    id: i32,
    _auth: BasicAuth,
    conn: DbConnection,
) -> Result<Value, status::Custom<Value>> {
    conn.run(move |c| {
        RustaceanRepository::find(c, id)
            .map(|rustacean| json!(rustacean))
            .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

#[post("/rustaceans", format = "json", data = "<new_rustacean>")]
async fn create_rustacean(
    _auth: BasicAuth,
    conn: DbConnection,
    new_rustacean: Json<NewRustacean>,
) -> Result<Value, status::Custom<Value>> {
    conn.run(|c| {
        RustaceanRepository::create(c, new_rustacean.into_inner())
            .map(|rustacean| json!(rustacean))
            .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

#[put("/rustaceans/<_id>", format = "json", data = "<rustacean>")]
async fn update_rustacean(
    _id: i32,
    _auth: BasicAuth,
    conn: DbConnection,
    rustacean: Json<UpdatedRustacean>,
) -> Result<Value, status::Custom<Value>> {
    conn.run(move |c| {
        RustaceanRepository::update(c, rustacean.into_inner())
            .map(|rustacean| json!(rustacean))
            .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

#[delete("/rustaceans/<id>")]
async fn delete_rustacean(
    id: i32,
    _auth: BasicAuth,
    conn: DbConnection,
) -> Result<status::NoContent, status::Custom<Value>> {
    conn.run(move |c| {
        RustaceanRepository::delete(c, id)
            .map(|_| status::NoContent)
            .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

#[catch(404)]
fn not_found() -> Value {
    json!("Not Found!")
}

#[catch(422)]
fn unprocessable_entity() -> Value {
    json!("Unprocessable Entity!")
}

async fn run_db_migrations(
    rocket: rocket::Rocket<Build>,
) -> Result<rocket::Rocket<Build>, rocket::Rocket<Build>> {
    DbConnection::get_one(&rocket)
        .await
        .expect("Failed to retrieve database connection")
        .run(|c| match embedded_migrations::run(c) {
            Ok(()) => Ok(rocket),
            Err(e) => {
                println!("Failed to run database migrations: {:?}", e);
                Err(rocket)
            }
        })
        .await
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    rocket::build()
        .mount(
            "/",
            routes![
                get_rustaceans,
                view_rustacean,
                create_rustacean,
                update_rustacean,
                delete_rustacean
            ],
        )
        .register("/", catchers![not_found, unprocessable_entity])
        .attach(DbConnection::fairing())
        .attach(AdHoc::try_on_ignite(
            "Database Migrations",
            run_db_migrations,
        ))
        .ignite()
        .await?
        .launch()
        .await
}
