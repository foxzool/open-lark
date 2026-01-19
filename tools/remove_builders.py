#!/usr/bin/env python3
"""批量删除冗余的 Builder 结构并更新 mod.rs 导出"""

import re
from pathlib import Path
from typing import List, Dict, Set

# 需要处理的文件列表（与之前相同）
FILES_TO_PROCESS = [
    # Bitable record 模块
    "crates/openlark-docs/src/base/bitable/v1/app/table/record/list.rs",
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
    "crates/openlark-docs/src/base/bitable/v1/app/table/batch_create.rs",
    "crates/openlark-docs/src/base/bitable/v1/app/table/batch_delete.rs",
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
]

# Builder 名称映射（Builder 名称 -> 主 Request 类型）
BUILDER_NAMES = {
    "ListRecordRequestBuilder": "ListRecordRequest",
    "GetRecordRequestBuilder": "GetRecordRequest",
    "BatchGetRecordRequestBuilder": "BatchGetRecordRequest",
    "BatchCreateRecordRequestBuilder": "BatchCreateRecordRequest",
    "BatchUpdateRecordRequestBuilder": "BatchUpdateRecordRequest",
    "BatchDeleteRecordRequestBuilder": "BatchDeleteRecordRequest",
    "CreateFieldRequestBuilder": "CreateFieldRequest",
    "UpdateFieldRequestBuilder": "UpdateFieldRequest",
    "ListFieldRequestBuilder": "ListFieldRequest",
    "DeleteFieldRequestBuilder": "DeleteFieldRequest",
    "PatchTableRequestBuilder": "PatchTableRequest",
    "CreateTableRequestBuilder": "CreateTableRequest",
    "ListTableRequestBuilder": "ListTableRequest",
    "DeleteTableRequestBuilder": "DeleteTableRequest",
    "BatchCreateTableRequestBuilder": "BatchCreateTableRequest",
    "BatchDeleteTableRequestBuilder": "BatchDeleteTableRequest",
    "PatchViewRequestBuilder": "PatchViewRequest",
    "CreateViewRequestBuilder": "CreateViewRequest",
    "GetViewRequestBuilder": "GetViewRequest",
    "ListViewRequestBuilder": "ListViewRequest",
    "DeleteViewRequestBuilder": "DeleteViewRequest",
    "PatchFormRequestBuilder": "PatchFormRequest",
    "GetFormRequestBuilder": "GetFormRequest",
    "PatchFormFieldRequestBuilder": "PatchFormFieldRequest",
    "ListFormFieldRequestBuilder": "ListFormFieldRequest",
    "UpdateRoleRequestBuilder": "UpdateRoleRequest",
    "ListRoleRequestBuilder": "ListRoleRequest",
    "CreateRoleRequestBuilder": "CreateRoleRequest",
    "DeleteRoleRequestBuilder": "DeleteRoleRequest",
    "ListRoleMemberRequestBuilder": "ListRoleMemberRequest",
    "CreateRoleMemberRequestBuilder": "CreateRoleMemberRequest",
    "DeleteRoleMemberRequestBuilder": "DeleteRoleMemberRequest",
    "BatchCreateRoleMemberRequestBuilder": "BatchCreateRoleMemberRequest",
    "BatchDeleteRoleMemberRequestBuilder": "BatchDeleteRoleMemberRequest",
}


def extract_builder_struct_and_impl(content: str, builder_name: str) -> tuple:
    """提取 Builder 结构定义和 impl 块"""
    # 查找 Builder 结构定义
    struct_pattern = rf'(?:///[^\n]*\n)*pub struct {builder_name}\s*\{{.*?^\}}'
    struct_match = re.search(struct_pattern, content, re.MULTILINE | re.DOTALL)

    # 查找 Builder impl 块
    impl_pattern = rf'impl {builder_name}\s*\{{.*?^\}}'
    impl_match = re.search(impl_pattern, content, re.MULTILINE | re.DOTALL)

    return struct_match, impl_match


def remove_builder_from_file(content: str, builder_name: str) -> tuple:
    """从文件中删除 Builder 结构和 impl 块"""
    struct_match, impl_match = extract_builder_struct_and_impl(content, builder_name)

    removed = False
    new_content = content

    # 删除 impl 块（先删除，因为位置靠后）
    if impl_match:
        new_content = new_content[:impl_match.start()] + new_content[impl_match.end():]
        removed = True

    # 删除结构定义
    if struct_match:
        new_content = new_content[:struct_match.start()] + new_content[struct_match.end():]
        removed = True

    return new_content, removed


def process_file(filepath: str) -> dict:
    """处理单个文件，删除其中的 Builder 结构"""
    print(f"处理文件: {filepath}")

    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            original_content = f.read()

        content = original_content
        changes = {
            'filepath': filepath,
            'builders_removed': [],
            'modified': False
        }

        # 查找文件中所有可能存在的 Builder
        for builder_name in BUILDER_NAMES:
            if builder_name in content:
                content, removed = remove_builder_from_file(content, builder_name)
                if removed:
                    changes['builders_removed'].append(builder_name)
                    changes['modified'] = True

        if changes['modified']:
            with open(filepath, 'w', encoding='utf-8') as f:
                f.write(content)
            print(f"  ✓ 已删除: {', '.join(changes['builders_removed'])}")
        else:
            print(f"  - 无 Builder 需要删除")

        return changes

    except Exception as e:
        print(f"  ✗ 错误: {e}")
        return {
            'filepath': filepath,
            'error': str(e),
            'modified': False
        }


def update_mod_rs_export(mod_rs_path: str, builders_to_remove: Set[str]) -> bool:
    """更新 mod.rs 文件，删除 Builder 的导出"""
    try:
        with open(mod_rs_path, 'r', encoding='utf-8') as f:
            content = f.read()

        original_content = content
        modified = False

        # 查找并删除 pub use ...::{...Builder} 形式的导出
        for builder_name in builders_to_remove:
            # 匹配 pub use super::{..., BuilderName}
            pattern1 = rf',\s*{re.escape(builder_name)}\s*'
            new_content, count = re.subn(pattern1, '', content)
            if count > 0:
                content = new_content
                modified = True

            # 匹配独立的 pub use ...::BuilderName;
            pattern2 = rf'pub use\s+[^:]+::{re.escape(builder_name)};\s*\n'
            new_content, count = re.subn(pattern2, '', content)
            if count > 0:
                content = new_content
                modified = True

        if modified:
            with open(mod_rs_path, 'w', encoding='utf-8') as f:
                f.write(content)
            return True

        return False

    except Exception as e:
        print(f"  ✗ 更新 mod.rs 错误: {e}")
        return False


def find_mod_rs_files(base_path: Path) -> Dict[str, List[str]]:
    """查找所有可能需要更新的 mod.rs 文件"""
    mod_files = {}

    # 定义需要检查的 mod.rs 路径
    mod_paths = [
        "crates/openlark-docs/src/base/bitable/v1/app/table/record/mod.rs",
        "crates/openlark-docs/src/base/bitable/v1/app/table/field/mod.rs",
        "crates/openlark-docs/src/base/bitable/v1/app/table/mod.rs",
        "crates/openlark-docs/src/base/bitable/v1/app/table/view/mod.rs",
        "crates/openlark-docs/src/base/bitable/v1/app/table/form/field/mod.rs",
        "crates/openlark-docs/src/base/bitable/v1/app/table/form/mod.rs",
        "crates/openlark-docs/src/base/bitable/v1/app/role/member/mod.rs",
        "crates/openlark-docs/src/base/bitable/v1/app/role/mod.rs",
    ]

    for mod_path in mod_paths:
        full_path = base_path / mod_path
        if full_path.exists():
            mod_files[mod_path] = []

    return mod_files


def main():
    """主函数"""
    base_path = Path("/Users/zool/RustroverProjects/open-lark")

    print("=" * 80)
    print("批量删除 Builder 结构")
    print("=" * 80)
    print()

    results = []
    all_removed_builders = set()

    # 处理所有文件
    for rel_path in FILES_TO_PROCESS:
        filepath = base_path / rel_path
        if filepath.exists():
            result = process_file(str(filepath))
            results.append(result)
            if result.get('builders_removed'):
                all_removed_builders.update(result['builders_removed'])
        else:
            print(f"文件不存在: {filepath}")

    print()
    print("=" * 80)
    print("删除总结")
    print("=" * 80)

    modified_count = sum(1 for r in results if r.get('modified', False))
    total_builders = sum(len(r.get('builders_removed', [])) for r in results)

    print(f"总文件数: {len(results)}")
    print(f"已修改: {modified_count}")
    print(f"删除的 Builder 数: {total_builders}")
    print(f"删除的 Builder 类型: {len(all_removed_builders)}")

    if all_removed_builders:
        print("\n已删除的 Builder 类型:")
        for builder in sorted(all_removed_builders):
            print(f"  - {builder}")

    # 提示用户手动更新 mod.rs 文件
    print()
    print("=" * 80)
    print("下一步：手动更新 mod.rs 文件")
    print("=" * 80)
    print("\n需要从以下 mod.rs 文件中删除 Builder 的导出:")
    print("  - crates/openlark-docs/src/base/bitable/v1/app/table/record/mod.rs")
    print("  - crates/openlark-docs/src/base/bitable/v1/app/table/field/mod.rs")
    print("  - crates/openlark-docs/src/base/bitable/v1/app/table/mod.rs")
    print("  - crates/openlark-docs/src/base/bitable/v1/app/table/view/mod.rs")
    print("  - crates/openlark-docs/src/base/bitable/v1/app/table/form/field/mod.rs")
    print("  - crates/openlark-docs/src/base/bitable/v1/app/table/form/mod.rs")
    print("  - crates/openlark-docs/src/base/bitable/v1/app/role/member/mod.rs")
    print("  - crates/openlark-docs/src/base/bitable/v1/app/role/mod.rs")


if __name__ == '__main__':
    main()
