#!/usr/bin/env python3
"""
批量修复旧代码文件中的 execute_with_options 方法
1. 添加 Some(option) 参数
2. 添加返回语句
"""

import re
from pathlib import Path

def fix_execute_with_options(content: str, filepath: str) -> tuple:
    """
    修复 execute_with_options 方法
    """
    count = 0

    # 模式1: Transport::request(..., &self.config, ).await?;
    # 修复为: Transport::request(..., &self.config, Some(option)).await?;
    # 并添加: extract_response_data(response, "操作");

    # 查找 execute_with_options 方法
    pattern = r'(pub async fn execute_with_options\([^}]+?Transport::request\([^,]+,\s*&self\.config,\s*)\)(\.await\?;\s*)(\s*\})'

    def replace_fn(match):
        nonlocal count
        count += 1

        transport_call = match.group(1)  # Transport::request(..., &self.config,
        await_part = match.group(2)      # ).await?;
        closing = match.group(3)          # 空白 + }

        # 从文件路径推断操作名称
        if 'subscribe' in filepath.lower():
            context = "订阅文件"
        elif 'create' in filepath.lower() and 'folder' in filepath.lower():
            context = "创建文件夹"
        elif 'create' in filepath.lower():
            context = "创建"
        elif 'delete' in filepath.lower():
            context = "删除"
        elif 'update' in filepath.lower() or 'patch' in filepath.lower():
            context = "更新"
        elif 'get' in filepath.lower() or 'list' in filepath.lower():
            context = "获取"
        elif 'upload' in filepath.lower():
            context = "上传"
        elif 'download' in filepath.lower():
            context = "下载"
        elif 'search' in filepath.lower():
            context = "搜索"
        elif 'transfer' in filepath.lower():
            context = "转移"
        elif 'auth' in filepath.lower():
            context = "授权"
        elif 'export' in filepath.lower():
            context = "导出"
        elif 'import' in filepath.lower():
            context = "导入"
        elif 'permission' in filepath.lower():
            context = "权限"
        else:
            context = "操作"

        return f'{transport_call}Some(option)){await_part}        extract_response_data(response, "{context}"){closing}'

    new_content = re.sub(pattern, replace_fn, content, flags=re.DOTALL)

    return new_content, count

def main():
    base_path = Path("/Users/zool/RustroverProjects/open-lark")

    # 需要修复的文件列表
    files_to_fix = [
        "crates/openlark-docs/src/ccm/drive/v1/file/subscribe.rs",
        "crates/openlark-docs/src/ccm/drive/v1/file/get_subscribe.rs",
        "crates/openlark-docs/src/ccm/drive/v1/file/delete_subscribe.rs",
        "crates/openlark-docs/src/ccm/drive/v1/file/task_check.rs",
        "crates/openlark-docs/src/ccm/drive/v1/file/version/create.rs",
        "crates/openlark-docs/src/ccm/drive/v1/file/version/delete.rs",
        "crates/openlark-docs/src/ccm/drive/v1/file/version/get.rs",
        "crates/openlark-docs/src/ccm/drive/v1/file/version/list.rs",
        "crates/openlark-docs/src/ccm/drive/v1/export_task/create.rs",
        "crates/openlark-docs/src/ccm/drive/v1/export_task/get.rs",
        "crates/openlark-docs/src/ccm/drive/v1/import_task/create.rs",
        "crates/openlark-docs/src/ccm/drive/v1/import_task/get.rs",
        "crates/openlark-docs/src/ccm/drive/v1/media/upload_prepare.rs",
        "crates/openlark-docs/src/ccm/drive/v1/media/upload_part.rs",
        "crates/openlark-docs/src/ccm/drive/v1/media/upload_finish.rs",
        "crates/openlark-docs/src/ccm/drive/v1/media/upload_all.rs",
        "crates/openlark-docs/src/ccm/drive/v1/media/batch_get_tmp_download_url.rs",
        "crates/openlark-docs/src/ccm/drive/v1/permission/member/auth.rs",
        "crates/openlark-docs/src/ccm/drive/v1/permission/member/list.rs",
        "crates/openlark-docs/src/ccm/drive/v1/permission/member/create.rs",
        "crates/openlark-docs/src/ccm/drive/v1/permission/member/batch_create.rs",
        "crates/openlark-docs/src/ccm/drive/v1/permission/member/update.rs",
        "crates/openlark-docs/src/ccm/drive/v1/permission/member/delete.rs",
        "crates/openlark-docs/src/ccm/drive/v1/permission/member/transfer_owner.rs",
        "crates/openlark-docs/src/ccm/drive/v1/permission/public/get.rs",
        "crates/openlark-docs/src/ccm/drive/v1/permission/public/patch.rs",
        "crates/openlark-docs/src/ccm/drive/v1/permission/public/password/create.rs",
        "crates/openlark-docs/src/ccm/drive/v1/permission/public/password/update.rs",
        "crates/openlark-docs/src/ccm/drive/v1/permission/public/password/delete.rs",
        "crates/openlark-docs/src/ccm/drive/v2/file/like/list.rs",
        "crates/openlark-docs/src/ccm/docx/v1/document/get.rs",
        "crates/openlark-docs/src/ccm/docx/v1/document/create.rs",
        "crates/openlark-docs/src/ccm/docx/v1/chat/announcement/get.rs",
        "crates/openlark-docs/src/ccm/wiki/v1/node/search.rs",
        "crates/openlark-docs/src/ccm/wiki/v2/task/get.rs",
        "crates/openlark-docs/src/ccm/wiki/v2/space/node/move_docs_to_wiki.rs",
        "crates/openlark-docs/src/minutes/minutes/v1/minute/get.rs",
        "crates/openlark-docs/src/minutes/minutes/v1/minute/media/get.rs",
        "crates/openlark-docs/src/minutes/minutes/v1/minute/statistics/get.rs",
        "crates/openlark-docs/src/ccm/ccm_drive_permission/old/default/member/transfer.rs",
        "crates/openlark-docs/src/ccm/ccm_drive_permission/old/default/member/permitted.rs",
        "crates/openlark-docs/src/ccm/ccm_drive_permission/old/default/v2/public.rs",
        # ccm_drive_explorer old files
        "crates/openlark-docs/src/ccm/ccm_drive_explorer/old/default/v2/file/docs/_doc_token.rs",
        "crates/openlark-docs/src/ccm/ccm_drive_explorer/old/default/v2/file/spreadsheets/_spreadsheet_token.rs",
        "crates/openlark-docs/src/ccm/ccm_drive_explorer/old/default/v2/folder/_folder_token.rs",
        "crates/openlark-docs/src/ccm/ccm_drive_explorer/old/default/v2/folder/_folder_token/children.rs",
        "crates/openlark-docs/src/ccm/ccm_drive_explorer/old/default/v2/folder/_folder_token/meta.rs",
        "crates/openlark-docs/src/ccm/ccm_drive_explorer/old/default/v2/root_folder/meta.rs",
    ]

    print("=" * 80)
    print("批量修复旧代码文件")
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

            new_content, count = fix_execute_with_options(content, rel_path)

            if count > 0:
                with open(filepath, 'w', encoding='utf-8') as f:
                    f.write(new_content)
                print(f"[修复] {rel_path}: {count} 处更改")
                total_changes += count
            else:
                print(f"  - 无需修改: {rel_path}")

        except Exception as e:
            print(f"[错误] {rel_path}: {e}")
            import traceback
            traceback.print_exc()

    print()
    print(f"总计: {total_changes} 处更改")

if __name__ == '__main__':
    main()
