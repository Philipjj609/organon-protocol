//! Conformance checking of a protocol stream (JSON Lines of [`Message`]s).

use std::io::BufRead;

use crate::message::Message;

/// A line that failed to parse or violated a rule.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("line {line}: {reason}")]
pub struct LineError {
    pub line: usize,
    pub reason: String,
}

/// Result of checking a stream.
#[derive(Debug, Default)]
pub struct Report {
    pub messages: Vec<Message>,
    pub errors: Vec<LineError>,
}

impl Report {
    #[must_use]
    pub fn is_ok(&self) -> bool {
        self.errors.is_empty()
    }
}

/// Parse every non-empty line and check the rules a type system can't: content ids must match
/// content, and only inferred claims carry a confidence.
///
/// # Errors
/// Only I/O errors from the reader; protocol violations are collected in the [`Report`].
pub fn check_stream(reader: impl BufRead) -> std::io::Result<Report> {
    let mut report = Report::default();
    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        let text = line.trim_start_matches('\u{feff}').trim();
        if text.is_empty() {
            continue;
        }
        let n = i + 1;
        match serde_json::from_str::<Message>(text) {
            Ok(msg) => {
                if let Some(reason) = rule_violation(&msg) {
                    report.errors.push(LineError { line: n, reason });
                }
                report.messages.push(msg);
            }
            Err(e) => report.errors.push(LineError {
                line: n,
                reason: e.to_string(),
            }),
        }
    }
    Ok(report)
}

fn rule_violation(msg: &Message) -> Option<String> {
    match msg {
        Message::Claim(c) if !c.id_is_consistent() => Some(format!(
            "claim {} does not match its content (expected {})",
            c.id,
            c.body.compute_id()
        )),
        Message::Claim(c) if c.confidence.is_some() && c.basis != crate::Basis::Inferred => Some(
            format!("claim {}: only inferred claims carry a confidence", c.id),
        ),
        Message::Observation(o) if !o.id_is_consistent() => {
            Some(format!("observation {} does not match its content", o.id))
        }
        _ => None,
    }
}
