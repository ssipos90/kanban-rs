extern crate dotenv;
extern crate rocket;
mod tools;
mod models;
mod projects;

use dotenv::dotenv;
use sqlx::postgres::PgPool;

pub const PAGE_SIZE: u32 = 12;

#[rocket::launch]
async fn rocket() -> _ {
    dotenv().ok();
    let pool = PgPool::connect(&dotenv::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    rocket::build()
        .manage::<PgPool>(pool)
        .mount("/projects", projects::routes())
}

