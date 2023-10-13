//! CustomAttribute

use serde::{Deserialize, Serialize};

/// A custom attribute value.
///
/// Each custom attribute value has a corresponding CustomAttributeDefinition object.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomAttribute {
    /// The identifier of the custom attribute definition and its corresponding custom attributes. This value can be a simple key, which is the key that is provided when the custom attribute definition is created, or a qualified key, if the requesting application is not the definition owner. The qualified key consists of the application ID of the custom attribute definition owner followed by the simple key that was provided when the definition was created. It has the format application_id:simple key.
    ///
    /// The value for a simple key can contain up to 60 alphanumeric characters, periods (.), underscores (_), and hyphens (-).
    ///
    /// Min Length 1
    pub key: Option<String>,
    /// The value assigned to the custom attribute. It is validated against the custom attribute definition's schema on write operations. For more information about custom attribute values, see [Custom Attributes Overview](https://developer.squareup.com/docs/devtools/customattributes/overview).
    pub value: Option<serde_json::Value>,
    /// Read only. The current version of the custom attribute. This field is incremented when the custom attribute is changed. When updating an existing custom attribute value, you can provide this field and specify the current version of the custom attribute to enable [optimistic concurrency](https://developer.squareup.com/docs/build-basics/common-api-patterns/optimistic-concurrency). This field can also be used to enforce strong consistency for reads. For more information about strong consistency for reads, see [Custom Attributes Overview](https://developer.squareup.com/docs/devtools/customattributes/overview).
    pub version: Option<i32>,
    /// Read only A copy of the visibility field value for the associated custom attribute definition.
    pub visibility: Option<CustomAttributeDefinitionVisibility>,
    /// Read only A copy of the associated custom attribute definition object. This field is only set when the optional `expand` field is specified on the request.
    pub definition: Option<CustomAttributeDefinition>,
    /// Read only The timestamp that indicates when the custom attribute was created or was most recently updated, in RFC 3339 format.
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub updated_at: Option<String>,
    /// Read only The timestamp that indicates when the custom attribute was created, in RFC 3339 format.
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub created_at: Option<String>,
}
