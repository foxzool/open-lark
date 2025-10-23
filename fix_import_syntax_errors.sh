#!/bin/bash

# 批量修复 hr-suite 模块中的导入语法错误
# 这种错误通常是在从 use open_lark_core::core { ... },
# 格式转换为 use open_lark_core::core::{ ... }; 时产生的

echo "正在修复 hr-suite 模块中的导入语法错误..."

# 找到所有需要修复的文件
files=$(find crates/open-lark-hr-suite/src -name "*.rs" -exec grep -l "use open_lark_core::core::{" {} \;)

for file in $files; do
    echo "修复文件: $file"

    # 使用 sed 修复导入语法错误
    # 将 "},\n    crate::hire::models" 模式替换为 "};\nuse crate::hire::models"
    sed -i '' \
        -e 's/},[[:space:]]*$[[:space:]]*crate::hire::models/};\nuse crate::hire::models/g' \
        "$file"

    # 确保没有多余的逗号在结尾
    sed -i '' \
        -e 's/},[[:space:]]*$/};/' \
        "$file"
done

echo "修复完成!"