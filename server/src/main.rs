use axum::{response::Html, routing::get, Router};
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route(
        "/hello",
        get(|| async { Html("Hello <strong>World</strong>") }),
    );

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Listening on http://{addr}\n");
    axum::serve(listener, routes_hello.into_make_service())
        .await
        .unwrap();
}
