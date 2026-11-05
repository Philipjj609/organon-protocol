//! Proposals: interpretation offered to memory. Memory validates them (known subjects, values
//! that look like ids must exist), resolves string values to anchors, attaches evidence.

use serde::{Deserialize, Serialize};

use crate::claim::{Anchor, Basis, Scope};

/// How evidence is attached to proposed claims that don't name any.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "lowercase")]
pub enum EvidencePolicy {
    /// Cite the sources behind observed/stated claims about the subject and the linked anchor.
    Auto,
}

/// A proposed claim: the short form `[about, relation, value, confidence?, rationale?]` or the
/// full object form. Values stay raw JSON until memory resolves them against what it knows.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum ProposedClaim {
    Short(Vec<serde_json::Value>),
    Full(serde_json::Map<String, serde_json::Value>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Proposal {
    pub author: String,
    pub message: String,
    /// Default basis for claims that don't state one.
    #[serde(default = "default_basis")]
    pub basis: Basis,
    /// Default confidence for inferred claims that don't state one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    #[serde(default)]
    pub scope: Scope,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evidence: Option<EvidencePolicy>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub anchors: Vec<Anchor>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub claims: Vec<ProposedClaim>,
}

fn default_basis() -> Basis {
    Basis::Inferred
}
