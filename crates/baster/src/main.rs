mod controller;
mod handler;
mod model;
mod resp;

use anyhow::Result;
use axum::{routing::post, Router};
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() -> Result<()> {
    let router = Router::new()
        .route("/", post(controller::register::add))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );

    let addr = "0.0.0.0:3010";

    println!("Listening on {}", addr);

    axum::Server::bind(&addr.parse().unwrap())
        .serve(router.into_make_service())
        .await?;
    Ok(())
}
