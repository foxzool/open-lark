#!/bin/bash

# 批量修复 open-lark-communication 中的 ApiRequest 私有字段访问错误
# 将 ApiRequest { ... } 结构替换为使用 builder 模式

echo "正在修复 open-lark-communication 中的 ApiRequest 私有字段访问错误..."

# 查找所有包含 ApiRequest { ... } 模式的文件
find crates/open-lark-communication/src -name "*.rs" -exec grep -l "ApiRequest {" {} \; | while read file; do
    echo "修复文件: $file"

    # 创建临时文件
    temp_file=$(mktemp)

    # 使用 sed 进行替换
    sed -i '' -E '
        # 替换 ApiRequest { http_method: ..., ... } 模式
        s/let mut api_req = ApiRequest \{([^}]+)\};/\
            let mut api_req = ApiRequest::default();\
            \1\
        /g;

        # 处理多行 ApiRequest 结构
        /ApiRequest \{/,/\}/ {
            s/ApiRequest \{/let mut api_req = ApiRequest::default();/g;
            s/http_method: ([^,]+),/api_req.set_http_method(\1);/g;
            s/api_path: ([^,]+),/api_req.set_api_path(\1);/g;
            s/supported_access_token_types: ([^,]+),/api_req.set_supported_access_token_types(\1);/g;
            s/body: ([^,]+),/api_req.body = \1;/g;
            s/\.\.\.Default::default\(\)//g;
            s/\}$//g;
        }
    ' "$file" > "$temp_file"

    # 替换原文件
    mv "$temp_file" "$file"
done

echo "修复完成!"