mod common;
mod resp;
mod service;

use anyhow::Result;
use common::CLIENT;
use model::register::RegisterBeatDTO;
use resp::Resp;

use crate::common::{init, CONTEXT};

pub const BASE: &str = "http://127.0.0.1:3010";

#[tokio::main]
async fn main() -> Result<()> {
    CONTEXT.get_or_init(init).await;

    let tasks = service::register().await;
    context!().tasks.add(tasks.as_slice());

    let handler = tokio::spawn(async move {
        loop {
            println!("Current Tasks: {:?}", context!().tasks.info());

            let tasks = { context!().tasks.info() };
            let resp = CLIENT
                .post(format!("{BASE}/beat"))
                .json(&RegisterBeatDTO { list: tasks })
                .send()
                .await
                .unwrap()
                .json::<Resp<String>>()
                .await
                .unwrap();

            println!("Beat: {}", resp.data.unwrap());
            tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
        }
    });

    handler.await.unwrap();
    Ok(())
}
