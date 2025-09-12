#!/usr/bin/env python3
"""
批量修复查询参数键中剩余的.to_string()调用
"""

import os
import re
import subprocess

# 常见的查询参数名称，需要移除.to_string()
QUERY_PARAMS = [
    "access_type", "activation_status", "activity_status", "app_id", "approval_code",
    "approval_id", "apps_type", "archive_time", "batch_id", "biz_entity_id", "biz_type",
    "category_id", "chat_id", "create_time_end", "create_time_start", "created_by_type",
    "custom_workplace_id", "department_id", "department_id_type", "doc_id", "employee_id", 
    "end_date", "end_time", "field_names", "file_id", "file_name", "file_type", "folder_id",
    "grant_type", "group_id", "image_type", "include_members", "instance_code", 
    "instance_status", "job_id", "keyword", "language", "limit", "locale", "max_results",
    "meeting_id", "message_id", "msg_type", "name", "offset", "order", "page",
    "page_size", "page_token", "receive_id_type", "recording_id", "room_id", "rule_type",
    "scope", "sort", "start_date", "start_time", "status", "task_status", "title",
    "type", "update_time_end", "update_time_start", "user_id", "user_id_type", "widget_id",
    # 新增更多参数
    "container_id", "container_id_type", "with_user_id", "direction", "text_type",
    "space_id", "node_type", "view_type", "table_id", "record_id", "with_shared_url",
    "with_revision", "export_type", "token", "range", "valueInputOption", "majorDimension",
    "dimension", "render_type", "dateTimeRenderOption", "user_id_format", "spreadsheetToken",
    "includeGridData", "fields", "filter", "interval", "unique", "search_key", "format",
    # 更多参数补充
    "relation", "is_delete", "only_avatar", "parent_type", "parent_id", "need_url",
    "thumb_size", "extra", "version", "mode", "size", "preview_type", "check_preview_permission",
    "width", "height", "count", "level", "entity_type", "classification_filter", "highlight",
    "lang", "repo_id", "owner_ids", "user_access_token", "direction", "sort_type", "ownership",
    "drive_type"
]

def fix_file(file_path):
    """修复单个文件中的查询参数"""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original_content = content
        
        # 修复每个查询参数的.to_string()调用
        for param in QUERY_PARAMS:
            # 匹配模式：.insert("param_name".to_string(), value)
            pattern = rf'\.insert\("{re.escape(param)}"\.to_string\(\)'
            replacement = f'.insert("{param}"'
            content = re.sub(pattern, replacement, content)
        
        # 如果内容有变化，写回文件
        if content != original_content:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(content)
            print(f"修复: {file_path}")
            return True
    
    except Exception as e:
        print(f"处理 {file_path} 时出错: {e}")
        return False
    
    return False

def main():
    """主函数"""
    fixed_files = 0
    
    # 遍历src目录下的所有.rs文件
    for root, dirs, files in os.walk('src'):
        for file in files:
            if file.endswith('.rs'):
                file_path = os.path.join(root, file)
                if fix_file(file_path):
                    fixed_files += 1
    
    print(f"\n共修复了 {fixed_files} 个文件")
    
    # 测试编译
    print("\n开始测试编译...")
    result = subprocess.run(['cargo', 'check'], capture_output=True, text=True)
    
    if result.returncode == 0:
        print("✅ 编译成功！")
    else:
        print("❌ 还有编译错误:")
        # 只显示前20行错误
        error_lines = result.stderr.split('\n')[:20]
        print('\n'.join(error_lines))

if __name__ == "__main__":
    main()