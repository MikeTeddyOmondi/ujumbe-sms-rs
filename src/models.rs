use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MessageBag {
    pub numbers: String,
    pub message: String,
    pub sender: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MessageBagContainer {
    pub message_bag: MessageBag,
}

#[derive(Debug, Default, Serialize, Clone)]
pub struct MessageRequest {
    pub data: Vec<MessageBagContainer>,
}

impl MessageRequest {
    pub fn new() -> Self {
        MessageRequest { data: Vec::new() }
    }

    pub fn add_message_bag(&mut self, numbers: String, message: String, sender: String) {
        let bag = MessageBag {
            numbers,
            message,
            sender,
        };
        let container = MessageBagContainer { message_bag: bag };
        self.data.push(container);
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DateTime {
    pub date: String,
    pub timezone_type: i32,
    pub timezone: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MetaInfo {
    pub recipients: i32,
    pub credits_deducted: i32,
    pub available_credits: String,
    pub user_email: String,
    pub date_time: DateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StatusInfo {
    pub code: String,
    pub r#type: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiResponse {
    pub status: StatusInfo,
    pub meta: Option<MetaInfo>,
}
