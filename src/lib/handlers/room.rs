use std::convert::Infallible;

use axum::{extract::State, Json};
use http_body_util::Full;
use hyper::{body::Bytes, Response, StatusCode};
use serde::Deserialize;
use tracing::instrument;

use crate::{models::room::Room, services::chat::ServerState};

#[derive(Deserialize, Debug)]
pub struct CreateRoomRequest {
    room_name: String,
}

#[instrument]
pub(crate) async fn get_rooms(
    State(app): State<ServerState>,
) -> Result<Response<Full<Bytes>>, Infallible> {
    let guard = app.rooms.lock().await;
    let rooms = guard.iter().cloned().collect::<Vec<Room>>();

    let json_bytes = serde_json::to_vec(&rooms).unwrap();

    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(Full::new(Bytes::from(json_bytes)))
        .unwrap())
}

#[instrument]
pub(crate) async fn create_room(
    State(app): State<ServerState>,
    Json(request): Json<CreateRoomRequest>,
) -> Response<Full<Bytes>> {
    let mut guard = app.rooms.lock().await;

    // Validate the room does not already exist
    if guard.iter().any(|r| r.name == request.room_name) {
        return Response::builder()
            .status(StatusCode::CONFLICT)
            .body(Full::new(Bytes::from(
                "Room with this name already exists!",
            )))
            .unwrap();
    }

    let room = Room::new(request.room_name);

    guard.push(room);

    Response::builder()
        .status(StatusCode::CREATED)
        .body(Full::new(Bytes::from("Room created successfully")))
        .unwrap()
}
