use model::register::Task;
use once_cell::sync::Lazy;
use reqwest::Client;
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::sync::OnceCell;

pub static CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .unwrap()
});

pub static CONTEXT: OnceCell<Context> = OnceCell::const_new();

pub async fn init() -> Context {
    Context {
        tasks: Arc::new(Mutex::new(vec![])),
    }
}

// 注册标识
pub struct Context {
    pub tasks: Arc<Mutex<Vec<Arc<Mutex<Task>>>>>,
}
