mod common;

use anyhow::Result;
use common::CLIENT;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Blave {
    host: String,
}

const BASE: &str = "http://seam/";

#[tokio::main]
async fn main() -> Result<()> {
    let blave = Blave {
        host: gethostname::gethostname().into_string().unwrap(),
    };

    let response = CLIENT.post(BASE).json(&blave).send().await?;

    println!("{}", response.text().await?);

    Ok(())
}
