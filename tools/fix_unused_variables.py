#!/usr/bin/env python3
"""
修复 openlark-platform 中 execute() 方法的未使用变量问题
"""

import os
import re

def fix_execute_method(file_path):
    """修复 execute() 方法中的未使用变量问题"""
    with open(file_path, 'r') as f:
        content = f.read()

    original_content = content

    # 模式1: execute() 方法开头有 url/request 变量定义
    # 但这些变量在调用 execute_with_options 后未被使用
    # 需要移除这些变量定义

    # 查找 execute() 方法中的未使用变量
    # 模式: let url = "..."; 和 let request = XXX {};

    lines = content.split('\n')
    new_lines = []
    i = 0
    in_execute = False
    execute_start_line = -1
    url_pattern = re.compile(r'^\s+let url = ["\'].*["\'];?\s*$')
    request_pattern = re.compile(r'^\s+let request = \w+\s*\{[^}]*\};?\s*$')

    while i < len(lines):
        line = lines[i]

        if 'pub async fn execute(self)' in line and 'execute_with_options' not in line:
            in_execute = True
            execute_start_line = i

        if in_execute:
            # 检查是否到达 execute_with_options 调用
            if 'self.execute_with_options(RequestOption::default())' in line or \
               'self.execute_with_options(openlark_core::req_option::RequestOption::default())' in line:
                in_execute = False
                # 往前查找未使用的 url 和 request 变量并移除
                j = i - 1
                removed_count = 0
                while j >= 0 and removed_count < 4:
                    prev_line = lines[j]
                    if url_pattern.match(prev_line) or request_pattern.match(prev_line):
                        # 移除这行
                        lines[j] = ''
                        removed_count += 1
                    elif prev_line.strip() == '' and removed_count > 0:
                        # 移除空行
                        lines[j] = ''
                    elif prev_line.strip() and not url_pattern.match(prev_line) and not request_pattern.match(prev_line):
                        break
                    j -= 1
                new_lines.append(line)
            else:
                new_lines.append(line)
        else:
            new_lines.append(line)

        i += 1

    new_content = '\n'.join(line for line in new_lines if line is not None)

    if new_content != original_content:
        with open(file_path, 'w') as f:
            f.write(new_content)
        return True
    return False

def main():
    """遍历所有文件并修复"""
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
