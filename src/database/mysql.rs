use std::str::FromStr;
use std::time::Duration;

use sea_orm::{ConnectOptions, Database, DbConn};
use tracing::log;

pub struct Config {
    pub max_idle_conns: u32,
    pub max_open_conns: u32,
    pub idle_timeout: Duration,
    pub max_lifetime: Duration,
    pub source: String,
    pub log_level: String,
}

pub async fn new_connection(conf: Config) -> DbConn {
    let mut opt = ConnectOptions::new(conf.source).to_owned();
    opt.min_connections(5)
        .max_connections(conf.max_open_conns)
        .connect_timeout(Duration::from_secs(5))
        .idle_timeout(conf.idle_timeout)
        .max_lifetime(conf.max_lifetime)
        .sqlx_logging(false)
        .sqlx_logging_level(log::LevelFilter::from_str(&conf.log_level).unwrap());

    let conn = Database::connect(opt)
        .await
        .expect("database connection error");
    conn
}
