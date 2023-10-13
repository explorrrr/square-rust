//! BankAccount

use serde::{Deserialize, Serialize};

use crate::api::models::enums::{country::Country, currency::Currency, bank_account_type::BankAccountType, bank_account_status::BankAccountStatus};

/// Represents a bank account.
/// For more information about linking a bank account to a Square account, see [Bank Accounts API](https://developer.squareup.com/docs/bank-accounts-api).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankAccount {
    /// The unique, Square-issued identifier for the bank account.
    /// Min Length 1
    /// Max Length 30
    pub id: String,
    /// The last few digits of the account number.
    /// Min Length 1
    pub account_number_suffix: String,
    /// The ISO 3166 Alpha-2 country code where the bank account is based.
    /// See [Country](#type-country) for possible values.
    pub country: Country,
    /// The 3-character ISO 4217 currency code indicating the operating currency of the bank account. For example, the currency code for US dollars is USD.
    /// See [Currency](#type-currency) for possible values.
    pub currency: Currency,
    /// The financial purpose of the associated bank account.
    /// See [BankAccountType](#type-bankaccounttype) for possible values.
    pub account_type: BankAccountType,
    /// Name of the account holder. This name must match the name on the targeted bank account record.
    /// Min Length 1
    pub holder_name: String,
    /// Primary identifier for the bank. For more information, see [Bank Accounts API](https://developer.squareup.com/docs/bank-accounts-api).
    /// Max Length 40
    pub primary_bank_identification_number: String,
    /// Secondary identifier for the bank. For more information, see [Bank Accounts API](https://developer.squareup.com/docs/bank-accounts-api).
    /// Max Length 40
    pub secondary_bank_identification_number: String,
    /// Reference identifier that will be displayed to UK bank account owners when collecting direct debit authorization. Only required for UK bank accounts.
    pub debit_mandate_reference_id: Option<String>,
    /// Client-provided identifier for linking the banking account to an entity in a third-party system (for example, a bank account number or a user identifier).
    pub reference_id: Option<String>,
    /// The location to which the bank account belongs.
    pub location_id: Option<String>,
    /// Read-only. The current verification status of this `BankAccount` object.
    /// See [BankAccountStatus](#type-bankaccountstatus) for possible values.
    pub status: BankAccountStatus,
    /// Indicates whether it is possible for Square to send money to this bank account.
    pub creditable: bool,
    /// Indicates whether it is possible for Square to take money from this bank account.
    pub debitable: bool,
    /// A Square-assigned, unique identifier for the bank account based on the account information. The account fingerprint can be used to compare account entries and determine if the they represent the same real-world bank account.
    pub fingerprint: Option<String>,
    /// The current version of the `BankAccount`.
    pub version: Option<i32>,
    /// Read only. Name of actual financial institution. For example "Bank of America".
    /// Max Length 100
    pub bank_name: Option<String>,
}
