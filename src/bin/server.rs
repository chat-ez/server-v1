use std::{net::SocketAddr, time::Duration};

use anyhow::Result;
use chat_ez::{
    api::shapes::{messages::PostMessageRequest, rooms::PostRoomRequest, users::PostUserRequest},
    services::chat::{router, ServerState},
};
use hyper::StatusCode;
use serde_json::json;
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

    let _ = tokio::task::spawn(pre_populate());
    axum::serve(TcpListener::bind(addr).await?, router).await?;

    info!("Stopping server");
    Ok(())
}

/// Utility function to pre-populate server during development
async fn pre_populate() {
    // Give tcp listener some time to actually set itself up
    tokio::time::sleep(Duration::from_secs(1)).await;
    let client = reqwest::Client::new();
    if client
        .get("http://localhost:3000/health")
        .send()
        .await
        .unwrap()
        .status()
        .eq(&StatusCode::OK)
    {
        for (room, users, messages) in [
            (
                "General",
                vec!["Bob", "Janice"],
                vec![("Bob", "hey"), ("Janice", "whats up?")],
            ),
            (
                "Shit Talk",
                vec!["Bob", "Janice"],
                vec![
                    ("Bob", "There's no toilet paper in the bathroom"),
                    ("Janice", "That sucks bob"),
                ],
            ),
            (
                "Memes",
                vec!["Bob", "Janice", "Chet"],
                vec![
                    ("Bob", "Yo check out this meme: meme.jpg"),
                    ("Janice", "Haha good one"),
                    ("Bob", "Thanks"),
                    ("Chet", "Look at what i found seriousmeme.gif"),
                    ("Bob", "Hhaha"),
                    ("Bob", "I saw that one yesterday"),
                ],
            ),
        ] {
            let payload = PostRoomRequest {
                name: room.to_string(),
                description: "".to_string(),
            };

            let _ = client
                .post("http://localhost:3000/rooms")
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&payload).unwrap())
                .send()
                .await
                .inspect(|r| info!("{room} created with status: {}", r.status()));

            for user in users {
                let payload = PostUserRequest {
                    name: user.to_string(),
                };

                let _ = client
                    .post(format!("http://localhost:3000/rooms/{}/users", room))
                    .header("Content-Type", "application/json")
                    .body(serde_json::to_string(&payload).unwrap())
                    .send()
                    .await
                    .inspect(|r| info!("{user} created with status: {}", r.status()));
            }

            for message in messages {
                let payload = PostMessageRequest {
                    user_uuid: message.0.to_string(),
                    content: message.1.to_string(),
                };

                let _ = client
                    .post(format!("http://localhost:3000/rooms/{}/messages", room))
                    .header("Content-Type", "application/json")
                    .body(serde_json::to_string(&payload).unwrap())
                    .send()
                    .await
                    .inspect(|r| info!("{} created with status: {}", message.1, r.status()));
            }
        }
    }
}
