//! Claims: the atom of memory. A qualified statement about an anchor (or about another claim).

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::id::{AnchorId, ClaimId, IdError, ObservationId, SourceId};

/// Where a claim's knowledge comes from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "lowercase")]
pub enum Basis {
    /// A tool observed it in a source (API response, file, parse).
    Observed,
    /// A source said it (an author, a README, Philip).
    Stated,
    /// A model or person reasoned to it; carries confidence and rationale.
    Inferred,
    /// Taken wholesale from another knowledge system.
    Imported,
    /// An intuition, kept as an intuition.
    Intuited,
}

impl Basis {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Observed => "observed",
            Self::Stated => "stated",
            Self::Inferred => "inferred",
            Self::Imported => "imported",
            Self::Intuited => "intuited",
        }
    }

    /// Strength for ranking evidence: observed is strongest.
    #[must_use]
    pub const fn rank(self) -> u8 {
        match self {
            Self::Observed => 0,
            Self::Stated => 1,
            Self::Imported => 2,
            Self::Inferred => 3,
            Self::Intuited => 4,
        }
    }
}

/// Whether a claim describes reality, an intention, a possibility, a fiction's canon, or a what-if.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "lowercase")]
pub enum Stance {
    #[default]
    Is,
    Should,
    Could,
    Canon,
    Hypothetical,
}

impl Stance {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Is => "is",
            Self::Should => "should",
            Self::Could => "could",
            Self::Canon => "canon",
            Self::Hypothetical => "hypothetical",
        }
    }
}

/// Lifecycle of a claim in memory. Claims are never deleted, only moved out of `Active`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Active,
    Superseded,
    Contradicted,
    Parked,
    Retracted,
}

impl Status {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Superseded => "superseded",
            Self::Contradicted => "contradicted",
            Self::Parked => "parked",
            Self::Retracted => "retracted",
        }
    }
}

/// What a claim is about: an anchor, or another claim (claims about claims are allowed).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub enum Subject {
    Anchor(AnchorId),
    Claim(ClaimId),
}

impl Subject {
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Anchor(a) => a.as_str(),
            Self::Claim(c) => c.as_str(),
        }
    }

    #[must_use]
    pub fn anchor(&self) -> Option<&AnchorId> {
        match self {
            Self::Anchor(a) => Some(a),
            Self::Claim(_) => None,
        }
    }
}

impl TryFrom<String> for Subject {
    type Error = IdError;
    fn try_from(s: String) -> Result<Self, IdError> {
        if s.starts_with(ClaimId::PREFIX) && !s.contains(':') {
            ClaimId::try_from(s).map(Self::Claim)
        } else {
            AnchorId::try_from(s).map(Self::Anchor)
        }
    }
}

impl std::str::FromStr for Subject {
    type Err = IdError;
    fn from_str(s: &str) -> Result<Self, IdError> {
        Self::try_from(s.to_owned())
    }
}

impl From<Subject> for String {
    fn from(s: Subject) -> String {
        match s {
            Subject::Anchor(a) => a.into(),
            Subject::Claim(c) => c.into(),
        }
    }
}

impl From<AnchorId> for Subject {
    fn from(a: AnchorId) -> Self {
        Self::Anchor(a)
    }
}

impl fmt::Display for Subject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(feature = "schema")]
impl schemars::JsonSchema for Subject {
    fn schema_name() -> String {
        "Subject".to_owned()
    }
    fn json_schema(g: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        <String as schemars::JsonSchema>::json_schema(g)
    }
}

/// The object of a claim.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "lowercase")]
pub enum Value {
    Anchor(AnchorId),
    Text(String),
    Number(f64),
    Bool(bool),
}

impl Value {
    #[must_use]
    pub fn anchor(&self) -> Option<&AnchorId> {
        match self {
            Self::Anchor(a) => Some(a),
            _ => None,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Anchor(a) => f.write_str(a.as_str()),
            Self::Text(t) => f.write_str(t),
            // Whole numbers print without a fraction: 58947, not 58947.0.
            #[allow(clippy::cast_possible_truncation)]
            Self::Number(n) if n.fract() == 0.0 && n.abs() < 1e15 => write!(f, "{}", *n as i64),
            Self::Number(n) => write!(f, "{n}"),
            Self::Bool(b) => write!(f, "{b}"),
        }
    }
}

/// A named relation. Open set (any domain may add relations); known ones have semantics in
/// [`Relation::info`].
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(transparent)]
pub struct Relation(String);

/// The edge layer a relation belongs to (used for filtering and weighting).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "lowercase")]
pub enum Family {
    Capability,
    Taxonomy,
    Workflow,
    Peer,
    Lineage,
    Use,
    Meaning,
    Message,
    Bounds,
    Mention,
    Topic,
    Meta,
    Personal,
    Frontier,
    State,
    Other,
}

impl Family {
    pub const ALL: [Self; 16] = [
        Self::Capability,
        Self::Taxonomy,
        Self::Workflow,
        Self::Peer,
        Self::Lineage,
        Self::Use,
        Self::Meaning,
        Self::Message,
        Self::Bounds,
        Self::Mention,
        Self::Topic,
        Self::Meta,
        Self::Personal,
        Self::Frontier,
        Self::State,
        Self::Other,
    ];

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Capability => "capability",
            Self::Taxonomy => "taxonomy",
            Self::Workflow => "workflow",
            Self::Peer => "peer",
            Self::Lineage => "lineage",
            Self::Use => "use",
            Self::Meaning => "meaning",
            Self::Message => "message",
            Self::Bounds => "bounds",
            Self::Mention => "mention",
            Self::Topic => "topic",
            Self::Meta => "meta",
            Self::Personal => "personal",
            Self::Frontier => "frontier",
            Self::State => "state",
            Self::Other => "other",
        }
    }

    /// Families that describe *what something means*, as opposed to metadata, tags or
    /// personal bookkeeping. Used for ranking and for "how connected is this".
    #[must_use]
    pub const fn is_meaningful(self) -> bool {
        !matches!(self, Self::Meta | Self::Topic | Self::Personal)
    }
}

/// What the protocol knows about a relation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RelationInfo {
    pub family: Family,
    /// Single-valued: a new value supersedes the old one instead of coexisting.
    pub functional: bool,
    /// Stance a new claim with this relation gets by default.
    pub default_stance: Stance,
}

impl Relation {
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Semantics of known relations; unknown relations are `Other`, multi-valued, `Is`.
    #[must_use]
    pub fn info(&self) -> RelationInfo {
        use Family as F;
        let (family, functional, default_stance) = match self.0.as_str() {
            "described_as" | "homepage" | "license" | "language" | "stars" | "owned_by"
            | "documented_in" | "title" | "likes" | "followers" | "downloads"
            | "capture_status" | "posted_by" | "pricing" | "has_media" | "has_article"
            | "channel" | "duration" | "has_captions" => (F::Meta, true, Stance::Is),
            "tagged" => (F::Topic, false, Stance::Is),
            "starred_by" | "saved_by" | "note" | "saved_group" => (F::Personal, false, Stance::Is),
            "links_to" | "listed_in" | "mentions" => (F::Mention, false, Stance::Is),
            "says" | "teaches" | "claims" | "shows" => (F::Message, false, Stance::Is),
            "speaker" | "speaks_in" => (F::Mention, false, Stance::Is),
            "used_at_stage" | "precedes" => (F::Workflow, false, Stance::Is),
            "provides" | "implements" => (F::Capability, false, Stance::Is),
            "part_of" | "instance_of" => (F::Taxonomy, false, Stance::Is),
            "alternative_to" | "complements" => (F::Peer, false, Stance::Is),
            "builds_on" | "inspired_by" => (F::Lineage, false, Stance::Is),
            "relevant_to" => (F::Use, false, Stance::Could),
            "uses" | "needs" | "used_in" => (F::Use, false, Stance::Is),
            "defined_as" | "purpose" | "mechanism" => (F::Meaning, false, Stance::Is),
            "limitation" | "failure_mode" | "constraint" => (F::Bounds, false, Stance::Is),
            "question" => (F::Frontier, false, Stance::Is),
            "status" | "verdict" => (F::State, true, Stance::Is),
            _ => (F::Other, false, Stance::Is),
        };
        RelationInfo {
            family,
            functional,
            default_stance,
        }
    }
}

impl From<&str> for Relation {
    fn from(s: &str) -> Self {
        Self(s.to_owned())
    }
}

impl fmt::Display for Relation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Where a claim is true: `world`, `domain:x`, `project:x`, `personal:philip`, `canon:x`, `play`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(transparent)]
pub struct Scope(String);

impl Scope {
    #[must_use]
    pub fn world() -> Self {
        Self("world".to_owned())
    }
    #[must_use]
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
    #[must_use]
    pub fn is_world(&self) -> bool {
        self.0 == "world"
    }
}

impl Default for Scope {
    fn default() -> Self {
        Self::world()
    }
}

impl fmt::Display for Scope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// One piece of evidence: a source (optionally a span of it), or a perception observation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum EvidenceRef {
    Source {
        source: SourceId,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        span: Option<String>,
    },
    Observation {
        observation: ObservationId,
    },
}

impl EvidenceRef {
    #[must_use]
    pub fn source(id: SourceId) -> Self {
        Self::Source {
            source: id,
            span: None,
        }
    }
}

/// A thing with identity. All meaning about it lives in claims.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Anchor {
    pub id: AnchorId,
    pub name: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub aliases: Vec<String>,
    #[serde(default)]
    pub scope: Scope,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
}

impl Anchor {
    /// The kind is the id's prefix (`repo`, `concept`, …), so it can never disagree with the id.
    #[must_use]
    pub fn kind(&self) -> &str {
        self.id.kind()
    }
}

/// A qualified statement. Build one with [`Claim::draft`] and [`ClaimDraft::finalize`], which
/// computes the content-addressed id.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Claim {
    pub id: ClaimId,
    #[serde(flatten)]
    pub body: ClaimBody,
}

/// Everything in a claim except its id.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ClaimBody {
    pub about: Subject,
    pub relation: Relation,
    pub value: Value,
    #[serde(default)]
    pub scope: Scope,
    /// When this became true in the world (not when we learned it; that's the commit time).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid_from: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid_to: Option<String>,
    pub basis: Basis,
    #[serde(default)]
    pub stance: Stance,
    /// Only for inferred claims.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rationale: Option<String>,
    /// Groups of evidence: every ref in a group is jointly required; groups are alternatives.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub evidence: Vec<Vec<EvidenceRef>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub derived_from: Vec<ClaimId>,
    pub author: String,
}

/// A claim under construction.
pub type ClaimDraft = ClaimBody;

impl ClaimBody {
    /// The canonical content that identifies a claim. Same statement → same id, whoever made it
    /// and with whatever evidence. **Contract**: changing this changes every claim id.
    #[must_use]
    pub fn compute_id(&self) -> ClaimId {
        let key = serde_json::json!([
            self.about,
            self.relation,
            self.value,
            self.scope,
            self.valid_from,
            self.stance
        ]);
        ClaimId::of(key.to_string().as_bytes())
    }

    #[must_use]
    pub fn finalize(self) -> Claim {
        Claim {
            id: self.compute_id(),
            body: self,
        }
    }

    #[must_use]
    pub fn with_evidence(mut self, source: SourceId) -> Self {
        self.evidence.push(vec![EvidenceRef::source(source)]);
        self
    }

    /// Distinct sources cited anywhere in the evidence, in order of first appearance.
    pub fn evidence_sources(&self) -> impl Iterator<Item = &SourceId> {
        let mut seen: Vec<&SourceId> = Vec::new();
        self.evidence.iter().flatten().filter_map(move |e| match e {
            EvidenceRef::Source { source, .. } if !seen.contains(&source) => {
                seen.push(source);
                Some(source)
            }
            _ => None,
        })
    }
}

impl Claim {
    /// Start a claim; the stance defaults to the relation's default.
    #[must_use]
    pub fn draft(
        about: impl Into<Subject>,
        relation: impl Into<Relation>,
        value: Value,
        basis: Basis,
        author: &str,
    ) -> ClaimDraft {
        let relation = relation.into();
        ClaimBody {
            about: about.into(),
            stance: relation.info().default_stance,
            relation,
            value,
            scope: Scope::world(),
            valid_from: None,
            valid_to: None,
            basis,
            confidence: None,
            rationale: None,
            evidence: Vec::new(),
            derived_from: Vec::new(),
            author: author.to_owned(),
        }
    }

    /// Whether the stored id matches the content.
    #[must_use]
    pub fn id_is_consistent(&self) -> bool {
        self.id == self.body.compute_id()
    }
}

impl std::ops::Deref for Claim {
    type Target = ClaimBody;
    fn deref(&self) -> &ClaimBody {
        &self.body
    }
}
