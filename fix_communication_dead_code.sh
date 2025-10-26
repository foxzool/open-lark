#!/bin/bash

# Add #[allow(dead_code)] to communication module structs that are used by the event system
echo "Adding #[allow(dead_code)] attributes to communication module files..."

# List of files that need the attribute
files=(
    "crates/open-lark-communication/src/im/v1/p2_im_chat_created_v1.rs"
    "crates/open-lark-communication/src/im/v1/p2_im_chat_disbanded_v1.rs"
    "crates/open-lark-communication/src/im/v1/p2_im_chat_member_user_added_v1.rs"
    "crates/open-lark-communication/src/im/v1/p2_im_chat_member_user_deleted_v1.rs"
    "crates/open-lark-communication/src/im/v1/p2_im_chat_updated_v1.rs"
    "crates/open-lark-communication/src/im/v1/p2_im_message_recalled_v1.rs"
)

for file in "${files[@]}"; do
    if [[ -f "$file" ]]; then
        echo "Processing $file..."

        # Add #[allow(dead_code)] before the struct definition
        sed -i '' 's/pub(crate) struct/#[allow(dead_code)]\npub(crate) struct/' "$file"

        echo "Fixed $file"
    else
        echo "File not found: $file"
    fi
done

echo "Done adding #[allow(dead_code)] attributes."