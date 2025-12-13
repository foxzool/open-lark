#!/usr/bin/env python3
"""
修复 IllegalParamError 问题的脚本 v2
将 LarkAPIError::IllegalParamError 替换为正确的 validation_error 函数调用
"""

import os
import re
import sys

def add_validation_error_import(content):
    """添加 validation_error 导入"""
    # 检查是否已经导入
    if 'use openlark_core::error::validation_error;' in content:
        return content

    # 查找 openlark_core 导入位置
    import_pattern = r'^(use openlark_core::\{[^}]+\};)'
    match = re.search(import_pattern, content, re.MULTILINE)

    if match:
        # 在现有导入中添加 validation_error
        existing_import = match.group(1)
        if 'error::' not in existing_import:
            # 添加 error:: 到导入列表
            new_import = existing_import.replace('use openlark_core::{', 'use openlark_core::{\n    error::validation_error,')
            content = content.replace(existing_import, new_import)
        else:
            # 在 error 导入中添加
            new_import = existing_import.replace('error::', 'error::{validation_error, ')
            content = content.replace(existing_import, new_import)
        return content
    else:
        # 如果没有找到现有导入，添加新的导入
        lines = content.split('\n')
        for i, line in enumerate(lines):
            if line.startswith('use ') and 'openlark_core' in line:
                lines.insert(i+1, 'use openlark_core::error::validation_error;')
                return '\n'.join(lines)

        # 如果没有找到任何 openlark_core 导入，在文件开头添加
        lines.insert(0, 'use openlark_core::error::validation_error;')
        return '\n'.join(lines)

def fix_illegal_param_errors(content):
    """修复所有 IllegalParamError 调用"""
    # 模式1: LarkAPIError::IllegalParamError("message".to_string())
    content = re.sub(
        r'openlark_core::error::LarkAPIError::IllegalParamError\(\s*"([^"]+)"\s*\.to_string\(\)\s*\)',
        r'validation_error("parameter", "\1")',
        content
    )

    # 模式2: LarkAPIError::IllegalParamError(message.to_string())
    content = re.sub(
        r'openlark_core::error::LarkAPIError::IllegalParamError\(\s*([^.\s]+)\.to_string\(\)\s*\)',
        r'validation_error("parameter", \1)',
        content
    )

    # 模式3: LarkAPIError::IllegalParamError(format!(...))
    content = re.sub(
        r'openlark_core::error::LarkAPIError::IllegalParamError\(\s*(format!\([^)]+\))\s*\)',
        r'validation_error("parameter", \1)',
        content
    )

    # 模式4: LarkAPIError::IllegalParamError(string_variable)
    content = re.sub(
        r'openlark_core::error::LarkAPIError::IllegalParamError\(\s*([a-zA-Z_][a-zA-Z0-9_]*)\s*\)',
        r'validation_error("parameter", \1)',
        content
    )

    # 处理没有 openlark_core::error:: 前缀的情况
    content = re.sub(
        r'LarkAPIError::IllegalParamError\(\s*"([^"]+)"\s*\.to_string\(\)\s*\)',
        r'validation_error("parameter", "\1")',
        content
    )

    content = re.sub(
        r'LarkAPIError::IllegalParamError\(\s*([a-zA-Z_][a-zA-Z0-9_]*)\s*\)',
        r'validation_error("parameter", \1)',
        content
    )

    return content

def process_file(file_path):
    """处理单个文件"""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()

        original_content = content

        # 添加导入
        content = add_validation_error_import(content)

        # 修复错误
        content = fix_illegal_param_errors(content)

        if content != original_content:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(content)
            print(f"✅ 修复完成: {file_path}")
            return True
        else:
            print(f"⏭️  无需修复: {file_path}")
            return False

    except Exception as e:
        print(f"❌ 处理失败 {file_path}: {e}")
        return False

def main():
    """主函数"""
    # 查找需要修复的文件
    files_to_fix = []
    for root, dirs, files in os.walk('crates/openlark-docs'):
        for file in files:
            if file.endswith('.rs'):
                file_path = os.path.join(root, file)
                try:
                    with open(file_path, 'r', encoding='utf-8') as f:
                        content = f.read()
                        if 'IllegalParamError' in content:
                            files_to_fix.append(file_path)
                except:
                    pass

    print(f"找到 {len(files_to_fix)} 个需要修复的文件")

    # 处理文件
    fixed_count = 0
    for file_path in files_to_fix:
        if process_file(file_path):
            fixed_count += 1

    print(f"\n🎉 完成！修复了 {fixed_count} 个文件")

if __name__ == '__main__':
    main()