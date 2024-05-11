use self::model::ModelController;
use self::web::{
    middleware::mw_res_map,
    routes::{routes_hello, routes_login, routes_static, routes_tickets},
};
use axum::{middleware, Router};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;

pub use self::error::{Error, Result};

mod error;
mod model;
mod web;

#[tokio::main]
async fn main() -> Result<()> {
    let mc = ModelController::new().await?;

    let routes_all = Router::new()
        .merge(routes_hello::routes())
        .merge(routes_login::routes())
        .nest("/api", routes_tickets::routes(mc.clone()))
        .layer(middleware::map_response(mw_res_map::mw_response_map))
        .layer(CookieManagerLayer::new())
        .merge(routes_static::routes());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Listening on http://{addr}\n");
    axum::serve(listener, routes_all.into_make_service())
        .await
        .unwrap();

    Ok(())
}
