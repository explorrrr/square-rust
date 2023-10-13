//! TerminalCheckoutQuery

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalCheckoutQuery {
    /// Options for filtering returned TerminalCheckout objects.
    pub filter: Option<TerminalCheckoutQueryFilter>,
    /// Option for sorting returned TerminalCheckout objects.
    pub sort: Option<TerminalCheckoutQuerySort>,
}
