#!/usr/bin/env python3
"""
修复所有缺少分号的Transport::request调用
"""

import re
from pathlib import Path

def fix_all_transport_calls(root_dir: Path):
    """修复所有Transport::request调用中缺少分号的问题"""
    fixed_count = 0

    for rust_file in root_dir.rglob("*.rs"):
        if 'target' in str(rust_file) or '.git' in str(rust_file):
            continue

        try:
            with open(rust_file, 'r', encoding='utf-8') as f:
                content = f.read()

            original_content = content

            # 修复模式: ...Default::default() } 后面直接跟着 Transport::request
            content = re.sub(
                r'\.\.Default::default\(\)\s*\}\s*Transport::request',
                r'..Default::default()\n        };\n        Transport::request',
                content
            )

            # 修复模式: ...Default::default() } 后面直接跟着 Transport::<
            content = re.sub(
                r'\.\.Default::default\(\)\s*\}\s*Transport::<',
                r'..Default::default()\n        };\n        Transport::<',
                content
            )

            if content != original_content:
                with open(rust_file, 'w', encoding='utf-8') as f:
                    f.write(content)
                fixed_count += 1
                print(f"Fixed: {rust_file.relative_to(root_dir)}")

        except Exception as e:
            print(f"Error processing {rust_file}: {e}")

    return fixed_count

def main():
    print("🚀 开始修复所有Transport::request的分号问题...")
    print("=" * 60)

    root_dir = Path('/Users/zool/RustroverProjects/open-lark/src')
    fixed_count = fix_all_transport_calls(root_dir)

    print(f"\n✅ 修复完成！总共修复了 {fixed_count} 个文件")

if __name__ == "__main__":
    main()