use serde::{Deserialize, Serialize};

use crate::id::EpisodeId;

/// What something cost. All fields optional: record what you know.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Cost {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tokens_in: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tokens_out: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seconds: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usd: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bytes: Option<u64>,
}

impl Cost {
    /// Sum of two costs; the model is kept only if both agree.
    #[must_use]
    pub fn plus(&self, other: &Self) -> Self {
        fn add<T: std::ops::Add<Output = T> + Copy>(a: Option<T>, b: Option<T>) -> Option<T> {
            match (a, b) {
                (Some(x), Some(y)) => Some(x + y),
                (x, None) => x,
                (None, y) => y,
            }
        }
        Self {
            tokens_in: add(self.tokens_in, other.tokens_in),
            tokens_out: add(self.tokens_out, other.tokens_out),
            model: if self.model == other.model {
                self.model.clone()
            } else {
                None
            },
            seconds: add(self.seconds, other.seconds),
            usd: add(self.usd, other.usd),
            bytes: add(self.bytes, other.bytes),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "lowercase")]
pub enum EpisodeType {
    Recall,
    Research,
    Perceive,
    Digest,
    Capture,
    /// A declared piece of work with a budget (organon intelligence); its cost is the sum of
    /// everything done for it.
    Task,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "lowercase")]
pub enum Outcome {
    Good,
    Partial,
    Bad,
    #[default]
    Unknown,
}

/// What happened when the system did something: the raw material of learning.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Episode {
    pub id: EpisodeId,
    /// What kind of work this was. (Named `kind` because `type` tags stream messages.)
    pub kind: EpisodeType,
    pub task: String,
    /// RFC 3339.
    pub started: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ended: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub inputs: Vec<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub outputs: Vec<serde_json::Value>,
    #[serde(default)]
    pub outcome: Outcome,
    #[serde(default)]
    pub cost: Cost,
    /// Path to a JSONL trace of the run (plan, queries, sources seen and rejected).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trace: Option<String>,
}
