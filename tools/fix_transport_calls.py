#!/usr/bin/env python3
"""
快速修复 Transport::request 调用，添加缺失的第三个参数
"""

import re
from pathlib import Path

def fix_transport_request(content: str) -> tuple:
    """
    修复 Transport::request 调用，将
        Transport::request(..., &self.config, ).await?;
    改为
        Transport::request(..., &self.config, Some(option)).await?;
    """
    count = 0

    # 匹配模式：Transport::request(..., &self.config, ).await?;
    # 注意：这个模式匹配 execute_with_options 方法中的调用
    pattern = r'Transport::request\(([^,]+),\s*&self\.config,\s*\)\.await\?'

    def replace_fn(match):
        nonlocal count
        count += 1
        return f'Transport::request({match.group(1)}, &self.config, Some(option)).await?'

    new_content = re.sub(pattern, replace_fn, content)

    return new_content, count

def main():
    base_path = Path("/Users/zool/RustroverProjects/open-lark")

    # 需要修复的文件列表
    files_to_fix = [
        "crates/openlark-docs/src/ccm/ccm_doc/old/default/_doc_token/content.rs",
        "crates/openlark-docs/src/ccm/ccm_doc/old/default/_doc_token/raw_content.rs",
        "crates/openlark-docs/src/ccm/ccm_doc/old/default/_doc_token/sheet_meta.rs",
        "crates/openlark-docs/src/ccm/ccm_doc/old/default/create.rs",
        "crates/openlark-docs/src/ccm/ccm_doc/old/default/meta/_doc_token.rs",
        "crates/openlark-docs/src/ccm/ccm_docs/old/default/docs-api/meta.rs",
        "crates/openlark-docs/src/ccm/ccm_docs/old/default/docs-api/search/object.rs",
        "crates/openlark-docs/src/ccm/ccm_drive_explorer/old/default/v2/file/_folder_token.rs",
        "crates/openlark-docs/src/ccm/ccm_drive_explorer/old/default/v2/file/copy/files/_file_token.rs",
    ]

    print("=" * 80)
    print("修复 Transport::request 调用")
    print("=" * 80)
    print()

    total_changes = 0
    for rel_path in files_to_fix:
        filepath = base_path / rel_path
        if not filepath.exists():
            print(f"文件不存在: {filepath}")
            continue

        try:
            with open(filepath, 'r', encoding='utf-8') as f:
                content = f.read()

            new_content, count = fix_transport_request(content)

            if count > 0:
                with open(filepath, 'w', encoding='utf-8') as f:
                    f.write(new_content)
                print(f"[修复] {filepath}: {count} 处更改")
                total_changes += count
            else:
                print(f"  - 无需修改: {filepath}")

        except Exception as e:
            print(f"[错误] {filepath}: {e}")

    print()
    print(f"总计: {total_changes} 处更改")

if __name__ == '__main__':
    main()
