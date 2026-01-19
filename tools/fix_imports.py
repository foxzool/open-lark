#!/usr/bin/env python3
"""修复导入语句格式错误"""

import re
from pathlib import Path

def fix_imports_in_file(filepath: str) -> bool:
    """修复单个文件的导入语句"""
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            content = f.read()

        original_content = content

        # 修复模式 1: 逗号在错误位置
        # use openlark_core::{..., , validate_required};
        pattern1 = r'use openlark_core::\s*\{([^}]+),\s*,\s*validate_required\s*\}'
        replacement1 = r'use openlark_core::{\1, validate_required}'
        content = re.sub(pattern1, replacement1, content, flags=re.MULTILINE | re.DOTALL)

        # 修复模式 2: validate_required 在错误的模块中
        # use openlark_core::{api::{..., validate_required}, ...};
        pattern2 = r'use openlark_core::\s*\{\s*api::\{([^}]+),\s*validate_required\s*\}([^}]*)\}'
        replacement2 = r'use openlark_core::{\1, validate_required}'
        content = re.sub(pattern2, replacement2, content, flags=re.MULTILINE | re.DOTALL)

        # 修复模式 3: 更复杂的情况，需要完全重写导入
        # use openlark_core::{
        #     api::ApiRequest,
        #     ...
        #     ,
        #     validate_required
        # };
        pattern3 = r'use openlark_core::\s*\{([^}]+)\}\s*;\s*use\s+openlark_core::\s*\{\s*validate_required\s*\};'
        replacement3 = r'use openlark_core::{\1, validate_required};'
        content = re.sub(pattern3, replacement3, content, flags=re.MULTILINE | re.DOTALL)

        if content != original_content:
            with open(filepath, 'w', encoding='utf-8') as f:
                f.write(content)
            return True

        return False

    except Exception as e:
        print(f"  ✗ 错误: {e}")
        return False


def main():
    """主函数"""
    base_path = Path("/Users/zool/RustroverProjects/open-lark")

    # 需要修复的文件列表
    files_to_fix = [
        "crates/openlark-docs/src/ccm/drive/v1/file/comment/create.rs",
        "crates/openlark-docs/src/ccm/drive/v1/file/comment/batch_query.rs",
        "crates/openlark-docs/src/ccm/drive/v1/file/comment/reply/delete.rs",
        "crates/openlark-docs/src/ccm/drive/v1/file/comment/reply/list.rs",
        "crates/openlark-docs/src/ccm/drive/v1/file/comment/reply/update.rs",
        "crates/openlark-docs/src/ccm/drive/v1/file/subscription/create.rs",
        "crates/openlark-docs/src/base/bitable/v1/app/role/create.rs",
    ]

    print("=" * 80)
    print("修复导入语句格式错误")
    print("=" * 80)
    print()

    fixed_count = 0
    for rel_path in files_to_fix:
        filepath = base_path / rel_path
        if filepath.exists():
            print(f"处理: {rel_path}")
            if fix_imports_in_file(str(filepath)):
                print(f"  ✓ 已修复")
                fixed_count += 1
            else:
                print(f"  - 无需修复")
        else:
            print(f"文件不存在: {filepath}")

    print()
    print(f"修复完成: {fixed_count} 个文件")


if __name__ == '__main__':
    main()
