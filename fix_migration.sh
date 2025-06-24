#!/bin/bash

echo "🔧 修复migration转换中的问题..."

# 移除所有错误的"UnknownRequest"宏应用
echo "🗑️ 移除包含UnknownRequest的错误宏应用..."
find src -name "*.rs" -exec sed -i '' '/crate::impl_executable_builder!(/,/);/{/UnknownRequest/,/);/d;}' {} \;

# 移除重复的宏应用 (查找连续的// === Trait实现 comment blocks)
echo "🗑️ 移除重复的trait实现块..."
find src -name "*.rs" -exec sed -i '' '/=== Trait实现：消除重复的execute方法 ===/N;/=== Trait实现：消除重复的execute方法 ===.*=== Trait实现：消除重复的execute方法 ===/,+20d' {} \;

# 修复错位的导入语句
echo "🔧 修复错位的导入语句..."
find src -name "*.rs" -exec sed -i '' 's/use super::models::{/&/; /use crate::core::trait_system::ExecutableBuilder;/{/use super::models::/d; N; s/.*\n//}' {} \;

echo "✅ 修复完成！"