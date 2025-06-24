#!/usr/bin/env python3
"""
自动修复UnknownRequest, UnknownResponse, UnknownService类型的脚本
"""
import os
import re
import glob

def find_correct_request_type(file_path):
    """根据文件内容找到正确的请求类型"""
    with open(file_path, 'r', encoding='utf-8') as f:
        content = f.read()
    
    # 查找 fn build(self) -> RequestType
    build_match = re.search(r'fn build\(self\) -> (\w+Request)', content)
    if build_match:
        return build_match.group(1)
    
    # 查找 pub struct XxxRequestBuilder
    builder_match = re.search(r'pub struct (\w+)RequestBuilder', content)
    if builder_match:
        builder_name = builder_match.group(1)
        request_name = builder_name.replace('Builder', '')
        return request_name
    
    return None

def find_correct_response_type(file_path):
    """根据文件内容找到正确的响应类型"""
    with open(file_path, 'r', encoding='utf-8') as f:
        content = f.read()
    
    # 查找 pub struct XxxResponse
    response_match = re.search(r'pub struct (\w+Response)\s*\{', content)
    if response_match:
        return response_match.group(1)
    
    # 查找 impl ApiResponseTrait for XxxResponse
    impl_match = re.search(r'impl ApiResponseTrait for (\w+Response)', content)
    if impl_match:
        return impl_match.group(1)
    
    return None

def find_correct_service_type(file_path):
    """根据文件内容找到正确的服务类型"""
    with open(file_path, 'r', encoding='utf-8') as f:
        content = f.read()
    
    # 查找已存在的正确服务类型
    service_match = re.search(r'crate::impl_executable_builder!\(\s*\w+RequestBuilder,\s*([^,\s]+),', content)
    if service_match:
        service_type = service_match.group(1)
        if 'Unknown' not in service_type:
            return service_type
    
    # 根据路径推断服务类型
    if 'bitable/v1/app_table_record' in file_path:
        return 'super::AppTableRecordService'
    elif 'bitable/v1/app_dashboard' in file_path:
        return 'super::AppDashboardService'
    elif 'attendance/v1' in file_path:
        return 'super::AttendanceService'
    elif 'assistant/v1' in file_path:
        return 'super::AssistantService'
    
    return None

def fix_file(file_path):
    """修复单个文件中的类型问题"""
    with open(file_path, 'r', encoding='utf-8') as f:
        content = f.read()
    
    if 'Unknown' not in content:
        return False
    
    print(f"修复文件: {file_path}")
    
    # 找到正确的类型
    request_type = find_correct_request_type(file_path)
    response_type = find_correct_response_type(file_path)
    service_type = find_correct_service_type(file_path)
    
    # 替换类型
    modified = False
    
    if request_type and 'UnknownRequest' in content:
        content = content.replace('UnknownRequest', request_type)
        print(f"  - UnknownRequest -> {request_type}")
        modified = True
    
    if response_type and 'UnknownResponse' in content:
        content = content.replace('UnknownResponse', response_type)
        print(f"  - UnknownResponse -> {response_type}")
        modified = True
    
    if service_type and 'UnknownService' in content:
        content = content.replace('UnknownService', service_type)
        print(f"  - UnknownService -> {service_type}")
        modified = True
    
    if modified:
        with open(file_path, 'w', encoding='utf-8') as f:
            f.write(content)
        return True
    
    return False

def main():
    """主函数"""
    print("开始修复Unknown类型问题...")
    
    # 找到所有包含Unknown类型的文件
    rust_files = glob.glob('/Users/zool/RustroverProjects/open-lark/src/**/*.rs', recursive=True)
    
    fixed_count = 0
    for file_path in rust_files:
        if fix_file(file_path):
            fixed_count += 1
    
    print(f"\n修复完成！共修复 {fixed_count} 个文件")

if __name__ == "__main__":
    main()