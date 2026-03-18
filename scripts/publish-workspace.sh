#!/bin/bash
set -euo pipefail

echo "🚀 OpenLark Workspace Publish Script"
echo "===================================="
echo ""

# Configuration
SLEEP_DURATION=30
DRY_RUN="${DRY_RUN:-false}"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Publish function
publish_crate() {
    local crate=$1
    echo -e "${YELLOW}Publishing ${crate}...${NC}"
    
    if [ "$DRY_RUN" = "true" ]; then
        echo "  [DRY RUN] cargo publish -p ${crate} --dry-run"
        cargo publish -p "${crate}" --dry-run 2>&1 | head -20 || true
    else
        echo "  Publishing to crates.io..."
        if cargo publish -p "${crate}" 2>&1; then
            echo -e "${GREEN}✅ ${crate} published successfully${NC}"
        else
            echo -e "${RED}❌ Failed to publish ${crate}${NC}"
            return 1
        fi
        
        echo "  Waiting ${SLEEP_DURATION}s for index propagation..."
        sleep "${SLEEP_DURATION}"
    fi
    echo ""
}

echo "📦 Layer 1: Foundation (Protocol)"
echo "-----------------------------------"
publish_crate "openlark-protocol"

echo "📦 Layer 2: Core"
echo "----------------"
publish_crate "openlark-core"

echo "📦 Layer 3: Business Crates"
echo "---------------------------"
BUSINESS_CRATES=(
    "openlark-auth"
    "openlark-security"
    "openlark-communication"
    "openlark-cardkit"
    "openlark-webhook"
    "openlark-docs"
    "openlark-hr"
    "openlark-ai"
    "openlark-application"
    "openlark-platform"
    "openlark-meeting"
    "openlark-helpdesk"
    "openlark-mail"
    "openlark-workflow"
    "openlark-analytics"
    "openlark-user"
)

for crate in "${BUSINESS_CRATES[@]}"; do
    publish_crate "${crate}"
done

echo "📦 Layer 4: Client"
echo "------------------"
publish_crate "openlark-client"

echo "📦 Layer 5: Root Crate"
echo "---------------------"
publish_crate "openlark"

echo -e "${GREEN}====================================${NC}"
echo -e "${GREEN}🎉 All crates published successfully!${NC}"
echo -e "${GREEN}====================================${NC}"
