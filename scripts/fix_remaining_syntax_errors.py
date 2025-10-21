#!/usr/bin/env python3
"""
修复剩余的语法错误
主要修复缺少分号的问题
"""

import re
from pathlib import Path
from typing import List, Tuple

def fix_missing_semicolons(root_dir: Path) -> List[Tuple[Path, str]]:
    """修复缺少分号的问题"""
    fixed_files = []

    # 需要修复的文件列表
    problem_files = [
        'src/service/contact/v3/department.rs',
        'src/service/contact/v3/functional_role_member.rs',
        'src/service/contact/v3/group.rs',
        'src/service/contact/v3/unit.rs',
        'src/service/contact/v3/user.rs',
        'src/service/im/v1/message_card/mod.rs',
        'src/service/im/v2/app_feed_card/mod.rs',
        'src/service/im/v2/groups_bots/mod.rs',
        'src/service/search/v2/data_item/mod.rs',
        'src/service/search/v2/data_source/mod.rs',
        'src/service/search/v2/schema/mod.rs'
    ]

    for file_path_str in problem_files:
        file_path = root_dir / file_path_str
        if not file_path.exists():
            continue

        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()

            original_content = content

            # 修复模式1: } 后面直接跟着 let 语句，缺少分号
            content = re.sub(
                r'\n        }\s*\n\s*let resp = ',
                r'\n        };\n        let resp = ',
                content
            )

            # 修复模式2: } 后面直接跟着 Transport::request，缺少分号
            content = re.sub(
                r'\n        }\s*\n\s*Transport::request\(',
                r'\n        };\n        Transport::request(',
                content
            )

            # 修复模式3: 其他可能的情况
            content = re.sub(
                r'\n        }\s*\n([a-zA-Z_])',
                r'\n        };\n\1',
                content
            )

            # 只有内容发生变化时才写回文件
            if content != original_content:
                with open(file_path, 'w', encoding='utf-8') as f:
                    f.write(content)
                fixed_files.append((file_path, "添加缺少的分号"))

        except Exception as e:
            print(f"❌ 修复文件失败 {file_path}: {e}")

    return fixed_files

def main():
    print("🚀 开始修复剩余语法错误...")
    print("=" * 60)

    root_dir = Path('/Users/zool/RustroverProjects/open-lark')

    fixed_files = fix_missing_semicolons(root_dir)

    print(f"\n📊 修复结果:")
    print(f"   • 修复文件数: {len(fixed_files)}")

    if fixed_files:
        print(f"   • 修复的文件:")
        for file_path, reason in fixed_files:
            relative_path = file_path.relative_to('/Users/zool/RustroverProjects/open-lark')
            print(f"     • {relative_path} - {reason}")

    print(f"\n✅ 剩余语法错误修复完成！")

    return 0

if __name__ == "__main__":
    exit(main())