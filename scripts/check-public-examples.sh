#!/usr/bin/env bash

set -euo pipefail

echo "Checking root README aligned example..."
cargo check --example readme_quick_start --no-default-features --features "auth,docs-bitable"

echo "Checking simple API example..."
cargo check --example simple_api_call --no-default-features --features "auth,communication"

echo "Checking docs helper example..."
cargo check --example docs_helpers --no-default-features --features "auth,docs-bitable"

echo "Checking workflow example..."
cargo check --example workflow_api_example --no-default-features --features "workflow"

echo "Checking websocket example..."
cargo check --example websocket_echo_bot --no-default-features --features "communication,websocket"
