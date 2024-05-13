use std::env;

use dotenv::dotenv;
use lazy_static::lazy_static;

// in this file, we are taking out database sensitive information from .env file. 
// whenever we need any thiing, we can call them lazily and get the desire value

lazy_static! {
    pub static ref DATABASE_URL: String = get_database_url();
    pub static ref USERNAME: String = get_username();
    pub static ref PASSWORD: String = get_password();
    pub static ref NS: String = get_ns();
    pub static ref DB: String = get_db();
    pub static ref TOKEN: String = get_token();
    pub static ref JWT_SECRET: String = get_secret_key();
}

fn get_database_url() ->String{
    dotenv().ok();
    let var= env::var("DATABASE_URL").expect("Unable to find the database url");
    var
}
fn get_username() -> String {
    dotenv().ok();
   let var=  env::var("USERNAME").expect("Unable to find the database username");
   var
}

fn get_password() -> String {
    dotenv().ok();
   let var=  env::var("PASSWORD").expect("Unable to find the password");
   var
}

fn get_ns() -> String {
    dotenv().ok();
   let var=  env::var("NS").expect("Unable to find the database ns");
   var
}

fn get_db() -> String {
    dotenv().ok();
   let var=  env::var("DB").expect("Unable to find the database db");
   var
}


fn get_token() -> String {
    dotenv().ok();
   let var=  env::var("TOKEN").expect("Unable to find the token");
   var
}
fn get_secret_key() -> String {
    dotenv().ok();
   let var=  env::var("JWT_SECRET").expect("Unable to find the secret key");
   var
}
