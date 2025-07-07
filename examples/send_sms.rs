use ujumbe_sms::{UjumbeSmsClient, UjumbeSmsConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    // Load .env variables
    let api_key = std::env::var("UJUMBESMS_API_KEY")?;
    let email = std::env::var("UJUMBESMS_EMAIL")?;

    // Create configuration
    let config = UjumbeSmsConfig::new(
        api_key, // "ZDIzNTFm....".to_string()
        email, // "user@example.com".to_string()
    );

    // Initialize client
    let client = UjumbeSmsClient::new(config)?;

    // Use the convenience method for a single message with mutiple recipients separated with commas
    let response = client
        .send_single_message(
            "2547XXXXXXXX", // "254712345678,254711223344", // Should be a full number and comma separated for multiple
            "UjumbeSMS - message from Rust client library made by mt0.dev!",
            "UjumbeSMS",
        )
        .await?;

    println!("Single message response: {:#?}", response);

    Ok(())
}
