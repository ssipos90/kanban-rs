use crate::{
    models::{InsertUser, ListUser},
    tools::{acquire_db, Res, PAGE_SIZE}
};
use ormx::{Insert};
use sqlx;
use rocket::{
    http::Status,
    response::status::Custom,
    serde::{json::Json, Deserialize},
    FromForm, Route,
};
use sqlx::PgPool;

#[derive(FromForm)]
struct UserListFilters<'r> {
    name: Option<&'r str>,
    email: Option<&'r str>,
}

#[rocket::get("/?<page>&<filters..>")]
async fn list_users<'r>(
    pool: &rocket::State<PgPool>,
    filters: UserListFilters<'r>,
    page: Option<u32>,
) -> Res<Vec<ListUser>> {
    let mut db = acquire_db(pool).await?;

    let skip: u32 = match page {
        None | Some(0) => 0,
        Some(page) => (page - 1) * PAGE_SIZE,
    };
   
    ormx::conditional_query_as!(
        ListUser,
        "SELECT id, name, email"
        "FROM users"
        "WHERE 1=1"
        Some(name) = filters.name => {
          "AND name LIKE "?(format!("%{}%", name))
        }
        Some(email) = filters.email => {
          "AND email LIKE "?(format!("%{}%", email))
        }
        "ORDER BY name"
        "LIMIT" ?(PAGE_SIZE as i64)
        "OFFSET" ?(skip as i64)
    )
        .fetch_all(&mut *db)
        .await
        .map(Json)
        .map_err(|_| {
            Custom(
                Status::InternalServerError,
                String::from("Failed loading users."),
            )
        })
}

#[derive(Deserialize)]
pub struct CreateUser<'r> {
    name: &'r str,
    email: &'r str,
}

#[rocket::post("/", format = "application/json", data = "<input>")]
async fn create_user<'r>(
    pool: &rocket::State<PgPool>,
    input: Json<CreateUser<'r>>,
) -> Res<ListUser> {
    let mut db = acquire_db(pool).await?;

    InsertUser {
        name: input.name.to_string(),
        email: input.email.to_string(),
        password_hash: None
    }
    .insert(&mut *db)
    .await
    .map(|user| Json(ListUser {
        id: user.id,
        name: user.name,
        email: user.email,
    }))
    .map_err(|_| Custom(Status::InternalServerError, String::from("Error inserting")))
}

pub fn routes() -> Vec<Route> {
    rocket::routes![list_users, create_user]
}
