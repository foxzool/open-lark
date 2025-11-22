#!/bin/bash

# 修复ApiRequest::new()调用 - 改为使用具体的HTTP方法
files=$(find crates/openlark-docs/src/bitable -name "*.rs" -type f | xargs grep -l "ApiRequest::new()")

for file in $files; do
    echo "Fixing ApiRequest::new in $file"
    # 将ApiRequest::new()改为ApiRequest::post()，因为大多数是POST请求
    sed -i '' 's/ApiRequest::new()/ApiRequest::post("")/g' "$file"
done

echo "ApiRequest::new calls fixed"