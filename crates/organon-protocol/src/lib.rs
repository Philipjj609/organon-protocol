//! The shared language of organon organs.
//!
//! Every organ (memory, research, learning, perception of video, sound, images, text) reads
//! and writes these objects, one JSON object per line, wrapped in a [`Message`]. Depend on
//! this crate, never on another organ: that is what makes organs interchangeable.
//!
//! - [`Source`]: what a sense captured (raw signal + metadata, content-hashed).
//! - [`Observation`]: a located feature extracted by perception, with no interpretation.
//! - [`Claim`]: a qualified statement with basis, stance, scope, time and evidence.
//! - [`Proposal`]: interpretation offered to memory (anchors + claims to validate).
//! - [`Episode`]: what happened when the system did something, with its [`Cost`].
//!
//! Identity is content: ids are prefixed SHA-256 digests of a canonical form, so the same
//! thing produced twice has the same id. The canonical forms are contracts (see `tests/golden.rs`).

#![forbid(unsafe_code)]

mod check;
mod claim;
mod episode;
mod hash;
mod id;
mod message;
mod observation;
mod proposal;
mod source;

pub use check::{check_stream, LineError, Report};
pub use claim::{
    Anchor, Basis, Claim, ClaimBody, ClaimDraft, EvidenceRef, Family, Relation, RelationInfo,
    Scope, Stance, Status, Subject, Value,
};
pub use episode::{Cost, Episode, EpisodeType, Outcome};
pub use hash::{sha256_hex, short_hash};
pub use id::{AnchorId, ClaimId, CommitId, EpisodeId, IdError, ObservationId, SourceId};
pub use message::{Message, PROTOCOL_VERSION};
pub use observation::{Location, Observation, ObservationKind};
pub use proposal::{EvidencePolicy, Proposal, ProposedClaim};
pub use source::{ProducedBy, Source};
