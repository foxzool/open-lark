#!/usr/bin/env python3
"""
完整API映射表生成器 - 处理所有1551个API
"""

import csv
import re
import os
import subprocess
from pathlib import Path
import time

def find_api_implementation_efficient(api_name, method, path):
    """高效的API实现查找"""

    # 从路径提取服务信息
    path_parts = path.strip('/').split('/')
    if len(path_parts) >= 2 and path_parts[0] == 'open-apis':
        service_parts = path_parts[1:]
        service_name = service_parts[0] if service_parts else 'unknown'

        # 提取版本
        version = 'v1'
        for part in service_parts:
            if part.startswith('v') and part[1:].isdigit():
                version = part
                break
    else:
        service_parts = path_parts
        service_name = service_parts[0] if service_parts else 'unknown'
        version = 'v1'

    # 优先搜索的服务目录路径
    search_dirs = []
    if service_name != 'unknown':
        search_dirs.extend([
            f"src/service/{service_name}/",
            f"src/service/{service_name}/{version}/"
        ])

    # 构建搜索关键词（按优先级）
    keywords = []

    # 1. 从路径最后部分提取
    if service_parts:
        last_part = service_parts[-1]
        clean_last = re.sub(r'[^a-zA-Z0-9_]', '', last_part)
        if clean_last:
            keywords.append(clean_last)

    # 2. 从API名称提取关键词
    name_parts = re.sub(r'(获取|创建|删除|更新|修改|查询|搜索)', '', api_name)
    name_parts = re.sub(r'[^a-zA-Z0-9\u4e00-\u9fff]', '', name_parts)
    if len(name_parts) >= 2:
        keywords.append(name_parts)

    # 3. HTTP方法 + 路径组合
    if service_parts:
        http_combo = f"{method.lower()}_{service_parts[-1]}"
        http_combo = re.sub(r'[^a-zA-Z0-9_]', '', http_combo)
        keywords.append(http_combo)

    # 4. 基于服务名的搜索
    if service_name != 'unknown':
        keywords.append(service_name)

    # 在最可能的目录中搜索
    for search_dir in search_dirs:
        if not os.path.exists(search_dir):
            continue

        for keyword in keywords[:3]:  # 限制关键词数量
            try:
                # 搜索相关函数
                cmd = [
                    "grep", "-r", "-n", "--include=*.rs",
                    f"pub async fn.*{keyword}",
                    search_dir
                ]

                result = subprocess.run(cmd, capture_output=True, text=True, timeout=3)
                if result.returncode == 0 and result.stdout.strip():
                    lines = result.stdout.strip().split('\n')
                    for line in lines[:1]:  # 只取第一个最佳匹配
                        if ':' in line and 'pub async fn' in line:
                            parts = line.split(':', 2)
                            if len(parts) >= 3:
                                file_path = parts[0]
                                line_num = parts[1]
                                content = parts[2].strip()

                                if os.path.exists(file_path):
                                    rel_path = os.path.relpath(file_path, os.getcwd())
                                    return rel_path, line_num, content
            except (subprocess.TimeoutExpired, Exception):
                continue

    # 如果精确搜索失败，在src/service目录中广泛搜索
    try:
        broader_cmd = [
            "grep", "-r", "-n", "--include=*.rs",
            f"pub async fn.*{keywords[0] if keywords else service_name}",
            "src/service/"
        ]

        result = subprocess.run(broader_cmd, capture_output=True, text=True, timeout=5)
        if result.returncode == 0 and result.stdout.strip():
            lines = result.stdout.strip().split('\n')
            for line in lines[:2]:  # 尝试前2个结果
                if ':' in line and 'pub async fn' in line:
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

def process_all_apis():
    """处理所有API并生成完整映射表"""
    csv_file = "server_api_list.csv"
    output_file = "complete_api_implementation_map.md"

    if not os.path.exists(csv_file):
        print(f"错误：找不到文件 {csv_file}")
        return

    # 读取所有API
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

    print(f"总共读取到 {len(apis)} 个API")

    # 分批处理API
    batch_size = 200
    total_batches = (len(apis) + batch_size - 1) // batch_size
    all_results = []
    found_count = 0
    start_time = time.time()

    for batch_idx in range(total_batches):
        start_idx = batch_idx * batch_size
        end_idx = min(start_idx + batch_size, len(apis))
        batch_apis = apis[start_idx:end_idx]

        print(f"\n处理批次 {batch_idx + 1}/{total_batches} (API {start_idx + 1}-{end_idx})")

        for i, api in enumerate(batch_apis):
            current_api_num = start_idx + i + 1
            if current_api_num % 50 == 0:  # 每50个显示进度
                elapsed = time.time() - start_time
                rate = current_api_num / elapsed if elapsed > 0 else 0
                remaining = (len(apis) - current_api_num) / rate if rate > 0 else 0
                print(f"  进度: {current_api_num}/{len(apis)} ({current_api_num/len(apis)*100:.1f}%) - "
                      f"速度: {rate:.1f} API/s - 预计剩余: {remaining/60:.1f}分钟")

            file_path, line_num, content = find_api_implementation_efficient(
                api['name'], api['method'], api['path']
            )

            if file_path:
                found_count += 1
                all_results.append({
                    **api,
                    'file_path': file_path,
                    'line_number': line_num,
                    'implementation_preview': content[:60] + "..." if len(content) > 60 else content,
                    'status': 'found'
                })
            else:
                all_results.append({
                    **api,
                    'file_path': "未找到",
                    'line_number': "-",
                    'implementation_preview': "-",
                    'status': 'not_found'
                })

        # 每批次后短暂休息，避免系统过载
        if batch_idx < total_batches - 1:
            time.sleep(1)

    total_time = time.time() - start_time
    print(f"\n处理完成！")
    print(f"- 总API数: {len(apis)}")
    print(f"- 找到实现: {found_count}")
    print(f"- 实现率: {found_count/len(apis)*100:.1f}%")
    print(f"- 总耗时: {total_time/60:.1f} 分钟")
    print(f"- 平均速度: {len(apis)/total_time:.1f} API/秒")

    # 生成Markdown文件
    print(f"\n生成映射表文件...")
    with open(output_file, 'w', encoding='utf-8') as f:
        f.write("# 完整API实现映射表\n\n")
        f.write(f"**生成时间**: 2025-11-05  \n")
        f.write(f"**总API数**: {len(apis)}  \n")
        f.write(f"**已实现**: {found_count}  \n")
        f.write(f"**实现率**: {found_count/len(apis)*100:.1f}%  \n")
        f.write(f"**处理耗时**: {total_time/60:.1f} 分钟  \n\n")

        f.write("| 序号 | API名称 | 请求方式 | API地址 | 文件路径 | 行号 | 状态 |\n")
        f.write("|------|---------|----------|---------|----------|------|------|\n")

        for i, result in enumerate(all_results, 1):
            name = result['name'].replace('|', '\\|')
            method = result['method']
            path = result['path'].replace('|', '\\|')
            file_path = result['file_path'].replace('|', '\\|')
            line_num = result['line_number']
            status = "✅ 已实现" if result['status'] == 'found' else "❌ 未实现"

            f.write(f"| {i} | {name} | {method} | `{path}` | `{file_path}` | {line_num} | {status} |\n")

        # 添加统计信息
        f.write("\n\n## 实现统计\n\n")

        # 按服务分类
        service_stats = {}
        for result in all_results:
            if result['status'] == 'found':
                path_parts = result['path'].split('/')
                if len(path_parts) >= 2 and path_parts[0] == 'open-apis':
                    service = path_parts[1]
                else:
                    service = 'unknown'
                service_stats[service] = service_stats.get(service, 0) + 1

        if service_stats:
            f.write("### 按服务分类的实现情况\n\n")
            for service, count in sorted(service_stats.items(), key=lambda x: x[1], reverse=True):
                percentage = (count / found_count) * 100 if found_count > 0 else 0
                f.write(f"- **{service}**: {count} 个API ({percentage:.1f}%)\n")

        # 未实现的API
        not_found = [r for r in all_results if r['status'] == 'not_found']
        if not_found:
            f.write(f"\n### 未实现的API ({len(not_found)}个)\n\n")
            f.write("以下是前50个未实现的API:\n\n")
            for api in not_found[:50]:
                f.write(f"- {api['name']} ({api['method']} {api['path']})\n")
            if len(not_found) > 50:
                f.write(f"- ... 还有 {len(not_found) - 50} 个未实现的API\n")

    print(f"完整API映射表已保存到: {output_file}")
    return output_file

if __name__ == "__main__":
    process_all_apis()