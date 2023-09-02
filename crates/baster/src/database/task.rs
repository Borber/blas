use std::time::{SystemTime, UNIX_EPOCH};

use sea_orm::{entity::prelude::*, Set};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "task")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub name: String,
    pub version: String,
    pub start: String,
    pub stop: String,
    pub restart: String,
    pub status: String,
    pub create_time: i64,
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    pub fn new(
        name: String,
        version: String,
        start: String,
        stop: String,
        restart: String,
        status: String,
    ) -> Self {
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        Self {
            name: Set(name),
            version: Set(version),
            start: Set(start),
            stop: Set(stop),
            restart: Set(restart),
            status: Set(status),
            create_time: Set(time as i64),
            ..Default::default()
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
