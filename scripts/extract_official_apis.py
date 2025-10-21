#!/usr/bin/env python3
"""
提取官方API数据
解析官方API CSV文件并提取模块URL映射
"""

import csv
import re
from pathlib import Path
from typing import Dict, List, Set
from collections import defaultdict

def parse_official_apis_csv(csv_file: Path) -> Dict[str, List[Dict]]:
    """解析官方API CSV文件"""
    apis_by_module = defaultdict(list)

    try:
        with open(csv_file, 'r', encoding='utf-8') as f:
            content = f.read()

        # 手动解析CSV，处理包含逗号的字段
        lines = content.split('\n')
        if len(lines) < 2:
            print("❌ CSV文件格式不正确")
            return {}

        header = lines[0].split(',')
        print(f"📊 CSV文件包含 {len(lines)-1} 条API记录")

        for line_num, line in enumerate(lines[1:], 2):
            if not line.strip():
                continue

            # 简单解析：如果以引号开头，可能是包含逗号的字段
            if line.startswith('"') and '","' in line:
                # 处理包含逗号的字段
                parts = [part.strip('"') for part in line.split('","')]
            else:
                # 普通分割
                parts = line.split(',')

            if len(parts) < 6:
                # 如果分割不正确，跳过该行
                continue

            try:
                module = parts[0].strip()
                method = parts[1].strip()
                endpoint = parts[2].strip()
                api_name = parts[3].strip()
                description = parts[4].strip().strip('"')
                doc_url = parts[5].strip().strip('"')
                version = parts[6].strip() if len(parts) > 6 else "v1"

                # 验证URL格式
                if doc_url.startswith('https://open.feishu.cn/document/'):
                    apis_by_module[module].append({
                        'method': method,
                        'endpoint': endpoint,
                        'name': api_name,
                        'description': description,
                        'url': doc_url,
                        'version': version
                    })

            except Exception as e:
                print(f"⚠️  解析第 {line_num} 行时出错: {e}")
                continue

    except Exception as e:
        print(f"❌ 读取CSV文件失败: {e}")
        return {}

    return dict(apis_by_module)

def get_module_base_urls(apis_by_module: Dict[str, List[Dict]]) -> Dict[str, str]:
    """为每个模块获取基础URL"""
    module_urls = {}

    for module, apis in apis_by_module.items():
        if not apis:
            continue

        # 收集该模块的所有URL
        urls = [api['url'] for api in apis if api['url']]

        if urls:
            # 找到最常见的URL模式作为基础URL
            url_patterns = defaultdict(int)

            for url in urls:
                # 提取URL的模式（去掉最后的具体方法部分）
                # 例如: https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/job/get
                # 提取: https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1
                pattern_match = re.match(r'(https://open\.feishu\.cn/document/[^/]+/[^/]+/[^/]+-v\d+)', url)
                if pattern_match:
                    base_url = pattern_match.group(1)
                    url_patterns[base_url] += 1
                else:
                    # 如果不匹配，尝试其他模式
                    parts = url.split('/')
                    if len(parts) >= 6:
                        # 例如: https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1
                        base_url = '/'.join(parts[:6])
                        url_patterns[base_url] += 1

            # 选择出现次数最多的URL模式
            if url_patterns:
                best_pattern = max(url_patterns.items(), key=lambda x: x[1])[0]
                module_urls[module] = best_pattern

    return module_urls

def find_implemented_modules() -> Set[str]:
    """查找已实现的模块"""
    service_dir = Path('/Users/zool/RustroverProjects/open-lark/src/service')
    if not service_dir.exists():
        return set()

    return {d.name for d in service_dir.iterdir() if d.is_dir() and not d.name.startswith('.')}

def main():
    print("🚀 开始提取官方API数据...")
    print("=" * 60)

    # 1. 解析官方API数据
    csv_file = Path('/Users/zool/RustroverProjects/open-lark/reports/official_apis_by_module.csv')
    if not csv_file.exists():
        print(f"❌ 未找到CSV文件: {csv_file}")
        return 1

    apis_by_module = parse_official_apis_csv(csv_file)
    print(f"📊 解析得到 {len(apis_by_module)} 个模块的API数据")

    # 2. 获取模块基础URL
    module_urls = get_module_base_urls(apis_by_module)
    print(f"📊 为 {len(module_urls)} 个模块生成了基础URL")

    # 3. 找到已实现的模块
    implemented_modules = find_implemented_modules()
    print(f"📊 发现 {len(implemented_modules)} 个已实现的模块")

    # 4. 分析匹配情况
    print("\n📈 模块匹配情况:")
    print("-" * 60)

    # 有官方数据的已实现模块
    matched_modules = implemented_modules & set(module_urls.keys())
    print(f"✅ 有官方数据的已实现模块: {len(matched_modules)}")

    # 已实现但没有官方数据的模块
    no_official_data = implemented_modules - set(module_urls.keys())
    if no_official_data:
        print(f"⚠️  已实现但没有官方数据的模块: {len(no_official_data)}")
        for module in sorted(no_official_data):
            print(f"     • {module}")

    # 有官方数据但未实现的模块
    not_implemented = set(module_urls.keys()) - implemented_modules
    if not_implemented:
        print(f"💡 有官方数据但未实现的模块: {len(not_implemented)}")
        for module in sorted(not_implemented)[:10]:  # 只显示前10个
            print(f"     • {module}")
        if len(not_implemented) > 10:
            print(f"     ... 还有 {len(not_implemented) - 10} 个")

    # 5. 输出URL映射结果
    print(f"\n📋 模块URL映射结果:")
    print("-" * 60)

    for module in sorted(matched_modules):
        url = module_urls[module]
        api_count = len(apis_by_module.get(module, []))
        print(f"{module:<20} -> {api_count:<3} APIs -> {url}")

    # 6. 保存结果到文件
    output_file = Path('/Users/zool/RustroverProjects/open-lark/scripts/module_url_mapping.py')
    with open(output_file, 'w', encoding='utf-8') as f:
        f.write('#!/usr/bin/env python3\n')
        f.write('"""\n')
        f.write('模块URL映射数据\n')
        f.write('从官方API数据提取的模块基础URL映射\n')
        f.write('"""\n\n')
        f.write('# 模块URL映射 - 用于为没有文档URL的模块添加文档\n')
        f.write('MODULE_URL_MAPPING = {\n')

        for module in sorted(matched_modules):
            url = module_urls[module]
            api_count = len(apis_by_module.get(module, []))
            f.write(f'    "{module}": {{\n')
            f.write(f'        "base_url": "{url}",\n')
            f.write(f'        "api_count": {api_count},\n')
            f.write(f'        "apis": [\n')

            # 添加该模块的API列表（前5个作为示例）
            apis = apis_by_module.get(module, [])
            for api in apis[:5]:
                method = api['method']
                name = api['name']
                url = api['url']
                f.write(f'            {{"method": "{method}", "name": "{name}", "url": "{url}"}},\n')

            if len(apis) > 5:
                f.write(f'            # ... 还有 {len(apis) - 5} 个API\n')

            f.write(f'        ]\n')
            f.write(f'    }},\n')

        f.write('}\n\n')
        f.write('# 缺少官方数据的已实现模块\n')
        f.write('MODULES_WITHOUT_OFFICIAL_DATA = [\n')
        for module in sorted(no_official_data):
            f.write(f'    "{module}",\n')
        f.write(']\n')

    print(f"\n💾 结果已保存到: {output_file}")

    return 0

if __name__ == "__main__":
    exit(main())