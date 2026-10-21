#!/usr/bin/env bash
# Run the AuthTokenProvider get_or_fetch TLA+ checks with a pinned TLC.
# Exit 0 only when:
#   1) FailedFetchDoesNotWrite (+ TypeOK) hold on the model of current code, and
#   2) SingleFlight is still violated (known finding in current Rust).
# If (1) fails → real regression or model/code drift (treat as failure).
# If (2) starts passing → single-flight may have been implemented; update the
#   verify skill / finding before treating that as success.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")" && pwd)"
cd "$ROOT"

TLA_VERSION="${TLA_VERSION:-1.8.0}"
TLA_JAR_URL="${TLA_JAR_URL:-https://github.com/tlaplus/tlaplus/releases/download/v${TLA_VERSION}/tla2tools.jar}"
CACHE_DIR="${TLA_CACHE_DIR:-${XDG_CACHE_HOME:-$HOME/.cache}/openlark-tla}"
JAR="$CACHE_DIR/tla2tools-${TLA_VERSION}.jar"

if [[ ! -f "$JAR" ]]; then
  mkdir -p "$CACHE_DIR"
  echo "Downloading tla2tools ${TLA_VERSION} → $JAR"
  curl -fsSL -o "$JAR.partial" "$TLA_JAR_URL"
  mv "$JAR.partial" "$JAR"
fi

if ! command -v java >/dev/null 2>&1; then
  echo "error: java is required to run TLC (OpenJDK 17+ recommended)" >&2
  exit 2
fi

cleanup_traces() {
  rm -f "$ROOT"/TokenProvider_TTrace_*.tla "$ROOT"/TokenProvider_TTrace_*.bin
  rm -rf "$ROOT"/states
}
trap cleanup_traces EXIT

JAVA_OPTS=(-XX:+UseParallelGC)
TLC=(java "${JAVA_OPTS[@]}" -cp "$JAR" tlc2.TLC -workers auto -nowarning)

echo "==> Checking FailedFetchDoesNotWrite (must PASS)"
set +e
"${TLC[@]}" -config TokenProvider.cfg TokenProvider | tee "$ROOT/last-pass.log"
pass_rc=${PIPESTATUS[0]}
set -e
if [[ "$pass_rc" -ne 0 ]]; then
  echo "FAIL: TokenProvider.cfg expected to pass (exit $pass_rc). See last-pass.log" >&2
  exit 1
fi
if ! grep -q "Model checking completed. No error has been found." "$ROOT/last-pass.log"; then
  echo "FAIL: TokenProvider.cfg did not report a clean completion" >&2
  exit 1
fi

echo "==> Checking SingleFlight (known FINDING — must be VIOLATED by current code)"
set +e
"${TLC[@]}" -config TokenProviderSingleFlight.cfg TokenProvider | tee "$ROOT/last-singleflight.log"
sf_rc=${PIPESTATUS[0]}
set -e
if grep -q "Model checking completed. No error has been found." "$ROOT/last-singleflight.log"; then
  echo "UNEXPECTED PASS: SingleFlight now holds." >&2
  echo "If AuthTokenProvider gained single-flight, update" >&2
  echo ".agents/skills/openlark-formal-token-provider/SKILL.md and clear the finding." >&2
  exit 3
fi
if ! grep -q "Invariant SingleFlight is violated" "$ROOT/last-singleflight.log"; then
  echo "FAIL: expected a SingleFlight violation; TLC exit=$sf_rc. See last-singleflight.log" >&2
  exit 1
fi

echo "OK: FailedFetchDoesNotWrite holds; SingleFlight still violated (known finding)."
echo "Counterexample (code terms): two callers both miss get_cached, both enter fetch()."
exit 0
