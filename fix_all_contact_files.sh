#!/bin/bash

echo "Comprehensive fix for all contact v3 files..."

# List of all files that need fixing
files=(
    "functional_role_member"
    "group_member"
    "job_family"
    "job_level"
    "job_title"
    "scope"
    "unit"
    "user"
    "work_city"
)

# Function to fix import structure
fix_imports() {
    local file="$1"
    local filepath="crates/open-lark-communication/src/contact/v3/${file}.rs"

    if [ -f "$filepath" ]; then
        echo "Fixing $file.rs..."

        # Fix malformed import structure
        sed -i '' '1s/use open_lark_core::core::{/use open_lark_core::core::{/g' "$filepath"
        sed -i '' 's/    contact::models::*,/use crate::contact::models::*;/g' "$filepath"
        sed -i '' 's/},$/};/g' "$filepath"

        # Fix empty ApiRequest structures by adding proper implementations
        sed -i '' 's/let api_req = ApiRequest {$/        let api_req = ApiRequest {/g' "$filepath"
        sed -i '' 's/    };$/        };/g' "$filepath"

        # Fix missing semicolons after Ok() calls
        sed -i '' 's/Ok(resp\.data\.unwrap_or_default())/Ok(resp.data.unwrap_or_default());/g' "$filepath"

        # Fix missing semicolons after Transport calls
        sed -i '' 's/Transport::.*::request(api_req, \&self\.config, None)\.await?$/Transport::<&1>::request(api_req, \&self.config, None).await?;/g' "$filepath"

        echo "Fixed $file.rs"
    fi
}

# Apply fixes to all files
for file in "${files[@]}"; do
    fix_imports "$file"
done

echo "Comprehensive fix completed"