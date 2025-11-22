#!/bin/bash

echo "修复Value导入问题..."

# 找到所有使用Value但没有导入serde_json::Value的文件
files=$(find crates/openlark-docs/src/bitable -name "*.rs" -type f | xargs grep -l "Value" | while read file; do
    if ! grep -q "use serde_json::Value;" "$file" && ! grep -q "pub use openlark_core::prelude::\*;" "$file"; then
        echo "$file"
    fi
done)

for file in $files; do
    echo "修复Value导入: $file"

    # 找到第一个use语句的位置
    first_use_line=$(grep -n "^use " "$file" | head -1 | cut -d: -f1)

    if [ -n "$first_use_line" ]; then
        # 在第一个use语句前插入serde_json::Value导入
        sed -i '' "${first_use_line}i\\
use serde_json::Value;
" "$file"
    else
        # 如果没有use语句，在第一个mod或struct前插入
        first_content_line=$(grep -n "^\(pub\|struct\|enum\|impl\|#\!\[" "$file" | head -1 | cut -d: -f1)
        if [ -n "$first_content_line" ]; then
            sed -i '' "${first_content_line}i\\
use serde_json::Value;
" "$file"
        fi
    fi
done

echo "Value导入修复完成"