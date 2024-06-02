use axum::{extract::State, Extension, Json};
use chrono::Local;

use std::sync::Arc;
use surrealdb::{engine::remote::ws::Client, Surreal};
use validator::Validate;

use crate::{
    models::{
        auth_model::{
            EmailVerificationSuccess, LoginInput, OTPFromUser, OTPRecord, OtpDetails,
            RegisterInput, RegisterdDataDb, TokenPayload, UnifiedResponse, UserEmail, UserRecord,
            OTP,
        },
        student_model::{GetUserEmail, Student},
    },
    services::verification_code::{email_sent, generate_otp},
    utils::{
        api_error::{ApiResult, Error},
        app_state::AppState,
        encryption, jwt,
    },
};

// type DB = Extension<Arc<Surreal<Client>>>;
type DB = State<AppState>;

/// This function called when a user register for a account
pub async fn sign_up(
    State(db_instence): DB,
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
        email_exists(State(db_instence.clone()), register_input.email.clone()).await; // TODO: can be bugged to connect database

    match user {
        Some(user_record) => return Err(Error::DuplicateUserEmail.into()), // return - "account is already exist"
        None => (),
    };

    //  bcrypt the password , and set it database,
    let register: Vec<UserRecord> = db_instence
        .surreal_client
        .create("user")
        .content(RegisterdDataDb {
            password: encryption::hash_password(register_input.password.to_string()).await, // encrypted the password
            name: register_input.name.clone(),
            is_email_verified: false,
            email: register_input.email.clone(),
        })
        .await
        .map_err(|err| Error::DbPostError(err))?; // if something error happend on surrealDb post request, map_err() will capture the error and return it as out custom defined error

    let store_some_data_on_student_record: Option<Student> = db_instence
        .surreal_client
        .create(("school",register[0].id.id.to_string().clone()))
        .content(Student{
            name: register_input.name,
            is_male:true,
            roll:99
        })
        .await
        .map_err(|err| Error::DbPostError(err))?;

    let user_email = register[0].email.clone().to_string();

    // generate a otp and send the otp as verification code to registered user email
    let otp = generate_otp();

    // save the current otp on database, so that we could verify the otp in /otp_verify (route)

    // save the generated otp in the database
    let save_otp: Option<OTP> = db_instence
        .surreal_client
        .create(("otp", user_email.clone()))
        .content(OTP { otp: otp.clone() })
        .await
        .map_err(|err| Error::DbPostError(err))?;

    // save some extra creadential for checking further,( otp sending limit verification)
    let save_otp_details: Option<OtpDetails> = db_instence
        .surreal_client
        .update(("otp", user_email.clone()))
        .merge(OtpDetails {
            sent_count: 1,                              // initially setted the 1 time sent
            last_sent_time: Local::now().naive_local(), // storing the time when the otp has been sent
        })
        .await
        .map_err(|err| Error::DbPostError(err))?;

    // sent the otp to the registerd userEmail
    email_sent(register_input.email, otp);

    // Ok(format!("{:#?}",UnifiedResponse{
    //     status:"Success".to_string(),
    //     message:"Please verify your otp".to_string()
    // }));

    // serialize all data intoj json to return as response
    return Ok(serde_json::to_string(&UnifiedResponse {
        status: "Success".to_string(),
        message: "Please verify your otp".to_string(),
    })
    .expect("unable to serialize"));
}

/// This handler function is called when a user sign in
pub async fn sign_in(
    State(db_instence): DB,
    Json(login_input): Json<LoginInput>,
) -> ApiResult<String> {
    // check if the email is valid or not
    // then return EmailValidationErrror
    match login_input.validate() {
        Ok(_) => (),
        Err(_) => return Err(Error::EmailValidationError.into()), // if email is not valid
    };

    // find the id using loginInput email, get the whole record of that targette user
    // if the the email is not exists, return UnAuthosized Error

    let user: Option<UserRecord> =
        email_exists(State(db_instence.clone()), login_input.email.clone()).await;
    let user_id;
    match user {
        Some(user_record) => user_id = user_record.id.id.to_string(),
        None => return Err(Error::WrongCredentials.into()), // return - "wrong credentials because there are no email in the whole database"
    };

    // after getting the hashed password - verify it.
    let user_record: Option<UserRecord> = db_instence
        .surreal_client // get a single user record for password verification,
        .select(("user", user_id.clone()))
        .await
        .map_err(|err| Error::DbPostError(err))?;

    let hash_pass = if let Some(value) = &user_record {
        value.password.clone()
    } else {
        "".to_string()
    };

    // println!("hased pass from db {}", hash_pass);

    let is_verified = encryption::veriy_password(login_input.password, hash_pass).await; // verifing the password if it's matches with logini inputted password

    if is_verified {
        // Case: what if a user registered his record but did't verify his email. in that case, we have verify his email again, then we will give him access to log in
        match user_record {
            Some(user_rec) => {
                if user_rec.is_email_verified == false {
                    println!("commiting false");
                    // give him a alert message to verify his email
                    return Err(Error::EmailVerificationError)?;
                }
            }
            None => (),
        }

        // sign a new jwt token then pass to user every time he wants to login
        // if verfication is comlete then generate a jwt token to the the user:
        let token = jwt::sign(user_id);

        // sent the jwt token as response
        println!(" JWT Token: {:?}", token);
        match token {
            // Ok(jwt_token) => {
            //     return Ok(format!(
            //         "{:#?}",
            //         TokenPayload {
            //             access_token: jwt_token,
            //             token_type: "Bearer".to_string()
            //         }
            //     ))
            // }
            // Err(_) => return Err(Error::SomethingWentWrog.into()),
            Ok(jwt_token) => Ok(serde_json::to_string(&TokenPayload {
                access_token: jwt_token,
                token_type: "Bearer".to_string(),
            })
            .expect("unable to serialize")),
            Err(_) => return Err(Error::SomethingWentWrog.into()),
        }
    } else {
        return Err(Error::WrongCredentials.into()); // password is not verified.
    }
}

// this function will return Option<UserRecord>

async fn email_exists(State(db_instence): DB, email: String) -> Option<UserRecord> {
    let sql_cmnd = format!("SELECT * FROM user WHERE email = '{}'", email);
    let mut repnse = db_instence
        .surreal_client
        .query(sql_cmnd)
        .bind(("user", "email"))
        .await
        .expect("unable to query");

    let user: Option<UserRecord> = repnse
        .take(0)
        .expect("something went wrong to get data form data table");
    user
}

/// this function is used for otp verification
pub async fn otp_verification(
    State(db_instence): DB,
    Json(otp_code_from_user): Json<OTPFromUser>,
) -> ApiResult<String> {
    let otp_from_db: Option<OTPRecord> = db_instence
        .surreal_client
        .select(("otp", otp_code_from_user.email.clone()))
        .await
        .map_err(|err| Error::DbPostError(err))?;

    println!(" Otp from db{:#?}", otp_from_db);

    let mut is_verified = false;

    if let Some(db_otp) = otp_from_db {
        if db_otp.otp == otp_code_from_user.otp {
            println!("{} {}", db_otp.otp, otp_code_from_user.otp);
            println!("Otp is matched");
            is_verified = true;
        }
    }

    if is_verified {
        // if otp is verified, then we should change user (is_email_verified = true)
        // first get the user id from userRecord using his email
        let user_record =
            email_exists(State(db_instence.clone()), otp_code_from_user.email.clone())
                .await
                .expect("error found to get user_record");

        // set to user record (is_email_verified = true)

        let user_id = user_record.id.id.to_string();
        println!(" User id is: {}", user_id);
        let register: Option<UserRecord> = db_instence
            .surreal_client
            .update(("user", user_id.clone()))
            .merge(EmailVerificationSuccess {
                is_email_verified: true,
            })
            .await
            .map_err(|err| Error::DbPostError(err))?;

        // for more secuirity: we will delete the otp after verificaion of a user so that hacker could not gain that otp of targetted user

        // let delete_usero_otp: Option<()> = db_instence
        //     .delete(("otp", otp_code_from_user.email))
        //     .await
        //     .map_err(|err| Error::DbPostError(err))?;

        // if verfication is comlete then generate a jwt token to the the user:
        let token = jwt::sign(user_id.clone());

        // sent the jwt token as response
        println!(" JWT Token: {:?}", token);
        match token {
            // Ok(jwt_token) => Ok(format!(
            //     "User verified successfully {:#?}",
            //     TokenPayload {
            //         access_token: jwt_token,
            //         token_type: "Bearer".to_string()
            //     }
            // )),
            // Err(_) => return Err(Error::SomethingWentWrog.into()),
            Ok(jwt_token) => Ok(serde_json::to_string(&TokenPayload {
                access_token: jwt_token,
                token_type: "Bearer".to_string(),
            })
            .expect("unable to serialize")),
            Err(_) => return Err(Error::SomethingWentWrog.into()),
        }
    } else {
        Err(Error::SomethingWentWrog.into())
    }
}

/// when a user request to send for an otp, this function get called
pub async fn resend_otp_code(
    db_instence: DB,
    Json(user_email): Json<UserEmail>,
) -> ApiResult<String> {
    let user_email = user_email.email.clone();
    //check if the user already request 3 time for sending otp
    // if he sent less than 3 we will sent a otp code for him, otherwise we will show the time when he can send request again for otp,

    // get the last otp record( counter, last sent time)
    let otp_details_from_db: Option<OtpDetails> = db_instence
        .surreal_client
        .select(("otp", user_email.clone()))
        .await
        .map_err(|err| Error::DbPostError(err))?;

    println!("Otp details form db:  {:?}", otp_details_from_db);

    match otp_details_from_db {
        Some(otp_detls) => {
            //check if a user continuely tapping button -- resent otp ( we have to sent a message for him - Too many request, try again after 10 seconds)
            let time_diff = Local::now()
                .naive_local()
                .signed_duration_since(otp_detls.last_sent_time)
                .num_seconds();

            println!("time differrence:  {}", time_diff);

            //if a user tap again in resend otp button between 30 seconds- this message will show appeared
            if (time_diff <= 30) {
                return Err(Error::OtpContinuoslyResendingError)?;
            }

            println!("sent otp count  {}", otp_detls.sent_count);

            // check if counter if less than 3
            if (otp_detls.sent_count >= 3) {
                // check if the limit crossing punishment is over, we have to set the sent_count =0, so that user could request for otp again
                // we punished the user for not to request an otp for 5 miniutes,
                // check if the 5 miniutes are over (5*60= 300 seconds)
                if (time_diff <= 300) {
                    // if 300 seconds is not over
                    return Err(Error::OtpSentMultipleTimeError)?;
                } else {
                    // if 300 seconds is over
                    let save_otp_details: Option<OtpDetails> = db_instence
                        .surreal_client
                        .update(("otp", user_email.clone()))
                        .merge(OtpDetails {
                            sent_count: 0, // set to 0
                            last_sent_time: Local::now().naive_local(),
                        })
                        .await
                        .map_err(|err| Error::DbPostError(err))?;
                }
            }

            // generate e new otp code for user
            let otp = generate_otp();

            // save the otp into otp table according to user id
            let save_otp_to_db: Option<OTP> = db_instence
                .surreal_client
                .update(("otp", user_email.clone()))
                .merge(OTP { otp: otp.clone() })
                .await
                .map_err(|err| Error::DbPostError(err))?;

            // sent the otp to the registerd userEmail
            email_sent(user_email.clone(), otp);

            // update the (sent_count & last_sent_time) value
            let save_otp_details: Option<OtpDetails> = db_instence
                .surreal_client
                .update(("otp", user_email.clone()))
                .merge(OtpDetails {
                    sent_count: otp_detls.sent_count + 1, // increment the message sent counter
                    last_sent_time: Local::now().naive_local(), // storing the time when the otp has been sent
                })
                .await
                .map_err(|err| Error::DbPostError(err))?;
        }

        // if alll checking complete successfully: we will sent otp to the user and update sent_count, sent_last_time value
        None => {}
    }

    // Ok(format!("New Verification code has been sent..."))

    return Ok(serde_json::to_string(&UnifiedResponse {
        status: "Success".to_string(),
        message: "New Verification code has been sent".to_string(),
    })
    .expect("unable to serialize"));
}

/// get the token from request , then decode and verify it,
pub async fn get_account() -> String {
    "Hello".to_string()
}
