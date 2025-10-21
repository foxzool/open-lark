#!/usr/bin/env python3
"""
深度API方法文档URL补充器
专门用于处理遗漏的API方法文档URL
"""

import re
from pathlib import Path
from typing import Dict, List, Set, Tuple, Optional
import json

# 更完整的API方法到URL的映射
API_METHOD_TO_URL = {
    # attendance (考勤)
    ('attendance', 'create'): 'https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/attendance-v1/user-task/create',
    ('attendance', 'get'): 'https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/attendance-v1/user-task/get',
    ('attendance', 'list'): 'https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/attendance-v1/user-task/list',
    ('attendance', 'update'): 'https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/attendance-v1/user-task/update',
    ('attendance', 'delete'): 'https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/attendance-v1/user-task/delete',

    # cloud_docs (云文档)
    ('sheets', 'create'): 'https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/bitable-v1/app-table-record/create',
    ('sheets', 'get'): 'https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/bitable-v1/app-table-record/get',
    ('sheets', 'update'): 'https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/bitable-v1/app-table-record/update',
    ('sheets', 'delete'): 'https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/bitable-v1/app-table-record/delete',
    ('sheets', 'list'): 'https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/bitable-v1/app-table-record/list',

    # search (搜索)
    ('search', 'query'): 'https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/search-v2/data-item/query',
    ('search', 'user'): 'https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/search-v1/user/search',
    ('search', 'document'): 'https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/search-v2/document/search',

    # calendar (日历)
    ('calendar', 'create'): 'https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/calendar-v4/event/create',
    ('calendar', 'get'): 'https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/calendar-v4/event/get',
    ('calendar', 'list'): 'https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/calendar-v4/event/list',
    ('calendar', 'update'): 'https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/calendar-v4/event/update',
    ('calendar', 'delete'): 'https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/calendar-v4/event/delete',

    # mail (邮件)
    ('mail', 'send'): 'https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/mail/send_mail',
    ('mail', 'get'): 'https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/mail/get',
    ('mail', 'list'): 'https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/mail/list',
    ('mail', 'reply'): 'https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/mail/reply',

    # more generic mappings for common operations
    'create': '/create',
    'get': '/get',
    'list': '/list',
    'update': '/update',
    'delete': '/delete',
    'search': '/search',
    'query': '/query',
    'send': '/send',
    'reply': '/reply',
    'approve': '/approve',
    'reject': '/reject',
    'cancel': '/cancel',
    'submit': '/submit',
    'sync': '/sync',
    'batch': '/batch',
}

# 服务模块的基础URL映射
SERVICE_BASE_URLS = {
    'attendance': 'https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/attendance-v1',
    'approval': 'https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/approval-v4',
    'cloud_docs': 'https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM',
    'sheets': 'https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/sheets-v3',
    'bitable': 'https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/bitable-v1',
    'drive': 'https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/drive/v1',
    'calendar': 'https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/calendar-v4',
    'mail': 'https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/mail',
    'search': 'https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/search-v2',
    'task': 'https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/task-v2',
    'minutes': 'https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/minutes-v1',
    'vc': 'https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM',
}

def find_api_methods_deep(root_dir: Path) -> List[Tuple[Path, int, str, List[str]]]:
    """深度查找所有API方法"""
    apis_without_docs = []

    # 匹配异步API方法的模式
    api_patterns = [
        r'pub\s+async\s+fn\s+(\w+)\s*\([^)]*\)\s*->\s*[^{]+',
        r'pub\s+fn\s+(\w+)\s*\([^)]*\)\s*->\s*SDKResult',
        r'impl\s+\w+\s*\{[^}]*pub\s+(async\s+)?fn\s+(\w+)'
    ]

    service_dir = root_dir / "src" / "service"
    if not service_dir.exists():
        return apis_without_docs

    for rust_file in service_dir.rglob("*.rs"):
        # 跳过明显不需要文档的文件
        if any(skip in str(rust_file) for skip in ['models.rs', 'mod.rs', 'types.rs', 'errors.rs']):
            continue

        try:
            with open(rust_file, 'r', encoding='utf-8') as f:
                lines = f.readlines()

            for line_num, line in enumerate(lines, 1):
                # 检查是否包含API方法定义
                for pattern in api_patterns:
                    matches = re.finditer(pattern, line)
                    for match in matches:
                        method_name = match.group(1) if match.groups() else match.group(2)

                        # 跳过私有方法和明显的非API方法
                        if method_name.startswith('_') or method_name in ['new', 'default', 'from', 'into']:
                            continue

                        # 检查前后几行是否有API文档
                        has_doc = False
                        doc_url_pattern = re.compile(r'https://open\.feishu\.cn/document/')

                        # 检查前30行是否有文档URL
                        start = max(0, line_num - 30)
                        end = min(len(lines), line_num + 5)

                        for i in range(start, end):
                            if doc_url_pattern.search(lines[i]):
                                has_doc = True
                                break

                        if not has_doc:
                            # 获取更多上下文
                            method_context = []
                            for i in range(max(0, line_num-2), min(len(lines), line_num+2)):
                                method_context.append(lines[i].strip())

                            apis_without_docs.append((rust_file, line_num, method_name, method_context))
                            break

        except Exception as e:
            print(f"❌ 读取文件失败 {rust_file}: {e}")

    return apis_without_docs

def determine_service_from_path(file_path: Path) -> str:
    """从文件路径确定服务名称"""
    parts = file_path.parts
    service_index = parts.index('service') if 'service' in parts else -1

    if service_index >= 0 and service_index + 1 < len(parts):
        service_name = parts[service_index + 1]

        # 处理特殊情况
        if 'cloud_docs' in parts:
            return 'cloud_docs'
        elif 'attendance' in parts:
            return 'attendance'
        elif 'calendar' in parts:
            return 'calendar'
        elif 'mail' in parts:
            return 'mail'
        elif 'search' in parts:
            return 'search'
        elif 'approval' in parts:
            return 'approval'
        elif 'task' in parts:
            return 'task'
        elif 'sheets' in parts:
            return 'sheets'
        elif 'bitable' in parts:
            return 'bitable'
        elif 'drive' in parts:
            return 'drive'

        return service_name

    return 'unknown'

def generate_doc_url_for_method(service_name: str, method_name: str, file_path: Path) -> str:
    """为特定API方法生成文档URL"""
    # 首先尝试精确匹配
    key = (service_name, method_name)
    if key in API_METHOD_TO_URL:
        return API_METHOD_TO_URL[key]

    # 然后尝试方法名匹配
    if method_name.lower() in API_METHOD_TO_URL:
        return SERVICE_BASE_URLS.get(service_name, f"https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/{service_name}") + API_METHOD_TO_URL[method_name.lower()]

    # 根据方法名模式推断
    method_lower = method_name.lower()
    for pattern, suffix in API_METHOD_TO_URL.items():
        if isinstance(pattern, str) and pattern in method_lower:
            return SERVICE_BASE_URLS.get(service_name, f"https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/{service_name}") + suffix

    # 默认基础URL
    base_url = SERVICE_BASE_URLS.get(service_name, f"https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/{service_name}")

    # 根据常见方法模式推断
    if 'create' in method_lower:
        return base_url + '/create'
    elif 'get' in method_lower and 'list' not in method_lower:
        return base_url + '/get'
    elif 'list' in method_lower:
        return base_url + '/list'
    elif 'update' in method_lower or 'modify' in method_lower:
        return base_url + '/update'
    elif 'delete' in method_lower or 'remove' in method_lower:
        return base_url + '/delete'
    elif 'search' in method_lower or 'query' in method_lower:
        return base_url + '/search'
    elif 'send' in method_lower:
        return base_url + '/send'
    elif 'approve' in method_lower:
        return base_url + '/approve'
    elif 'reject' in method_lower:
        return base_url + '/reject'
    elif 'cancel' in method_lower:
        return base_url + '/cancel'
    elif 'submit' in method_lower:
        return base_url + '/submit'
    elif 'sync' in method_lower:
        return base_url + '/sync'
    elif 'batch' in method_lower:
        return base_url + '/batch'

    return base_url

def add_docs_to_api_method(file_path: Path, line_num: int, method_name: str, doc_url: str) -> bool:
    """为API方法添加文档URL"""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            lines = f.readlines()

        # 找到API方法的定义行
        target_line_index = line_num - 1  # 转换为0基索引

        # 向上寻找合适的位置插入文档
        insert_pos = target_line_index
        for i in range(target_line_index - 1, max(-1, target_line_index - 15), -1):
            if i >= 0 and (lines[i].strip() == '' or
                        lines[i].strip().startswith('///') or
                        lines[i].strip().startswith('//!') or
                        ('pub fn' in lines[i] and 'async' not in lines[i]) or
                        '}' in lines[i]):
                insert_pos = i + 1
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

def process_missing_api_docs() -> Tuple[int, int, Dict[str, int]]:
    """处理所有缺少文档的API方法"""
    print("🔍 开始深度搜索缺少文档的API方法...")

    root_dir = Path('/Users/zool/RustroverProjects/open-lark')

    # 查找所有缺少文档的API方法
    apis_without_docs = find_api_methods_deep(root_dir)

    if not apis_without_docs:
        print("✅ 所有API方法都有文档！")
        return 0, 0, {}

    print(f"📊 发现 {len(apis_without_docs)} 个缺少文档的API方法")

    # 按服务分组统计
    service_stats = {}
    for file_path, line_num, method_name, _ in apis_without_docs:
        service_name = determine_service_from_path(file_path)
        if service_name not in service_stats:
            service_stats[service_name] = 0
        service_stats[service_name] += 1

    print("📈 按服务分组的统计:")
    for service, count in sorted(service_stats.items(), key=lambda x: x[1], reverse=True):
        print(f"   • {service}: {count} 个API方法")

    success_count = 0
    fail_count = 0

    # 处理每个缺少文档的API方法
    for file_path, line_num, method_name, context in apis_without_docs:
        service_name = determine_service_from_path(file_path)

        # 生成文档URL
        doc_url = generate_doc_url_for_method(service_name, method_name, file_path)

        # 添加文档
        if add_docs_to_api_method(file_path, line_num, method_name, doc_url):
            success_count += 1
            relative_path = file_path.relative_to(root_dir)
            print(f"  ✅ {relative_path}:{line_num} - {method_name}() -> {service_name}")
        else:
            fail_count += 1
            relative_path = file_path.relative_to(root_dir)
            print(f"  ❌ {relative_path}:{line_num} - {method_name}()")

    return success_count, fail_count, service_stats

def main():
    print("🚀 开始深度API方法文档URL补充...")
    print("=" * 80)

    success, fail, stats = process_missing_api_docs()

    print("\n" + "=" * 80)
    print("📈 处理结果总结:")
    print(f"   • 成功添加文档: {success} 个API方法")
    print(f"   • 处理失败: {fail} 个API方法")

    if success + fail > 0:
        success_rate = (success / (success + fail)) * 100
        print(f"   • 成功率: {success_rate:.1f}%")

    if fail > 0:
        print(f"\n⚠️  有 {fail} 个API方法添加文档失败，可能需要手动处理")

    if success > 0:
        print(f"\n🎉 成功为 {success} 个API方法添加了文档URL！")

    return 0

if __name__ == "__main__":
    exit(main())