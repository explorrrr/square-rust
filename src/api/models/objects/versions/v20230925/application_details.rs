//! ApplicationDetails


use crate::api::models::enums::versions::v20230925::application_details_external_square_product::ApplicationDetailsExternalSquareProcutV20230925;


use serde::{Deserialize, Serialize};

/// Details about the application that took the payment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationDetailsV20230925 {
    /// The Square product, such as Square Point of Sale (POS), Square Invoices, or Square Virtual Terminal.
    pub square_product: Option<ApplicationDetailsExternalSquareProcutV20230925>,
    /// The Square ID assigned to the application used to take the payment. Application developers can use this information
    ///  to identify payments that their application processed. For example, if a developer uses a custom application to process payments,
    /// this field contains the application ID from the Developer Dashboard.
    /// If a seller uses a [Square App Marketplace](https://developer.squareup.com/docs/app-marketplace) application to process payments,
    /// the field contains the corresponding application ID.
    pub application_id: Option<String>,
}