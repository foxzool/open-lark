#!/bin/bash

echo "修复ApiRequest语法错误..."

# 修复语法错误的ApiRequest构造
files=$(find crates/openlark-docs/src/bitable -name "*.rs" -type f | xargs grep -l "ApiRequest::" | while read file; do
    if grep -q "url: format!(" "$file" && grep -q "query()" "$file"; then
        echo "修复API构造语法: $file"

        # 修复错误的ApiRequest构造语法
        sed -i '' 's/let api_req = ApiRequest::/let api_req = ApiRequest::get(/g' "$file"
        sed -i '' 's/url: format!(/"/g' "$file"
        sed -i '' 's/\.query()/.query(HashMap::new())/g' "$file"
        sed -i '' '/url: format!/,${
            /query()/d
            /url: format!(/d
            s/^\.$//
        }' "$file"

        # 清理重复的query()调用
        sed -i '' 's/\.query(HashMap::new())\.query(HashMap::new())/.query(HashMap::new())/g' "$file"
    fi
done

echo "ApiRequest语法修复完成"