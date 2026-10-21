---
name: verify-openlark
description: "Prove OpenLark (Feishu/Lark Rust SDK) changes the way a maintainer does — cargo build/test, fmt, clippy, CI feature combinations, public examples, and offline Feishu e2e gates. Use when verifying a library change, before claiming an SDK fix works, or when asked to drive OpenLark verification."
---

# Verify OpenLark

OpenLark is a **Rust workspace library SDK**, not a GUI or long-lived server. Verification means compiling and exercising the SDK the way CI and maintainers do: `cargo` build/test/fmt/clippy, the feature-combination commands in `.github/workflows/ci.yml`, public examples (offline unless a live section says otherwise), and the Python harnesses under `tools/`. Read this skill cold mid-task; follow it literally.

Put `control-openlark` on `PATH` for the session:

```bash
export PATH="$(pwd)/.cursor/skills/verify-openlark/scripts:$PATH"
# or from anywhere:
export PATH="/absolute/path/to/repo/.cursor/skills/verify-openlark/scripts:$PATH"
```

Evidence lives under `.cursor/skills/verify-openlark/artifacts/<run-id>/` (override with `VERIFY_OPENLARK_ARTIFACTS`). Cleanup never deletes that tree.

Feature recipes: [features/README.md](features/README.md). A proof that drives one convenient entry point is incomplete when the map lists others for the change under test.

## Launch

There is no daemon to keep alive. Launch = toolchain ready + one workspace build so later drives share a warm `target/`.

```bash
# Optional isolation (recommended when sharing a machine):
export CARGO_TARGET_DIR="/tmp/openlark-verify-$USER-$$"
control-openlark launch --target-dir "$CARGO_TARGET_DIR"
# Ready when the transcript ends with: READY workspace build finished
# Equivalent raw command (CI does not set a separate build step; this is the workspace compile):
#   cargo build --workspace --all-features
```

Teardown is `control-openlark cleanup` (see Cleanup). Do not leave isolated `CARGO_TARGET_DIR` trees forever; purge only paths under `/tmp/openlark-verify-*` via `cleanup --purge-target`.

If `rustc` is older than **1.88**, upgrade before generating proofs (`rustup update stable && rustup default stable`). Edition 2024 requires a modern Cargo. CI's `CARGO_TOOLCHAIN` is `stable`.

## Doctor

Run first whenever anything looks off, and before the first drive of a run:

```bash
control-openlark doctor
```

Doctor passes only when all of these hold:

- Repo root contains `Cargo.toml` and `crates/openlark-core`
- `rustc` / `cargo` / `python3` on `PATH`
- `rustc` major.minor ≥ **1.88**
- `cargo metadata --no-deps` succeeds (manifest parses)
- Prints `doctor=PASS` and writes `artifacts/<run-id>/doctor.txt`

If `OPENLARK_USER_SEARCH_NAME` or `OPENLARK_CHAT_SEARCH_NAME` is set, doctor warns that examples may hit the live Feishu API — unset them for offline proofs.

If `OPENLARK_CHANNEL_E2E`, `OPENLARK_WS_CONNECT`, or `OPENLARK_LIVE_USER_CALL` is set, doctor warns that a raw command may open a real Feishu call. Harness offline drives unset those names. Doctor prints only whether they are set, never their values.

Never drive a shared interactive cargo session you did not start for this verification run.

## Drive

Prefer `control-openlark` over raw commands so transcripts land in the run evidence directory. The raw column is the CI or example command, copied from the repo and executed while this skill was written.

| Intent | Harness command | Maintainer / CI command |
|--------|-----------------|------------------------|
| Workspace build | `control-openlark launch` | `cargo build --workspace --all-features` |
| Unit/integration tests | `control-openlark test` | `cargo test --workspace --all-features --no-fail-fast` |
| Protobuf package gate | `control-openlark protobuf` | `python3 -m unittest tools.tests.test_lark_websocket_protobuf_package` |
| Public example compile gate | `control-openlark example check` | `bash scripts/check-public-examples.sh` |
| Offline example run | `control-openlark example run client_setup --features "auth,communication"` | `cargo run --example client_setup --features "auth,communication"` |
| Format check | `control-openlark fmt` | `env RUSTFLAGS="-D warnings" cargo fmt --all -- --check` |
| CI lint job | `control-openlark lint` | fmt check, then both clippy lines below |
| Clippy all features | (inside `lint`) | `env RUSTFLAGS="-D warnings" cargo clippy --workspace --all-targets --all-features -- -D warnings` |
| Clippy no default features | (inside `lint`) | `env RUSTFLAGS="-D warnings" cargo clippy --workspace --all-targets --no-default-features -- -D warnings` |
| One CI feature cell | `control-openlark features` or `control-openlark features auth,communication` | see [features/feature-combinations.md](features/feature-combinations.md) |
| Typed API coverage | `control-openlark api-coverage --crate openlark-docs` | `python3 tools/validate_apis.py --crate openlark-docs` |
| Field sanity (offline) | `control-openlark field-verify --crate openlark-workflow` | `python3 tools/verify_api_fields.py --crate openlark-workflow` |
| Channel e2e offline | `control-openlark e2e channel` | ignored test with live env names unset; see the e2e map |
| Typed WS offline | `control-openlark e2e ws-typed` | `cargo run --example websocket_typed_handler --no-default-features --features "communication,websocket"` with `OPENLARK_WS_CONNECT` unset |
| User-token example offline | `control-openlark e2e oauth` | `cargo run --example oauth_user_token --no-default-features --features "auth,communication"` with `OPENLARK_LIVE_USER_CALL` unset |

Stable handles for this repo: crate names (`openlark-core`, `openlark-docs`, …), example names from root `Cargo.toml` `[[example]]`, script paths `scripts/check-public-examples.sh`, `tools/validate_apis.py`, `tools/verify_api_fields.py`, and the test filter `channel_card_callback_e2e`. Do not invent alternate entry points.

`just lint` uses `-Dwarnings` (no space) and does not export `RUSTFLAGS`. CI's lint job uses `RUSTFLAGS: "-D warnings"` and clippy `-- -D warnings` (with a space). Drive the CI spelling.

`just test` omits `--no-fail-fast`. The CI test job includes it. Drive the CI command.

`just test-features` is not the CI feature matrix. It runs `cargo test --no-default-features --lib` without `-p openlark -p openlark-client`, and it calls `cargo hack`. Use the commands in the feature-combinations map.

Deep field verification against Feishu SPA docs is owned by `.agents/skills/openlark-api-field-verify/` — invoke that skill when `--fetch-docs` is insufficient; this verify skill only wraps the CLI gate.

Live Feishu HTTP and long-connection steps are in [features/feishu-live-e2e.md](features/feishu-live-e2e.md). The harness has no `--live` flag. `control-openlark example run` refuses `first_api_call` and `websocket_echo_bot`.

## Evidence

Proof standards for a library SDK:

- Exercise the **maintainer path** (cargo/examples/tools), not private test hooks that skip the public API surface you claim to fix.
- Capture **command + exit code + full stdout/stderr** in `artifacts/<run-id>/`.
- For mutations to API models, prove with `cargo test` on the affected crate **and** (when the change is coverage/fields) the matching Python harness.
- Side effects: compiled binaries under `CARGO_TARGET_DIR`, reports under `reports/api_validation/` (copied into evidence by `api-coverage`).
- Mocks/wiremock inside unit tests are fine — they are the production-isolated boundary. Do **not** claim a live Feishu round-trip unless the transcript shows the live path (no skip line) and the run intentionally had credentials. A test that prints `跳过 channel 卡片回调端到端` did not send a card and did not open a long connection.
- `client_setup` without `OPENLARK_USER_SEARCH_NAME` / `OPENLARK_CHAT_SEARCH_NAME` is an **offline** proof; observe that the transcript says those lookups were skipped — do not trust the example name alone if those env vars might be set.
- `websocket_typed_handler` without `OPENLARK_WS_CONNECT=1` prints `未设置 OPENLARK_WS_CONNECT=1，跳过真实长连接` and returns before `LarkWsClient::open`.
- `oauth_user_token` without `OPENLARK_LIVE_USER_CALL=1` prints `未设置 OPENLARK_LIVE_USER_CALL=1，跳过真实 API 调用`.

Locate the current evidence directory:

```bash
control-openlark evidence-path
```

Minimum artifact set for a claimed fix: `doctor.txt`, the drive transcript for the mapped feature you ran (`test.txt`, `example-client_setup.txt`, `e2e-channel.txt`, …), and exit status 0 on that drive.

## Cleanup

```bash
control-openlark cleanup                 # drop .run-state readiness markers only
control-openlark cleanup --purge-target  # also rm isolated /tmp/openlark-verify-* target dir
```

Rules:

- Never `pkill` / kill by process name. This skill starts no long-lived processes; cargo/python children exit with the drive.
- Never delete `artifacts/` proof trees.
- Do not `cargo clean` the developer's default `target/` unless this run used an isolated `CARGO_TARGET_DIR` and `--purge-target` is set.

After cleanup, confirm evidence still exists at the path printed by `evidence-path` during the run (re-create path from `artifacts/<run-id>/` listing if state was cleared).

## Helpers

Executable: `.cursor/skills/verify-openlark/scripts/control-openlark`

```bash
control-openlark doctor
control-openlark launch [--target-dir DIR]
control-openlark test [--package PKG] [--lib] [-- -- FILTER]
control-openlark example run NAME [--features FEATS]
control-openlark example check
control-openlark api-coverage [--crate NAME|--all-crates]
control-openlark field-verify --crate NAME [--fetch-docs] [--api-id ID]
control-openlark fmt
control-openlark lint
control-openlark features [FEATURE_STRING]
control-openlark protobuf
control-openlark e2e channel|ws-typed|oauth
control-openlark evidence-init [RUN_ID]
control-openlark evidence-path
control-openlark cleanup [--purge-target]
```

`features` with no argument is the CI empty cell (`--no-default-features` and no `--features`). Any other argument is one matrix string such as `auth,communication`.

`e2e` always strips the live switches for that drive. It cannot be pointed at a real app.

## Maintenance

Keep the feature map honest with `/maintain-verification-skill` as the SDK surface changes (new examples, harness flags, MSRV bumps).
