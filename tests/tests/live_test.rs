use razorpay::{
    models::{
        CreateCustomerRequest, CreateOrderRequest, CreatePaymentLinkRequest, CreatePlanItem,
        CreatePlanRequest, CreateSubscriptionRequest, ListOptions, PlanPeriod,
    },
    Creatable, Fetchable, Listable, RazorpayClient,
};
use std::env;

/// Helper function to load credentials and return a live client.
/// Returns None if valid credentials are not configured in .env or environment.
fn get_live_client() -> Option<RazorpayClient> {
    dotenvy::dotenv().ok();

    let key_id = env::var("RAZORPAY_KEY_ID")
        .or_else(|_| env::var("API_KEY"))
        .ok()?;
    let key_secret = env::var("RAZORPAY_KEY_SECRET")
        .or_else(|_| env::var("SECRET"))
        .ok()?;

    if key_id.trim().is_empty()
        || key_secret.trim().is_empty()
        || key_id.starts_with("test_key")
        || key_id == "rzp_test_key"
    {
        eprintln!("Skipping live API test: RAZORPAY_KEY_ID / SECRET not configured with real keys.");
        return None;
    }

    RazorpayClient::new(key_id, key_secret).ok()
}

#[tokio::test]
#[ignore = "Live API test against api.razorpay.com. Run with: cargo test-live"]
async fn test_live_orders_flow() {
    let client = match get_live_client() {
        Some(c) => c,
        None => return,
    };

    println!("Executing live Orders::create against Razorpay API...");
    let req = CreateOrderRequest {
        amount: 10000, // 100.00 INR in paise
        currency: "INR".to_string(),
        receipt: Some("live_test_receipt_001".to_string()),
        partial_payment: Some(false),
        first_payment_min_amount: None,
        transfers: None,
        notes: None,
    };

    let order = client
        .orders()
        .create(req, None)
        .await
        .expect("Live order creation should succeed");

    println!("Live Order Created with ID: {}", order.id);
    assert!(!order.id.is_empty());
    assert_eq!(order.amount, 10000);

    // Fetch the newly created order
    let fetched = client
        .orders()
        .fetch(&order.id, None)
        .await
        .expect("Live order fetch should succeed");

    assert_eq!(fetched.id, order.id);

    // List orders
    let list = client
        .orders()
        .all(
            Some(ListOptions {
                count: Some(5),
                skip: None,
                from: None,
                to: None,
            }),
            None,
        )
        .await
        .expect("Live order list should succeed");

    println!("Live Orders count returned: {}", list.count);
    assert!(!list.items.is_empty());
}

#[tokio::test]
#[ignore = "Live API test against api.razorpay.com. Run with: cargo test-live"]
async fn test_live_customers_flow() {
    let client = match get_live_client() {
        Some(c) => c,
        None => return,
    };

    println!("Executing live Customers::create against Razorpay API...");
    let unique_timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let req = CreateCustomerRequest {
        name: format!("Live Test User {}", unique_timestamp),
        email: Some(format!("live_{}@example.com", unique_timestamp)),
        contact: Some("+919876543210".to_string()),
        gstin: None,
        fail_existing: Some(0),
        notes: None,
    };

    let customer = client
        .customers()
        .create(req, None)
        .await
        .expect("Live customer creation should succeed");

    println!("Live Customer Created with ID: {}", customer.id);
    assert!(!customer.id.is_empty());

    // Fetch customer
    let fetched = client
        .customers()
        .fetch(&customer.id, None)
        .await
        .expect("Live customer fetch should succeed");

    assert_eq!(fetched.id, customer.id);
}

#[tokio::test]
#[ignore = "Live API test against api.razorpay.com. Run with: cargo test-live"]
async fn test_live_payment_links_flow() {
    let client = match get_live_client() {
        Some(c) => c,
        None => return,
    };

    println!("Executing live PaymentLinks::create against Razorpay API...");
    let req = CreatePaymentLinkRequest {
        amount: 25000, // 250.00 INR
        currency: Some("INR".to_string()),
        accept_partial: Some(false),
        first_min_partial_amount: None,
        expire_by: None,
        reference_id: None,
        description: Some("Live SDK Integration Test Link".to_string()),
        customer: None,
        notify: None,
        reminder_enable: None,
        notes: None,
        callback_url: None,
        callback_method: None,
    };

    let link = client
        .payment_links()
        .create(req, None)
        .await
        .expect("Live payment link creation should succeed");

    println!("Live Payment Link Created: {} (URL: {})", link.id, link.short_url);
    assert!(!link.id.is_empty());
    assert!(!link.short_url.is_empty());

    // Cancel the payment link
    let cancelled = client
        .payment_links()
        .cancel(&link.id, None)
        .await
        .expect("Live payment link cancellation should succeed");

    assert_eq!(cancelled.status, "cancelled");
}

#[tokio::test]
#[ignore = "Live API test against api.razorpay.com. Run with: cargo test-live"]
async fn test_live_plans_and_subscriptions_flow() {
    let client = match get_live_client() {
        Some(c) => c,
        None => return,
    };

    println!("Executing live Plans::create against Razorpay API...");
    let unique_timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let plan_req = CreatePlanRequest {
        period: PlanPeriod::Monthly,
        interval: 1,
        item: CreatePlanItem {
            name: format!("Live Test Plan {}", unique_timestamp),
            amount: 49900,
            currency: "INR".to_string(),
            description: Some("Live plan subscription test".to_string()),
        },
        notes: None,
    };

    let plan = client
        .plans()
        .create(plan_req, None)
        .await
        .expect("Live plan creation should succeed");

    println!("Live Plan Created with ID: {}", plan.id);
    assert!(!plan.id.is_empty());

    // Fetch plan
    let fetched_plan = client
        .plans()
        .fetch(&plan.id, None)
        .await
        .expect("Live plan fetch should succeed");
    assert_eq!(fetched_plan.id, plan.id);

    // Create Subscription on the plan
    println!("Executing live Subscriptions::create on plan {}...", plan.id);
    let sub_req = CreateSubscriptionRequest {
        plan_id: plan.id.clone(),
        total_count: 6,
        quantity: Some(1),
        start_at: None,
        expire_by: None,
        customer_notify: Some(false),
        addons: None,
        offer_id: None,
        notes: None,
        notify_info: None,
    };

    let subscription = client
        .subscriptions()
        .create(sub_req, None)
        .await
        .expect("Live subscription creation should succeed");

    println!("Live Subscription Created with ID: {}", subscription.id);
    assert!(!subscription.id.is_empty());

    // Fetch subscription
    let fetched_sub = client
        .subscriptions()
        .fetch(&subscription.id, None)
        .await
        .expect("Live subscription fetch should succeed");
    assert_eq!(fetched_sub.id, subscription.id);

    // Cancel the subscription
    let cancelled_sub = client
        .subscriptions()
        .cancel(&subscription.id, false, None)
        .await
        .expect("Live subscription cancellation should succeed");

    println!("Live Subscription Cancelled. Status: {:?}", cancelled_sub.status);
}

#[tokio::test]
#[ignore = "Live API test against api.razorpay.com. Run with: cargo test-live"]
async fn test_live_payments_and_invoices_listing() {
    let client = match get_live_client() {
        Some(c) => c,
        None => return,
    };

    // List payments
    println!("Executing live Payments::all against Razorpay API...");
    let payments = client
        .payments()
        .all(Some(ListOptions { count: Some(5), skip: None, from: None, to: None }), None)
        .await
        .expect("Live payments list should succeed");
    println!("Live Payments count: {}", payments.count);

    // List invoices
    println!("Executing live Invoices::all against Razorpay API...");
    let invoices = client
        .invoices()
        .all(Some(ListOptions { count: Some(5), skip: None, from: None, to: None }), None)
        .await
        .expect("Live invoices list should succeed");
    println!("Live Invoices count: {}", invoices.count);
}
