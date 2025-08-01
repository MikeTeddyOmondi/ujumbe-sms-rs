use std::error::Error;
use std::fmt;

/// Custom error type for UjumbeSMS client
/// This error type encapsulates various errors that can occur while using the UjumbeSMS API
/// and provides a way to handle them in a consistent manner.
#[derive(Debug)]
pub enum UjumbeSmsError {
    NetworkError(reqwest::Error),
    ApiError(String, String), // code, description
    SerializationError(serde_json::Error),
    InvalidConfig(String),
}

impl fmt::Display for UjumbeSmsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UjumbeSmsError::NetworkError(e) => write!(f, "Network error: {e}"),
            UjumbeSmsError::ApiError(code, desc) => write!(f, "API error {code}: {desc}"),
            UjumbeSmsError::SerializationError(e) => write!(f, "Serialization error: {e}"),
            UjumbeSmsError::InvalidConfig(msg) => write!(f, "Invalid configuration: {msg}"),
        }
    }
}

impl Error for UjumbeSmsError {}

impl From<reqwest::Error> for UjumbeSmsError {
    fn from(error: reqwest::Error) -> Self {
        UjumbeSmsError::NetworkError(error)
    }
}

impl From<serde_json::Error> for UjumbeSmsError {
    fn from(error: serde_json::Error) -> Self {
        UjumbeSmsError::SerializationError(error)
    }
}
