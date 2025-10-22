#!/bin/bash

# 批量修复ApiRequest私有字段访问的脚本

echo "开始批量修复ApiRequest私有字段访问..."

# 找到所有需要修复的Rust文件
files=$(find crates/open-lark-communication crates/open-lark-collaboration -name "*.rs" -not -path "*/target/*" -not -name "mod.rs" | xargs grep -l "api_req\.\(http_method\|supported_access_token_types\|api_path\) = ")

total_files=$(echo "$files" | wc -l)
echo "找到 $total_files 个需要修复的文件"

current_file=0

for file in $files; do
    current_file=$((current_file + 1))
    echo "[$current_file/$total_files] 修复文件: $file"

    # 创建临时文件
    temp_file=$(mktemp)

    # 应用修复规则
    sed -E '
        # 修复 http_method 赋值
        s|api_req\.http_method = ([^;]+);|api_req.set_http_method(\1);|g

        # 修复 supported_access_token_types 赋值
        s|api_req\.supported_access_token_types = ([^;]+);|api_req.set_supported_access_token_types(\1);|g

        # 修复 api_path 赋值
        s|api_req\.api_path = ([^;]+);|api_req.set_api_path(\1);|g
    ' "$file" > "$temp_file"

    # 检查是否有变化
    if ! cmp -s "$file" "$temp_file"; then
        mv "$temp_file" "$file"
        echo "  ✓ 已修复"
    else
        rm "$temp_file"
        echo "  - 无需修复"
    fi
done

echo "批量修复完成！"