use ormx::Table;
use rocket::form::FromForm;
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Table, FromForm, Serialize, Deserialize)]
#[ormx(table = "projects", id = id, insertable)]
pub struct Project {
    #[ormx(column = "id", default)]
    pub id: i32,
    pub name: String,
}
