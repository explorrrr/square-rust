//! OauthAuthorizationRevokedEventRevocationObject

use serde::{Deserialize, Serialize};

use crate::api::models::enums::versions::v20230925::oauth_authorization_revoked_event_revoker_type::OauthAuthorizationRevokedEventRevokerTypeV20230925;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OauthAuthorizationRevokedEventRevocationObjectV20230925 {
    /// Timestamp of when the revocation event occurred, in RFC 3339 format.
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub revoked_at: Option<String>,
    /// Type of client that performed the revocation, either APPLICATION, MERCHANT, or SQUARE.
    pub revoker_type: Option<OauthAuthorizationRevokedEventRevokerTypeV20230925>,
}
