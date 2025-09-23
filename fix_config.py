#!/usr/bin/env python3
import re
import os

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

# 修复每个文件中的 Config 结构体字面量
fixed_files = 0
for filepath in files_to_fix:
    try:
        with open(filepath, 'r') as f:
            content = f.read()

        original_content = content

        # 基本的 app_id + app_secret 模式
        content = re.sub(
            r'Config \{\s*app_id: "([^"]+)"\.to_string\(\),\s*app_secret: "([^"]+)"\.to_string\(\),\s*\.\.Default::default\(\)\s*\}',
            r'Config::builder().app_id("\1").app_secret("\2").build()',
            content,
            flags=re.MULTILINE | re.DOTALL
        )

        # 包含 app_type 的模式
        content = re.sub(
            r'Config \{\s*app_id: "([^"]+)"\.to_string\(\),\s*app_secret: "([^"]+)"\.to_string\(\),\s*app_type: AppType::([^,\s]+),\s*\.\.Default::default\(\)\s*\}',
            r'Config::builder().app_id("\1").app_secret("\2").app_type(AppType::\3).build()',
            content,
            flags=re.MULTILINE | re.DOTALL
        )

        # 只有 app_id 的模式
        content = re.sub(
            r'Config \{\s*app_id: "([^"]+)"\.to_string\(\),\s*\.\.Default::default\(\)\s*\}',
            r'Config::builder().app_id("\1").build()',
            content,
            flags=re.MULTILINE | re.DOTALL
        )

        if content != original_content:
            with open(filepath, 'w') as f:
                f.write(content)
            fixed_files += 1
            print(f'Fixed: {filepath}')
    except Exception as e:
        print(f'Error processing {filepath}: {e}')

print(f'Fixed {fixed_files} files automatically')