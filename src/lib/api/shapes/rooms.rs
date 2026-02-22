use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PostRoomRequest {
    pub name: String,
    pub description: String,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PostRoomResponse {
    /// Unique id for the room
    pub uuid: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PutRoomRequest {
    /// Unique id for the room to update
    pub uuid: String,
    /// Pass a value here to change the name
    pub name: Option<String>,
    /// Pass a value here to change the description
    pub description: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PutRoomResponse {
    pub uuid: String,
}
