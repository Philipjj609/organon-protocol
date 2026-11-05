//! Golden tests: canonical forms are contracts. These ids come from real data in a memory
//! written before this crate existed; if one changes, every stored id would break.

use organon_protocol::*;

fn parse_claim(json: &str) -> Claim {
    serde_json::from_str(json).expect("golden claim parses")
}

#[test]
fn observed_number_claim_keeps_its_id() {
    let c = parse_claim(
        r#"{"id": "c_9b63203677b54ab2", "about": "repo:zhaosenlin12-creator/MotionSites", "relation": "stars", "value": {"number": 59.0}, "scope": "world", "valid_from": "2026-11-05", "basis": "observed", "stance": "is", "evidence": [[{"source": "s_51518e9fc5c3bda7"}]], "author": "mneme/capture-github@0.1"}"#,
    );
    assert!(c.id_is_consistent(), "computed {}", c.body.compute_id());
}

#[test]
fn inferred_claim_keeps_its_id() {
    let c = parse_claim(
        r#"{"id": "c_de0580a92a66a730", "about": "repo:zhaosenlin12-creator/MotionSites", "relation": "provides", "value": {"anchor": "concept:web-effects"}, "scope": "world", "basis": "inferred", "stance": "is", "confidence": 0.6, "rationale": "Prompt collection for motion-rich sites.", "evidence": [[{"source": "s_51518e9fc5c3bda7"}]], "author": "claude-opus-5-5"}"#,
    );
    assert!(c.id_is_consistent(), "computed {}", c.body.compute_id());
}

#[test]
fn claim_serializes_back_to_the_same_json() {
    let raw = r#"{"id":"c_de0580a92a66a730","about":"repo:zhaosenlin12-creator/MotionSites","relation":"provides","value":{"anchor":"concept:web-effects"},"scope":"world","basis":"inferred","stance":"is","confidence":0.6,"rationale":"Prompt collection for motion-rich sites.","evidence":[[{"source":"s_51518e9fc5c3bda7"}]],"author":"claude-opus-5-5"}"#;
    let c = parse_claim(raw);
    assert_eq!(serde_json::to_string(&c).unwrap(), raw);
}

#[test]
fn identity_ignores_author_evidence_and_confidence() {
    let a = Claim::draft(
        "concept:agent-memory".parse::<AnchorId>().unwrap(),
        "part_of",
        Value::Anchor("concept:knowledge".parse().unwrap()),
        Basis::Inferred,
        "someone",
    );
    let mut b = a.clone();
    b.author = "someone-else".into();
    b.confidence = Some(0.9);
    b.evidence
        .push(vec![EvidenceRef::source(SourceId::of(b"x"))]);
    assert_eq!(a.finalize().id, b.finalize().id);
}

#[test]
fn relation_sets_default_stance() {
    let c = Claim::draft(
        "repo:a/b".parse::<AnchorId>().unwrap(),
        "relevant_to",
        Value::Anchor("project:koda".parse().unwrap()),
        Basis::Inferred,
        "t",
    );
    assert_eq!(c.stance, Stance::Could);
    assert_eq!(Relation::from("relevant_to").info().family, Family::Use);
    assert!(Relation::from("stars").info().functional);
    assert_eq!(Relation::from("brand-new").info().family, Family::Other);
}

#[test]
fn claims_can_be_about_claims() {
    let s: Subject = "c_de0580a92a66a730".parse().unwrap();
    assert!(matches!(s, Subject::Claim(_)));
    let s: Subject = "repo:a/b".parse().unwrap();
    assert!(matches!(s, Subject::Anchor(_)));
}

#[test]
fn evidence_can_point_to_an_observation() {
    let e: EvidenceRef = serde_json::from_str(r#"{"observation":"o_0123456789abcdef"}"#).unwrap();
    assert!(matches!(e, EvidenceRef::Observation { .. }));
    let e: EvidenceRef =
        serde_json::from_str(r#"{"source":"s_0123456789abcdef","span":"t=12:34"}"#).unwrap();
    assert!(matches!(e, EvidenceRef::Source { span: Some(_), .. }));
}

#[test]
fn observation_id_ignores_the_producer() {
    let src = SourceId::of(b"video");
    let kind = ObservationKind::new(ObservationKind::TRANSCRIPT_SEGMENT);
    let at = Location::Time {
        t0: 754.2,
        t1: 761.8,
    };
    let v = serde_json::json!({"text": "as you can see"});
    let a = Observation::new(
        src.clone(),
        kind.clone(),
        at.clone(),
        v.clone(),
        ProducedBy {
            organ: "a".into(),
            ..Default::default()
        },
    );
    let b = Observation::new(
        src,
        kind,
        at,
        v,
        ProducedBy {
            organ: "b".into(),
            ..Default::default()
        },
    );
    assert_eq!(a.id, b.id);
    assert!(a.id_is_consistent());
}

#[test]
fn locations_parse_by_shape() {
    let t: Location = serde_json::from_str(r#"{"t0":1.0,"t1":2.5}"#).unwrap();
    assert!(matches!(t, Location::Time { .. }));
    let p: Location = serde_json::from_str(r#"{"page":3,"bbox":[0,0,10,10]}"#).unwrap();
    assert!(matches!(p, Location::Page { page: 3, .. }));
    let f: Location = serde_json::from_str(r#"{"frame":84}"#).unwrap();
    assert!(matches!(f, Location::Frame { frame: 84 }));
}

#[test]
fn costs_add_up() {
    let a = Cost {
        tokens_in: Some(10),
        usd: Some(0.5),
        model: Some("m".into()),
        ..Default::default()
    };
    let b = Cost {
        tokens_in: Some(5),
        seconds: Some(2.0),
        model: Some("m".into()),
        ..Default::default()
    };
    let s = a.plus(&b);
    assert_eq!(
        (s.tokens_in, s.usd, s.seconds, s.model.as_deref()),
        (Some(15), Some(0.5), Some(2.0), Some("m"))
    );
}
