#!/usr/bin/env python3
"""
修复 Rust 文件中 inner attribute 位置的脚本
将 inner attributes (#![...]) 移动到文件开头，在任何文档注释之前
"""

import re
import os
import sys
from pathlib import Path

def fix_file_attributes(file_path):
    """修复单个文件中的 inner attribute 位置"""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
    except Exception as e:
        print(f"Error reading {file_path}: {e}")
        return False

    # 保存原始内容
    original_content = content

    # 提取所有的 inner attributes
    inner_attrs_pattern = r'^#!\[[^\]]*\]\s*\n?'
    inner_attrs = []

    # 按行分割内容
    lines = content.split('\n')
    new_lines = []
    attrs_to_move = []
    doc_comments = []
    other_lines = []

    # 第一阶段：提取和分类
    i = 0
    while i < len(lines):
        line = lines[i]

        # 如果是 inner attribute
        if re.match(r'^#!\[[^\]]*\]', line):
            # 检查前面是否有文档注释
            if new_lines and any(re.match(r'^///', l) for l in new_lines[-3:]):
                # 前面有文档注释，需要移动
                attrs_to_move.append(line)
            else:
                # 位置正确，保留
                other_lines.append(line)
        else:
            # 不是 inner attribute
            other_lines.append(line)
        i += 1

    # 如果有需要移动的 attributes，重新组织文件
    if attrs_to_move:
        # 找到第一个非注释、非空行的位置
        first_code_line_idx = 0
        for i, line in enumerate(other_lines):
            stripped = line.strip()
            if stripped and not stripped.startswith('///') and not stripped.startswith('//'):
                first_code_line_idx = i
                break

        # 重新组合内容
        final_lines = []

        # 添加需要移动的 inner attributes
        for attr in attrs_to_move:
            final_lines.append(attr)

        # 如果原来没有空的 attributes 行，添加一个空行分隔
        if final_lines and other_lines and other_lines[0].strip() != '':
            final_lines.append('')

        # 添加其他所有行
        for line in other_lines:
            final_lines.append(line)

        # 写入新内容
        new_content = '\n'.join(final_lines)

        # 如果内容有变化，写入文件
        if new_content != original_content:
            try:
                with open(file_path, 'w', encoding='utf-8') as f:
                    f.write(new_content)
                print(f"Fixed: {file_path}")
                return True
            except Exception as e:
                print(f"Error writing {file_path}: {e}")
                return False

    return False

def main():
    """主函数"""
    # 获取所有需要修复的文件
    base_dir = Path("crates/openlark-docs/src")

    # 使用 find 命令获取文件列表
    import subprocess
    result = subprocess.run(
        ["find", str(base_dir), "-name", "*.rs", "-exec", "grep", "-l", "^#!\\[", "{}", ";"],
        capture_output=True,
        text=True
    )

    if result.returncode != 0:
        print("Error finding files")
        sys.exit(1)

    files = result.stdout.strip().split('\n')
    files = [f for f in files if f]  # 过滤空字符串

    print(f"Found {len(files)} files with inner attributes")

    fixed_count = 0
    for file_path in files:
        if fix_file_attributes(file_path):
            fixed_count += 1

    print(f"\nFixed {fixed_count} files")

if __name__ == "__main__":
    main()