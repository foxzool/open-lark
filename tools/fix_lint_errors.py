#!/usr/bin/env python3
"""
批量清理 lint 错误 - 移除 execute() 方法中未使用的变量
"""

import os
import re

def fix_execute_method(file_path):
    """修复 execute() 方法中的未使用变量"""
    with open(file_path, 'r') as f:
        content = f.read()

    original = content

    # 模式: execute() 方法中有 url 和 request 变量定义，但 execute() 方法只是调用 execute_with_options()
    # 需要移除这些未使用的变量

    # 匹配 execute() 方法体中开头的变量定义，直到调用 execute_with_options
    # 模式: 
    #   pub async fn execute(...) -> SDKResult<...> {
    #       let url = ...;
    #       let request = ...;
    #       ...execute_with_options...

    # 查找 execute() 方法
    execute_pattern = re.compile(
        r'(pub async fn execute\(self\) -> SDKResult<[^>]+>\s*\{\s*)'
        r'(let url\s*=\s*[^;]+;\s*)'
        r'(let request\s*=\s*[^;]+;\s*)?'
        r'(\s*self\.execute_with_options)',
        re.MULTILINE
    )

    # 修复模式：移除 url 和 request 变量
    def fix_execute(match):
        execute_sig = match.group(1)
        url_var = match.group(2)
        request_var = match.group(3) or ''
        execute_call = match.group(4)
        return f"{execute_sig}{execute_call}.await\n    }}"

    content = execute_pattern.sub(fix_execute, content)

    if content != original:
        with open(file_path, 'w') as f:
            f.write(content)
        return True
    return False

def main():
    base_dir = "/Users/zool/RustroverProjects/open-lark/crates/openlark-platform/src"
    fixed_count = 0

    for root, dirs, files in os.walk(base_dir):
        for file in files:
            if file.endswith('.rs'):
                file_path = os.path.join(root, file)
                try:
                    if fix_execute_method(file_path):
                        print(f"Fixed: {file_path}")
                        fixed_count += 1
                except Exception as e:
                    print(f"Error processing {file_path}: {e}")

    print(f"\nTotal files fixed: {fixed_count}")

if __name__ == "__main__":
    main()
