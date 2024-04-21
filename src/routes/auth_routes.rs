use axum::{
    http::Method,
    routing::{get, post},
    Router,
};

use tower_http::cors::{Any, CorsLayer};

use crate::handlers::
    auth_handlers::{sign_in, sign_up};

pub fn auth_route() -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(Any);

    let router = Router::new()
        .route("/register", post(sign_up))
        .route("/login", get(sign_in))
        .layer(cors);

    router
}
