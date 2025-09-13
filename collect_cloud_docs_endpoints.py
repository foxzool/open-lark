#!/usr/bin/env python3
"""
收集CloudDocs模块中的所有API端点
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

def generate_endpoint_constants(all_endpoints):
    """生成端点常量定义"""
    constants = []
    
    # 按子模块分组
    grouped = defaultdict(list)
    
    for endpoint in sorted(all_endpoints):
        # 解析端点以确定分组
        parts = endpoint.split('/')
        if len(parts) >= 4:  # /open-apis/service/version/...
            service = parts[2]
            version = parts[3] if parts[3].startswith('v') else None
            
            # 生成常量名称
            const_name = endpoint.replace('/open-apis/', '').replace('/', '_').replace('-', '_').upper()
            const_name = re.sub(r'\{[^}]+\}', 'ID', const_name)  # 替换路径参数
            
            grouped[service].append((const_name, endpoint))
    
    # 生成常量定义
    for service, service_endpoints in sorted(grouped.items()):
        constants.append(f"\n    // ==================== {service.upper()} 服务端点 ====================\n")
        
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
    print("\n=== 生成的端点常量 ===")
    constants_code = generate_endpoint_constants(all_endpoints)
    
    # 保存到文件
    output_file = "/Users/zool/RustroverProjects/open-lark/cloud_docs_endpoints.rs"
    with open(output_file, 'w', encoding='utf-8') as f:
        f.write("// CloudDocs模块端点常量\n")
        f.write("// 自动生成，用于Phase 2.4.4a优化任务\n")
        f.write(constants_code)
    
    print(f"端点常量已保存到: {output_file}")
    
    # 显示一些示例端点
    print("\n=== 示例端点 ===")
    for i, endpoint in enumerate(sorted(all_endpoints)[:10]):
        print(f"  {endpoint}")
    
    if len(all_endpoints) > 10:
        print(f"  ... 还有 {len(all_endpoints) - 10} 个端点")

if __name__ == "__main__":
    main()