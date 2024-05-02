use clap::Parser;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;
use axum::Router;
use axum::routing::get;
use tokio::signal;
use tower_http::trace;
use tower_http::trace::TraceLayer;
use tracing::Level;

use tracing_subscriber::{Layer, layer::Filter, layer::SubscriberExt, util::SubscriberInitExt};
use tracing_subscriber::filter::LevelFilter;
use crate::conf::Log;

mod app;
mod conf;
mod data;
mod database;
mod error;
mod handler;
mod router;
mod service;
mod utils;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "FILE", default_value = "src/configs/config.toml")]
    conf: String,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let conf = Arc::new(conf::Config::load(cli.conf.as_str()));
    let _guard = init_tracing(&conf.log);
    let data = Arc::new(data::Data::new(conf.clone()).await);
    let service = service::Service::new(conf.clone(), data);

    let app = router::init_router(service);
    let addr = SocketAddr::from(([0, 0, 0, 0], conf.server.port));
    tracing::info!("listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

fn init_tracing(log_conf: &Log) -> tracing_appender::non_blocking::WorkerGuard {
    if !log_conf.path.is_empty() {
        let file_appender = tracing_appender::rolling::daily(&log_conf.path, "web3-sharp");
        let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
        let file_layer = tracing_subscriber::fmt::layer()
            .json()
            .with_writer(non_blocking)
            .with_target(false)
            .with_file(true)
            .with_line_number(true)
            .with_ansi(false)
            .with_filter(LevelFilter::from_str(&log_conf.level).unwrap());

        tracing_subscriber::registry()
            .with(file_layer)
            .init();
        _guard
    } else {
        let (non_blocking, _guard) = tracing_appender::non_blocking(std::io::stdout());
        let std_layer = tracing_subscriber::fmt::layer()
            .with_writer(non_blocking)
            .with_target(false)
            .with_file(true)
            .with_line_number(true)
            .with_ansi(false)
            .with_filter(LevelFilter::from_str(&log_conf.level).unwrap());

        tracing_subscriber::registry()
            .with(std_layer)
            .init();
        _guard
    }
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
        let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
        let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
              tracing::info!("[HTTP] server stopping");
        },
        _ = terminate => {
              tracing::info!("[HTTP] server stopping");
        },
    }
}


