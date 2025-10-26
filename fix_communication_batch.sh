#!/bin/bash

echo "=== 批量修复communication模块中的语法错误 ==="

# 1. 修复所有缺少函数体闭合大括号的问题
echo "修复函数体闭合大括号问题..."
for file in $(find crates/open-lark-communication -name "*.rs" -type f); do
    echo "处理文件: $file"

    # 为每个缺少闭合大括号的函数添加大括号
    # 查找所有以 ) -> open_lark_core::core::SDKResult< 开头但缺少 { 的行
    sed -i '' 's/    ) -> open_lark_core::core::SDKResult<[^>]*> {$/    ) -> open_lark_core::core::SDKResult<\1> {\n/' "$file"

    # 为每个函数添加闭合大括号 }
    # 查找函数结束模式，在合适位置添加 }
    sed -i '' '/Ok(resp\.data\.unwrap_or_default())/{
        /Ok(resp\.data\.unwrap_or_default())/a\
    }
' "$file"

    # 修复ApiRequest构造模式
    sed -i '' 's/let api_req = ApiRequest {$/let mut api_req = ApiRequest::default();/g' "$file"
    sed -i '' '/let mut api_req = ApiRequest::default();/,/Default::default()/{
        /let mut api_req = ApiRequest::default();/{
            s/            http_method: reqwest::Method::POST,/            api_req.set_http_method(reqwest::Method::POST);/
            s/            http_method: reqwest::Method::GET,/            api_req.set_http_method(reqwest::Method::GET);/
            s/            api_path: EndpointBuilder::replace_param(/            api_req.set_api_path(EndpointBuilder::replace_param(/g
            s/            api_path: open_lark_core::core::endpoints::[^,]*,/            api_req.set_api_path(open_lark_core::core::endpoints::\1);/g
            s/            supported_access_token_types: vec!\[AccessTokenType::Tenant\],/            api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);/g
            s/            body: serde_json::to_vec(req)?,/            api_req.body = serde_json::to_vec(req)?;/g
            s/            body: Vec::new(),/            api_req.body = Vec::new();/g
            s/            query_params: std::collections::HashMap::new(),/            api_req.query_params = std::collections::HashMap::new();/g
            /            ..Default::default()/d
        }' "$file"
done

echo "批量修复完成！"