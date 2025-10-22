#!/bin/bash

echo "Manually fixing syntax errors..."

# Fix the malformed import structure in each file
for file in employee_type_enum group functional_role functional_role_member group_member job_family job_level job_title scope unit user work_city; do
  filepath="/Users/zool/RustroverProjects/open-lark/crates/open-lark-communication/src/contact/v3/${file}.rs"
  if [ -f "$filepath" ]; then
    # Fix the broken import
    sed -i '' '1s/use open_lark_core::core::{/use open_lark_core::core::{/g' "$filepath"
    # Remove the extra empty line
    sed -i '' '/^        /d' "$filepath"
    # Fix the closing brace
    sed -i '' 's/    },$/    },/g' "$filepath"
    # Fix contact import to be a proper use statement
    sed -i '' 's/    contact::models::,/use crate::contact::models::*;/g' "$filepath"
    # Fix the closing brace structure
    sed -i '' 's/},$/};/g' "$filepath"
  fi
done

echo "Manual syntax fixes completed"