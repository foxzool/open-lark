#!/usr/bin/env python3
import re
import os

def fix_syntax_errors(content):
    """Fix syntax errors caused by the previous script"""

    # 修复函数签名问题
    content = re.sub(
        r'fn create_test_config\(\) -> Config::default\(\)',
        r'fn create_test_config() -> Config {\n        Config::default()\n    }',
        content
    )

    # 修复结构体定义问题
    content = re.sub(
        r'pub struct (\w+)Config::default\(\)',
        r'pub struct \1Config {\n    // TODO: Add fields\n}\n\nimpl Default for \1Config {\n    fn default() -> Self {\n        Self {\n            // TODO: Set default values\n        }\n    }\n}',
        content
    )

    # 修复 impl 声明问题
    content = re.sub(
        r'impl Default for (\w+)Config::default\(\)',
        r'impl Default for \1Config {',
        content
    )

    content = re.sub(
        r'impl (\w+)Config::default\(\)',
        r'impl \1Config {',
        content
    )

    return content

# 找到所有需要修复的文件
files_to_fix = []
for root, dirs, files in os.walk('src/service'):
    for file in files:
        if file.endswith('.rs'):
            filepath = os.path.join(root, file)
            try:
                with open(filepath, 'r') as f:
                    content = f.read()
                    if 'Config::default()' in content and ('fn create_test_config()' in content or 'pub struct' in content):
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
        content = fix_syntax_errors(content)

        if content != original_content:
            with open(filepath, 'w') as f:
                f.write(content)
            fixed_files += 1
            print(f'Fixed: {filepath}')
    except Exception as e:
        print(f'Error processing {filepath}: {e}')

print(f'Fixed {fixed_files} files automatically')