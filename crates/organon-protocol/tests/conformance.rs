//! Conformance: the fixture streams every implementation must accept or reject the same way.
//! Other organs (in any language) can run their parser against `tests/fixtures/`.

use std::io::Cursor;

use organon_protocol::{check_stream, Message};
use proptest::prelude::*;

const VALID: &str = include_str!("fixtures/valid.jsonl");
const INVALID: &str = include_str!("fixtures/invalid.jsonl");

#[test]
fn valid_stream_passes() {
    let r = check_stream(Cursor::new(VALID)).unwrap();
    assert!(r.is_ok(), "{:?}", r.errors);
    let kinds: Vec<_> = r.messages.iter().map(Message::type_name).collect();
    assert_eq!(
        kinds,
        [
            "source",
            "observation",
            "anchor",
            "claim",
            "proposal",
            "episode",
            "observation",
            "episode"
        ]
    );
}

#[test]
fn every_invalid_line_is_reported() {
    let r = check_stream(Cursor::new(INVALID)).unwrap();
    let lines: Vec<_> = r.errors.iter().map(|e| e.line).collect();
    let expected: Vec<_> = INVALID
        .lines()
        .enumerate()
        .filter(|(_, l)| !l.trim().is_empty())
        .map(|(i, _)| i + 1)
        .collect();
    assert_eq!(lines, expected, "{:#?}", r.errors);
}

#[test]
fn messages_round_trip() {
    let r = check_stream(Cursor::new(VALID)).unwrap();
    for m in r.messages {
        let text = serde_json::to_string(&m).unwrap();
        let back: Message = serde_json::from_str(&text).unwrap();
        assert_eq!(back, m, "{text}");
    }
}

proptest! {
    #[test]
    fn content_ids_are_deterministic_and_well_formed(data in proptest::collection::vec(any::<u8>(), 0..256)) {
        let a = organon_protocol::SourceId::of(&data);
        let b = organon_protocol::SourceId::of(&data);
        prop_assert_eq!(&a, &b);
        let reparsed: organon_protocol::SourceId = a.as_str().parse().unwrap();
        prop_assert_eq!(reparsed, a);
    }

    #[test]
    fn anchor_ids_never_contain_whitespace(name in "[a-z]{1,8}:[^\\s]{1,20}") {
        prop_assert!(name.parse::<organon_protocol::AnchorId>().is_ok());
    }
}
