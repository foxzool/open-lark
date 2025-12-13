#!/usr/bin/env python3
"""
修复导入错误
"""

import os
import re

def fix_import_errors(content):
    """修复导入错误"""
    # 修复 use openlark_core::use openlark_core 的问题
    content = re.sub(
        r'use openlark_core::\nuse openlark_core::error::validation_error;',
        'use openlark_core::',
        content
    )

    content = re.sub(
        r'use openlark_core::\nuse openlark_core::error::validation_error;',
        'use openlark_core::{\n    error::validation_error,',
        content
    )

    # 修复重复的 validation_error 导入
    content = re.sub(
        r'error::validation_error,\s*error::validation_error',
        'error::validation_error',
        content
    )

    # 修复其他可能的导入错误
    content = re.sub(
        r'use openlark_core::\nuse openlark_core::',
        'use openlark_core::',
        content
    )

    return content

def process_file(file_path):
    """处理单个文件"""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()

        original_content = content
        content = fix_import_errors(content)

        if content != original_content:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(content)
            print(f"✅ 修复导入错误: {file_path}")
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
                        if 'use openlark_core::\nuse openlark_core::' in content or 'r#use' in content:
                            files_to_fix.append(file_path)
                except:
                    pass

    print(f"找到 {len(files_to_fix)} 个需要修复导入的文件")

    # 处理文件
    fixed_count = 0
    for file_path in files_to_fix:
        if process_file(file_path):
            fixed_count += 1

    print(f"\n🎉 完成！修复了 {fixed_count} 个文件的导入错误")

if __name__ == '__main__':
    main()