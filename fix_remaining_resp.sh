#!/bin/bash

# 批量修复剩余的 resp 变量问题

echo "修复剩余的 resp 变量问题..."

# 需要修复的文件列表
FILES=(
    "crates/open-lark-communication/src/contact/v3/group_member.rs"
    "crates/open-lark-communication/src/contact/v3/job_family.rs"
    "crates/open-lark-communication/src/contact/v3/job_level.rs"
)

for file in "${FILES[@]}"; do
    if [ -f "$file" ]; then
        echo "修复文件: $file"

        # 使用 sed 修复未完成的 resp 赋值
        # 找到模式：let api_req = ApiRequest { }; 后跟 let resp = 或空的 resp 赋值
        sed -i.tmp '
            # 修复第一个模式：create 方法
            s/\(let api_req = ApiRequest {\)\s*};\s*\n\s*let resp =/\1\n            http_method: reqwest::Method::POST,\n            api_path: open_lark_core::core::endpoints::contact::CONTACT_V3_GROUPS.to_string(),\n            supported_access_token_types: vec![AccessTokenType::Tenant],\n            body: serde_json::to_vec(req)?,\n            ..Default::default()\n        };\n\n        let resp =/g

        # 修复第二个模式：其他方法
            s/let resp =\s*$/\n            http_method: reqwest::Method::GET,\n            api_path: open_lark_core::core::endpoints::contact::CONTACT_V3_GROUPS.to_string(),\n            supported_access_token_types: vec![AccessTokenType::Tenant],\n            body: Vec::new(),\n            query_params: std::collections::HashMap::new(),\n            ..Default::default()\n        };\n\n        let resp =/g
        ' "$file"

        # 删除临时文件
        rm -f "$file.tmp"
    fi
done

echo "批量修复完成"