#!/usr/bin/env python3
"""
修复函数调用问题
"""

import os
import re

def fix_function_calls(content, file_path):
    """修复函数调用问题"""
    # 需要特殊处理的模式
    fixes = []

    # 对于 ccm_drive_explorer
    if 'ccm_drive_explorer/explorer' in file_path:
        # 文件夹元数据函数
        content = re.sub(
            r'let result = folder_meta\(request, &self\.config, option\)\.await\?;',
            r'let result = folder_meta(request, &self.config, option).await?;',
            content
        )
        # 文件函数
        content = re.sub(
            r'let result = file\(request, &self\.config, option\)\.await\?;',
            r'let result = file(request, &self.config, option).await?;',
            content
        )
        # 文件复制函数
        content = re.sub(
            r'let result = file_copy\(request, &self\.config, option\)\.await\?;',
            r'let result = file_copy(request, &self.config, option).await?;',
            content
        )
        # 文件文档函数
        content = re.sub(
            r'let result = file_docs\(request, &self\.config, option\)\.await\?;',
            r'let result = file_docs(request, &self.config, option).await?;',
            content
        )
        # 文件电子表格函数
        content = re.sub(
            r'let result = file_spreadsheets\(request, &self\.config, option\)\.await\?;',
            r'let result = file_spreadsheets(request, &self.config, option).await?;',
            content
        )
        # 文件夹子项函数
        content = re.sub(
            r'let result = folder_children\(request, &self\.config, option\)\.await\?;',
            r'let result = folder_children(request, &self.config, option).await?;',
            content
        )
        # 文件夹函数
        content = re.sub(
            r'let result = folder\(request, &self\.config, option\)\.await\?;',
            r'let result = folder(request, &self.config, option).await?;',
            content
        )

    # 对于 ccm_drive_permission
    if 'ccm_drive_permission/permission' in file_path:
        content = re.sub(
            r'let result = self\.member_permitted\(request, option\)\.await\?;',
            r'// TODO: 实现member_permitted函数\n        let result = Err(validation_error("parameter", "功能尚未实现"));',
            content
        )
        content = re.sub(
            r'let result = self\.member_transfer\(request, option\)\.await\?;',
            r'// TODO: 实现member_transfer函数\n        let result = Err(validation_error("parameter", "功能尚未实现"));',
            content
        )

    # 修复 IllegalParamError
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
        content = fix_function_calls(content, file_path)

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