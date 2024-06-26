#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_parens)]

use std::sync::Arc;

use axum::{middleware, Extension, Router};
use handlers::student_handlers::get_specifik_students;
use tokio::net::TcpListener;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;
use tracing_appender::rolling;
use tracing_subscriber::fmt::writer::MakeWriterExt;
use utils::{
    app_state::{self, AppState},
    db_instance::{self, unified_db_instance},
    guard::guard,
};
mod handlers;
mod models;
mod routes;
mod services;
mod utils;

#[tokio::main]
pub async fn main() -> surrealdb::Result<()> {
    // creates a file for saving logs, although now it's saving, if you want to save uncomment 2 line - 29, 30
    let _info_file = rolling::hourly("./logs", "info").with_max_level(tracing::Level::INFO);

    // initialize tracing
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        // .with_writer(info_file)// save all the trace in the log file
        // .with_max_level(tracing::Level::TRACE) // set's all the details of tracing
        .init();

    // let db_instance = Arc::new(utils::db_instance::unified_db_instance().await);
    let db_state = AppState {
        surreal_client: unified_db_instance().await,
    };

    // Passing database using arguments, that argument will be used as state in student_route()
    let student_route = routes::student_route::student_route(db_state.clone());

    // Passing the surreal Db Client Database instance using state
    let auth_route = routes::auth_routes::auth_route().with_state(db_state);

    let app = Router::new()
        .merge(student_route) // for only students routes
        .merge(auth_route) // for all authentication
        .layer(
            // initialize tracing for every response - whatever error or success happens on response
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        );

    let listener = TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Unable to connect with 127.0.0.1:3000");

    println!("Application started");

    axum::serve::serve(listener, app.into_make_service())
        .await
        .expect("App servicing failed");

    Ok(())
}
