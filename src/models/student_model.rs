use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Serialize, Deserialize, Debug)]
pub struct Student {
    pub name: String,
    pub roll: u8,
    pub is_male: bool,
    // pub results: SemesterResult,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SemesterResult {
    pub math: u8,
    pub english: u8,
    pub physics: u8,
    pub chemistry: u8,
}


// just an example to get the username after validating with jwt

#[derive(Serialize, Deserialize, Debug)]
pub struct GetUserEmail{
    pub email: String,
}
