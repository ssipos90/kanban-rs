use crate::{models::User, tools::{acquire_db, Res}};
use rocket::{
    http::Status,
    response::status::Custom,
    serde::{json::Json, Deserialize, Serialize},
    Route,
};
use sqlx::PgPool;

#[derive(Deserialize)]
struct LoginCredentials<'r> {
    email: &'r str,
    password: &'r str,
}

#[derive(Serialize)]
struct AuthPayload {
    token: String,
}

#[rocket::post("/", format = "application/json", data = "<credentials>")]
async fn do_login<'r>(
    pool: &rocket::State<PgPool>,
    credentials: Json<LoginCredentials<'r>>
) -> Res<AuthPayload> {
    let mut db = acquire_db(pool).await?;

    let user = ormx::conditional_query_as!(
        User,
        "SELECT *"
        "FROM users"
        "WHERE email="?(credentials.email)
        "AND password_hash IS NOT NULL"
    )
        .fetch_one(&mut *db)
        .await
        .map_err(|_| {
            Custom(
                Status::Unauthorized,
                String::from("User or password is incorrect."),
            )
        })?;

    if user.password_hash.unwrap().eq(credentials.password) {
        return Err(Custom(
            Status::Unauthorized,
            String::from("User or password is incorrect."),
        ));
    }

    // TODO check password lol
    // TODO generate json token
    let mut token = user.id.to_string();
    token.push(':');
    token.push_str(&user.email);

    Ok(Json(AuthPayload { token }))
}

pub fn routes() -> Vec<Route> {
    rocket::routes![do_login]
}
