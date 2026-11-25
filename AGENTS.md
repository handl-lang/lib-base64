# lib-base64

`base64` — Handl stdlib codec implementing the `Encoding` trait from
`lib-encoding`. Hybrid package: `.hnl` surface plus Rust atoms.

## Layout

| path | role |
| --- | --- |
| `src/index.hnl`, `src/index_flat.hnl` | the `.hnl` interface |
| `src/encode.hnl`, `src/decode.hnl` | codec arrows |
| `src/base64.test.hnl` | `.hnl` tests (gated by `hnl test`) |
| `src/lib.rs`, `src/docs/base64.yml` | Rust atoms + docs spec |
| `Cargo.toml`, `xtask/` | Rust crate + vendored repo gate |
| `pkg.hnl`, `scripts/hnl-gate.sh` | manifest + `.hnl` gate |
| `workflows/gate.yaml` | CI — shared `handl-gates` campaign |

## Contract

- Implements `lib-encoding`'s `Encoding` trait; dep declared in
  `pkg.hnl` as a git dep on `branch:main`.
- `.hnl` files are gated by `hnl-gate.sh`; the Rust side by
  `cargo xtask gate`.

## Verification

```bash
HNL=../host-cli/target/debug/hnl scripts/hnl-gate.sh
cargo xtask gate        # fmt --check → clippy -D warnings → test → doc
```

<!-- myco:agent-awareness -->

## Agent awareness

- Session start / any work discussion: run `myco status --prompt`
  before proposing next steps — know the board first.
- User describes new work → offer to capture it (`myco todo add`,
  `issue new`, `proposal new`). Don't wait to be asked.
- Picking up a task: `myco resume <ID>` is the briefing.
- Wrapping up: `myco handoff <ID>` anything resumable; keep statuses
  honest (claimed → in-progress → blocked/done).

<!-- /myco:agent-awareness -->
