use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use tracing_subscriber::field::debug;

use super::student_model::SemesterResult;

#[derive(Debug, Deserialize, Serialize)]
pub struct Record{
    pub id:Thing,
    pub name:String,
    pub roll:u8,
    pub is_male: bool,
    // pub hello: String,
    pub results: SemesterResult,
}