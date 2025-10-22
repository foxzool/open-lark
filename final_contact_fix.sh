#!/bin/bash

echo "Final fix for remaining contact v3 syntax issues..."

# Fix job_family.rs malformed import
sed -i '' 's/    contact::models::*,/use crate::contact::models::*;/g' crates/open-lark-communication/src/contact/v3/job_family.rs
sed -i '' 's/},$/};/g' crates/open-lark-communication/src/contact/v3/job_family.rs

# Fix similar issues in other files
files=("job_level" "job_title" "scope" "unit" "user" "work_city")

for file in "${files[@]}"; do
    filepath="crates/open-lark-communication/src/contact/v3/${file}.rs"
    if [ -f "$filepath" ]; then
        echo "Fixing $file.rs..."
        sed -i '' 's/    contact::models::*,/use crate::contact::models::*;/g' "$filepath"
        sed -i '' 's/},$/};/g' "$filepath"
    fi
done

# Fix incomplete Config::builder() calls
find crates/open-lark-communication/src/contact/v3/ -name "*.rs" -exec sed -i '' 's/let config = Config::builder()$/let config = Config::builder().build();/g' {} \;

echo "Final contact fix completed"