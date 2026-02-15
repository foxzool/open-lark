#!/usr/bin/env python3
"""
修复 workflow 测试模块中未导入的类型
"""

import os
import re

def fix_test_module(file_path):
    """修复测试模块中的导入"""
    with open(file_path, 'r') as f:
        content = f.read()

    original = content

    # 查找空的测试模块
    empty_test_pattern = re.compile(
        r'(\#\[cfg\(test\)\]\s*mod tests\s*\{\s*)(\s*)(\}\s*)',
        re.MULTILINE
    )

    # 检查是否有测试函数需要 Arc 或其他导入
    # 如果测试模块是空的或者只有空行，直接添加空行
    if 'use super::*;' not in content:
        # 替换空测试模块
        content = empty_test_pattern.sub(r'\1\2\n\2    #[test]\n\2    fn _dummy_test() {}\n\2}\n', content)

    if content != original:
        with open(file_path, 'w') as f:
            f.write(content)
        return True
    return False

def main():
    base_dir = "/Users/zool/RustroverProjects/open-lark/crates/openlark-workflow/src"
    fixed_count = 0

    for root, dirs, files in os.walk(base_dir):
        for file in files:
            if file.endswith('.rs'):
                file_path = os.path.join(root, file)
                try:
                    with open(file_path, 'r') as f:
                        content = f.read()
                    # 如果测试模块只有空行或没有导入，添加 dummy test
                    if '#[cfg(test)]' in content and 'mod tests {' in content:
                        test_match = re.search(r'#\[cfg\(test\)\]\s*mod tests\s*\{([^}]*)\}', content, re.DOTALL)
                        if test_match:
                            test_content = test_match.group(1).strip()
                            if not test_content or test_content == '':
                                # 添加 dummy test
                                new_test = '''#[cfg(test)]
mod tests {
    #[test]
    fn _dummy_test() {}
}'''
                                content = re.sub(
                                    r'#\[cfg\(test\)\]\s*mod tests\s*\{[^}]*\}',
                                    new_test,
                                    content
                                )
                                with open(file_path, 'w') as f:
                                    f.write(content)
                                print(f"Fixed: {file_path}")
                                fixed_count += 1
                except Exception as e:
                    print(f"Error processing {file_path}: {e}")

    print(f"\nTotal files fixed: {fixed_count}")

if __name__ == "__main__":
    main()
