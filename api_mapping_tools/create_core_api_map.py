#!/usr/bin/env python3
"""
创建核心API实现映射表 - 专注于最重要的API
"""

import re
import csv
import os
import subprocess
from pathlib import Path

def find_api_implementation_smart(api_name, method, path):
    """智能API实现搜索"""

    # 从路径提取服务和操作信息
    path_parts = path.strip('/').split('/')
    if len(path_parts) >= 2 and path_parts[0] == 'open-apis':
        service_parts = path_parts[1:]
    else:
        service_parts = path_parts

    # 确定服务名称和版本
    service_name = service_parts[0] if service_parts else 'unknown'
    version = 'v1'
    for part in service_parts:
        if part.startswith('v') and part[1:].isdigit():
            version = part
            break

    # 构建可能的文件路径
    possible_paths = [
        f"src/service/{service_name}/{version}/mod.rs",
        f"src/service/{service_name}/{version}/",
        f"src/service/{service_name}/",
        f"src/service/{service_name}.rs",
    ]

    # 构建搜索关键词
    keywords = []

    # 1. 从路径最后部分提取
    if service_parts:
        last_part = service_parts[-1]
        keywords.append(re.sub(r'[^a-zA-Z0-9_]', '', last_part))

    # 2. 从API名称提取
    name_parts = api_name.replace('获取', '').replace('创建', '').replace('删除', '').replace('更新', '').replace('修改', '')
    name_parts = re.sub(r'[^a-zA-Z0-9\u4e00-\u9fff]', '', name_parts)
    if name_parts:
        keywords.append(name_parts)

    # 3. HTTP方法 + 路径组合
    if service_parts:
        http_keyword = f"{method.lower()}_{service_parts[-1]}"
        http_keyword = re.sub(r'[^a-zA-Z0-9_]', '', http_keyword)
        keywords.append(http_keyword)

    # 搜索实现
    for possible_path in possible_paths:
        if os.path.exists(possible_path.replace('/mod.rs', '')):
            search_dir = possible_path if possible_path.endswith('/') else possible_path.replace('/mod.rs', '')

            for keyword in keywords[:3]:
                try:
                    cmd = [
                        "grep", "-r", "-n", "--include=*.rs",
                        f"pub async fn.*{keyword}",
                        search_dir
                    ]

                    result = subprocess.run(cmd, capture_output=True, text=True, timeout=5)
                    if result.returncode == 0 and result.stdout.strip():
                        lines = result.stdout.strip().split('\n')
                        for line in lines[:1]:
                            if ':' in line:
                                parts = line.split(':', 2)
                                if len(parts) >= 3:
                                    file_path = parts[0]
                                    line_num = parts[1]
                                    content = parts[2].strip()

                                    abs_path = os.path.abspath(file_path)
                                    if os.path.exists(abs_path):
                                        rel_path = os.path.relpath(abs_path, os.getcwd())
                                        return rel_path, line_num, content
                except Exception:
                    continue

    # 如果精确搜索失败，进行更广泛的搜索
    try:
        cmd = [
            "grep", "-r", "-n", "--include=*.rs",
            f"pub async fn",
            f"src/service/{service_name}/"
        ]

        result = subprocess.run(cmd, capture_output=True, text=True, timeout=3)
        if result.returncode == 0 and result.stdout.strip():
            lines = result.stdout.strip().split('\n')
            for line in lines[:3]:  # 取前3个结果
                if ':' in line:
                    parts = line.split(':', 2)
                    if len(parts) >= 3:
                        file_path = parts[0]
                        line_num = parts[1]
                        content = parts[2].strip()
                        abs_path = os.path.abspath(file_path)
                        if os.path.exists(abs_path):
                            rel_path = os.path.relpath(abs_path, os.getcwd())
                            return rel_path, line_num, content
    except Exception:
        pass

    return None, None, None

def select_core_apis():
    """选择核心API"""
    csv_file = "server_api_list.csv"
    core_apis = []

    # 核心服务列表
    core_services = [
        'authen',      # 用户认证
        'auth',        # 访问令牌
        'contact',     # 通讯录
        'im',          # 消息
        'user',        # 用户管理
        'department',  # 部门管理
        'group',       # 群组
        'message',     # 消息相关
        'search'       # 搜索
    ]

    with open(csv_file, 'r', encoding='utf-8') as f:
        reader = csv.reader(f)
        next(reader)  # 跳过标题行

        for row in reader:
            if len(row) >= 7:
                name, method, path, desc, self_build, store_app, doc_link = row[:7]

                # 检查是否为核心服务
                is_core = any(core_service in path for core_service in core_services)

                # 额外选择一些重要的API
                important_keywords = [
                    'token', 'access_token', 'user_info', '创建', '删除', '更新',
                    '发送消息', '获取', '搜索', 'batch', 'list'
                ]

                is_important = any(keyword in name or keyword in path for keyword in important_keywords)

                if is_core or is_important:
                    core_apis.append({
                        'name': name,
                        'method': method,
                        'path': path,
                        'description': desc,
                        'self_build': self_build,
                        'store_app': store_app,
                        'doc_link': doc_link
                    })

                # 限制数量到300个
                if len(core_apis) >= 300:
                    break

    return core_apis

def main():
    """主函数"""
    csv_file = "server_api_list.csv"
    md_file = "core_api_implementation_map.md"

    if not os.path.exists(csv_file):
        print(f"错误: 找不到文件 {csv_file}")
        return

    print("开始创建核心API映射表...")

    # 选择核心API
    core_apis = select_core_apis()
    print(f"选择了 {len(core_apis)} 个核心API进行映射")

    # 搜索实现
    results = []
    found_count = 0

    for i, api in enumerate(core_apis, 1):
        print(f"处理 {i}/{len(core_apis)}: {api['name']}")

        file_path, line_num, content = find_api_implementation_smart(
            api['name'], api['method'], api['path']
        )

        if file_path:
            found_count += 1
            results.append({
                **api,
                'file_path': file_path,
                'line_number': line_num,
                'implementation': content[:80] + "..." if len(content) > 80 else content,
                'status': 'found'
            })
        else:
            results.append({
                **api,
                'file_path': "未找到",
                'line_number': "-",
                'implementation': "-",
                'status': 'not_found'
            })

    print(f"\n搜索完成！")
    print(f"- 核心API总数: {len(core_apis)}")
    print(f"- 找到实现: {found_count}")
    print(f"- 实现率: {found_count/len(core_apis)*100:.1f}%")

    # 生成Markdown表格
    with open(md_file, 'w', encoding='utf-8') as f:
        f.write("# 核心API实现映射表\n\n")
        f.write(f"**生成时间**: 2025-11-05  \n")
        f.write(f"**核心API总数**: {len(core_apis)}  \n")
        f.write(f"**已实现**: {found_count}  \n")
        f.write(f"**实现率**: {found_count/len(core_apis)*100:.1f}%  \n\n")

        f.write("| 序号 | API名称 | 请求方式 | API地址 | 文件路径 | 行号 | 实现预览 |\n")
        f.write("|------|---------|----------|---------|----------|------|----------|\n")

        for i, result in enumerate(results, 1):
            name = result['name'].replace('|', '\\|')
            method = result['method']
            path = result['path'].replace('|', '\\|')
            file_path = result['file_path'].replace('|', '\\|')
            line_num = result['line_number']
            impl = result['implementation'].replace('|', '\\|')

            # 为不同的状态添加标识
            status_icon = "✅" if result['status'] == 'found' else "❌"
            name_with_status = f"{status_icon} {name}"

            f.write(f"| {i} | {name_with_status} | {method} | `{path}` | `{file_path}` | {line_num} | {impl} |\n")

        # 添加分类统计
        f.write("\n\n## 分类统计\n\n")

        # 按服务分类
        service_stats = {}
        for result in results:
            if result['status'] == 'found':
                # 从路径中提取服务名
                path_parts = result['path'].split('/')
                if len(path_parts) >= 2 and path_parts[0] == 'open-apis':
                    service = path_parts[1]
                else:
                    service = 'unknown'
                service_stats[service] = service_stats.get(service, 0) + 1

        f.write("### 按服务分类的实现情况\n\n")
        for service, count in sorted(service_stats.items(), key=lambda x: x[1], reverse=True):
            percentage = (count / found_count) * 100 if found_count > 0 else 0
            f.write(f"- **{service}**: {count} 个API ({percentage:.1f}%)\n")

        # 未实现的API
        not_found = [r for r in results if r['status'] == 'not_found']
        if not_found:
            f.write(f"\n### 未实现的核心API ({len(not_found)}个)\n\n")
            for api in not_found[:10]:  # 只显示前10个
                f.write(f"- {api['name']} ({api['method']} {api['path']})\n")
            if len(not_found) > 10:
                f.write(f"- ... 还有 {len(not_found) - 10} 个未实现的核心API\n")

    print(f"核心API映射表已保存到 {md_file}")
    return md_file

if __name__ == "__main__":
    main()