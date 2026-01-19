#!/usr/bin/env python3
"""
综合修复 openlark-docs API 代码规范

处理以下问题：
1. 添加 execute_with_options 方法（处理旧模式）
2. 统一响应提取方式为 extract_response_data
3. 统一序列化方式为 serialize_params
4. 统一 RequestOption 传递（None -> Some(option)）
5. 删除冗余的 Builder 结构体
"""

import re
from pathlib import Path
from typing import List, Tuple, Dict


def add_execute_with_options_old_pattern(content: str) -> Tuple[str, int]:
    """
    处理旧模式的 execute 方法
    模式：只有 execute() 方法，传递 None 给 Transport
    """
    count = 0

    # 查找需要修改的 execute 方法
    # 模式：传递 None 给 Transport，使用 response.data.ok_or_else
    pattern = r'(pub async fn execute\(self\)\s*->\s*SDKResult<([^>]+)>\s*\{)([^}]*?)(let response = Transport::request\([^)]+,\s*&self\.config,\s+)None(\)\.await\?;[^}]*?response\s*\.\s*data\s*\.\s*ok_or_else\([^}]+\))(\s*\})'

    def replace_fn(match):
        nonlocal count
        count += 1

        fn_def_start = match.group(1)      # pub async fn execute(self) -> SDKResult<Type> {
        response_type = match.group(2)     # Type
        fn_body = match.group(3)           # 函数体（验证等）
        transport_start = match.group(4)   # let response = Transport::request(..., &self.config,
        none_part = match.group(5)         # None).await?;
        response_extract = match.group(6)  # response.data.ok_or_else(...)
        closing_brace = match.group(7)     # }

        # 提取上下文用于 extract_response_data
        context_match = re.search(r'"([^"]*)"', response_extract)
        context = context_match.group(1) if context_match else "操作"

        # 构建 execute_with_options
        new_execute = f'''{fn_def_start}{fn_body}        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
        }}

        pub async fn execute_with_options(
            self,
            option: openlark_core::req_option::RequestOption,
        ) -> SDKResult<{response_type}> {{
{fn_body}            let response = Transport::request({transport_start}Some(option){none_part}
            extract_response_data(response, "{context}")
        }}'''

        return new_execute

    new_content = re.sub(pattern, replace_fn, content, flags=re.DOTALL)

    return new_content, count


def replace_response_data_extraction(content: str) -> Tuple[str, int]:
    """
    替换 response.data.ok_or_else 为 extract_response_data
    """
    count = 0

    # 模式1: response.data.ok_or_else(|| openlark_core::error::validation_error(...))
    pattern1 = r'response\s*\.\s*data\s*\.\s*ok_or_else\(\s*\|\|\s*openlark_core::error::validation_error\s*\(\s*"([^"]+)"\s*,\s*"([^"]+)"\s*\)\s*\)'

    def replace1(match):
        nonlocal count
        count += 1
        return f'extract_response_data(response, "{match.group(1)}")'

    new_content = re.sub(pattern1, replace1, content, flags=re.DOTALL)

    # 模式2: response.data.ok_or_else(|| validation_error(...)) (已导入 validation_error)
    pattern2 = r'response\s*\.\s*data\s*\.\s*ok_or_else\(\s*\|\|\s*validation_error\s*\(\s*"([^"]+)"\s*,\s*"([^"]+)"\s*\)\s*\)'

    def replace2(match):
        nonlocal count
        count += 1
        return f'extract_response_data(response, "{match.group(1)}")'

    new_content = re.sub(pattern2, replace2, new_content, flags=re.DOTALL)

    return new_content, count


def replace_serialize_params(content: str) -> Tuple[str, int]:
    """
    替换 serde_json::to_vec 为 serialize_params
    """
    count = 0

    # 模式: .body(serde_json::to_vec(&...)?)
    # 这个比较复杂，需要找到 serde_json::to_vec 的调用并提取上下文
    pattern = r'\.body\(serde_json::to_vec\(&([^)]+)\)\?\)'

    def replace_fn(match):
        nonlocal count
        count += 1
        param_name = match.group(1)
        # 尝试从上下文推断操作名称
        return f'.body(serialize_params(&{param_name}, "操作")?)'

    new_content = re.sub(pattern, replace_fn, content, flags=re.DOTALL)

    return new_content, count


def fix_request_option_none(content: str) -> Tuple[str, int]:
    """
    将 Transport::request 中的 None 替换为 Some(option)
    这个需要在有 execute_with_options 方法的情况下
    """
    count = 0

    # 只在 execute_with_options 方法中替换
    # 查找 execute_with_options 方法体
    pattern = r'pub async fn execute_with_options\([^{]+\{\s*(.*?)(\n\s*pub\s+async\s+fn|\n\}\s*$)'

    def replace_fn(match):
        nonlocal count
        method_body = match.group(1)

        # 在方法体中替换 None 为 Some(option)
        old_body = method_body
        new_body = re.sub(
            r'(Transport::request\([^,]+,\s*&[^,]+,\s+)None(\)',
            r'\1Some(option)\2',
            method_body
        )

        if new_body != old_body:
            count += 1

        return f'pub async fn execute_with_options(\n    self,\n    option: openlark_core::req_option::RequestOption,\n) -> SDKResult<T> {{\n        {new_body}\n{match.group(2)}'

    new_content = re.sub(pattern, replace_fn, content, flags=re.DOTALL)

    return new_content, count


def remove_builder_struct(content: str) -> Tuple[str, List[str]]:
    """
    删除冗余的 Builder 结构体
    """
    removed_builders = []

    # 查找所有 Builder 结构定义
    # 模式: pub struct XxxRequestBuilder
    builder_pattern = r'/// [^\n]*\n?pub struct (\w+RequestBuilder)\s*\{.*?^\n\}'

    def remove_struct(match):
        builder_name = match.group(1)
        removed_builders.append(builder_name)
        return ''  # 删除整个结构定义

    new_content = re.sub(builder_pattern, remove_struct, content, flags=re.MULTILINE | re.DOTALL)

    # 删除 Builder impl 块
    for builder_name in removed_builders:
        impl_pattern = rf'\nimpl {builder_name}\s*\{{.*?^\}}\n?'
        new_content = re.sub(impl_pattern, '', new_content, flags=re.MULTILINE | re.DOTALL)

    return new_content, removed_builders


def add_api_utils_import(content: str) -> Tuple[str, int]:
    """
    添加 api_utils 导入（如果需要）
    """
    # 检查是否使用了 extract_response_data 或 serialize_params
    needs_utils = 'extract_response_data' in content or 'serialize_params' in content

    if not needs_utils:
        return content, 0

    # 检查是否已经导入了 api_utils
    if 'api_utils' in content:
        return content, 0

    # 查找 use crate::common::... 语句
    pattern = r'use crate::common::\{([^}]+)\}'

    def add_to_import(match):
        existing = match.group(1)
        if 'api_utils' not in existing:
            return f'use crate::common::{{{existing}, api_utils::*}}'
        return match.group(0)

    new_content, count = re.subn(pattern, add_to_import, content)

    if count == 0:
        # 没有找到现有的 common 导入，在开头添加
        # 查找第一个 use 语句
        use_pattern = r'\n(use crate::common::[^\n]+\n)'
        new_content, count = re.subn(
            use_pattern,
            r'\nuse crate::common::api_utils::*;\1',
            content,
            count=1
        )

    return new_content, count


def process_file(filepath: str, dry_run: bool = False) -> Dict:
    """处理单个文件"""
    print(f"处理文件: {filepath}")

    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            original_content = f.read()

        content = original_content
        changes = {
            'filepath': filepath,
            'execute_added': 0,
            'response_fixed': 0,
            'serialize_fixed': 0,
            'builders_removed': [],
            'import_added': 0,
            'modified': False
        }

        # 1. 添加 execute_with_options 方法
        content, count = add_execute_with_options_old_pattern(content)
        changes['execute_added'] = count
        if count > 0:
            changes['modified'] = True

        # 2. 替换响应提取方式
        content, count = replace_response_data_extraction(content)
        changes['response_fixed'] = count
        if count > 0:
            changes['modified'] = True

        # 3. 替换序列化方式
        content, count = replace_serialize_params(content)
        changes['serialize_fixed'] = count
        if count > 0:
            changes['modified'] = True

        # 4. 删除 Builder 结构体
        content, removed = remove_builder_struct(content)
        changes['builders_removed'] = removed
        if removed:
            changes['modified'] = True

        # 5. 添加 api_utils 导入
        content, count = add_api_utils_import(content)
        changes['import_added'] = count
        if count > 0:
            changes['modified'] = True

        if changes['modified'] and not dry_run:
            with open(filepath, 'w', encoding='utf-8') as f:
                f.write(content)
            print(f"  ✓ 已修改: execute({changes['execute_added']}) response({changes['response_fixed']}) serialize({changes['serialize_fixed']}) builders({len(changes['builders_removed'])}) import({changes['import_added']})")
        elif changes['modified']:
            print(f"  [预览] 将修改: execute({changes['execute_added']}) response({changes['response_fixed']}) serialize({changes['serialize_fixed']}) builders({len(changes['builders_removed'])}) import({changes['import_added']})")
        else:
            print(f"  - 无需修改")

        return changes

    except Exception as e:
        print(f"  ✗ 错误: {e}")
        import traceback
        traceback.print_exc()
        return {
            'filepath': filepath,
            'error': str(e),
            'modified': False
        }


def find_files_to_process(base_path: Path) -> List[str]:
    """查找需要处理的文件"""
    files = []

    # 扫描 openlark-docs/src 目录下的所有 .rs 文件
    src_path = base_path / "crates/openlark-docs/src"

    for rs_file in src_path.rglob("*.rs"):
        # 跳过 mod.rs 和旧代码
        if "old" in str(rs_file):
            continue
        files.append(str(rs_file))

    return files


def main():
    """主函数"""
    import argparse
    parser = argparse.ArgumentParser(description="综合修复 openlark-docs API 代码规范")
    parser.add_argument("--dry-run", action="store_true", help="预览模式，不修改文件")
    parser.add_argument("--path", default="crates/openlark-docs/src", help="源代码路径")
    args = parser.parse_args()

    base_path = Path(args.path).parent.parent.parent

    print("=" * 80)
    print("综合修复 openlark-docs API 代码规范")
    print("=" * 80)
    print()

    files = find_files_to_process(base_path)
    print(f"找到 {len(files)} 个 .rs 文件")
    print()

    results = []
    for filepath in files:
        result = process_file(filepath, args.dry_run)
        results.append(result)

    print()
    print("=" * 80)
    print("修复总结")
    print("=" * 80)

    modified_count = sum(1 for r in results if r.get('modified', False))
    error_count = sum(1 for r in results if 'error' in r)

    print(f"总文件数: {len(results)}")
    print(f"已修改: {modified_count}")
    print(f"错误: {error_count}")

    total_execute = sum(r.get('execute_added', 0) for r in results)
    total_response = sum(r.get('response_fixed', 0) for r in results)
    total_serialize = sum(r.get('serialize_fixed', 0) for r in results)
    total_builders = sum(len(r.get('builders_removed', [])) for r in results)
    total_import = sum(r.get('import_added', 0) for r in results)

    print(f"\nexecute_with_options 添加: {total_execute}")
    print(f"响应提取修复: {total_response}")
    print(f"序列化修复: {total_serialize}")
    print(f"Builder 删除: {total_builders}")
    print(f"导入添加: {total_import}")

    if error_count > 0:
        print("\n错误的文件:")
        for r in results:
            if 'error' in r:
                print(f"  - {r['filepath']}: {r['error']}")


if __name__ == '__main__':
    main()
