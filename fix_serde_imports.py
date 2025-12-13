#!/usr/bin/env python3
"""
批量修复 openlark-docs crate 中的 Serialize/Deserialize 导入问题
"""

import os
import re
import subprocess
import sys

def get_files_need_serde_import():
    """获取需要添加 Serialize/Deserialize 导入的文件列表"""
    result = subprocess.run(
        ["cargo", "check", "-p", "openlark-docs"],
        capture_output=True,
        text=True,
        cwd="/Users/zool/RustroverProjects/open-lark"
    )

    files = set()
    # 查找所有包含 "cannot find derive macro" 的文件
    for line in result.stderr.split('\n'):
        if "cannot find derive macro" in line and "-->" in line:
            # 提取文件路径
            file_path = line.split("-->")[1].split(":")[0].strip()
            if file_path.endswith(".rs"):
                files.add(file_path)

    return sorted(files)

def fix_serde_import(file_path):
    """修复单个文件的 Serialize/Deserialize 导入"""
    with open(file_path, 'r', encoding='utf-8') as f:
        content = f.read()

    # 检查是否已经有 serde 导入
    if re.search(r'use\s+serde\s*::\s*[{#]', content):
        return False  # 已经有导入，不需要修复

    # 检查是否使用了 prelude
    if 'use crate::prelude' in content or 'use super::prelude' in content:
        return False  # 使用了 prelude，应该已经有

    # 查找插入位置（在最后一个 use 语句之后）
    use_lines = re.findall(r'^\s*use\s+.*?;', content, re.MULTILINE)

    if use_lines:
        # 找到最后一个 use 语句的行号
        last_use = max(m.start() for m in re.finditer(r'^\s*use\s+.*?;', content, re.MULTILINE))
        # 找到该行的结束位置
        line_end = content.find('\n', last_use) + 1

        # 插入新的导入
        new_import = "\n// 导入序列化支持\nuse serde::{Deserialize, Serialize};\n"
        content = content[:line_end] + new_import + content[line_end:]
    else:
        # 如果没有 use 语句，在第一个注释块后添加
        # 尝试在文件开头添加
        lines = content.split('\n')
        insert_idx = 0

        # 跳过注释和空行
        for i, line in enumerate(lines):
            if line.strip() and not line.strip().startswith('//') and not line.strip().startswith("///"):
                insert_idx = i
                break

        lines.insert(insert_idx, "// 导入序列化支持")
        lines.insert(insert_idx + 1, "use serde::{Deserialize, Serialize};")
        lines.insert(insert_idx + 2, "")
        content = '\n'.join(lines)

    # 写回文件
    with open(file_path, 'w', encoding='utf-8') as f:
        f.write(content)

    return True

def main():
    print("开始批量修复 Serialize/Deserialize 导入问题...")

    # 获取需要修复的文件
    files = get_files_need_serde_import()

    if not files:
        print("没有找到需要修复的文件")
        return

    print(f"找到 {len(files)} 个需要修复的文件")

    # 修复每个文件
    fixed_count = 0
    for file_path in files:
        print(f"  修复: {file_path}")
        if fix_serde_import(file_path):
            fixed_count += 1

    print(f"\n成功修复了 {fixed_count} 个文件")

if __name__ == "__main__":
    main()