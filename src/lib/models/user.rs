use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub(crate) struct User {
    uuid: String,
    user_name: String,
}

impl User {
    pub(crate) fn new(user_name: String) -> Self {
        Self {
            uuid: Uuid::new_v4().to_string(),
            user_name,
        }
    }
}
