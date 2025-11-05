#!/usr/bin/env python3
"""
分析API映射关系，了解URL路径到服务名称的分布情况
"""

import csv
import json
from collections import Counter, defaultdict

def analyze_api_mapping():
    """分析API映射关系"""

    # 读取CSV文件
    apis = []
    with open('api_mapping_tools/server_api_list.csv', 'r', encoding='utf-8') as f:
        reader = csv.DictReader(f)
        for row in reader:
            apis.append({
                'name': row['name'],
                'method': row['method'],
                'path': row['path'],
                'description': row['description'],
                'self_build': row['self_build'],
                'store_app': row['store_app'],
                'doc_link': row['doc_link']
            })

    print(f"总共分析了 {len(apis)} 个API")

    # 分析URL路径中的服务名称
    service_counter = Counter()
    service_details = defaultdict(list)
    path_services = []

    for api in apis:
        path = api['path']
        if path.startswith('/open-apis/'):
            # 提取服务名称: /open-apis/service/vX/endpoint -> service
            parts = path.split('/')
            if len(parts) >= 3:
                service_name = parts[2]  # /open-apis/SERVICE/...
                service_counter[service_name] += 1
                service_details[service_name].append(api)
                path_services.append({
                    'name': api['name'],
                    'path': path,
                    'service': service_name,
                    'method': api['method'],
                    'description': api['description'][:100] + '...' if len(api['description']) > 100 else api['description']
                })

    # 打印服务分布统计
    print(f"\n发现 {len(service_counter)} 个不同的服务名称:")
    print("=" * 60)

    for i, (service, count) in enumerate(service_counter.most_common(), 1):
        percentage = (count / len(apis)) * 100
        print(f"{i:2d}. {service:<15} : {count:3d} 个API ({percentage:5.1f}%)")

    # 保存详细分析结果
    analysis_result = {
        'total_apis': len(apis),
        'unique_services': len(service_counter),
        'service_distribution': dict(service_counter),
        'top_services': dict(service_counter.most_common(10)),
        'path_services': path_services,
        'service_details': {k: v[:5] for k, v in service_details.items()}  # 每个服务保存前5个API示例
    }

    with open('api_mapping_analysis.json', 'w', encoding='utf-8') as f:
        json.dump(analysis_result, f, indent=2, ensure_ascii=False)

    print(f"\n详细分析结果已保存到: api_mapping_analysis.json")

    # 检查与现有功能标志的对应关系
    print("\n" + "=" * 60)
    print("与现有功能标志的对应关系分析:")
    print("=" * 60)

    # 读取现有的Cargo.toml功能标志
    existing_features = []
    try:
        with open('Cargo.toml', 'r', encoding='utf-8') as f:
            content = f.read()
            in_features = False
            for line in content.split('\n'):
                line = line.strip()
                if line.startswith('[features]'):
                    in_features = True
                    continue
                elif line.startswith('[') and in_features:
                    in_features = False
                    continue
                elif in_features and '=' in line and not line.startswith('#'):
                    feature_name = line.split('=')[0].strip()
                    existing_features.append(feature_name)
    except FileNotFoundError:
        print("未找到Cargo.toml文件")

    print(f"现有功能标志数量: {len(existing_features)}")
    print("功能标志列表:")
    for feature in sorted(existing_features):
        print(f"  - {feature}")

    # 分析映射关系
    print("\n映射关系分析:")
    matched_services = []
    unmatched_services = []

    for service in service_counter.keys():
        if service in existing_features:
            matched_services.append(service)
        else:
            unmatched_services.append(service)

    print(f"  匹配的服务: {len(matched_services)}")
    for service in sorted(matched_services):
        print(f"    ✓ {service}")

    print(f"  不匹配的服务: {len(unmatched_services)}")
    for service in sorted(unmatched_services):
        print(f"    ✗ {service}")

    # 检查特殊命名情况
    print("\n特殊命名情况分析:")
    print("=" * 60)

    special_cases = []
    for service in service_counter.keys():
        # 检查authen vs auth的差异
        if service == 'authen':
            special_cases.append(('authen', 'auth', '认证服务，URL使用authen但功能标志可能使用auth'))
        # 检查其他可能的命名差异
        elif service in ['im', 'message']:
            special_cases.append((service, 'im', '即时通讯服务'))
        elif service in ['drive', 'docx']:
            special_cases.append((service, 'cloud-docs', '云文档服务'))

    for url_service, potential_feature, description in special_cases:
        if potential_feature in existing_features:
            print(f"  {url_service} -> {potential_feature} ({description}) ✓")
        else:
            print(f"  {url_service} -> {potential_feature} ({description}) ?")

    return analysis_result

if __name__ == "__main__":
    analyze_api_mapping()