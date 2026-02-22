use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Message {
    uuid: String,
    /// Encrypted structure of the message, the server will just generate a uuid and store the
    /// message contents here. Clients will determine how to deserialize.
    content: String,
    timestamp: DateTime<Utc>,
}
