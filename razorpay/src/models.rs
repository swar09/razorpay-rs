// Razorpay API Models
// Source: https://razorpay.com/docs/api/
// Covers: Orders, Payments, Refunds, Settlements, Customers, Payment Links,
//         Subscriptions, Plans, Invoices, QR Codes, Transfers, Disputes,
//         Items, Virtual Accounts (Smart Collect), Documents, and more.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Shared / Common Types

/// Generic paginated list response wrapper used by all collection endpoints.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RazorpayList<T> {
    pub entity: String,
    pub count: u32,
    pub items: Vec<T>,
}

/// Notes object — arbitrary key-value pairs attached to any entity.
/// Handles both map (`{"k": "v"}`) and empty array (`[]`) representations returned by the API.
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

/// Generic Razorpay API error response body.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RazorpayError {
    pub code: String,
    pub description: String,
    pub source: Option<String>,
    pub step: Option<String>,
    pub reason: Option<String>,
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    pub field: Option<String>,
}

/// Top-level error envelope returned by the API on 4xx / 5xx.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RazorpayErrorResponse {
    pub error: RazorpayError,
}

/// Common pagination/filter options for list endpoints.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListOptions {
    /// Max records to return (default 10, max 100).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,
    /// Skip the first N records.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<u32>,
    /// UNIX timestamp : fetch records created after this time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<u64>,
    /// UNIX timestamp : fetch records created before this time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<u64>,
}

// Orders  https://razorpay.com/docs/api/orders/entity

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum OrderStatus {
    #[default]
    Created,
    Attempted,
    Paid,
}

/// Razorpay Order entity (`entity: "order"`).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Order {
    pub id: String,
    pub entity: String,
    /// Amount in smallest currency sub-unit (paise for INR).
    pub amount: u64,
    #[serde(default)]
    pub amount_paid: Option<u64>,
    #[serde(default)]
    pub amount_due: Option<u64>,
    pub currency: String,
    pub receipt: Option<String>,
    pub offer_id: Option<String>,
    pub status: OrderStatus,
    #[serde(default)]
    pub partial_payment: bool,
    #[serde(default)]
    pub attempts: u32,
    pub notes: Option<Notes>,
    pub created_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOrderRequest {
    pub amount: u64,
    pub currency: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partial_payment: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_payment_min_amount: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfers: Option<Vec<TransferRequest>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Notes>,
}

// Payments  https://razorpay.com/docs/api/payments/entity

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum PaymentStatus {
    #[default]
    Created,
    Authorized,
    Captured,
    Refunded,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentRefundStatus {
    Partial,
    Full,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethod {
    Card,
    NetBanking,
    Wallet,
    Emi,
    Upi,
    CardlessEmi,
    Paylater,
    Ach,
}

/// Card details — populated when `expand[]=card` is requested.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CardDetails {
    pub id: String,
    pub entity: String,
    pub name: String,
    /// Last 4 digits of the card number.
    pub last4: String,
    /// Card network (Visa, MasterCard, RuPay, …).
    pub network: String,
    /// `"credit"` or `"debit"`.
    #[serde(rename = "type")]
    pub card_type: String,
    pub issuer: Option<String>,
    pub international: bool,
    pub emi: bool,
    pub sub_type: Option<String>,
    pub token_iin: Option<String>,
    pub fingerprint: Option<String>,
}

/// EMI details — populated when `expand[]=emi` is requested.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EmiDetails {
    pub issuer: String,
    pub rate: u32,
    pub duration: u32,
}

/// UPI-specific info attached to a Payment.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpiInfo {
    pub payer_account_type: Option<String>,
    pub vpa: Option<String>,
    pub flow: Option<String>,
}

/// Acquirer-specific data returned by the bank/network.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AcquirerData {
    pub rrn: Option<String>,
    pub upi_transaction_id: Option<String>,
    pub bank_transaction_id: Option<String>,
    pub auth_code: Option<String>,
}

/// Razorpay Payment entity (`entity: "payment"`).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Payment {
    /// e.g. `pay_DBJOWzybf0sJbb`
    pub id: String,
    pub entity: String,
    pub amount: u64,
    pub currency: String,
    pub status: PaymentStatus,
    pub order_id: Option<String>,
    pub invoice_id: Option<String>,
    pub international: bool,
    pub method: Option<PaymentMethod>,
    pub amount_refunded: u64,
    pub refund_status: Option<PaymentRefundStatus>,
    pub captured: bool,
    pub description: Option<String>,
    pub card_id: Option<String>,
    /// Populated when `expand[]=card` is passed.
    pub card: Option<CardDetails>,
    pub bank: Option<String>,
    pub wallet: Option<String>,
    pub vpa: Option<String>,
    pub email: Option<String>,
    pub contact: Option<String>,
    pub customer_id: Option<String>,
    pub token_id: Option<String>,
    pub notes: Option<Notes>,
    pub fee: Option<u64>,
    pub tax: Option<u64>,
    pub error_code: Option<String>,
    pub error_description: Option<String>,
    pub error_source: Option<String>,
    pub error_step: Option<String>,
    pub error_reason: Option<String>,
    /// Populated when `expand[]=emi` is passed.
    pub emi: Option<EmiDetails>,
    pub acquirer_data: Option<AcquirerData>,
    pub upi: Option<UpiInfo>,
    pub reward: Option<String>,
    pub base_amount: Option<u64>,
    pub created_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapturePaymentRequest {
    pub amount: u64,
    pub currency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePaymentRequest {
    pub notes: Notes,
}

// Refunds  https://razorpay.com/docs/api/refunds/entity

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum RefundSpeed {
    #[default]
    Normal,
    Instant,
    Optimum,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RefundStatusValue {
    Pending,
    Processed,
    Failed,
}

/// Razorpay Refund entity (`entity: "refund"`).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Refund {
    pub id: String,
    pub entity: String,
    pub amount: u64,
    pub currency: String,
    pub payment_id: String,
    pub notes: Option<Notes>,
    pub receipt: Option<String>,
    pub acquirer_data: Option<AcquirerData>,
    pub created_at: u64,
    pub batch_id: Option<String>,
    pub status: String,
    pub speed_processed: RefundSpeed,
    pub speed_requested: RefundSpeed,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateRefundRequest {
    /// Payment ID (required when creating via standalone POST /v1/refunds).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_id: Option<String>,
    /// Amount to refund in paise. Omit for a full refund.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<RefundSpeed>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Notes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRefundRequest {
    pub notes: Notes,
}

// Settlements  https://razorpay.com/docs/api/settlements/entity

/// Razorpay Settlement entity (`entity: "settlement"`).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Settlement {
    pub id: String,
    pub entity: String,
    pub amount: u64,
    pub status: String,
    pub fees: u64,
    pub tax: u64,
    pub utr: Option<String>,
    pub created_at: u64,
}

/// A single line in the settlement reconciliation report.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SettlementReconItem {
    pub entity_id: String,
    #[serde(rename = "type")]
    pub transaction_type: String,
    pub amount: u64,
    pub fee: u64,
    pub tax: u64,
    pub debit: u64,
    pub credit: u64,
    pub currency: String,
    pub settled: bool,
    pub created_at: u64,
    pub settled_at: Option<u64>,
    pub settlement_id: Option<String>,
    pub description: Option<String>,
    pub notes: Option<Notes>,
    pub payment_id: Option<String>,
    pub order_id: Option<String>,
    pub order_receipt: Option<String>,
    pub method: Option<String>,
    pub card_network: Option<String>,
    pub card_issuer: Option<String>,
    pub card_international: Option<bool>,
    pub bank: Option<String>,
    pub wallet: Option<String>,
    pub vpa: Option<String>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub contact: Option<String>,
    pub error_code: Option<String>,
    pub error_description: Option<String>,
}

// Instant Settlements  https://razorpay.com/docs/api/settlements/instant/entity

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InstantSettlement {
    pub id: String,
    pub entity: String,
    pub amount: u64,
    pub amount_settled: u64,
    pub fees: u64,
    pub tax: u64,
    pub currency: String,
    pub settle_full_balance: bool,
    pub status: String,
    pub description: Option<String>,
    pub notes: Option<Notes>,
    pub scheduled: Option<bool>,
    pub created_at: u64,
    pub ondemand_payouts: Option<Vec<InstantSettlementPayout>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InstantSettlementPayout {
    pub id: String,
    pub entity: String,
    pub amount: u64,
    pub amount_settled: Option<u64>,
    pub fees: u64,
    pub tax: u64,
    pub utr: Option<String>,
    pub status: String,
    pub created_at: u64,
    pub processed_at: Option<u64>,
    pub reversed_at: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateInstantSettlementRequest {
    pub amount: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settle_full_balance: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Notes>,
}

// Customers  https://razorpay.com/docs/api/customers/entity

/// Razorpay Customer entity (`entity: "customer"`).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Customer {
    pub id: String,
    pub entity: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub contact: Option<String>,
    pub gstin: Option<String>,
    pub notes: Option<Notes>,
    pub created_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateCustomerRequest {
    pub name: String,
    pub email: Option<String>,
    pub contact: Option<String>,
    pub gstin: Option<String>,
    /// Set to `0` to allow duplicate email/contact. Default `1`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fail_existing: Option<u8>,
    pub notes: Option<Notes>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EditCustomerRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gstin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Notes>,
}

/// Razorpay Token entity for customer saved instruments.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Token {
    pub id: String,
    pub entity: String,
    pub customer_id: Option<String>,
    pub token: Option<String>,
    pub method: Option<String>,
    pub card: Option<CardDetails>,
    pub bank: Option<String>,
    pub wallet: Option<String>,
    pub vpa: Option<UpiInfo>,
    pub recurring: Option<bool>,
    pub auth_type: Option<String>,
    pub max_amount: Option<u64>,
    pub status: Option<String>,
    pub created_at: u64,
}

/// Generic delete response returned by Razorpay entity deletion endpoints.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeleteResponse {
    pub deleted: bool,
}

// Payment Links  https://razorpay.com/docs/api/payments/payment-links/entity

/// Payment Link entity (`entity: "payment_link"`).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentLink {
    pub id: String,
    #[serde(default)]
    pub entity: Option<String>,
    #[serde(default)]
    pub accept_partial: bool,
    pub amount: u64,
    #[serde(default)]
    pub amount_paid: u64,
    pub cancelled_at: Option<u64>,
    #[serde(default)]
    pub created_at: u64,
    #[serde(default)]
    pub currency: String,
    pub customer_id: Option<String>,
    pub description: Option<String>,
    pub expire_by: Option<u64>,
    pub expired_at: Option<u64>,
    pub first_min_partial_amount: Option<u64>,
    pub notes: Option<Notes>,
    pub notify: Option<PaymentLinkNotify>,
    pub payments: Option<Vec<PaymentLinkPayment>>,
    pub reference_id: Option<String>,
    #[serde(default)]
    pub reminder_enable: bool,
    #[serde(default)]
    pub short_url: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub updated_at: u64,
    #[serde(default)]
    pub upi_link: bool,
    pub user_id: Option<String>,
    pub callback_url: Option<String>,
    pub callback_method: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentLinkNotify {
    pub email: bool,
    pub sms: bool,
    pub whatsapp: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentLinkPayment {
    pub amount: u64,
    pub created_at: u64,
    pub payment_id: Option<String>,
    pub plink_id: String,
    pub status: String,
    pub updated_at: u64,
}

/// Inline customer fields when creating a Payment Link.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentLinkCustomer {
    pub name: Option<String>,
    pub email: Option<String>,
    pub contact: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePaymentLinkRequest {
    pub amount: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_partial: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_min_partial_amount: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_by: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<PaymentLinkCustomer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify: Option<PaymentLinkNotify>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reminder_enable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Notes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_method: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EditPaymentLinkRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_partial: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_by: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Notes>,
}

/// Notification medium for Payment Links and Invoices.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NotifyMedium {
    Sms,
    Email,
}

// QR Codes  https://razorpay.com/docs/api/qr-codes/entity

/// Razorpay QR Code entity (`entity: "qr_code"`).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QrCode {
    pub id: String,
    pub entity: String,
    pub created_at: u64,
    pub close_by: Option<u64>,
    pub close_reason: Option<String>,
    pub closed_at: Option<u64>,
    pub customer_id: Option<String>,
    pub description: Option<String>,
    pub fixed_amount: bool,
    pub image_url: String,
    pub name: Option<String>,
    pub notes: Option<Notes>,
    pub payment_amount: Option<u64>,
    pub payments_amount_received: u64,
    pub payments_count_received: u32,
    pub status: String,
    #[serde(rename = "type")]
    pub qr_type: String,
    /// `"single_use"` or `"multiple_use"`.
    pub usage: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateQrCodeRequest {
    #[serde(rename = "type")]
    pub qr_type: String,
    pub name: Option<String>,
    /// `"single_use"` or `"multiple_use"`.
    pub usage: String,
    pub fixed_amount: bool,
    pub payment_amount: Option<u64>,
    pub description: Option<String>,
    pub customer_id: Option<String>,
    pub close_by: Option<u64>,
    pub notes: Option<Notes>,
}

// Invoices  https://razorpay.com/docs/api/payments/invoices/entity

/// Generic postal address used in invoices and linked accounts.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Address {
    pub line1: Option<String>,
    pub line2: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub zipcode: Option<String>,
    pub country: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InvoiceCustomerDetails {
    pub id: Option<String>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub contact: Option<String>,
    pub gstin: Option<String>,
    pub billing_address: Option<Address>,
    pub shipping_address: Option<Address>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaxLine {
    pub id: String,
    pub name: String,
    pub rate: f64,
    pub amount: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InvoiceLineItem {
    pub id: String,
    pub item_id: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub amount: u64,
    pub unit_amount: u64,
    pub gross_amount: u64,
    pub tax_amount: u64,
    pub taxable_amount: u64,
    pub net_amount: u64,
    pub currency: String,
    pub quantity: u32,
    pub taxes: Vec<TaxLine>,
}

/// Razorpay Invoice entity (`entity: "invoice"`).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Invoice {
    pub id: String,
    pub entity: String,
    /// `"invoice"` or `"link"` or `"ecod"`.
    #[serde(rename = "type")]
    pub invoice_type: String,
    pub status: String,
    pub invoice_number: Option<String>,
    pub customer_id: Option<String>,
    pub customer_details: Option<InvoiceCustomerDetails>,
    pub order_id: Option<String>,
    pub line_items: Vec<InvoiceLineItem>,
    pub payment_id: Option<String>,
    pub date: Option<u64>,
    pub due_date: Option<u64>,
    pub expire_by: Option<u64>,
    pub expired_at: Option<u64>,
    pub issued_at: Option<u64>,
    pub paid_at: Option<u64>,
    pub cancelled_at: Option<u64>,
    pub sms_status: Option<String>,
    pub email_status: Option<String>,
    pub currency: String,
    pub amount: Option<u64>,
    pub amount_paid: Option<u64>,
    pub amount_due: Option<u64>,
    pub short_url: Option<String>,
    pub description: Option<String>,
    pub notes: Option<Notes>,
    pub terms: Option<String>,
    pub comment: Option<String>,
    pub created_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateInvoiceRequest {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub invoice_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<InvoiceCustomerDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_items: Option<Vec<InvoiceLineItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_by: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_notify: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_notify: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partial_payment: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Notes>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EditInvoiceRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_by: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Notes>,
}

// Items  https://razorpay.com/docs/api/payments/invoices/item-entity

/// Razorpay Item entity (`entity: "item"`).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Item {
    pub id: String,
    pub entity: String,
    pub active: bool,
    pub amount: u64,
    pub unit_amount: u64,
    pub currency: String,
    pub name: String,
    pub description: Option<String>,
    pub unit: Option<String>,
    pub tax_inclusive: bool,
    pub hsn_code: Option<String>,
    pub sac_code: Option<String>,
    pub tax_rate: Option<f64>,
    pub taxes: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateItemRequest {
    pub name: String,
    pub amount: u64,
    pub currency: String,
    pub description: Option<String>,
}

// Subscriptions  https://razorpay.com/docs/api/payments/subscriptions/entity
// Plans          https://razorpay.com/docs/api/payments/subscriptions/plans-entity

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum PlanPeriod {
    Daily,
    Weekly,
    #[default]
    Monthly,
    Yearly,
}

/// Razorpay Plan entity (`entity: "plan"`).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Plan {
    pub id: String,
    pub entity: String,
    pub interval: u32,
    pub period: PlanPeriod,
    pub item: PlanItem,
    pub notes: Option<Notes>,
    pub created_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlanItem {
    pub id: String,
    pub active: bool,
    pub amount: u64,
    pub unit_amount: u64,
    pub currency: String,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePlanRequest {
    pub period: PlanPeriod,
    pub interval: u32,
    pub item: CreatePlanItem,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Notes>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreatePlanItem {
    pub name: String,
    pub amount: u64,
    pub currency: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionStatus {
    #[default]
    Created,
    Authenticated,
    Active,
    Pending,
    Halted,
    Cancelled,
    Completed,
    Expired,
    Paused,
}

/// Razorpay Subscription entity (`entity: "subscription"`).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Subscription {
    pub id: String,
    pub entity: String,
    pub plan_id: String,
    pub status: SubscriptionStatus,
    pub current_start: Option<u64>,
    pub current_end: Option<u64>,
    pub ended_at: Option<u64>,
    pub quantity: u32,
    pub notes: Option<Notes>,
    pub charge_at: Option<u64>,
    pub start_at: Option<u64>,
    pub end_at: Option<u64>,
    pub auth_attempts: u32,
    pub total_count: u32,
    pub paid_count: u32,
    pub customer_notify: bool,
    pub created_at: u64,
    pub expire_by: Option<u64>,
    pub short_url: Option<String>,
    pub has_scheduled_changes: bool,
    pub change_scheduled_at: Option<u64>,
    pub source: Option<String>,
    pub payment_method: Option<String>,
    pub offer_id: Option<String>,
    pub remaining_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SubscriptionAddon {
    pub item: CreatePlanItem,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SubscriptionNotifyInfo {
    pub notify_phone: Option<String>,
    pub notify_email: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSubscriptionRequest {
    pub plan_id: String,
    pub total_count: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_at: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_by: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_notify: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addons: Option<Vec<SubscriptionAddon>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offer_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Notes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_info: Option<SubscriptionNotifyInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateSubscriptionRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_at: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_change_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_notify: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offer_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Notes>,
}

/// Razorpay Addon entity (`entity: "addon"`).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Addon {
    pub id: String,
    pub entity: String,
    pub item: PlanItem,
    pub subscription_id: Option<String>,
    pub invoice_id: Option<String>,
    pub created_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateAddonRequest {
    pub item: CreatePlanItem,
    pub quantity: Option<u32>,
}

// Disputes  https://razorpay.com/docs/api/disputes/entity

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisputeEvidence {
    pub amount: u64,
    pub summary: Option<String>,
    pub shipping_proof: Option<Vec<String>>,
    pub billing_proof: Option<Vec<String>>,
    pub cancellation_proof: Option<Vec<String>>,
    pub customer_communication: Option<Vec<String>>,
    pub proof_of_service: Option<Vec<String>>,
    pub explanation_letter: Option<Vec<String>>,
    pub refund_confirmation: Option<Vec<String>>,
    pub access_activity_log: Option<Vec<String>>,
    pub refund_cancellation_policy: Option<Vec<String>>,
    pub terms_and_conditions: Option<Vec<String>>,
    pub others: Option<Vec<DisputeOtherDocument>>,
    pub submitted_at: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisputeOtherDocument {
    pub document_id: String,
    pub document_name: String,
}

/// Razorpay Dispute entity (`entity: "dispute"`).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Dispute {
    pub id: String,
    pub entity: String,
    pub payment_id: String,
    pub amount: u64,
    pub currency: String,
    pub amount_deducted: u64,
    pub reason_code: String,
    pub reason_description: String,
    pub respond_by: u64,
    pub status: String,
    pub phase: String,
    pub created_at: u64,
    pub evidence: Option<DisputeEvidence>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ContestDisputeRequest {
    pub amount: u64,
    pub summary: Option<String>,
    pub shipping_proof: Option<Vec<String>>,
    pub billing_proof: Option<Vec<String>>,
    pub cancellation_proof: Option<Vec<String>>,
    pub customer_communication: Option<Vec<String>>,
    pub proof_of_service: Option<Vec<String>>,
    pub explanation_letter: Option<Vec<String>>,
    pub refund_confirmation: Option<Vec<String>>,
    pub access_activity_log: Option<Vec<String>>,
    pub refund_cancellation_policy: Option<Vec<String>>,
    pub terms_and_conditions: Option<Vec<String>>,
    pub others: Option<Vec<DisputeOtherDocument>>,
    /// `"draft"` or `"submit"`
    pub action: String,
}

// Transfers / Route  https://razorpay.com/docs/api/payments/route/transfers-entity

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferError {
    pub code: Option<String>,
    pub description: Option<String>,
    pub reason: Option<String>,
    pub field: Option<String>,
    pub step: Option<String>,
    pub id: Option<String>,
    pub source: Option<String>,
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

/// Razorpay Transfer entity (`entity: "transfer"`).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Transfer {
    pub id: String,
    pub entity: String,
    pub source: String,
    pub recipient: String,
    pub amount: u64,
    pub currency: String,
    pub amount_reversed: u64,
    pub notes: Option<Notes>,
    pub linked_account_notes: Option<Vec<String>>,
    pub on_hold: bool,
    pub on_hold_until: Option<u64>,
    pub recipient_settlement_id: Option<String>,
    pub created_at: u64,
    pub processed_at: Option<u64>,
    pub error: Option<TransferError>,
}

/// Used in order/payment creation to specify route transfers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferRequest {
    pub account: String,
    pub amount: u64,
    pub currency: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Notes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_account_notes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_hold: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_hold_until: Option<u64>,
}

/// Transfer reversal entity.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferReversal {
    pub id: String,
    pub entity: String,
    pub transfer_id: String,
    pub amount: u64,
    pub fee: u64,
    pub tax: u64,
    pub currency: String,
    pub notes: Option<Notes>,
    pub initiator_id: Option<String>,
    pub customer_refund_id: Option<String>,
    pub created_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ReverseTransferRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Notes>,
    /// Set to `1` to reverse all unsettled transfers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse_all: Option<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EditTransferRequest {
    pub on_hold: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_hold_until: Option<u64>,
}

/// Razorpay Stakeholder entity for linked accounts (`entity: "stakeholder"`).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Stakeholder {
    pub id: String,
    pub entity: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub relationship: Option<HashMap<String, serde_json::Value>>,
    pub notes: Option<Notes>,
    pub created_at: u64,
}

// Virtual Accounts (Smart Collect)
// https://razorpay.com/docs/api/payments/smart-collect/entity

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VirtualAccountReceiver {
    pub id: String,
    pub entity: String,
    /// `"bank_account"` or `"vpa"`.
    #[serde(rename = "type")]
    pub receiver_type: String,
    pub ifsc: Option<String>,
    pub bank_name: Option<String>,
    pub name: Option<String>,
    pub notes: Option<Notes>,
    pub account_number: Option<String>,
    /// VPA address if `type == "vpa"`.
    pub address: Option<String>,
}

/// Razorpay Virtual Account entity (`entity: "virtual_account"`).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VirtualAccount {
    pub id: String,
    pub entity: String,
    pub name: String,
    pub description: Option<String>,
    pub amount_expected: Option<u64>,
    pub amount_paid: u64,
    pub status: String,
    pub receivers: Option<Vec<VirtualAccountReceiver>>,
    pub close_by: Option<u64>,
    pub closed_at: Option<u64>,
    pub close_reason: Option<String>,
    pub notes: Option<Notes>,
    pub customer_id: Option<String>,
    pub created_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateVirtualAccountReceivers {
    /// e.g. `vec!["bank_account"]` or `vec!["bank_account", "vpa"]`.
    pub types: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateVirtualAccountRequest {
    pub receivers: CreateVirtualAccountReceivers,
    pub description: Option<String>,
    pub amount: Option<u64>,
    pub customer_id: Option<String>,
    pub close_by: Option<u64>,
    pub notes: Option<Notes>,
}

// Linked Accounts (Route)
// https://razorpay.com/docs/api/payments/route/linked-account-entity

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkedAccountAddresses {
    pub registered: Option<Address>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkedAccountProfile {
    pub category: Option<String>,
    pub subcategory: Option<String>,
    pub addresses: Option<LinkedAccountAddresses>,
}

/// Razorpay Linked Account entity (`entity: "account"`).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkedAccount {
    pub id: String,
    pub entity: String,
    pub type_: Option<String>,
    pub status: Option<String>,
    pub email: String,
    pub profile: Option<LinkedAccountProfile>,
    pub notes: Option<Notes>,
    pub created_at: u64,
}

// Payment Downtime  https://razorpay.com/docs/api/payments/downtime/entity

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DowntimeInstrument {
    pub bank: Option<String>,
    pub psp: Option<String>,
    pub issuer: Option<String>,
    pub network: Option<String>,
}

/// Razorpay Payment Downtime entity (`entity: "payment.downtime"`).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentDowntime {
    pub id: String,
    pub entity: String,
    pub method: String,
    pub begin: u64,
    pub end: Option<u64>,
    pub status: String,
    pub scheduled: bool,
    pub severity: String,
    pub instrument: Option<DowntimeInstrument>,
    pub created_at: u64,
    pub updated_at: u64,
}

// Documents  https://razorpay.com/docs/api/documents/entity

/// Razorpay Document entity (`entity: "document"`).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Document {
    pub id: String,
    pub entity: String,
    pub name: String,
    pub document_type: String,
    pub document_category: Option<String>,
    pub url: Option<String>,
    pub size: Option<u64>,
    pub created_at: u64,
}
