import os
import csv
from collections import defaultdict

# 读取API列表
ccm_apis = []
with open('/Users/zool/RustroverProjects/open-lark/api_list_export.csv', 'r', encoding='utf-8') as f:
    reader = csv.DictReader(f)
    for row in reader:
        if row['bizTag'] == 'ccm':
            ccm_apis.append(row)

# 按project分组
project_stats = defaultdict(list)
for api in ccm_apis:
    project = api['meta.Project']
    project_stats[project].append(api)

# 定义API到文件名的映射
api_file_mapping = {
    # ccm_doc
    'create': 'create',
    'get_document_meta': 'meta',
    'get_sheet_meta': 'sheet_meta',
    'get_raw_content': 'raw_content',
    'get_content': 'content',
    'batch_update': 'batch_update',

    # ccm_docs
    'search_object': 'search_object',
    'get_meta': 'meta',
}

# 检查现有实现
base_path = '/Users/zool/RustroverProjects/open-lark/crates/openlark-docs/src/ccm'

def extract_api_name_from_url(url):
    """从URL提取API名称"""
    parts = url.split('/')
    if len(parts) > 0:
        return parts[-1]
    return ""

def check_implementation(project, api_name):
    """检查API是否已实现"""
    # 构建可能的文件路径
    project_path = os.path.join(base_path, project)
    if not os.path.exists(project_path):
        return False, f"Project {project} directory not found"

    # 检查 old/v1 或 old/v2 目录
    for version in ['v1', 'v2']:
        old_path = os.path.join(project_path, 'old', version)
        if os.path.exists(old_path):
            # 直接查找对应名称的文件
            for root, dirs, files in os.walk(old_path):
                for file in files:
                    if file.endswith('.rs') and file != 'mod.rs' and file != 'models.rs':
                        # 文件名转换为小写进行比较
                        file_base = file.replace('.rs', '').lower()
                        if api_name.lower() in file_base or file_base in api_name.lower():
                            return True, os.path.join(root, file)

    # 检查其他可能的目录结构
    for root, dirs, files in os.walk(project_path):
        for file in files:
            if file.endswith('.rs') and file != 'mod.rs' and file != 'models.rs':
                # 文件名转换为小写进行比较
                file_base = file.replace('.rs', '').lower()
                if api_name.lower() in file_base or file_base in api_name.lower():
                    return True, os.path.join(root, file)

    return False, f"API {api_name} not found"

# 统计实现情况
total_apis = 0
implemented_apis = 0
missing_apis = []

print("=== CCM API实现情况检查 ===\n")

for project, apis in project_stats.items():
    print(f"\n{project}: {len(apis)}个API")
    project_implemented = 0

    for api in apis:
        total_apis += 1
        # 从URL提取API操作名称
        url_parts = api['url'].split('/')
        api_name = extract_api_name_from_url(api['url'])

        # 特殊处理某些API名称
        if ':' in api_name:
            api_name = api_name.split(':')[0]

        implemented, path = check_implementation(project, api_name)
        if implemented:
            project_implemented += 1
            implemented_apis += 1
        else:
            missing_apis.append({
                'project': project,
                'name': api['name'],
                'url': api['url'],
                'api_name': api_name
            })

    print(f"  已实现: {project_implemented}/{len(apis)}")

print(f"\n总计:")
print(f"  总API数: {total_apis}")
print(f"  已实现: {implemented_apis}")
print(f"  缺失: {len(missing_apis)}")

if missing_apis:
    print(f"\n缺失的API:")
    for missing in missing_apis:
        print(f"  - {missing['project']}: {missing['name']} ({missing['api_name']}) - {missing['url']}")