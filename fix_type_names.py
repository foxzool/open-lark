#!/usr/bin/env python3
"""
修正类型名称的脚本
"""
import re

type_corrections = {
    # Assistant subscription 
    "CreateSubscription": "CreateSubscriptionRequest",
    "GetSubscription": "GetSubscriptionRequest", 
    "PatchSubscription": "PatchSubscriptionRequest",
    
    # Bitable
    "CopyDashboard": "CopyDashboardRequest",
    "ListDashboard": "ListDashboardRequest",
    "BatchDeleteRecord": "BatchDeleteRecordRequest",
    
    # Comments
    "ListComments": "ListCommentsRequest",
    "ListReplies": "ListRepliesRequest", 
    "CreateComment": "CreateCommentRequest",
    "PatchComment": "PatchCommentRequest",
    "GetComment": "GetCommentRequest",
    "UpdateReply": "UpdateReplyRequest",
    "BatchQueryComments": "BatchQueryCommentsRequest",
    "DeleteReply": "DeleteReplyRequest",
    
    # App roles
    "ListAppRole": "ListAppRoleRequest",
    "CreateAppRole": "CreateAppRoleRequest",
    "UpdateAppRole": "UpdateAppRoleRequest",
    
    # Role members
    "ListRoleMember": "ListRoleMemberRequest",
    "BatchCreateRoleMember": "BatchCreateRoleMemberRequest",
    "CreateRoleMember": "CreateRoleMemberRequest",
    "DeleteRoleMember": "DeleteRoleMemberRequest",
    "BatchDeleteRoleMember": "BatchDeleteRoleMemberRequest",
    
    # Fields
    "ListField": "ListFieldRequest",
    "CreateField": "CreateFieldRequest",
    "UpdateField": "UpdateFieldRequest",
    
    # Workflows
    "ListWorkflow": "ListWorkflowRequest",
    "UpdateWorkflow": "UpdateWorkflowRequest",
    
    # Forms
    "PatchFormQuestion": "PatchFormQuestionRequest",
}

def fix_file(file_path):
    """修正单个文件"""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        modified = False
        for wrong_type, correct_type in type_corrections.items():
            if wrong_type in content:
                content = content.replace(wrong_type, correct_type)
                print(f"修复 {file_path}: {wrong_type} -> {correct_type}")
                modified = True
        
        if modified:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(content)
        
        return modified
    except Exception as e:
        print(f"错误处理文件 {file_path}: {e}")
        return False

def main():
    import glob
    rust_files = glob.glob('/Users/zool/RustroverProjects/open-lark/src/**/*.rs', recursive=True)
    
    fixed_count = 0
    for file_path in rust_files:
        if fix_file(file_path):
            fixed_count += 1
    
    print(f"修复完成！共修复 {fixed_count} 个文件")

if __name__ == "__main__":
    main()