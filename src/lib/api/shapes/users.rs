use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct PostUserRequest;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct PostUserResponse;

impl IntoResponse for PostUserResponse {
    fn into_response(self) -> axum::response::Response {
        todo!()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct PutUserRequest;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct PutUserResponse;

impl IntoResponse for PutUserResponse {
    fn into_response(self) -> axum::response::Response {
        todo!()
    }
}
