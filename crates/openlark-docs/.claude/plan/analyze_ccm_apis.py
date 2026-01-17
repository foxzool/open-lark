#!/usr/bin/env python3
"""
CCM业务域API分析脚本
按照meta.Project分析CCM API的分布情况
"""

import csv
import json
from collections import defaultdict

def analyze_ccm_apis():
    """分析CCM业务域API的project-version-resource分布"""

    ccm_apis = []
    project_stats = defaultdict(lambda: defaultdict(lambda: defaultdict(int)))

    # 读取CSV文件
    with open('/Users/zool/RustroverProjects/open-lark/analysis/data/api_list_export.csv', 'r', encoding='utf-8') as f:
        reader = csv.DictReader(f)

        for row in reader:
            if row['bizTag'] == 'ccm':
                # 解析meta信息
                meta_project = row['meta.Project']
                meta_version = row['meta.Version']
                meta_resource = row['meta.Resource']
                meta_type = row['meta.Type']

                api_info = {
                    'id': row['id'],
                    'name': row['name'],
                    'docPath': row['docPath'],
                    'url': row['url'],
                    'meta_project': meta_project,
                    'meta_version': meta_version,
                    'meta_resource': meta_resource,
                    'meta_type': meta_type,
                    'method': row['url'].split(':')[0] if ':' in row['url'] else 'GET'
                }

                ccm_apis.append(api_info)
                project_stats[meta_project][meta_version][meta_resource] += 1

    # 生成详细统计
    print("🎯 CCM业务域API分析结果")
    print("=" * 60)
    print(f"📊 总API数量: {len(ccm_apis)}")
    print()

    print("📁 按meta.Project分组:")
    projects = defaultdict(list)
    for api in ccm_apis:
        projects[api['meta_project']].append(api)

    for project, apis in sorted(projects.items()):
        print(f"  {project}: {len(apis)}个API")

        # 按版本分组
        versions = defaultdict(list)
        for api in apis:
            versions[api['meta_version']].append(api)

        for version, version_apis in sorted(versions.items()):
            print(f"    v{version}: {len(version_apis)}个API")

            # 按资源分组
            resources = defaultdict(list)
            for api in version_apis:
                resources[api['meta_resource']].append(api)

            for resource, resource_apis in sorted(resources.items()):
                print(f"      {resource}: {len(resource_apis)}个API")

    print()
    print("🔗 详细的PVR结构:")
    print("=" * 60)

    for project in sorted(projects.keys()):
        print(f"📂 {project}/")
        versions = defaultdict(list)
        for api in projects[project]:
            versions[api['meta_version']].append(api)

        for version in sorted(versions.keys(), reverse=True):  # 优先处理最新版本
            print(f"  └── v{version}/")
            resources = defaultdict(list)
            for api in versions[version]:
                resources[api['meta_resource']].append(api)

            for resource in sorted(resources.keys()):
                print(f"      └── {resource}/ ({len(resources[resource])}个API)")
                for api in resources[resource]:
                    print(f"        - {api['name']} ({api['method']})")

    print()
    print("📋 建议的目录结构:")
    print("=" * 60)

    for project in sorted(projects.keys()):
        print(f"src/{project}/")
        versions = set()
        resources = set()

        for api in projects[project]:
            versions.add(api['meta_version'])
            resources.add(api['meta_resource'])

        for version in sorted(versions, reverse=True):
            print(f"├── v{version}/")
            for resource in sorted(resources):
                print(f"│   ├── {resource}/")
                print(f"│   │   ├── mod.rs")
                print(f"│   │   ├── models.rs")
                print(f"│   │   └── services.rs")

        print(f"├── mod.rs")
        print(f"└── lib.rs")
        print()

if __name__ == "__main__":
    analyze_ccm_apis()