#!/bin/bash

# 批量修复Value导入问题
files=$(find crates/openlark-docs/src/bitable -name "*.rs" -type f | xargs grep -l "Value" | grep -v "/models.rs")

for file in $files; do
    if ! grep -q "use serde_json::Value;" "$file"; then
        echo "Fixing $file"
        # 在第一个use语句后添加Value导入
        sed -i '' '/^use.*{$/a\
use serde_json::Value;
' "$file"
    fi
done

echo "Value imports fixed"