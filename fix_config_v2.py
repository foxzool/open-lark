#!/usr/bin/env python3
import re
import os

def fix_config_literal(content):
    """Fix Config literal patterns in content"""

    # 更复杂的模式匹配，处理多行和不同字段组合

    # 匹配完整的Config结构体（多行）
    pattern = r'Config\s*\{\s*([^}]+)\s*\}'

    def replace_config_block(match):
        fields_block = match.group(1)

        # 解析字段
        fields = {}

        # 匹配各种字段模式
        field_patterns = [
            (r'app_id:\s*"([^"]+)"\.to_string\(\)', 'app_id'),
            (r'app_secret:\s*"([^"]+)"\.to_string\(\)', 'app_secret'),
            (r'base_url:\s*"([^"]+)"\.to_string\(\)', 'base_url'),
            (r'app_type:\s*(AppType::[^,\s]+)', 'app_type'),
            (r'req_timeout:\s*Some\(Duration::from_secs\((\d+)\)\)', 'req_timeout'),
            (r'enable_token_cache:\s*(true|false)', 'enable_token_cache'),
        ]

        for pattern, field_name in field_patterns:
            matches = re.findall(pattern, fields_block)
            if matches:
                fields[field_name] = matches[0]

        # 构建 builder 调用
        builder_calls = []

        if 'app_id' in fields:
            builder_calls.append(f'.app_id("{fields["app_id"]}")')
        if 'app_secret' in fields:
            builder_calls.append(f'.app_secret("{fields["app_secret"]}")')
        if 'base_url' in fields:
            builder_calls.append(f'.base_url("{fields["base_url"]}")')
        if 'app_type' in fields:
            builder_calls.append(f'.app_type({fields["app_type"]})')
        if 'req_timeout' in fields:
            builder_calls.append(f'.req_timeout(Duration::from_secs({fields["req_timeout"]}))')
        if 'enable_token_cache' in fields:
            builder_calls.append(f'.enable_token_cache({fields["enable_token_cache"]})')

        if builder_calls:
            return f'Config::builder(){"".join(builder_calls)}.build()'
        else:
            return 'Config::default()'

    # 应用替换
    return re.sub(pattern, replace_config_block, content, flags=re.MULTILINE | re.DOTALL)

# 找到所有需要修复的文件
files_to_fix = []
for root, dirs, files in os.walk('src/service'):
    for file in files:
        if file.endswith('.rs'):
            filepath = os.path.join(root, file)
            try:
                with open(filepath, 'r') as f:
                    content = f.read()
                    if 'Config {' in content:
                        files_to_fix.append(filepath)
            except:
                pass

print(f'Found {len(files_to_fix)} files to fix')

# 修复每个文件
fixed_files = 0
for filepath in files_to_fix:
    try:
        with open(filepath, 'r') as f:
            content = f.read()

        original_content = content
        content = fix_config_literal(content)

        if content != original_content:
            with open(filepath, 'w') as f:
                f.write(content)
            fixed_files += 1
            print(f'Fixed: {filepath}')
    except Exception as e:
        print(f'Error processing {filepath}: {e}')

print(f'Fixed {fixed_files} files automatically')