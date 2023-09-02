use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskAddDTO {
    pub name: String,
    pub version: String,
    pub start: String,
    pub stop: String,
    pub restart: String,
    pub status: String,
}
