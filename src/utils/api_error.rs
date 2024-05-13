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
    #[error("something went wrong to get")]
    DbGetError(surrealdb::Error),
    #[error("something went wrong to post ")]
    DbPostError(#[from] surrealdb::Error),
    #[error("something went wrong to update ")]
    DbUpdateError,
    #[error("something went wrong to delete")]
    DbDeletError,
    #[error("wrong credentials")]
    WrongCredentials,
    #[error("password doesn't match")]
    WrongPassword,
    #[error("account is already exist")]
    DuplicateUserEmail,
    #[error("name is already taken")]
    DuplicateUserName,
    #[error("something went wrong")]
    SomethingWentWrog,
    #[error("email is not valid")]
    EmailValidationError,
    #[error("Your email is not verified yet, Please verify")]
    EmailVerificationError,
    #[error("Limit Crossed. Please try again after 5 minutes")]
    OtpSentMultipleTimeError,
    #[error("Too many requests. Please try again after 30 seconds")]
    OtpContinuoslyResendingError,
}

pub type Result<T> = std::result::Result<T, Error>;

pub type ApiError = (StatusCode, Json<Value>);

pub type ApiResult<T> = std::result::Result<T, ApiError>;

// converting Error to ApiError
//read more here:  https://doc.rust-lang.org/rust-by-example/conversion/from_into.html

impl From<Error> for ApiError {
    fn from(err: Error) -> Self {
        let status = match err {
            Error::WrongCredentials => StatusCode::UNAUTHORIZED,
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
