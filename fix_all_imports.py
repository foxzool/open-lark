#!/usr/bin/env python3
"""
修复所有的导入错误
"""

import os
import re

def fix_broken_imports(content):
    """修复损坏的导入语句"""
    # 修复 use openlark_core::use openlark_core 的问题
    content = re.sub(
        r'use openlark_core::\s*use openlark_core::error::validation_error;',
        'use openlark_core::{',
        content
    )

    # 修复其他可能的导入问题
    content = re.sub(
        r'use openlark_core::\s*use openlark_core::',
        'use openlark_core::',
        content
    )

    # 修复重复的 validation_error 导入
    content = re.sub(
        r'error::\{[^}]*validation_error[^}]*\},\s*error::validation_error',
        'error::validation_error',
        content
    )

    # 确保所有文件都有正确的导入
    if 'LarkAPIError::IllegalParamError' in content:
        if 'use openlark_core::error::validation_error;' not in content:
            # 找到 openlark_core 导入位置并添加
            if 'use openlark_core::{' in content:
                content = re.sub(
                    r'(use openlark_core::\{[^}]*error::[^}]*\})',
                    r'\1,\n    validation_error',
                    content
                )
            elif 'use openlark_core::' in content:
                content = re.sub(
                    r'(use openlark_core::[^;]+;)',
                    r'\1\nuse openlark_core::error::validation_error;',
                    content
                )

    return content

def process_file(file_path):
    """处理单个文件"""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()

        original_content = content
        content = fix_broken_imports(content)

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