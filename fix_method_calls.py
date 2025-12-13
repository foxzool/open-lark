#!/usr/bin/env python3
"""
修复方法调用问题
"""

import os
import re

def fix_method_calls(content, file_path):
    """修复方法调用问题"""
    # 修复 CcmDocV1Service 的方法调用
    if 'ccm_doc/v1/mod.rs' in file_path:
        # 这些应该是模块函数，不是方法
        content = re.sub(
            r'self\.create_document\(request, option\)',
            r'create_document(request, &self.config, option)',
            content
        )
        content = re.sub(
            r'self\.get_document_meta\(request, option\)',
            r'meta(request, &self.config, option)',
            content
        )
        content = re.sub(
            r'self\.get_sheet_meta\(request, option\)',
            r'sheet_meta(request, &self.config, option)',
            content
        )
        content = re.sub(
            r'self\.get_raw_content\(request, option\)',
            r'get_raw_content(request, &self.config, option)',
            content
        )
        content = re.sub(
            r'self\.get_document_content\(request, option\)',
            r'content(request, &self.config, option)',
            content
        )
        content = re.sub(
            r'self\.batch_update_document\(request, option\)',
            r'batch_update_document(request, &self.config, option)',
            content
        )

    # 修复返回类型问题
    content = re.sub(
        r'super::responses::RawContentData',
        r'raw_content::RawContentResponse',
        content
    )
    content = re.sub(
        r'super::responses::BatchUpdateData',
        r'batch_update::BatchUpdateResponse',
        content
    )

    # 修复剩余的 IllegalParamError
    content = re.sub(
        r'openlark_core::error::LarkAPIError::IllegalParamError\(\s*"([^"]+)"\s*\.to_string\(\)\s*\)',
        r'validation_error("parameter", "\1")',
        content
    )

    content = re.sub(
        r'openlark_core::error::LarkAPIError::IllegalParamError\(\s*([a-zA-Z_][a-zA-Z0-9_]*)\s*\)',
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
        content = fix_method_calls(content, file_path)

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