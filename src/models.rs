use serde::{Deserialize, Serialize};

/// `MessageBag` represents the request structure for sending messages
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MessageBag {
    pub numbers: String,
    pub message: String,
    pub sender: String,
}

/// `MessageBagContainer` represents a container for the message bag
/// This is used to wrap the message bag in a list for the API request.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MessageBagContainer {
    pub message_bag: MessageBag,
}

/// `MessageRequest` represents the request structure for sending messages
/// This structure contains a list of message bags to be sent.
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

/// `DateTime` structure to represent the date and time information
/// returned by the API. This structure includes the date, timezone type,
/// and timezone string.
/// The `date` is in the format "YYYY-MM-DD HH:MM:SS".
/// The `timezone_type` is an integer that indicates the type of timezone.
/// The `timezone` is a string that represents the timezone, e.g., "Africa/Nairobi".  
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DateTime {
    pub date: String,
    pub timezone_type: i32,
    pub timezone: String,
}

/// `MetaInfo` structure to represent the metadata information
/// returned by the API. This structure includes the number of recipients,
/// credits deducted, available credits, user email, and date/time information.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MetaInfo {
    pub recipients: i32,
    pub credits_deducted: i32,
    pub available_credits: String,
    pub user_email: String,
    pub date_time: DateTime,
}

/// `StatusInfo` structure to represent the status information
/// returned by the API. This structure includes the status code,
/// type, and description.
/// The status code is a string that indicates the status of the request.
/// The type is a string that indicates the type of status.
/// The description is a string that provides a detailed description of the status.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StatusInfo {
    pub code: String,
    pub r#type: String,
    pub description: String,
}

/// `ApiResponse` structure to represent the response from the API.
/// This structure includes the status information and optional metadata.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiResponse {
    pub status: StatusInfo,
    pub meta: Option<MetaInfo>,
}
