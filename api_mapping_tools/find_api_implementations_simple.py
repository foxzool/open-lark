#!/usr/bin/env python3
"""
API定位脚本 - 使用系统工具为CSV中的API找到对应的实现文件和行号
"""

import re
import csv
import os
import subprocess
from pathlib import Path

def find_api_implementation_grep(api_name, method, path):
    """使用grep在代码库中搜索API实现"""

    # 从路径提取关键词
    path_parts = path.strip('/').split('/')
    if len(path_parts) >= 2 and path_parts[0] == 'open-apis':
        service_parts = path_parts[1:]  # 跳过 'open-apis'
    else:
        service_parts = path_parts

    # 搜索模式
    search_patterns = []

    # 基于路径的最后几部分
    if len(service_parts) >= 2:
        # 组合最后两个部分
        pattern = '_'.join(service_parts[-2:])
        pattern = re.sub(r'[^a-zA-Z0-9_]', '_', pattern)
        pattern = re.sub(r'_+', '_', pattern)
        search_patterns.append(pattern)

    # 最后一个部分
    if service_parts:
        last_part = service_parts[-1]
        last_part = re.sub(r'[^a-zA-Z0-9_]', '_', last_part)
        search_patterns.append(last_part)

    # 基于API名称
    name_pattern = api_name.replace(' ', '_').replace('-', '_')
    name_pattern = re.sub(r'[^a-zA-Z0-9_]', '_', name_pattern)
    name_pattern = re.sub(r'_+', '_', name_pattern)
    search_patterns.append(name_pattern)

    # HTTP方法 + 路径组合
    if service_parts:
        method_pattern = method.lower()
        path_pattern = service_parts[-1]
        path_pattern = re.sub(r'[^a-zA-Z0-9_]', '_', path_pattern)
        search_patterns.append(f"{method_pattern}_{path_pattern}")

    results = []

    # 在src/service目录中搜索
    service_dir = Path("src/service")
    if not service_dir.exists():
        return (None, None, None)

    for pattern in search_patterns[:3]:  # 限制搜索模式数量
        try:
            # 使用grep搜索
            cmd = [
                "grep", "-r", "-n", "--include=*.rs",
                f"pub async fn.*{pattern}",
                str(service_dir)
            ]

            result = subprocess.run(cmd, capture_output=True, text=True, timeout=10)
            if result.returncode == 0 and result.stdout.strip():
                lines = result.stdout.strip().split('\n')
                for line in lines[:2]:  # 只取前两个结果
                    if ':' in line:
                        parts = line.split(':', 2)
                        if len(parts) >= 3:
                            file_path = parts[0]
                            line_num = parts[1]
                            content = parts[2].strip()

                            # 转换为绝对路径
                            abs_path = os.path.abspath(file_path)
                            if os.path.exists(abs_path):
                                results.append((abs_path, line_num, content))
                                break

                if results:
                    break

        except (subprocess.TimeoutExpired, Exception) as e:
            print(f"搜索错误 {pattern}: {e}")
            continue

    # 如果没找到，尝试更广泛的搜索
    if not results and service_parts:
        try:
            keyword = service_parts[0]
            cmd = [
                "grep", "-r", "-n", "--include=*.rs",
                f"pub async fn.*{keyword}",
                str(service_dir)
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
                            abs_path = os.path.abspath(file_path)
                            if os.path.exists(abs_path):
                                results.append((abs_path, line_num, content))

        except (subprocess.TimeoutExpired, Exception) as e:
            print(f"广泛搜索错误: {e}")

    return results[0] if results else (None, None, None)

def main():
    """主函数"""
    csv_file = "server_api_list.csv"
    md_file = "api_implementation_map.md"

    if not os.path.exists(csv_file):
        print(f"错误: 找不到文件 {csv_file}")
        return

    print("开始处理API列表...")

    # 读取CSV文件的前50行作为示例
    apis = []
    with open(csv_file, 'r', encoding='utf-8') as f:
        reader = csv.reader(f)
        header = next(reader)  # 跳过标题行

        for i, row in enumerate(reader):
            if i >= 50:  # 只处理前50个作为示例
                break
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

    print(f"读取到 {len(apis)} 个API（示例数据）")

    # 搜索实现
    results = []
    found_count = 0
    for i, api in enumerate(apis, 1):
        print(f"处理 {i}/{len(apis)}: {api['name']}")

        file_path, line_num, content = find_api_implementation_grep(
            api['name'], api['method'], api['path']
        )

        if file_path:
            found_count += 1
            # 转换为相对路径
            rel_path = os.path.relpath(file_path, os.getcwd())
            results.append({
                **api,
                'file_path': rel_path,
                'line_number': line_num,
                'implementation': content[:100] + "..." if len(content) > 100 else content
            })
        else:
            results.append({
                **api,
                'file_path': "未找到",
                'line_number': "-",
                'implementation': "-"
            })

    print(f"搜索完成，找到 {found_count} 个API实现")

    # 生成Markdown表格
    with open(md_file, 'w', encoding='utf-8') as f:
        f.write("# API实现映射表（前50个API示例）\n\n")
        f.write("| 名称 | 请求方式 | 地址 | 文件路径 | 行号 | 实现预览 |\n")
        f.write("|------|----------|------|----------|------|----------|\n")

        for result in results:
            name = result['name'].replace('|', '\\|')
            method = result['method']
            path = result['path'].replace('|', '\\|')
            file_path = result['file_path'].replace('|', '\\|')
            line_num = result['line_number']
            impl = result['implementation'].replace('|', '\\|')

            f.write(f"| {name} | {method} | {path} | `{file_path}` | {line_num} | {impl} |\n")

        f.write(f"\n\n**统计信息：**\n")
        f.write("- 总API数：50（示例）\n")
        f.write(f"- 找到实现：{found_count}\n")
        f.write(f"- 实现率：{found_count/50*100:.1f}%\n")

    print(f"完成! 结果已保存到 {md_file}")

if __name__ == "__main__":
    main()