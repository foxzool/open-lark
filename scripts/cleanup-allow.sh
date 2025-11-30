#!/bin/bash

# 批量清理#![allow(...)]指令的脚本

echo "开始清理 allow 指令..."

# 查找所有包含allow指令的.rs文件
find crates/openlark-docs/src/base crates/openlark-docs/src/bitable -name "*.rs" -type f | while read -r file; do
    echo "处理文件: $file"

    # 创建临时文件
    temp_file=$(mktemp)

    # 删除所有#![allow(...)]行
    sed '/^#!\[allow(/d' "$file" > "$temp_file"

    # 如果文件有变化，则替换原文件
    if ! diff -q "$file" "$temp_file" > /dev/null; then
        echo "  ✓ 已清理 allow 指令"
        mv "$temp_file" "$file"
    else
        rm "$temp_file"
        echo "  - 无需清理"
    fi
done

echo "清理完成！"