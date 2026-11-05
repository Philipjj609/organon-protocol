# Working in organon-protocol (for AI agents)

Follow [organon/docs/ENGINEERING.md](https://github.com/Philipjj609/organon/blob/main/docs/ENGINEERING.md).

Invariants:
1. **Canonical forms are contracts.** `ClaimBody::compute_id` and `Observation::compute_id` must not change without a protocol major bump; `tests/golden.rs` pins real ids.
2. Wire format stays plain JSON: new fields are optional (`#[serde(default)]`), unknown fields are tolerated.
3. Every rule added to `check.rs` gets a line in `tests/fixtures/invalid.jsonl`.
4. Gate: `cargo fmt --check && cargo clippy --all-targets --all-features -- -D warnings && cargo test --all-features`.
