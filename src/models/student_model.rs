use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Student{
    pub name:String,
    pub roll:u8,
    pub is_male: bool,
    pub results: SemesterResult,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct SemesterResult{
    pub math:u8,
    pub english:u8,
    pub physics:u8,
    pub chemistry:u8,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Token{
    pub token : String
}


