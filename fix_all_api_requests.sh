#!/bin/bash

# 批量修复所有ApiRequest私有字段访问问题

echo "开始批量修复所有ApiRequest私有字段访问问题..."

# 查找所有包含ApiRequest结构体初始化的文件
files=$(find crates/open-lark-communication crates/open-lark-collaboration -name "*.rs" -not -path "*/target/*" -exec grep -l "ApiRequest {" {} \;)

total_files=$(echo "$files" | wc -l)
echo "找到 $total_files 个包含ApiRequest初始化的文件"

current_file=0
fixed_count=0

for file in $files; do
    current_file=$((current_file + 1))
    echo "[$current_file/$total_files] 处理文件: $file"

    # 创建临时文件
    temp_file=$(mktemp)

    # 使用Python进行更精确的文本替换
    python3 -c "
import re
import sys

def fix_api_request_init(content):
    # 匹配 ApiRequest 结构体初始化
    pattern = r'(\w+\s*=\s*)(\w*::)?ApiRequest\s*\{([^}]+)\}'

    def replacer(match):
        var_prefix = match.group(1)
        namespace = match.group(2) if match.group(2) else ''
        fields = match.group(3)

        # 提取字段
        fields_dict = {}
        for line in fields.split(',\n'):
            line = line.strip()
            if ':' in line:
                key, value = line.split(':', 1)
                fields_dict[key.strip()] = value.strip()

        # 生成新的代码
        result_lines = []

        # HTTP方法
        http_method = fields_dict.get('http_method', 'Method::GET')
        if namespace:
            result_lines.append(f'{var_prefix}mut {namespace}ApiRequest::with_method({http_method});')
        else:
            result_lines.append(f'{var_prefix}mut ApiRequest::with_method({http_method});')

        # API路径
        if 'api_path' in fields_dict:
            api_path = fields_dict['api_path']
            var_name = var_prefix.replace('let mut ', '').replace('let ', '').replace(' = ', '')
            result_lines.append(f'        {var_name}.set_api_path({api_path});')

        # 访问令牌类型
        if 'supported_access_token_types' in fields_dict:
            token_types = fields_dict['supported_access_token_types']
            var_name = var_prefix.replace('let mut ', '').replace('let ', '').replace(' = ', '')
            result_lines.append(f'        {var_name}.set_supported_access_token_types({token_types});')

        # 请求体
        if 'body' in fields_dict:
            body = fields_dict['body']
            var_name = var_prefix.replace('let mut ', '').replace('let ', '').replace(' = ', '')
            result_lines.append(f'        {var_name}.set_body({body});')

        # 其他字段
        for field in ['query_params', 'file', 'path_params']:
            if field in fields_dict:
                value = fields_dict[field]
                var_name = var_prefix.replace('let mut ', '').replace('let ', '').replace(' = ', '')
                result_lines.append(f'        {var_name}.{field} = {value};')

        return '\n'.join(result_lines)

    # 读取文件内容
    with open('$file', 'r') as f:
        content = f.read()

    # 应用修复
    fixed_content = re.sub(pattern, replacer, content, flags=re.MULTILINE | re.DOTALL)

    # 写入结果
    with open('$temp_file', 'w') as f:
        f.write(fixed_content)

    # 检查是否有变化
    return content != fixed_content
"

    # 检查是否有变化
    if python3 -c "
import re

with open('$file', 'r') as f:
    content = f.read()

with open('$temp_file', 'r') as f:
    fixed_content = f.read()

print('CHANGED' if content != fixed_content else 'SAME')
" | grep -q "CHANGED"; then
        mv "$temp_file" "$file"
        fixed_count=$((fixed_count + 1))
        echo "  ✓ 已修复"
    else
        rm "$temp_file"
        echo "  - 无需修复"
    fi
done

echo "批量修复完成！共修复了 $fixed_count 个文件"