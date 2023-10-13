//! CollectedData

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectedData {
    /// Read only The buyer's input text.
    pub input_text: Option<String>,
}
