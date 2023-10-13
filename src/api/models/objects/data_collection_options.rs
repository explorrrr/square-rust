//! DataCollectionOptions

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataCollectionOptions {
    /// The title text to display in the data collection flow on the Terminal.
    /// Max Length: 250
    pub title: String,
    /// The body text to display under the title in the data collection screen flow on the Terminal.
    /// Max Length: 10000
    pub body: String,
    /// Represents the type of the input text.
    pub input_type: DataCollectionOptionsInputType,
    /// Read only The buyer’s input text from the data collection screen.
    pub collected_data: Option<CollectedData>,
}
