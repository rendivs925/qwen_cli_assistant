use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Clone)]
pub struct ChatSession {
    pub messages: Vec<Message>,
}

impl ChatSession {
    pub fn new(safe_mode: bool) -> Self {
        let system_prompt = if safe_mode {
            "You are an ultra-safe CLI assistant.              Convert natural language requests into POSIX shell commands.              Avoid destructive operations, never format disks, and avoid sudo.              When in doubt, prefer read-only commands and conservative actions."
        } else {
            "You are a CLI assistant that converts natural language requests into POSIX shell commands.              The user will review all commands before running."
        };

        let messages = vec![Message {
            role: "system".to_string(),
            content: system_prompt.to_string(),
        }];

        Self { messages }
    }

    pub fn push_user(&mut self, content: String) {
        self.messages.push(Message {
            role: "user".to_string(),
            content,
        });
    }

    pub fn push_assistant(&mut self, content: String) {
        self.messages.push(Message {
            role: "assistant".to_string(),
            content,
        });
    }
}
