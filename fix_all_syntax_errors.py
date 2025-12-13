#!/usr/bin/env python3
"""
修复所有 inner attribute 移动后产生的语法错误
"""

import os
import re
from pathlib import Path

def fix_file_syntax(file_path):
    """修复文件的语法错误"""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
    except:
        return False

    original_content = content

    # 修复1: IllegalParamError 后多余的逗号
    content = re.sub(
        r'(IllegalParamError\(\s*"[^"]+"\s*\.\s*to_string\(\)\s*),\s*(\))',
        r'\1\2',
        content,
        flags=re.MULTILINE
    )

    # 修复2: 修复错误的导入语句（合并两个import）
    content = re.sub(
        r'use openlark_core::\{\s*e::api::Response\s*\};\s*\n\s*api::\{ApiRequest, ApiResponseTrait, ResponseFormat\}',
        'use openlark_core::{\n    api::{ApiRequest, ApiResponseTrait, ResponseFormat}',
        content
    )

    # 修复3: 修复多个连续的空行
    content = re.sub(r'\n\n\n+', '\n\n', content)

    if content != original_content:
        try:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(content)
            return True
        except:
            return False

    return False

def main():
    """主函数"""
    base_dir = Path("crates/openlark-docs/src")
    fixed_count = 0

    # 遍历所有 .rs 文件
    for root, dirs, files in os.walk(base_dir):
        for file in files:
            if file.endswith('.rs'):
                file_path = os.path.join(root, file)
                if fix_file_syntax(file_path):
                    print(f"Fixed: {file_path}")
                    fixed_count += 1

    print(f"\nTotal files fixed: {fixed_count}")

if __name__ == "__main__":
    main()