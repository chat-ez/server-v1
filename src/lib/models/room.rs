use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::models::{message::Message, user::User};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Room {
    pub(crate) name: String,
    messages: Vec<Message>,
    users: HashSet<User>,
}

impl Room {
    pub(crate) fn new(name: String) -> Self {
        Self {
            name,
            messages: Vec::new(),
            users: HashSet::new(),
        }
    }

    pub(crate) fn get_messages(&self) -> Vec<&Message> {
        self.messages.iter().collect()
    }

    pub(crate) fn new_message(&mut self, message: Message) {
        self.messages.push(message);
    }
}
