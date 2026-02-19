use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct PostRoomRequest {
    name: String,
    description: String,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct PostRoomResponse;

impl IntoResponse for PostRoomResponse {
    fn into_response(self) -> axum::response::Response {
        todo!()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct PutRoomRequest {
    name: Option<String>,
    description: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct PutRoomResponse;

impl IntoResponse for PutRoomResponse {
    fn into_response(self) -> axum::response::Response {
        todo!()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct JoinRoomRequest {
    user_id: String,
    room_id: String,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct JoinRoomResponse;

impl IntoResponse for JoinRoomResponse {
    fn into_response(self) -> axum::response::Response {
        todo!()
    }
}
