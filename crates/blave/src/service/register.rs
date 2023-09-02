use anyhow::Result;
use async_recursion::async_recursion;
use model::register::{Blave, Task};

use crate::common::CLIENT;
use crate::resp::Resp;
use crate::BASE;

pub async fn index() -> Vec<Task> {
    register(&Blave {
        host: gethostname::gethostname()
            .into_string()
            .unwrap_or("unknow".to_string()),
    })
    .await
}

#[async_recursion]
async fn register(blave: &Blave) -> Vec<Task> {
    match add(blave).await {
        Ok(tasks) => tasks,
        Err(e) => {
            println!("{}", e.to_string());
            println!("Register failed, try again after 3 seconds");
            tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
            register(blave).await
        }
    }
}

async fn add(blave: &Blave) -> Result<Vec<Task>> {
    let resp = CLIENT
        .post(BASE)
        .json(blave)
        .send()
        .await?
        .json::<Resp<Vec<Task>>>()
        .await?;
    resp.data.ok_or(anyhow::anyhow!("Add failed"))
}
