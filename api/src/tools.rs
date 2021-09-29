use sqlx::Postgres;
use sqlx::pool::PoolConnection;
use rocket::http::Status;
use rocket::response::{status::{Custom}};
use rocket::serde::json::{Json};
use sqlx::postgres::PgPool;

pub type Res<T> = Result<Json<T>, Custom<String>>;

pub async fn acquire_db(pool: &PgPool) -> Result<PoolConnection<Postgres>, Custom<String>> {
    pool.acquire()
        .await
        .map_err(|_| Custom(Status::InternalServerError, String::from("Error acquiring db pool")))
}
