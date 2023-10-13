//! DisputeEvidence

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisputeEvidence {
    /// The Square-generated ID of the evidence.
    ///
    /// Min Length 1 Max Length 40
    pub id: Option<String>,
    /// The ID of the dispute the evidence is associated with.
    ///
    /// Min Length 1 Max Length 40
    pub dispute_id: Option<String>,
    /// Image, PDF, TXT
    pub evidence_file: Option<DisputeEvidenceFile>,
    /// Raw text
    ///
    /// Min Length 1 Max Length 500
    pub evidence_text: Option<String>,
    /// The time when the evidence was uploaded, in RFC 3339 format.
    ///
    /// Min Length 1 Max Length 40
    pub uploaded_at: Option<String>,
    /// The type of the evidence.
    pub evidence_type: Option<DisputeEvidenceType>,
}
