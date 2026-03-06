use axum::{body::Body, response::Response};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};

use crate::api::shapes::error::ServerError;

/// Struct used when sending a post /users requests to the server in order to create a user
/// instance on the server
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PostUserRequest {
    /// public key that the server will store on behalf of user
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PostUserResponse {
    pub uuid: String,
    pub message: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PutUserRequest;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PutUserResponse;
