# Razorpay Go SDK (`github.com/razorpay/razorpay-go`) - API Function Reference

Complete reference of all client resources, functions, arguments, and return types present in the official Razorpay Go SDK.

---

## SDK Design Overview
- **Base Client Initialization:**
  ```go
  client := razorpay.NewClient("<KEY_ID>", "<KEY_SECRET>")
  ```
- **Consistent Method Signatures:**
  In `razorpay-go`, almost all resource methods take:
  1. Primary ID (`string`) if operating on a specific resource.
  2. Optional/Mandatory payload (`data map[string]interface{}` or `queryParams map[string]interface{}`).
  3. Optional custom headers (`extraHeaders map[string]string`).
- **Return Type:** `(map[string]interface{}, error)` across all resource endpoints.

---

## 1. Payments (`client.Payment`)

| Method | Arguments | Return Type | Description |
| :--- | :--- | :--- | :--- |
| `All` | `queryParams map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Fetch all payments |
| `Fetch` | `paymentID string`, `queryParams map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Fetch payment by ID |
| `Capture` | `paymentID string`, `amount int`, `queryParams map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Capture an authorized payment |
| `Refund` | `paymentID string`, `amount int`, `queryParams map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Create a refund for a payment |
| `Transfer` | `paymentID string`, `data map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Transfer payment to linked accounts |
| `BankTransfer` | `paymentID string`, `queryParams map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Fetch bank transfer details |
| `FetchCardDetails` | `paymentID string`, `queryParams map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Fetch card details of a payment |
| `FetchPaymentDowntime` | `queryParams map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Fetch payment downtime details |
| `FetchPaymentDowntimeById` | `downtimeID string`, `queryParams map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Fetch downtime details by ID |
| `Edit` | `paymentID string`, `data map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Update payment (notes) |
| `OtpGenerate` | `paymentID string`, `data map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Generate OTP for a payment |
| `OtpResend` | `paymentID string`, `data map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Resend OTP |
| `OtpSubmit` | `paymentID string`, `data map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Submit OTP for authentication |

---

## 2. Orders (`client.Order`)

| Method | Arguments | Return Type | Description |
| :--- | :--- | :--- | :--- |
| `All` | `queryParams map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Fetch all orders |
| `Fetch` | `orderID string`, `queryParams map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Fetch order by ID |
| `Create` | `data map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Create a new order |
| `Update` | `orderID string`, `data map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Update order details (notes) |
| `Payments` | `orderID string`, `queryParams map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Fetch payments for an order |

---

## 3. Refunds (`client.Refund`)

| Method | Arguments | Return Type | Description |
| :--- | :--- | :--- | :--- |
| `All` | `queryParams map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Fetch all refunds |
| `Fetch` | `refundID string`, `queryParams map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Fetch refund by ID |
| `Create` | `data map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Create a normal or instant refund |
| `Edit` | `refundID string`, `data map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Update refund details (notes) |

---

## 4. Customers (`client.Customer`)

| Method | Arguments | Return Type | Description |
| :--- | :--- | :--- | :--- |
| `All` | `queryParams map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Fetch all customers |
| `Fetch` | `customerID string`, `queryParams map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Fetch customer by ID |
| `Create` | `data map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Create a customer |
| `Edit` | `customerID string`, `data map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Edit customer details |
| `FetchTokens` | `customerID string`, `queryParams map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Fetch saved card tokens |
| `FetchToken` | `customerID string`, `tokenID string`, `queryParams map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Fetch a specific token |
| `DeleteToken` | `customerID string`, `tokenID string`, `queryParams map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Delete customer token |

---

## 5. Plans (`client.Plan`)

| Method | Arguments | Return Type | Description |
| :--- | :--- | :--- | :--- |
| `All` | `queryParams map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Fetch all plans |
| `Fetch` | `planID string`, `queryParams map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Fetch plan by ID |
| `Create` | `data map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Create a recurring billing plan |

---

## 6. Subscriptions (`client.Subscription`)

| Method | Arguments | Return Type | Description |
| :--- | :--- | :--- | :--- |
| `All` | `queryParams map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Fetch all subscriptions |
| `Fetch` | `subscriptionID string`, `queryParams map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Fetch subscription by ID |
| `Create` | `data map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Create a new subscription |
| `Cancel` | `subscriptionID string`, `data map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Cancel subscription |
| `Update` | `subscriptionID string`, `data map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Update subscription |
| `PendingUpdate` | `subscriptionID string`, `queryParams map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Fetch pending update details |
| `CancelScheduledChanges` | `subscriptionID string`, `data map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Cancel scheduled changes |
| `Pause` | `subscriptionID string`, `data map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Pause subscription |
| `Resume` | `subscriptionID string`, `data map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Resume paused subscription |
| `DeleteOffer` | `subscriptionID string`, `offerID string`, `queryParams map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Delete offer on subscription |

---

## 7. Addons (`client.Addon`)

| Method | Arguments | Return Type | Description |
| :--- | :--- | :--- | :--- |
| `Fetch` | `addonID string`, `queryParams map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Fetch addon by ID |
| `Delete` | `addonID string`, `queryParams map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Delete an addon |
| `All` | `queryParams map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Fetch all addons |

---

## 8. Payment Links / Invoices (`client.PaymentLink` / `client.Invoice`)

| Method | Arguments | Return Type | Description |
| :--- | :--- | :--- | :--- |
| `All` | `queryParams map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Fetch all invoices/payment links |
| `Fetch` | `invoiceID string`, `queryParams map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Fetch invoice by ID |
| `Create` | `data map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Create invoice / payment link |
| `Cancel` | `invoiceID string`, `queryParams map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Cancel invoice / payment link |
| `Edit` | `invoiceID string`, `data map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Update invoice / payment link |
| `NotifyBy` | `invoiceID string`, `medium string`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Resend notification (`sms`/`email`) |
| `Issue` | `invoiceID string`, `queryParams map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Issue a draft invoice |
| `Delete` | `invoiceID string`, `queryParams map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Delete a draft invoice |

---

## 9. Items (`client.Item`)

| Method | Arguments | Return Type | Description |
| :--- | :--- | :--- | :--- |
| `All` | `queryParams map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Fetch all items |
| `Fetch` | `itemID string`, `queryParams map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Fetch item by ID |
| `Create` | `data map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Create a line item |
| `Update` | `itemID string`, `data map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Update line item |
| `Delete` | `itemID string`, `queryParams map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Delete line item |

---

## 10. QR Codes (`client.QrCode`)

| Method | Arguments | Return Type | Description |
| :--- | :--- | :--- | :--- |
| `All` | `queryParams map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Fetch all QR codes |
| `Fetch` | `qrCodeID string`, `queryParams map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Fetch QR code by ID |
| `Create` | `data map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Create a QR code |
| `Close` | `qrCodeID string`, `queryParams map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Close/deactivate a QR code |
| `FetchPayments` | `qrCodeID string`, `queryParams map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Fetch payments on a QR code |

---

## 11. Settlements (`client.Settlement`)

| Method | Arguments | Return Type | Description |
| :--- | :--- | :--- | :--- |
| `All` | `queryParams map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Fetch all settlements |
| `Fetch` | `settlementID string`, `queryParams map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Fetch settlement by ID |
| `Reports` | `queryParams map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Fetch settlement recon reports |
| `CreateOndemandSettlement` | `data map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Create instant on-demand settlement |
| `FetchAllOndemandSettlement`| `queryParams map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Fetch all on-demand settlements |
| `FetchOndemandSettlementById`| `ondemandID string`, `queryParams map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Fetch on-demand settlement by ID |

---

## 12. Transfers & Route (`client.Transfer`)

| Method | Arguments | Return Type | Description |
| :--- | :--- | :--- | :--- |
| `All` | `queryParams map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Fetch all transfers |
| `Fetch` | `transferID string`, `queryParams map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Fetch transfer by ID |
| `Create` | `data map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Create a direct transfer |
| `Edit` | `transferID string`, `data map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Update transfer (hold/release) |
| `Reverse` | `transferID string`, `data map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Reverse a transfer |
| `Reversals` | `transferID string`, `queryParams map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Fetch reversals for a transfer |

---

## 13. Virtual Accounts / Smart Collect (`client.VirtualAccount`)

| Method | Arguments | Return Type | Description |
| :--- | :--- | :--- | :--- |
| `All` | `queryParams map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Fetch all virtual accounts |
| `Fetch` | `vaID string`, `queryParams map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Fetch virtual account by ID |
| `Create` | `data map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Create a virtual account |
| `Close` | `vaID string`, `queryParams map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Close a virtual account |
| `Payments` | `vaID string`, `queryParams map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Fetch payments received on VA |
| `AddReceiver` | `vaID string`, `data map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Add receiver (bank/VPA) to VA |

---

## 14. Disputes (`client.Dispute`)

| Method | Arguments | Return Type | Description |
| :--- | :--- | :--- | :--- |
| `All` | `queryParams map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Fetch all disputes |
| `Fetch` | `disputeID string`, `queryParams map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Fetch dispute by ID |
| `Accept` | `disputeID string`, `data map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Accept a dispute |
| `Contest` | `disputeID string`, `data map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Contest dispute with evidence |

---

## 15. Linked Accounts (`client.Account`)

| Method | Arguments | Return Type | Description |
| :--- | :--- | :--- | :--- |
| `Create` | `data map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Create a linked account |
| `Fetch` | `accountID string`, `queryParams map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Fetch account by ID |
| `Edit` | `accountID string`, `data map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Update linked account details |
| `Delete` | `accountID string`, `queryParams map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Delete a linked account |

---

## 16. Stakeholders (`client.Stakeholder`)

| Method | Arguments | Return Type | Description |
| :--- | :--- | :--- | :--- |
| `Create` | `accountID string`, `data map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Create a stakeholder for an account |
| `Fetch` | `accountID string`, `stakeholderID string`, `queryParams map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Fetch stakeholder by ID |
| `All` | `accountID string`, `queryParams map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Fetch all stakeholders of an account |
| `Edit` | `accountID string`, `stakeholderID string`, `data map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Update stakeholder details |

---

## 17. Documents (`client.Document`)

| Method | Arguments | Return Type | Description |
| :--- | :--- | :--- | :--- |
| `Create` | `data map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Upload/create a document |
| `Fetch` | `documentID string`, `queryParams map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Fetch document metadata |

---

## 18. Cards & Tokens (`client.Card`, `client.Token`)

| Resource / Method | Arguments | Return Type | Description |
| :--- | :--- | :--- | :--- |
| `Card.Fetch` | `cardID string`, `queryParams map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Fetch card details by ID |
| `Token.Fetch` | `customerID string`, `tokenID string`, `queryParams map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Fetch token by ID |
| `Token.Delete` | `customerID string`, `tokenID string`, `queryParams map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Delete a saved token |

---

## 19. Fund Accounts & Payouts (`client.FundAccount`, `client.Payout`)

| Resource / Method | Arguments | Return Type | Description |
| :--- | :--- | :--- | :--- |
| `FundAccount.Create` | `data map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Create fund account (bank/VPA/card) |
| `FundAccount.All` | `queryParams map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Fetch all fund accounts |
| `Payout.Create` | `data map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Create a payout |
| `Payout.Fetch` | `payoutID string`, `queryParams map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Fetch payout by ID |
| `Payout.Cancel` | `payoutID string`, `queryParams map[string]interface{}`, `extraHeaders map[string]string` | `(map[string]interface{}, error)` | Cancel a queued payout |

---

## 20. Utilities & Signature Verification (`utils`)

| Function | Arguments | Return Type | Description |
| :--- | :--- | :--- | :--- |
| `VerifyWebhookSignature` | `body string`, `signature string`, `secret string` | `bool` | Verify incoming webhook HMAC-SHA256 signature |
| `VerifyPaymentSignature` | `params map[string]interface{}`, `secret string` | `bool` | Verify checkout signature (`order_id\|payment_id`) |
| `VerifySubscriptionPaymentSignature` | `params map[string]interface{}`, `secret string` | `bool` | Verify subscription signature (`payment_id\|subscription_id`) |
