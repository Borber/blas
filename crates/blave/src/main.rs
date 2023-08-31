mod common;
mod resp;

use std::sync::{Arc, Mutex};

use anyhow::Result;
use common::CLIENT;
use model::register::{RegisterBeatBean, RegisterBeatDTO, Task};
use resp::Resp;
use serde::Serialize;

use crate::common::{init, CONTEXT};

#[derive(Debug, Serialize)]
pub struct Blave {
    host: String,
}

const BASE: &str = "http://127.0.0.1:3010";

// TODO 全局储存是否已经注册
// TODO 全局储存任务列表, 以及每个任务的版本号, 启动, 停止, 更新, 删除任务的脚本

#[tokio::main]
async fn main() -> Result<()> {
    CONTEXT.get_or_init(init).await;

    let blave = Blave {
        host: gethostname::gethostname().into_string().unwrap(),
    };

    let resp = CLIENT
        .post(format!("{BASE}"))
        .json(&blave)
        .send()
        .await?
        .json::<Resp<Vec<Task>>>()
        .await?;

    let aaa = &CONTEXT.get().unwrap().tasks;
    resp.data.unwrap().into_iter().for_each(|task| {
        aaa.lock().unwrap().push(Arc::new(Mutex::new(task)));
    });

    // TODO 新开一个线程，定时发送心跳
    let handler = tokio::spawn(async move {
        loop {
            println!("Current Tasks: {:#?}", CONTEXT.get().unwrap().tasks);
            let tasks = CONTEXT
                .get()
                .unwrap()
                .tasks
                .lock()
                .unwrap()
                .iter()
                .map(|task| {
                    let task = task.lock().unwrap();
                    RegisterBeatBean {
                        name: task.name.clone(),
                        version: task.version.clone(),
                    }
                })
                .collect::<Vec<RegisterBeatBean>>();

            let resp = CLIENT
                .post(format!("{BASE}/beat"))
                .json(&RegisterBeatDTO { list: tasks })
                .send()
                .await
                .unwrap()
                .json::<Resp<String>>()
                .await
                .unwrap();

            println!("Beat stdout: {}", resp.data.unwrap());
            tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
        }
    });

    handler.await.unwrap();
    Ok(())
}
