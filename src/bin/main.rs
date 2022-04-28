#[macro_use]
extern crate rocket;

use diesel::prelude::*;
use rocket_app::models::*;
use rocket_app::schema::*;
use rocket_app::BasicAuth;

use rocket::{
    response::status,
    serde::json::{json, Json, Value},
};
use rocket_sync_db_pools::database;

#[database("mysql_logs")]
struct LogsDbConn(diesel::MysqlConnection);

#[get("/rustaceans")]
async fn get_rustaceans(_auth: BasicAuth, conn: LogsDbConn) -> Value {
    conn.run(|c| {
        let all = rustaceans::table
            .limit(100)
            .load::<Rustacean>(c)
            .expect("Error loading rustaceans from DB");
        json!(all)
    })
    .await
}

#[get("/rustaceans/<id>")]
async fn view_rustacean(id: i32, _auth: BasicAuth, conn: LogsDbConn) -> Value {
    conn.run(move |c| {
        let rustacean = rustaceans::table
            .find(id)
            .get_result::<Rustacean>(c)
            .expect("Error loading rustacean from DB");
        json!(rustacean)
    })
    .await
}

#[post("/rustaceans", format = "json", data = "<new_rustacean>")]
async fn create_rustacean(
    _auth: BasicAuth,
    conn: LogsDbConn,
    new_rustacean: Json<NewRustacean>,
) -> Value {
    conn.run(|c| {
        let result = diesel::insert_into(rustaceans::table)
            .values(new_rustacean.into_inner())
            .execute(c)
            .expect("Error adding rustaceans to DB");
        json!(result)
    })
    .await
}

#[put("/rustaceans/<id>", format = "json", data = "<rustacean>")]
async fn update_rustacean(
    id: i32,
    _auth: BasicAuth,
    conn: LogsDbConn,
    rustacean: Json<Rustacean>,
) -> Value {
    conn.run(move |c| {
        let result = diesel::update(rustaceans::table.find(id))
            .set((
                rustaceans::name.eq(rustacean.name.to_owned()),
                rustaceans::email.eq(rustacean.email.to_owned()),
            ))
            .execute(c)
            .expect("Error updating rustacean in DB");
        json!(result)
    })
    .await
}

#[delete("/rustaceans/<id>")]
async fn delete_rustacean(id: i32, _auth: BasicAuth, conn: LogsDbConn) -> status::NoContent {
    conn.run(move |c| {
        diesel::delete(rustaceans::table.find(id))
            .execute(c)
            .expect("Error deleting rustacean from DB");
        status::NoContent
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

#[rocket::main]
async fn main() {
    let _ = rocket::build()
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
        .attach(LogsDbConn::fairing())
        .launch()
        .await;
}
