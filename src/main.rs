#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_sync_db_pools;
#[macro_use] extern crate diesel;
use rocket::serde::json::Json;

mod schema;
mod models;

use diesel::prelude::*;
use diesel::insert_into;

use self::models::Distillery;

#[database("mysql")]
struct LogsDbConn(diesel::MysqlConnection);

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index_diistilleries, get_diistillery, create_distillery])
        .attach(LogsDbConn::fairing())
}

#[get("/distilleries/<id>")]
async fn get_diistillery(conn: LogsDbConn, id: u32) -> Json<Distillery> {
    use schema::distilleries::dsl::*;

    conn.run(|c| {
        let result = distilleries.find(id).first(c).expect("Error Loading");
        Json(result)
    }).await
}

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

#[post("/distilleries")]
async fn create_distillery(conn: LogsDbConn) -> Json<usize> {
    use schema::distilleries::dsl::*;

    conn.run(|c| {
        let result = insert_into(distilleries).values(
            (whisky_type.eq("Scotch"),
            region.eq("test"),
            name.eq("test distillery"),
            name_ja.eq("テスト蒸留所"),
            owner.eq("test owner"),
            owner_ja.eq("テストオーナー"))
        ).execute(c)
        .expect("Failed");

        println!("result is {:?}", result);

        Json(result)
    }).await

}
