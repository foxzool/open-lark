#!/bin/bash

# Fix unused imports in HR suite files
echo "Fixing unused imports in HR suite files..."

# List of files with unused imports
files=(
    "crates/open-lark-hr-suite/src/hire/v1/p2_hire_application_stage_changed_v1.rs"
    "crates/open-lark-hr-suite/src/hire/v1/p2_hire_offer_status_changed_v1.rs"
    "crates/open-lark-hr-suite/src/hire/v1/p2_hire_talent_deleted_v1.rs"
)

for file in "${files[@]}"; do
    if [[ -f "$file" ]]; then
        echo "Processing $file..."

        # Remove the unused imports from lines 3-4 (ApiRequest, ApiResponseTrait, Config, AccessTokenType, Transport)
        sed -i '' '3,4d' "$file"

        echo "Fixed $file"
    else
        echo "File not found: $file"
    fi
done

echo "Done fixing unused imports in HR suite files."