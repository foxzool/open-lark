#!/usr/bin/env python3
"""批量修复所有导入语句"""

import re
from pathlib import Path
from typing import List

def fix_import_statement(content: str) -> str:
    """修复导入语句"""
    # 模式 1: validate_required 在 api::{...} 内
    pattern1 = r'use openlark_core::\s*\{\s*api::\{([^}]+),\s*validate_required\s*\}([^}]*)\}'
    def replace1(match):
        api_content = match.group(1).strip()
        other_content = match.group(2).strip()
        # 移除 validation_error 从 error 中
        other_content = re.sub(r',\s*validation_error', '', other_content)
        other_content = re.sub(r'error::\{[^}]*validation_error[^}]*\}', 'error::SDKResult', other_content)

        parts = [f'api::{{{api_content}}}', 'validate_required']
        if other_content:
            parts.append(other_content)

        return f'use openlark_core::{{\n    {",\n    ".join(parts)}\n}}'

    content = re.sub(pattern1, replace1, content, flags=re.MULTILINE | re.DOTALL)

    return content

def process_file(filepath: str) -> bool:
    """处理单个文件"""
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            content = f.read()

        # 检查是否需要修复
        if 'api::' not in content or 'validate_required' not in content:
            return False

        original_content = content
        content = fix_import_statement(content)

        if content != original_content:
            with open(filepath, 'w', encoding='utf-8') as f:
                f.write(content)
            return True

        return False

    except Exception as e:
        print(f"  ✗ 错误: {e}")
        return False

def find_files_with_issues(base_path: Path) -> List[str]:
    """查找所有需要修复的文件"""
    issues = []
    for rs_file in base_path.rglob("*.rs"):
        try:
            with open(rs_file, 'r', encoding='utf-8') as f:
                content = f.read()
                # 检查是否有错误的导入
                if 'api::' in content and 'validate_required' in content:
                    if re.search(r'api::\{[^}]*validate_required', content):
                        issues.append(str(rs_file))
        except:
            pass
    return issues

def main():
    """主函数"""
    base_path = Path("/Users/zool/RustroverProjects/open-lark/crates/openlark-docs/src")

    print("=" * 80)
    print("批量修复导入语句")
    print("=" * 80)
    print()

    # 查找所有需要修复的文件
    files_to_fix = find_files_with_issues(base_path)

    if not files_to_fix:
        print("没有需要修复的文件")
        return

    print(f"找到 {len(files_to_fix)} 个需要修复的文件")
    print()

    fixed_count = 0
    for filepath in files_to_fix:
        rel_path = Path(filepath).relative_to("/Users/zool/RustroverProjects/open-lark")
        print(f"处理: {rel_path}")
        if process_file(filepath):
            print(f"  ✓ 已修复")
            fixed_count += 1
        else:
            print(f"  - 跳过")

    print()
    print(f"修复完成: {fixed_count}/{len(files_to_fix)} 个文件")

if __name__ == '__main__':
    main()
