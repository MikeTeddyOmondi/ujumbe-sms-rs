use crate::config::UjumbeSmsConfig;
use crate::errors::UjumbeSmsError;
use crate::models::{
    BalanceApiResponse, MessageHistoryApiResponse, MessageRequest, MessagingApiResponse,
};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use reqwest::Client as ReqwestClient;

/// UjumbeSMS Rust client for sending messages using the UjumbeSMS API
/// Crate: https://crates.io/crates/ujumbe_sms
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

    /// Internal method to attach headers with `UjumbeSmsConfig` configurations set
    fn attach_headers(&self) -> Result<HeaderMap, UjumbeSmsError> {
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

        Ok(headers)
    }

    /// Sends messages using the UjumbeSMS API: https://ujumbesms.co.ke/api/messaging
    pub async fn send_messages(
        &self,
        request: MessageRequest,
    ) -> Result<MessagingApiResponse, UjumbeSmsError> {
        let url = format!(
            "{}{}",
            self.config.base_url,
            ApiEndpoint::Messaging.as_str()
        );

        let headers = self.attach_headers()?;

        let response = self
            .http_client
            .post(&url)
            .headers(headers)
            .json(&request)
            .send()
            .await?;

        if response.status().is_success() {
            let api_response = response.json::<MessagingApiResponse>().await?;
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
    ) -> Result<MessagingApiResponse, UjumbeSmsError> {
        let mut request = MessageRequest::new();
        request.add_message_bag(numbers.to_string(), message.to_string(), sender.to_string());
        self.send_messages(request).await
    }

    /// Credit balance inquiry: https://ujumbesms.co.ke/api/balance
    pub async fn balance(&self) -> Result<BalanceApiResponse, UjumbeSmsError> {
        let url = format!("{}{}", self.config.base_url, ApiEndpoint::Balances.as_str());

        let headers = self.attach_headers()?;

        let response = self.http_client.post(&url).headers(headers).send().await?;

        if response.status().is_success() {
            let api_response = response.json::<BalanceApiResponse>().await?;
            Ok(api_response)
        } else {
            let status = response.status().to_string();
            let error_text = response.text().await?;
            Err(UjumbeSmsError::ApiError(status, error_text))
        }
    }

    /// Get messages history: https://ujumbesms.co.ke/api/messages
    pub async fn get_messages_history(&self) -> Result<MessageHistoryApiResponse, UjumbeSmsError> {
        let url = format!("{}{}", self.config.base_url, ApiEndpoint::Messages.as_str());

        let headers = self.attach_headers()?;

        let response = self.http_client.post(&url).headers(headers).send().await?;

        if response.status().is_success() {
            let api_response = response.json::<MessageHistoryApiResponse>().await?;
            Ok(api_response)
        } else {
            let status = response.status().to_string();
            let error_text = response.text().await?;
            Err(UjumbeSmsError::ApiError(status, error_text))
        }
    }
}

/// `ApiEndpoint` Enum representation of UjumbeSMS API endpoints
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApiEndpoint {
    Messaging,
    Balances,
    Messages,
}

impl ApiEndpoint {
    pub fn as_str(&self) -> &'static str {
        match self {
            ApiEndpoint::Messaging => "/api/messaging",
            ApiEndpoint::Balances => "/api/balance",
            ApiEndpoint::Messages => "/api/messages",
        }
    }
}

impl std::str::FromStr for ApiEndpoint {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "/api/messaging" => Ok(ApiEndpoint::Messaging),
            "/api/balance" => Ok(ApiEndpoint::Balances),
            "/api/messages" => Ok(ApiEndpoint::Messages),
            _ => Err(()),
        }
    }
}
