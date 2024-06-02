use axum::{
    http::Method,
    routing::{get, post},
    Router,
};

use tower_http::cors::{Any, CorsLayer};

use crate::{
    handlers::auth_handlers::{get_account, otp_verification, resend_otp_code, sign_in, sign_up},
    utils::app_state::AppState,
};

pub fn auth_route() -> Router<AppState> {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(Any);

    let router = Router::new()
        .route("/register", post(sign_up))
        .route("/login", post(sign_in))
        .route("/verify_otp", post(otp_verification))
        .route("/resend_otp", post(resend_otp_code))
        .route("/account", get(get_account))
        // .route("/change_password", post()
        // .route("/", post()
        .layer(cors);

    router
}
