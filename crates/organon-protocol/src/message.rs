use serde::{Deserialize, Serialize};

use crate::claim::{Anchor, Claim};
use crate::episode::Episode;
use crate::observation::Observation;
use crate::proposal::Proposal;
use crate::source::Source;

/// Major version of the protocol this crate implements. Organs declare which they speak.
pub const PROTOCOL_VERSION: u32 = 0;

/// One line of a protocol stream: `{"type": "observation", …fields}`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(tag = "type", rename_all = "lowercase")]
#[allow(clippy::large_enum_variant)]
pub enum Message {
    Source(Source),
    Observation(Observation),
    Anchor(Anchor),
    Claim(Claim),
    Proposal(Proposal),
    Episode(Episode),
}

impl Message {
    #[must_use]
    pub const fn type_name(&self) -> &'static str {
        match self {
            Self::Source(_) => "source",
            Self::Observation(_) => "observation",
            Self::Anchor(_) => "anchor",
            Self::Claim(_) => "claim",
            Self::Proposal(_) => "proposal",
            Self::Episode(_) => "episode",
        }
    }
}
