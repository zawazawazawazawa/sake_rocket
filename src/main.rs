#[macro_use] extern crate rocket;

use rocket_sync_db_pools::{database, diesel};

#[database("mysql")]
struct LogsDbConn(diesel::MysqlConnection);

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index_deistilleries, get_deistillery])
        .attach(LogsDbConn::fairing())
}

#[get("/distilleries/<id>")]
fn get_deistillery(id: u32) -> String {
    println!("{}", id);
    String::from("a destillery")
}

#[get("/distilleries")]
fn index_deistilleries(conn: LogsDbConn) -> String {
    String::from("all destilleries")
}
