use axum::{http::Method, middleware, routing::get, Router};

use tower_http::cors::{Any, CorsLayer};

use crate::{
    handlers::student_handlers::get_specifik_students,
    utils::{app_state::AppState, guard::guard},
};

pub fn student_route(state: AppState) -> Router {
    // creating a http cors layer, for routing , so that this Router can be merged with Axum-Router
    // this applies the http-CORS middleware which add header
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(Any); // allow all the request

    let router = Router::new()
        .route("/students", get(get_specifik_students))
        .route_layer(middleware::from_fn(guard)) // set middleware wall for above routes which will first authenticate the jwt token and then gives the responses what user's requested for
        .with_state(state)// passing the database instance to every handler functioin using state
        .layer(cors);

    router
}
