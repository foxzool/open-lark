#!/usr/bin/env python3

import os
import re
from pathlib import Path

def fix_file(file_path):
    """修复单个文件中的crate路径问题"""
    print(f"修复文件: {file_path}")

    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()

        # 备份原文件
        backup_path = f"{file_path}.backup"
        with open(backup_path, 'w', encoding='utf-8') as bf:
            bf.write(content)

        # 执行各种修复
        fixed_content = content

        # 1. 修复 use crate:: 开头的导入
        fixed_content = re.sub(
            r'use crate::([^{}]*)::',
            r'use open_lark_core::core::\1::',
            fixed_content
        )

        # 2. 修复 -> crate::core:: 模式
        fixed_content = re.sub(
            r'-> crate::core::',
            r'-> open_lark_core::core::',
            fixed_content
        )

        # 3. 修复 crate::core:: 模式
        fixed_content = re.sub(
            r'crate::core::',
            r'open_lark_core::core::',
            fixed_content
        )

        # 4. 修复 let api_req = ApiRequest { 模式为 builder 模式
        fixed_content = re.sub(
            r'let api_req = ApiRequest \{([^}]*?)\}',
            r'let mut api_req = ApiRequest::default();\n        \1',
            fixed_content
        )

        # 5. 添加必要的 set 调用
        # 在 ApiRequest::default() 后面添加 set 调用
        api_req_pattern = r'let mut api_req = ApiRequest::default\(\);'

        def add_set_calls(match):
            method = match.group(1)
            if method == 'POST':
                return f'{match.group(0)}        api_req.set_http_method(reqwest::Method::POST);'
            elif method == 'GET':
                return f'{match.group(0)}        api_req.set_http_method(reqwest::Method::GET);'
            elif method == 'PATCH':
                return f'{match.group(0)}        api_req.set_http_method(reqwest::Method::PATCH);'
            elif method == 'PUT':
                return f'{match.group(0)}        api_req.set_http_method(reqwest::Method::PUT);'
            elif method == 'DELETE':
                return f'{match.group(0)}        api_req.set_http_method(reqwest::Method::DELETE);'
            return ''

        fixed_content = re.sub(
            api_req_pattern,
            add_set_calls,
            fixed_content
        )

        # 6. 修复 http_method, api_path, body 等赋值
        field_patterns = [
            (r'(\s+)http_method: reqwest::Method::[^,]*,', r'\1api_req.set_http_method(reqwest::Method::\2);'),
            (r'(\s+)api_path: [^,]*', r'\1api_req.set_api_path(\2);'),
            (r'(\s+)body: [^,]*', r'\1api_req.body = \2;'),
            (r'(\s+)supported_access_token_types: [^,]*', r'\1api_req.set_supported_access_token_types(\2);'),
        ]

        for old_pattern, new_pattern in field_patterns:
            fixed_content = re.sub(old_pattern, new_pattern, fixed_content)

        # 7. 移除旧的 ..Default::default() 和多余的分号
        fixed_content = re.sub(r',\s*\.\.\.default\(\);*\s*}', '', fixed_content)
        fixed_content = re.sub(r'};\s*}', '}', fixed_content)

        # 写入修复后的内容
        if fixed_content != content:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(fixed_content)
            print(f"  ✅ 修复完成: {file_path}")
            return True
        else:
            print(f"  ⚠️  无需修复: {file_path}")
            return False

    except Exception as e:
        print(f"  ❌ 修复文件 {file_path} 时出错: {e}")
        return False

def main():
    """主函数"""
    print("=== Python路径修复工具 ===")

    # 查找所有需要修复的文件
    crates_dir = Path("crates")
    fixed_count = 0
    total_count = 0

    # 遍历所有crate目录
    for crate_dir in crates_dir.iterdir():
        if crate_dir.is_dir():
            for rust_file in crate_dir.rglob("**/*.rs"):
                total_count += 1

                with open(rust_file, 'r', encoding='utf-8') as f:
                    content = f.read()

                # 检查是否需要修复
                needs_fix = (
                    'crate::' in content and
                    'open_lark_core::core::' not in content
                ) or (
                    'let api_req = ApiRequest {' in content and
                    'ApiRequest::default()' not in content
                )

                if needs_fix:
                    print(f"发现需要修复的文件: {rust_file}")
                    if fix_file(rust_file):
                        fixed_count += 1
                else:
                    print(f"无需修复: {rust_file}")

    print(f"=== 修复完成 ===")
    print(f"总文件数: {total_count}")
    print(f"修复文件数: {fixed_count}")
    print(f"剩余未修复文件数: {total_count - fixed_count}")

if __name__ == "__main__":
    main()