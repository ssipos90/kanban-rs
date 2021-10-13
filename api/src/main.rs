extern crate dotenv;
extern crate rocket;

use dotenv::dotenv;
use sqlx::postgres::PgPool;

mod auth;
mod models;
mod projects;
mod tools;
mod users;

#[rocket::launch]
async fn rocket() -> _ {
    dotenv().ok();
    let pool = PgPool::connect(&dotenv::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    rocket::build()
        .manage::<PgPool>(pool)
        .mount("/projects", projects::routes())
        .mount("/users", users::routes())
        .mount("/auth", auth::routes())
}

