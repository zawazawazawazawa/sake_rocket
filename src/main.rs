#[macro_use] extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index_deistilleries, get_deistillery])
}

#[get("/distilleries/<id>")]
fn get_deistillery(id: u32) -> String {
    println!("{}", id);
    String::from("a destillery")
}

#[get("/distilleries")]
fn index_deistilleries() -> String {
    String::from("all destilleries")
}
