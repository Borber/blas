use once_cell::sync::Lazy;
use reqwest::Client;
use std::time::Duration;

pub static CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .unwrap()
});
