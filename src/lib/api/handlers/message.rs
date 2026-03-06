use std::convert::Infallible;

use axum::{
    extract::{Path, State},
    Json,
};
use http_body_util::Full;
use hyper::{body::Bytes, Response, StatusCode};
use tracing::instrument;

use crate::{
    api::shapes::{
        error::ServerError,
        messages::{PostMessageRequest, PostMessageResponse},
    },
    models::message::Message,
    services::chat::ServerState,
};

#[instrument]
pub(crate) async fn post_message(
    State(app): State<ServerState>,
    Path(room_name): Path<String>,
    Json(request): Json<PostMessageRequest>,
) -> Result<Response<Full<Bytes>>, ServerError> {
    let mut guard = app.rooms.lock().await;
    if let Some(room) = guard.iter_mut().find(|r| r.name == room_name) {
        let message = Message::from(request);
        room.new_message(message.clone());

        let response = PostMessageResponse {
            uuid: message.uuid().to_string(),
            timestamp: message.timestamp(),
        };
        let json_bytes = serde_json::to_vec(&response)?;

        Ok(Response::builder()
            .status(StatusCode::CREATED)
            .body(Full::new(Bytes::from(json_bytes)))
            .unwrap())
    } else {
        Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Full::new(Bytes::from("Room not found")))
            .unwrap())
    }
}
