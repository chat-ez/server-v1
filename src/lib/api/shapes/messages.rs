use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PostMessageRequest {
    pub user_uuid: String,
    pub content: String,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PostMessageResponse {
    /// unique id of the message that was sent
    pub uuid: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PutMessageRequest;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PutMessageResponse;
