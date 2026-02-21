use serde::{Deserialize, Serialize};

use crate::models::user::User;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Message(User, String);
