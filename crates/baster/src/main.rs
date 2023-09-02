mod common;
mod controller;
mod database;
mod handler;
mod resp;

use anyhow::Result;
use axum::{routing::post, Router};
use tower_http::cors::{Any, CorsLayer};

// TODO 维护 主机-任务列表, 任务-主机列表
// TODO 在每次心跳的时候查询整体任务是否符合预期
// TODO Task持久化, 增删改查
// TODO beat 应该汇报主机状态, 任务状态, 任务日志

#[tokio::main]
async fn main() -> Result<()> {
    let router = Router::new()
        .route("/", post(controller::register::add))
        .route("/beat", post(controller::register::beat))
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
