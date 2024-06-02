use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use validator::Validate;

/// these data will comes from our application when a user log in, (flutter)
/// 

///
/// # Example
/// ```
/// {
///     "email":"user@gmail.com"
///     "password":"password1234"
/// }
/// ```
#[derive(Deserialize, Validate, Debug)]
pub struct LoginInput {
    // these data will comes from our application when a user log in, (flutter)
    #[validate(email)]
    pub email: String,
    pub password: String,
}

/// this model data will come when a user want to Register a new account.
/// 
/// # Example
/// ```
/// {
///     "name":"User Name",
///     "email":"user@gmail.com",
///     "password":"password1234",
/// }
/// ```
#[derive(Serialize, Deserialize, Validate, Debug)]
pub struct RegisterInput {
    // these data will comes from our application when a user sign up,(flutter)
    pub name: String,
    #[validate(email)]
    // here we are validating email,  ->https://crates.io/crates/validator
    pub email: String,
    pub password: String,
}

/// after sign up , data will store in the database like this, we don't need to use it
/// initialy in the database ['is_email_verified = false'] because user didn't verified his email after register
/// 
/// # Example
/// ```
/// {
///     "name":"User Name",
///     "email":"user@gmail.com",
///     "password":"password1234",   
///     "is_email_verified":false // initialy in the database ['is_email_verified = false'] 
/// }
/// ```

#[derive(Serialize, Deserialize, Validate, Debug)]
pub struct RegisterdDataDb {
    pub name: String,
    #[validate(email)]
    // here we are validating email,  ->https://crates.io/crates/validator
    pub email: String,
    pub password: String,
    pub is_email_verified: bool,
}

/// Fetch all the data of user from the Database record
/// 
#[derive(Serialize, Deserialize, Debug)] // these data will fetch from db, for checking if the user is exists or nor
pub struct UserRecord {
    pub id: Thing,
    pub name: String,
    pub email: String,
    pub password: String,
    pub is_email_verified: bool,
}

// This model is used for return a simple response message to the user, 

/// after success registration
/// we will send a message to user to verify the otp
/// #### Example
/// ```
/// {
///     "status":"success",
///     "message":"an otp has been sent, Please verify your otp"
/// }
/// ```
/// 
/// or in Api error handling or fallback url
/// ## Example
/// ```
/// {
///     "status":"error",
///     "message":"Something went wrong "
/// }
/// 
/// ```

#[derive(Serialize, Deserialize, Debug)]
pub struct UnifiedResponse {
    // this struct is using for show as response, (like Api Error, New verfication sent)
    pub status: String,
    pub message: String,
}

/// to save the generated otp on Database according to userEmail
/// 
/// /// ## Example
/// ``````
/// {
///     "id":"user@gmail.com",
///     "otp":"123456"
/// }
#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd)]
pub struct OTP {
    pub otp: String,
}

/// fetches the otp code from database with id[ which is userEmail]
#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd)]
pub struct OTPRecord {
    pub id: Thing,
    pub otp: String,
}

// if a user tries to verify his otp, he will send like this, 
/// ## Example
/// ```
/// {
///     "email":"user@gmail.com",
///     "otp":"123456"
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd)]
pub struct OTPFromUser {
    pub email: String,
    pub otp: String,
}

/// after otp verification successfull, we set is_email_verified = true
/// 
/// #### Before verification
///```
/// {
///     "name":"User Name",
///     "email":"user@gmail.com",
///     "password":"password1234",
///     "is_email_verified":false
/// }
/// ```
///  
/// #### After verification
///```
/// {
///     "name" : "User Name",
///     "email" : "user@gmail.com",
///     "password" : "password1234",
///     "is_email_verified" : true
/// }
///```
#[derive(Serialize, Deserialize, Debug)]
pub struct EmailVerificationSuccess {
    pub is_email_verified: bool,
}



#[derive(Serialize, Deserialize, Debug)]
pub struct UserEmail {
    pub email: String,
}

/// we will set some extra data when we save the otp, 
/// it will store the current date-time when the user send request for otp,
///  using this last time sent data we can easily verify when we will send the otp for him.
#[derive(Serialize, Deserialize, Debug)]
pub struct OtpDetails {
    pub sent_count: u8,

    pub last_sent_time: NaiveDateTime,
    //it will store the current date-time when the user send request for otp,
    // using this last time sent data we can easily verify when we will send the otp for him.
}

/// after successfull login or register , we will generate a response which will return Jwt token and the token type to the user.
/// 
/// ## Example
/// ```
/// {
///     "access_token" : "asjfoonosf0wor0r255o2t2t4tb2btttknXXXXXXXXXXXXXXXXX",
///     "token_type" : "Bearer"
/// }
/// ```
#[derive(Debug, Serialize)]
pub struct TokenPayload {
    pub access_token: String,
    pub token_type: String,
}
