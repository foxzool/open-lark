#!/usr/bin/env python3
"""
修复剩余的旧代码文件 - 处理特殊的 Transport::request 格式
"""

import re
from pathlib import Path

def fix_transport_request_special(content: str, filepath: str) -> tuple:
    """
    修复特殊的 Transport::request 格式：
    Transport::request(..., &self.config,
    ).await?;
    """
    count = 0

    # 匹配跨行的 Transport::request 调用
    pattern = r'Transport::request\(([^,]+),\s*&self\.config,\s*\)\s*\.await\?;'

    def infer_context(path):
        """从文件路径推断操作名称"""
        path_lower = path.lower()
        if 'subscribe' in path_lower:
            if 'file' in path_lower:
                return "订阅文件"
        elif 'create' in path_lower:
            if 'folder' in path_lower:
                return "创建文件夹"
            elif 'version' in path_lower:
                return "创建版本"
            return "创建"
        elif 'delete' in path_lower:
            if 'subscribe' in path_lower:
                return "删除订阅"
            return "删除"
        elif 'get' in path_lower:
            if 'subscribe' in path_lower:
                return "获取订阅信息"
            return "获取"
        elif 'list' in path_lower:
            return "列表"
        elif 'update' in path_lower or 'patch' in path_lower:
            return "更新"
        elif 'upload' in path_lower:
            return "上传"
        elif 'download' in path_lower:
            return "下载"
        elif 'search' in path_lower:
            return "搜索"
        elif 'transfer' in path_lower:
            return "转移"
        elif 'auth' in path_lower:
            return "授权"
        elif 'export' in path_lower:
            return "导出"
        elif 'import' in path_lower:
            return "导入"
        elif 'permission' in path_lower:
            return "权限"
        elif 'task' in path_lower:
            if 'check' in path_lower:
                return "任务检查"
            return "任务"
        elif 'version' in path_lower:
            return "版本"
        elif 'media' in path_lower:
            return "媒体"
        elif 'statistics' in path_lower:
            return "统计"
        elif 'announcement' in path_lower:
            return "公告"
        elif 'node' in path_lower:
            return "节点"
        elif 'wiki' in path_lower:
            return "知识库"
        elif 'minute' in path_lower:
            return "会议纪要"
        elif 'docx' in path_lower:
            return "文档"
        elif 'spreadsheet' in path_lower:
            return "电子表格"
        elif 'folder' in path_lower:
            if 'meta' in path_lower:
                return "文件夹元数据"
            if 'children' in path_lower:
                return "子文件夹"
            return "文件夹"
        return "操作"

    def replace_fn(match):
        nonlocal count
        count += 1

        request_param = match.group(1)  # 第一个参数（request 或 api_request）
        context = infer_context(filepath)

        return f'Transport::request({request_param}, &self.config, Some(option)).await?;\n        extract_response_data(response, "{context}")'

    new_content = re.sub(pattern, replace_fn, content)

    return new_content, count

def main():
    base_path = Path("/Users/zool/RustroverProjects/open-lark")

    # 需要修复的文件列表（drive/v1 下的大部分文件）
    files_to_fix = [
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
        "crates/openlark-docs/src/ccm/docx/v1/document/create.rs",
        "crates/openlark-docs/src/ccm/docx/v1/chat/announcement/get.rs",
        "crates/openlark-docs/src/ccm/wiki/v1/node/search.rs",
        "crates/openlark-docs/src/ccm/wiki/v2/task/get.rs",
        "crates/openlark-docs/src/ccm/wiki/v2/space/node/move_docs_to_wiki.rs",
        "crates/openlark-docs/src/minutes/minutes/v1/minute/get.rs",
        "crates/openlark-docs/src/minutes/minutes/v1/minute/media/get.rs",
        "crates/openlark-docs/src/minutes/minutes/v1/minute/statistics/get.rs",
        "crates/openlark-docs/src/ccm/ccm_drive_explorer/old/default/v2/folder/_folder_token/children.rs",
    ]

    print("=" * 80)
    print("修复剩余的旧代码文件")
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

            new_content, count = fix_transport_request_special(content, rel_path)

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
