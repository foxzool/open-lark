#!/usr/bin/env python3
"""
批量修复 Rust 文件中 inner attribute 位置的脚本
将所有的 #![...] 移动到文件最开头，在任何文档注释之前
"""

import os
import re
from pathlib import Path

def fix_rust_file(file_path):
    """修复单个 Rust 文件"""
    with open(file_path, 'r', encoding='utf-8') as f:
        content = f.read()

    # 检查是否需要修复
    # 查找文档注释后的 inner attribute
    lines = content.split('\n')

    # 收集所有 inner attributes
    inner_attrs = []
    other_content = []

    # 提取 inner attributes
    for line in lines:
        if re.match(r'^#!\[[^\]]*\]', line):
            inner_attrs.append(line)
        else:
            other_content.append(line)

    # 如果有 inner attributes
    if inner_attrs:
        # 重新构建文件内容
        # 首先放所有的 inner attributes
        new_lines = []

        # 添加 inner attributes
        for attr in inner_attrs:
            new_lines.append(attr)

        # 如果其他内容的第一个非空行不是空行，添加一个空行分隔
        if other_content:
            # 找到第一个非空行
            first_non_empty_idx = 0
            for i, line in enumerate(other_content):
                if line.strip():
                    first_non_empty_idx = i
                    break

            # 如果第一个非空行是文档注释，添加空行
            if other_content[first_non_empty_idx].strip().startswith('///'):
                new_lines.append('')

        # 添加其他内容
        new_lines.extend(other_content)

        new_content = '\n'.join(new_lines)

        # 如果内容有变化，写入文件
        if new_content != content:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(new_content)
            return True

    return False

# 需要修复的文件列表
files_to_fix = [
    "crates/openlark-docs/src/ccm/sheets/v2/batch_read.rs",
    "crates/openlark-docs/src/ccm/sheets/v2/sheet_cells.rs",
    "crates/openlark-docs/src/ccm/sheets/v2/single_read.rs",
    "crates/openlark-docs/src/ccm/sheets/v2/range.rs",
    "crates/openlark-docs/src/ccm/sheets/v2/style.rs",
    "crates/openlark-docs/src/ccm/sheets/v2/spreadsheet.rs",
    "crates/openlark-docs/src/ccm/sheets/v2/batch_write.rs",
    "crates/openlark-docs/src/ccm/sheets/v2/image_write.rs",
    "crates/openlark-docs/src/ccm/sheets/v2/worksheet.rs",
    "crates/openlark-docs/src/ccm/sheets/v3/mod.rs",
    "crates/openlark-docs/src/ccm/wiki/v1/mod.rs",
    "crates/openlark-docs/src/ccm/wiki/v2/space_node/mod.rs",
    "crates/openlark-docs/src/ccm/wiki/v2/space_setting/mod.rs",
    "crates/openlark-docs/src/ccm/wiki/v2/mod.rs",
    "crates/openlark-docs/src/ccm/wiki/v2/task/mod.rs",
    "crates/openlark-docs/src/ccm/wiki/v2/space_member/mod.rs",
    "crates/openlark-docs/src/ccm/wiki/v2/space/mod.rs",
]

# 修复文件
fixed_count = 0
for file_path in files_to_fix:
    if os.path.exists(file_path):
        if fix_rust_file(file_path):
            print(f"Fixed: {file_path}")
            fixed_count += 1
        else:
            print(f"No changes needed: {file_path}")
    else:
        print(f"File not found: {file_path}")

print(f"\nTotal files fixed: {fixed_count}")