//! Typed identities. Each is a string on the wire (`#[serde(try_from, into)]`), checked when
//! parsed, so inside the program an id is always well-formed and never confused with another kind.

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::hash::short_hash;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum IdError {
    #[error("'{value}' is not a {kind}: expected `{prefix}` followed by 16 lowercase hex digits")]
    BadHashId {
        kind: &'static str,
        prefix: &'static str,
        value: String,
    },
    #[error(
        "'{0}' is not an anchor id: expected `kind:name` with a lowercase kind and no whitespace"
    )]
    BadAnchorId(String),
}

/// Content-addressed ids: a fixed prefix plus 16 hex digits of SHA-256.
macro_rules! hash_id {
    ($(#[$doc:meta])* $name:ident, $prefix:literal, $kind:literal) => {
        $(#[$doc])*
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        #[serde(try_from = "String", into = "String")]
        pub struct $name(String);

        impl $name {
            pub const PREFIX: &'static str = $prefix;

            /// Id of the given canonical content.
            #[must_use]
            pub fn of(content: &[u8]) -> Self {
                Self(short_hash($prefix, content))
            }

            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }

            fn is_valid(s: &str) -> bool {
                s.strip_prefix($prefix).is_some_and(|h| {
                    h.len() == 16 && h.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
                })
            }
        }

        impl TryFrom<String> for $name {
            type Error = IdError;
            fn try_from(s: String) -> Result<Self, IdError> {
                if Self::is_valid(&s) {
                    Ok(Self(s))
                } else {
                    Err(IdError::BadHashId { kind: $kind, prefix: $prefix, value: s })
                }
            }
        }

        impl std::str::FromStr for $name {
            type Err = IdError;
            fn from_str(s: &str) -> Result<Self, IdError> {
                Self::try_from(s.to_owned())
            }
        }

        impl From<$name> for String {
            fn from(id: $name) -> String {
                id.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(&self.0)
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                &self.0
            }
        }

        #[cfg(feature = "schema")]
        impl schemars::JsonSchema for $name {
            fn schema_name() -> String {
                stringify!($name).to_owned()
            }
            fn json_schema(_: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
                string_schema(concat!("^", $prefix, "[0-9a-f]{16}$"))
            }
        }
    };
}

hash_id!(
    /// A captured source (`s_…`).
    SourceId, "s_", "source id"
);
hash_id!(
    /// A perception observation (`o_…`).
    ObservationId, "o_", "observation id"
);
hash_id!(
    /// A claim (`c_…`): hash of its canonical content, see [`crate::Claim::compute_id`].
    ClaimId, "c_", "claim id"
);
hash_id!(
    /// A memory commit (`m_…`).
    CommitId, "m_", "commit id"
);
hash_id!(
    /// An episode (`e_…`).
    EpisodeId, "e_", "episode id"
);

/// A thing with identity and continuity, named `kind:name` (e.g. `repo:owner/name`,
/// `concept:agent-memory`). Not a hash: anchors are named by what they are.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct AnchorId(String);

impl AnchorId {
    /// The part before the first `:` (`repo`, `concept`, …).
    #[must_use]
    pub fn kind(&self) -> &str {
        self.0.split_once(':').map_or("", |(k, _)| k)
    }

    /// The part after the first `:`.
    #[must_use]
    pub fn name(&self) -> &str {
        self.0.split_once(':').map_or("", |(_, n)| n)
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Whether `s` has the shape of an anchor id. Used to tell ids from free text.
    #[must_use]
    pub fn looks_like(s: &str) -> bool {
        match s.split_once(':') {
            Some((k, n)) => {
                !k.is_empty()
                    && k.bytes()
                        .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
                    && !n.is_empty()
                    && !n.contains(char::is_whitespace)
            }
            None => false,
        }
    }
}

impl TryFrom<String> for AnchorId {
    type Error = IdError;
    fn try_from(s: String) -> Result<Self, IdError> {
        if Self::looks_like(&s) {
            Ok(Self(s))
        } else {
            Err(IdError::BadAnchorId(s))
        }
    }
}

impl std::str::FromStr for AnchorId {
    type Err = IdError;
    fn from_str(s: &str) -> Result<Self, IdError> {
        Self::try_from(s.to_owned())
    }
}

impl From<AnchorId> for String {
    fn from(id: AnchorId) -> String {
        id.0
    }
}

impl fmt::Display for AnchorId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl AsRef<str> for AnchorId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::borrow::Borrow<str> for AnchorId {
    fn borrow(&self) -> &str {
        &self.0
    }
}

#[cfg(feature = "schema")]
impl schemars::JsonSchema for AnchorId {
    fn schema_name() -> String {
        "AnchorId".to_owned()
    }
    fn json_schema(_: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        string_schema(r"^[a-z0-9-]+:\S+$")
    }
}

#[cfg(feature = "schema")]
fn string_schema(pattern: &str) -> schemars::schema::Schema {
    use schemars::schema::{InstanceType, SchemaObject, StringValidation};
    SchemaObject {
        instance_type: Some(InstanceType::String.into()),
        string: Some(Box::new(StringValidation {
            pattern: Some(pattern.to_owned()),
            ..Default::default()
        })),
        ..Default::default()
    }
    .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_ids_check_prefix_and_digits() {
        assert!("c_9b63203677b54ab2".parse::<ClaimId>().is_ok());
        assert!(
            "s_9b63203677b54ab2".parse::<ClaimId>().is_err(),
            "wrong prefix"
        );
        assert!(
            "c_9B63203677B54AB2".parse::<ClaimId>().is_err(),
            "uppercase"
        );
        assert!("c_9b63".parse::<ClaimId>().is_err(), "too short");
    }

    #[test]
    fn anchor_ids_split_kind_and_name() {
        let a: AnchorId = "repo:owner/name".parse().unwrap();
        assert_eq!((a.kind(), a.name()), ("repo", "owner/name"));
        assert!(
            "Concept:x".parse::<AnchorId>().is_err(),
            "kind must be lowercase"
        );
        assert!("no colon".parse::<AnchorId>().is_err());
        assert!("concept:two words".parse::<AnchorId>().is_err());
    }

    #[test]
    fn ids_are_plain_strings_on_the_wire() {
        let id: SourceId = "s_51518e9fc5c3bda7".parse().unwrap();
        assert_eq!(
            serde_json::to_string(&id).unwrap(),
            "\"s_51518e9fc5c3bda7\""
        );
        assert!(serde_json::from_str::<SourceId>("\"x\"").is_err());
    }
}
