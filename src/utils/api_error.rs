// #![allow(dead_code)]
// #![allow(unused_variables)]

use axum::{http::StatusCode, Json};
use serde_json::{json, Value};
use thiserror::Error;

use crate::models::auth_model::UnifiedResponse;

// here we made Enum type error -> using thiserror trait..
// https://crates.io/crates/thiserror

// if any Enum type error called, we show our custom messages for each enums as reponse.

#[derive(Debug, Error)]
pub enum Error {
    /// if database error happeneed in Get request
    #[error("something went wrong to get")]
    DbGetError(surrealdb::Error),
    /// if database error happeneed in Post request

    #[error("something went wrong to post ")]
    DbPostError(#[from] surrealdb::Error),
    /// if database error happeneed in Update request

    #[error("something went wrong to update ")]
    DbUpdateError,
    /// if database error happeneed in Delete request

    #[error("something went wrong to delete")]
    DbDeletError,
    /// if Authentication credentials is wrong

    #[error("wrong credentials")]
    WrongCredentials,
    /// If user password didn't match
    #[error("password doesn't match")]
    WrongPassword,
    /// If user wants to register with the same email which is alredy registered
    #[error("account is already exist")]
    DuplicateUserEmail,
    /// If user wants to register with the same username which is alredy registered
    #[error("name is already taken")]
    DuplicateUserName,
    /// If something went wrong
    #[error("something went wrong")]
    SomethingWentWrog,
    /// If user inputted email is not valid
    #[error("email is not valid")]
    EmailValidationError,
    /// If user's email is not verified with otp
    #[error("Your email is not verified yet, Please verify")]
    EmailVerificationError,
    /// If user request for otp verification for 3 times, but didn't verify , we won't let user to request for otp in 5 miniutes, After 5 minutes user can request sent an otp for verification
    #[error("Limit Crossed. Please try again after 5 minutes")]
    OtpSentMultipleTimeError,
    /// If user continuesly tapping resend otp button,
    #[error("Too many requests. Please try again after 30 seconds")]
    OtpContinuoslyResendingError,
    /// If user's jwt token is not valid
    #[error("token is not valid,")]
    InvalidJwtToken,
    /// If token is not authorized
    #[error("Token is not authorized")]
    UnAuthorizedJwtToken,
    /// If jwt token is expired
    #[error("Token has been expired")]
    JwtTokenExpired,
}

pub type Result<T> = std::result::Result<T, Error>;

pub type ApiError = (StatusCode, Json<Value>);

pub type ApiResult<T> = std::result::Result<T, ApiError>;

// converting Error to ApiError
//read more here:  https://doc.rust-lang.org/rust-by-example/conversion/from_into.html

impl From<Error> for ApiError {
    fn from(err: Error) -> Self {
        let status = match err {
            Error::WrongCredentials | Error::UnAuthorizedJwtToken | Error::InvalidJwtToken => {
                StatusCode::UNAUTHORIZED
            }
            Error::DuplicateUserEmail => StatusCode::CONFLICT,
            Error::DuplicateUserName => StatusCode::CONFLICT,
            Error::OtpSentMultipleTimeError => StatusCode::TOO_MANY_REQUESTS,
            Error::OtpContinuoslyResendingError => StatusCode::TOO_MANY_REQUESTS,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        // let payload= json!({"message":err.to_string()});
        let payload = UnifiedResponse {
            status: "error".to_string(),
            message: err.to_string(),
        };

        (status, Json(json!(payload)))
    }
}
