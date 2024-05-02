pub mod entity;

use sea_orm::{DbConn};
use std::sync::Arc;
use crate::{conf, database};

pub struct Data {
    pub db: DbConn,
}

impl Data {
    pub async fn new(conf: Arc<conf::Config>) -> Self {
        let conn = new_mysql(conf).await;
        Data { db: conn }
    }
}

pub async fn new_mysql(conf: Arc<conf::Config>) -> DbConn {
    let c = database::mysql::Config {
        max_idle_conns: conf.database.max_idle_conns,
        max_open_conns: conf.database.max_open_conns,
        idle_timeout: conf.database.idle_timeout,
        max_lifetime: conf.database.max_lifetime,
        source: conf.database.source.to_owned(),
        log_level: conf.database.log_level.to_owned(),
    };
    database::mysql::new_connection(c).await
}