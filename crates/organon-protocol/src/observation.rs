//! Observations: what perception extracts. Located (in time, on a page, in a frame), typed,
//! attributed — and never an interpretation. Meaning is decided later, by comprehension.

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::id::{ObservationId, SourceId};
use crate::source::ProducedBy;

/// Where in a source an observation sits.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum Location {
    /// A time range in seconds (audio, video).
    Time { t0: f64, t1: f64 },
    /// A page, optionally a region `[x, y, w, h]` in page units (documents, images: page 1).
    Page {
        page: u32,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        bbox: Option<[f64; 4]>,
    },
    /// A single video frame, by index.
    Frame { frame: u64 },
    /// The whole source.
    Whole {},
}

/// Kind of observation. Open set; the first known kinds are constants here.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(transparent)]
pub struct ObservationKind(String);

impl ObservationKind {
    pub const TRANSCRIPT_SEGMENT: &'static str = "transcript_segment";
    pub const CHAPTER: &'static str = "chapter";
    pub const SHOT: &'static str = "shot";
    pub const KEYFRAME: &'static str = "keyframe";
    pub const SCREEN_TEXT: &'static str = "screen_text";
    pub const SPEAKER_TURN: &'static str = "speaker_turn";
    /// An aligned window (words + frames + screen text + cues): the unit comprehension reads.
    pub const MOMENT: &'static str = "moment";
    /// A reason to look at the picture: deictic phrase, slide change, chapter start.
    pub const VISUAL_CUE: &'static str = "visual_cue";

    #[must_use]
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ObservationKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// A located feature of a source.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Observation {
    pub id: ObservationId,
    pub source: SourceId,
    pub kind: ObservationKind,
    pub at: Location,
    /// Kind-specific payload (`{"text": …}`, `{"file": …, "phash": …}`).
    pub value: serde_json::Value,
    pub produced_by: ProducedBy,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    /// A correction replaces an earlier observation instead of overwriting it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub supersedes: Option<ObservationId>,
}

impl Observation {
    /// Content id of an observation: source, kind, location and value (not who produced it,
    /// so two tools agreeing produce the same id). **Contract.**
    #[must_use]
    pub fn compute_id(
        source: &SourceId,
        kind: &ObservationKind,
        at: &Location,
        value: &serde_json::Value,
    ) -> ObservationId {
        let key = serde_json::json!([source, kind, at, value]);
        ObservationId::of(key.to_string().as_bytes())
    }

    #[must_use]
    pub fn new(
        source: SourceId,
        kind: ObservationKind,
        at: Location,
        value: serde_json::Value,
        produced_by: ProducedBy,
    ) -> Self {
        Self {
            id: Self::compute_id(&source, &kind, &at, &value),
            source,
            kind,
            at,
            value,
            produced_by,
            confidence: None,
            supersedes: None,
        }
    }

    #[must_use]
    pub fn id_is_consistent(&self) -> bool {
        self.id == Self::compute_id(&self.source, &self.kind, &self.at, &self.value)
    }
}
