#!/usr/bin/env python3
"""
全面模块检查脚本
检查所有服务模块的URL覆盖情况
"""

import os
import re
from pathlib import Path
from typing import Dict, List, Set, Tuple

def find_all_service_modules(root_dir: Path) -> Dict[str, List[Path]]:
    """查找所有服务模块"""
    modules = {}
    service_dir = root_dir / "src" / "service"

    if not service_dir.exists():
        return modules

    for module_dir in service_dir.iterdir():
        if module_dir.is_dir() and not module_dir.name.startswith('.'):
            # 找到该模块下的所有Rust文件
            rust_files = list(module_dir.rglob("*.rs"))
            if rust_files:
                modules[module_dir.name] = rust_files

    return modules

def find_apis_without_docs(root_dir: Path) -> List[Tuple[Path, int, str]]:
    """查找没有文档的API方法"""
    apis_without_docs = []

    # 匹配异步API方法的模式
    api_patterns = [
        r'pub\s+async\s+fn\s+(\w+)\s*\(',
        r'pub\s+fn\s+(\w+)\s*\([^)]*\)\s*->\s*SDKResult',
        r'pub\s+async\s+fn\s+(\w+)\s*\([^)]*\)\s*->\s*SDKResult'
    ]

    for rust_file in root_dir.rglob("*.rs"):
        if 'target' in str(rust_file) or '.git' in str(rust_file):
            continue

        try:
            with open(rust_file, 'r', encoding='utf-8') as f:
                lines = f.readlines()

            for line_num, line in enumerate(lines, 1):
                # 检查是否包含API方法定义
                for pattern in api_patterns:
                    match = re.search(pattern, line)
                    if match:
                        method_name = match.group(1)

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
                            apis_without_docs.append((rust_file, line_num, method_name))
                            break

        except Exception as e:
            print(f"❌ 读取文件失败 {rust_file}: {e}")

    return apis_without_docs

def analyze_service_coverage():
    """分析服务模块的URL覆盖情况"""
    root_dir = Path('/Users/zool/RustroverProjects/open-lark')

    print("🔍 分析服务模块覆盖情况...")

    # 1. 找到所有服务模块
    modules = find_all_service_modules(root_dir)
    print(f"📊 发现 {len(modules)} 个服务模块")

    # 2. 分析每个模块的情况
    doc_url_pattern = re.compile(r'https://open\.feishu\.cn/document/')
    module_stats = {}

    for module_name, files in modules.items():
        total_urls = 0
        api_methods = 0
        files_with_urls = 0

        for file_path in files:
            try:
                with open(file_path, 'r', encoding='utf-8') as f:
                    content = f.read()

                # 统计URL数量
                urls = doc_url_pattern.findall(content)
                total_urls += len(urls)

                if urls:
                    files_with_urls += 1

                # 统计API方法数量（简单统计）
                api_count = len(re.findall(r'pub\s+async\s+fn\s+\w+', content))
                api_methods += api_count

            except Exception as e:
                print(f"❌ 读取文件失败 {file_path}: {e}")

        module_stats[module_name] = {
            'total_files': len(files),
            'files_with_urls': files_with_urls,
            'total_urls': total_urls,
            'api_methods': api_methods,
            'coverage_ratio': files_with_urls / len(files) if files else 0
        }

    # 3. 输出统计结果
    print("\n📈 模块覆盖情况统计:")
    print("-" * 80)
    print(f"{'模块名':<20} {'文件数':<8} {'有URL文件':<10} {'URL总数':<8} {'API方法':<8} {'覆盖率':<8}")
    print("-" * 80)

    sorted_modules = sorted(module_stats.items(), key=lambda x: x[1]['total_urls'], reverse=True)

    for module_name, stats in sorted_modules:
        print(f"{module_name:<20} {stats['total_files']:<8} {stats['files_with_urls']:<10} "
              f"{stats['total_urls']:<8} {stats['api_methods']:<8} {stats['coverage_ratio']:<8.1%}")

    # 4. 找出可能需要处理的模块
    print("\n⚠️  可能需要处理的模块:")

    # 低覆盖率的模块
    low_coverage = [(name, stats) for name, stats in module_stats.items()
                   if stats['coverage_ratio'] < 0.5 and stats['api_methods'] > 0]

    if low_coverage:
        print("   低覆盖率模块 (< 50%):")
        for name, stats in low_coverage:
            print(f"     • {name}: {stats['coverage_ratio']:.1%} ({stats['files_with_urls']}/{stats['total_files']} 文件有URL)")

    # 无URL的模块
    no_url = [(name, stats) for name, stats in module_stats.items()
             if stats['total_urls'] == 0 and stats['api_methods'] > 0]

    if no_url:
        print("   无URL文档的模块:")
        for name, stats in no_url:
            print(f"     • {name}: {stats['api_methods']} 个API方法，0个文档URL")

    return module_stats, low_coverage, no_url

def check_official_data_coverage():
    """检查官方数据中尚未覆盖的模块"""
    print("\n🔍 检查官方API数据覆盖情况...")

    csv_file = Path('/Users/zool/RustroverProjects/open-lark/reports/official_apis_by_module.csv')

    if not csv_file.exists():
        print("❌ 未找到官方API数据文件")
        return set()

    official_modules = set()
    try:
        with open(csv_file, 'r', encoding='utf-8') as f:
            content = f.read()
            # 简单提取模块名
            lines = content.split('\n')
            for line in lines[1:]:  # 跳过标题行
                if line.strip():
                    parts = line.split(',')
                    if len(parts) >= 1:
                        module_name = parts[0].strip()
                        if module_name:
                            official_modules.add(module_name)
    except Exception as e:
        print(f"❌ 读取官方数据失败: {e}")
        return set()

    print(f"📊 官方API数据包含 {len(official_modules)} 个模块")

    # 获取当前已实现的服务模块
    root_dir = Path('/Users/zool/RustroverProjects/open-lark')
    implemented_modules = set(find_all_service_modules(root_dir).keys())

    print(f"📊 已实现 {len(implemented_modules)} 个服务模块")

    # 找出有官方数据但未实现的模块
    not_implemented = official_modules - implemented_modules
    if not_implemented:
        print(f"💡 有官方数据但未实现的模块 ({len(not_implemented)} 个):")
        for module in sorted(not_implemented):
            print(f"     • {module}")

    # 找出已实现但官方数据中没有的模块
    not_in_official = implemented_modules - official_modules
    if not_in_official:
        print(f"⚠️  已实现但官方数据中没有的模块 ({len(not_in_official)} 个):")
        for module in sorted(not_in_official):
            print(f"     • {module}")

    return official_modules, implemented_modules, not_implemented, not_in_official

def main():
    print("🚀 开始全面模块检查...")
    print("=" * 80)

    try:
        # 1. 分析服务模块覆盖情况
        module_stats, low_coverage, no_url = analyze_service_coverage()

        # 2. 检查官方数据覆盖情况
        official_data_result = check_official_data_coverage()

        # 3. 查找没有文档的API方法
        print("\n🔍 查找缺少文档的API方法...")
        root_dir = Path('/Users/zool/RustroverProjects/open-lark')
        apis_without_docs = find_apis_without_docs(root_dir)

        if apis_without_docs:
            print(f"⚠️  发现 {len(apis_without_docs)} 个缺少文档的API方法:")
            for file_path, line_num, method_name in apis_without_docs[:10]:  # 只显示前10个
                relative_path = file_path.relative_to(root_dir)
                print(f"     • {relative_path}:{line_num} - {method_name}()")

            if len(apis_without_docs) > 10:
                print(f"     ... 还有 {len(apis_without_docs) - 10} 个")
        else:
            print("✅ 所有API方法都有文档")

        # 4. 总结和建议
        print("\n" + "=" * 80)
        print("📋 总结和建议:")

        total_modules = len(module_stats)
        modules_with_urls = sum(1 for stats in module_stats.values() if stats['total_urls'] > 0)
        total_urls = sum(stats['total_urls'] for stats in module_stats.values())

        print(f"   • 服务模块总数: {total_modules}")
        print(f"   • 有URL文档的模块: {modules_with_urls}")
        print(f"   • 文档URL总数: {total_urls}")
        print(f"   • 缺少文档的API方法: {len(apis_without_docs)}")

        if low_coverage or no_url or apis_without_docs:
            print(f"\n🎯 建议优先处理:")

            if no_url:
                print(f"   1. 为无URL模块添加文档 ({len(no_url)} 个模块)")

            if low_coverage:
                print(f"   2. 提高低覆盖率模块的文档 ({len(low_coverage)} 个模块)")

            if apis_without_docs:
                print(f"   3. 为缺少文档的API方法添加URL ({len(apis_without_docs)} 个方法)")
        else:
            print(f"\n🎉 所有模块的文档覆盖情况良好！")

    except Exception as e:
        print(f"❌ 检查过程出错: {e}")
        return 1

    return 0

if __name__ == "__main__":
    exit(main())