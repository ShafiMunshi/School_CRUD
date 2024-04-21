use std::sync::Arc;

use axum::{extract::{Path, Query}, http::response, Extension, Json};

use serde::Deserialize;
use surrealdb::{engine::remote::ws::Client, Surreal};
use validator::Validate;

use crate::{
    models::auth_model::{ LoginInput, RegisterInput},
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
        Err(_) => return Err(Error::EmailValidationError.into()),// if its not validate we return ""email is not valid"" as response
    };

    // then - check if the email is already exist in database or not
    //TODO: if it's already in database, - return Duplicate Email
    

    let que= db_instence.query("SELECT * FROM user WHERE email = 'habib@gmail.com'").await.expect("unable to query");
    
    println!(" {:?}",que);


    //  bcrypt the password , and set it database,
    let register: Vec<RegisterInput> = db_instence
        .create("user")
        .content(RegisterInput {
            password: encryption::hash_password(register_input.password.to_string()).await, // encrypted the password
            name: register_input.name,
            is_email_verified: false,
            email: register_input.email,
        })
        .await
        .map_err(|err| Error::DbPostError(err))?;// if something error happend on surrealDb post request, map_err() will capture the error and return it as out custom defined error

    Ok(format!("Registerd User: {:?}", register))
}




pub async fn sign_in(db_instence: DB, Json(login_input): Json<LoginInput>) -> ApiResult<String> {
   

    // check if the email is valid or not
    // then return EmailValidationErrror
    match login_input.validate() {
        Ok(_) => (),
        Err(_) => return Err(Error::EmailValidationError.into()),
    };

    // TODO: find the id using loginInput email, get the whole record of that targette user
    // if the the email is not exists, return UnAuthosized Error
    let que= db_instence.query("SELECT * FROM user WHERE email = 'shafi@gmail.com'").await.expect("unable to query");
    // println!(" {:?}",que);

    // after getting the hashed password - verify it.
    let hash_pass: Option<RegisterInput> = db_instence// get a single user record for password verification, 
        .select(("user", "35rlgrciyi13vp7l09mp"))
        .await
        .unwrap();


    let pass = "pass123";

    let hash_pass = if let Some(value) = hash_pass {
        value.password
    } else {
        "".to_string()
    };

    // println!("hased pass from db {}", hash_pass);

    let is_verified = encryption::veriy_password(pass.to_string(), hash_pass).await;// verifing the password if it's matches with logini inputted password

    Ok(format!("Is Password verified? ==  {}",is_verified))// if password verifies , return true, otherwise false
}
























pub async fn email_exists(db_instence: DB, email: String) -> ApiResult<bool> {
    let cmd = format!("SELECT * FROM user WHERE email = '{email}'");

    let result = db_instence
        .query(cmd)
        .bind(("table", "user"))
        .await
        .expect("something went wrong");

    println!(" {:?}", result);
    Ok(true)
}




#[derive(Deserialize)]
struct EmailQuery {
    email: String, 
}

// async fn get_id_by_email(
//     Path(email_query): Path<EmailQuery>,
//     db_instence: DB,// Assuming SurrealDB client managed as Axum extension
// ) -> Result<Json<String>, surrealdb_rs::Error> {

//     let query_str = format!("SELECT id FROM my_table WHERE email = '{}';", email_query.email);
//     let query = Query::new(&query_str);

//     let results: Vec<EmailQuery> = db_instence.query(&query).await.unwrap();

//     // Assuming ID is a string and there's at most one matching record
//     let id_value = results.get(0).and_then(|result| result[0].as_str()).unwrap_or("");

//     Ok(Json(id_value.to_string()))
// }
