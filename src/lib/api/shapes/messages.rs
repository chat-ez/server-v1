use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct PostMessageRequest;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct PostMessageResponse;

impl IntoResponse for PostMessageResponse {
    fn into_response(self) -> axum::response::Response {
        todo!()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct PutMessageRequest;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct PutMessageResponse;

impl IntoResponse for PutMessageResponse {
    fn into_response(self) -> axum::response::Response {
        todo!()
    }
}
