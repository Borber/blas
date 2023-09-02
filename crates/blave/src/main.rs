mod common;
mod resp;
mod service;

use anyhow::Result;

use crate::common::{init, CONTEXT};

pub const BASE: &str = "http://127.0.0.1:3010";

// TODO 构建一个任务执行,  直接操作 Task 的一个实例

#[tokio::main]
async fn main() -> Result<()> {
    CONTEXT.get_or_init(init).await;

    let tasks = service::register::index().await;
    context!().tasks.add(tasks.as_slice());

    let handler = tokio::spawn(async move {
        loop {
            let resp = service::beat::index().await;
            println!("{}", resp);
            tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
        }
    });

    handler.await.unwrap();
    Ok(())
}
