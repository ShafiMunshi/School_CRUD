use axum::{http::Method, routing::get, Router};

use tower_http::cors::{Any, CorsLayer};

use crate::handlers::student_handlers::{self, create_student};

pub fn student_route() -> Router {
    // creating a http cors layer, for routing , so that this Router can be merged with Axum-Router
    // this applies the http-CORS middleware which add header 
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(Any);// allow all the request

    let router = Router::new()
        .route(
            "/students",
            get(student_handlers::get_all_students).post(create_student),// we can make multiple handlers on single route path like this, but note that- every handler method should be different
        )
        .layer(cors);

    router
}
