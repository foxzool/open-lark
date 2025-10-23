#!/bin/bash

# Script to completely fix import issues in collaboration module files

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
        echo "Completely fixing imports in $file"

        # Replace the empty import with the full proper import
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

echo "Complete import fixing completed!"