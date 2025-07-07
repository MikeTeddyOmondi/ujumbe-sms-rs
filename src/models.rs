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

/// `MessagingMetaInfo` structure to represent the metadata information
/// returned by the API. This structure includes the number of recipients,
/// credits deducted, available credits, user email, and date/time information.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MessagingMetaInfo {
    pub recipients: i32,
    pub credits_deducted: i32,
    pub available_credits: String,
    pub user_email: String,
    pub date_time: DateTime,
}

/// `BalanceMetaInfo` structure to represent the metadata information
/// returned by the API. This structure includes the number of recipients,
/// credits deducted, available credits, user email, and date/time information.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BalanceMetaInfo {
    pub user: String,
    pub credits: i32,
    pub rate: i32,
    pub date_time: DateTime,
}

/// `MessageHistoryMetaInfo` structure to represent the metadata information
/// returned by the API. This structure includes the number of recipients,
/// credits deducted, available credits, user email, and date/time information.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MessageHistoryMetaInfo {
    pub user: String,
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

/// `MessagingApiResponse` structure to represent the response from the Messaging API.
/// This structure includes the status information and optional metadata.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MessagingApiResponse {
    pub status: StatusInfo,
    pub meta: Option<MessagingMetaInfo>,
}

/// `BalanceApiResponse` structure to represent the response from the Balance API.
/// This structure includes the status information and optional metadata.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BalanceApiResponse {
    pub status: StatusInfo,
    pub meta: Option<BalanceMetaInfo>,
}

/// `MessageHistoryApiResponse` structure to represent the response from the Messages History API.
/// This structure includes the status information and optional metadata.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MessageHistoryApiResponse {
    pub status: StatusInfo,
    pub meta: Option<MessageHistoryMetaInfo>,
    pub items: Items,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Items {
    pub total: i32,
    pub per_page: i32,
    pub current_page: i32,
    pub last_page: i32,
    pub next_page_url: Option<String>,
    pub prev_page_url: Option<String>,
    pub from: i32,
    pub to: i32,
    pub data: Vec<MessageSent>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MessageSent {
    pub id: i64,
    pub request_id: i64,
    pub number: String,
    pub message: String,
    pub user_id: i32,
    pub sender_id: String,
    pub transaction_id: String,
    pub message_count: i32,
    pub status: String,
    pub flag: String,
    pub created_at: String,
    pub updated_at: String,
    pub scheduled_date: String,
}

// Example usage:
impl MessageHistoryApiResponse {
    /// Parse JSON string into MessageHistoryApiResponse
    pub fn from_json(json_str: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json_str)
    }

    /// Convert MessageHistoryApiResponse to JSON string
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Get all successfully delivered messages
    pub fn get_delivered_messages(&self) -> Vec<&MessageSent> {
        self.items
            .data
            .iter()
            .filter(|msg| msg.status == "DeliveredToTerminal")
            .collect()
    }

    /// Get all failed messages
    pub fn get_failed_messages(&self) -> Vec<&MessageSent> {
        self.items
            .data
            .iter()
            .filter(|msg| msg.status.contains("Blacklisted") || msg.status.contains("Invalid"))
            .collect()
    }

    /// Get messages by phone number
    pub fn get_messages_by_number(&self, number: &str) -> Vec<&MessageSent> {
        self.items
            .data
            .iter()
            .filter(|msg| msg.number == number)
            .collect()
    }
}
