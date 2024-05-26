use axum::{
    extract::{Path, Query},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

use serde::Deserialize;

pub fn routes() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello2))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}
// e.g., /hello?name=Richard
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!(
        "->> {:<12} - {:<24} - {:?}",
        "HANDLER", "handler_hello", params
    );
    let name = params.name.as_deref().unwrap_or("World!");
    Html(format!("Hello <strong>{name}</strong>"))
}
// e.g., /hello/Richard
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!(
        "->> {:<12} - {:<24} - {}",
        "HANDLER", "handler_hello2", name
    );
    Html(format!("Hello <strong>{name}</strong>"))
}
