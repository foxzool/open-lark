#!/usr/bin/env bash

set -euo pipefail

OUTPUT_DIR="${1:-reports/pre_release_compatibility}"

mkdir -p "${OUTPUT_DIR}"

REQUIRED_DOCS=(
  "docs/PUBLIC_API_STABILITY_POLICY.md"
  "docs/TYPED_API_SEMVER_RULES.md"
  "docs/HELPER_SEMVER_RULES.md"
  "docs/api-compatibility-note-template.md"
  "docs/migration-guide.md"
)

echo "== Verify compatibility policy documents =="
for file in "${REQUIRED_DOCS[@]}"; do
  test -f "${file}"
  echo "  ✓ ${file}"
done

echo
echo "== Check formatting =="
cargo fmt --all -- --check

echo
echo "== Run clippy =="
cargo clippy --workspace --all-targets --all-features -- -Dwarnings -A missing_docs

echo
echo "== Run workspace tests =="
cargo test --workspace --all-features

echo
echo "== Validate public examples =="
bash scripts/check-public-examples.sh

echo
echo "== Run common compatibility feature combinations =="
cargo test --no-default-features --features "essential" --lib
cargo test --no-default-features --features "enterprise" --lib
cargo test --no-default-features --features "communication,websocket" --lib

echo
echo "== Generate typed API coverage report =="
python3 tools/validate_apis.py --all-crates

echo
echo "== Generate crate quality status summary =="
python3 tools/release_quality_status.py --output "${OUTPUT_DIR}/release_quality_status.md"

cat > "${OUTPUT_DIR}/summary.md" <<EOF
# Pre-release Compatibility Verification Summary

## Scope

- Public API stability / deprecation policy documents present
- Workspace fmt / clippy / tests green
- Public README / example compile-check green
- Common feature combinations still expose expected root surface
- Typed API coverage summary regenerated
- Crate quality status summary regenerated

## Triggered artifacts

- \`${OUTPUT_DIR}/release_quality_status.md\`
- \`reports/api_validation/summary.json\`
- \`reports/api_validation/dashboards/core_business.json\`
EOF
