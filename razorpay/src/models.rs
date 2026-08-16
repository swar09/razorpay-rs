// Razorpay API Models
// Source: https://razorpay.com/docs/api/
// Covers: Orders, Payments, Refunds, Settlements, Customers, Payment Links,
//         Subscriptions, Plans, Invoices, QR Codes, Transfers, Disputes,
//         Items, Virtual Accounts (Smart Collect), Documents, and more.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Shared Types

/// Generic paginated list response wrapper returned by collection endpoints.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RazorpayList<T> {
    /// Entity name, typically `"collection"`.
    pub entity: String,
    /// Total count of items returned in the current page.
    #[serde(default)]
    pub count: u32,
    /// Vector containing individual entity items.
    #[serde(default)]
    pub items: Vec<T>,
}

/// Key-value metadata dictionary attached to any entity.
///
/// Handles both standard JSON objects (`{"key": "value"}`) and empty arrays (`[]`)
/// returned by Razorpay when no notes are set.
#[derive(Debug, Clone, Serialize, Default, PartialEq, Eq)]
pub struct Notes(pub HashMap<String, String>);

impl std::ops::Deref for Notes {
    type Target = HashMap<String, String>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Notes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<HashMap<String, String>> for Notes {
    fn from(map: HashMap<String, String>) -> Self {
        Notes(map)
    }
}

impl From<Notes> for HashMap<String, String> {
    fn from(notes: Notes) -> Self {
        notes.0
    }
}

impl<K: Into<String>, V: Into<String>> FromIterator<(K, V)> for Notes {
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let mut map = HashMap::new();
        for (k, v) in iter {
            map.insert(k.into(), v.into());
        }
        Notes(map)
    }
}

impl<'de> serde::Deserialize<'de> for Notes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct NotesVisitor;

        impl<'de> serde::de::Visitor<'de> for NotesVisitor {
            type Value = Notes;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a key-value map or an empty array")
            }

            fn visit_seq<A>(self, mut _seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                Ok(Notes(HashMap::new()))
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: serde::de::MapAccess<'de>,
            {
                let mut notes = HashMap::new();
                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    let val_str = match v {
                        serde_json::Value::String(s) => s,
                        other => other.to_string(),
                    };
                    notes.insert(k, val_str);
                }
                Ok(Notes(notes))
            }
        }

        deserializer.deserialize_any(NotesVisitor)
    }
}

/// Standard error payload returned by Razorpay on 4xx/5xx responses.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RazorpayError {
    /// Error code categorizing the failure (e.g., `"BAD_REQUEST_ERROR"`, `"GATEWAY_ERROR"`).
    pub code: String,
    /// Human-readable description explaining the error.
    pub description: String,
    /// Originating source of the error (e.g., `"business"`, `"customer"`, `"gateway"`).
    pub source: Option<String>,
    /// Processing step at which the error occurred.
    pub step: Option<String>,
    /// Granular reason code for the error.
    pub reason: Option<String>,
    /// Additional metadata context associated with the failure.
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    /// Specific request body field that caused the validation error.
    pub field: Option<String>,
}

/// Top-level error envelope containing a `RazorpayError`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RazorpayErrorResponse {
    /// Nested error object details.
    pub error: RazorpayError,
}

/// Query parameters for pagination and date-range filtering across list endpoints.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListOptions {
    /// Number of records to return (default: 10, max: 100).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,
    /// Number of records to skip from the beginning.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<u32>,
    /// Fetch entities created on or after this UNIX timestamp (in seconds).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<u64>,
    /// Fetch entities created on or before this UNIX timestamp (in seconds).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<u64>,
}

// Orders (https://razorpay.com/docs/api/orders/)

/// Current lifecycle status of an Order.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum OrderStatus {
    /// Order is created and awaiting payment attempt.
    #[default]
    Created,
    /// Payment was attempted by the customer on the order.
    Attempted,
    /// Order is completely paid.
    Paid,
}

/// Represents a Razorpay Order entity (`entity: "order"`).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Order {
    /// Unique identifier for the order (e.g., `"order_EKwxwAgItmmXdp"`).
    pub id: String,
    /// Entity name, always `"order"`.
    pub entity: String,
    /// Total amount for the order in smallest currency sub-units (e.g., paise for INR).
    pub amount: u64,
    /// Amount paid against this order so far in smallest currency sub-units.
    #[serde(default)]
    pub amount_paid: Option<u64>,
    /// Remaining amount due on this order in smallest currency sub-units.
    #[serde(default)]
    pub amount_due: Option<u64>,
    /// ISO 4217 currency code (e.g., `"INR"`, `"USD"`).
    pub currency: String,
    /// Merchant internal receipt identifier.
    pub receipt: Option<String>,
    /// Offer ID applied to the order, if any.
    pub offer_id: Option<String>,
    /// Current status of the order.
    pub status: OrderStatus,
    /// Indicates whether partial payments are permitted for this order.
    #[serde(default)]
    pub partial_payment: bool,
    /// Number of payment attempts made against this order.
    #[serde(default)]
    pub attempts: u32,
    /// Custom key-value pairs stored with the order.
    pub notes: Option<Notes>,
    /// UNIX timestamp (in seconds) when the order was created.
    pub created_at: u64,
}

/// Request parameters to create a new Order via `POST /v1/orders`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOrderRequest {
    /// Amount to be charged in smallest currency sub-units (paise for INR).
    pub amount: u64,
    /// ISO 4217 currency code (e.g., `"INR"`).
    pub currency: String,
    /// Custom receipt identifier for tracking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt: Option<String>,
    /// Allow partial payments on this order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partial_payment: Option<bool>,
    /// Minimum amount for the first partial payment installment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_payment_min_amount: Option<u64>,
    /// List of linked account split transfers (Route).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfers: Option<Vec<TransferRequest>>,
    /// Key-value metadata dictionary attached to the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Notes>,
}

/// Request parameters to update order notes via `PATCH /v1/orders/{id}`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateOrderRequest {
    /// Updated key-value metadata notes.
    pub notes: Notes,
}

// Payments (https://razorpay.com/docs/api/payments/)

/// Current lifecycle status of a Payment.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum PaymentStatus {
    /// Payment initialized by customer.
    #[default]
    Created,
    /// Payment authorized by bank, pending merchant capture.
    Authorized,
    /// Payment captured and settled to merchant account.
    Captured,
    /// Payment has been partially or fully refunded.
    Refunded,
    /// Payment failed at bank or network gateway.
    Failed,
}

/// Refund status indicator on a payment entity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentRefundStatus {
    /// Part of the paid amount has been refunded.
    Partial,
    /// The entire paid amount has been refunded.
    Full,
}

/// Payment method used by the customer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethod {
    /// Credit or debit card.
    Card,
    /// Internet banking.
    NetBanking,
    /// Digital wallet (e.g., Paytm, Mobikwik).
    Wallet,
    /// Equated Monthly Installments.
    Emi,
    /// Unified Payments Interface (UPI).
    Upi,
    /// Cardless EMI payment.
    CardlessEmi,
    /// Buy Now Pay Later service.
    Paylater,
    /// Automated Clearing House / e-NACH mandate.
    Ach,
}

/// Card details returned when expanding payment cards (`expand[]=card`).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CardDetails {
    /// Unique card identifier.
    pub id: String,
    /// Entity name, always `"card"`.
    pub entity: String,
    /// Cardholder name on card.
    pub name: String,
    /// Last 4 digits of the card number.
    pub last4: String,
    /// Card payment network (e.g., `"Visa"`, `"MasterCard"`, `"RuPay"`).
    pub network: String,
    /// Card type: `"credit"` or `"debit"`.
    #[serde(rename = "type")]
    pub card_type: String,
    /// Issuing bank name.
    pub issuer: Option<String>,
    /// Indicates if the card is internationally issued.
    pub international: bool,
    /// Indicates if EMI is available for this card.
    pub emi: bool,
    /// Card sub-type (e.g., `"consumer"`, `"business"`).
    pub sub_type: Option<String>,
    /// Token IIN reference for tokenized cards.
    pub token_iin: Option<String>,
    /// Unique card fingerprint identifier.
    pub fingerprint: Option<String>,
}

/// EMI tenure details attached to an EMI payment.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EmiDetails {
    /// Bank or financial institution offering the EMI.
    pub issuer: String,
    /// Annual interest rate charged.
    pub rate: u32,
    /// Duration of the EMI plan in months.
    pub duration: u32,
}

/// UPI metadata associated with a UPI payment.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpiInfo {
    /// Type of bank account linked to the UPI ID.
    pub payer_account_type: Option<String>,
    /// Customer Virtual Payment Address (VPA).
    pub vpa: Option<String>,
    /// UPI interaction flow (e.g., `"collect"`, `"intent"`).
    pub flow: Option<String>,
}

/// Bank and acquirer transaction identifiers.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AcquirerData {
    /// Retrieval Reference Number from the bank gateway.
    pub rrn: Option<String>,
    /// UPI transaction reference ID from NPCI.
    pub upi_transaction_id: Option<String>,
    /// Bank internal transaction reference ID.
    pub bank_transaction_id: Option<String>,
    /// Authorization code issued by the card network.
    pub auth_code: Option<String>,
}

/// Represents a Razorpay Payment entity (`entity: "payment"`).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Payment {
    /// Unique payment identifier (e.g., `"pay_29AeabbJyL3mAO"`).
    pub id: String,
    /// Entity name, always `"payment"`.
    pub entity: String,
    /// Transaction amount in smallest currency sub-units (paise for INR).
    pub amount: u64,
    /// ISO 4217 currency code (e.g., `"INR"`).
    pub currency: String,
    /// Current status of the payment.
    pub status: PaymentStatus,
    /// Associated order ID, if created against an order.
    pub order_id: Option<String>,
    /// Associated invoice ID, if created against an invoice.
    pub invoice_id: Option<String>,
    /// Indicates if international payment method was used.
    pub international: bool,
    /// Payment method used for the transaction.
    pub method: Option<PaymentMethod>,
    /// Total amount refunded back to the customer so far in smallest currency sub-units.
    pub amount_refunded: u64,
    /// Refund status if any refund has been issued.
    pub refund_status: Option<PaymentRefundStatus>,
    /// Indicates if the payment has been captured.
    pub captured: bool,
    /// Merchant transaction description.
    pub description: Option<String>,
    /// Token/Card ID used for card payments.
    pub card_id: Option<String>,
    /// Expanded card details (when `expand[]=card` is requested).
    pub card: Option<CardDetails>,
    /// Bank code used for NetBanking transactions.
    pub bank: Option<String>,
    /// Digital wallet code used for wallet payments.
    pub wallet: Option<String>,
    /// Customer VPA used for UPI transactions.
    pub vpa: Option<String>,
    /// Customer email address.
    pub email: Option<String>,
    /// Customer contact phone number.
    pub contact: Option<String>,
    /// Associated customer ID.
    pub customer_id: Option<String>,
    /// Saved card/instrument token ID.
    pub token_id: Option<String>,
    /// Custom key-value notes attached to the payment.
    pub notes: Option<Notes>,
    /// Razorpay transaction processing fee charged to merchant (in paise).
    pub fee: Option<u64>,
    /// Goods and Services Tax (GST) applied on the transaction fee (in paise).
    pub tax: Option<u64>,
    /// Error code if the payment failed.
    pub error_code: Option<String>,
    /// Error description if the payment failed.
    pub error_description: Option<String>,
    /// Error source component (e.g., `"bank"`, `"customer"`).
    pub error_source: Option<String>,
    /// Error workflow step.
    pub error_step: Option<String>,
    /// Granular error reason.
    pub error_reason: Option<String>,
    /// Expanded EMI details if applicable.
    pub emi: Option<EmiDetails>,
    /// Bank/Acquirer reference numbers.
    pub acquirer_data: Option<AcquirerData>,
    /// Expanded UPI info if applicable.
    pub upi: Option<UpiInfo>,
    /// Loyalty/Reward identifier applied.
    pub reward: Option<String>,
    /// Base original amount before currency conversions.
    pub base_amount: Option<u64>,
    /// UNIX timestamp (in seconds) when payment was initiated.
    pub created_at: u64,
}

/// Request parameters to capture an authorized payment via `POST /v1/payments/{id}/capture`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapturePaymentRequest {
    /// Amount to capture in smallest currency sub-units (paise for INR).
    pub amount: u64,
    /// ISO 4217 currency code (e.g., `"INR"`).
    pub currency: String,
}

/// Request parameters to update payment notes via `PATCH /v1/payments/{id}`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePaymentRequest {
    /// Updated key-value metadata notes.
    pub notes: Notes,
}

// Refunds (https://razorpay.com/docs/api/refunds/)

/// Processing speed requested or applied for a refund.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum RefundSpeed {
    /// Standard refund processed in 5-7 business days.
    #[default]
    Normal,
    /// Instant refund credited back to customer account within minutes.
    Instant,
    /// Razorpay selects between Instant and Normal based on routing availability.
    Optimum,
}

/// Lifecycle status of a refund.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum RefundStatus {
    /// Refund is pending batch submission to bank.
    #[default]
    Pending,
    /// Refund processed successfully by bank.
    Processed,
    /// Refund failed at banking gateway.
    Failed,
}

/// Backwards compatibility type alias for [`RefundStatus`].
pub type RefundStatusValue = RefundStatus;

/// Represents a Razorpay Refund entity (`entity: "refund"`).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Refund {
    /// Unique refund identifier (e.g., `"rfnd_8x9a29AeabbJyL"`).
    pub id: String,
    /// Entity name, always `"refund"`.
    pub entity: String,
    /// Refund amount in smallest currency sub-units (paise for INR).
    pub amount: u64,
    /// ISO 4217 currency code.
    pub currency: String,
    /// Payment ID from which this refund was issued.
    pub payment_id: String,
    /// Key-value metadata notes attached to the refund.
    pub notes: Option<Notes>,
    /// Merchant receipt number for the refund.
    pub receipt: Option<String>,
    /// Bank acquirer reference information (e.g. ARN).
    pub acquirer_data: Option<AcquirerData>,
    /// UNIX timestamp when refund was created.
    pub created_at: u64,
    /// Batch ID if processed via a bulk refund batch.
    pub batch_id: Option<String>,
    /// Status of the refund.
    pub status: RefundStatus,
    /// Actual speed at which refund was processed by bank.
    pub speed_processed: RefundSpeed,
    /// Speed tier requested when creating the refund.
    pub speed_requested: RefundSpeed,
}

/// Request parameters to issue a refund via `POST /v1/refunds` or `POST /v1/payments/{id}/refund`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateRefundRequest {
    /// Payment ID to refund (required when calling standalone `POST /v1/refunds`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_id: Option<String>,
    /// Amount to refund in paise. Omit for a full refund of remaining balance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<u64>,
    /// Speed option for the refund (`normal`, `instant`, `optimum`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<RefundSpeed>,
    /// Key-value notes attached to the refund.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Notes>,
    /// Custom merchant receipt identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt: Option<String>,
}

/// Request parameters to update refund notes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRefundRequest {
    /// Updated key-value metadata notes.
    pub notes: Notes,
}

// Settlements (https://razorpay.com/docs/api/settlements/)

/// Lifecycle status of a settlement.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum SettlementStatus {
    /// Settlement initiated.
    #[default]
    Created,
    /// Settlement processed and credited to merchant bank account.
    Processed,
    /// Settlement failed.
    Failed,
}

/// Represents a Razorpay Settlement entity (`entity: "settlement"`).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Settlement {
    /// Unique settlement identifier (e.g., `"setl_1234567890"`).
    pub id: String,
    /// Entity name, always `"settlement"`.
    pub entity: String,
    /// Net settled amount deposited to merchant bank account (in paise).
    pub amount: u64,
    /// Settlement status.
    pub status: SettlementStatus,
    /// Total transaction fee deducted across transactions in this settlement (in paise).
    pub fees: u64,
    /// Total GST deducted across transactions in this settlement (in paise).
    pub tax: u64,
    /// Unique Transaction Reference (UTR) provided by the bank.
    pub utr: Option<String>,
    /// UNIX timestamp when settlement was initiated.
    pub created_at: u64,
}

/// A line item in the settlement reconciliation breakdown.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SettlementReconItem {
    /// Identifier of the entity (e.g., payment ID, refund ID).
    pub entity_id: String,
    /// Transaction type: `"payment"`, `"refund"`, `"transfer"`, `"adjustment"`.
    #[serde(rename = "type")]
    pub transaction_type: String,
    /// Gross amount in smallest currency sub-units.
    pub amount: u64,
    /// Razorpay fee deducted on this item.
    pub fee: u64,
    /// GST deducted on this item fee.
    pub tax: u64,
    /// Debit amount applied to settlement balance.
    pub debit: u64,
    /// Credit amount applied to settlement balance.
    pub credit: u64,
    /// ISO 4217 currency code.
    pub currency: String,
    /// Settlement completion flag.
    pub settled: bool,
    /// UNIX timestamp when the source transaction occurred.
    pub created_at: u64,
    /// UNIX timestamp when this item was settled.
    pub settled_at: Option<u64>,
    /// Parent settlement ID.
    pub settlement_id: Option<String>,
    /// Item description.
    pub description: Option<String>,
    /// Key-value metadata notes.
    pub notes: Option<Notes>,
    /// Payment ID if associated with a payment or refund.
    pub payment_id: Option<String>,
    /// Order ID if associated with an order.
    pub order_id: Option<String>,
    /// Merchant order receipt number.
    pub order_receipt: Option<String>,
    /// Payment method used.
    pub method: Option<String>,
    /// Card network if paid by card.
    pub card_network: Option<String>,
    /// Card issuer bank if paid by card.
    pub card_issuer: Option<String>,
    /// International card indicator.
    pub card_international: Option<bool>,
    /// Bank code if paid via NetBanking.
    pub bank: Option<String>,
    /// Wallet code if paid via Wallet.
    pub wallet: Option<String>,
    /// Customer VPA if paid via UPI.
    pub vpa: Option<String>,
    /// Customer name.
    pub name: Option<String>,
    /// Customer email address.
    pub email: Option<String>,
    /// Customer phone number.
    pub contact: Option<String>,
    /// Error code if applicable.
    pub error_code: Option<String>,
    /// Error description if applicable.
    pub error_description: Option<String>,
}

/// Instant/On-demand settlement entity.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InstantSettlement {
    /// Unique on-demand settlement request ID.
    pub id: String,
    /// Entity name, always `"settlement"`.
    pub entity: String,
    /// Requested amount to settle (in paise).
    pub amount: u64,
    /// Net settled amount credited (in paise).
    pub amount_settled: u64,
    /// Instant payout fees.
    pub fees: u64,
    /// Tax applied on instant payout fee.
    pub tax: u64,
    /// Currency code.
    pub currency: String,
    /// Settle entire available merchant balance flag.
    pub settle_full_balance: bool,
    /// Processing status.
    pub status: String,
    /// Merchant reference description.
    pub description: Option<String>,
    /// Key-value metadata notes.
    pub notes: Option<Notes>,
    /// Scheduled settlement flag.
    pub scheduled: Option<bool>,
    /// Creation UNIX timestamp.
    pub created_at: u64,
    /// List of on-demand payout disbursements.
    pub ondemand_payouts: Option<Vec<InstantSettlementPayout>>,
}

/// Individual payout record within an instant settlement.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InstantSettlementPayout {
    /// Payout ID.
    pub id: String,
    /// Entity name, always `"payout"`.
    pub entity: String,
    /// Payout amount in paise.
    pub amount: u64,
    /// Amount settled in paise.
    pub amount_settled: Option<u64>,
    /// Processing fee in paise.
    pub fees: u64,
    /// Tax on fee in paise.
    pub tax: u64,
    /// Bank UTR number.
    pub utr: Option<String>,
    /// Payout status.
    pub status: String,
    /// Creation timestamp.
    pub created_at: u64,
    /// Processing timestamp.
    pub processed_at: Option<u64>,
    /// Reversal timestamp if reversed.
    pub reversed_at: Option<u64>,
}

/// Request parameters to initiate an instant settlement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateInstantSettlementRequest {
    /// Amount to settle in paise.
    pub amount: u64,
    /// Settle entire available balance flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settle_full_balance: Option<bool>,
    /// Custom description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Key-value metadata notes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Notes>,
}

// Customers (https://razorpay.com/docs/api/customers/)

/// Represents a Razorpay Customer entity (`entity: "customer"`).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Customer {
    /// Unique customer identifier (e.g., `"cust_1234567890"`).
    pub id: String,
    /// Entity name, always `"customer"`.
    pub entity: String,
    /// Full name of the customer.
    pub name: Option<String>,
    /// Email address of the customer.
    pub email: Option<String>,
    /// Phone/contact number of the customer.
    pub contact: Option<String>,
    /// Customer GSTIN number.
    pub gstin: Option<String>,
    /// Custom key-value notes.
    pub notes: Option<Notes>,
    /// UNIX timestamp when the customer was created.
    pub created_at: u64,
}

/// Request parameters to create a new customer via `POST /v1/customers`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateCustomerRequest {
    /// Customer full name.
    pub name: String,
    /// Customer email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Customer phone number with country code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<String>,
    /// Customer GSTIN tax registration number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gstin: Option<String>,
    /// Set to `0` to allow duplicate customer creation if email/contact exists (default: `1`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fail_existing: Option<u8>,
    /// Custom key-value notes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Notes>,
}

/// Request parameters to edit customer details via `PUT /v1/customers/{id}`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EditCustomerRequest {
    /// Updated name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Updated email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Updated contact number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<String>,
    /// Updated GSTIN number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gstin: Option<String>,
    /// Updated key-value notes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Notes>,
}

/// Token entity for saved cards and payment instruments on a customer.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Token {
    /// Token identifier (e.g., `"token_1234567890"`).
    pub id: String,
    /// Entity name, always `"token"`.
    pub entity: String,
    /// Customer ID owning this token.
    pub customer_id: Option<String>,
    /// Vault token reference.
    pub token: Option<String>,
    /// Payment method type (`"card"`, `"bank_account"`, `"vpa"`).
    pub method: Option<String>,
    /// Card details for saved cards.
    pub card: Option<CardDetails>,
    /// Bank code for NetBanking mandates.
    pub bank: Option<String>,
    /// Wallet code for wallet instruments.
    pub wallet: Option<String>,
    /// UPI mandate details.
    pub vpa: Option<UpiInfo>,
    /// Recurring mandate capability flag.
    pub recurring: Option<bool>,
    /// Authorization mechanism (e.g., `"otp"`, `"pin"`).
    pub auth_type: Option<String>,
    /// Maximum allowable recurring transaction charge (in paise).
    pub max_amount: Option<u64>,
    /// Status: `"active"`, `"rejected"`, `"suspended"`.
    pub status: Option<String>,
    /// Token creation timestamp.
    pub created_at: u64,
}

/// Standard boolean deletion response returned by entity delete endpoints.
#[derive(Debug, Clone, Serialize, Default, PartialEq, Eq)]
pub struct DeleteResponse {
    /// Indicates whether the entity was successfully deleted.
    pub deleted: bool,
}

impl<'de> serde::Deserialize<'de> for DeleteResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct DeleteVisitor;

        impl<'de> serde::de::Visitor<'de> for DeleteVisitor {
            type Value = DeleteResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a boolean deleted map or empty array")
            }

            fn visit_seq<A>(self, _seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                Ok(DeleteResponse { deleted: true })
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: serde::de::MapAccess<'de>,
            {
                let mut deleted = true;
                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    if k == "deleted" {
                        deleted = v.as_bool().unwrap_or(true);
                    }
                }
                Ok(DeleteResponse { deleted })
            }
        }

        deserializer.deserialize_any(DeleteVisitor)
    }
}

// Payment Links (https://razorpay.com/docs/api/payments/payment-links/)

/// Represents a Razorpay Standard Payment Link entity (`entity: "payment_link"`).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentLink {
    /// Unique payment link identifier (e.g., `"plink_1234567890"`).
    pub id: String,
    /// Entity name, typically `"payment_link"`.
    #[serde(default)]
    pub entity: Option<String>,
    /// Allows customer to make partial installment payments.
    #[serde(default)]
    pub accept_partial: bool,
    /// Total payment amount requested (in paise).
    pub amount: u64,
    /// Amount paid against this link so far (in paise).
    #[serde(default)]
    pub amount_paid: u64,
    /// UNIX timestamp when the link was cancelled.
    pub cancelled_at: Option<u64>,
    /// Creation UNIX timestamp.
    #[serde(default)]
    pub created_at: u64,
    /// Currency code (e.g., `"INR"`).
    #[serde(default)]
    pub currency: String,
    /// Associated customer ID.
    pub customer_id: Option<String>,
    /// Link description displayed to the customer.
    pub description: Option<String>,
    /// Expiration timestamp in seconds.
    pub expire_by: Option<u64>,
    /// Timestamp when the link expired.
    pub expired_at: Option<u64>,
    /// Minimum amount required for the first partial payment installment.
    pub first_min_partial_amount: Option<u64>,
    /// Custom key-value notes.
    pub notes: Option<Notes>,
    /// Customer notification preferences.
    pub notify: Option<PaymentLinkNotify>,
    /// List of payments made against this payment link.
    pub payments: Option<Vec<PaymentLinkPayment>>,
    /// Merchant internal reference identifier.
    pub reference_id: Option<String>,
    /// Automatic SMS/email payment reminders flag.
    #[serde(default)]
    pub reminder_enable: bool,
    /// Hosted short URL where customer completes payment (e.g., `https://rzp.io/i/xxxx`).
    #[serde(default)]
    pub short_url: String,
    /// Status of the payment link.
    #[serde(default)]
    pub status: PaymentLinkStatus,
    /// Last updated timestamp.
    #[serde(default)]
    pub updated_at: u64,
    /// Indicates if link is a UPI-only link.
    #[serde(default)]
    pub upi_link: bool,
    /// User ID of merchant staff who generated the link.
    pub user_id: Option<String>,
    /// Webhook/Redirect URL triggered upon customer payment completion.
    pub callback_url: Option<String>,
    /// Callback HTTP method (`"get"` or `"post"`).
    pub callback_method: Option<String>,
}

/// Lifecycle status of a Payment Link.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum PaymentLinkStatus {
    /// Link created and awaiting customer payment.
    #[default]
    Created,
    /// Customer made a partial payment on the link.
    PartiallyPaid,
    /// Link has been paid in full.
    Paid,
    /// Link was manually cancelled by the merchant.
    Cancelled,
    /// Link expired before full payment.
    Expired,
}

/// Notification dispatch configuration for Payment Links.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentLinkNotify {
    /// Send payment link via Email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<bool>,
    /// Send payment link via SMS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms: Option<bool>,
    /// Send payment link via WhatsApp.
    pub whatsapp: Option<bool>,
}

/// Customer details object for Payment Links.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentLinkCustomer {
    /// Customer full name.
    pub name: Option<String>,
    /// Customer email address.
    pub email: Option<String>,
    /// Customer contact phone number.
    pub contact: Option<String>,
}

/// Payment record associated with a Payment Link.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentLinkPayment {
    /// Amount paid (in paise).
    pub amount: u64,
    /// Creation timestamp.
    pub created_at: u64,
    /// Unique Payment ID (`pay_xxx`).
    pub payment_id: Option<String>,
    /// Parent payment link ID.
    pub plink_id: String,
    /// Payment status.
    pub status: String,
    /// Updated timestamp.
    pub updated_at: u64,
}

/// Request parameters to create a new Payment Link via `POST /v1/payment_links`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePaymentLinkRequest {
    /// Amount to charge in smallest currency sub-units (paise for INR).
    pub amount: u64,
    /// ISO 4217 currency code (default: `"INR"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Allow partial payment installments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_partial: Option<bool>,
    /// Minimum amount for the first partial payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_min_partial_amount: Option<u64>,
    /// Expiration UNIX timestamp in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_by: Option<u64>,
    /// Merchant custom tracking reference ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<String>,
    /// Description shown on the payment page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Customer contact details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<PaymentLinkCustomer>,
    /// Notification settings for SMS/Email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify: Option<PaymentLinkNotify>,
    /// Enable automated payment reminders.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reminder_enable: Option<bool>,
    /// Custom key-value notes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Notes>,
    /// Redirect URL upon payment completion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_url: Option<String>,
    /// HTTP redirect method (`get` or `post`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_method: Option<String>,
}

/// Request parameters to edit an existing Payment Link via `PATCH /v1/payment_links/{id}`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EditPaymentLinkRequest {
    /// Enable/disable partial payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_partial: Option<bool>,
    /// Update expiration timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_by: Option<u64>,
    /// Update reference ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<String>,
    /// Update description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Update metadata notes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Notes>,
}

/// Notification communication medium.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NotifyMedium {
    /// Send SMS notification.
    Sms,
    /// Send Email notification.
    Email,
}

// QR Codes (https://razorpay.com/docs/api/qr-codes/)

/// Lifecycle status of a QR Code.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum QrCodeStatus {
    /// QR Code is active and accepting payments.
    #[default]
    Active,
    /// QR Code has been closed.
    Closed,
}

/// Represents a Razorpay Dynamic/Static QR Code entity (`entity: "qr_code"`).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QrCode {
    /// Unique QR code identifier (e.g., `"qr_1234567890"`).
    pub id: String,
    /// Entity name, always `"qr_code"`.
    pub entity: String,
    /// Creation timestamp.
    pub created_at: u64,
    /// Scheduled auto-close timestamp.
    pub close_by: Option<u64>,
    /// Reason code if closed.
    pub close_reason: Option<String>,
    /// Actual closure timestamp.
    pub closed_at: Option<u64>,
    /// Associated customer ID.
    pub customer_id: Option<String>,
    /// QR Code description.
    pub description: Option<String>,
    /// Indicates if payment amount is fixed (`true`) or customer-entered (`false`).
    pub fixed_amount: bool,
    /// URL of the generated QR code image (PNG).
    pub image_url: String,
    /// QR Code name/label.
    pub name: Option<String>,
    /// Key-value metadata notes.
    pub notes: Option<Notes>,
    /// Expected payment amount (in paise) for fixed-amount QR codes.
    pub payment_amount: Option<u64>,
    /// Total amount received across all payments on this QR code (in paise).
    pub payments_amount_received: u64,
    /// Total number of successful payments received.
    pub payments_count_received: u32,
    /// Status of the QR code.
    pub status: QrCodeStatus,
    /// QR code type, typically `"upi_qr"`.
    #[serde(rename = "type")]
    pub qr_type: String,
    /// QR code usage tier: `"single_use"` or `"multiple_use"`.
    pub usage: String,
}

/// Request parameters to create a new QR Code via `POST /v1/payments/qr_codes`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateQrCodeRequest {
    /// Type of QR Code (e.g., `"upi_qr"`).
    #[serde(rename = "type")]
    pub qr_type: String,
    /// QR Code display name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Usage: `"single_use"` or `"multiple_use"`.
    pub usage: String,
    /// Fixed amount requirement flag.
    pub fixed_amount: bool,
    /// Payment amount in paise (required if `fixed_amount == true`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_amount: Option<u64>,
    /// Description for customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Customer ID to link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    /// Expiration timestamp in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_by: Option<u64>,
    /// Custom key-value notes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Notes>,
}

// Invoices (https://razorpay.com/docs/api/payments/invoices/)

/// Postal address entity used across Invoices and Linked Accounts.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Address {
    /// Street address line 1.
    pub line1: Option<String>,
    /// Street address line 2.
    pub line2: Option<String>,
    /// City / Town.
    pub city: Option<String>,
    /// State / Province.
    pub state: Option<String>,
    /// Postal ZIP / PIN code.
    pub zipcode: Option<String>,
    /// Country code (e.g., `"IN"`, `"US"`).
    pub country: Option<String>,
}

/// Detailed customer contact and address information for an Invoice.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InvoiceCustomerDetails {
    /// Customer ID.
    pub id: Option<String>,
    /// Customer name.
    pub name: Option<String>,
    /// Customer email address.
    pub email: Option<String>,
    /// Customer contact number.
    pub contact: Option<String>,
    /// Customer GSTIN registration number.
    pub gstin: Option<String>,
    /// Billing address.
    pub billing_address: Option<Address>,
    /// Shipping address.
    pub shipping_address: Option<Address>,
}

/// Tax line item calculation.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaxLine {
    /// Tax identifier.
    pub id: String,
    /// Tax name (e.g., `"GST"`, `"CGST"`, `"SGST"`).
    pub name: String,
    /// Tax percentage rate (e.g., `18.0`).
    pub rate: f64,
    /// Calculated tax amount (in paise).
    pub amount: u64,
}

/// A line item in an Invoice or Order.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InvoiceLineItem {
    /// Line item ID.
    pub id: String,
    /// Associated Item ID from catalog.
    pub item_id: Option<String>,
    /// Name of item.
    pub name: String,
    /// Description of item.
    pub description: Option<String>,
    /// Total line item amount (in paise).
    pub amount: u64,
    /// Unit price per item (in paise).
    pub unit_amount: u64,
    /// Gross amount before deductions.
    pub gross_amount: u64,
    /// Total tax amount for this item.
    pub tax_amount: u64,
    /// Taxable base amount.
    pub taxable_amount: u64,
    /// Net payable amount.
    pub net_amount: u64,
    /// Currency code (e.g., `"INR"`).
    pub currency: String,
    /// Quantity ordered.
    pub quantity: u32,
    /// Breakdown of individual taxes applied.
    pub taxes: Vec<TaxLine>,
}

/// Lifecycle status of an Invoice.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum InvoiceStatus {
    /// Draft invoice.
    #[default]
    Draft,
    /// Issued invoice awaiting payment.
    Issued,
    /// Invoice paid in full.
    Paid,
    /// Partially paid invoice.
    PartiallyPaid,
    /// Cancelled invoice.
    Cancelled,
    /// Expired invoice.
    Expired,
    /// Deleted invoice.
    Deleted,
}

/// Represents a Razorpay Invoice entity (`entity: "invoice"`).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Invoice {
    /// Unique invoice identifier (e.g., `"inv_1234567890"`).
    pub id: String,
    /// Entity name, always `"invoice"`.
    pub entity: String,
    /// Invoice type: `"invoice"`, `"link"`, `"ecod"`.
    #[serde(rename = "type")]
    pub invoice_type: String,
    /// Status of the invoice.
    pub status: InvoiceStatus,
    /// Merchant invoice number (e.g., `"INV-2026-001"`).
    pub invoice_number: Option<String>,
    /// Customer ID.
    pub customer_id: Option<String>,
    /// Customer contact and billing details.
    pub customer_details: Option<InvoiceCustomerDetails>,
    /// Associated Order ID.
    pub order_id: Option<String>,
    /// List of line items included in the invoice.
    pub line_items: Vec<InvoiceLineItem>,
    /// Payment ID if paid.
    pub payment_id: Option<String>,
    /// Invoice issuance timestamp.
    pub date: Option<u64>,
    /// Payment due date timestamp.
    pub due_date: Option<u64>,
    /// Expiration timestamp in seconds.
    pub expire_by: Option<u64>,
    /// Actual expiration timestamp.
    pub expired_at: Option<u64>,
    /// Timestamp when invoice was issued.
    pub issued_at: Option<u64>,
    /// Timestamp when invoice was paid.
    pub paid_at: Option<u64>,
    /// Timestamp when invoice was cancelled.
    pub cancelled_at: Option<u64>,
    /// SMS delivery status.
    pub sms_status: Option<String>,
    /// Email delivery status.
    pub email_status: Option<String>,
    /// Currency code.
    pub currency: String,
    /// Total invoice amount (in paise).
    pub amount: Option<u64>,
    /// Amount paid so far (in paise).
    pub amount_paid: Option<u64>,
    /// Amount remaining due (in paise).
    pub amount_due: Option<u64>,
    /// Hosted short URL for invoice payment.
    pub short_url: Option<String>,
    /// Invoice description.
    pub description: Option<String>,
    /// Key-value metadata notes.
    pub notes: Option<Notes>,
    /// Terms and conditions text.
    pub terms: Option<String>,
    /// Internal merchant comments.
    pub comment: Option<String>,
    /// Creation UNIX timestamp.
    pub created_at: u64,
}

/// Request parameters to create a new Invoice via `POST /v1/invoices`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateInvoiceRequest {
    /// Type: `"invoice"`, `"link"`, `"ecod"`.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub invoice_type: Option<String>,
    /// Invoice description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Associated Customer ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    /// Customer details object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<InvoiceCustomerDetails>,
    /// Line items in invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_items: Option<Vec<InvoiceLineItem>>,
    /// Expiration timestamp in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_by: Option<u64>,
    /// Automatically dispatch SMS to customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_notify: Option<bool>,
    /// Automatically dispatch Email to customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_notify: Option<bool>,
    /// Allow customer partial payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partial_payment: Option<bool>,
    /// Currency code (e.g., `"INR"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Key-value metadata notes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Notes>,
}

/// Request parameters to edit an existing Invoice via `PATCH /v1/invoices/{id}`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EditInvoiceRequest {
    /// Updated description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Updated expiration timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_by: Option<u64>,
    /// Updated key-value notes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Notes>,
}

// Items (https://razorpay.com/docs/api/payments/invoices/item-entity/)

/// Represents a Razorpay Item catalog entity (`entity: "item"`).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Item {
    /// Unique item identifier (e.g., `"item_1234567890"`).
    pub id: String,
    /// Entity name, typically `"item"`.
    #[serde(default)]
    pub entity: Option<String>,
    /// Active/available in catalog flag.
    #[serde(default)]
    pub active: bool,
    /// Item price in smallest currency sub-units (in paise).
    pub amount: u64,
    /// Unit amount (in paise).
    pub unit_amount: u64,
    /// Currency code.
    pub currency: String,
    /// Name of item.
    pub name: String,
    /// Description of item.
    pub description: Option<String>,
    /// Unit of measure (e.g., `"pc"`, `"kg"`).
    pub unit: Option<String>,
    /// Tax inclusive pricing flag.
    pub tax_inclusive: bool,
    /// HSN code for goods.
    pub hsn_code: Option<String>,
    /// SAC code for services.
    pub sac_code: Option<String>,
    /// Tax rate percentage.
    pub tax_rate: Option<f64>,
    /// Tax details vector.
    pub taxes: Option<Vec<serde_json::Value>>,
}

/// Request parameters to create an Item via `POST /v1/items`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateItemRequest {
    /// Name of item.
    pub name: String,
    /// Price in paise.
    pub amount: u64,
    /// Currency code (e.g., `"INR"`).
    pub currency: String,
    /// Description of item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Request parameters to update an Item via `PATCH /v1/items/{id}`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateItemRequest {
    /// Updated name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Updated amount in paise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<u64>,
    /// Updated currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Updated description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Updated active status flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
}

// Plans & Subscriptions (https://razorpay.com/docs/api/payments/subscriptions/)

/// Billing interval period for recurring plans.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum PlanPeriod {
    /// Daily recurring cycle.
    Daily,
    /// Weekly recurring cycle.
    Weekly,
    /// Monthly recurring cycle.
    #[default]
    Monthly,
    /// Yearly recurring cycle.
    Yearly,
}

/// Represents a Razorpay Subscription Plan entity (`entity: "plan"`).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Plan {
    /// Unique plan identifier (e.g., `"plan_1234567890"`).
    pub id: String,
    /// Entity name, always `"plan"`.
    pub entity: String,
    /// Billing frequency multiplier (e.g., interval `2` with `Monthly` period = billed every 2 months).
    pub interval: u32,
    /// Period unit: `Daily`, `Weekly`, `Monthly`, `Yearly`.
    pub period: PlanPeriod,
    /// Catalog item details attached to the plan.
    pub item: PlanItem,
    /// Key-value metadata notes.
    pub notes: Option<Notes>,
    /// UNIX creation timestamp.
    pub created_at: u64,
}

/// Catalog item details attached to a subscription plan.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlanItem {
    /// Item ID.
    pub id: String,
    /// Active state flag.
    pub active: bool,
    /// Recurring charge amount (in paise).
    pub amount: u64,
    /// Unit charge amount (in paise).
    pub unit_amount: u64,
    /// Currency code (e.g., `"INR"`).
    pub currency: String,
    /// Plan item name.
    pub name: String,
    /// Plan item description.
    pub description: Option<String>,
}

/// Request parameters to create a Plan via `POST /v1/plans`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePlanRequest {
    /// Billing period: `daily`, `weekly`, `monthly`, `yearly`.
    pub period: PlanPeriod,
    /// Frequency interval multiplier.
    pub interval: u32,
    /// Plan item details.
    pub item: CreatePlanItem,
    /// Custom key-value notes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Notes>,
}

/// Item configuration used when creating a Plan.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreatePlanItem {
    /// Name of the plan.
    pub name: String,
    /// Recurring billing amount in smallest currency units (paise).
    pub amount: u64,
    /// Currency code (e.g., `"INR"`).
    pub currency: String,
    /// Plan description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Lifecycle status of a recurring subscription.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionStatus {
    /// Subscription created, awaiting first payment authentication.
    #[default]
    Created,
    /// Mandate successfully authenticated by customer.
    Authenticated,
    /// Active subscription actively being charged on recurring schedule.
    Active,
    /// Charge attempt in progress.
    Pending,
    /// Charging halted due to recurring payment failures.
    Halted,
    /// Subscription cancelled by merchant or customer.
    Cancelled,
    /// All scheduled billing cycles completed.
    Completed,
    /// Subscription expired before activation.
    Expired,
    /// Temporarily paused by merchant.
    Paused,
}

/// Represents a Razorpay Subscription entity (`entity: "subscription"`).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Subscription {
    /// Unique subscription identifier (e.g., `"sub_1234567890"`).
    pub id: String,
    /// Entity name, always `"subscription"`.
    pub entity: String,
    /// Associated Plan ID.
    pub plan_id: String,
    /// Current subscription lifecycle status.
    pub status: SubscriptionStatus,
    /// Start timestamp of current active billing cycle.
    pub current_start: Option<u64>,
    /// End timestamp of current active billing cycle.
    pub current_end: Option<u64>,
    /// Timestamp when subscription was terminated.
    pub ended_at: Option<u64>,
    /// Number of plan seats/units subscribed.
    pub quantity: u32,
    /// Key-value metadata notes.
    pub notes: Option<Notes>,
    /// Next scheduled auto-debit charge timestamp.
    pub charge_at: Option<u64>,
    /// Start timestamp for recurring charges.
    pub start_at: Option<u64>,
    /// Scheduled end timestamp.
    pub end_at: Option<u64>,
    /// Number of authorization attempts made.
    pub auth_attempts: u32,
    /// Total billing cycles in subscription lifetime.
    pub total_count: u32,
    /// Number of successful billing cycles completed.
    pub paid_count: u32,
    /// Customer notification setting.
    pub customer_notify: bool,
    /// Creation UNIX timestamp.
    pub created_at: u64,
    /// Expiration timestamp if not authenticated.
    pub expire_by: Option<u64>,
    /// Hosted authentication short URL.
    pub short_url: Option<String>,
    /// Indicates if there are pending scheduled plan changes.
    pub has_scheduled_changes: bool,
    /// Timestamp when scheduled changes take effect.
    pub change_scheduled_at: Option<u64>,
    /// Subscription origin source.
    pub source: Option<String>,
    /// Recurring payment method (e.g., `"card"`, `"upi"`, `"nach"`).
    pub payment_method: Option<String>,
    /// Associated discount offer ID.
    pub offer_id: Option<String>,
    /// Remaining billing cycles.
    pub remaining_count: u32,
}

/// Addon item attached during subscription creation.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SubscriptionAddon {
    /// Addon item details.
    pub item: CreatePlanItem,
}

/// Notification destination info for subscriptions.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SubscriptionNotifyInfo {
    /// Destination phone number for SMS.
    pub notify_phone: Option<String>,
    /// Destination email address.
    pub notify_email: Option<String>,
}

/// Request parameters to create a Subscription via `POST /v1/subscriptions`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSubscriptionRequest {
    /// ID of the Plan to subscribe to.
    pub plan_id: String,
    /// Total number of billing cycles to execute.
    pub total_count: u32,
    /// Number of plan seats/units (default: 1).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u32>,
    /// UNIX timestamp when the subscription should start.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_at: Option<u64>,
    /// Expiration timestamp in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_by: Option<u64>,
    /// Send automated notification to customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_notify: Option<bool>,
    /// Upfront addons to charge with first invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addons: Option<Vec<SubscriptionAddon>>,
    /// Discount offer ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offer_id: Option<String>,
    /// Custom key-value notes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Notes>,
    /// Customer contact information for notifications.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_info: Option<SubscriptionNotifyInfo>,
}

/// Request parameters to update a Subscription via `PATCH /v1/subscriptions/{id}`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateSubscriptionRequest {
    /// New Plan ID to switch to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    /// Updated quantity of seats.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u32>,
    /// Updated remaining cycle count.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining_count: Option<u32>,
    /// Start timestamp for updated schedule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_at: Option<u64>,
    /// When to apply schedule change (`"now"`, `"cycle_end"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_change_at: Option<String>,
    /// Send notification to customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_notify: Option<bool>,
    /// Updated offer ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offer_id: Option<String>,
    /// Updated metadata notes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Notes>,
}

/// Represents a Razorpay Subscription Addon entity (`entity: "addon"`).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Addon {
    /// Unique addon identifier (e.g., `"ao_1234567890"`).
    pub id: String,
    /// Entity name, always `"addon"`.
    pub entity: String,
    /// Item pricing details.
    pub item: PlanItem,
    /// Associated subscription ID.
    pub subscription_id: Option<String>,
    /// Associated invoice ID.
    pub invoice_id: Option<String>,
    /// Creation UNIX timestamp.
    pub created_at: u64,
}

/// Request parameters to create an Addon via `POST /v1/subscriptions/{id}/addons`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateAddonRequest {
    /// Addon item details.
    pub item: CreatePlanItem,
    /// Quantity of the addon to add.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u32>,
}

// Disputes (https://razorpay.com/docs/api/disputes/)

/// Evidence documentation attached to contest a chargeback dispute.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisputeEvidence {
    /// Disputed charge amount (in paise).
    pub amount: u64,
    /// Summary explanation defending the charge.
    pub summary: Option<String>,
    /// Document IDs for proof of shipping/delivery.
    pub shipping_proof: Option<Vec<String>>,
    /// Document IDs for customer billing invoice proof.
    pub billing_proof: Option<Vec<String>>,
    /// Document IDs for cancellation policy proof.
    pub cancellation_proof: Option<Vec<String>>,
    /// Document IDs for email/chat customer communications.
    pub customer_communication: Option<Vec<String>>,
    /// Document IDs for proof of digital service fulfillment.
    pub proof_of_service: Option<Vec<String>>,
    /// Document IDs for formal merchant explanation letter.
    pub explanation_letter: Option<Vec<String>>,
    /// Document IDs for refund confirmation receipts.
    pub refund_confirmation: Option<Vec<String>>,
    /// Document IDs for digital IP / user activity logs.
    pub access_activity_log: Option<Vec<String>>,
    /// Document IDs for terms and refund cancellation policy.
    pub refund_cancellation_policy: Option<Vec<String>>,
    /// Document IDs for accepted terms and conditions.
    pub terms_and_conditions: Option<Vec<String>>,
    /// Additional supporting document attachments.
    pub others: Option<Vec<DisputeOtherDocument>>,
    /// UNIX timestamp when evidence was submitted.
    pub submitted_at: Option<u64>,
}

/// An attached supporting file uploaded via Documents API.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisputeOtherDocument {
    /// Document ID from Documents API (`doc_xxx`).
    pub document_id: String,
    /// Document file name.
    pub document_name: String,
}

/// Represents a Razorpay Dispute / Chargeback entity (`entity: "dispute"`).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Dispute {
    /// Unique dispute identifier (e.g., `"disp_1234567890"`).
    pub id: String,
    /// Entity name, always `"dispute"`.
    pub entity: String,
    /// Disputed payment ID (`pay_xxx`).
    pub payment_id: String,
    /// Disputed transaction amount in paise.
    pub amount: u64,
    /// Currency code.
    pub currency: String,
    /// Amount currently held/deducted by bank (in paise).
    pub amount_deducted: u64,
    /// Bank chargeback reason code.
    pub reason_code: String,
    /// Human-readable explanation of dispute reason.
    pub reason_description: String,
    /// Deadline timestamp to respond with evidence.
    pub respond_by: u64,
    /// Dispute status.
    pub status: DisputeStatus,
    /// Dispute lifecycle phase.
    pub phase: DisputePhase,
    /// Creation timestamp.
    pub created_at: u64,
    /// Submitted merchant evidence.
    pub evidence: Option<DisputeEvidence>,
}

/// Lifecycle status of a Dispute.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum DisputeStatus {
    /// Dispute opened by issuing bank.
    #[default]
    Open,
    /// Merchant evidence under review by bank.
    UnderReview,
    /// Dispute resolved in favor of merchant.
    Won,
    /// Dispute resolved in favor of cardholder / customer.
    Lost,
    /// Dispute closed without action.
    Closed,
}

/// Lifecycle phase of a Dispute.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum DisputePhase {
    /// Fraud notification phase.
    #[default]
    Fraud,
    /// Information retrieval request phase.
    Retrieval,
    /// Formal chargeback phase.
    Chargeback,
    /// Pre-arbitration dispute phase.
    PreArbitration,
    /// Arbitration dispute phase before card networks.
    Arbitration,
}

/// Request parameters to contest or draft dispute evidence via `POST /v1/disputes/{id}/contest`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ContestDisputeRequest {
    /// Disputed amount contested (in paise).
    pub amount: u64,
    /// Text summary explaining merchant defense.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// Proof of shipping document IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_proof: Option<Vec<String>>,
    /// Proof of billing document IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_proof: Option<Vec<String>>,
    /// Proof of cancellation policy document IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_proof: Option<Vec<String>>,
    /// Customer communication document IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_communication: Option<Vec<String>>,
    /// Proof of service document IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof_of_service: Option<Vec<String>>,
    /// Explanation letter document IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation_letter: Option<Vec<String>>,
    /// Refund confirmation document IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_confirmation: Option<Vec<String>>,
    /// Activity log document IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_activity_log: Option<Vec<String>>,
    /// Refund cancellation policy document IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_cancellation_policy: Option<Vec<String>>,
    /// Terms and conditions document IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_and_conditions: Option<Vec<String>>,
    /// Other supporting document records.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub others: Option<Vec<DisputeOtherDocument>>,
    /// Action to perform: `"draft"` (save work) or `"submit"` (transmit to bank).
    pub action: String,
}

// Transfers & Route (https://razorpay.com/docs/api/payments/route/)

/// Transfer processing failure error details.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferError {
    /// Error code.
    pub code: Option<String>,
    /// Error description.
    pub description: Option<String>,
    /// Error reason.
    pub reason: Option<String>,
    /// Request field causing failure.
    pub field: Option<String>,
    /// Workflow step.
    pub step: Option<String>,
    /// Entity ID.
    pub id: Option<String>,
    /// Error source.
    pub source: Option<String>,
    /// Extra metadata.
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

/// Represents a Razorpay Transfer entity (`entity: "transfer"`).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Transfer {
    /// Unique transfer identifier (e.g., `"trf_1234567890"`).
    pub id: String,
    /// Entity name, always `"transfer"`.
    pub entity: String,
    /// Source payment ID (`pay_xxx`) or order ID.
    pub source: String,
    /// Linked Account ID receiving the funds (`acc_xxx`).
    pub recipient: String,
    /// Transferred amount in paise.
    pub amount: u64,
    /// Currency code (e.g., `"INR"`).
    pub currency: String,
    /// Amount reversed back from linked account so far.
    pub amount_reversed: u64,
    /// Master merchant key-value notes.
    pub notes: Option<Notes>,
    /// Notes passed to the recipient linked account.
    pub linked_account_notes: Option<Vec<String>>,
    /// Hold settlement to linked account flag.
    pub on_hold: bool,
    /// Hold settlement until this UNIX timestamp.
    pub on_hold_until: Option<u64>,
    /// Linked account settlement reference ID.
    pub recipient_settlement_id: Option<String>,
    /// Creation timestamp.
    pub created_at: u64,
    /// Processing timestamp.
    pub processed_at: Option<u64>,
    /// Error details if transfer failed.
    pub error: Option<TransferError>,
}

/// Inline transfer definition used when splitting payments on order creation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferRequest {
    /// Recipient Linked Account ID (`acc_xxx`).
    pub account: String,
    /// Amount to transfer in paise.
    pub amount: u64,
    /// Currency code (e.g., `"INR"`).
    pub currency: String,
    /// Key-value metadata notes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Notes>,
    /// Linked account notes keys.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_account_notes: Option<Vec<String>>,
    /// Hold funds flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_hold: Option<bool>,
    /// Hold funds until timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_hold_until: Option<u64>,
}

/// Represents a Transfer Reversal entity (`entity: "reversal"`).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferReversal {
    /// Unique reversal identifier (e.g., `"rev_1234567890"`).
    pub id: String,
    /// Entity name, always `"reversal"`.
    pub entity: String,
    /// Parent transfer ID being reversed.
    pub transfer_id: String,
    /// Reversed amount in paise.
    pub amount: u64,
    /// Reversal processing fee in paise.
    pub fee: u64,
    /// Tax on fee in paise.
    pub tax: u64,
    /// Currency code.
    pub currency: String,
    /// Key-value metadata notes.
    pub notes: Option<Notes>,
    /// User or system initiator ID.
    pub initiator_id: Option<String>,
    /// Associated customer refund ID if linked to a refund.
    pub customer_refund_id: Option<String>,
    /// Creation timestamp.
    pub created_at: u64,
}

/// Request parameters to reverse a Transfer via `POST /v1/transfers/{id}/reversals`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ReverseTransferRequest {
    /// Amount to reverse in paise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<u64>,
    /// Key-value notes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Notes>,
    /// Set to `1` to reverse all unsettled transfers on the payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse_all: Option<u8>,
}

/// Request parameters to edit transfer hold settings via `PATCH /v1/transfers/{id}`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EditTransferRequest {
    /// Place or release settlement hold.
    pub on_hold: bool,
    /// Hold until UNIX timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_hold_until: Option<u64>,
}

/// Stakeholder person entity attached to a Linked Account.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Stakeholder {
    /// Stakeholder ID (e.g., `"sth_1234567890"`).
    pub id: String,
    /// Entity name, always `"stakeholder"`.
    pub entity: String,
    /// Full name of stakeholder.
    pub name: Option<String>,
    /// Email address.
    pub email: Option<String>,
    /// Phone number.
    pub phone: Option<String>,
    /// Relationship details (e.g., director, executive, owner).
    pub relationship: Option<HashMap<String, serde_json::Value>>,
    /// Key-value metadata notes.
    pub notes: Option<Notes>,
    /// Creation timestamp.
    pub created_at: u64,
}

// Virtual Accounts / Smart Collect (https://razorpay.com/docs/api/payments/smart-collect/)

/// Receiver payment instrument attached to a Virtual Account.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VirtualAccountReceiver {
    /// Receiver ID.
    pub id: String,
    /// Entity name (`"bank_account"` or `"vpa"`).
    pub entity: String,
    /// Type: `"bank_account"` or `"vpa"`.
    #[serde(rename = "type")]
    pub receiver_type: String,
    /// Bank IFSC code (for `bank_account`).
    pub ifsc: Option<String>,
    /// Bank name (e.g., `"HDFC Bank"`).
    pub bank_name: Option<String>,
    /// Account beneficiary name.
    pub name: Option<String>,
    /// Key-value notes.
    pub notes: Option<Notes>,
    /// Virtual bank account number.
    pub account_number: Option<String>,
    /// Virtual VPA address (for `vpa`).
    pub address: Option<String>,
}

/// Represents a Razorpay Virtual Account entity (`entity: "virtual_account"`).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VirtualAccount {
    /// Unique virtual account identifier (e.g., `"va_1234567890"`).
    pub id: String,
    /// Entity name, always `"virtual_account"`.
    pub entity: String,
    /// Name assigned to the virtual account.
    pub name: String,
    /// Description.
    pub description: Option<String>,
    /// Expected payment amount (in paise).
    pub amount_expected: Option<u64>,
    /// Total amount paid into this account so far (in paise).
    pub amount_paid: u64,
    /// Status of the virtual account.
    pub status: VirtualAccountStatus,
    /// List of receiver instruments (Virtual Bank Account, Virtual UPI VPA).
    pub receivers: Option<Vec<VirtualAccountReceiver>>,
    /// Scheduled closure timestamp.
    pub close_by: Option<u64>,
    /// Actual closure timestamp.
    pub closed_at: Option<u64>,
    /// Closure reason code.
    pub close_reason: Option<String>,
    /// Key-value metadata notes.
    pub notes: Option<Notes>,
    /// Associated customer ID.
    pub customer_id: Option<String>,
    /// Creation timestamp.
    pub created_at: u64,
}

/// Lifecycle status of a Virtual Account.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum VirtualAccountStatus {
    /// Virtual account is active and accepting deposits.
    #[default]
    Active,
    /// Virtual account has received full expected payment.
    Paid,
    /// Virtual account has been closed.
    Closed,
}

/// Receiver types requested for virtual account creation.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateVirtualAccountReceivers {
    /// Types to create: `vec!["bank_account"]`, `vec!["vpa"]`, or `vec!["bank_account", "vpa"]`.
    pub types: Vec<String>,
}

/// Request parameters to create a Virtual Account via `POST /v1/virtual_accounts`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateVirtualAccountRequest {
    /// Receiver types configuration (`bank_account`, `vpa`).
    pub receivers: CreateVirtualAccountReceivers,
    /// Description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Expected amount in paise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<u64>,
    /// Customer ID to link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    /// Expiration timestamp in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_by: Option<u64>,
    /// Key-value metadata notes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Notes>,
}

// Linked Accounts & Route v2 (https://razorpay.com/docs/api/payments/route/linked-account-entity/)

/// Address container for Linked Account registration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkedAccountAddresses {
    /// Registered business address.
    pub registered: Option<Address>,
}

/// Business profile for a Linked Account.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkedAccountProfile {
    /// Business category.
    pub category: Option<String>,
    /// Business sub-category.
    pub subcategory: Option<String>,
    /// Business address details.
    pub addresses: Option<LinkedAccountAddresses>,
}

/// Represents a Razorpay Route Linked Account entity (`entity: "account"`).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkedAccount {
    /// Unique account identifier (e.g., `"acc_1234567890"`).
    pub id: String,
    /// Entity name, always `"account"`.
    pub entity: String,
    /// Account type: `"standard"`, `"custom"`.
    pub type_: Option<String>,
    /// Account status: `"created"`, `"activated"`, `"suspended"`.
    pub status: Option<String>,
    /// Account owner email address.
    pub email: String,
    /// Business profile info.
    pub profile: Option<LinkedAccountProfile>,
    /// Key-value metadata notes.
    pub notes: Option<Notes>,
    /// Creation timestamp.
    pub created_at: u64,
}

// Payment Downtime (https://razorpay.com/docs/api/payments/downtime/)

/// Affected payment instrument in a downtime event.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DowntimeInstrument {
    /// Bank code affected (e.g., `"HDFC"`).
    pub bank: Option<String>,
    /// UPI PSP app affected (e.g., `"google_pay"`, `"phonepe"`).
    pub psp: Option<String>,
    /// Card issuer bank affected.
    pub issuer: Option<String>,
    /// Card network affected (e.g., `"Visa"`).
    pub network: Option<String>,
}

/// Represents a Payment Downtime event (`entity: "payment.downtime"`).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentDowntime {
    /// Unique downtime identifier.
    pub id: String,
    /// Entity name, always `"payment.downtime"`.
    pub entity: String,
    /// Affected payment method (`"card"`, `"netbanking"`, `"upi"`).
    pub method: String,
    /// Downtime start UNIX timestamp.
    pub begin: u64,
    /// Downtime end UNIX timestamp if resolved.
    pub end: Option<u64>,
    /// Status: `"scheduled"`, `"started"`, `"resolved"`.
    pub status: String,
    /// Indicates if downtime is planned maintenance.
    pub scheduled: bool,
    /// Impact severity: `"low"`, `"medium"`, `"high"`.
    pub severity: String,
    /// Specific instrument affected.
    pub instrument: Option<DowntimeInstrument>,
    /// Creation timestamp.
    pub created_at: u64,
    /// Last updated timestamp.
    pub updated_at: u64,
}

// Documents (https://razorpay.com/docs/api/documents/)

/// Represents a Razorpay Document entity (`entity: "document"`).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Document {
    /// Unique document identifier (e.g., `"doc_1234567890"`).
    pub id: String,
    /// Entity name, always `"document"`.
    pub entity: String,
    /// Uploaded file name.
    pub name: String,
    /// Document file type / extension.
    pub document_type: String,
    /// Category classification (e.g., `"dispute_evidence"`, `"kyc"`).
    pub document_category: Option<String>,
    /// Secure download URL.
    pub url: Option<String>,
    /// File size in bytes.
    pub size: Option<u64>,
    /// Upload timestamp.
    pub created_at: u64,
}

// Fund Accounts & Payouts (https://razorpay.com/docs/api/x/)

/// Represents a RazorpayX Fund Account entity (`entity: "fund_account"`).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FundAccount {
    /// Unique fund account identifier (e.g., `"fa_1234567890"`).
    pub id: String,
    /// Entity name, always `"fund_account"`.
    pub entity: String,
    /// Associated Contact ID (`cont_xxx`).
    pub contact_id: String,
    /// Account type: `"bank_account"`, `"vpa"`, `"card"`, `"wallet"`.
    pub account_type: String,
    /// Active state flag.
    pub active: bool,
    /// Bank account details payload.
    pub bank_account: Option<serde_json::Value>,
    /// VPA details payload.
    pub vpa: Option<serde_json::Value>,
    /// Card details payload.
    pub card: Option<serde_json::Value>,
    /// Wallet details payload.
    pub wallet: Option<serde_json::Value>,
    /// Creation timestamp.
    pub created_at: u64,
}

/// Request parameters to create a Fund Account via `POST /v1/fund_accounts`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateFundAccountRequest {
    /// Contact ID to attach fund account to.
    pub contact_id: String,
    /// Type: `"bank_account"`, `"vpa"`, `"card"`, `"wallet"`.
    pub account_type: String,
    /// Bank account details object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account: Option<serde_json::Value>,
    /// UPI VPA details object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpa: Option<serde_json::Value>,
    /// Card details object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<serde_json::Value>,
    /// Wallet details object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet: Option<serde_json::Value>,
}

/// Represents a RazorpayX Payout disbursement entity (`entity: "payout"`).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Payout {
    /// Unique payout identifier (e.g., `"pout_1234567890"`).
    pub id: String,
    /// Entity name, always `"payout"`.
    pub entity: String,
    /// Recipient Fund Account ID (`fa_xxx`).
    pub fund_account_id: String,
    /// Payout amount in paise.
    pub amount: u64,
    /// Currency code (e.g., `"INR"`).
    pub currency: String,
    /// Key-value metadata notes.
    pub notes: Option<Notes>,
    /// RazorpayX transaction fee in paise.
    pub fees: Option<u64>,
    /// GST on fee in paise.
    pub tax: Option<u64>,
    /// Status: `"queued"`, `"pending"`, `"processing"`, `"processed"`, `"reversed"`, `"cancelled"`, `"rejected"`.
    pub status: String,
    /// Purpose category (e.g., `"salary"`, `"vendor"`, `"refund"`).
    pub purpose: Option<String>,
    /// Bank UTR reference number.
    pub utr: Option<String>,
    /// Transfer mode: `"NEFT"`, `"RTGS"`, `"IMPS"`, `"UPI"`, `"card"`.
    pub mode: String,
    /// Merchant custom tracking reference ID.
    pub reference_id: Option<String>,
    /// Statement narration appearing on recipient bank passbook.
    pub narration: Option<String>,
    /// Creation timestamp.
    pub created_at: u64,
}

/// Request parameters to initiate a Payout via `POST /v1/payouts`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreatePayoutRequest {
    /// Source RazorpayX current account number to debit.
    pub account_number: String,
    /// Destination Fund Account ID.
    pub fund_account_id: String,
    /// Payout amount in paise.
    pub amount: u64,
    /// Currency code (e.g., `"INR"`).
    pub currency: String,
    /// Mode: `"NEFT"`, `"RTGS"`, `"IMPS"`, `"UPI"`, `"card"`.
    pub mode: String,
    /// Purpose: `"salary"`, `"vendor"`, `"refund"`, etc.
    pub purpose: String,
    /// Queue payout if account has insufficient balance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_if_low_balance: Option<bool>,
    /// Merchant reference tracking ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<String>,
    /// Narration text on bank statement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub narration: Option<String>,
    /// Key-value metadata notes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Notes>,
}

// Cards & IINs (https://razorpay.com/docs/api/payments/cards/)

/// Represents a Razorpay Card entity (`entity: "card"`).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Card {
    /// Unique card reference ID.
    pub id: String,
    /// Entity name, always `"card"`.
    pub entity: String,
    /// Cardholder name.
    pub name: Option<String>,
    /// Last 4 digits of card number.
    pub last4: String,
    /// Card network (e.g., `"Visa"`, `"MasterCard"`, `"RuPay"`).
    pub network: String,
    /// Card type: `"credit"` or `"debit"`.
    #[serde(rename = "type")]
    pub card_type: Option<String>,
    /// Card sub-type (e.g., `"consumer"`, `"business"`).
    pub sub_type: Option<String>,
    /// Issuing bank name.
    pub issuer: Option<String>,
    /// International issuance flag.
    pub international: Option<bool>,
    /// EMI availability flag.
    pub emi: Option<bool>,
    /// Expiry month (1-12).
    pub expiry_month: Option<u8>,
    /// Expiry year (4-digit format).
    pub expiry_year: Option<u16>,
}

/// Represents an Issuer Identification Number (IIN) entity (`entity: "iin"`).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Iin {
    /// 6-digit or 8-digit IIN / BIN prefix.
    pub iin: String,
    /// Entity name, typically `"iin"`.
    pub entity: Option<String>,
    /// Card network (e.g., `"Visa"`, `"MasterCard"`, `"RuPay"`).
    pub network: Option<String>,
    /// Card type: `"credit"` or `"debit"`.
    #[serde(rename = "type")]
    pub card_type: Option<String>,
    /// Sub-type classification.
    pub sub_type: Option<String>,
    /// Bank issuer code (e.g., `"HDFC"`).
    pub issuer_code: Option<String>,
    /// Bank issuer name (e.g., `"HDFC Bank"`).
    pub issuer_name: Option<String>,
    /// International card indicator.
    pub international: Option<bool>,
    /// Tokenization enabled flag.
    pub is_tokenized: Option<bool>,
    /// Recurring e-mandate support flag.
    pub recurring: Option<bool>,
}

// Products & Terms (Route v2)

/// Linked Account Product Configuration entity (`GET /v2/accounts/{id}/products`).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProductConfiguration {
    /// Configuration record ID.
    pub id: Option<String>,
    /// Linked Account ID (`acc_xxx`).
    pub account_id: Option<String>,
    /// Product name (e.g., `"payment_gateway"`, `"route"`).
    pub product_name: Option<String>,
    /// Configuration status: `"requested"`, `"active"`, `"under_review"`.
    pub status: Option<String>,
    /// Product settings and payment methods configuration.
    pub configuration: Option<serde_json::Value>,
    /// KYC and activation requirements payload.
    pub requirements: Option<serde_json::Value>,
}

/// Terms and Conditions response payload (`GET /v2/products/{product_name}/tnc`).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TncResponse {
    /// Entity name.
    pub entity: Option<String>,
    /// Product name.
    pub product_name: Option<String>,
    /// Terms and conditions text and metadata.
    pub tnc: Option<serde_json::Value>,
    /// UNIX timestamp when terms were last updated.
    pub last_updated_at: Option<u64>,
}

/// Request payload to submit an OTP for a payment (`POST /v1/payments/{id}/otp/submit`).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OtpSubmitRequest {
    /// 4-to-6 digit numeric OTP entered by the customer.
    pub otp: String,
}

// Bills (https://razorpay.com/docs/api/payments/bills)

/// Represents a Razorpay Retail/POS Bill entity (`id: "bill_xxxx"`).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Bill {
    /// Unique bill identifier (e.g., `"bill_PYamApGCFTAjkh"`).
    pub id: String,
    /// Business type (e.g., `"retail"`, `"ecommerce"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_type: Option<String>,
    /// Business category (e.g., `"retail_and_consumer_goods"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_category: Option<String>,
    /// Customer details object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<serde_json::Value>,
    /// Loyalty points and rewards data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loyalty: Option<serde_json::Value>,
    /// Retail store code identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_code: Option<String>,
    /// Receipt UNIX timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_timestamp: Option<u64>,
    /// Receipt / Invoice number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_number: Option<String>,
    /// Receipt type (e.g., `"tax_invoice"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_type: Option<String>,
    /// Receipt delivery method: `"digital"` or `"print"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_delivery: Option<String>,
    /// Hosted digital receipt URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_url: Option<String>,
    /// Barcode data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bar_code_number: Option<serde_json::Value>,
    /// QR Code data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qr_code_number: Option<serde_json::Value>,
    /// Physical billing POS terminal machine number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_pos_number: Option<String>,
    /// POS terminal category.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pos_category: Option<String>,
    /// Order tracking number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_number: Option<String>,
    /// Order service type (e.g., `"dine_in"`, `"takeaway"`, `"delivery"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_service_type: Option<String>,
    /// Delivery status tracking URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_status_url: Option<String>,
    /// Line items in bill.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_items: Option<Vec<serde_json::Value>>,
    /// Summary totals, gross, and net payable amounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_summary: Option<serde_json::Value>,
    /// Tax calculations breakdown.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxes: Option<Vec<serde_json::Value>>,
    /// Payment method transactions list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payments: Option<Vec<serde_json::Value>>,
    /// POS event payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<serde_json::Value>,
    /// Metadata tags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Creation UNIX timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<u64>,
}

/// Request parameters to create a retail Bill via `POST /v1/bills`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateBillRequest {
    /// Business type (e.g., `"retail"`, `"ecommerce"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_type: Option<String>,
    /// Business category (e.g., `"retail_and_consumer_goods"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_category: Option<String>,
    /// Customer details object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<serde_json::Value>,
    /// Loyalty points and rewards data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loyalty: Option<serde_json::Value>,
    /// Retail store code identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_code: Option<String>,
    /// Receipt UNIX timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_timestamp: Option<u64>,
    /// Receipt / Invoice number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_number: Option<String>,
    /// Receipt type (e.g., `"tax_invoice"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_type: Option<String>,
    /// Receipt delivery method: `"digital"` or `"print"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_delivery: Option<String>,
    /// Hosted digital receipt URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_url: Option<String>,
    /// Physical billing POS terminal machine number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_pos_number: Option<String>,
    /// POS terminal category.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pos_category: Option<String>,
    /// Order tracking number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_number: Option<String>,
    /// Line items in bill.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_items: Option<Vec<serde_json::Value>>,
    /// Summary totals, gross, and net payable amounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_summary: Option<serde_json::Value>,
    /// Tax calculations breakdown.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxes: Option<Vec<serde_json::Value>>,
    /// Payment method transactions list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payments: Option<Vec<serde_json::Value>>,
    /// Metadata tags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

/// Request parameters to update a retail Bill via `PATCH /v1/bills/{id}`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateBillRequest {
    /// Updated customer details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<serde_json::Value>,
    /// Updated line items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_items: Option<Vec<serde_json::Value>>,
    /// Updated summary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_summary: Option<serde_json::Value>,
    /// Updated taxes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxes: Option<Vec<serde_json::Value>>,
    /// Updated payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payments: Option<Vec<serde_json::Value>>,
    /// Updated metadata tags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

// Webhooks (https://razorpay.com/docs/api/webhooks/)

/// Represents a Razorpay Webhook subscription configuration entity (`entity: "webhook"`).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Webhook {
    /// Unique webhook identifier (e.g., `"hook_1234567890"`).
    pub id: String,
    /// Entity name, always `"webhook"`.
    #[serde(default)]
    pub entity: Option<String>,
    /// Webhook destination endpoint URL.
    pub url: String,
    /// Alert notification email on delivery failure.
    pub alert_email: Option<String>,
    /// Webhook secret for HMAC signature verification.
    pub secret: Option<String>,
    /// Subscribed event types.
    pub events: Vec<String>,
    /// Webhook active status.
    #[serde(default)]
    pub active: bool,
    /// Linked Account ID if configured on a route account.
    pub account_id: Option<String>,
    /// Creation UNIX timestamp.
    #[serde(default)]
    pub created_at: u64,
    /// Last updated timestamp.
    pub updated_at: Option<u64>,
}

/// Request parameters to create a Webhook via `POST /v1/webhooks` or `POST /v2/accounts/{id}/webhooks`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateWebhookRequest {
    /// Webhook endpoint URL.
    pub url: String,
    /// Subscribed event names (e.g., `vec!["payment.captured", "order.paid"]`).
    pub events: Vec<String>,
    /// Secret string used to verify webhook signatures.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// Email address for delivery failure alerts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_email: Option<String>,
    /// Active state flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
}

/// Request parameters to update a Webhook via `PUT /v1/webhooks/{id}` or `PATCH /v2/accounts/{id}/webhooks/{id}`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateWebhookRequest {
    /// Updated destination URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Updated events list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<String>>,
    /// Updated secret.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// Updated alert email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_email: Option<String>,
    /// Updated active state flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
}

/// Known standard Razorpay Webhook event types.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WebhookEventType {
    /// Payment authorized.
    #[serde(rename = "payment.authorized")]
    PaymentAuthorized,
    /// Payment captured.
    #[serde(rename = "payment.captured")]
    PaymentCaptured,
    /// Payment failed.
    #[serde(rename = "payment.failed")]
    PaymentFailed,
    /// Order paid in full.
    #[serde(rename = "order.paid")]
    OrderPaid,
    /// Invoice paid.
    #[serde(rename = "invoice.paid")]
    InvoicePaid,
    /// Subscription authenticated.
    #[serde(rename = "subscription.authenticated")]
    SubscriptionAuthenticated,
    /// Subscription activated.
    #[serde(rename = "subscription.activated")]
    SubscriptionActivated,
    /// Subscription charged successfully.
    #[serde(rename = "subscription.charged")]
    SubscriptionCharged,
    /// Subscription completed.
    #[serde(rename = "subscription.completed")]
    SubscriptionCompleted,
    /// Subscription paused.
    #[serde(rename = "subscription.paused")]
    SubscriptionPaused,
    /// Subscription resumed.
    #[serde(rename = "subscription.resumed")]
    SubscriptionResumed,
    /// Subscription cancelled.
    #[serde(rename = "subscription.cancelled")]
    SubscriptionCancelled,
    /// Refund processed successfully.
    #[serde(rename = "refund.processed")]
    RefundProcessed,
    /// Refund failed.
    #[serde(rename = "refund.failed")]
    RefundFailed,
    /// Dispute created / opened.
    #[serde(rename = "dispute.created")]
    DisputeCreated,
    /// Dispute won by merchant.
    #[serde(rename = "dispute.won")]
    DisputeWon,
    /// Dispute lost by merchant.
    #[serde(rename = "dispute.lost")]
    DisputeLost,
    /// Dispute closed.
    #[serde(rename = "dispute.closed")]
    DisputeClosed,
    /// Other unmapped event type.
    #[serde(other)]
    Unknown,
}

/// Container wrapping a typed entity inside a webhook payload.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WebhookEntityContainer<T> {
    /// The nested entity object.
    pub entity: T,
}

/// Entities payload map contained in an incoming webhook event.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WebhookEntities {
    /// Nested payment entity if present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment: Option<WebhookEntityContainer<Payment>>,
    /// Nested order entity if present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<WebhookEntityContainer<Order>>,
    /// Nested subscription entity if present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<WebhookEntityContainer<Subscription>>,
    /// Nested refund entity if present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund: Option<WebhookEntityContainer<Refund>>,
    /// Nested dispute entity if present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dispute: Option<WebhookEntityContainer<Dispute>>,
    /// Nested invoice entity if present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<WebhookEntityContainer<Invoice>>,
    /// Nested virtual account entity if present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_account: Option<WebhookEntityContainer<VirtualAccount>>,
}

/// Standard Razorpay Webhook Event payload envelope.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookPayload {
    /// Entity name, always `"event"`.
    #[serde(default)]
    pub entity: String,
    /// Account ID owning the webhook.
    #[serde(default)]
    pub account_id: String,
    /// Event name string (e.g. `"payment.captured"`).
    pub event: String,
    /// List of entities contained in the payload.
    #[serde(default)]
    pub contains: Vec<String>,
    /// Nested entity payloads.
    pub payload: WebhookEntities,
    /// UNIX timestamp when event was generated.
    #[serde(default)]
    pub created_at: u64,
}
