use std::sync::Arc;

use axum::{Extension, Json};
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::{
    models::{
        record_model::Record,
        student_model::{SemesterResult, Student},
    },
    utils::api_error::ApiResult,
};

use crate::utils::api_error::Error;

type DB = Extension<Arc<Surreal<Client>>>;

pub async fn get_all_students(db_instance: DB) -> ApiResult<String> {
    let all_student: Vec<Record> = db_instance
        .select("school")
        .await
        .map_err(|err| Error::DbGetError(err))?;// if something error happend on surrealDb get request, map_err() will capture the error and return it as out custom defined error
    Ok(format!(" {:#?}", all_student))


}

pub async fn create_student(
    Extension(db_instance): DB,
    Json(student): Json<Student>,
) -> ApiResult<String> {
    let create_student: Vec<Record> = db_instance
        .create("school")
        .content(student)
        .await
        .map_err(|err| Error::DbPostError(err))?;

    Ok(format!("Adeedd: {:#?}", create_student))
}

