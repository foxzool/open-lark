#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
修复所有剩余的非标准格式文档URL

自动处理所有不使用标准格式的URL，将其替换为官方API数据中的正确URL
"""

import csv
import re
import os
from pathlib import Path
from typing import Dict, List, Tuple, Optional
import json

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

            # 创建API标识符
            api_key = f"{module}:{method}:{endpoint}"

            if api_key not in official_apis:
                official_apis[api_key] = {
                    'module': module,
                    'method': method,
                    'endpoint': endpoint,
                    'doc_url': doc_url,
                    'version': version,
                    'name': name,
                    'description': description
                }

    print(f"✅ 成功加载官方API数据: {len(official_apis)} 个端点")
    return official_apis

def extract_url_pattern(url: str) -> Optional[str]:
    """从URL中提取API路径模式"""
    # 匹配各种URL格式
    patterns = [
        r'open\.feishu\.cn/document/([^/]+(?:/v\d+)?)?/([^/]+)',
        r'document/([^/]+)/([^/]+)',
        r'([^/]+)/([^/]+)/([^/]+)$'  # module/version/endpoint 格式
    ]

    for pattern in patterns:
        match = re.search(pattern, url)
        if match:
            return match.group(0)

    return None

def guess_api_info_from_url(url: str, file_path: Path) -> Dict:
    """从URL和文件路径推断API信息"""
    # 提取文件路径信息
    path_parts = file_path.parts
    module = None
    service = None

    # 尝试从路径中提取模块名
    for i, part in enumerate(path_parts):
        if part == "service" and i + 1 < len(path_parts):
            module = path_parts[i + 1]
            if i + 2 < len(path_parts):
                service = path_parts[i + 2]
            break

    # 从URL中提取API路径
    url_path = ""
    if "document/" in url:
        url_path = url.split("document/")[-1]
    elif "/document/" in url:
        url_path = url.split("/document/")[-1]

    return {
        'module': module,
        'service': service,
        'url_path': url_path,
        'original_url': url
    }

def find_best_match(api_info: Dict, official_apis: Dict) -> Optional[str]:
    """为API信息找到最佳匹配的官方URL"""
    url_path = api_info['url_path']
    module = api_info.get('module', '')

    # 分解URL路径为关键字
    path_keywords = [kw for kw in url_path.split('/') if kw and len(kw) > 2]

    best_match = None
    best_score = 0

    for api_key, api_data in official_apis.items():
        score = 0

        # 模块匹配
        if api_data['module'].lower() == module.lower():
            score += 10

        # 路径关键字匹配
        for keyword in path_keywords:
            if keyword.lower() in api_data['endpoint'].lower():
                score += 5
            if keyword.lower() in api_data['doc_url'].lower():
                score += 3

        # API名称匹配
        for keyword in path_keywords:
            if keyword.lower() in api_data['name'].lower():
                score += 4
            if keyword.lower() in api_data['description'].lower():
                score += 2

        if score > best_score:
            best_score = score
            best_match = api_data['doc_url']

    # 设置最低分数阈值
    if best_score >= 3:
        return best_match

    return None

def fix_urls_in_file(file_path: Path, official_apis: Dict) -> int:
    """修复单个文件中的URL"""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
    except Exception as e:
        print(f"❌ 读取文件失败 {file_path}: {e}")
        return 0

    # 查找所有需要修复的URL模式
    url_patterns = [
        r'https://open\.feishu\.cn/document/[^/]+/[^/\s>]+(?:/v\d+)?/[^/\s>]+',
        r'https://open\.feishu\.cn/document/([^/]+)/([^/\s>]+)',
        r'https://open\.feishu\.cn/document/([^/]+)/([^/]+)/([^/\s>]+)'
    ]

    content_modified = content
    fixes_made = 0
    processed_urls = set()  # 避免重复处理同一个URL

    for pattern in url_patterns:
        matches = list(re.finditer(pattern, content))

        # 按URL长度降序处理，避免短URL匹配长URL的一部分
        matches.sort(key=lambda m: len(m.group(0)), reverse=True)

        for match in matches:
            old_url = match.group(0)

            if old_url in processed_urls:
                continue

            processed_urls.add(old_url)

            # 跳过已经是标准格式的URL
            if 'uAjLw4CM' in old_url or 'ukTMukTMukTM' in old_url:
                continue

            api_info = guess_api_info_from_url(old_url, file_path)
            correct_url = find_best_match(api_info, official_apis)

            if correct_url and old_url in content_modified:
                content_modified = content_modified.replace(old_url, correct_url)
                fixes_made += 1
                print(f"   ✅ 修复: {old_url[:60]}... → {correct_url[:60]}...")
            else:
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

def find_files_with_non_standard_urls():
    """查找包含非标准URL的文件"""
    base_path = Path(__file__).parent.parent / "src" / "service"
    files_to_fix = []

    for root, dirs, files in os.walk(base_path):
        for file in files:
            if file.endswith('.rs') and not file.endswith('.bak'):
                file_path = Path(root) / file
                try:
                    with open(file_path, 'r', encoding='utf-8') as f:
                        content = f.read()
                        # 查找包含open.feishu.cn/document但不是标准格式的URL
                        if ('https://open.feishu.cn/document' in content and
                            'uAjLw4CM' not in content and
                            'server-docs' not in content):
                            files_to_fix.append(file_path)
                except Exception as e:
                    print(f"❌ 读取文件失败 {file_path}: {e}")

    return files_to_fix

def main():
    """主函数"""
    print("🔧 修复所有剩余的非标准格式文档URL")
    print("=" * 60)

    # 1. 加载官方API数据
    official_apis = load_official_apis()
    if not official_apis:
        return

    # 2. 查找需要修复的文件
    print(f"\n🔍 查找包含非标准URL的文件...")
    files_to_fix = find_files_with_non_standard_urls()

    if not files_to_fix:
        print("✅ 没有找到需要修复的文件")
        return

    print(f"📁 找到 {len(files_to_fix)} 个文件需要处理")

    # 3. 批量处理文件
    total_fixes = 0
    processed_files = 0

    for file_path in files_to_fix:
        print(f"\n🔧 处理文件: {file_path.relative_to(Path(__file__).parent.parent)}")
        fixes = fix_urls_in_file(file_path, official_apis)
        total_fixes += fixes
        if fixes > 0:
            processed_files += 1

    print(f"\n🎉 批量修复完成！")
    print(f"📁 处理文件数: {processed_files}")
    print(f"🔧 修复URL数: {total_fixes}")

    # 4. 验证结果
    print(f"\n🔍 验证修复结果...")
    try:
        result = os.popen('python3 scripts/doc_url_checker_simple.py --no-url-check | grep "格式错误" | head -1').read()
        if result.strip():
            print(f"   📊 当前格式错误状态: {result.strip()}")

        # 获取详细统计
        stats = os.popen('python3 scripts/doc_url_checker_simple.py --no-url-check | tail -10').read()
        if "覆盖率" in stats:
            for line in stats.split('\n'):
                if '覆盖率' in line:
                    print(f"   📈 {line.strip()}")
    except:
        print(f"   ⚠️  无法获取验证信息")

if __name__ == "__main__":
    main()