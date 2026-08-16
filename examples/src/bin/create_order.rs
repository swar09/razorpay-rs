use razorpay::{Creatable, RazorpayClient, models::CreateOrderRequest};
use std::{collections::HashMap, env};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let key = env::var("API_KEY").unwrap_or_else(|_| "rzp_test_key".into());
    let secret = env::var("SECRET").unwrap_or_else(|_| "test_secret".into());

    let client = RazorpayClient::new(key, secret)?;

    let mut notes = HashMap::new();
    notes.insert("customer_tier".to_string(), "premium".to_string());
    notes.insert("purpose".to_string(), "Annual Subscription".to_string());

    let req = CreateOrderRequest {
        amount: 49900, // 499.00 INR in paisa
        currency: "INR".to_string(),
        receipt: Some("receipt_#12345".to_string()),
        partial_payment: Some(false),
        first_payment_min_amount: None,
        transfers: None,
        notes: Some(notes.into()),
    };

    println!("Sending CreateOrder request to Razorpay API...");
    match client.orders().create(req, None).await {
        Ok(order) => {
            println!("Order successfully created!");
            println!("Order ID: {}", order.id);
            println!("Amount: {} {}", order.amount, order.currency);
            println!("Status: {:?}", order.status);
        }
        Err(e) => {
            eprintln!("Failed to create order: {e}");
        }
    }

    Ok(())
}
