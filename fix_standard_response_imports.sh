#!/bin/bash

# 为所有需要 StandardResponse trait 的文件添加导入
files=(
    "crates/open-lark-communication/src/im/v1/chats.rs"
    "crates/open-lark-communication/src/im/v1/pin/mod.rs"
    "crates/open-lark-communication/src/im/v1/url_preview/mod.rs"
    "crates/open-lark-communication/src/im/v1/message/send.rs"
    "crates/open-lark-communication/src/im/v1/batch_message/mod.rs"
    "crates/open-lark-communication/src/im/v1/buzz_messages/mod.rs"
    "crates/open-lark-communication/src/im/v1/message_reaction/mod.rs"
    "crates/open-lark-communication/src/im/v1/file/mod.rs"
    "crates/open-lark-communication/src/im/v1/image/mod.rs"
    "crates/open-lark-communication/src/im/v1/message_card/mod.rs"
)

for file in "${files[@]}"; do
    if [ -f "$file" ]; then
        echo "添加 StandardResponse 导入: $file"
        # 检查是否已经有这个导入
        if ! grep -q "use open_lark_core::core::standard_response::StandardResponse;" "$file"; then
            # 在第一个 use open_lark_core 语句之前添加
            sed -i '' '/^use open_lark_core::core::{/i\
use open_lark_core::core::standard_response::StandardResponse;
' "$file"
        fi
    fi
done

echo "StandardResponse 导入添加完成"