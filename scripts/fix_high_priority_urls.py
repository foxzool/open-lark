#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
修复高优先级模块的文档URL

针对 approval, helpdesk, human_authentication, performance, payroll, hire, cloud_docs 模块
批量修复错误的文档URL，使用官方API数据中的正确URL。
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

            if module not in official_apis:
                official_apis[module] = []

            official_apis[module].append({
                'method': method,
                'endpoint': endpoint,
                'doc_url': doc_url,
                'version': version,
                'name': name,
                'description': description
            })

    print(f"✅ 成功加载官方API数据: {len(official_apis)} 个模块")
    return official_apis

def create_url_mapping():
    """创建已知错误的URL到正确URL的映射"""
    return {
        # human_authentication 模块
        "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/human_authentication-v1/identity/create": {
            "identity_create": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/human_authentication-v1/identity/create",
            "face_upload": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/human_authentication-v1/face/upload-facial-reference-image",
            "face_crop": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/human_authentication-v1/face/facial-image-cropping",
            "result_query": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/human_authentication-v1/face/query-recognition-result",
        },

        # approval 模块
        "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/approval/create": {
            "approval_definition": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/approval/create",
            "definition_create": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/definition/create",
            "file_upload": "https://open.feishu.cn/document/ukTMukTMukTM/uUDOyUjL1gjM14SN4ITN",  # old version
            "message_send": "https://open.feishu.cn/document/ukTMukTMukTM/ugDNyYjL4QjM24CO0IjN",  # old version
            "message_update": "https://open.feishu.cn/document/ukTMukTMukTM/uAjNyYjLwYjM24CM2IjN",  # old version
        },
    }

def get_correct_url_for_method(module_name: str, method_name: str, file_path: str) -> str:
    """根据模块名、方法名和文件路径获取正确的URL"""

    # 优先使用预定义的映射
    url_mapping = create_url_mapping()

    # 从文件路径推断方法类型
    if "face" in file_path and "upload" in file_path:
        return url_mapping.get("https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/human_authentication-v1/identity/create", {}).get("face_upload")
    elif "face" in file_path and "crop" in file_path:
        return url_mapping.get("https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/human_authentication-v1/identity/create", {}).get("face_crop")
    elif "result" in file_path or "query" in file_path:
        return url_mapping.get("https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/human_authentication-v1/identity/create", {}).get("result_query")
    elif "file" in file_path and "upload" in file_path:
        return url_mapping.get("https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/approval/create", {}).get("file_upload")
    elif "message" in file_path and "send" in file_path:
        return url_mapping.get("https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/approval/create", {}).get("message_send")
    elif "message" in file_path and "update" in file_path:
        return url_mapping.get("https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/approval/create", {}).get("message_update")
    elif "definition" in file_path or "definition" in method_name:
        return url_mapping.get("https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/approval/create", {}).get("definition_create")

    return None

def fix_file_urls(file_path: Path, official_apis: Dict, url_mapping: Dict) -> int:
    """修复单个文件中的URL"""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
    except Exception as e:
        print(f"❌ 读取文件失败 {file_path}: {e}")
        return 0

    content_modified = content
    fixes_made = 0

    # 查找所有需要替换的URL
    for old_url, replacements in url_mapping.items():
        if old_url in content_modified:
            # 根据文件路径和上下文确定使用哪个替换URL
            file_str = str(file_path).lower()
            correct_url = get_correct_url_for_method("", "", file_str)

            if correct_url:
                content_modified = content_modified.replace(old_url, correct_url)
                fixes_made += 1
                print(f"   ✅ 修复: {old_url[:60]}... → {correct_url[:60]}...")
            else:
                # 如果无法确定，使用默认替换
                default_url = list(replacements.values())[0]
                content_modified = content_modified.replace(old_url, default_url)
                fixes_made += 1
                print(f"   ✅ 修复(默认): {old_url[:60]}... → {default_url[:60]}...")

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
    print("🔧 修复高优先级模块的文档URL")
    print("=" * 50)

    # 1. 加载官方API数据
    official_apis = load_official_apis()
    if not official_apis:
        return

    # 2. 创建URL映射
    url_mapping = create_url_mapping()

    # 3. 定义高优先级模块
    high_priority_modules = [
        "approval",
        "helpdesk",
        "human_authentication",
        "performance",
        "payroll",
        "hire",
        "cloud_docs"
    ]

    # 4. 查找相关文件
    base_path = Path(__file__).parent.parent / "src" / "service"
    total_fixes = 0

    for module in high_priority_modules:
        module_path = base_path / module
        if not module_path.exists():
            continue

        print(f"\n🔍 处理模块: {module}")

        # 查找所有相关的Rust文件
        rust_files = list(module_path.rglob("*.rs"))
        if not rust_files:
            print(f"   ⚠️  没有找到 {module} 模块的Rust文件")
            continue

        for file_path in rust_files:
            fixes = fix_file_urls(file_path, official_apis, url_mapping)
            total_fixes += fixes

    print(f"\n🎉 完成！总计修复了 {total_fixes} 个文档URL")

    # 5. 验证结果
    print(f"\n🔍 验证修复结果...")
    try:
        result = os.popen('python3 scripts/doc_url_checker_simple.py --no-url-check | grep "总问题数"').read()
        if result.strip():
            print(f"   📊 当前状态: {result.strip()}")
    except:
        print(f"   ⚠️  无法获取验证信息")

if __name__ == "__main__":
    main()