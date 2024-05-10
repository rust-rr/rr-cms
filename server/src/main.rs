use axum::{
    extract::{Path, Query},
    response::{Html, IntoResponse},
    routing::{get, get_service},
    Router,
};
use serde::Deserialize;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

pub use self::error::{Error, Result};

mod error;
mod web;

#[tokio::main]
async fn main() {
    let routes_all = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .fallback_service(routes_static());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Listening on http://{addr}\n");
    axum::serve(listener, routes_all).await.unwrap();
}

fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello2))
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
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
