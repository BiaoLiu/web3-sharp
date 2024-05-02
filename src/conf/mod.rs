use std::time::Duration;

use config::Config as ConfigRs;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Server {
    pub port: u16,
}

#[derive(Deserialize)]
pub struct Log {
    pub level: String,
    pub path: String,
}

#[serde_with::serde_as]
#[derive(Deserialize)]
pub struct Database {
    pub max_idle_conns: u32,
    pub max_open_conns: u32,
    #[serde_as(as = "serde_with::DurationSeconds<u64>")]
    pub idle_timeout: Duration,
    #[serde_as(as = "serde_with::DurationSeconds<u64>")]
    pub max_lifetime: Duration,
    pub source: String,
    pub log_level: String,
}

#[serde_with::serde_as]
#[derive(Deserialize)]
pub struct Config {
    pub server: Server,
    pub log: Log,
    pub database: Database,
}

impl Config {
    pub fn load(name: &str) -> Self {
        let settings = ConfigRs::builder()
            .add_source(config::File::with_name(name))
            .build()
            .unwrap();

        let config = settings.try_deserialize::<Self>().unwrap();
        config
    }
}
