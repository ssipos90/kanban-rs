use ormx::Table;
use rocket::form::FromForm;
use rocket::serde::Serialize;
use sqlx::types::time::Date;

#[derive(Debug, Table, FromForm, Serialize)]
#[ormx(table = "projects", id = id, insertable)]
pub struct Project {
    #[ormx(column = "id", default)]
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Table)]
#[ormx(table = "users", id = id, insertable)]
pub struct User {
    #[ormx(column = "id", default)]
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password_hash: Option<String>,
    #[ormx(default, set)]
    pub created_at: Date,
}

#[derive(Debug, Table, Serialize)]
#[ormx(table = "users", id = id)]
pub struct ListUser {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Table)]
#[ormx(table = "project_users", id = id, insertable)]
pub struct ProjectUser {
    #[ormx(column = "id", default)]
    pub id: i32,
    pub project_id: i32,
    pub user_id: i32,
    #[ormx(default, set)]
    pub added_at: Date,
}
