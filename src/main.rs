#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_sync_db_pools;
#[macro_use] extern crate diesel;

mod schema;
mod models;
use diesel::prelude::*;

use self::models::*;

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
async fn index_deistilleries(conn: LogsDbConn) -> String {
    use schema::posts::dsl::*;

    conn.run(|c| {
        let result = posts.filter(published.eq(true))
            .limit(5)
            .load::<Post>(c)
            .expect("Error Loading");
        println!("result is {:#?}", result);
        String::from("all destilleries")
    }).await
}
