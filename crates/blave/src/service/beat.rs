use model::register::RegisterBeatDTO;

use crate::{common::CLIENT, context, resp::Resp, BASE};

pub async fn index() -> String {
    let tasks = { context!().tasks.info() };
    let resp = CLIENT
        .post(format!("{BASE}/beat"))
        .json(&RegisterBeatDTO { list: tasks })
        .send()
        .await
        .unwrap()
        .json::<Resp<String>>()
        .await
        .unwrap();
    resp.data.unwrap()
}
