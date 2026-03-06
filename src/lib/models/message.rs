use std::time::{Instant, SystemTime, UNIX_EPOCH};

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

use crate::api::shapes::messages::PostMessageRequest;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Message {
    uuid: String,
    user: String,
    /// Encrypted structure of the message, the server will just generate a uuid and store the
    /// message contents here. Clients will determine how to deserialize.
    content: String,
    timestamp: DateTime<Utc>,
}

impl Message {
    pub(crate) fn uuid(&self) -> &str {
        &self.uuid
    }

    pub(crate) fn content(&self) -> &str {
        &self.content
    }

    pub(crate) fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }
}

impl From<PostMessageRequest> for Message {
    fn from(value: PostMessageRequest) -> Self {
        let timestamp = DateTime::<Utc>::from(SystemTime::now());
        Self {
            uuid: uuid::Uuid::new_v4().to_string(),
            user: value.user_uuid,
            content: value.content.to_owned(),
            timestamp,
        }
    }
}
