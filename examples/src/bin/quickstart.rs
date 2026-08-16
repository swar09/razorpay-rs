use razorpay::RazorpayClient;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let key = env::var("API_KEY").unwrap_or_else(|_| "rzp_test_key".into());
    let secret = env::var("SECRET").unwrap_or_else(|_| "test_secret".into());

    let client = RazorpayClient::new(key, secret)?;
    println!(
        "Razorpay client initialized with base URL: {}",
        client.config().base_url
    );

    Ok(())
}
