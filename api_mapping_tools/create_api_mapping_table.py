#!/usr/bin/env python3
"""
API映射表生成器 - 从CSV文件中的API列表查找对应代码实现位置
"""

import csv
import re
import os
import subprocess
from pathlib import Path

def find_api_implementation(api_name, method, path):
    """在代码库中查找API实现的位置"""

    # 从路径中提取服务和操作信息
    path_parts = path.strip('/').split('/')
    if len(path_parts) >= 2 and path_parts[0] == 'open-apis':
        service_parts = path_parts[1:]
    else:
        service_parts = path_parts

    # 确定服务名称
    service_name = service_parts[0] if service_parts else 'unknown'

    # 提取版本信息
    version = 'v1'
    for part in service_parts:
        if part.startswith('v') and part[1:].isdigit():
            version = part
            break

    # 构建搜索关键词
    keywords = []

    # 1. 基于路径最后部分
    if service_parts:
        last_part = service_parts[-1]
        keywords.append(re.sub(r'[^a-zA-Z0-9_]', '', last_part))

    # 2. 基于API名称的关键词
    name_keywords = api_name.replace('获取', '').replace('创建', '').replace('删除', '').replace('更新', '')
    name_keywords = re.sub(r'[^a-zA-Z0-9\u4e00-\u9fff]', '', name_keywords)
    if len(name_keywords) >= 2:
        keywords.append(name_keywords)

    # 3. HTTP方法 + 路径组合
    if service_parts:
        http_keyword = f"{method.lower()}_{service_parts[-1]}"
        http_keyword = re.sub(r'[^a-zA-Z0-9_]', '', http_keyword)
        keywords.append(http_keyword)

    # 搜索可能的实现文件
    search_paths = []
    if service_name != 'unknown':
        search_paths.append(f"src/service/{service_name}/")
        search_paths.append(f"src/service/{service_name}/{version}/")

    # 在可能的服务目录中搜索
    for search_path in search_paths:
        if os.path.exists(search_path):
            for keyword in keywords[:3]:  # 限制搜索关键词数量
                try:
                    # 使用grep搜索相关的异步函数
                    cmd = [
                        "grep", "-r", "-n", "--include=*.rs",
                        f"pub async fn.*{keyword}",
                        search_path
                    ]

                    result = subprocess.run(cmd, capture_output=True, text=True, timeout=5)
                    if result.returncode == 0 and result.stdout.strip():
                        lines = result.stdout.strip().split('\n')
                        for line in lines[:1]:  # 只取第一个结果
                            if ':' in line:
                                parts = line.split(':', 2)
                                if len(parts) >= 3:
                                    file_path = parts[0]
                                    line_num = parts[1]
                                    content = parts[2].strip()

                                    # 验证文件存在
                                    if os.path.exists(file_path):
                                        rel_path = os.path.relpath(file_path, os.getcwd())
                                        return rel_path, line_num, content
                except (subprocess.TimeoutExpired, Exception):
                    continue

    # 如果在服务目录中没找到，进行更广泛的搜索
    try:
        cmd = [
            "grep", "-r", "-n", "--include=*.rs",
            f"pub async fn.*{keywords[0] if keywords else service_name}",
            "src/service/"
        ]

        result = subprocess.run(cmd, capture_output=True, text=True, timeout=8)
        if result.returncode == 0 and result.stdout.strip():
            lines = result.stdout.strip().split('\n')
            for line in lines[:1]:
                if ':' in line:
                    parts = line.split(':', 2)
                    if len(parts) >= 3:
                        file_path = parts[0]
                        line_num = parts[1]
                        content = parts[2].strip()
                        if os.path.exists(file_path):
                            rel_path = os.path.relpath(file_path, os.getcwd())
                            return rel_path, line_num, content
    except Exception:
        pass

    return None, None, None

def process_api_list():
    """处理API列表并生成映射表"""
    csv_file = "server_api_list.csv"
    output_file = "api_implementation_map.md"

    if not os.path.exists(csv_file):
        print(f"错误：找不到文件 {csv_file}")
        return

    # 读取CSV文件
    apis = []
    with open(csv_file, 'r', encoding='utf-8') as f:
        reader = csv.reader(f)
        header = next(reader)  # 跳过标题行

        for row in reader:
            if len(row) >= 7:
                name, method, path, desc, self_build, store_app, doc_link = row[:7]
                apis.append({
                    'name': name,
                    'method': method,
                    'path': path,
                    'description': desc,
                    'self_build': self_build,
                    'store_app': store_app,
                    'doc_link': doc_link
                })

    print(f"读取到 {len(apis)} 个API")

    # 查找实现位置
    results = []
    found_count = 0

    for i, api in enumerate(apis[:100], 1):  # 处理前100个API作为示例
        print(f"处理 {i}/100: {api['name']}")

        file_path, line_num, content = find_api_implementation(
            api['name'], api['method'], api['path']
        )

        if file_path:
            found_count += 1
            results.append({
                **api,
                'file_path': file_path,
                'line_number': line_num,
                'implementation_preview': content[:80] + "..." if len(content) > 80 else content,
                'status': 'found'
            })
        else:
            results.append({
                **api,
                'file_path': "未找到",
                'line_number': "-",
                'implementation_preview': "-",
                'status': 'not_found'
            })

    # 生成Markdown表格
    with open(output_file, 'w', encoding='utf-8') as f:
        f.write("# API实现映射表\n\n")
        f.write(f"**生成时间**: 2025-11-05  \n")
        f.write(f"**总API数（示例）**: 100  \n")
        f.write(f"**找到实现**: {found_count}  \n")
        f.write(f"**实现率**: {found_count/100*100:.1f}%  \n\n")

        f.write("| 序号 | API名称 | 请求方式 | API地址 | 文件路径 | 行号 | 实现预览 |\n")
        f.write("|------|---------|----------|---------|----------|------|----------|\n")

        for i, result in enumerate(results, 1):
            name = result['name'].replace('|', '\\|')
            method = result['method']
            path = result['path'].replace('|', '\\|')
            file_path = result['file_path'].replace('|', '\\|')
            line_num = result['line_number']
            preview = result['implementation_preview'].replace('|', '\\|')

            # 添加状态标识
            status_icon = "✅" if result['status'] == 'found' else "❌"
            name_with_status = f"{status_icon} {name}"

            f.write(f"| {i} | {name_with_status} | {method} | `{path}` | `{file_path}` | {line_num} | {preview} |\n")

        # 添加统计信息
        f.write("\n\n## 统计信息\n\n")

        # 按服务分类统计
        service_stats = {}
        for result in results:
            if result['status'] == 'found':
                path_parts = result['path'].split('/')
                if len(path_parts) >= 2 and path_parts[0] == 'open-apis':
                    service = path_parts[1]
                else:
                    service = 'unknown'
                service_stats[service] = service_stats.get(service, 0) + 1

        if service_stats:
            f.write("### 按服务分类的实现数量\n\n")
            for service, count in sorted(service_stats.items(), key=lambda x: x[1], reverse=True):
                f.write(f"- **{service}**: {count} 个API\n")

        # 未实现的API
        not_found = [r for r in results if r['status'] == 'not_found']
        if not_found:
            f.write(f"\n### 未实现的API ({len(not_found)}个)\n\n")
            for api in not_found[:10]:
                f.write(f"- {api['name']} ({api['method']} {api['path']})\n")
            if len(not_found) > 10:
                f.write(f"- ... 还有 {len(not_found) - 10} 个未实现的API\n")

    print(f"完成！映射表已保存到 {output_file}")
    return output_file

if __name__ == "__main__":
    process_api_list()