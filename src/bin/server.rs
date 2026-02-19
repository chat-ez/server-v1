use std::net::SocketAddr;

use anyhow::Result;
use chat_ez::services::chat::{router, ServerState};
use tokio::net::TcpListener;
use tracing::info;
use tracing_appender::rolling;
use tracing_subscriber::{fmt, EnvFilter};

fn init_tracing(app_name: &str) -> tracing_appender::non_blocking::WorkerGuard {
    // Create directory: /tmp/__application_name__/
    let log_dir = format!("/tmp/{}", app_name);
    std::fs::create_dir_all(&log_dir).expect("Failed to create log directory");

    // Create hourly rolling file appender
    let file_appender = rolling::hourly(&log_dir, "log");

    // Make logging non-blocking
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    // Build subscriber
    let subscriber = fmt()
        .with_writer(non_blocking)
        .with_target(true)
        .with_level(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");

    guard
}

#[tokio::main]
async fn main() -> Result<()> {
    let _guard = init_tracing("ChatService");
    info!("Starting server");

    let app = ServerState::default();
    let router = router(app);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    axum::serve(TcpListener::bind(addr).await?, router).await?;

    info!("Stopping server");
    Ok(())
}
