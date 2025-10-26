#!/bin/bash

# 修复 contact/mod.rs 文件中的文档注释格式错误
file="crates/open-lark-communication/src/contact/mod.rs"

if [ -f "$file" ]; then
    echo "修复文档注释格式: $file"
    # 将内部文档注释 (//!) 改为普通注释 (//)
    sed -i '' 's/^\/\/! /\/\//! /g' "$file"
    # 删除多余的空行中的文档注释
    sed -i '' '/^\/\/!$/d' "$file"
    echo "文档注释修复完成"
fi