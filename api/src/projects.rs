use crate::{
    models::{InsertProject, Project},
    tools::{acquire_db, Res, PAGE_SIZE}
};
use ormx::{Insert, Table};
use rocket::{
    http::Status,
    response::status::Custom,
    serde::{json::Json, Deserialize},
    FromForm, Route,
};
use sqlx::PgPool;

#[derive(FromForm)]
struct ProjectListFilters<'r> {
    name: Option<&'r str>,
}

#[rocket::get("/?<page>&<filters..>")]
async fn list_projects<'r>(
    pool: &rocket::State<PgPool>,
    filters: ProjectListFilters<'r>,
    page: Option<u32>,
) -> Res<Vec<Project>> {
    let mut db = acquire_db(pool).await?;

    let skip: u32 = match page {
        None | Some(0) => 0,
        Some(page) => (page - 1) * PAGE_SIZE,
    };

    ormx::conditional_query_as!(
        Project,
        "SELECT id, name"
        "FROM projects"
        "WHERE 1=1"
        Some(name) = filters.name => {
          "AND name LIKE "?(format!("%{}%", name))
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
            String::from("Failed loading projects."),
        )
    })
}

#[derive(Deserialize)]
pub struct CreateProject<'r> {
    name: &'r str,
}

#[rocket::post("/", format = "application/json", data = "<input>")]
async fn create_project<'r>(
    pool: &rocket::State<PgPool>,
    input: Json<CreateProject<'r>>,
) -> Res<Project> {
    let mut db = acquire_db(pool).await?;

    InsertProject {
        name: input.name.to_string(),
    }
    .insert(&mut *db)
    .await
    .map(Json)
    .map_err(|_| Custom(Status::InternalServerError, String::from("Error inserting")))
}

#[derive(Deserialize)]
struct UpdateProjectBody<'r> {
    name: &'r str,
}

#[rocket::patch("/<project_id>", format = "application/json", data = "<input>")]
async fn update_project<'r>(
    pool: &rocket::State<PgPool>,
    project_id: i32,
    input: Json<UpdateProjectBody<'r>>,
) -> Res<Project> {
    let mut db = acquire_db(pool).await?;

    let mut project = Project::get(&mut *db, project_id)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => Custom(
                Status::NotFound,
                format!("project_id {} not found", project_id),
            ),
            _ => Custom(
                Status::InternalServerError,
                String::from("Error fetching from database."),
            ),
        })?;

    project.name = input.name.to_string();
    project
        .update(&mut *db)
        .await
        .map(|_| Json(project))
        .map_err(|_| Custom(Status::InternalServerError, String::from("Error inserting")))
}

pub fn routes() -> Vec<Route> {
    rocket::routes![list_projects, create_project, update_project]
}

