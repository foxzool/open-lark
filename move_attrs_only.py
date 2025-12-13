#!/usr/bin/env python3
"""
只移动 inner attributes 的位置，不做其他修改
"""

import os
import re

def fix_file(file_path):
    """只移动 inner attributes，不做其他修改"""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
    except:
        return False

    # 分割成行
    lines = content.split('\n')

    # 找到所有文档注释和 inner attributes
    doc_comments = []
    inner_attrs = []
    other_lines = []

    # 标记是否已经开始遇到非文档注释行
    seen_non_doc = False

    for line in lines:
        # 如果是 inner attribute
        if re.match(r'^#!\[[^\]]*\]', line):
            inner_attrs.append(line)
        # 如果是文档注释
        elif line.strip().startswith('///'):
            # 如果已经看到了非文档行，这是一个新的文档块
            if seen_non_doc:
                # 将之前的 other_lines 保存，开始新的收集
                doc_comments.append(line)
            else:
                doc_comments.append(line)
        else:
            # 非文档注释行
            if line.strip():  # 非空行
                seen_non_doc = True
            other_lines.append(line)

    # 如果有 inner attributes 且有文档注释在它们前面，需要重新排序
    if inner_attrs and doc_comments and other_lines:
        # 新的文件结构
        new_lines = []

        # 首先添加所有的 inner attributes
        for attr in inner_attrs:
            new_lines.append(attr)

        # 添加一个空行（如果需要）
        if doc_comments:
            new_lines.append('')

        # 然后添加所有其他内容
        for line in doc_comments + other_lines:
            new_lines.append(line)

        new_content = '\n'.join(new_lines)

        # 写回文件
        if new_content != content:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(new_content)
            return True

    return False

# 只修复特定的文件
files_to_fix = [
    "crates/openlark-docs/src/lib.rs",
    "crates/openlark-docs/src/ccm/docs/v1/mod.rs"
]

for file_path in files_to_fix:
    if os.path.exists(file_path):
        if fix_file(file_path):
            print(f"Fixed: {file_path}")