//! School is used for authentication for personal application.
//!
//! # Table of contents
//!
//! - [Authentication Model](#authentication-model)

//! - [Routing](#routing)
//! - [Handler Function](#example)
//! - [Handlers](#handlers)
//! - [Error handling](#error-handling)
//! - [Middleware](#middleware)

//!
//! # Authentication Model

//! #####  Used Json Data (theese data comes from the frontend)
//! ```rust
//! // for login
//! {
//!     "email" : "user@gmail.com"
//!     "password" : "password1234"
//! }
//!
//! // for register
//! {
//!     "name" : "User Name"
//!     "email" : "user@gmail.com"
//!     "password" : "password1234"
//! }
//!
//! // for otp verification
//! {
//!     "email" : "user@gmail.com"
//!     "otp" : "123456"
//! }
//!
//! // resend otp
//! {
//!     "email" : "user@gmail.com"
//! }
//!
//! ```
//! See [`auth_model`](crate::models::auth_model) for more detaitls on Authenticaion Model.

//!

//! # Routing
//! Here is all the authentication routes we have used

//!
//! ```rust
//!
//! let router = Router::new()
//!     .route("/register", post(sign_up))
//!     .route("/login", post(sign_in))
//!     .route("/verify_otp", post(otp_verification))
//!     .route("/resend_otp", post(resend_otp_code))
//!     .route("/account", get(get_account))
//!     .route("/change_password", post(change_password);
//!     
//!
//! // which calls one of these handlers

//! # let _: Router = app;
//! ```
//!
//! See [`auth_routes`](crate::routes::auth_routes) for more details on routing.
//!
//! # Handlers
//! ```rust
//! //
//! async fn sign_up() {}
//! async fn sign_in() {}
//! async fn otp_verification() {}
//! async fn resend_otp_code() {}
//! async fn get_account() {}
//! async fn change_password() {}
//!```
//!
//! See [`auth_routes`](crate::handlers::auth_handlers) for more details on handling routes.
//!
//! # Error handling
//!
//! we have used a structural error handling using enums,
//! first we separate all the error into different enums then return an error response according to that error
//!
//! ```rust
//!
//! #[derive(Debug, Error)]
//! pub enum Error {
//!     #[error("something went wrong to get")]
//!     DbGetError(surrealdb::Error),
//!     #[error("something went wrong to post ")]
//!     DbPostError(#[from] surrealdb::Error),
//!     #[error("something went wrong to update ")]
//!     DbUpdateError,
//!     #[error("something went wrong to delete")]
//!     DbDeletError,
//!     #[error("wrong credentials")]
//!     WrongCredentials,
//!     #[error("password doesn't match")]
//!     WrongPassword,
//!     #[error("account is already exist")]
//!     DuplicateUserEmail,
//!     #[error("name is already taken")]
//!     DuplicateUserName,
//!     #[error("something went wrong")]
//!     SomethingWentWrog,
//!     #[error("email is not valid")]
//!     EmailValidationError,
//!     #[error("Your email is not verified yet, Please verify")]
//!     EmailVerificationError,
//!     #[error("Limit Crossed. Please try again after 5 minutes")]
//!     OtpSentMultipleTimeError,
//!     #[error("Too many requests. Please try again after 30 seconds")]
//!     OtpContinuoslyResendingError,
//!     #[error("token is not valid,")]
//!     InvalidJwtToken,
//!     #[error("Token is not authorized")]
//!     UnAuthorizedJwtToken,
//!     #[error("Token has been expired")]
//!     JwtTokenExpired,

//! }
//!
//! ```
//!
//! See [`api_error`](crate::utils::api_error) for more details on error handling
//! error handling model and how to handle errors gracefully.
//!
//! # Middleware
//!
//! There are several different ways to write middleware for axum. See
//! [`middleware`](crate::middleware) for more details.
//!
//! # Sharing state with handlers
//!
//! It is common to share some state between handlers. For example, a
//! pool of database connections or clients to other services may need to
//! be shared.
//!
//! The three most common ways of doing that are:
//! - Using the [`State`] extractor
//! - Using request extensions
//! - Using closure captures
//!
// #![deny(missing_docs)]

mod handlers;
mod models;
pub mod routes;
pub mod services;
mod utils;

pub use models::auth_model::LoginInput;
pub use models::auth_model::OTPFromUser;
pub use models::auth_model::OtpDetails;
pub use models::auth_model::RegisterInput;
pub use models::auth_model::RegisterdDataDb;
pub use models::auth_model::UnifiedResponse;
pub use models::auth_model::UserRecord;
pub use models::auth_model::OTP;
pub use utils::api_error::Error;
// pub use utils::jwt;

pub use handlers::auth_handlers::sign_in;
