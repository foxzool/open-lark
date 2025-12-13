#!/bin/bash

# 需要修复的文件列表
files=(
  "crates/openlark-docs/src/ccm/doc/v2/batch_update.rs"
  "crates/openlark-docs/src/ccm/doc/v2/content.rs"
  "crates/openlark-docs/src/ccm/doc/v2/create.rs"
  "crates/openlark-docs/src/ccm/doc/v2/meta.rs"
  "crates/openlark-docs/src/ccm/doc/v2/sheet_meta.rs"
  "crates/openlark-docs/src/ccm/drive/explorer/services.rs"
  "crates/openlark-docs/src/ccm/drive/permission/services.rs"
  "crates/openlark-docs/src/ccm/export_tasks/services.rs"
  "crates/openlark-docs/src/ccm/wiki/v2/space_member.rs"
  "crates/openlark-docs/src/ccm/wiki/v2/space_node.rs"
  "crates/openlark-docs/src/ccm/wiki/v2/task.rs"
)

for file in "${files[@]}"; do
  if [ -f "$file" ]; then
    echo "修复文件: $file"
    # 检查是否已经导入了 HttpMethod
    if ! grep -q "HttpMethod," "$file"; then
      # 在 ApiRequest 后添加 HttpMethod 导入
      sed -i '' 's/api::{ApiRequest,/api::{ApiRequest, HttpMethod,/' "$file"
      sed -i '' 's/api::ApiRequest,/api::{ApiRequest, HttpMethod},/' "$file"
    fi
  fi
done

echo "完成！"
