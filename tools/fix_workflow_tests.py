#!/usr/bin/env python3
"""
修复 workflow 模块中测试模块的未使用 super::* 导入
"""

import os
import re

def fix_test_module(file_path):
    """修复测试模块中的 super::* 导入"""
    with open(file_path, 'r') as f:
        content = f.read()

    original = content

    # 查找 #[cfg(test)] mod tests { use super::*; ... }
    pattern = re.compile(
        r'(\#\[cfg\(test\)\]\s*mod tests\s*\{\s*)use super::\*;(\s*)',
        re.MULTILINE
    )

    # 检查是否有 API endpoint 的导入需求
    if 'crate::common::api_endpoints' in content:
        # 替换为具体的导入
        content = pattern.sub(r'\1use crate::common::api_endpoints::TaskApiV1;\2', content)
    else:
        # 移除整个 use super::*; 行
        content = pattern.sub(r'\1\2', content)

    if content != original:
        with open(file_path, 'w') as f:
            f.write(content)
        return True
    return False

def main():
    base_dir = "/Users/zool/RustroverProjects/open-lark/crates/openlark-workflow/src"
    fixed_count = 0

    for root, dirs, files in os.walk(base_dir):
        for file in files:
            if file.endswith('.rs'):
                file_path = os.path.join(root, file)
                try:
                    if fix_test_module(file_path):
                        print(f"Fixed: {file_path}")
                        fixed_count += 1
                except Exception as e:
                    print(f"Error processing {file_path}: {e}")

    print(f"\nTotal files fixed: {fixed_count}")

if __name__ == "__main__":
    main()
