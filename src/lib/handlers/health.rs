use std::convert::Infallible;

use axum::extract::State;
use http_body_util::Full;
use hyper::{Response, StatusCode, body::Bytes};
use tracing::instrument;

use crate::services::chat::ServerState;

#[instrument]
/// Handles a request for /health
/// This will just return an Ok and does not really do any validation that the
/// server itself is actually healthy
pub(crate) async fn handle_request(
    State(_app): State<ServerState>,
) -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(Full::new(Bytes::from("Server is Healthy")))
        .unwrap())
}
