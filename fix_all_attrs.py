#!/usr/bin/env python3
"""
修复 openlark-docs crate 中所有文件的 inner attribute 位置问题
"""

import os
import re
import subprocess
from pathlib import Path

def find_files_with_inner_attrs():
    """查找所有包含 inner attributes 的文件"""
    base_dir = Path("crates/openlark-docs/src")
    files = []

    # 使用 find 命令查找所有 .rs 文件
    result = subprocess.run(
        ["find", str(base_dir), "-name", "*.rs", "-type", "f"],
        capture_output=True,
        text=True
    )

    if result.returncode == 0:
        files = result.stdout.strip().split('\n')
        files = [f for f in files if f]

    return files

def fix_rust_file(file_path):
    """修复单个 Rust 文件的 inner attribute 位置"""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            lines = f.readlines()
    except Exception as e:
        print(f"Error reading {file_path}: {e}")
        return False

    # 分类行
    inner_attrs = []
    other_lines = []
    has_doc_comment = False

    # 检查文件开头是否有文档注释
    for line in lines:
        stripped = line.strip()
        if stripped.startswith('///'):
            has_doc_comment = True
            break

    # 收集 inner attributes 和其他内容
    for line in lines:
        if re.match(r'^#!\[[^\]]*\]', line):
            inner_attrs.append(line)
        else:
            other_lines.append(line)

    # 如果有 inner attributes，重新组织文件
    if inner_attrs:
        new_lines = []

        # 添加所有 inner attributes 到开头
        for attr in inner_attrs:
            new_lines.append(attr)

        # 如果原来有文档注释，添加空行分隔
        if has_doc_comment and other_lines and other_lines[0].strip() != '':
            new_lines.append('\n')

        # 添加其他所有内容
        for line in other_lines:
            new_lines.append(line)

        # 写回文件
        try:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.writelines(new_lines)
            return True
        except Exception as e:
            print(f"Error writing {file_path}: {e}")
            return False

    return False

def main():
    """主函数"""
    # 查找所有文件
    files = find_files_with_inner_attrs()
    print(f"Found {len(files)} Rust files")

    # 修复每个文件
    fixed_count = 0
    for file_path in files:
        if fix_rust_file(file_path):
            print(f"Fixed: {file_path}")
            fixed_count += 1

    print(f"\nTotal files fixed: {fixed_count}")

if __name__ == "__main__":
    main()