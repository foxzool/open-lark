#!/bin/bash

# Script to fix all nested import issues in collaboration module

echo "Fixing nested imports systematically..."

# Find all files with the malformed pattern and fix them
find /Users/zool/RustroverProjects/open-lark/crates/open-lark-collaboration/src -name "*.rs" | while read file; do
    if grep -q "use open_lark_core::core{" "$file" && grep -q "use super::" "$file"; then
        echo "Processing: $file"

        # Create backup
        cp "$file" "$file.backup"

        # Remove the nested imports from inside the core block
        sed -i '' '/^use open_lark_core::core::{$/,/^};${
            /use super::/d
        }' "$file"

        # Close the core imports properly if not already closed
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

        # Remove any duplicate closing braces
        sed -i '' '/^};$/{
            N
            s/};};/};/
        }' "$file"

        echo "Fixed: $file"
    fi
done

echo "Nested import fixing completed!"