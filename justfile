lint:
  @echo "Lint"
  cargo clippy --workspace --all-targets --all-features -- -Dwarnings