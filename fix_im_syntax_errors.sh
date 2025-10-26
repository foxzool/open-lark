#!/bin/bash

# 修复 IM 模块中的语法错误
files=(
    "crates/open-lark-communication/src/im/v1/pin/mod.rs"
    "crates/open-lark-communication/src/im/v1/file/mod.rs"
    "crates/open-lark-communication/src/im/v1/message_card/mod.rs"
    "crates/open-lark-communication/src/im/v1/url_preview/mod.rs"
    "crates/open-lark-communication/src/im/v1/image/mod.rs"
)

for file in "${files[@]}"; do
    if [ -f "$file" ]; then
        echo "修复文件: $file"
        # 使用 sed 修复语法错误
        sed -i '' 's/use open_lark_core::core::{/use open_lark_core::core::{/' "$file"
        sed -i '' '/use open_lark_core::core::standard_response::StandardResponse;/d' "$file"
        sed -i '' 's/&    api_req::ApiRequest,/    api_req::ApiRequest,/' "$file"
    fi
done

echo "语法错误修复完成"