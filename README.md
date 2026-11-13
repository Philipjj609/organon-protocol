# organon-protocol

**The shared language of [organon](https://github.com/Philipjj609/organon) organs.** Every organ (memory, research, learning, perception of video, sound, images and text) reads and writes these objects. Depend on this, never on another organ, and organs stay interchangeable.

| Object | What it is |
|---|---|
| `Source` | what a sense captured: raw signal + metadata, content-hashed |
| `Observation` | a *located* feature (time range, page region, frame) extracted by perception, with no interpretation |
| `Anchor` | a thing with identity, named `kind:name` |
| `Claim` | a qualified statement: basis (`observed`/`stated`/`inferred`/`imported`/`intuited`), stance, scope, time, evidence |
| `Proposal` | interpretation offered to memory, which validates it |
| `Episode` | what happened when the system did something, with its `Cost` (tokens, model, seconds, money) |

On the wire, it's **JSON Lines**, one `Message` per line, tagged by `"type"`:

```json
{"type":"observation","id":"o_52b4f23fc2d3e573","source":"s_0123456789abcdef","kind":"transcript_segment","at":{"t0":754.2,"t1":761.8},"value":{"text":"as you can see on this slide"},"produced_by":{"organ":"organon-video","version":"0.1.0","tool":"captions:manual"},"confidence":0.98}
```

**Identity is content.** Ids are a prefix plus 16 hex digits of the SHA-256 of a canonical form (`s_` source, `o_` observation, `c_` claim, `m_` commit, `e_` episode). The same thing produced twice gets the same id. A claim's id ignores who said it and with what evidence, so two sources asserting the same statement reinforce *one* claim. The canonical forms are contracts, pinned by golden tests against real stored data.

## Use

Rust:
```toml
organon-protocol = { git = "https://github.com/Philipjj609/organon-protocol", tag = "v0.1.4" }
```
```rust
use organon_protocol::{Claim, Value, Basis, AnchorId};
let claim = Claim::draft("repo:a/b".parse::<AnchorId>()?, "provides",
                         Value::Anchor("concept:agent-memory".parse()?), Basis::Inferred, "me").finalize();
```

Any language: validate your organ's output, or read the JSON Schema.
```bash
cargo install --git https://github.com/Philipjj609/organon-protocol organon-protocol-cli
organon-protocol validate output.jsonl     # exit 0 = conforms
organon-protocol schema > message.schema.json
```
The fixtures in `crates/organon-protocol/tests/fixtures/` (`valid.jsonl`, `invalid.jsonl`) are the conformance suite: a conforming parser accepts every line of one and rejects every line of the other.

## Status

v0.1, protocol major version **0** (expect changes). Design notes and open questions are in [docs/DESIGN.md](docs/DESIGN.md). Engineering standard: [organon/docs/ENGINEERING.md](https://github.com/Philipjj609/organon/blob/main/docs/ENGINEERING.md).

## License

MIT OR Apache-2.0.
