//! Snippet

use serde::{Deserialize, Serialize};

/// Represents the snippet that is added to a Square Online site.
///
/// The snippet code is injected into the head element of all pages on the site, except for checkout pages.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnippetV20230925 {
    /// The Square-assigned ID for the snippet.
    ///
    /// Max Length 48
    pub id: Option<String>,
    /// The ID of the site that contains the snippet.
    pub site_id: Option<String>,
    /// The snippet code, which can contain valid HTML, JavaScript, or both.
    ///
    /// Min Length 1
    /// Max Length 65535
    pub content: String,
    /// The timestamp of when the snippet was initially added to the site, in RFC 3339 format.
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// * UTC: 2020-01-26T02:25:34Z
    ///
    /// * Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub created_at: Option<String>,
    /// The timestamp of when the snippet was last updated on the site, in RFC 3339 format.
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// * UTC: 2020-01-26T02:25:34Z
    ///
    /// * Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub updated_at: Option<String>,
}
