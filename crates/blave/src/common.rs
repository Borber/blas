use model::register::{Task, TasksInfo};
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use reqwest::Client;
use std::{sync::Arc, time::Duration};
use tokio::sync::OnceCell;

pub static CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .unwrap()
});

#[macro_export]
macro_rules! context {
    () => {
        $crate::common::CONTEXT.get().unwrap()
    };
}

pub static CONTEXT: OnceCell<Context> = OnceCell::const_new();

pub async fn init() -> Context {
    Context {
        tasks: Tasks::init(),
    }
}

// 注册标识
pub struct Context {
    pub tasks: Tasks,
}

#[derive(Debug)]
pub struct Tasks {
    list: Arc<Mutex<Vec<Arc<Mutex<Task>>>>>,
}

impl Tasks {
    pub fn init() -> Self {
        Tasks {
            list: Arc::new(Mutex::new(vec![])),
        }
    }

    pub fn info(&self) -> Vec<TasksInfo> {
        self.list
            .lock()
            .iter()
            .map(|task| TasksInfo {
                name: task.lock().name.clone(),
                version: task.lock().version.clone(),
            })
            .collect()
    }

    pub fn add(&self, tasks: &[Task]) {
        let mut list = tasks
            .iter()
            .map(|task| Arc::new(Mutex::new(task.clone())))
            .collect();

        self.list.lock().append(&mut list);
    }

    pub fn get(&self) -> Vec<Task> {
        self.list
            .lock()
            .iter()
            .map(|task| task.lock().clone())
            .collect()
    }

    pub fn remove(&self, tasks: Vec<&str>) {
        let mut list = self.list.lock();
        tasks
            .into_iter()
            .for_each(|task| list.retain(|t| t.lock().name != *task));
    }

    pub fn update(&self, tasks: &[Task]) {
        let mut list = self.list.lock();
        tasks.iter().for_each(|task| {
            list.iter_mut().for_each(|t| {
                let mut t = t.lock();
                if t.name == task.name {
                    *t = task.clone();
                }
            })
        });
    }
}
