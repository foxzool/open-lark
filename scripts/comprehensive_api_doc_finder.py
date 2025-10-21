#!/usr/bin/env python3
"""
全面API文档URL查找和修复器
专门处理复杂的API方法模式和边缘情况
"""

import re
from pathlib import Path
from typing import Dict, List, Set, Tuple, Optional

def analyze_service_directory_structure(root_dir: Path) -> Dict[str, List[Path]]:
    """分析服务目录结构"""
    service_dir = root_dir / "src" / "service"
    if not service_dir.exists():
        return {}

    services = {}
    for item in service_dir.iterdir():
        if item.is_dir():
            service_name = item.name
            rust_files = list(item.rglob("*.rs"))
            services[service_name] = rust_files

    return services

def find_all_api_methods_comprehensive(root_dir: Path) -> List[Tuple[Path, int, str, str, List[str]]]:
    """全面查找所有API方法，包括复杂的模式"""
    all_api_methods = []

    # 更全面的API方法匹配模式
    api_patterns = [
        # 异步方法
        r'pub\s+async\s+fn\s+(\w+)\s*\([^)]*\)\s*->\s*[^{]+',
        # 同步方法返回SDKResult
        r'pub\s+fn\s+(\w+)\s*\([^)]*\)\s*->\s*[^:]*SDKResult',
        # 实现块中的公共方法
        r'impl\s+\w+\s*\{[^}]*pub\s+(async\s+)?fn\s+(\w+)\s*\(',
        # 更复杂的模式
        r'pub\s+(async\s+)?fn\s+(\w+)\s*\([^)]*\)\s*->\s*[^{]*(?:Result|Response)',
    ]

    service_dir = root_dir / "src" / "service"
    if not service_dir.exists():
        return all_api_methods

    for rust_file in service_dir.rglob("*.rs"):
        # 跳过明显不需要文档的文件类型
        if any(skip_pattern in str(rust_file).lower() for skip_pattern in [
            'models.rs', 'mod.rs', 'types.rs', 'errors.rs', 'utils.rs', 'helper.rs',
            'test.rs', 'tests.rs', 'mock.rs', 'example.rs'
        ]):
            continue

        try:
            with open(rust_file, 'r', encoding='utf-8') as f:
                content = f.read()
                lines = content.split('\n')

            for line_num, line in enumerate(lines, 1):
                # 检查是否包含API方法定义
                for pattern in api_patterns:
                    matches = re.finditer(pattern, line)
                    for match in matches:
                        # 获取方法名
                        method_name = match.group(1) if match.groups() and match.group(1) else match.group(2)

                        if not method_name:
                            continue

                        # 跳过非API方法
                        skip_methods = {
                            'new', 'default', 'from', 'into', 'clone', 'debug', 'display',
                            'eq', 'ne', 'lt', 'le', 'gt', 'ge', 'hash', 'partial_eq',
                            'serialize', 'deserialize', 'builder', 'build', 'execute',
                            'as_ref', 'as_mut', 'deref', 'deref_mut', 'borrow', 'borrow_mut',
                            'to_owned', 'into_owned', 'to_string', 'from_str',
                            'is_some', 'is_none', 'unwrap', 'expect', 'ok', 'err',
                            'map', 'and_then', 'or_else', 'filter', 'find', 'position',
                            'len', 'is_empty', 'capacity', 'reserve', 'shrink',
                            'push', 'pop', 'insert', 'remove', 'clear', 'truncate',
                            'iter', 'iter_mut', 'into_iter', 'drain', 'split',
                            'get', 'get_mut', 'contains_key', 'insert_kv', 'remove_kv',
                            'keys', 'values', 'values_mut', 'into_keys', 'into_values',
                            'validate', 'check', 'verify', 'ensure', 'assert', 'confirm',
                            'parse', 'format', 'encode', 'decode', 'compress', 'decompress',
                            'hash_with', 'encrypt', 'decrypt', 'sign', 'verify_signature'
                        }

                        if method_name.lower() in skip_methods:
                            continue

                        # 跳过私有方法
                        if method_name.startswith('_'):
                            continue

                        # 确定服务名称
                        service_name = determine_service_from_file_path(rust_file)

                        # 检查是否已有文档URL
                        has_doc_url = check_existing_doc_url(lines, line_num)

                        # 获取方法签名和上下文
                        context_lines = get_method_context(lines, line_num)

                        all_api_methods.append((rust_file, line_num, method_name, service_name, context_lines))

        except Exception as e:
            print(f"❌ 处理文件失败 {rust_file}: {e}")

    return all_api_methods

def determine_service_from_file_path(file_path: Path) -> str:
    """从文件路径确定服务名称"""
    path_str = str(file_path)

    # 检查服务名称的优先级顺序
    services = [
        'attendance', 'approval', 'calendar', 'cloud_docs', 'contact', 'mail',
        'search', 'im', 'ai', 'bot', 'cardkit', 'directory', 'drive', 'sheets',
        'bitable', 'task', 'minutes', 'vc', 'ehr', 'corehr', 'helpdesk',
        'hire', 'moments', 'okr', 'payroll', 'performance', 'elearning',
        'lingo', 'mdm', 'verification', 'trust_party', 'workplace',
        'admin', 'acs', 'apass', 'application', 'authentication', 'group',
        'human_authentication', 'personal_settings', 'report', 'security_and_compliance',
        'tenant', 'tenant_tag'
    ]

    for service in services:
        if service in path_str:
            return service

    # 从路径中提取服务名称
    if 'src/service/' in path_str:
        service_part = path_str.split('src/service/')[1]
        if '/' in service_part:
            return service_part.split('/')[0]

    return 'unknown'

def check_existing_doc_url(lines: List[str], line_num: int) -> bool:
    """检查API方法是否已有文档URL"""
    doc_url_pattern = re.compile(r'https://open\.feishu\.cn/document/')

    # 检查前30行
    start = max(0, line_num - 30)
    end = min(len(lines), line_num + 5)

    for i in range(start, end):
        if doc_url_pattern.search(lines[i]):
            return True

    return False

def get_method_context(lines: List[str], line_num: int) -> List[str]:
    """获取方法的上下文"""
    context = []
    start = max(0, line_num - 3)
    end = min(len(lines), line_num + 3)

    for i in range(start, end):
        context.append(f"{i+1:4d}: {lines[i]}")

    return context

def categorize_api_methods(api_methods: List[Tuple]) -> Dict[str, Dict]:
    """将API方法分类"""
    categories = {
        'missing_docs': [],
        'has_docs': [],
        'uncertain': []
    }

    for file_path, line_num, method_name, service_name, context in api_methods:
        has_doc = check_existing_doc_url(context, len(context) // 2)

        method_info = {
            'file': file_path,
            'line': line_num,
            'method': method_name,
            'service': service_name,
            'context': context
        }

        if has_doc:
            categories['has_docs'].append(method_info)
        else:
            # 进一步判断是否真的是API方法
            if is_likely_api_method(method_name, context):
                categories['missing_docs'].append(method_info)
            else:
                categories['uncertain'].append(method_info)

    return categories

def is_likely_api_method(method_name: str, context: List[str]) -> bool:
    """判断是否可能是API方法"""
    method_lower = method_name.lower()

    # 明确的API方法模式
    api_patterns = [
        r'create', r'get', r'list', r'update', r'patch', r'delete', r'remove',
        r'search', r'query', r'find', r'filter', r'sort', r'count',
        r'send', r'reply', r'forward', r'broadcast', r'notify',
        r'approve', r'reject', r'cancel', r'submit', r'withdraw',
        r'start', r'stop', r'pause', r'resume', r'restart',
        r'upload', r'download', r'import', r'export', r'sync',
        r'login', r'logout', r'register', r'auth', r'verify',
        r'batch', r'bulk', r'multiple', r'all', r'each'
    ]

    for pattern in api_patterns:
        if pattern in method_lower:
            return True

    # 检查上下文中的关键词
    context_text = ' '.join(context).lower()
    api_keywords = [
        'sdkresult', 'request', 'response', 'transport::request',
        'api_path', 'http_method', 'access_token'
    ]

    for keyword in api_keywords:
        if keyword in context_text:
            return True

    return False

def generate_comprehensive_report(api_methods: List[Tuple]) -> Dict:
    """生成全面的报告"""
    categories = categorize_api_methods(api_methods)

    # 统计信息
    total_methods = len(api_methods)
    has_docs = len(categories['has_docs'])
    missing_docs = len(categories['missing_docs'])
    uncertain = len(categories['uncertain'])

    # 按服务统计
    service_stats = {}
    for category_name, methods in categories.items():
        service_stats[category_name] = {}
        for method_info in methods:
            service = method_info['service']
            if service not in service_stats[category_name]:
                service_stats[category_name][service] = 0
            service_stats[category_name][service] += 1

    return {
        'total_methods': total_methods,
        'has_docs': has_docs,
        'missing_docs': missing_docs,
        'uncertain': uncertain,
        'coverage_rate': (has_docs / total_methods * 100) if total_methods > 0 else 0,
        'service_stats': service_stats,
        'categories': categories
    }

def main():
    print("🔍 开始全面API方法文档分析...")
    print("=" * 80)

    root_dir = Path('/Users/zool/RustroverProjects/open-lark')

    # 分析目录结构
    print("📁 分析服务目录结构...")
    services = analyze_service_directory_structure(root_dir)
    print(f"   发现 {len(services)} 个服务模块")

    # 查找所有API方法
    print("\n🔍 全面搜索API方法...")
    all_api_methods = find_all_api_methods_comprehensive(root_dir)
    print(f"   总共找到 {len(all_api_methods)} 个API方法")

    # 生成报告
    print("\n📊 生成分析报告...")
    report = generate_comprehensive_report(all_api_methods)

    # 显示摘要
    print(f"\n📈 分析结果摘要:")
    print(f"   • 总API方法数: {report['total_methods']}")
    print(f"   • 有文档的方法: {report['has_docs']}")
    print(f"   • 缺少文档的方法: {report['missing_docs']}")
    print(f"   • 不确定的方法: {report['uncertain']}")
    print(f"   • 文档覆盖率: {report['coverage_rate']:.1f}%")

    # 显示缺少文档最多的服务
    print(f"\n🚨 最需要处理的服务（缺少文档的方法数）:")
    missing_by_service = report['service_stats']['missing_docs']
    for service, count in sorted(missing_by_service.items(), key=lambda x: x[1], reverse=True)[:10]:
        if count > 0:
            print(f"   • {service}: {count} 个方法")

    # 显示一些具体缺少文档的方法示例
    print(f"\n📋 缺少文档的API方法示例:")
    missing_methods = report['categories']['missing_docs'][:10]
    for method_info in missing_methods:
        relative_path = method_info['file'].relative_to(root_dir)
        print(f"   • {relative_path}:{method_info['line']} - {method_info['method']}() ({method_info['service']})")

    if report['missing_docs'] > 0:
        print(f"\n⚠️  还有 {report['missing_docs']} 个API方法缺少文档URL")
        print("建议运行相应的文档补充脚本。")
    else:
        print(f"\n🎉 所有API方法都有文档URL！")

    return report

if __name__ == "__main__":
    report = main()