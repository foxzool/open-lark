#!/usr/bin/env python3
"""
终极API方法扫描器
进行最全面、最深入的API方法搜索，确保不遗漏任何可能需要文档URL的方法
"""

import re
from pathlib import Path
from typing import Dict, List, Set, Tuple, Optional
import json

def find_all_possible_api_methods(root_dir: Path) -> List[Tuple[Path, int, str, str, List[str]]]:
    """查找所有可能的API方法，使用最宽松的匹配规则"""
    all_methods = []

    # 最全面的API方法匹配模式
    comprehensive_patterns = [
        # 标准异步方法
        r'pub\s+async\s+fn\s+(\w+)\s*\([^)]*\)\s*->',
        # 标准同步方法
        r'pub\s+fn\s+(\w+)\s*\([^)]*\)\s*->',
        # impl块中的方法
        r'impl\s+[A-Za-z0-9_]+\s*\{[^}]*pub\s+(async\s+)?fn\s+(\w+)\s*\(',
        # 复杂的返回类型模式
        r'pub\s+(async\s+)?fn\s+(\w+)\s*\([^)]*\)\s*->\s*[^{]*(?:Result|Response|SDKResult)',
        # 可能是API操作的方法
        r'pub\s+(async\s+)?fn\s+(\w+)\s*\([^)]*\)\s*(?:\{|->)',
    ]

    service_dir = root_dir / "src" / "service"
    if not service_dir.exists():
        return all_methods

    rust_files = list(service_dir.rglob("*.rs"))
    print(f"📁 扫描 {len(rust_files)} 个Rust文件...")

    for rust_file in rust_files:
        # 跳过明显不需要文档的文件
        skip_patterns = [
            'test.rs', 'tests.rs', 'mod.rs', 'models.rs', 'types.rs', 'errors.rs',
            'utils.rs', 'helper.rs', 'mock.rs', 'example.rs', 'benches.rs',
            'benchmarks.rs', 'benchmark.rs', 'examples.rs'
        ]

        if any(pattern in str(rust_file).lower() for pattern in skip_patterns):
            continue

        try:
            with open(rust_file, 'r', encoding='utf-8') as f:
                content = f.read()
                lines = content.split('\n')

            # 检查文件中是否包含API相关的代码
            has_api_indicators = any(indicator in content for indicator in [
                'SDKResult', 'Transport::request', 'ApiRequest', 'api_path',
                'AccessTokenType', 'http_method', 'RequestOption',
                'serde_json::to_vec', 'endpoints::'
            ])

            if not has_api_indicators:
                continue

            for line_num, line in enumerate(lines, 1):
                # 跳过注释行
                if line.strip().startswith('//') or line.strip().startswith('*'):
                    continue

                for pattern in comprehensive_patterns:
                    matches = re.finditer(pattern, line, re.IGNORECASE)
                    for match in matches:
                        # 获取方法名
                        if match.groups():
                            method_name = match.group(1) if match.group(1) else match.group(2)
                        else:
                            continue

                        if not method_name or method_name.strip() == '':
                            continue

                        # 跳过明显的非API方法
                        non_api_patterns = [
                            r'^new$', r'^default$', r'^from_', r'^to_', r'^as_',
                            r'^is_', r'^has_', r'^can_', r'^should_', r'^would_',
                            r'^debug_', r'^display_', r'^clone_', r'^copy_',
                            r'^validate_', r'^check_', r'^verify_', r'^ensure_',
                            r'^with_', r'^without_', r'^set_', r'^get_',
                            r'^len$', r'^size$', r'^count$', r'^is_empty$',
                            r'^clear$', r'^reset$', r'^flush$', r'^sync$',
                            r'^serialize$', r'^deserialize$', r'^encode$', r'^decode$',
                            r'^parse$', r'^format$', r'^convert$', r'^transform$',
                            r'^map$', r'^filter$', r'^fold$', r'^reduce$',
                            r'^iter$', r'^next$', r'^collect$', r'^unwrap$',
                            r'^expect$', r'^ok$', r'^err$', r'^some$', r'^none$',
                            r'^drop$', r'^free$', r'^cleanup$', r'^dispose$',
                            r'^init$', r'^setup$', r'^configure$', r'^build$'
                        ]

                        is_non_api = any(re.match(pattern, method_name, re.IGNORECASE)
                                       for pattern in non_api_patterns)

                        if is_non_api:
                            continue

                        # 跳过私有方法
                        if method_name.startswith('_'):
                            continue

                        # 检查是否已有文档URL
                        has_existing_doc = check_existing_documentation(lines, line_num)

                        # 获取服务名称
                        service_name = determine_service_from_file_path(rust_file)

                        # 获取方法上下文
                        context_lines = get_method_context(lines, line_num)

                        all_methods.append((rust_file, line_num, method_name, service_name, context_lines))

        except Exception as e:
            print(f"❌ 处理文件失败 {rust_file}: {e}")

    return all_methods

def determine_service_from_file(file_path: Path) -> str:
    """从文件路径确定服务名称"""
    path_str = str(file_path).lower()

    # 服务名称的优先级匹配
    services = [
        'attendance', 'approval', 'calendar', 'cloud_docs', 'contact', 'mail',
        'search', 'im', 'ai', 'bot', 'cardkit', 'directory', 'drive', 'sheets',
        'bitable', 'task', 'minutes', 'vc', 'ehr', 'corehr', 'helpdesk',
        'hire', 'moments', 'okr', 'payroll', 'performance', 'elearning',
        'lingo', 'mdm', 'verification', 'trust_party', 'workplace',
        'admin', 'acs', 'apass', 'application', 'authentication', 'group',
        'human_authentication', 'personal_settings', 'report', 'security_and_compliance',
        'tenant', 'tenant_tag', 'referral', 'address_book', 'approval_engine',
        'bi', 'suite', 'meeting', 'wiki', 'base', 'doc', 'file', 'storage'
    ]

    for service in services:
        if service in path_str:
            return service

    # 从路径中提取
    if 'src/service/' in path_str:
        parts = path_str.split('src/service/')
        if len(parts) > 1:
            first_part = parts[1]
            if '/' in first_part:
                return first_part.split('/')[0]

    return 'unknown'

def check_existing_documentation(lines: List[str], line_num: int) -> bool:
    """检查是否已有文档URL"""
    doc_pattern = re.compile(r'https://open\.feishu\.cn/document/')

    # 检查前50行（扩大范围）
    start = max(0, line_num - 50)
    end = min(len(lines), line_num + 10)

    for i in range(start, end):
        if doc_pattern.search(lines[i]):
            return True

    return False

def get_method_context(lines: List[str], line_num: int) -> List[str]:
    """获取方法的上下文信息"""
    context = []
    start = max(0, line_num - 5)
    end = min(len(lines), line_num + 5)

    for i in range(start, end):
        prefix = ">>>" if i == line_num - 1 else "   "
        context.append(f"{prefix} {i+1:4d}: {lines[i]}")

    return context

def analyze_api_method_importance(method_name: str, context: List[str], service: str) -> str:
    """分析API方法的重要性"""
    method_lower = method_name.lower()
    context_text = ' '.join(context).lower()

    # 高重要性指标
    high_importance_indicators = [
        'transport::request', 'api_path', 'http_method', 'sdkresult',
        'requestoption', 'accesstokentype', 'endpoint', 'open-apis'
    ]

    # 中等重要性指标
    medium_importance_indicators = [
        'api_req', 'request', 'response', 'serde_json', 'serialize'
    ]

    # API操作关键词
    api_operations = [
        'create', 'get', 'list', 'update', 'delete', 'remove', 'add', 'insert',
        'search', 'query', 'find', 'filter', 'sort', 'batch', 'bulk',
        'send', 'receive', 'reply', 'forward', 'broadcast',
        'approve', 'reject', 'submit', 'cancel', 'withdraw', 'archive',
        'start', 'stop', 'pause', 'resume', 'restart', 'execute',
        'upload', 'download', 'import', 'export', 'sync', 'backup',
        'login', 'logout', 'auth', 'verify', 'validate', 'check'
    ]

    high_score = sum(1 for indicator in high_importance_indicators if indicator in context_text)
    medium_score = sum(1 for indicator in medium_importance_indicators if indicator in context_text)
    operation_score = sum(1 for operation in api_operations if operation in method_lower)

    if high_score >= 2 or (high_score >= 1 and operation_score >= 1):
        return 'HIGH'
    elif medium_score >= 2 or operation_score >= 2:
        return 'MEDIUM'
    elif medium_score >= 1 or operation_score >= 1:
        return 'LOW'
    else:
        return 'UNCERTAIN'

def generate_ultimate_report(methods: List[Tuple]) -> Dict:
    """生成最终的全面报告"""
    total_methods = len(methods)

    if total_methods == 0:
        return {
            'total_methods': 0,
            'has_docs': 0,
            'missing_docs': 0,
            'coverage_rate': 100.0,
            'categories': {},
            'service_stats': {},
            'importance_stats': {}
        }

    # 分类方法
    categories = {
        'has_docs': [],
        'missing_docs': {
            'HIGH': [],
            'MEDIUM': [],
            'LOW': [],
            'UNCERTAIN': []
        }
    }

    service_stats = {}
    importance_stats = {'HIGH': 0, 'MEDIUM': 0, 'LOW': 0, 'UNCERTAIN': 0}

    for file_path, line_num, method_name, service_name, context in methods:
        has_doc = check_existing_documentation([line.split('   ', 1)[-1] for line in context if '>>>' not in line], 0)

        method_info = {
            'file': file_path,
            'line': line_num,
            'method': method_name,
            'service': service_name,
            'context': context,
            'importance': analyze_api_method_importance(method_name, context, service_name)
        }

        if has_doc:
            categories['has_docs'].append(method_info)
        else:
            importance = method_info['importance']
            categories['missing_docs'][importance].append(method_info)
            importance_stats[importance] += 1

        # 服务统计
        if service_name not in service_stats:
            service_stats[service_name] = {'has_docs': 0, 'missing_docs': 0}

        if has_doc:
            service_stats[service_name]['has_docs'] += 1
        else:
            service_stats[service_name]['missing_docs'] += 1

    has_docs_count = len(categories['has_docs'])
    missing_docs_count = total_methods - has_docs_count
    coverage_rate = (has_docs_count / total_methods * 100) if total_methods > 0 else 0

    return {
        'total_methods': total_methods,
        'has_docs': has_docs_count,
        'missing_docs': missing_docs_count,
        'coverage_rate': coverage_rate,
        'categories': categories,
        'service_stats': service_stats,
        'importance_stats': importance_stats
    }

def main():
    print("🔍 开始终极API方法扫描...")
    print("=" * 80)

    root_dir = Path('/Users/zool/RustroverProjects/open-lark')

    # 执行最全面的搜索
    print("🚀 执行全面的API方法搜索...")
    all_methods = find_all_possible_api_methods(root_dir)
    print(f"   找到 {len(all_methods)} 个可能的API方法")

    if not all_methods:
        print("✅ 没有找到需要处理的API方法")
        return 0

    # 生成报告
    print("\n📊 生成分析报告...")
    report = generate_ultimate_report(all_methods)

    # 显示摘要
    print(f"\n🎯 终极扫描结果:")
    print(f"   • 总API方法数: {report['total_methods']}")
    print(f"   • 已有文档的方法: {report['has_docs']}")
    print(f"   • 缺少文档的方法: {report['missing_docs']}")
    print(f"   • 文档覆盖率: {report['coverage_rate']:.1f}%")

    # 按重要性显示缺少文档的方法
    missing_by_importance = report['categories']['missing_docs']
    print(f"\n📋 按重要性分类的缺少文档方法:")
    for importance in ['HIGH', 'MEDIUM', 'LOW', 'UNCERTAIN']:
        count = len(missing_by_importance.get(importance, []))
        if count > 0:
            print(f"   • {importance}: {count} 个方法")

    # 显示最需要关注的服务
    print(f"\n🚨 最需要处理的服务（缺少文档的方法数）:")
    sorted_services = sorted(report['service_stats'].items(),
                          key=lambda x: x[1]['missing_docs'], reverse=True)
    for service, stats in sorted_services[:10]:
        if stats['missing_docs'] > 0:
            print(f"   • {service}: {stats['missing_docs']} 个缺失")

    # 显示高重要性的缺少文档方法
    high_priority = missing_by_importance.get('HIGH', [])
    if high_priority:
        print(f"\n⚠️  高优先级缺少文档的方法（必须处理）:")
        for method_info in high_priority[:10]:
            relative_path = method_info['file'].relative_to(root_dir)
            print(f"   • {relative_path}:{method_info['line']} - {method_info['method']}() ({method_info['service']})")

    if report['missing_docs'] > 0:
        print(f"\n🔧 总计需要处理 {report['missing_docs']} 个API方法")

        # 生成处理建议
        high_count = len(missing_by_importance.get('HIGH', []))
        medium_count = len(missing_by_importance.get('MEDIUM', []))

        if high_count > 0:
            print(f"   🚨 立即处理: {high_count} 个高重要性方法")
        if medium_count > 0:
            print(f"   ⚠️  建议处理: {medium_count} 个中等重要性方法")

        print(f"\n建议运行针对性的文档补充脚本。")
    else:
        print(f"\n🎉 所有API方法都有文档URL！")

    return report

if __name__ == "__main__":
    report = main()