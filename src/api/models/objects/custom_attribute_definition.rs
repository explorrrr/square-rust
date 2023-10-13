//! CustomAttributeDefinition

use serde::{Deserialize, Serialize};

/// Represents a definition for custom attribute values.
///
/// A custom attribute definition specifies the key, visibility, schema, and other properties for a custom attribute.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomArributeDefinition {
    /// The identifier of the custom attribute definition and its corresponding custom attributes. This value can be a simple key, which is the key that is provided when the custom attribute definition is created, or a qualified key, if the requesting application is not the definition owner. The qualified key consists of the application ID of the custom attribute definition owner followed by the simple key that was provided when the definition was created. It has the format application_id:simple key.
    ///
    /// The value for a simple key can contain up to 60 alphanumeric characters, periods (.), underscores (_), and hyphens (-).
    ///
    /// This field can not be changed after the custom attribute definition is created. This field is required when creating a definition and must be unique per application, seller, and resource type.
    ///
    /// Min Length 1
    pub key: String,
    /// The JSON schema for the custom attribute definition, which determines the data type of the corresponding custom attributes. For more information, see [Custom Attributes Overview](https://developer.squareup.com/docs/devtools/customattributes/overview). This field is required when creating a definition.
    pub schema: serde_json::Value,
    /// The name of the custom attribute definition for API and seller-facing UI purposes. The name must be unique within the seller and application pair. This field is required if the visibility field is `VISIBILITY_READ_ONLY` or `VISIBILITY_READ_WRITE_VALUES`.
    ///
    /// Max Length 255
    pub name: Option<String>,
    /// Seller-oriented description of the custom attribute definition, including any constraints that the seller should observe. May be displayed as a tooltip in Square UIs. This field is required if the visibility field is `VISIBILITY_READ_ONLY` or `VISIBILITY_READ_WRITE_VALUES`.
    ///
    /// Max Length 255
    pub description: Option<String>,
    /// Specifies how the custom attribute definition and its values should be shared with the seller and other applications. If no value is specified, the value defaults to `VISIBILITY_HIDDEN`.
    pub visibility: Option<CustomAttributeDefinitionVisibility>,
    /// Read only. The current version of the custom attribute definition. The value is incremented each time the custom attribute definition is updated. When updating a custom attribute definition, you can provide this field and specify the current version of the custom attribute definition to enable [optimistic concurrency](https://developer.squareup.com/docs/build-basics/common-api-patterns/optimistic-concurrency).
    ///
    /// On writes, this field must be set to the latest version. Stale writes are rejected.
    ///
    /// This field can also be used to enforce strong consistency for reads. For more information about strong consistency for reads, see [Custom Attributes Overview](https://developer.squareup.com/docs/devtools/customattributes/overview).
    pub version: Option<i32>,
    /// Read only The timestamp that indicates when the custom attribute definition was created or most recently updated, in RFC 3339 format.
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    /// * UTC: 2020-01-26T02:25:34Z
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub updated_at: Option<String>,
    /// Read only The timestamp that indicates when the custom attribute definition was created, in RFC 3339 format.
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    /// * UTC: 2020-01-26T02:25:34Z
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub created_at: Option<String>,
}
