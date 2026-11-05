# organon-protocol: design

Status: **v0.1 (2026-11-05)**. Living.

## Decisions
- **Typed ids, plain strings on the wire.** `#[serde(try_from = "String", into = "String")]`: the format stays a string; inside Rust an id is always well-formed and can't be confused with another kind.
- **Anchor kind comes from the id** (`repo:x` → `repo`). The old separate `kind` field is ignored on read, so the two can never disagree.
- **Claim = id + flattened body.** `ClaimBody::compute_id` hashes `[about, relation, value, scope, valid_from, stance]`. That's identical to mneme v0, and verified against all 3,583 stored anchors and claims (0 mismatches).
- **Open sets are newtypes, closed sets are enums.** `Relation`, `Scope` and `ObservationKind` are open; `Basis`, `Stance`, `Status` and `Family` are closed. Relation semantics (`family`, `functional`, default stance) live here, because every organ that writes claims needs the same vocabulary.
- **Evidence can point at observations,** so a claim made from a video cites the exact moment and frame.
- **Episodes use `kind`, not `type`,** because `type` tags stream messages.
- **The JSON Schema is generated** (feature `schema`), not hand-written, so it can't drift.

## Open questions
- Should time (`valid_from`, `captured_at`) become a typed timestamp with explicit precision?
- Is a protocol-level "commit" needed, or is history purely memory's concern?
- Registry of observation kinds per modality: here, or in each perception organ?
