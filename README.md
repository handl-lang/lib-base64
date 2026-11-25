# lib-base64

`base64` — Handl stdlib codec. Implements the `Encoding` trait from
`lib-encoding`; the `.hnl` surface lives in `src/index.hnl`.

## Layout

```text
src/                     the interface plus the Rust atoms
pkg.hnl                  package manifest
scripts/hnl-gate.sh      the gate
workflows/gate.yaml                    bupha entry (handl-gates)
```

## Gate

```bash
HNL=../host-cli/target/debug/hnl scripts/hnl-gate.sh
```

## License

MIT.
