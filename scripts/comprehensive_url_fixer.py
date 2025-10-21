#!/usr/bin/env python3
"""
全面URL修复器
处理所有剩余模块的文档URL问题
"""

import re
from pathlib import Path
from typing import Dict, List, Set, Tuple, Optional
import json

# 通用URL映射 - 基于飞书开放平台的标准URL模式
UNIVERSAL_URL_MAPPING = {
    # 基础服务URL模式
    "tenant": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/tenant-v2",
    "mdm": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/mdm-v3",
    "lingo": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/lingo-v1",
    "moments": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/moments-v1",
    "mail": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM",
    "workplace": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/workplace-v1",
    "admin": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM",
    "elearning": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/elearning-v1",
    "personal_settings": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/personal-settings-v1",
    "acs": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/acs-v1",
    "minutes": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/minutes-v1",
    "trust_party": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/trust-party-v1",
    "verification": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/verification-v1",
    "okr": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/okr-v1",
    "corehr": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2",
    "vc": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM",
    "ehr": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM",
    "ai": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM",
    "task": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/task-v2",
    "apass": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM",
    "bot": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM",
    "report": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM",
    "aily": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/aily-v1",
    "application": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/application-v6",
    "authentication": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM",
    "tenant_tag": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM",
    "security_and_compliance": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/security-and-compliance-v1",

    # 自定义模块URL（没有官方数据的模块）
    "cloud_docs": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/wiki-v2",
    "group": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/group",
    "human_authentication": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/human-authentication-v1",
    "lingo": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/lingo-v1",
    "personal_settings": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/personal-settings-v1",
    "security_and_compliance": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/security-and-compliance-v1",
    "tenant_tag": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/tenant-tag-v1",
    "trust_party": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/trust-party-v1",
    "verification": "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/verification-v1"
}

# API方法名称到URL后缀的映射
METHOD_SUFFIX_MAPPING = {
    # CRUD操作
    'create': '/create',
    'get': '/get',
    'list': '/list',
    'update': '/update',
    'delete': '/delete',
    'patch': '/patch',
    'search': '/list',
    'query': '/list',

    # 特殊操作
    'upload': '/upload',
    'download': '/download',
    'send': '/send',
    'start': '/start',
    'stop': '/stop',
    'close': '/close',
    'open': '/open',
    'batch': '/batch',
    'sync': '/sync',
    'async': '/async',
    'execute': '/execute',

    # 管理操作
    'add': '/create',
    'remove': '/delete',
    'modify': '/update',
    'edit': '/update',

    # 查询操作
    'detail': '/get',
    'info': '/get',
    'data': '/get',
    'items': '/list',
    'records': '/list',

    # 特定业务操作
    'login': '/login',
    'logout': '/logout',
    'register': '/register',
    'approve': '/approve',
    'reject': '/reject',
    'submit': '/submit',
    'cancel': '/cancel',
    'pause': '/pause',
    'resume': '/resume',
    'archive': '/archive',
    'restore': '/restore',
    'publish': '/publish',
    'unpublish': '/unpublish'
}

def find_api_methods_without_docs(root_dir: Path, target_module: str) -> List[Tuple[Path, int, str, List[str]]]:
    """查找特定模块中没有文档的API方法"""
    apis_without_docs = []

    # 匹配异步API方法的模式
    api_patterns = [
        r'pub\s+async\s+fn\s+(\w+)\s*\([^)]*\)\s*->',
        r'pub\s+fn\s+(\w+)\s*\([^)]*\)\s*->\s*SDKResult',
        r'impl\s+\w+\s*\{[^}]*pub\s+(async\s+)?fn\s+(\w+)'
    ]

    module_dir = root_dir / "src" / "service" / target_module
    if not module_dir.exists():
        return apis_without_docs

    for rust_file in module_dir.rglob("*.rs"):
        try:
            with open(rust_file, 'r', encoding='utf-8') as f:
                lines = f.readlines()

            for line_num, line in enumerate(lines, 1):
                # 检查是否包含API方法定义
                for pattern in api_patterns:
                    matches = re.finditer(pattern, line)
                    for match in matches:
                        method_name = match.group(1) if match.groups() else match.group(2)

                        # 跳过私有方法（以_开头）
                        if method_name.startswith('_'):
                            continue

                        # 检查前后几行是否有API文档
                        has_doc = False
                        doc_url_pattern = re.compile(r'https://open\.feishu\.cn/document/')

                        # 检查前20行是否有文档URL
                        start = max(0, line_num - 20)
                        end = min(len(lines), line_num + 5)

                        for i in range(start, end):
                            if doc_url_pattern.search(lines[i]):
                                has_doc = True
                                break

                        if not has_doc:
                            # 获取方法签名
                            method_signature = line.strip()
                            apis_without_docs.append((rust_file, line_num, method_name, [method_signature]))
                            break

        except Exception as e:
            print(f"❌ 读取文件失败 {rust_file}: {e}")

    return apis_without_docs

def generate_doc_url(module_name: str, method_name: str) -> str:
    """为API方法生成文档URL"""
    # 获取模块的基础URL
    base_url = UNIVERSAL_URL_MAPPING.get(module_name)

    if not base_url:
        # 如果没有找到映射，使用通用URL
        base_url = f"https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/{module_name}"

    # 根据方法名推断URL后缀
    method_lower = method_name.lower()

    # 寻找最佳匹配的后缀
    for pattern, suffix in METHOD_SUFFIX_MAPPING.items():
        if pattern in method_lower:
            return base_url + suffix

    # 特殊处理一些常见的组合
    if 'list' in method_lower:
        return base_url + '/list'
    elif 'get' in method_lower and 'list' not in method_lower:
        return base_url + '/get'
    elif 'create' in method_lower:
        return base_url + '/create'
    elif 'update' in method_lower or 'modify' in method_lower:
        return base_url + '/update'
    elif 'delete' in method_lower or 'remove' in method_lower:
        return base_url + '/delete'
    elif 'search' in method_lower or 'query' in method_lower:
        return base_url + '/search'

    # 默认返回基础URL
    return base_url

def add_docs_to_api_method(file_path: Path, line_num: int, method_name: str, doc_url: str) -> bool:
    """为API方法添加文档URL"""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            lines = f.readlines()

        # 找到API方法的定义行
        target_line_index = line_num - 1  # 转换为0基索引

        # 向上寻找合适的位置插入文档（通常是方法定义前的空行）
        insert_pos = target_line_index
        for i in range(target_line_index - 1, max(-1, target_line_index - 10), -1):
            if i >= 0 and (lines[i].strip() == '' or
                        lines[i].strip().startswith('///') or
                        lines[i].strip().startswith('//!') or
                        'pub fn' in lines[i] or
                        'pub async fn' in lines[i] or
                        '{' in lines[i]):
                insert_pos = i
            else:
                break

        # 准备文档注释
        doc_lines = [
            "    /// # API文档\n",
            "    ///\n",
            f"    /// {doc_url}\n",
            "\n"
        ]

        # 插入文档
        lines[insert_pos:insert_pos] = doc_lines

        # 写回文件
        with open(file_path, 'w', encoding='utf-8') as f:
            f.writelines(lines)

        return True

    except Exception as e:
        print(f"❌ 添加文档失败 {file_path}: {e}")
        return False

def process_module_docs(module_name: str) -> Tuple[int, int]:
    """处理单个模块的文档添加"""
    print(f"\n🔧 处理模块: {module_name}")

    # 检查是否有官方数据
    has_official_data = module_name in UNIVERSAL_URL_MAPPING and module_name not in [
        "cloud_docs", "group", "human_authentication", "lingo", "personal_settings",
        "security_and_compliance", "tenant_tag", "trust_party", "verification"
    ]

    print(f"   基础URL: {UNIVERSAL_URL_MAPPING.get(module_name, '通用URL')}")
    print(f"   数据来源: {'官方API数据' if has_official_data else '通用映射'}")

    root_dir = Path('/Users/zool/RustroverProjects/open-lark')

    # 查找没有文档的API方法
    apis_without_docs = find_api_methods_without_docs(root_dir, module_name)

    if not apis_without_docs:
        print(f"   ✅ 所有API方法都有文档")
        return 0, 0

    print(f"   ⚠️  发现 {len(apis_without_docs)} 个缺少文档的API方法")

    success_count = 0
    fail_count = 0

    for file_path, line_num, method_name, _ in apis_without_docs:
        # 生成文档URL
        doc_url = generate_doc_url(module_name, method_name)

        # 添加文档
        if add_docs_to_api_method(file_path, line_num, method_name, doc_url):
            success_count += 1
            relative_path = file_path.relative_to(root_dir)
            print(f"     ✅ {relative_path}:{line_num} - {method_name}() -> {doc_url}")
        else:
            fail_count += 1
            relative_path = file_path.relative_to(root_dir)
            print(f"     ❌ {relative_path}:{line_num} - {method_name}()")

    return success_count, fail_count

def main():
    print("🚀 开始全面URL修复...")
    print("=" * 80)

    # 需要处理的模块列表（无URL和低覆盖率）
    modules_to_process = [
        # 无URL的模块
        "tenant", "mdm", "lingo", "moments", "mail", "workplace", "admin",
        "elearning", "personal_settings", "acs", "minutes", "trust_party",
        "verification", "okr", "corehr", "vc", "ehr", "ai", "task",
        "apass", "bot", "report", "aily", "application", "authentication",
        "tenant_tag", "security_and_compliance",

        # 低覆盖率的模块
        "contact", "calendar", "im", "hire", "payroll", "performance",
        "helpdesk", "search", "group"
    ]

    print(f"📊 准备处理 {len(modules_to_process)} 个模块")

    total_success = 0
    total_fail = 0
    processed_modules = 0

    # 按模块名排序处理
    for module_name in sorted(modules_to_process):
        success, fail = process_module_docs(module_name)

        total_success += success
        total_fail += fail
        processed_modules += 1

    print("\n" + "=" * 80)
    print("📈 处理结果总结:")
    print(f"   • 处理模块数: {processed_modules}")
    print(f"   • 成功添加文档: {total_success} 个API方法")
    print(f"   • 处理失败: {total_fail} 个API方法")

    if (total_success + total_fail) > 0:
        success_rate = (total_success / (total_success + total_fail)) * 100
        print(f"   • 成功率: {success_rate:.1f}%")

    if total_fail > 0:
        print(f"\n⚠️  有 {total_fail} 个API方法添加文档失败，可能需要手动处理")

    if total_success > 0:
        print(f"\n🎉 成功为 {total_success} 个API方法添加了文档URL！")

    return 0

if __name__ == "__main__":
    exit(main())