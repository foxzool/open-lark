#!/usr/bin/env python3
"""批量修复 openlark-docs 中的验证规范问题"""

import re
from pathlib import Path
from typing import List, Tuple

# 需要处理的文件列表
FILES_TO_PROCESS = [
    # Bitable record 模块
    "crates/openlark-docs/src/base/bitable/v1/app/table/record/get.rs",
    "crates/openlark-docs/src/base/bitable/v1/app/table/record/batch_get.rs",
    "crates/openlark-docs/src/base/bitable/v1/app/table/record/batch_create.rs",
    "crates/openlark-docs/src/base/bitable/v1/app/table/record/batch_update.rs",
    "crates/openlark-docs/src/base/bitable/v1/app/table/record/batch_delete.rs",
    # Bitable field 模块
    "crates/openlark-docs/src/base/bitable/v1/app/table/field/update.rs",
    "crates/openlark-docs/src/base/bitable/v1/app/table/field/list.rs",
    "crates/openlark-docs/src/base/bitable/v1/app/table/field/delete.rs",
    "crates/openlark-docs/src/base/bitable/v1/app/table/field/create.rs",
    # Bitable table 模块
    "crates/openlark-docs/src/base/bitable/v1/app/table/patch.rs",
    "crates/openlark-docs/src/base/bitable/v1/app/table/create.rs",
    "crates/openlark-docs/src/base/bitable/v1/app/table/list.rs",
    "crates/openlark-docs/src/base/bitable/v1/app/table/delete.rs",
    # Bitable view 模块
    "crates/openlark-docs/src/base/bitable/v1/app/table/view/patch.rs",
    "crates/openlark-docs/src/base/bitable/v1/app/table/view/list.rs",
    "crates/openlark-docs/src/base/bitable/v1/app/table/view/get.rs",
    "crates/openlark-docs/src/base/bitable/v1/app/table/view/delete.rs",
    "crates/openlark-docs/src/base/bitable/v1/app/table/view/create.rs",
    # Bitable form 模块
    "crates/openlark-docs/src/base/bitable/v1/app/table/form/patch.rs",
    "crates/openlark-docs/src/base/bitable/v1/app/table/form/get.rs",
    "crates/openlark-docs/src/base/bitable/v1/app/table/form/field/patch.rs",
    "crates/openlark-docs/src/base/bitable/v1/app/table/form/field/list.rs",
    # Bitable role 模块
    "crates/openlark-docs/src/base/bitable/v1/app/role/update.rs",
    "crates/openlark-docs/src/base/bitable/v1/app/role/list.rs",
    "crates/openlark-docs/src/base/bitable/v1/app/role/delete.rs",
    "crates/openlark-docs/src/base/bitable/v1/app/role/create.rs",
    "crates/openlark-docs/src/base/bitable/v1/app/role/member/list.rs",
    "crates/openlark-docs/src/base/bitable/v1/app/role/member/delete.rs",
    "crates/openlark-docs/src/base/bitable/v1/app/role/member/create.rs",
    "crates/openlark-docs/src/base/bitable/v1/app/role/member/batch_delete.rs",
    "crates/openlark-docs/src/base/bitable/v1/app/role/member/batch_create.rs",
    # Drive 模块
    "crates/openlark-docs/src/ccm/drive/v1/file/subscription/patch.rs",
    "crates/openlark-docs/src/ccm/drive/v1/file/subscription/create.rs",
    "crates/openlark-docs/src/ccm/drive/v1/file/comment/reply/update.rs",
    "crates/openlark-docs/src/ccm/drive/v1/file/comment/reply/list.rs",
    "crates/openlark-docs/src/ccm/drive/v1/file/comment/reply/delete.rs",
    "crates/openlark-docs/src/ccm/drive/v1/file/comment/create.rs",
    "crates/openlark-docs/src/ccm/drive/v1/file/comment/batch_query.rs",
]


def fix_validation_imports(content: str) -> Tuple[str, int]:
    """修复导入语句，移除 validation_error，添加 validate_required"""
    count = 0

    # 查找 openlark_core 的 use 语句
    pattern = r'use openlark_core::\s*\{([^}]+)\}'
    matches = list(re.finditer(pattern, content, re.MULTILINE | re.DOTALL))

    if not matches:
        return content, 0

    for match in matches:
        original_imports = match.group(1)
        imports = [item.strip() for item in original_imports.split(',')]

        modified = False
        new_imports = []

        for imp in imports:
            # 移除 validation_error，但保留其他错误类型
            if imp == 'validation_error':
                modified = True
                continue
            new_imports.append(imp)

        # 添加 validate_required
        if 'validate_required' not in new_imports:
            new_imports.append('validate_required')
            modified = True

        if modified:
            count += 1
            new_import_str = ',\n    '.join(new_imports)
            replacement = f'use openlark_core::\n    {{\n    {new_import_str}\n}}'
            content = content[:match.start()] + replacement + content[match.end():]

    return content, count


def fix_validation_calls(content: str) -> Tuple[str, int]:
    """修复验证调用，将手写的 is_empty 检查替换为 validate_required! 宏"""
    count = 0

    # 匹配模式 1: if self.field.trim().is_empty() { return Err(validation_error(...)); }
    pattern1 = r'if self\.(\w+)\.trim\(\)\.is_empty\(\)\s*\{\s*return Err\(validation_error\([^)]+\)\);\s*\}'

    def replace1(match):
        nonlocal count
        count += 1
        field_name = match.group(1)
        # 提取错误消息
        full_match = match.group(0)
        msg_match = re.search(r'"([^"]+)"', full_match)
        message = msg_match.group(1) if msg_match else f"{field_name}不能为空"
        return f'validate_required!(self.{field_name}.trim(), "{message}");'

    content = re.sub(pattern1, replace1, content, flags=re.MULTILINE | re.DOTALL)

    # 匹配模式 2: if self.field.is_empty() { return Err(validation_error(...)); }
    pattern2 = r'if self\.(\w+)\.is_empty\(\)\s*\{\s*return Err\(validation_error\([^)]+\)\);\s*\}'

    def replace2(match):
        nonlocal count
        count += 1
        field_name = match.group(1)
        # 提取错误消息
        full_match = match.group(0)
        msg_match = re.search(r'"([^"]+)"', full_match)
        message = msg_match.group(1) if msg_match else f"{field_name}不能为空"
        return f'validate_required!(self.{field_name}, "{message}");'

    content = re.sub(pattern2, replace2, content, flags=re.MULTILINE | re.DOTALL)

    return content, count


def fix_validation_error_calls(content: str) -> Tuple[str, int]:
    """修复剩余的 validation_error 调用，使用完整路径"""
    count = 0

    # 替换剩余的 validation_error 调用为 openlark_core::error::validation_error
    pattern = r'\bvalidation_error\('
    replacement = 'openlark_core::error::validation_error('

    new_content, n = re.subn(pattern, replacement, content)
    count += n

    return new_content, count


def remove_builder_struct(content: str, filename: str) -> Tuple[str, int]:
    """删除冗余的 Builder 结构"""
    # 提取主结构名称
    struct_match = re.search(r'pub struct (\w+Request)\s*\{', content)
    if not struct_match:
        return content, 0

    main_struct_name = struct_match.group(1)
    builder_name = f"{main_struct_name}Builder"

    # 查找 Builder 结构的开始和结束
    builder_pattern = rf'/// .*Builder\s*pub struct {builder_name}\s*{{.*?^}}'

    def remove_builder(match):
        return ''  # 删除整个 Builder 结构

    new_content, count = re.subn(builder_pattern, remove_builder, content, flags=re.MULTILINE | re.DOTALL)

    # 查找 Builder impl 块
    impl_pattern = rf'impl {builder_name}\s*{{.*?^}}'

    def remove_impl(match):
        return ''  # 删除整个 impl 块

    new_content, impl_count = re.subn(impl_pattern, remove_impl, new_content, flags=re.MULTILINE | re.DOTALL)

    return new_content, count + impl_count


def process_file(filepath: str) -> dict:
    """处理单个文件"""
    print(f"处理文件: {filepath}")

    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            original_content = f.read()

        content = original_content
        changes = {
            'filepath': filepath,
            'import_fixes': 0,
            'validation_fixes': 0,
            'validation_error_fixes': 0,
            'builder_removed': False,
            'modified': False
        }

        # 1. 修复导入
        content, count = fix_validation_imports(content)
        changes['import_fixes'] = count
        if count > 0:
            changes['modified'] = True

        # 2. 修复验证调用
        content, count = fix_validation_calls(content)
        changes['validation_fixes'] = count
        if count > 0:
            changes['modified'] = True

        # 3. 修复 validation_error 调用
        content, count = fix_validation_error_calls(content)
        changes['validation_error_fixes'] = count
        if count > 0:
            changes['modified'] = True

        # 4. 删除 Builder 结构（可选，需要手动检查）
        # content, count = remove_builder_struct(content, filepath)
        # changes['builder_removed'] = count > 0
        # if count > 0:
        #     changes['modified'] = True

        if changes['modified']:
            with open(filepath, 'w', encoding='utf-8') as f:
                f.write(content)
            print(f"  ✓ 已修改: 导入({changes['import_fixes']}) 验证({changes['validation_fixes']}) error_fix({changes['validation_error_fixes']})")
        else:
            print(f"  - 无需修改")

        return changes

    except Exception as e:
        print(f"  ✗ 错误: {e}")
        return {
            'filepath': filepath,
            'error': str(e),
            'modified': False
        }


def main():
    """主函数"""
    base_path = Path("/Users/zool/RustroverProjects/open-lark")

    print("=" * 80)
    print("批量修复验证规范问题")
    print("=" * 80)
    print()

    results = []
    for rel_path in FILES_TO_PROCESS:
        filepath = base_path / rel_path
        if filepath.exists():
            result = process_file(str(filepath))
            results.append(result)
        else:
            print(f"文件不存在: {filepath}")
            results.append({'filepath': str(filepath), 'error': '文件不存在'})

    print()
    print("=" * 80)
    print("修复总结")
    print("=" * 80)

    modified_count = sum(1 for r in results if r.get('modified', False))
    error_count = sum(1 for r in results if 'error' in r)

    print(f"总文件数: {len(results)}")
    print(f"已修改: {modified_count}")
    print(f"错误: {error_count}")

    total_import_fixes = sum(r.get('import_fixes', 0) for r in results)
    total_validation_fixes = sum(r.get('validation_fixes', 0) for r in results)
    total_error_fixes = sum(r.get('validation_error_fixes', 0) for r in results)

    print(f"\n导入修复: {total_import_fixes}")
    print(f"验证修复: {total_validation_fixes}")
    print(f"error 修复: {total_error_fixes}")

    if error_count > 0:
        print("\n错误的文件:")
        for r in results:
            if 'error' in r:
                print(f"  - {r['filepath']}: {r['error']}")


if __name__ == '__main__':
    main()
