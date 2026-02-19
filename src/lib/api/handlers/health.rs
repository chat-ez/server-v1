use http_body_util::Full;
use hyper::{body::Bytes, Response, StatusCode};
use tracing::instrument;

use crate::api::shapes::error::ServerError;

#[instrument]
/// Handles a request for /health
/// This will just return an Ok and does not really do any validation that the
/// server itself is actually healthy
pub(crate) async fn handle_request() -> Result<Response<Full<Bytes>>, ServerError> {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(Full::new(Bytes::from("Server is Healthy")))
        .map_err(|_| ServerError::HealthError)?)
}
