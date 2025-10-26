#!/bin/bash

echo "=== 最终修复所有crate路径解析错误 ==="

# 修复所有crate::路径错误
echo "修复crate::路径错误..."

# 查找所有包含crate::的Rust文件
find crates -name "*.rs" -type f -print0 | xargs -0 grep -l "crate::" | while IFS= read -r -d '' file; do
    echo "处理文件: $file"

    # 统计crate::出现次数
    crate_count=$(grep -c "crate::" "$file" | wc -l | tr -d ' ')
    echo "  发现 $crate_count 个crate::引用"

    # 如果文件包含crate::，进行批量替换
    if [ "$crate_count" -gt 0 ]; then
        echo "  批量替换crate::为open_lark_core::"

        # 备份原文件
        cp "$file" "$file.backup"

        # 执行替换
        sed -i '' 's/crate::/open_lark_core::/g' "$file"

        # 特殊处理一些常见模式
        sed -i '' 's/use crate::\([^:]*)::/use open_lark_core::core::\1::/g' "$file"
        sed -i '' 's/crate::core::/open_lark_core::core::/g' "$file"
        sed -i '' 's/-> crate::core::/-> open_lark_core::core::/g' "$file"
        sed -i '' 's/crate::\([^{}]*)::/open_lark_core::\1::/g' "$file"

        echo "  ✅ 修复完成: $file"
    else
        echo "  ⚠️  未发现crate::引用，跳过"
    fi
done

echo "=== 验证修复结果 ==="
error_count=$(find crates -name "*.rs" -type f -exec grep -l "crate::" {} \; | wc -l)
echo "剩余crate::引用文件数: $error_count"

if [ "$error_count" -gt 0 ]; then
    echo "⚠️ 仍有 $error_count 个文件包含crate::引用"
    find crates -name "*.rs" -type f -exec grep -l "crate::" {} \;
else
    echo "✅ 所有crate::引用已修复"
fi

echo "批量修复完成！"