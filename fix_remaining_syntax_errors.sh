#!/bin/bash

# 修复剩余的语法错误（& 字符）
files=(
    "crates/open-lark-communication/src/im/v1/image/mod.rs"
    "crates/open-lark-communication/src/im/v1/file/mod.rs"
)

for file in "${files[@]}"; do
    if [ -f "$file" ]; then
        echo "修复文件中的 & 字符: $file"
        # 删除独立的 & 字符
        sed -i '' '/^&$/d' "$file"
    fi
done

echo "剩余语法错误修复完成"