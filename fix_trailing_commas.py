#!/usr/bin/env python3
"""
修复 Rust 文件中多余的尾随逗号
特别是在错误处理代码中的 IllegalParamError 调用
"""

import os
import re
from pathlib import Path

def fix_trailing_commas(file_path):
    """修复文件中的尾随逗号"""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
    except Exception as e:
        print(f"Error reading {file_path}: {e}")
        return False

    # 修复模式：移除 IllegalParamError 最后一行的逗号
    pattern = r'(IllegalParamError\(\s*"[^"]+"\s*\.\s*to_string\(\)\s*),\s*(\)\s*\))'
    new_content = re.sub(pattern, r'\1\2', content, flags=re.MULTILINE | re.DOTALL)

    # 如果内容有变化，写入文件
    if new_content != content:
        try:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(new_content)
            return True
        except Exception as e:
            print(f"Error writing {file_path}: {e}")
            return False

    return False

# 查找所有需要修复的文件
base_dir = Path("crates/openlark-docs/src")
files_to_fix = []

for root, dirs, files in os.walk(base_dir):
    for file in files:
        if file.endswith('.rs'):
            file_path = os.path.join(root, file)
            # 检查文件是否包含错误模式
            try:
                with open(file_path, 'r', encoding='utf-8') as f:
                    content = f.read()
                    if 'IllegalParamError(' in content and '),\n' in content:
                        files_to_fix.append(file_path)
            except:
                pass

print(f"Found {len(files_to_fix)} files to fix")

# 修复文件
fixed_count = 0
for file_path in files_to_fix:
    if fix_trailing_commas(file_path):
        print(f"Fixed: {file_path}")
        fixed_count += 1

print(f"\nTotal files fixed: {fixed_count}")