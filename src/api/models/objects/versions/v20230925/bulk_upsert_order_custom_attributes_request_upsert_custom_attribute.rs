//! BulkUpsertOrderCustomAttributesRequestUpsertCustomAttribute

use serde::{Deserialize, Serialize};

use crate::api::models::objects::versions::v20230925::custom_attribute::CustomAttributeV20230925;

/// Represents one upsert within the bulk operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkUpsertOrderCustomAttributesRequestUpsertCustomAttributeV20230925 {
    /// REQUIRED
    /// The custom attribute to create or update, with the following fields:
    /// value. This value must conform to the schema specified by the definition. For more information, see [Value data types](https://developer.squareup.com/docs/customer-custom-attributes-api/custom-attributes#value-data-types).
    /// version. To enable [optimistic concurrency](https://developer.squareup.com/docs/build-basics/common-api-patterns/optimistic-concurrency) control, include this optional field and specify the current version of the custom attribute.
    pub custom_attribute: CustomAttributeV20230925,
    /// A unique identifier for this request, used to ensure idempotency. For more information, see Idempotency.
    /// Min Length 1
    /// Max Length 45
    pub idempotency_key: Option<String>,
    /// REQUIRED
    /// The ID of the target order.
    /// Min Length 1
    /// Max Length 255
    pub order_id: String,
}
