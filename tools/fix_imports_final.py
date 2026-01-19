#!/usr/bin/env python3
"""最终版本：修复所有导入问题"""

import re
from pathlib import Path

def fix_file_imports(filepath: str) -> bool:
    """修复单个文件的导入"""
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            lines = f.readlines()

        # 查找并修复导入语句
        new_lines = []
        i = 0
        modified = False

        while i < len(lines):
            line = lines[i]

            # 检查是否是问题导入的开始
            if 'use openlark_core::' in line and 'validate_required' in ''.join(lines[i:min(i+20, len(lines))]):
                # 跳过整个导入块
                while i < len(lines) and not lines[i].strip().endswith('};'):
                    i += 1
                if i < len(lines):
                    i += 1  # 跳过 };

                # 写入新的导入
                new_lines.append('use openlark_core::{\n')
                new_lines.append('    api::{ApiRequest, ApiResponseTrait, ResponseFormat},\n')
                new_lines.append('    config::Config,\n')
                new_lines.append('    error::SDKResult,\n')
                new_lines.append('    http::Transport,\n')
                new_lines.append('    validate_required,\n')
                new_lines.append('};\n')
                modified = True
            else:
                new_lines.append(line)
                i += 1

        if modified:
            with open(filepath, 'w', encoding='utf-8') as f:
                f.writelines(new_lines)
            return True

        return False

    except Exception as e:
        print(f"  ✗ 错误: {e}")
        return False

def main():
    """主函数"""
    base_path = Path("/Users/zool/RustroverProjects/open-lark/crates/openlark-docs/src")

    # 需要修复的文件列表
    files_to_fix = [
        "base/bitable/v1/app/role/delete.rs",
        "base/bitable/v1/app/role/list.rs",
        "base/bitable/v1/app/role/update.rs",
        "base/bitable/v1/app/role/member/batch_create.rs",
        "base/bitable/v1/app/role/member/batch_delete.rs",
        "base/bitable/v1/app/role/member/create.rs",
        "base/bitable/v1/app/role/member/delete.rs",
        "base/bitable/v1/app/role/member/list.rs",
        "base/bitable/v1/app/table/create.rs",
        "base/bitable/v1/app/table/delete.rs",
        "base/bitable/v1/app/table/list.rs",
        "base/bitable/v1/app/table/patch.rs",
        "base/bitable/v1/app/table/field/create.rs",
        "base/bitable/v1/app/table/field/delete.rs",
        "base/bitable/v1/app/table/field/list.rs",
        "base/bitable/v1/app/table/field/update.rs",
        "base/bitable/v1/app/table/record/batch_create.rs",
        "base/bitable/v1/app/table/record/batch_delete.rs",
        "base/bitable/v1/app/table/record/batch_get.rs",
        "base/bitable/v1/app/table/record/batch_update.rs",
        "base/bitable/v1/app/table/record/get.rs",
        "base/bitable/v1/app/table/view/create.rs",
        "base/bitable/v1/app/table/view/delete.rs",
        "base/bitable/v1/app/table/view/get.rs",
        "base/bitable/v1/app/table/view/list.rs",
        "base/bitable/v1/app/table/view/patch.rs",
        "base/bitable/v1/app/table/form/field/list.rs",
        "base/bitable/v1/app/table/form/field/patch.rs",
        "base/bitable/v1/app/table/form/get.rs",
        "base/bitable/v1/app/table/form/patch.rs",
    ]

    print("=" * 80)
    print("最终修复：所有导入问题")
    print("=" * 80)
    print()

    fixed_count = 0
    for rel_path in files_to_fix:
        filepath = base_path / rel_path
        if filepath.exists():
            print(f"处理: {rel_path}")
            if fix_file_imports(str(filepath)):
                print(f"  ✓ 已修复")
                fixed_count += 1
            else:
                print(f"  - 跳过")
        else:
            print(f"文件不存在: {filepath}")

    print()
    print(f"修复完成: {fixed_count}/{len(files_to_fix)} 个文件")

if __name__ == '__main__':
    main()
