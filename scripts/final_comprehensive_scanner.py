#!/usr/bin/env python3
"""
最终全面扫描器
简化的最终扫描，确保不遗漏任何API方法
"""

import re
from pathlib import Path
from typing import Dict, List, Set, Tuple, Optional

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
        'tenant', 'tenant_tag'
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

    # 检查前50行
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
    operation_score = sum(1 for operation in api_operations if operation in method_lower)

    if high_score >= 2 or (high_score >= 1 and operation_score >= 1):
        return 'HIGH'
    elif operation_score >= 2:
        return 'MEDIUM'
    elif operation_score >= 1:
        return 'LOW'
    else:
        return 'UNCERTAIN'

def find_api_methods_simple(root_dir: Path) -> List[Tuple[Path, int, str, str, List[str]]]:
    """简化的API方法查找"""
    api_methods = []

    # 标准API方法模式
    api_patterns = [
        r'pub\s+async\s+fn\s+(\w+)\s*\([^)]*\)\s*->',
        r'pub\s+fn\s+(\w+)\s*\([^)]*\)\s*->\s*[^{]*SDKResult',
        r'pub\s+(async\s+)?fn\s+(\w+)\s*\([^)]*\)\s*->\s*[^{]*(?:Result|Response)',
    ]

    service_dir = root_dir / "src" / "service"
    if not service_dir.exists():
        return api_methods

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
                'AccessTokenType', 'http_method', 'RequestOption'
            ])

            if not has_api_indicators:
                continue

            for line_num, line in enumerate(lines, 1):
                # 跳过注释行
                if line.strip().startswith('//') or line.strip().startswith('*'):
                    continue

                for pattern in api_patterns:
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
                        non_api_methods = {
                            'new', 'default', 'from', 'into', 'clone', 'debug', 'display',
                            'validate', 'check', 'verify', 'ensure', 'builder', 'build'
                        }

                        if method_name.lower() in non_api_methods:
                            continue

                        # 跳过私有方法
                        if method_name.startswith('_'):
                            continue

                        # 获取服务名称
                        service_name = determine_service_from_file(rust_file)

                        # 检查是否已有文档URL
                        has_existing_doc = check_existing_documentation(lines, line_num)

                        if not has_existing_doc:
                            # 获取方法上下文
                            context_lines = get_method_context(lines, line_num)
                            api_methods.append((rust_file, line_num, method_name, service_name, context_lines))

        except Exception as e:
            print(f"❌ 处理文件失败 {rust_file}: {e}")

    return api_methods

def main():
    print("🔍 开始最终全面API方法扫描...")
    print("=" * 80)

    root_dir = Path('/Users/zool/RustroverProjects/open-lark')

    # 执行搜索
    print("🚀 执行API方法搜索...")
    all_methods = find_api_methods_simple(root_dir)
    print(f"   找到 {len(all_methods)} 个缺少文档的API方法")

    if not all_methods:
        print("✅ 所有API方法都有文档！")
        return 0

    # 分析重要性
    print(f"\n📊 分析方法重要性...")
    importance_stats = {'HIGH': 0, 'MEDIUM': 0, 'LOW': 0, 'UNCERTAIN': 0}
    service_stats = {}

    for file_path, line_num, method_name, service_name, context in all_methods:
        importance = analyze_api_method_importance(method_name, context, service_name)
        importance_stats[importance] += 1

        if service_name not in service_stats:
            service_stats[service_name] = 0
        service_stats[service_name] += 1

    # 显示结果
    print(f"\n🎯 扫描结果:")
    print(f"   • 总缺少文档的方法: {len(all_methods)}")

    print(f"\n📋 按重要性分类:")
    for importance, count in importance_stats.items():
        if count > 0:
            print(f"   • {importance}: {count} 个方法")

    print(f"\n🚨 最需要处理的服务:")
    sorted_services = sorted(service_stats.items(), key=lambda x: x[1], reverse=True)
    for service, count in sorted_services[:10]:
        print(f"   • {service}: {count} 个方法")

    # 显示高重要性方法
    high_priority = []
    for file_path, line_num, method_name, service_name, context in all_methods:
        importance = analyze_api_method_importance(method_name, context, service_name)
        if importance == 'HIGH':
            high_priority.append((file_path, line_num, method_name, service_name))

    if high_priority:
        print(f"\n⚠️  高优先级缺少文档的方法（必须处理）:")
        for file_path, line_num, method_name, service_name in high_priority[:10]:
            relative_path = file_path.relative_to(root_dir)
            print(f"   • {relative_path}:{line_num} - {method_name}() ({service_name})")

        if len(high_priority) > 10:
            print(f"   ... 还有 {len(high_priority) - 10} 个高重要性方法")

    return len(all_methods)

if __name__ == "__main__":
    count = main()
    if count > 0:
        print(f"\n🔧 需要为 {count} 个API方法添加文档URL")
    else:
        print(f"\n🎉 所有API方法都有文档URL！")