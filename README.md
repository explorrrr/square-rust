## Usage
### Installation
```
cargo add square-rust
```

### How to use
Example:
```rust
use square_rust::api::models::request::create_customer::versions::v20230925::CreateCustomerRequestV20230925;
use square_rust::config::SquareApiConfig;
use square_rust::http::client::http_client::SquareHttpClient;

let idempotency_key = None;
let given_name = Some("given_name".to_string());
let family_name = Some("family_name".to_string());
let company_name = None;
let nickname = None;
let email_address = None;
let address = None;
let phone_number = None;
let reference_id = None;
let note = None;
let birthday = None;
let tax_ids = None;
let config = SquareApiConfig::builder().build();
let http_client = SquareHttpClient::try_new(&config.http_client_config).unwrap();
let client = CustomersApi::new(config, http_client);
let request = CreateCustomerRequestV20230925::new(
    idempotency_key,
    given_name,
    family_name,
    company_name,
    nickname,
    email_address,
    address,
    phone_number,
    reference_id,
    note,
    birthday,
    tax_ids,
);
let _ = client.create_customer(request).await.unwrap();
```

### Planed features

- Payments
  - Payments
  - Refunds
  - Disputes
  - Checkout
  - Apple pay
  - Cards
  - Payouts
- Terminal
- Orders
- Subscriptions
- Invoices
- Catalog
- Inventory
- Customers
- Loyalty
- Gift Cards
- Bookings
- Business
- Team
- Financials
- Online
- Auth
- Webhook Subscriptions
