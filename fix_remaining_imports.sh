#!/bin/bash

echo "Fixing remaining malformed import statements..."

# Files that still have malformed imports
files=("job_level" "job_title" "scope" "unit" "user" "work_city")

for file in "${files[@]}"; do
    filepath="crates/open-lark-communication/src/contact/v3/${file}.rs"
    if [ -f "$filepath" ]; then
        echo "Fixing $file.rs..."
        # Fix malformed import structure
        sed -i '' '1s/use open_lark_core::core::{/use open_lark_core::core::{/g' "$filepath"
        sed -i '' 's/        };$/    },/g' "$filepath"
        sed -i '' 's/    contact::models::*,/use crate::contact::models::*;/g' "$filepath"
        sed -i '' 's/},$/};/g' "$filepath"
    fi
done

echo "Fixed remaining import statements"
