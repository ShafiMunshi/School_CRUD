use axum::{Extension, Json};
use chrono::Local;

use std::sync::Arc;
use surrealdb::{engine::remote::ws::Client, Surreal};
use validator::Validate;

use crate::{
    models::auth_model::{
        EmailVerificationSuccess, LoginInput, OTPFromUser, OTPRecord, OtpDetails, RegisterInput,
        UserRecord, ID, OTP,
    },
    services::verification_code::{email_sent, generate_otp},
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
            email: register_input.email.clone(),
        })
        .await
        .map_err(|err| Error::DbPostError(err))?; // if something error happend on surrealDb post request, map_err() will capture the error and return it as out custom defined error

    let user_id = register[0].id.id.clone().to_string();

    // generate a otp and send the otp as verification code to registered user email
    let otp = generate_otp();

    // save the current otp on database, so that we could verify the otp in /otp_verify (route)
    println!(" REgistered id is: {}", user_id);

    // save the generated otp in the database
    let save_otp: Option<OTP> = db_instence
        .create(("otp", user_id.clone()))
        .content(OTP { otp: otp.clone() })
        .await
        .map_err(|err| Error::DbPostError(err))?;

    // save some extra creadential for checking further,( otp sending limit verification)
    let save_otp_details: Option<OtpDetails> = db_instence
        .update(("otp", user_id.clone()))
        .merge(OtpDetails {
            sent_count: 1,                              // initially setted the 1 time sent
            last_sent_time: Local::now().naive_local(), // storing the time when the otp has been sent
        })
        .await
        .map_err(|err| Error::DbPostError(err))?;

    // sent the otp to the registerd userEmail
    email_sent(register_input.email, otp);

    Ok(format!("Registerd User: {:?}", register))
}

pub async fn sign_in(db_instence: DB, Json(login_input): Json<LoginInput>) -> ApiResult<String> {
    // check if the email is valid or not
    // then return EmailValidationErrror
    match login_input.validate() {
        Ok(_) => (),
        Err(_) => return Err(Error::EmailValidationError.into()), // if email is not valid
    };

    // find the id using loginInput email, get the whole record of that targette user
    // if the the email is not exists, return UnAuthosized Error

    let user: Option<UserRecord> =
        email_exists(db_instence.clone(), login_input.email.clone()).await;
    let user_id;
    match user {
        Some(user_record) => user_id = user_record.id.id.to_string(),
        None => return Err(Error::WrongCredentials.into()), // return - "wrong credentials because there are no email in the whole database"
    };

    // after getting the hashed password - verify it.
    let user_record: Option<UserRecord> = db_instence // get a single user record for password verification,
        .select(("user", user_id))
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

        return Ok(format!("Authentication Successfull")); // authentication successfull
    } else {
        return Err(Error::WrongCredentials.into()); // password is not verified.
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

pub async fn otp_verification(
    db_instence: DB,
    Json(otp_code_from_user): Json<OTPFromUser>,
) -> ApiResult<String> {
    let otp_from_db: Option<OTPRecord> = db_instence
        .select(("otp", otp_code_from_user.id.clone()))
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
        let register: Option<UserRecord> = db_instence
            .update(("user", otp_code_from_user.id))
            .merge(EmailVerificationSuccess {
                is_email_verified: true,
            })
            .await
            .map_err(|err| Error::DbPostError(err))?;

        // for more secuirity: we will delete the otp after verificaion of a user

        Ok(format!("User verified successfully"))
    } else {
        Ok(format!("Could not verify the user"))
    }
}


pub async fn resend_otp_code(db_instence: DB, Json(user_id): Json<ID>) -> ApiResult<String> {
    let user_id = user_id.id.clone();
    //check if the user already request 3 time for sending otp
    // if he sent less than 3 we will sent a otp code for him, otherwise we will show the time when he can send request again for otp,

    // get the last otp record( counter, last sent time)
    let otp_details_from_db: Option<OtpDetails> = db_instence
        .select(("otp", user_id.clone()))
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

            //if a user tap again in resend otp button - this message will show appeared
            if (time_diff <= 30) {
                return Err(Error::OtpContinuoslyResendingError)?;
            }

            println!("sent otp count  {}", otp_detls.sent_count);

            // check if counter if less than 3
            if (otp_detls.sent_count >= 3) {
                return Err(Error::OtpSentMultipleTimeError)?;
            }

            // generate e new otp code for user
            let otp = generate_otp();

            // save the otp into otp table according to user id
            let save_otp_to_db: Option<OTP> = db_instence
                .update(("otp", user_id.clone()))
                .merge(OTP { otp: otp.clone() })
                .await
                .map_err(|err| Error::DbPostError(err))?;

            // get the user email using his id
            let user: Option<UserRecord> = db_instence // get a single user record for password verification,
                .select(("user", user_id.clone()))
                .await
                .map_err(|err| Error::DbGetError(err))?;

           

            // sent the otp to the registerd userEmail
            match user.as_ref() {
                Some(user_data) => email_sent(user.unwrap().email, otp), //TODO: using unwrap() in Option is not good practise, Furuther modify
                None => println!("No email found to sent, check you id again "),
            }


            // update the (sent_count & last_sent_time) value
            let save_otp_details: Option<OtpDetails> = db_instence
                .update(("otp", &user_id))
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

    Ok(format!("New Verification code has been sent..."))
}
