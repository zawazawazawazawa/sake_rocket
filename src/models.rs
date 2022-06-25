use rocket::serde::Serialize;

#[derive(Queryable, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Distillery {
    pub id: i32,
    pub whisky_type: String,
    pub region: Option<String>,
    pub name: String,
    pub name_ja: String,
    pub owner: String,
    pub owner_ja: String,
}
