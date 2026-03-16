## 2026-03-14 F1 audit learnings
- `openlark-webhook` compiles and its current test suite passes with `cargo check -p openlark-webhook --all-features` and `cargo test -p openlark-webhook --all-features`.
- Must-have coverage is strongest in `crates/openlark-webhook/tests/integration_webhook.rs` (text send, all message types, HTTP error handling, optional signature headers).
- Plan audits should compare exact expected evidence filenames, not just loosely related artifacts; combined or renamed files make task traceability fail.
