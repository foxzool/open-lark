#!/bin/bash

echo "修复v2模块的use语句问题..."

# 修复v2模块中的错误use语句
files=$(find crates/openlark-docs/src/bitable/v2 -name "*.rs" -type f)

for file in $files; do
    echo "修复use语句: $file"

    # 修复错误的use语句插入
    sed -i '' '/^use {$/,/^};$/{
        /use serde_json::Value;/d
        /^use {$/{
            N
            /^use {\nuse serde_json::Value;/d
        }
    }' "$file"

    # 在正确位置添加use语句
    if ! grep -q "use serde_json::Value;" "$file" && grep -q "Value" "$file"; then
        sed -i '' '/^use std::collections::HashMap;/a\
use serde_json::Value;
' "$file"
    fi

    # 修复Transport构造函数调用
    sed -i '' 's/Transport::new(&self.config)/Transport::new()/g' "$file"
done

echo "v2模块use语句修复完成"