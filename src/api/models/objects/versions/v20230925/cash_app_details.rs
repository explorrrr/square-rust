//! CashAppDetails

use serde::{Deserialize, Serialize};

use crate::api::models::enums::versions::v20230925::country::CountryV20230925;

/// Additional details about WALLET type payments with the brand of CASH_APP.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CashAppDetailsV20230925 {
    /// The name of the Cash App account holder.
    /// Max Length 255
    pub buyer_full_name: Option<String>,
    /// The country of the Cash App account holder, in ISO 3166-1-alpha-2 format.
    /// For possible values, see Country.
    /// Min Length 2
    /// Max Length 2
    pub buyer_country_code: Option<CountryV20230925>,
    /// Read only $Cashtag of the Cash App account holder.
    /// Min Length 1
    /// Max Length 21
    pub buyer_cashtag: Option<String>,
}
