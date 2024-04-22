use std::sync::Arc;

use axum::{Extension, Json};

use surrealdb::{engine::remote::ws::Client, Surreal};
use validator::Validate;

use crate::{
    models::auth_model::{LoginInput, RegisterInput, UserRecord},
    utils::{
        api_error::{ApiResult, Error},
        encryption,
    },
};

type DB = Extension<Arc<Surreal<Client>>>;

pub async fn sign_up(
    db_instence: DB,
    Json(register_input): Json<RegisterInput>,
) -> ApiResult<String> {
    //check email is valid - we can use regex or validator crate
    // we validate it in our Model
    match register_input.validate() {
        Ok(_) => (),
        Err(_) => return Err(Error::EmailValidationError.into()), // if its not validate we return ""email is not valid"" as response
    };

    // then - check if the email is already exist in database or not
    // if it's already in database, - return DuplicateEmailError

    let user: Option<UserRecord> =
        email_exists(db_instence.clone(), register_input.email.clone()).await;

    match user {
        Some(user_record) => return Err(Error::DuplicateUserEmail.into()), // return - "account is already exist"
        None => (),
    };

    //  bcrypt the password , and set it database,
    let register: Vec<UserRecord> = db_instence
        .create("user")
        .content(RegisterInput {
            password: encryption::hash_password(register_input.password.to_string()).await, // encrypted the password
            name: register_input.name,
            is_email_verified: false,
            email: register_input.email,
        })
        .await
        .map_err(|err| Error::DbPostError(err))?; // if something error happend on surrealDb post request, map_err() will capture the error and return it as out custom defined error

    Ok(format!("Registerd User: {:?}", register))
}

pub async fn sign_in(db_instence: DB, Json(login_input): Json<LoginInput>) -> ApiResult<String> {
    // check if the email is valid or not
    // then return EmailValidationErrror
    match login_input.validate() {
        Ok(_) => (),
        Err(_) => return Err(Error::EmailValidationError.into()),
    };

    // find the id using loginInput email, get the whole record of that targette user
    // if the the email is not exists, return UnAuthosized Error

    let user: Option<UserRecord> =
        email_exists(db_instence.clone(), login_input.email.clone()).await;
    let user_id;
    match user {
        Some(user_record) => user_id= user_record.id.id.to_string(),
        None => return Err(Error::WrongCredentials.into()), // return - "wrong credentials because there are no email in the whole database"
    };

    // after getting the hashed password - verify it.
    let hash_pass: Option<UserRecord> = db_instence // get a single user record for password verification,
        .select(("user", user_id))
        .await
        .unwrap();

    

    let hash_pass = if let Some(value) = hash_pass {
        value.password
    } else {
        "".to_string()
    };

    // println!("hased pass from db {}", hash_pass);

    let is_verified = encryption::veriy_password(login_input.password, hash_pass).await; // verifing the password if it's matches with logini inputted password

    if (is_verified) {
       return  Ok(format!("Authentication Successfull")) ;// authentication successfull
    }else{
        return  Err(Error::WrongCredentials.into())// password is not verified. 
    }
}

// this function will return Option<UserRecord>
pub async fn email_exists(db_instence: DB, email: String) -> Option<UserRecord> {
    let sql_cmnd = format!("SELECT * FROM user WHERE email = '{}'", email);
    let mut repnse = db_instence
        .query(sql_cmnd)
        .bind(("user", "email"))
        .await
        .expect("unable to query");

    let user: Option<UserRecord> = repnse
        .take(0)
        .expect("something went wrong to get data form data table");
    user
}
