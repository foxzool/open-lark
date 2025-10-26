#!/bin/bash

echo "=== 修复collaboration模块的导入路径问题 ==="

# 查找所有包含 "open_lark_core::core" 的文件
echo "查找需要修复的文件..."
find crates/open-lark-collaboration -name "*.rs" -type f -exec grep -l "open_lark_core::core" {} \;

echo ""
echo "=== 修复导入路径 ==="

# 修复 open_lark_core::core 导入路径问题
find crates/open-lark-collaboration -name "*.rs" -type f -exec sed -i '' 's/use open_lark_core::core::{/use open_lark_core::core::{/g' {} \;

# 修复 crate 开头的路径问题（只能在起始位置使用）
find crates/open-lark-collaboration -name "*.rs" -type f -exec sed -i '' 's/    crate::/    super::/g' {} \;

# 修复 service:: 开头的导入路径
find crates/open-lark-collaboration -name "*.rs" -type f -exec sed -i '' 's/service::bitable/crate::cloud_docs::bitable/g' {} \;
find crates/open-lark-collaboration -name "*.rs" -type f -exec sed -i '' 's/service::sheets/crate::cloud_docs::sheets/g' {} \;
find crates/open-lark-collaboration -name "*.rs" -type f -exec sed -i '' 's/service::drive/crate::cloud_docs::drive/g' {} \;

# 修复 core:: 导入路径
find crates/open-lark-collaboration -name "*.rs" -type f -exec sed -i '' 's/use core::/use open_lark_core::core::/g' {} \;

# 修复 event:: 导入路径
find crates/open-lark-collaboration -name "*.rs" -type f -exec sed -i '' 's/use crate::event::/use open_lark_core::event::/g' {} \;

# 修复 impl_executable_builder_config 导入问题
find crates/open-lark-collaboration -name "*.rs" -type f -exec sed -i '' 's/impl_executable_builder_config/impl_executable_builder_owned/g' {} \;

echo "修复完成！"
