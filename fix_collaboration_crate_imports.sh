#!/bin/bash

echo "=== 修复collaboration模块中的crate导入问题 ==="

# 查找所有有问题的文件
echo "查找需要修复的文件..."
files=$(find crates/open-lark-collaboration -name "*.rs" -type f -exec grep -l "could not find.*open_lark_core" {} \;)

if [ -z "$files" ]; then
    echo "没有找到需要修复的文件"
    exit 0
fi

echo "找到以下文件需要修复："
echo "$files"

# 修复 crate:: 开头的路径（只能在起始位置使用）
find crates/open-lark-collaboration -name "*.rs" -type f -exec sed -i '' 's/    crate::/    /use /g' {} \;

# 修复 super::super:: 这种多层嵌套路径
find crates/open-lark-collaboration -name "*.rs" -type f -exec sed -i '' 's/super::super::/super::/g' {} \;

# 修复 crate::impl_executable_builder 这种用法
find crates/open-lark-collaboration -name "*.rs" -type f -exec sed -i '' 's/use crate::impl_executable_builder/use super::impl_executable_builder/g' {} \;

echo "修复完成！"