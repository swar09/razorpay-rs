# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [0.1.0-alpha.1] - 2026-08-16

### Added
- Complete asynchronous, strongly-typed Rust SDK for the Razorpay API.
- Support for core resources:
  - **Orders & Payments**: Create, fetch, update orders, capture payments, refunds, downtime check, card details, OTP and transfers.
  - **Payment Links & QR Codes**: Standard payment links and BharatQR/UPI QR code generation and closure.
  - **Subscriptions & Plans**: Full recurring billing engine, addon charges, subscription pausing/resuming.
  - **Route & Multi-Party Split Payments**: Linked accounts, stakeholders, product configuration, terms and conditions.
  - **Smart Collect & Virtual Accounts**: Virtual bank accounts and UPI IDs with automated payment reconciliation.
  - **Disputes & Chargebacks**: Dispute status retrieval, acceptances, and evidence submission.
  - **Documents Upload**: Multipart file uploads (`Documents::create`, `Documents::create_from_bytes`) for KYC and dispute evidence.
  - **Webhooks**: Complete webhook management resource (`Webhooks::create`, `fetch`, `edit`, `all`, `delete`) for standard and Route v2 accounts.
  - **Webhook & Payment Signatures**: Timing-attack safe HMAC-SHA256 signature verification and typed event dispatcher (`WebhookPayload`, `WebhookEventType`).
  - **Settlements & Bills**: Instant/on-demand settlement creation, combined reconciliation reports, and POS retail bills.
- Strong domain type-safety with dedicated status enums:
  - `RefundStatus`, `InvoiceStatus`, `PaymentLinkStatus`, `QrCodeStatus`, `DisputeStatus`, `DisputePhase`, `VirtualAccountStatus`, `SettlementStatus`.
- Granular error handling via `RazorpayError` (API error envelope parsing, HTTP/network errors, serde errors, IO errors, signature mismatches).

### Changed
- Parameter order for `verify_subscription_payment_signature` aligned with HMAC payload order (`payment_id`, `subscription_id`, `signature`, `secret`).
- V2 Route API endpoints use clean versioned routing (`post_v2`, `get_v2`, `patch_v2`, `delete_v2`).
- Optional request fields annotated with `skip_serializing_if = "Option::is_none"` to prevent unexpected `null` payloads.

---

## Versioning Policy

`razorpay-rs` follows **Semantic Versioning 2.0.0**:

- **`0.1.0-alpha.x`**: Initial alpha releases under active integration testing and API stabilization.
- **`0.1.0-beta.x`**: Feature-complete beta releases for sandbox and staging integration.
- **`0.1.0-rc.x`**: Release candidates before general production availability.
- **`0.1.0+`**: Initial public stable release.
- **`MAJOR.MINOR.PATCH`**:
  - `MAJOR`: Incompatible API breaking changes.
  - `MINOR`: New backwards-compatible features or resource additions.
  - `PATCH`: Backwards-compatible bug fixes and security patches.
