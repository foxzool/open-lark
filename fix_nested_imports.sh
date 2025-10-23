#!/bin/bash

# Script to fix nested import issues in collaboration module files

files=(
    "/Users/zool/RustroverProjects/open-lark/crates/open-lark-collaboration/src/cloud_docs/bitable/v1/app_table_field/list.rs"
    "/Users/zool/RustroverProjects/open-lark/crates/open-lark-collaboration/src/cloud_docs/bitable/v1/app_table_field/delete.rs"
    "/Users/zool/RustroverProjects/open-lark/crates/open-lark-collaboration/src/cloud_docs/bitable/v1/app_table_field/update.rs"
    "/Users/zool/RustroverProjects/open-lark/crates/open-lark-collaboration/src/cloud_docs/bitable/v1/app_role_member/batch_create.rs"
)

for file in "${files[@]}"; do
    if [ -f "$file" ]; then
        echo "Fixing nested imports in $file"

        # Move the nested imports outside the core imports block
        sed -i '' '/^use super::v1::/,/^},$/{
            /^use super::v1::/{
                s/.*//
            }
            /^},$/{
                s/.*//
            }
            /^$/d
        }' "$file"

        # Remove empty lines in the core imports block
        sed -i '' '/^use open_lark_core::core::{$/,/^};${
            /^$/d
        }' "$file"

        # Close the core imports block properly if it's not closed
        sed -i '' 's/use open_lark_core::core{$/use open_lark_core::core {\
    api_req::ApiRequest,\
    api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},\
    config::Config,\
    constants::AccessTokenType,\
    endpoints::cloud_docs::*,\
    http::Transport,\
    req_option::RequestOption,\
    SDKResult,\
};/g' "$file"

        echo "Fixed $file"
    else
        echo "File not found: $file"
    fi
done

echo "Nested import fixing completed!"