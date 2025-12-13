#!/usr/bin/env python3
"""
修复 IllegalParamError 问题的脚本
将 LarkAPIError::IllegalParamError 替换为正确的 validation_error 函数调用
"""

import os
import re
import sys

def fix_illegal_param_error(content):
    """修复 IllegalParamError 调用"""
    # 首先添加必要的导入
    if 'use openlark_core::error::validation_error;' not in content and 'LarkAPIError::IllegalParamError' in content:
        # 在 openlark_core 导入后添加 validation_error 导入
        content = re.sub(
            r'(use openlark_core::[^;]+;)',
            r'\1\nuse openlark_core::error::validation_error;',
            content,
            count=1
        )

    # 替换错误模式1: LarkAPIError::IllegalParamError("message".to_string())
    content = re.sub(
        r'LarkAPIError::IllegalParamError\(([^)]+)\.to_string\(\)\)',
        r'validation_error("parameter", \1)',
        content
    )

    # 替换错误模式2: LarkAPIError::IllegalParamError(format!("..."))
    content = re.sub(
        r'LarkAPIError::IllegalParamError\((format!\([^)]+)\))',
        r'validation_error("parameter", \1)',
        content
    )

    # 替换错误模式3: LarkAPIError::IllegalParamError(string_value)
    content = re.sub(
        r'LarkAPIError::IllegalParamError\(([^)]+)\)',
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
        content = fix_illegal_param_error(content)

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
                        if 'LarkAPIError::IllegalParamError' in content:
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