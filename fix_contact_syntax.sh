#!/bin/bash

# Fix syntax errors in contact files
echo "Fixing contact syntax errors..."

# Fix each file manually
for file in employee_type_enum group functional_role functional_role_member group_member job_family job_level job_title scope unit user work_city; do
  filepath="/Users/zool/RustroverProjects/open-lark/crates/open-lark-communication/src/contact/v3/${file}.rs"
  if [ -f "$filepath" ]; then
    echo "Fixing ${file}.rs..."
    # Fix the broken import structure
    sed -i '' '1s/use open_lark_core::core::{/use open_lark_core::core::{/g' "$filepath"
    sed -i '' '2,$s/^        /    /g' "$filepath"
    sed -i '' 's/    },$/    },/g' "$filepath"
    # Fix contact import
    sed -i '' 's/    contact::models::,/use crate::contact::models::*;/g' "$filepath"
    sed -i '' 's/},$/};/g' "$filepath"
  fi
done

echo "Contact syntax fixes completed"