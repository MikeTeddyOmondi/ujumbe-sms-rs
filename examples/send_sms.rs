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

    // Option 1: Use the convenience method for a single message
    let response = client
        .send_single_message(
            "2547XXXXXXXX", // "254712345678,254711223344", // Should be a full number and comma separated for multiple
            "UjumbeSMS - message from Rust client library made by mt0.dev!",
            "UjumbeSMS",
        )
        .await?;

    println!("Single message response: {:?}", response);

    // Option 2: Create and send multiple message bags
    let mut request = ujumbe_sms::MessageRequest::new();

    // First message bag
    request.add_message_bag(
        "2547XXXXXXX".to_string(),
        "Hi Mom! Its me testing SMS sending using a library".to_string(),
        "UjumbeSMS".to_string(),
    );

    // Second message bag with different message content
    request.add_message_bag(
        "2547XXXXXXXX,2547XXXXXXXX".to_string(), // "254712345678,254711223344", // Should be a full number and comma separated for multiple
        "UjumbeSMS - message from Rust client library made by mt0.dev!".to_string(),
        "UjumbeSMS".to_string(),
    );

    let response = client.send_messages(request).await?;
    println!("Multiple message response: {:?}", response);

    Ok(())
}
