use crate::config::UjumbeSmsConfig;
use crate::errors::UjumbeSmsError;
use crate::models::{ApiResponse, MessageRequest};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use reqwest::Client as ReqwestClient;

const API_MESSAGING_ENDPOINT: &str = "/api/messaging";

pub struct UjumbeSmsClient {
    config: UjumbeSmsConfig,
    http_client: ReqwestClient,
}

impl UjumbeSmsClient {
    /// Creates a new UjumbeSMS client with the given configuration
    pub fn new(config: UjumbeSmsConfig) -> Result<Self, UjumbeSmsError> {
        let http_client = ReqwestClient::builder()
            .build()
            .map_err(UjumbeSmsError::from)?;

        Ok(UjumbeSmsClient {
            config,
            http_client,
        })
    }

    /// Sends messages using the UjumbeSMS API
    pub async fn send_messages(
        &self,
        request: MessageRequest,
    ) -> Result<ApiResponse, UjumbeSmsError> {
        let url = format!("{}{}", self.config.base_url, API_MESSAGING_ENDPOINT);

        let mut headers = HeaderMap::new();
        headers.insert(
            "X-Authorization",
            HeaderValue::from_str(&self.config.api_key)
                .map_err(|_| UjumbeSmsError::InvalidConfig("Invalid API key format".to_string()))?,
        );
        headers.insert(
            "Email",
            HeaderValue::from_str(&self.config.email)
                .map_err(|_| UjumbeSmsError::InvalidConfig("Invalid email format".to_string()))?,
        );
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert("Cache-Control", HeaderValue::from_static("no-cache"));

        let response = self
            .http_client
            .post(&url)
            .headers(headers)
            .json(&request)
            .send()
            .await?;

        if response.status().is_success() {
            let api_response = response.json::<ApiResponse>().await?;
            Ok(api_response)
        } else {
            // Store the status before consuming the response with text()
            let status = response.status().to_string();
            let error_text = response.text().await?;
            Err(UjumbeSmsError::ApiError(status, error_text))
        }
    }

    /// Convenience method to send a single message to multiple recipients
    pub async fn send_single_message(
        &self,
        numbers: &str,
        message: &str,
        sender: &str,
    ) -> Result<ApiResponse, UjumbeSmsError> {
        let mut request = MessageRequest::new();
        request.add_message_bag(numbers.to_string(), message.to_string(), sender.to_string());
        self.send_messages(request).await
    }
}
