use axum::{body::Body, response::IntoResponse};
use hyper::{Response, StatusCode};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServerError {
    /// Thrown by the /health API when the service is not healthy or the Response constructor
    /// fails
    #[error("The Service returned an error indiciating it's unhealthy")]
    Health,

    #[error("Error during response construction")]
    ResponseConstruction,

    #[error("Json parser error {0}")]
    JsonParse(#[from] serde_json::Error),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> axum::response::Response {
        Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from(self.to_string()))
            .unwrap_or_default()
    }
}
