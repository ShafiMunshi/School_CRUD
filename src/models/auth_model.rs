use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use validator::{Validate, ValidationError};


#[derive(Serialize, Deserialize,Validate, Debug)]
pub struct LoginInput {// these data will comes from our application when a user log in, (flutter)
    #[validate(email)]
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize,Validate, Debug)]
pub struct RegisterInput {// these data will comes from our application when a user sign up,(flutter)
    pub name: String,
    #[validate(email)]/// here we are validating email,  ->https://crates.io/crates/validator
    pub email: String,
    pub password: String,
    pub is_email_verified: bool,
}



#[derive(Serialize, Deserialize, Debug)]// these data will fetch from db, for checking if the user is exists or nor
pub struct UserRecord {
    pub id : Thing,
    pub name: String,
    pub email: String,
    pub password: String,
    pub is_email_verified: bool,
}