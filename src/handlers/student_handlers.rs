use std::sync::Arc;

use axum::{extract::State, Extension, Json};
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::{
    models::{
        record_model::Record,
        student_model::{SemesterResult, Student},
    },
    utils::{api_error::ApiResult, app_state::AppState, jwt::Claims},
};

use crate::utils::api_error::Error;

type DB = State<AppState>;

pub async fn get_specifik_students(
    State(db_instance): State<AppState>,
    Extension(claim): Extension<Claims>,
) -> ApiResult<String> {


    println!("from handler function  id is :  {}",claim.id);
    let specifik_student: Option<Record> = db_instance
        .surreal_client
        .select(("school", &claim.id))
        .await
        .map_err(|err| Error::DbGetError(err))?; // if something error happend on surrealDb get request, map_err() will capture the error and return it as out custom defined error
    Ok(format!(" {:#?}", specifik_student))
}

pub async fn get_all_students(State(db_instance): State<AppState>) -> ApiResult<String> {
    let all_student: Vec<Record> = db_instance
        .surreal_client
        .select("school")
        .await
        .map_err(|err| Error::DbGetError(err))?; // if something error happend on surrealDb get request, map_err() will capture the error and return it as out custom defined error
    Ok(format!(" {:#?}", all_student))
}

pub async fn create_student(
    // Extension(db_instance): DB,
    State(db_instance): State<AppState>,
    Json(student): Json<Student>,
) -> ApiResult<String> {
    let create_student: Vec<Record> = db_instance
        .surreal_client
        .create("school")
        .content(student)
        .await
        .map_err(|err| Error::DbPostError(err))?;

    Ok(format!("Adeedd: {:#?}", create_student))
}
