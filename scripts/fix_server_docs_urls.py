#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
批量修复server-docs格式的文档URL

将所有server-docs格式的URL替换为从官方API数据中获取的正确URL
"""

import csv
import re
import os
from pathlib import Path
from typing import Dict, List, Tuple

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
            name = row['API名称']
            description = row['描述']

            # 创建API标识符用于匹配
            api_key = f"{method}:{endpoint}"

            if api_key not in official_apis:
                official_apis[api_key] = []

            official_apis[api_key].append({
                'module': module,
                'method': method,
                'endpoint': endpoint,
                'doc_url': doc_url,
                'version': version,
                'name': name,
                'description': description
            })

    print(f"✅ 成功加载官方API数据: {len(official_apis)} 个端点")
    return official_apis

def find_server_docs_files():
    """查找所有包含server-docs URL的文件"""
    base_path = Path(__file__).parent.parent / "src" / "service"
    files_with_server_docs = []

    for root, dirs, files in os.walk(base_path):
        for file in files:
            if file.endswith('.rs') and not file.endswith('.bak'):
                file_path = Path(root) / file
                try:
                    with open(file_path, 'r', encoding='utf-8') as f:
                        content = f.read()
                        if 'https://open.feishu.cn/document/server-docs' in content:
                            files_with_server_docs.append(file_path)
                except Exception as e:
                    print(f"❌ 读取文件失败 {file_path}: {e}")

    return files_with_server_docs

def extract_api_info_from_file(file_path: Path) -> List[Dict]:
    """从文件中提取API信息"""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
    except Exception as e:
        print(f"❌ 读取文件失败 {file_path}: {e}")
        return []

    # 查找所有server-docs URL
    server_docs_pattern = r'https://open\.feishu\.cn/document/server-docs/([^\s>]+)'
    matches = re.finditer(server_docs_pattern, content)

    apis_found = []
    for match in matches:
        old_url = match.group(0)
        api_path = match.group(1)

        # 尝试从文件路径推断API信息
        file_str = str(file_path)
        module_info = file_path.parts[-3] if len(file_path.parts) >= 3 else "unknown"
        service_info = file_path.parts[-2] if len(file_path.parts) >= 2 else "unknown"

        apis_found.append({
            'file_path': file_path,
            'old_url': old_url,
            'api_path': api_path,
            'module': module_info,
            'service': service_info,
            'line_content': content[match.start():match.end()+50]  # 包含一些上下文
        })

    return apis_found

def find_matching_official_url(api_info: Dict, official_apis: Dict) -> str:
    """为API信息查找匹配的官方URL"""
    api_path = api_info['api_path']
    module = api_info['module']

    # 根据API路径推断HTTP方法和端点
    # 这里使用启发式方法来匹配
    for api_key, api_list in official_apis.items():
        method, endpoint = api_key.split(':', 1)

        # 检查模块是否匹配
        if api_list and api_list[0]['module'].lower() == module.lower():
            # 检查API路径是否匹配
            if any(keyword in endpoint.lower() for keyword in api_path.lower().split('/')):
                return api_list[0]['doc_url']

    # 如果没有找到精确匹配，尝试模糊匹配
    for api_key, api_list in official_apis.items():
        if api_list:
            doc_url = api_list[0]['doc_url']
            # 检查URL路径是否包含相似的关键词
            if any(keyword in doc_url.lower() for keyword in api_path.lower().split('/')):
                return doc_url

    return None

def fix_file_urls(file_path: Path, apis_to_fix: List[Dict], official_apis: Dict) -> int:
    """修复单个文件中的URL"""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
    except Exception as e:
        print(f"❌ 读取文件失败 {file_path}: {e}")
        return 0

    content_modified = content
    fixes_made = 0

    # 按照URL长度降序处理，避免短URL匹配长URL的一部分
    apis_to_fix.sort(key=lambda x: len(x['old_url']), reverse=True)

    for api_info in apis_to_fix:
        old_url = api_info['old_url']
        correct_url = find_matching_official_url(api_info, official_apis)

        if correct_url and old_url in content_modified:
            content_modified = content_modified.replace(old_url, correct_url)
            fixes_made += 1
            print(f"   ✅ 修复: {old_url[:60]}... → {correct_url[:60]}...")
        elif not correct_url:
            print(f"   ⚠️  未找到匹配: {old_url[:60]}...")

    # 写回文件
    if fixes_made > 0:
        try:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(content_modified)
            print(f"   📝 {file_path.name}: {fixes_made} 个URL已修复")
        except Exception as e:
            print(f"❌ 写入文件失败 {file_path}: {e}")
            return 0

    return fixes_made

def main():
    """主函数"""
    print("🔧 批量修复server-docs格式的文档URL")
    print("=" * 50)

    # 1. 加载官方API数据
    official_apis = load_official_apis()
    if not official_apis:
        return

    # 2. 查找所有包含server-docs URL的文件
    print(f"\n🔍 查找包含server-docs URL的文件...")
    files_to_process = find_server_docs_files()

    if not files_to_process:
        print("✅ 没有找到需要修复的文件")
        return

    print(f"📁 找到 {len(files_to_process)} 个文件需要处理")

    # 3. 分析每个文件的API信息
    total_fixes = 0
    processed_files = 0

    for file_path in files_to_process:
        print(f"\n🔧 处理文件: {file_path.relative_to(Path(__file__).parent.parent)}")

        # 提取API信息
        apis_to_fix = extract_api_info_from_file(file_path)

        if not apis_to_fix:
            print("   ⚠️  未找到API信息")
            continue

        print(f"   📊 发现 {len(apis_to_fix)} 个API需要修复")

        # 修复URL
        fixes = fix_file_urls(file_path, apis_to_fix, official_apis)
        total_fixes += fixes
        processed_files += 1

    print(f"\n🎉 完成！")
    print(f"📁 处理文件数: {processed_files}")
    print(f"🔧 修复URL数: {total_fixes}")

    # 4. 验证结果
    print(f"\n🔍 验证修复结果...")
    try:
        result = os.popen('python3 scripts/doc_url_checker_simple.py --no-url-check | grep "格式错误"').read()
        if result.strip():
            print(f"   📊 当前状态: {result.strip()}")
        else:
            print("   ✅ 所有格式错误已修复")
    except:
        print(f"   ⚠️  无法获取验证信息")

if __name__ == "__main__":
    main()