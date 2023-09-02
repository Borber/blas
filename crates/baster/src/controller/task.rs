use model::task::TaskAddDTO;
use sea_orm::*;

use crate::{database::task, handler::Json, pool, resp::Resp};

// TODO 新增 Task 添加新的版本
pub async fn add(Json(dto): Json<TaskAddDTO>) -> Resp<String> {
    let entity = task::Entity::insert(task::ActiveModel::new(
        dto.name,
        dto.version,
        dto.start,
        dto.stop,
        dto.restart,
        dto.status,
    ))
    .exec(pool!())
    .await
    .unwrap();
    Resp::success("ok".to_string())
}
// TODO 删除 Task 删除所有版本
pub async fn remove() -> Resp<String> {
    Resp::success("ok".to_string())
}
// TODO 查询 Task 返回所有版本, 按已创建时间排序
pub async fn list() -> Resp<String> {
    Resp::success("ok".to_string())
}
