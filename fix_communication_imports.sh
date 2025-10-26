#!/bin/bash

echo "=== 修复communication模块的导入路径问题 ==="

# 修复 crate 开头的导入路径
find crates/open-lark-communication -name "*.rs" -type f -exec sed -i '' 's/use crate::{/use open_lark_core::{/g' {} \;

# 修复 core:: 开头的导入路径
find crates/open-lark-communication -name "*.rs" -type f -exec sed -i '' 's/use core::/use open_lark_core::core::/g' {} \;

# 修复具体的模块导入路径
find crates/open-lark-communication -name "*.rs" -type f -exec sed -i '' 's/crate::service::contact::models::/use crate::contact::models::/g' {} \;
find crates/open-lark-communication -name "*.rs" -type f -exec sed -i '' 's/crate::service::contact::v3::/use crate::contact::v3::/g' {} \;

echo "修复完成！"
