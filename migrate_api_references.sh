#!/bin/bash

# API引用迁移脚本
# 将旧的api_req和api_resp引用迁移到新的API模块

set -e

echo "🔄 开始API引用迁移..."

# 备份重要文件
echo "📦 备份当前状态..."
git add .
git commit -m "chore: 迁移前的状态备份" || echo "无需备份，状态已是最新"

# 定义要处理的文件列表
files=(
    "crates/openlark-core/src/http.rs"
    "crates/openlark-core/src/validation/pagination/mod.rs"
    "crates/openlark-core/src/request_builder/mod.rs"
    "crates/openlark-core/src/request_executor.rs"
    "crates/openlark-core/src/req_translator.rs"
    "crates/openlark-core/src/error/handler.rs"
    "crates/openlark-core/src/test_utils.rs"
    "crates/openlark-core/src/migration_guide.rs"
    "crates/openlark-core/src/request_executor_example.rs"
    "crates/openlark-core/src/improved_response_handler.rs"
    "crates/openlark-core/src/standard_response.rs"
    "crates/openlark-core/src/token_manager.rs"
    "crates/openlark-core/src/trait_system/macros.rs"
    "crates/openlark-core/src/api/traits.rs"
)

# 处理每个文件
for file in "${files[@]}"; do
    if [ -f "$file" ]; then
        echo "🔧 处理文件: $file"

        # 备份原文件
        cp "$file" "$file.backup"

        # 替换 api_req:: 为新的导入
        sed -i.tmp 's/use crate::api_req::/use crate::api::{LegacyApiRequest as ApiRequest, /g' "$file" || true
        sed -i.tmp 's/api_req::ApiRequest/ApiRequest/g' "$file" || true

        # 替换 api_resp:: 为新的导入
        sed -i.tmp 's/use crate::api_resp::{\([^}]*\)}/use crate::api::{LegacyApiResponse as ApiResponse, LegacyBaseResponse as BaseResponse, \1}/g' "$file" || true
        sed -i.tmp 's/api_resp::BaseResponse/BaseResponse/g' "$file" || true
        sed -i.tmp 's/api_resp::ApiResponseTrait/ApiResponse/g' "$file" || true
        sed -i.tmp 's/api_resp::RawResponse/RawResponse/g' "$file" || true

        # 清理临时文件
        rm -f "$file.tmp"

        echo "✅ 完成处理: $file"
    else
        echo "⚠️  文件不存在: $file"
    fi
done

# 处理contact模块中的文件
contact_files=(
    "crates/openlark-core/src/contact/v3"/*.rs
    "crates/openlark-core/src/contact/models"/*.rs
    "crates/openlark-core/src/contact"/*.rs.working
)

for pattern in "${contact_files[@]}"; do
    for file in $pattern; do
        if [ -f "$file" ] && [[ "$file" != *"backup"* ]]; then
            echo "🔧 处理contact文件: $file"

            # 备份原文件
            cp "$file" "$file.backup"

            # 添加新的导入
            if ! grep -q "use crate::api::" "$file"; then
                sed -i.tmp '1i use crate::api::{LegacyApiRequest as ApiRequest, LegacyBaseResponse as BaseResponse};' "$file" || true
            fi

            # 替换引用
            sed -i.tmp 's/api_req::ApiRequest/ApiRequest/g' "$file" || true
            sed -i.tmp 's/api_resp::BaseResponse/BaseResponse/g' "$file" || true
            sed -i.tmp 's/api_resp::ApiResponseTrait/ApiResponse/g' "$file" || true
            sed -i.tmp 's/api_resp::RawResponse/RawResponse/g' "$file" || true

            # 清理临时文件
            rm -f "$file.tmp"

            echo "✅ 完成处理contact文件: $file"
        fi
    done
done

echo "🧪 验证编译状态..."
if cargo check -p openlark-core; then
    echo "✅ 编译成功！"

    echo "🗑️  清理备份文件..."
    find crates/openlark-core/src -name "*.backup" -delete

    echo "📝 提交更改..."
    git add .
    git commit -m "refactor(api): 迁移到新的API模块结构

- 将api_req::引用迁移到新的api模块
- 将api_resp::引用迁移到新的api模块
- 添加向后兼容的类型别名
- 更新所有相关文件的导入语句

注意：此提交保持向后兼容性，后续可逐步移除旧模块"

    echo "🎉 API引用迁移完成！"
else
    echo "❌ 编译失败，回滚更改..."

    # 恢复备份文件
    find crates/openlark-core/src -name "*.backup" | while read backup; do
        original="${backup%.backup}"
        mv "$backup" "$original"
    done

    echo "🔄 已恢复到迁移前状态"
    exit 1
fi