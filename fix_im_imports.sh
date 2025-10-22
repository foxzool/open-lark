#!/bin/bash

# 修复 IM 模块中的导入路径问题

# 找到所有有问题的 IM 文件
IM_FILES=$(find crates/open-lark-communication/src/im -name "*.rs" -type f)

echo "修复 IM 模块导入路径..."

for file in $IM_FILES; do
    echo "处理文件: $file"

    # 修复 im::models 的导入路径
    sed -i.bak 's/open_lark_core::im::models/crate::im::models/g' "$file"

    # 删除备份文件
    rm -f "$file.bak"
done

echo "IM 模块导入路径修复完成"