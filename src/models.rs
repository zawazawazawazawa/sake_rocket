use rocket::serde::Serialize;

#[derive(Queryable, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}
