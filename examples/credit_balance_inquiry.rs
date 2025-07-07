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
        email,   // "user@example.com".to_string()
    );

    // Initialize client
    let client = UjumbeSmsClient::new(config)?;

    let response = client.balance().await?;

    println!("Credit balance Inquiry: {:#?}", response);

    Ok(())
}
