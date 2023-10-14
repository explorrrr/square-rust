//! DataCollectionOptions

use serde::{Deserialize, Serialize};

use super::collected_data::CollectedDataV20230925;
use crate::api::models::enums::versions::v20230925::data_collection_options_input_type::DataCollectionOptionsInputTypeV20230925;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataCollectionOptionsV20230925 {
    /// The title text to display in the data collection flow on the Terminal.
    /// Max Length: 250
    pub title: String,
    /// The body text to display under the title in the data collection screen flow on the Terminal.
    /// Max Length: 10000
    pub body: String,
    /// Represents the type of the input text.
    pub input_type: DataCollectionOptionsInputTypeV20230925,
    /// Read only The buyer’s input text from the data collection screen.
    pub collected_data: Option<CollectedDataV20230925>,
}
