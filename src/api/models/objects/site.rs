//! Site

use serde::{Deserialize, Serialize};

/// Represents a Square Online site, which is an online store for a Square seller.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Site {
    /// Read only The Square-assigned ID of the site.
    ///
    /// Max Length 32
    pub id: Option<String>,
    /// The title of the site.
    pub site_title: Option<String>,
    /// The domain of the site (without the protocol). For example, mysite1.square.site.
    pub domain: Option<String>,
    /// Indicates whether the site is published.
    pub is_published: Option<bool>,
    /// Read only The timestamp of when the site was created, in RFC 3339 format.
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub created_at: Option<String>,
    /// Read only The timestamp of when the site was last updated, in RFC 3339 format.
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub updated_at: Option<String>,
}
