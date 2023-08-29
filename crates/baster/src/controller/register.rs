use crate::{handler::Json, model::dto::RegisterAddDTO, resp::Resp};

/// Add a new register
pub async fn add(Json(dto): Json<RegisterAddDTO>) -> Resp<String> {
    println!("host: {}", dto.host);
    Resp::success("Hello, World!".to_string())
}
