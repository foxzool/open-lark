#!/bin/bash

# 修复 contact v3 中未完成的方法实现

# 需要修复的文件列表
FILES=(
    "crates/open-lark-communication/src/contact/v3/functional_role_member.rs"
    "crates/open-lark-communication/src/contact/v3/group_member.rs"
    "crates/open-lark-communication/src/contact/v3/job_family.rs"
    "crates/open-lark-communication/src/contact/v3/job_level.rs"
)

echo "修复未完成的方法实现..."

for file in "${FILES[@]}"; do
    if [ -f "$file" ]; then
        echo "处理文件: $file"

        # 使用 Python 脚本修复未完成的方法
        python3 << 'EOF'
import re
import sys

# 读取文件内容
with open(sys.argv[1], 'r', encoding='utf-8') as f:
    content = f.read()

# 修复模式：找到空的 ApiRequest 和未完成的 resp 赋值
pattern = r'(\s+)let api_req = ApiRequest {\s*};\s*\n(\s+)let resp =\s*\n(\s+)Ok\(resp\.data\.unwrap_or_default\(\)\);'

def replacement(match):
    indent = match.group(1)
    return f'''{indent}let api_req = ApiRequest {{
{indent}    http_method: reqwest::Method::POST,
{indent}    api_path: open_lark_core::core::endpoints::contact::CONTACT_V3_FUNCTIONAL_ROLES.to_string(),
{indent}    supported_access_token_types: vec![AccessTokenType::Tenant],
{indent}    body: serde_json::to_vec(req)?,
{indent}    ..Default::default()
{indent} }};

{match.group(2)}let resp = Transport::<_>::request(api_req, &self.config, None).await?;
{match.group(3)}Ok(resp.data.unwrap_or_default());'''

# 应用修复
content = re.sub(pattern, replacement, content, flags=re.MULTILINE)

# 写回文件
with open(sys.argv[1], 'w', encoding='utf-8') as f:
    f.write(content)

print(f"修复完成: {sys.argv[1]}")
EOF "$file"
    fi
done

echo "未完成方法修复完成"