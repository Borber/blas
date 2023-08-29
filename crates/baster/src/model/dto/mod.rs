use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RegisterAddDTO {
    pub host: String,
}
