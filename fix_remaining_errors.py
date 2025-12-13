#!/usr/bin/env python3
"""
修复剩余的编译错误
"""

import os
import re

def fix_remaining_errors(content):
    """修复剩余的错误"""
    # 修复带完整路径的 IllegalParamError
    content = re.sub(
        r'openlark_core::error::LarkAPIError::IllegalParamError\(\s*"([^"]+)"\s*\.to_string\(\)\s*\)',
        r'validation_error("parameter", "\1")',
        content
    )

    content = re.sub(
        r'openlark_core::error::LarkAPIError::IllegalParamError\(\s*([^.\s]+)\.to_string\(\)\s*\)',
        r'validation_error("parameter", \1)',
        content
    )

    content = re.sub(
        r'openlark_core::error::LarkAPIError::IllegalParamError\(\s*([a-zA-Z_][a-zA-Z0-9_]*)\s*\)',
        r'validation_error("parameter", \1)',
        content
    )

    # 修复函数调用 - 将模块名改为函数调用
    # 模式: let result = module_name(request, ...)
    content = re.sub(
        r'let result = (\w+)\(request, &self\.config, option\)\.await\?;',
        r'let result = self.\1(request, option).await?;',
        content
    )

    # 返回类型问题 - 修复私有结构体访问
    # 使用 super::responses::StructName
    content = re.sub(
        r'raw_content::RawContentData',
        r'super::responses::RawContentData',
        content
    )

    content = re.sub(
        r'batch_update::BatchUpdateData',
        r'super::responses::BatchUpdateData',
        content
    )

    return content

def process_file(file_path):
    """处理单个文件"""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()

        original_content = content
        content = fix_remaining_errors(content)

        if content != original_content:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(content)
            print(f"✅ 修复: {file_path}")
            return True
        else:
            return False

    except Exception as e:
        print(f"❌ 处理失败 {file_path}: {e}")
        return False

def main():
    """主函数"""
    files_fixed = 0

    # 遍历所有 rs 文件
    for root, dirs, files in os.walk('crates/openlark-docs'):
        for file in files:
            if file.endswith('.rs'):
                file_path = os.path.join(root, file)
                if process_file(file_path):
                    files_fixed += 1

    print(f"\n🎉 完成！修复了 {files_fixed} 个文件")

if __name__ == '__main__':
    main()