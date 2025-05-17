# UjumbeSMS - Rust Client Library

A robust and modular client library for the UjumbeSMS API, allowing you to easily send SMS messages from your Rust applications.

## Features

- Simple and intuitive API for sending SMS messages
- Support for sending multiple messages in a single request
- Proper error handling and reporting
- Configurable base URL and other options
- Comprehensive type definitions for all API requests and responses

## Installation

Add the library to your `Cargo.toml`:

```toml
[dependencies]
ujumbe_sms = "1.0.0" # Check for the latest version on `crates.io/crates/ujumbe_sms`
```

## Quick Start

```rust
use ujumbe_sms::{UjumbeSmsClient, UjumbeSmsConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create configuration
    let config = UjumbeSmsConfig::new(
        "your_api_key".to_string(),
        "your@email.com".to_string(),
    );

    // Initialize client
    let client = UjumbeSmsClient::new(config)?;

    // Send a simple message
    let response = client.send_single_message(
        "254712345678", // must start with 254 country code...
        "Hello from UjumbeSMS!",
        "UjumbeSMS" // "UjumbeSMS" is the default sender; else add your own
    ).await?;

    println!("Message sent! Response: {:?}", response);
    println!("Credits remaining: {}", response.meta.available_credits);

    Ok(())
}
```

## Usage Guide

### Configuration

Create a configuration with your API credentials:

```rust
let config = UjumbeSmsConfig::new(
    "your_api_key".to_string(),
    "your_email@example.com".to_string(),
);

// Optionally configure base URL
let config = UjumbeSmsConfig::new(
    "your_api_key".to_string(),
    "your@email.com".to_string(),
).with_base_url("https://api.ujumbe.co.ke".to_string()); // configure url incase UjumbeSMS changes their API URL
```

### Creating a Client

Initialize a client with your configuration:

```rust
let client = UjumbeSmsClient::new(config)?;
```

### Sending a Single Message

Use the convenience method to send a single message to one or more recipients:

```rust
let response = client.send_single_message(
    "254712345678,254712345679",  // Multiple numbers separated by commas
    "Message content...",
    "SENDER_ID"  // Your registered sender ID or default "UjumbeSMS"
).await?;

println!("Message sent to {} recipients", response.meta.recipients);
```

### Sending Multiple Messages

Create and send multiple message bags in a single request:

```rust
use ujumbe_sms::MessageRequest;

let mut request = MessageRequest::new();

// Add first message bag
request.add_message_bag(
    "254712345678".to_string(),
    "First message content".to_string(),
    "SENDER_ID".to_string(),
);

// Add second message bag with different content or recipients
request.add_message_bag(
    "254712345679,254712345679".to_string(),
    "Second message content".to_string(),
    "SENDER_ID".to_string(),
);

let response = client.send_messages(request).await?;
```

### Error Handling

The library provides detailed error information through the `UjumbeSmsError` type:

```rust
match client.send_single_message("0712345678", "Test message", "SENDER").await {
    Ok(response) => {
        println!("Message sent successfully!");
        println!("Status: {}", response.status.description);
        println!("Recipients: {}", response.meta.recipients);
        println!("Credits used: {}", response.meta.credits_deducted);
        println!("Credits remaining: {}", response.meta.available_credits);
    },
    Err(err) => match err {
        UjumbeSmsError::NetworkError(e) => println!("Network error: {}", e),
        UjumbeSmsError::ApiError(code, desc) => println!("API error {}: {}", code, desc),
        UjumbeSmsError::SerializationError(e) => println!("Serialization error: {}", e),
        UjumbeSmsError::InvalidConfig(msg) => println!("Configuration error: {}", msg),
    }
}
```

## API Reference

### UjumbeSmsConfig

```rust
struct UjumbeSmsConfig {
    pub api_key: String,
    pub email: String,
    pub base_url: String,
}

impl UjumbeSmsConfig {
    pub fn new(api_key: String, email: String) -> Self;
    pub fn with_base_url(self, base_url: String) -> Self;
}
```

### UjumbeSmsClient

```rust
struct UjumbeSmsClient {
    // ...
}

impl UjumbeSmsClient {
    pub fn new(config: UjumbeSmsConfig) -> Result<Self, UjumbeSmsError>;

    pub async fn send_messages(
        &self,
        request: MessageRequest
    ) -> Result<ApiResponse, UjumbeSmsError>;

    pub async fn send_single_message(
        &self,
        numbers: &str,
        message: &str,
        sender: &str,
    ) -> Result<ApiResponse, UjumbeSmsError>;
}
```

### MessageRequest

```rust
struct MessageRequest {
    pub data: Vec<MessageBagContainer>,
}

impl MessageRequest {
    pub fn new() -> Self;

    pub fn add_message_bag(
        &mut self,
        numbers: String,
        message: String,
        sender: String
    );
}
```

### API Response Types

```rust
struct ApiResponse {
    pub status: StatusInfo,
    pub meta: Option<MetaInfo>,
}

struct StatusInfo {
    pub code: String,
    pub r#type: String,
    pub description: String,
}

struct MetaInfo {
    pub recipients: i32,
    pub credits_deducted: i32,
    pub available_credits: String,
    pub user_email: String,
    pub date_time: DateTime,
}

struct DateTime {
    pub date: String,
    pub timezone_type: i32,
    pub timezone: String,
}
```

### Error Types

```rust
enum UjumbeSmsError {
    NetworkError(reqwest::Error),
    ApiError(String, String), // code, description
    SerializationError(serde_json::Error),
    InvalidConfig(String),
}
```

## License

This library is licensed under the [MIT](./LICENSE.md) License.
