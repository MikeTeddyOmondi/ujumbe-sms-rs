pub mod client;
pub mod config;
pub mod errors;
pub mod models;

pub use client::UjumbeSmsClient;
pub use config::UjumbeSmsConfig;
pub use errors::UjumbeSmsError;
pub use models::{
    BalanceApiResponse, BalanceMetaInfo, DateTime, MessageBag, MessageHistoryApiResponse,
    MessageHistoryMetaInfo, MessageRequest, MessagingApiResponse, MessagingMetaInfo, StatusInfo,
};

/// UjumbeSMS Rust Client lib tests
#[cfg(test)]
mod tests {
    use crate::{MessageRequest, UjumbeSmsClient, UjumbeSmsConfig};
    use mockito::{Matcher, Server};

    // Use #[test] instead of #[tokio::test]
    #[test]
    fn test_send_single_message() {
        // Request a new server from the pool
        let mut server = Server::new();

        // Use one of these addresses to configure your client
        // let host = server.host_with_port();
        let url = server.url();

        // Use tokio::runtime::Runtime to create a runtime manually
        let rt = tokio::runtime::Runtime::new().unwrap();

        // Execute the async code within the runtime
        rt.block_on(async {
            // Create a mock endpoint
            let _mock = server
                .mock("POST", "/api/messaging")
                .with_status(200)
                .with_header("content-type", "application/json")
                .with_body(
                    r#"{
                    "status": {
                        "code": "1008",
                        "type": "success",
                        "description": "Your messages have been queued"
                    },
                    "meta": {
                        "recipients": 1,
                        "credits_deducted": 1,
                        "available_credits": "6608",
                        "user_email": "test@email.com",
                        "date_time": {
                            "date": "20150815 18:19:47",
                            "timezone_type": 3,
                            "timezone": "Africa/Nairobi"
                        }
                    }
                }"#,
                )
                .create();

            // Create client with mock server URL
            let config =
                UjumbeSmsConfig::new("test_api_key".to_string(), "test@example.com".to_string())
                    .with_base_url(url);

            let client = UjumbeSmsClient::new(config).unwrap();

            // Test sending a single message
            let result = client
                .send_single_message("254712345678", "Single test message", "UjumbeSMS")
                .await;

            // Verify the mock was called
            _mock.assert();
            // println!("{:?}", &result);

            assert!(result.is_ok());
            let response = result.unwrap();
            assert_eq!(response.status.code, "1008");
            assert_eq!(response.status.r#type, "success");
            assert_eq!(response.meta.clone().unwrap().recipients, 1);
            assert_eq!(response.meta.clone().unwrap().credits_deducted, 1);
            assert_eq!(response.meta.clone().unwrap().user_email, "test@email.com");
            assert_eq!(response.meta.clone().unwrap().available_credits, "6608");
        });
    }

    #[test]
    fn test_send_multiple_messages() {
        // Request a new server from the pool
        let mut server = Server::new();

        // Use one of these addresses to configure your client
        // let host = server.host_with_port();
        let url = server.url();

        let rt = tokio::runtime::Runtime::new().unwrap();

        rt.block_on(async {
            // Create a mock endpoint
            let _mock = server
                .mock("POST", "/api/messaging")
                .with_status(200)
                .with_header("content-type", "application/json")
                .with_body(
                    r#"{
                    "status": {
                        "code": "1008",
                        "type": "success",
                        "description": "Your messages have been queued"
                    },
                    "meta": {
                        "recipients": 3,
                        "credits_deducted": 3,
                        "available_credits": "6605",
                        "user_email": "test@email.com",
                        "date_time": {
                            "date": "20150815 18:19:47",
                            "timezone_type": 3,
                            "timezone": "Africa/Nairobi"
                        }
                    }
                }"#,
                )
                .create();

            // Create client with mock server URL
            let config =
                UjumbeSmsConfig::new("test_api_key".to_string(), "test@email.com".to_string())
                    .with_base_url(url);

            let client = UjumbeSmsClient::new(config).unwrap();

            // Create message request with multiple bags
            let mut request = MessageRequest::new();
            request.add_message_bag(
                "254712345678".to_string(),
                "First test message".to_string(),
                "UjumbeSMS".to_string(),
            );
            request.add_message_bag(
                "254712345679,254712345679".to_string(),
                "Second test message".to_string(),
                "UjumbeSMS".to_string(),
            );

            // Test sending multiple messages
            let result = client.send_messages(request).await;

            // Verify the mock was called
            _mock.assert();
            // println!("{:?}", &result);

            assert!(result.is_ok());
            let response = result.unwrap();
            assert_eq!(response.status.code, "1008");
            assert_eq!(response.status.r#type, "success");
            assert_eq!(response.meta.clone().unwrap().recipients, 3);
            assert_eq!(response.meta.clone().unwrap().credits_deducted, 3);
            assert_eq!(response.meta.clone().unwrap().available_credits, "6605");
            assert_eq!(response.meta.clone().unwrap().user_email, "test@email.com");
        });
    }

    #[test]
    fn test_error_handling() {
        // Request a new server from the pool
        let mut server = Server::new();

        // Use one of these addresses to configure your client
        // let host = server.host_with_port();
        let url = server.url();

        let rt = tokio::runtime::Runtime::new().unwrap();

        rt.block_on(async {
            // Create a mock endpoint
            let _mock = server
                .mock("POST", "/api/messaging")
                .with_status(400)
                .with_header("content-type", "application/json")
                .with_body(
                    r#"{
                    "status": {
                        "code": "1001",
                        "type": "error",
                        "description": "Invalid API credentials"
                    }
                }"#,
                )
                .create();

            // Create client with mock server URL
            let config = UjumbeSmsConfig::new(
                "invalid_api_key".to_string(),
                "test@example.com".to_string(),
            )
            .with_base_url(url);

            let client = UjumbeSmsClient::new(config).unwrap();

            // Test error handling
            let result = client
                .send_single_message("07123456789", "Error test message", "UjumbeSMS")
                .await;

            // Verify the mock was called
            _mock.assert();

            assert!(result.is_err());
            if let Err(error) = result {
                match error {
                    crate::errors::UjumbeSmsError::ApiError(code, desc) => {
                        assert_eq!(code, "400 Bad Request");
                        assert!(desc.contains("Invalid API credentials"));
                    }
                    _ => panic!("Expected ApiError variant"),
                }
            }
        });
    }

    #[test]
    fn test_request_validation() {
        let mut server = Server::new();
        let url = server.url();
        let rt = tokio::runtime::Runtime::new().unwrap();

        rt.block_on(async {
            let _mock = server
                .mock("POST", "/api/messaging")
                .match_header("X-Authorization", "test_api_key")
                .match_header("Email", "test@email.com")
                .match_header("Content-Type", "application/json")
                .match_body(Matcher::Json(serde_json::json!({
                    "data": [
                        {
                            "message_bag": {
                                "numbers": "254712345678",
                                "message": "Request validation test message",
                                "sender": "UjumbeSMS"
                            }
                        }
                    ]
                })))
                .with_status(200)
                .with_header("content-type", "application/json")
                .with_body(
                    r#"{
                    "status": {
                        "code": "1008",
                        "type": "success",
                        "description": "Your messages have been queued"
                    },
                    "meta": {
                        "recipients": 1,
                        "credits_deducted": 1,
                        "available_credits": "6608",
                        "user_email": "test@email.com",
                        "date_time": {
                            "date": "2025-05-03T12:34:56Z",
                            "timezone_type": 3,
                            "timezone": "UTC"
                        }
                    }
                }"#,
                )
                .create();

            let config =
                UjumbeSmsConfig::new("test_api_key".to_string(), "test@email.com".to_string())
                    .with_base_url(url);
            let client = UjumbeSmsClient::new(config).unwrap();

            let result = client
                .send_single_message(
                    "254712345678",
                    "Request validation test message",
                    "UjumbeSMS",
                )
                .await;

            _mock.assert();
            // println!("{:?}", &result);

            assert!(result.is_ok());
        });
    }

    #[test]
    fn test_balance_inquiry() {
        // Request a new server from the pool
        let mut server = Server::new();

        // Use one of these addresses to configure your client
        // let host = server.host_with_port();
        let url = server.url();

        // Use tokio::runtime::Runtime to create a runtime manually
        let rt = tokio::runtime::Runtime::new().unwrap();

        // Execute the async code within the runtime
        rt.block_on(async {
            // Create a mock endpoint
            let _mock = server
                .mock("POST", "/api/balance")
                .with_status(200)
                .with_header("content-type", "application/json")
                .with_body(
                    r#"{
                    "status": {
                        "code": "1008",
                        "type": "success",
                        "description": "Balance inquiry"
                    },
                    "meta": {
                        "user": "test@email.com",
                        "credits": 10,
                        "rate": 1,
                        "user_email": "test@email.com",
                        "date_time": {
                            "date": "20150815 18:19:47",
                            "timezone_type": 3,
                            "timezone": "Africa/Nairobi"
                        }
                    }
                }"#,
                )
                .create();

            // Create client with mock server URL
            let config =
                UjumbeSmsConfig::new("test_api_key".to_string(), "test@example.com".to_string())
                    .with_base_url(url);

            let client = UjumbeSmsClient::new(config).unwrap();

            // Test sending a single message
            let result = client
                .balance()
                .await;

            // Verify the mock was called
            _mock.assert();
            // println!("{:?}", &result);

            assert!(result.is_ok());
            let response = result.unwrap();
            assert_eq!(response.status.code, "1008");
            assert_eq!(response.status.r#type, "success");
            assert_eq!(response.meta.clone().unwrap().rate, 1);
            assert_eq!(response.meta.clone().unwrap().user, "test@email.com");
        });
    }

    #[test]
    fn test_get_messages_history() {
        // Request a new server from the pool
        let mut server = Server::new();

        // Use one of these addresses to configure your client
        // let host = server.host_with_port();
        let url = server.url();

        // Use tokio::runtime::Runtime to create a runtime manually
        let rt = tokio::runtime::Runtime::new().unwrap();

        // Execute the async code within the runtime
        rt.block_on(async {
            // Create a mock endpoint
            let _mock = server
                .mock("POST", "/api/messages")
                .with_status(200)
                .with_header("content-type", "application/json")
                .with_body(
                    r#"{
                    "status": {
                        "code": "1008",
                        "type": "success",
                        "description": "Your messages have been queued"
                    },
                    "meta": {
                        "recipients": 1,
                        "credits_deducted": 1,
                        "available_credits": "6608",
                        "user_email": "test@email.com",
                        "date_time": {
                            "date": "20150815 18:19:47",
                            "timezone_type": 3,
                            "timezone": "Africa/Nairobi"
                        }
                    }
                }"#,
                )
                .create();

            // Create client with mock server URL
            let config =
                UjumbeSmsConfig::new("test_api_key".to_string(), "test@example.com".to_string())
                    .with_base_url(url);

            let client = UjumbeSmsClient::new(config).unwrap();

            // Test sending a single message
            let result = client
                .send_single_message("254712345678", "Single test message", "UjumbeSMS")
                .await;

            // Verify the mock was called
            _mock.assert();
            // println!("{:?}", &result);

            assert!(result.is_ok());
            let response = result.unwrap();
            assert_eq!(response.status.code, "1008");
            assert_eq!(response.status.r#type, "success");
            assert_eq!(response.meta.clone().unwrap().recipients, 1);
            assert_eq!(response.meta.clone().unwrap().credits_deducted, 1);
            assert_eq!(response.meta.clone().unwrap().user_email, "test@email.com");
            assert_eq!(response.meta.clone().unwrap().available_credits, "6608");
        });
    }
}
