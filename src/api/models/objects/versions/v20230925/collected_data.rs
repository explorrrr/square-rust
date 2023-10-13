//! CollectedData

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectedDataV20230925 {
    /// Read only The buyer's input text.
    pub input_text: Option<String>,
}
