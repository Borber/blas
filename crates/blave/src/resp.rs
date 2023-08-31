use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Resp<T> {
    pub code: i64,
    pub msg: Option<String>,
    pub data: Option<T>,
}
