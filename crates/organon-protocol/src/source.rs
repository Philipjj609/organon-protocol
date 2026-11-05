use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::id::SourceId;

/// Who produced an object: organ, version, and the tool or model behind it.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ProducedBy {
    pub organ: String,
    pub version: String,
    /// External tool used (`yt-dlp 2026.10`, `ffmpeg 7.1`, `tesseract 5`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool: Option<String>,
    /// Model used, if any (`claude-haiku-5-5`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
}

/// What a sense captured: immutable evidence. Its bytes live with the organ or memory that
/// captured it (never in a public repo if they are third-party content); this record travels.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Source {
    pub id: SourceId,
    /// `github-star`, `web-page`, `x-post`, `youtube-video`, `local-file`, …
    pub kind: String,
    pub uri: String,
    pub title: String,
    /// RFC 3339.
    pub captured_at: String,
    /// Lowercase hex SHA-256 of the captured content.
    pub content_hash: String,
    /// Named files that belong to this source (captions, audio, frames dir), relative paths.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub files: BTreeMap<String, String>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub meta: BTreeMap<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub produced_by: Option<ProducedBy>,
}
