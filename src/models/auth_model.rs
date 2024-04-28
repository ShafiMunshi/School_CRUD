use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use validator::Validate;

#[derive(Serialize, Deserialize, Validate, Debug)]
pub struct LoginInput {
    // these data will comes from our application when a user log in, (flutter)
    #[validate(email)]
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Validate, Debug)]
pub struct RegisterInput {
    // these data will comes from our application when a user sign up,(flutter)
    pub name: String,
    #[validate(email)]
    /// here we are validating email,  ->https://crates.io/crates/validator
    pub email: String,
    pub password: String,
    pub is_email_verified: bool,
}

#[derive(Serialize, Deserialize, Debug)] // these data will fetch from db, for checking if the user is exists or nor
pub struct UserRecord {
    pub id: Thing,
    pub name: String,
    pub email: String,
    pub password: String,
    pub is_email_verified: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd)] // these data will fetch from db, for checking if the user is exists or nor
pub struct OTP {
    // pub id: String,
    pub otp: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd)] // these data will fetch from db, for checking if the user is exists or nor
pub struct OTPRecord {
    pub id: Thing,
    pub otp: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd)] // these data will fetch from db, for checking if the user is exists or nor
pub struct OTPFromUser {
    pub id: String,
    pub otp: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EmailVerificationSuccess {
    pub is_email_verified: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ID {
    pub id: String,
}

// for checking resend otp counter:
#[derive(Serialize, Deserialize, Debug)]
pub struct OtpDetails {
    // we use Arc<RwLock<Counter>>  operation happen in the same time, it will ocuur as a bug.
    // because we have e risk in some moment two read & write
    pub sent_count: u8,
    
    pub last_sent_time: NaiveDateTime, 
    //it will store the current date-time when the user send request for otp, 
    // using this last time sent data we can easily verify when we will send the otp for him. 
}
