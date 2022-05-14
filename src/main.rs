#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_sync_db_pools;
#[macro_use] extern crate diesel;
use rocket::serde::json::Json;

mod schema;
mod models;

use diesel::prelude::*;

use self::models::Distillery;

#[database("mysql")]
struct LogsDbConn(diesel::MysqlConnection);

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index_diistilleries])
        .attach(LogsDbConn::fairing())
}

// #[get("/distilleries/<id>")]
// fn get_diistillery(id: u32) -> String {
//     println!("{}", id);
//     String::from("a distillery")
// }

#[get("/distilleries")]
async fn index_diistilleries(conn: LogsDbConn) -> Json<Vec<Distillery>> {
    use schema::distilleries::dsl::*;

    conn.run(|c| {
        let result = distilleries
            .load::<Distillery>(c)
            .expect("Error Loading");
        
        Json(result)
    }).await
}
