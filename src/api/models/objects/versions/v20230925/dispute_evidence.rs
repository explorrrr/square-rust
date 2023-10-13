//! DisputeEvidence

use serde::{Deserialize, Serialize};

use crate::api::models::enums::versions::v20230925::dispute_evidence_type::DisputeEvidenceTypeV20230925;

use super::dispute_evidence_file::DisputeEvidenceFileV20230925;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisputeEvidenceV20230925 {
    /// The Square-generated ID of the evidence.
    ///
    /// Min Length 1 Max Length 40
    pub id: Option<String>,
    /// The ID of the dispute the evidence is associated with.
    ///
    /// Min Length 1 Max Length 40
    pub dispute_id: Option<String>,
    /// Image, PDF, TXT
    pub evidence_file: Option<DisputeEvidenceFileV20230925>,
    /// Raw text
    ///
    /// Min Length 1 Max Length 500
    pub evidence_text: Option<String>,
    /// The time when the evidence was uploaded, in RFC 3339 format.
    ///
    /// Min Length 1 Max Length 40
    pub uploaded_at: Option<String>,
    /// The type of the evidence.
    pub evidence_type: Option<DisputeEvidenceTypeV20230925>,
}
