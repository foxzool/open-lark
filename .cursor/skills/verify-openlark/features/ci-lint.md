# CI lint job

The CI lint job in `.github/workflows/ci.yml` checks formatting and runs clippy twice. The job sets `RUSTFLAGS: "-D warnings"`. Clippy's own flag is `-- -D warnings` (space after `-D`).

## Sub-features

- `fmt-check` runs `cargo fmt --all -- --check` under that `RUSTFLAGS` value.
- `clippy-all-features` runs workspace clippy with `--all-targets --all-features`.
- `clippy-no-default-features` runs workspace clippy with `--all-targets --no-default-features`.

## How to get to it (user POV)

- From the repo root, run `env RUSTFLAGS="-D warnings" cargo fmt --all -- --check`.
- From the repo root, run `env RUSTFLAGS="-D warnings" cargo clippy --workspace --all-targets --all-features -- -D warnings`.
- From the repo root, run `env RUSTFLAGS="-D warnings" cargo clippy --workspace --all-targets --no-default-features -- -D warnings`.

## Driving it with control-openlark

Preconditions:

- `control-openlark doctor` printed `doctor=PASS`.
- `rustfmt` and `clippy` are installed on the active toolchain (`rustup component add rustfmt clippy` when they are missing).

- **Format.** Run the fmt step alone. Run `control-openlark fmt`. Exit code `0` and `fmt.txt` records `cargo fmt --all -- --check`.
- **Full lint job.** Run fmt and both clippy modes. Run `control-openlark lint`. Exit code `0` and `lint.txt` contains all three commands. One mode passing while another fails is not a pass.
- **Proof.** Retain `fmt.txt` or `lint.txt`. A transcript that shows `-Dwarnings` without the space is the justfile spelling, not this CI job.

## Gotchas

- `just lint` is `cargo clippy ... -- -Dwarnings` and does not export `RUSTFLAGS`. Do not record that spelling as the CI gate.
- Feature-gated code can be clippy-clean in one mode and dirty in the other. Both clippy lines must succeed.
- This is a static analysis proof. It does not replace `cargo test` for behavioral changes.
- Full-workspace clippy is slow. Do not kill it by process name. Stop the PID this run started, or wait.
