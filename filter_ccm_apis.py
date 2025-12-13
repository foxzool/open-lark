import csv
import json

# 读取CSV文件
ccm_apis = []
with open('/Users/zool/RustroverProjects/open-lark/api_list_export.csv', 'r', encoding='utf-8') as f:
    reader = csv.DictReader(f)
    for row in reader:
        if row['bizTag'] == 'ccm':
            ccm_apis.append(row)

# 统计数量
print(f"CCM API总数: {len(ccm_apis)}")

# 按meta.Project分组统计
project_stats = {}
for api in ccm_apis:
    project = api['meta.Project']
    if project not in project_stats:
        project_stats[project] = []
    project_stats[project].append(api)

print("\n按Project分组:")
for project, apis in project_stats.items():
    print(f"  {project}: {len(apis)}个")

# 输出每个API的详细信息
print("\nCCM API列表:")
for api in ccm_apis:
    print(f"{api['meta.Project']}: {api['name']} - {api['url']}")