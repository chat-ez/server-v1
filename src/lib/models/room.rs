use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::models::{message::Message, user::User};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Room {
    pub(crate) uuid: String,
    pub(crate) name: String,
    messages: Vec<Message>,
    users: HashSet<User>,
    description: String,
}

impl Room {
    pub(crate) fn new(name: String, description: String) -> Self {
        Self {
            uuid: uuid::Uuid::new_v4().to_string(),
            name,
            messages: Vec::new(),
            users: HashSet::new(),
            description,
        }
    }

    pub(crate) fn get_messages(&self) -> Vec<&Message> {
        self.messages.iter().collect()
    }

    pub(crate) fn new_message(&mut self, message: Message) {
        self.messages.push(message);
    }

    pub(crate) fn add_user(&mut self, user: User) {
        self.users.insert(user);
    }
}
