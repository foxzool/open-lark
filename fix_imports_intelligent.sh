#!/bin/bash

echo "=== 智能修复导入路径错误 ==="

# 修复open_lark_core相关导入
echo "修复open_lark_core导入问题..."

# 1. 修复use open_lark_core::core::{开头的导入
find crates -name "*.rs" -type f -print0 | xargs -0 grep -l "use open_lark_core::core::{" | while IFS= read -r -d '' file; do
    echo "修复core导入: $file"
    # 将多层导入转换为单层
    sed -i '' 's/use open_lark_core::core::{$[^}]*} {$/use open_lark_core::core::{/g' "$file"
    # 闭合花括号
    sed -i '' 's/use open_lark_core::core::{$[^}]*} {$/use open_lark_core::core::{\n&/g' "$file"
    # 移除多余的花括号
    sed -i '' 's/}$/use open_lark_core::core::{/g' "$file"
done

# 2. 修复crate::开头的路径（主要在enterprise模块）
echo "修复crate::路径问题..."
find crates/open-lark-enterprise -name "*.rs" -type f -exec grep -l "crate::" {} \; | while read file; do
    echo " 修复enterprise模块: $file"

    # 统计错误数量
    error_count=$(grep -c "crate::" "$file" | wc -l | tr -d ' ')
    if [ "$error_count" -gt 0 ]; then
        # 根据错误类型进行针对性修复
        # 修复简单的crate::模块路径
        sed -i '' 's/crate::[^:]*::/open_lark_core::/g' "$file"

        # 修复复杂的crate::core::路径
        sed -i '' 's/crate::core::/open_lark_core::core::/g' "$file"

        # 修复返回类型路径
        sed -i '' 's/-> crate::core::/-> open_lark_core::core::/g' "$file"

        echo "  ✅ 修复完成: $file ($error_count 个错误)"
    fi
done

# 3. 修复hr-suite模块中的use crate::{模式
echo "修复hr-suite模块导入..."
find crates/open-lark-hr-suite -name "*.rs" -type f -exec grep -l "use crate::{" {} \; | while read file; do
    echo "修复hr-suite模块: $file"
    sed -i '' 's/use crate::{/use open_lark_core::{/g' "$file"
done

# 4. 修复ApiRequest构造问题
echo "修复ApiRequest构造模式..."
find crates -name "*.rs" -type f -exec grep -l "let api_req = ApiRequest {" {} \; | while read file; do
    echo "修复ApiRequest: $file"
    # 替换为新的builder模式
    sed -i '' 's/let api_req = ApiRequest {$/let mut api_req = ApiRequest::default();/g' "$file"

    # 添加必要的set方法调用
    sed -i '' '/let mut api_req = ApiRequest::default();/,/http_method: reqwest::Method::/,/{
        s/            http_method: reqwest::Method::\([^,]*),/            api_req.set_http_method(reqwest::Method::\1);/
    }' "$file"

    sed -i '' '/let mut api_req = ApiRequest::default();/,/api_path: /,/{
        s/            api_path: \([^,]*),/            api_req.set_api_path(\1);/
    }' "$file"

    sed -i '' '/let mut api_req = ApiRequest::default();/,/supported_access_token_types:/,/{
        s/            supported_access_token_types: \([^,]*),/            api_req.set_supported_access_token_types(\1);/
    }' "$file"

    sed -i '' '/let mut api_req = ApiRequest::default();/,/body: /,/{
        s/            body: \([^,]*),/            api_req.body = \1;/
    }' "$file"

    sed -i '' '/let mut api_req = ApiRequest::default();/,/query_params: /,/{
        s/            query_params: \([^,]*),/            api_req.query_params = \1;/
    }' "$file"

    # 移除旧的构造模式
    sed -i '' '/let mut api_req = ApiRequest::default();/,/            \.\.\.default()/,/        /            \.\.\.default()/d
    }' "$file"

    # 移除多余的分号
    sed -i '' 's/};$/}/g' "$file"

    echo "  ✅ 修复完成: $file"
done

echo "=== 验证修复结果 ==="
total_errors=$(find crates -name "*.rs" -type f -exec grep -l "crate::" {} \; | wc -l)
echo "剩余crate::错误总数: $total_errors"

if [ "$total_errors" -gt 0 ]; then
    echo "⚠️  仍有 $total_errors 个crate::引用需要手动检查"
else
    echo "✅ 所有crate::引用已修复"
fi

echo "智能修复完成！"