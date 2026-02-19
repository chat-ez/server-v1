use std::convert::Infallible;

use axum::{
    Json,
    extract::{Path, State},
};
use http_body_util::Full;
use hyper::{Response, StatusCode, body::Bytes};
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

#[derive(Deserialize, Debug)]
pub(crate) struct AddUserRequest {
    user_name: String,
}

#[instrument]
pub(crate) async fn add_user_to_room(
    State(app): State<ServerState>,
    Path(room_name): Path<String>,
    Json(request): Json<AddUserRequest>,
) -> Result<Response<Full<Bytes>>, Infallible> {
    let mut guard = app.rooms.lock().await;

    // Find the room
    if let Some(room) = guard.iter_mut().find(|r| r.name == room_name) {
        // Create and add user to room
        let user = crate::models::user::User::new(request.user_name);
        room.add_user(user);

        Ok(Response::builder()
            .status(StatusCode::OK)
            .body(Full::new(Bytes::from("User added to room successfully")))
            .unwrap())
    } else {
        Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Full::new(Bytes::from("Room not found")))
            .unwrap())
    }
}
