#!/bin/bash

# 修复API调用 - Transport.request需要config参数
files=$(find crates/openlark-docs/src/bitable -name "*.rs" -type f | xargs grep -l "transport.execute")

for file in $files; do
    echo "Fixing API calls in $file"
    # 将transport.execute改为Transport::request，并添加config参数
    sed -i '' 's/transport\.execute(\&api_request)/Transport::request(api_request, \&self.config, None)/g' "$file"
done

echo "API calls fixed"