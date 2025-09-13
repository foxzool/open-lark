#!/usr/bin/env python3
"""
收集CloudDocs模块中的所有API端点（修正版本）
用于Phase 2.4.4a的字符串分配优化任务
"""

import os
import re
from collections import defaultdict
from pathlib import Path

def collect_api_endpoints():
    """收集CloudDocs模块中的所有API端点"""
    cloud_docs_path = Path("/Users/zool/RustroverProjects/open-lark/src/service/cloud_docs")
    
    endpoints = defaultdict(list)
    all_endpoints = set()
    
    # 正则表达式匹配API路径
    api_pattern = re.compile(r'"/open-apis/([^"]+)"')
    
    for rust_file in cloud_docs_path.rglob("*.rs"):
        if rust_file.name == "mod.rs":
            continue
            
        try:
            with open(rust_file, 'r', encoding='utf-8') as f:
                content = f.read()
                
            # 查找所有API路径
            matches = api_pattern.findall(content)
            
            if matches:
                # 确定子模块
                relative_path = rust_file.relative_to(cloud_docs_path)
                parts = relative_path.parts
                if len(parts) >= 1:
                    submodule = parts[0]
                else:
                    submodule = "root"
                
                for match in matches:
                    full_path = f"/open-apis/{match}"
                    endpoints[submodule].append((full_path, str(relative_path)))
                    all_endpoints.add(full_path)
                    
        except Exception as e:
            print(f"Error processing {rust_file}: {e}")
    
    return endpoints, all_endpoints

def normalize_constant_name(endpoint):
    """将端点路径转换为有效的常量名称"""
    # 移除 /open-apis/ 前缀
    path = endpoint.replace('/open-apis/', '')
    
    # 将路径参数 {xxx} 替换为更描述性的名称
    path = re.sub(r'\{app_token\}', 'APP_TOKEN', path)
    path = re.sub(r'\{table_id\}', 'TABLE_ID', path) 
    path = re.sub(r'\{record_id\}', 'RECORD_ID', path)
    path = re.sub(r'\{field_id\}', 'FIELD_ID', path)
    path = re.sub(r'\{view_id\}', 'VIEW_ID', path)
    path = re.sub(r'\{role_id\}', 'ROLE_ID', path)
    path = re.sub(r'\{member_id\}', 'MEMBER_ID', path)
    path = re.sub(r'\{dashboard_id\}', 'DASHBOARD_ID', path)
    path = re.sub(r'\{block_id\}', 'BLOCK_ID', path)
    path = re.sub(r'\{form_id\}', 'FORM_ID', path)
    path = re.sub(r'\{question_id\}', 'QUESTION_ID', path)
    path = re.sub(r'\{workflow_id\}', 'WORKFLOW_ID', path)
    path = re.sub(r'\{file_token\}', 'FILE_TOKEN', path)
    path = re.sub(r'\{folder_token\}', 'FOLDER_TOKEN', path)
    path = re.sub(r'\{token\}', 'TOKEN', path)
    path = re.sub(r'\{node_id\}', 'NODE_ID', path)
    path = re.sub(r'\{space_id\}', 'SPACE_ID', path)
    path = re.sub(r'\{node_token\}', 'NODE_TOKEN', path)
    path = re.sub(r'\{wiki_token\}', 'WIKI_TOKEN', path)
    path = re.sub(r'\{document_id\}', 'DOCUMENT_ID', path)
    path = re.sub(r'\{document_token\}', 'DOCUMENT_TOKEN', path)
    path = re.sub(r'\{revision_id\}', 'REVISION_ID', path)
    path = re.sub(r'\{block_id\}', 'BLOCK_ID', path)
    path = re.sub(r'\{spreadsheet_token\}', 'SPREADSHEET_TOKEN', path)
    path = re.sub(r'\{sheet_id\}', 'SHEET_ID', path)
    path = re.sub(r'\{range\}', 'RANGE', path)
    path = re.sub(r'\{condition_id\}', 'CONDITION_ID', path)
    path = re.sub(r'\{float_image_id\}', 'FLOAT_IMAGE_ID', path)
    path = re.sub(r'\{protect_id\}', 'PROTECT_ID', path)
    path = re.sub(r'\{filter_id\}', 'FILTER_ID', path)
    path = re.sub(r'\{filter_view_id\}', 'FILTER_VIEW_ID', path)
    path = re.sub(r'\{data_validation_id\}', 'DATA_VALIDATION_ID', path)
    
    # 处理特殊情况的占位符
    path = re.sub(r'\{[^}]*\}', 'ID', path)  # 其他未匹配的参数统一替换为ID
    
    # 转换为常量格式
    const_name = path.replace('/', '_').replace('-', '_').upper()
    
    # 清理多余的下划线
    const_name = re.sub(r'_+', '_', const_name)
    const_name = const_name.strip('_')
    
    return const_name

def generate_endpoint_constants(all_endpoints):
    """生成端点常量定义"""
    constants = []
    
    # 按服务分组
    grouped = defaultdict(list)
    
    for endpoint in sorted(all_endpoints):
        # 解析端点以确定分组
        parts = endpoint.split('/')
        if len(parts) >= 4:  # /open-apis/service/version/...
            service = parts[2]
            
            # 生成常量名称
            const_name = normalize_constant_name(endpoint)
            
            grouped[service].append((const_name, endpoint))
    
    # 生成常量定义
    for service, service_endpoints in sorted(grouped.items()):
        service_title = service.replace('_', ' ').title()
        constants.append(f"\n    // ==================== {service_title} 服务端点 ====================\n")
        
        for const_name, endpoint in service_endpoints:
            constants.append(f'    /// {endpoint}')
            constants.append(f'    pub const {const_name}: &\'static str = "{endpoint}";')
            constants.append('')
    
    return '\n'.join(constants)

def main():
    print("正在收集CloudDocs模块的API端点...")
    
    endpoints, all_endpoints = collect_api_endpoints()
    
    print(f"\n发现 {len(all_endpoints)} 个唯一的API端点")
    print(f"涉及 {len(endpoints)} 个子模块")
    
    print("\n=== 按子模块统计 ===")
    total_files = 0
    for submodule, endpoint_list in sorted(endpoints.items()):
        unique_endpoints = set(ep[0] for ep in endpoint_list)
        files = set(ep[1] for ep in endpoint_list)
        print(f"{submodule:15} : {len(unique_endpoints):3d} 个端点，{len(files):3d} 个文件")
        total_files += len(files)
    
    print(f"\n总计: {len(all_endpoints)} 个唯一端点，{total_files} 个文件")
    
    # 生成端点常量
    print("\n正在生成端点常量...")
    constants_code = generate_endpoint_constants(all_endpoints)
    
    # 保存到文件
    output_file = "/Users/zool/RustroverProjects/open-lark/cloud_docs_endpoints_v2.rs"
    with open(output_file, 'w', encoding='utf-8') as f:
        f.write("// CloudDocs模块端点常量\n")
        f.write("// 自动生成，用于Phase 2.4.4a优化任务\n")
        f.write("// 需要添加到 src/core/endpoints.rs 中\n\n")
        f.write("impl Endpoints {\n")
        f.write(constants_code)
        f.write("\n}\n")
    
    print(f"端点常量已保存到: {output_file}")
    
    # 显示一些示例端点和常量名
    print("\n=== 端点映射示例 ===")
    sample_endpoints = sorted(all_endpoints)[:10]
    for endpoint in sample_endpoints:
        const_name = normalize_constant_name(endpoint)
        print(f"  {const_name:50} => {endpoint}")
    
    if len(all_endpoints) > 10:
        print(f"  ... 还有 {len(all_endpoints) - 10} 个端点")

if __name__ == "__main__":
    main()