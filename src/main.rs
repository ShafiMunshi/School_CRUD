#![allow(dead_code)]
#![allow(unused_variables)]


use std::sync::Arc;

use axum::{Extension, Router};
use tokio::net::TcpListener;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;
use tracing_appender::rolling;
use tracing_subscriber::fmt::writer::MakeWriterExt;


mod models;
mod routes;
mod utils;
mod handlers;

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    // creates a file for saving logs, although now it's saving, if you want to save uncomment 2 line - 29, 30
    let _info_file = rolling::hourly("./logs", "info").with_max_level(tracing::Level::INFO);

    // initialize tracing
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        // .with_writer(info_file)// save all the trace in the log file
        // .with_max_level(tracing::Level::TRACE) // set's all the details of tracing
        .init();

    
   let db_instance = Arc::new(utils::db_instance::unified_db_instance().await);

    let app = Router::new()
        .merge(routes::student_route::student_route())// for only students routes
        .merge(routes::auth_routes::auth_route())// for all authentication
        .layer(Extension(db_instance))
        .layer(                         // initialize tracing for every response - whatever error or success happens on response
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
