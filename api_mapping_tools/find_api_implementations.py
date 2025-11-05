#!/usr/bin/env python3
"""
API定位脚本 - 为CSV中的API找到对应的实现文件和行号
"""

import re
import csv
import os
import subprocess
from pathlib import Path

def extract_api_name_from_path(path):
    """从API路径中提取API名称"""
    # 移除版本号和常见前缀
    clean_path = re.sub(r'/v\d+', '', path)
    clean_path = re.sub(r'/open-apis/', '', clean_path)

    # 转换为函数名格式
    parts = clean_path.split('/')
    if len(parts) >= 2:
        # 取最后两个部分作为函数名
        service_name = parts[0]
        action_name = parts[1]

        # 转换为snake_case
        def to_snake_case(s):
            s = re.sub(r'[^a-zA-Z0-9]', '_', s)
            s = re.sub(r'_+', '_', s)
            return s.strip('_')

        return f"{to_snake_case(service_name)}_{to_snake_case(action_name)}"

    return clean_path.replace('/', '_')

def find_api_implementation(api_name, method, path):
    """在代码库中搜索API实现"""

    # 搜索模式
    search_patterns = [
        # 基于路径的函数名
        extract_api_name_from_path(path),

        # HTTP方法和路径组合
        f"{method.lower()}_{path.split('/')[-1]}",
        f"{method.lower()}{''.join([p.capitalize() for p in path.split('/')[-2:]])}",

        # 通用模式
        path.split('/')[-1],
        '_'.join(path.split('/')[-2:]),
    ]

    results = []

    # 在src/service目录中搜索
    service_dir = Path("src/service")
    if service_dir.exists():
        for pattern in search_patterns:
            try:
                # 使用ripgrep搜索
                cmd = [
                    "rg", "-n", "--type", "rust",
                    f"pub async fn {pattern}",
                    str(service_dir),
                    "--max-count", "5"
                ]

                result = subprocess.run(cmd, capture_output=True, text=True)
                if result.returncode == 0 and result.stdout.strip():
                    lines = result.stdout.strip().split('\n')
                    for line in lines:
                        if ':' in line:
                            file_path, line_num, content = line.split(':', 2)
                            abs_path = os.path.abspath(file_path)
                            results.append((abs_path, line_num, content.strip()))
                            break  # 只取第一个结果

                if results:
                    break

            except Exception as e:
                print(f"搜索错误: {e}")
                continue

    # 如果没找到，尝试更广泛的搜索
    if not results:
        # 搜索包含路径关键词的文件
        path_keywords = [p for p in path.split('/') if p and p != 'open-apis']
        if path_keywords:
            keyword = path_keywords[0]
            try:
                cmd = [
                    "rg", "-n", "--type", "rust",
                    f"pub async fn.*{keyword}",
                    "src/service",
                    "--max-count", "3"
                ]

                result = subprocess.run(cmd, capture_output=True, text=True)
                if result.returncode == 0 and result.stdout.strip():
                    lines = result.stdout.strip().split('\n')
                    for line in lines:
                        if ':' in line:
                            file_path, line_num, content = line.split(':', 2)
                            abs_path = os.path.abspath(file_path)
                            results.append((abs_path, line_num, content.strip()))

            except Exception as e:
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

    # 搜索实现
    results = []
    for i, api in enumerate(apis, 1):
        print(f"处理 {i}/{len(apis)}: {api['name']}")

        file_path, line_num, content = find_api_implementation(
            api['name'], api['method'], api['path']
        )

        if file_path:
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

    # 生成Markdown表格
    with open(md_file, 'w', encoding='utf-8') as f:
        f.write("# API实现映射表\n\n")
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

    print(f"完成! 结果已保存到 {md_file}")

if __name__ == "__main__":
    main()