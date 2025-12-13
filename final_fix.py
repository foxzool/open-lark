#!/usr/bin/env python3
"""
最终的修复脚本
"""

import os
import re

def fix_all_errors(content, file_path):
    """修复所有错误"""
    # 1. 修复 IllegalParamError
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

    # 2. 修复 CCM Doc V1 的函数调用
    if 'ccm_doc/v1/mod.rs' in file_path:
        content = re.sub(
            r'meta\(request, &self\.config, option\)',
            r'meta::get_meta(request, &self.config, option)',
            content
        )
        content = re.sub(
            r'sheet_meta\(request, &self\.config, option\)',
            r'sheet_meta::get_sheet_meta(request, &self.config, option)',
            content
        )
        content = re.sub(
            r'content\(request, &self\.config, option\)',
            r'content::get_content(request, &self.config, option)',
            content
        )

    # 3. 修复返回类型
    content = re.sub(
        r'raw_content::RawContentResponse',
        r'responses::RawContentData',
        content
    )
    content = re.sub(
        r'batch_update::BatchUpdateResponse',
        r'responses::BatchUpdateData',
        content
    )

    # 4. 修复 data 字段访问 - 使用 data_or_default() 或类似方法
    # 对于不同的响应类型，使用不同的处理方式
    if 'drive_explorer' in file_path:
        content = re.sub(
            r'result\.data\.ok_or_else\(\|\|',
            r'result.ok_or_else(||',
            content
        )
        content = re.sub(
            r'ResponseType::Data\([^)]+\)',
            r'ResponseType::Data',
            content
        )

    # 5. 修复 ApiResponseTrait 相关问题
    content = re.sub(
        r'impl ApiResponseTrait for (\w+) \{',
        r'impl ApiResponseTrait for \1 {',
        content
    )

    return content

def process_file(file_path):
    """处理单个文件"""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()

        original_content = content
        content = fix_all_errors(content, file_path)

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