#!/bin/bash

# 修复迁移后模块中的导入路径
# 将 openlark:: 或 crate:: 替换为 openlark_core::

echo "修复 openlark-docs 模块中的导入路径..."

# 修复 docs 模块
find crates/openlark-docs/src/docs/ -name "*.rs" -type f -exec sed -i '' 's/use config::/use openlark_core::config::/g' {} \;
find crates/openlark-docs/src/docs/ -name "*.rs" -type f -exec sed -i '' 's/use crate::/use openlark_core::/g' {} \;
find crates/openlark-docs/src/docs/ -name "*.rs" -type f -exec sed -i '' 's/use openlark::/use openlark_core::/g' {} \;

# 修复 sheet 模块
find crates/openlark-docs/src/sheet/ -name "*.rs" -type f -exec sed -i '' 's/use config::/use openlark_core::config::/g' {} \;
find crates/openlark-docs/src/sheet/ -name "*.rs" -type f -exec sed -i '' 's/use crate::/use openlark_core::/g' {} \;
find crates/openlark-docs/src/sheet/ -name "*.rs" -type f -exec sed -i '' 's/use openlark::/use openlark_core::/g' {} \;

# 修复 bitable 模块
find crates/openlark-docs/src/bitable/ -name "*.rs" -type f -exec sed -i '' 's/use config::/use openlark_core::config::/g' {} \;
find crates/openlark-docs/src/bitable/ -name "*.rs" -type f -exec sed -i '' 's/use crate::/use openlark_core::/g' {} \;
find crates/openlark-docs/src/bitable/ -name "*.rs" -type f -exec sed -i '' 's/use openlark::/use openlark_core::/g' {} \;

# 修复 wiki 模块
find crates/openlark-docs/src/wiki/ -name "*.rs" -type f -exec sed -i '' 's/use config::/use openlark_core::config::/g' {} \;
find crates/openlark-docs/src/wiki/ -name "*.rs" -type f -exec sed -i '' 's/use crate::/use openlark_core::/g' {} \;
find crates/openlark-docs/src/wiki/ -name "*.rs" -type f -exec sed -i '' 's/use openlark::/use openlark_core::/g' {} \;

# 修复 drive 模块
find crates/openlark-docs/src/drive/ -name "*.rs" -type f -exec sed -i '' 's/use config::/use openlark_core::config::/g' {} \;
find crates/openlark-docs/src/drive/ -name "*.rs" -type f -exec sed -i '' 's/use crate::/use openlark_core::/g' {} \;
find crates/openlark-docs/src/drive/ -name "*.rs" -type f -exec sed -i '' 's/use openlark::/use openlark_core::/g' {} \;

# 修复 ccm 模块
find crates/openlark-docs/src/ccm/ -name "*.rs" -type f -exec sed -i '' 's/use config::/use openlark_core::config::/g' {} \;
find crates/openlark-docs/src/ccm/ -name "*.rs" -type f -exec sed -i '' 's/use crate::/use openlark_core::/g' {} \;
find crates/openlark-docs/src/ccm/ -name "*.rs" -type f -exec sed -i '' 's/use openlark::/use openlark_core::/g' {} \;

# 修复其他常见的导入模式
find crates/openlark-docs/src/ -name "*.rs" -type f -exec sed -i '' 's/use api_resp::/use openlark_core::api_resp::/g' {} \;
find crates/openlark-docs/src/ -name "*.rs" -type f -exec sed -i '' 's/use error::/use openlark_core::error::/g' {} \;
find crates/openlark-docs/src/ -name "*.rs" -type f -exec sed -i '' 's/use constants::/use openlark_core::constants::/g' {} \;
find crates/openlark-docs/src/ -name "*.rs" -type f -exec sed -i '' 's/use client::/use openlark_core::client::/g' {} \;

echo "导入路径修复完成"