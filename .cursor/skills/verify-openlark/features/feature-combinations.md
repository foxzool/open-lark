# CI feature combinations

`.github/workflows/ci.yml` job `feature-combinations` builds, tests, and clippy-checks the root package once per feature string. The cargo argv below is that job. `timeout` values are the ones in the workflow (`20m` build, `15m` test). Clippy on this job has no `RUSTFLAGS` and no `timeout` wrapper.

## Sub-features

- `combo-empty` is the minimal build: `--no-default-features` and no `--features`.
- `combo-string` is any other matrix string, passed as `--features "<string>"`.
- `combo-auth-communication` is the `auth,communication` cell.

## How to get to it (user POV)

- Empty cell, from the repo root:

```bash
timeout 20m cargo build --no-default-features
timeout 15m cargo test -p openlark -p openlark-client --no-default-features --lib
cargo clippy --no-default-features --all-targets -- -D warnings
```

- Non-empty cell, same three lines with `--features "<string>"` on each cargo command. For `auth,communication`:

```bash
timeout 20m cargo build --no-default-features --features "auth,communication"
timeout 15m cargo test -p openlark -p openlark-client --no-default-features --features "auth,communication" --lib
cargo clippy --no-default-features --features "auth,communication" --all-targets -- -D warnings
```

Matrix strings in that job, in workflow order:

- `` (empty)
- `auth`
- `auth,communication`
- `auth,docs`
- `auth,hr`
- `auth,workflow`
- `auth,meeting`
- `auth,ai`
- `auth,security`
- `auth,platform`
- `auth,application`
- `auth,mail`
- `auth,helpdesk`
- `auth,analytics`
- `auth,user`
- `auth,cardkit`
- `bot`
- `pay`
- `auth,docs-base`
- `auth,docs-bitable`
- `auth,docs-drive`
- `auth,docs-full`
- `webhook`
- `webhook-card`
- `webhook-signature`
- `webhook-full`
- `essential`
- `enterprise`
- `communication,docs,auth`
- `communication,hr,auth`
- `docs,hr,auth`
- `hr,workflow,auth`
- `communication,docs,hr,auth`
- `auth,communication,webhook`
- `auth,docs,webhook-full`
- `auth,communication,cardkit`
- `essential,webhook-full`
- `enterprise,ai`
- `enterprise,meeting`
- `full`

## Driving it with control-openlark

Preconditions:

- `control-openlark doctor` printed `doctor=PASS`.
- `timeout` is on `PATH` (`/usr/bin/timeout`).
- One drive proves one cell. CI runs every string above with `fail-fast: false`.

- **Empty cell.** Run `control-openlark features`. Exit code `0`. `features-empty.txt` contains the three empty-cell commands and ends with clippy success.
- **auth,communication.** Run `control-openlark features auth,communication`. Exit code `0`. `features-auth-communication.txt` contains `--features auth,communication` on build, test, and clippy.
- **Another matrix string.** Run `control-openlark features <string>` with one string from the list, unchanged. Exit code `0` and the transcript names that string. Do not invent a string that is not in the list.
- **Proof.** Keep the `features-*.txt` for the cell you ran. Do not report the rest of the matrix as locally proven.

## Gotchas

- `just test-features` and `just test-features-quick` are different commands (`cargo test --no-default-features --lib` without `-p openlark -p openlark-client`, plus `cargo hack` on the slow target). They are not this job.
- `.github/workflows/feature-matrix.yml` is a second workflow. Its build/test shape is similar, its timeouts are different, its feature list is different, and it does not run this clippy line. Its `cargo hack` steps print a summary even when the command fails. Do not copy that file's list into this job.
- These commands target the root package plus `openlark-client` tests. They are not `cargo test --workspace`.
- Root `default` features are `["auth"]`. The empty cell passes `--no-default-features`, so it is not the default `auth` build. The `auth` string is a separate cell.
- A green empty cell does not prove `full`.
