use sea_orm::{Database, DatabaseConnection};
use tokio::sync::OnceCell;

#[macro_export]
macro_rules! pool {
    () => {
        &$crate::common::CONTEXT.get().unwrap().pool
    };
}

pub static CONTEXT: OnceCell<Context> = OnceCell::const_new();

pub async fn init() -> Context {
    Context {
        pool: Database::connect("sqlite://baster.db").await.unwrap(),
    }
}

pub struct Context {
    pub pool: DatabaseConnection,
}
