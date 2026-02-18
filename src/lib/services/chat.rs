use std::sync::Arc;
use std::time::Duration;

use axum::routing::post;
use axum::{body::Bytes, extract::Request, response::Response};
use axum::{routing::get, Router};
use hyper::HeaderMap;
use tokio::sync::Mutex;
use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};
use tracing::Span;

use crate::handlers;
use crate::models::room::Room;

#[derive(Debug, Clone)]
pub struct ServerState {
    pub(crate) rooms: Arc<Mutex<Vec<Room>>>,
}

impl Default for ServerState {
    fn default() -> Self {
        let room1 = Room::new("Bing".to_string());
        let room2 = Room::new("Bong".to_string());
        let rooms = vec![room1, room2];
        Self {
            rooms: Arc::new(Mutex::new(rooms)),
        }
    }
}

pub fn router(app: ServerState) -> Router {
    Router::new()
        .route("/health", get(handlers::health::handle_request))
        .route("/rooms", get(handlers::room::get_rooms))
        .route("/rooms", post(handlers::room::create_room))
        .with_state(app)
        // Configures tracing layer to log requests and responses in the log file
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|_request: &Request<_>| tracing::debug_span!("http-request"))
                .on_request(|request: &Request<_>, _span: &Span| {
                    tracing::debug!("started {} {}", request.method(), request.uri().path())
                })
                .on_response(|_response: &Response<_>, latency: Duration, _span: &Span| {
                    tracing::debug!("response generated in {:?}", latency)
                })
                .on_body_chunk(|chunk: &Bytes, _latency: Duration, _span: &Span| {
                    tracing::debug!("sending {} bytes", chunk.len())
                })
                .on_eos(
                    |_trailers: Option<&HeaderMap>, stream_duration: Duration, _span: &Span| {
                        tracing::debug!("stream closed after {:?}", stream_duration)
                    },
                )
                .on_failure(
                    |error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
                        tracing::error!("Server encountered an error {error:#?}")
                    },
                ),
        )
}
