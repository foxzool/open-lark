#!/usr/bin/env python3
"""
修复 execute_with_options 方法中缺少的返回语句
"""

import re
from pathlib import Path

def fix_missing_return(content: str) -> tuple:
    """
    修复缺少返回语句的 execute_with_options 方法
    """
    count = 0

    # 匹配模式：Transport::request(..., Some(option)).await?;
    # 后面没有其他语句或只有闭合大括号
    pattern = r'(Transport::request\([^)]+Some\([^\)]+\)\)\.await\?;)\s*(\n\s*\})'

    def replace_fn(match):
        nonlocal count
        count += 1

        # 提取上下文信息用于 extract_response_data
        transport_call = match.group(1)

        # 尝试从周围代码推断操作名称
        context = "操作"

        return f'{transport_call}\n            extract_response_data(response, "{context}")\n        }}'

    new_content = re.sub(pattern, replace_fn, content, flags=re.DOTALL)

    return new_content, count

def fix_file_manually(filepath: Path):
    """手动修复文件"""
    with open(filepath, 'r', encoding='utf-8') as f:
        content = f.read()

    # 查找 execute_with_options 方法中缺少返回的情况
    # 模式：Transport::request(...) 后面直接是闭合大括号
    pattern = r'(pub async fn execute_with_options[\s\S]*?Transport::request\([^)]+Some\([^)]+\)\)\.await\?;)\s*\}'

    def add_return(match):
        method_content = match.group(1)

        # 检查是否已经有返回语句
        if 'return ' in method_content or 'extract_response_data' in method_content:
            return match.group(0)

        # 添加 extract_response_data 调用
        # 尝试从文件名或上下文推断操作名称
        if 'GetDocContent' in method_content:
            context = "获取文档内容"
        elif 'GetDocRawContent' in method_content:
            context = "获取文档原始内容"
        elif 'GetDocSheetMeta' in method_content:
            context = "获取文档表格元数据"
        elif 'CreateDoc' in method_content:
            context = "创建文档"
        elif 'GetDocMeta' in method_content:
            context = "获取文档元数据"
        elif 'GetMeta' in method_content:
            context = "获取元数据"
        elif 'SearchObject' in method_content:
            context = "搜索对象"
        elif 'CopyFile' in method_content:
            context = "复制文件"
        else:
            context = "操作"

        return f'{method_content}\n            extract_response_data(response, "{context}")\n    }}'

    new_content = re.sub(pattern, add_return, content, flags=re.DOTALL)

    return new_content != content, new_content

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
    print("修复 execute_with_options 方法缺少的返回语句")
    print("=" * 80)
    print()

    total_changes = 0
    for rel_path in files_to_fix:
        filepath = base_path / rel_path
        if not filepath.exists():
            print(f"文件不存在: {filepath}")
            continue

        try:
            modified, new_content = fix_file_manually(filepath)

            if modified:
                with open(filepath, 'w', encoding='utf-8') as f:
                    f.write(new_content)
                print(f"[修复] {filepath}")
                total_changes += 1
            else:
                print(f"  - 无需修改: {filepath}")

        except Exception as e:
            print(f"[错误] {filepath}: {e}")
            import traceback
            traceback.print_exc()

    print()
    print(f"总计: {total_changes} 个文件已修复")

if __name__ == '__main__':
    main()
