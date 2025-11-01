#!/usr/bin/env python3

import os
import re
import sys

def fix_nested_core(file_path):
    """修复文件中嵌套的core导入问题"""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()

        original_content = content

        # 修复模式: open_lark_core::core::{ 后面跟着 core::{
        pattern1 = r'(use open_lark_core::core::\{)\s*\n\s+core::\{'
        replacement1 = r'\1'
        content = re.sub(pattern1, replacement1, content, flags=re.MULTILINE)

        # 移除嵌套的core块结束的大括号
        # 匹配:        }, 后面跟着其他imports或use语句结束
        pattern2 = r'(\s{4,}\},\s*\n)(\s{0,3}(?:use|pub|#\[cfg\]|}))'
        replacement2 = lambda m: m.group(2) if 'SDKResult,' in content[:content.find(m.group(1))] else m.group(1) + m.group(2)
        content = re.sub(pattern2, replacement2, content, flags=re.MULTILINE)

        # 如果有变化，写回文件
        if content != original_content:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(content)
            print(f"Fixed: {file_path}")
            return True
        else:
            print(f"No changes needed: {file_path}")
            return False

    except Exception as e:
        print(f"Error processing {file_path}: {e}")
        return False

def main():
    # 查找所有需要修复的文件
    hr_suite_dir = "crates/open-lark-hr-suite/src"

    fixed_count = 0
    total_count = 0

    for root, dirs, files in os.walk(hr_suite_dir):
        for file in files:
            if file.endswith('.rs'):
                file_path = os.path.join(root, file)
                total_count += 1
                if fix_nested_core(file_path):
                    fixed_count += 1

    print(f"\n修复完成: {fixed_count}/{total_count} 个文件被修改")

if __name__ == "__main__":
    main()