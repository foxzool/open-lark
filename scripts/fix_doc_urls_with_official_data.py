#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
基于官方API数据修正文档URL的工具

使用官方official_apis_by_module.csv数据，修正代码中错误的文档URL，
确保所有文档URL都来自官方数据源。
"""

import csv
import re
import os
import sys
from pathlib import Path

def load_official_apis():
    """加载官方API数据"""
    official_apis = {}

    csv_path = Path(__file__).parent.parent / "reports" / "official_apis_by_module.csv"
    if not csv_path.exists():
        print(f"❌ 错误: 找不到官方API数据文件 {csv_path}")
        return {}

    with open(csv_path, 'r', encoding='utf-8') as f:
        reader = csv.DictReader(f)
        for row in reader:
            module = row['模块']
            method = row['HTTP方法']
            endpoint = row['端点路径']
            doc_url = row['文档URL']
            version = row['版本']

            if module not in official_apis:
                official_apis[module] = []

            official_apis[module].append({
                'method': method,
                'endpoint': endpoint,
                'doc_url': doc_url,
                'version': version,
                'name': row['API名称'],
                'description': row['描述']
            })

    print(f"✅ 成功加载官方API数据: {len(official_apis)} 个模块")
    for module, apis in official_apis.items():
        print(f"   {module}: {len(apis)} 个API")

    return official_apis

def extract_endpoint_from_code(file_path):
    """从Rust代码中提取API端点路径"""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()

        # 查找API路径的模式
        patterns = [
            r'api_path:\s*([A-Z_]+\.to_string\(\)|"[^"]+")',
            r'EndpointBuilder::replace_param\([^,]+,\s*"[^"]*",\s*([^)]+)\)',
            r'path:\s*["\']([^"\']+)["\']',
            r'url:\s*["\']([^"\']+)["\']',
        ]

        endpoints = []
        for pattern in patterns:
            matches = re.findall(pattern, content)
            for match in matches:
                if match.startswith('APPROVAL_V4_') or match.startswith('HELPDESK_') or match.startswith('HIRE_V1_'):
                    # 处理常量替换
                    endpoint = match.replace('APPROVAL_V4_', '/open-apis/approval/v4/')
                    endpoint = endpoint.replace('HELPDESK_V1_', '/open-apis/helpdesk/v1/')
                    endpoint = endpoint.replace('HIRE_V1_', '/open-apis/hire/v1/')
                    endpoints.append(endpoint)
                else:
                    endpoints.append(match)

        return list(set(endpoints))  # 去重

    except Exception as e:
        print(f"❌ 读取文件失败 {file_path}: {e}")
        return []

def find_rust_files_with_doc_urls():
    """查找包含文档URL的Rust文件"""
    rust_files = []
    src_path = Path(__file__).parent.parent / "src"

    for rust_file in src_path.rglob("*.rs"):
        try:
            with open(rust_file, 'r', encoding='utf-8') as f:
                content = f.read()
                if 'https://open.feishu.cn/document/' in content:
                    rust_files.append(rust_file)
        except Exception:
            continue

    return rust_files

def extract_doc_urls_from_file(file_path):
    """从文件中提取所有文档URL"""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()

        # 查找所有文档URL
        urls = re.findall(r'https://open\.feishu\.cn/document/[^\\s\\)]+', content)
        return urls
    except Exception:
        return []

def analyze_file(file_path, official_apis):
    """分析单个文件，找出需要修正的URL"""
    doc_urls = extract_doc_urls_from_file(file_path)
    endpoints = extract_endpoint_from_code(file_path)

    issues = []

    for url in doc_urls:
        # 检查URL是否在官方数据中
        found_in_official = False
        for module, apis in official_apis.items():
            for api in apis:
                if api['doc_url'] == url:
                    found_in_official = True
                    break
            if found_in_official:
                break

        if not found_in_official:
            issues.append({
                'type': 'unofficial_url',
                'url': url,
                'file': str(file_path)
            })

    # 检查是否有已实现的API缺少文档URL
    for endpoint in endpoints:
        # 找到对应的官方API
        matching_api = None
        for module, apis in official_apis.items():
            for api in apis:
                if api['endpoint'] == endpoint:
                    matching_api = api
                    break
            if matching_api:
                break

        if matching_api:
            # 检查代码中是否包含对应的文档URL
            if matching_api['doc_url'] not in doc_urls:
                issues.append({
                    'type': 'missing_official_url',
                    'endpoint': endpoint,
                    'official_url': matching_api['doc_url'],
                    'module': matching_api.get('module', 'unknown'),
                    'file': str(file_path)
                })

    return issues

def generate_fix_script(issues):
    """生成修正脚本"""
    script_lines = [
        "#!/bin/bash",
        "# 自动修正文档URL脚本",
        "# 基于官方API数据生成",
        "",
        "echo '🚀 开始修正文档URL...'",
        ""
    ]

    # 按文件分组
    files_issues = {}
    for issue in issues:
        file_path = issue['file']
        if file_path not in files_issues:
            files_issues[file_path] = []
        files_issues[file_path].append(issue)

    for file_path, file_issues in files_issues.items():
        script_lines.append(f"# 修复文件: {file_path}")
        for issue in file_issues:
            if issue['type'] == 'unofficial_url':
                script_lines.append(f"#   替换非官方URL: {issue['url']}")
            elif issue['type'] == 'missing_official_url':
                script_lines.append(f"#   添加官方URL: {issue['official_url']}")
        script_lines.append("")

    # 写入脚本文件
    script_path = Path(__file__).parent / "fix_doc_urls.sh"
    with open(script_path, 'w', encoding='utf-8') as f:
        f.write('\n'.join(script_lines))

    os.chmod(script_path, 0o755)
    print(f"✅ 生成修正脚本: {script_path}")

def main():
    """主函数"""
    print("🔍 基于官方API数据修正文档URL")
    print("=" * 50)

    # 1. 加载官方API数据
    official_apis = load_official_apis()
    if not official_apis:
        return

    # 2. 查找包含文档URL的Rust文件
    rust_files = find_rust_files_with_doc_urls()
    print(f"📁 找到 {len(rust_files)} 个包含文档URL的Rust文件")

    # 3. 分析所有文件
    all_issues = []
    for rust_file in rust_files:
        print(f"🔍 分析: {rust_file}")
        issues = analyze_file(rust_file, official_apis)
        if issues:
            all_issues.extend(issues)
            print(f"   发现 {len(issues)} 个问题")

    # 4. 统计结果
    print("\n📊 分析结果:")
    unofficial_count = len([i for i in all_issues if i['type'] == 'unofficial_url'])
    missing_count = len([i for i in all_issues if i['type'] == 'missing_official_url'])

    print(f"   非官方URL: {unofficial_count} 个")
    print(f"   缺失官方URL: {missing_count} 个")
    print(f"   总问题数: {len(all_issues)} 个")

    # 5. 显示详细问题
    if all_issues:
        print("\n📋 详细问题列表:")
        for i, issue in enumerate(all_issues[:20], 1):  # 只显示前20个
            if issue['type'] == 'unofficial_url':
                print(f"   {i}. 非官方URL: {issue['url']}")
                print(f"      文件: {issue['file']}")
            elif issue['type'] == 'missing_official_url':
                print(f"   {i}. 缺失官方URL: {issue['endpoint']}")
                print(f"      应为: {issue['official_url']}")
                print(f"      模块: {issue['module']}")
                print(f"      文件: {issue['file']}")

    # 6. 生成修正建议
    print("\n💡 修正建议:")
    print("1. 使用官方API数据替换所有非官方URL")
    print("2. 为已实现但缺少文档的API添加正确的官方URL")
    print("3. 重点处理高频使用模块")

if __name__ == "__main__":
    main()