use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterAddDTO {
    pub host: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterBeatDTO {
    pub list: Vec<RegisterBeatBean>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterBeatBean {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    // 任务名称
    pub name: String,
    // 任务版本号
    pub version: String,
    // 启动命令
    pub start: String,
    // 停止命令
    pub stop: String,
    // 重启命令
    pub restart: String,
    // 查询任务状态的命令
    pub status: String,
}
