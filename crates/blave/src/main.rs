mod common;

use anyhow::Result;
use common::CLIENT;
use serde::Serialize;
use serde_json::Value;
use tokio::process::Command;

#[derive(Debug, Serialize)]
pub struct Blave {
    host: String,
}

const BASE: &str = "http://rack:3010";

#[tokio::main]
async fn main() -> Result<()> {
    let blave = Blave {
        host: gethostname::gethostname().into_string().unwrap(),
    };

    let response = CLIENT
        .post(format!("{BASE}"))
        .json(&blave)
        .send()
        .await?
        .json::<Value>()
        .await?;

    println!("Register: {}", response);

    let response = CLIENT
        .get(format!("{BASE}/init"))
        .send()
        .await?
        .json::<Value>()
        .await?;

    let data = response["data"].as_str().unwrap();

    let output = Command::new("sh")
        .arg("-c")
        .arg(data)
        .output()
        .await
        .unwrap();

    println!("Init stdout: {}", String::from_utf8_lossy(&output.stdout));

    Ok(())
}
