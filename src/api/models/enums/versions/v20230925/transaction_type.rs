//! TransactionType Enum

use serde::{Deserialize, Serialize};

/// The transaction type used in the disputed payment.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionTypeV20230925 {
    Debit,
    Credit,
}
