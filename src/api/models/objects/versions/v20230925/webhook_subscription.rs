//! WebhookSubscription

use serde::{Deserialize, Serialize};

/// Represents the details of a webhook subscription, including notification URL, event types, and signature key.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookSubscriptionV20230925 {
    /// Read only A Square-generated unique ID for the subscription.
    ///
    /// Max Length 64
    pub id: Option<String>,
    /// The name of this subscription.
    ///
    /// Max Length 64
    pub name: Option<String>,
    /// Indicates whether the subscription is enabled (true) or not (false).
    pub enabled: Option<bool>,
    /// The event types associated with this subscription.
    pub event_types: Option<Vec<String>>,
    /// The URL to which webhooks are sent.
    pub notification_url: Option<String>,
    /// The API version of the subscription. This field is optional for CreateWebhookSubscription. The value defaults to the API version used by the application.
    pub api_version: Option<String>,
    /// Read only The Square-generated signature key used to validate the origin of the webhook event.
    pub signature_key: Option<String>,
    /// Read only The timestamp of when the subscription was created, in RFC 3339 format. For example, "2016-09-04T23:59:33.123Z".
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub created_at: Option<String>,
    /// Read only The timestamp of when the subscription was last updated, in RFC 3339 format. For example, "2016-09-04T23:59:33.123Z".
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub updated_at: Option<String>,
}