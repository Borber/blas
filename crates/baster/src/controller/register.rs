use model::register::{RegisterAddDTO, RegisterBeatDTO, Task};

use crate::{handler::Json, resp::Resp};

/// Add a new register
/// 分配一个新的任务列表给注册的主机
pub async fn add(Json(dto): Json<RegisterAddDTO>) -> Resp<Vec<Task>> {
    println!("host: {}", dto.host);

    let tasks = vec![Task {
        name: "task1".to_owned(),
        version: "1231231231".to_owned(),
        start: "echo 'Task start'".to_owned(),
        stop: "echo 'Task stop'".to_owned(),
        restart: "echo 'Task restart'".to_owned(),
        status: "echo 'Task status'".to_owned(),
    }];

    Resp::success(tasks)
}

/// Beat the register
/// TODO 检测版本是否一致， 不一致则需要更新
/// 输入: 任务列表
/// 输出: 需要更新的任务 with 最新版本号
pub async fn beat(Json(dto): Json<RegisterBeatDTO>) -> Resp<String> {
    let tasks = dto.list;
    println!("tasks: {:#?}", tasks);
    Resp::success("ok".to_string())
}
