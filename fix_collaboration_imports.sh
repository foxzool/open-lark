#!/bin/bash

# Script to fix import issues in collaboration module files

files=(
    "/Users/zool/RustroverProjects/open-lark/crates/open-lark-collaboration/src/cloud_docs/bitable/v1/app_role_member/list.rs"
    "/Users/zool/RustroverProjects/open-lark/crates/open-lark-collaboration/src/cloud_docs/bitable/v1/app_role_member/batch_create.rs"
    "/Users/zool/RustroverProjects/open-lark/crates/open-lark-collaboration/src/cloud_docs/bitable/v1/app_role_member/create.rs"
    "/Users/zool/RustroverProjects/open-lark/crates/open-lark-collaboration/src/cloud_docs/bitable/v1/app_role_member/delete.rs"
    "/Users/zool/RustroverProjects/open-lark/crates/open-lark-collaboration/src/cloud_docs/bitable/v1/app_role_member/batch_delete.rs"
    "/Users/zool/RustroverProjects/open-lark/crates/open-lark-collaboration/src/cloud_docs/bitable/v1/app_table_field/list.rs"
    "/Users/zool/RustroverProjects/open-lark/crates/open-lark-collaboration/src/cloud_docs/bitable/v1/app_table_field/create.rs"
    "/Users/zool/RustroverProjects/open-lark/crates/open-lark-collaboration/src/cloud_docs/bitable/v1/app_table_field/delete.rs"
    "/Users/zool/RustroverProjects/open-lark/crates/open-lark-collaboration/src/cloud_docs/bitable/v1/app_table_field/update.rs"
)

for file in "${files[@]}"; do
    if [ -f "$file" ]; then
        echo "Fixing imports in $file"

        # Create backup
        cp "$file" "$file.backup"

        # Fix the malformed import pattern
        sed -i '' 's/use open_lark_core::core{$/use open_lark_core::core::{/g' "$file"
        sed -i '' '/^use crate::impl_executable_builder_owned;$/d' "$file"
        sed -i '' '/^    impl_executable_builder_owned,$/d' "$file"
        sed -i '' 's/^};$/};\
\
use crate::impl_executable_builder_owned;/g' "$file"

        echo "Fixed $file"
    else
        echo "File not found: $file"
    fi
done

echo "Import fixing completed!"