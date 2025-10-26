#!/bin/bash

# Add #[allow(dead_code)] to new functions in communication module
echo "Adding #[allow(dead_code)] to new functions in communication module..."

# List of files that need the attribute for the new function
files=(
    "crates/open-lark-communication/src/im/v1/p2_im_message_receive_v1.rs"
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

        # Add #[allow(dead_code)] before the pub(crate) fn new function
        sed -i '' 's/pub(crate) fn new(/#[allow(dead_code)]\npub(crate) fn new(/' "$file"

        echo "Fixed $file"
    else
        echo "File not found: $file"
    fi
done

echo "Done adding #[allow(dead_code)] to new functions."