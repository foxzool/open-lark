#!/usr/bin/env python3
"""
完整的1551个API处理器 - 生成全面的API实现映射表
优化的版本，专门处理所有飞书开放平台API
"""

import csv
import re
import os
import subprocess
from pathlib import Path
import time
import json
from datetime import datetime

class APIProcessor:
    def __init__(self):
        self.results = []
        self.found_count = 0
        self.processed_count = 0
        self.start_time = None
        self.service_stats = {}

    def extract_service_info(self, path):
        """提取服务信息的独立方法"""
        path_parts = path.strip('/').split('/')
        if len(path_parts) >= 2 and path_parts[0] == 'open-apis':
            service = path_parts[1]
            version = 'v1'
            for part in path_parts[1:]:
                if part.startswith('v') and part[1:].isdigit():
                    version = part
                    break
            return service, version
        return 'unknown', 'v1'

    def update_service_stats(self, service, found):
        """统一的服务统计更新方法"""
        if service not in self.service_stats:
            self.service_stats[service] = {
                'found': 0,
                'total': 0,
                'rate': 0.0
            }

        self.service_stats[service]['total'] += 1
        if found:
            self.service_stats[service]['found'] += 1

        # 实时计算实现率
        self.service_stats[service]['rate'] = (
            self.service_stats[service]['found'] /
            self.service_stats[service]['total'] * 100
        )

    def find_api_implementation_optimized(self, api_name, method, path):
        """优化的API实现查找"""

        # 使用新的服务信息提取方法
        service_name, version = self.extract_service_info(path)

        # 从路径提取service_parts用于关键词搜索
        path_parts = path.strip('/').split('/')
        service_parts = path_parts[1:] if len(path_parts) >= 2 and path_parts[0] == 'open-apis' else path_parts

        # 优先搜索的服务目录路径
        search_dirs = []
        if service_name != 'unknown':
            search_dirs.extend([
                f"../src/service/{service_name}/",
                f"../src/service/{service_name}/{version}/"
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

        # 5. 特殊映射规则（提高准确性）
        special_mappings = {
            'user_info': ['user_info'],
            'tenant_access_token': ['tenant_access_token'],
            'app_access_token': ['app_access_token'],
            'app_ticket': ['app_ticket'],
            'message': ['message', 'send_message'],
            'users': ['user'],
            'departments': ['department'],
            'employee': ['employee'],
            'sessions': ['session'],
            'scopes': ['scope'],
            'groups': ['group'],
            'roles': ['role'],
            'files': ['file'],
            'sheets': ['sheet'],
            'tasks': ['task'],
            'events': ['event'],
            'comments': ['comment'],
            'approval': ['approval'],
            'calendar': ['calendar'],
            'meeting': ['meeting'],
            'sheets': ['sheets'],
            'bitable': ['bitable']
        }

        for special_key, special_keywords in special_mappings.items():
            if special_key in path.lower() or special_key in api_name.lower():
                keywords.extend(special_keywords)

        # 去重并限制关键词数量
        keywords = list(dict.fromkeys(keywords))[:5]

        # 在最可能的目录中搜索
        for search_dir in search_dirs:
            if not os.path.exists(search_dir):
                continue

            for keyword in keywords:
                try:
                    # 搜索相关函数
                    cmd = [
                        "grep", "-r", "-n", "--include=*.rs",
                        f"pub async fn.*{keyword}",
                        search_dir
                    ]

                    result = subprocess.run(cmd, capture_output=True, text=True, timeout=2)
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
                                        # 更新服务统计
                                        if service_name not in self.service_stats:
                                            self.service_stats[service_name] = {'found': 0, 'total': 0}
                                        return rel_path, line_num, content
                except (subprocess.TimeoutExpired, Exception):
                    continue

        # 如果精确搜索失败，在src/service目录中广泛搜索
        try:
            broader_cmd = [
                "grep", "-r", "-n", "--include=*.rs",
                f"pub async fn.*{keywords[0] if keywords else service_name}",
                "../src/service/"
            ]

            result = subprocess.run(broader_cmd, capture_output=True, text=True, timeout=3)
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

    def process_single_api(self, api, index, total):
        """处理单个API"""
        try:
            file_path, line_num, content = self.find_api_implementation_optimized(
                api['name'], api['method'], api['path']
            )

            # 提取服务信息并更新统计
            service, _ = self.extract_service_info(api['path'])
            found = file_path is not None

            # 使用统一的统计更新方法
            self.update_service_stats(service, found)

            if found:
                self.found_count += 1

                result = {
                    **api,
                    'file_path': file_path,
                    'line_number': line_num,
                    'implementation_preview': content[:50] + "..." if len(content) > 50 else content,
                    'status': 'found'
                }
            else:
                result = {
                    **api,
                    'file_path': "未找到",
                    'line_number': "-",
                    'implementation_preview': "-",
                    'status': 'not_found'
                }

            self.results.append(result)
            self.processed_count += 1

            # 进度报告
            if index % 100 == 0 or index == total:
                elapsed = time.time() - self.start_time
                rate = index / elapsed if elapsed > 0 else 0
                remaining = (total - index) / rate if rate > 0 else 0
                progress_pct = (index / total) * 100
                found_rate = (self.found_count / index) * 100 if index > 0 else 0

                print(f"进度: {index}/{total} ({progress_pct:.1f}%) - "
                      f"找到实现: {self.found_count} ({found_rate:.1f}%) - "
                      f"速度: {rate:.1f} API/s - 预计剩余: {remaining/60:.1f}分钟")

        except Exception as e:
            print(f"处理API {api['name']} 时出错: {e}")

    def process_all_apis(self):
        """处理所有API"""
        csv_file = "server_api_list.csv"
        output_file = "../complete_all_api_implementation_map.md"
        json_file = "../api_implementation_data.json"

        if not os.path.exists(csv_file):
            print(f"错误：找不到文件 {csv_file}")
            return

        print(f"开始处理完整的API列表...")
        self.start_time = time.time()

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

        # 处理所有API
        for i, api in enumerate(apis, 1):
            self.process_single_api(api, i, len(apis))

        # 生成报告
        self.generate_reports(len(apis), output_file, json_file)

    def analyze_service_coverage(self):
        """分析服务覆盖率"""
        analysis = {
            'high_coverage': [],    # 实现率 >= 80%
            'medium_coverage': [],  # 实现率 50-79%
            'low_coverage': [],     # 实现率 < 50%
            'no_coverage': []       # 实现率 = 0%
        }

        for service, stats in self.service_stats.items():
            if stats['total'] == 0:
                continue

            rate = stats['rate']
            service_info = {
                'name': service,
                'found': stats['found'],
                'total': stats['total'],
                'rate': rate
            }

            if rate >= 80:
                analysis['high_coverage'].append(service_info)
            elif rate >= 50:
                analysis['medium_coverage'].append(service_info)
            elif rate > 0:
                analysis['low_coverage'].append(service_info)
            else:
                analysis['no_coverage'].append(service_info)

        return analysis

    def generate_module_grouped_report(self, f, sorted_services):
        """生成按模块分组的报告"""
        f.write("\n\n## 按模块分组的API实现情况\n\n")

        for service, stats in sorted_services:
            if stats['total'] == 0:
                continue

            # 模块标题
            rate = stats['rate']
            status_emoji = "🟢" if rate >= 80 else "🟡" if rate >= 50 else "🔴"
            f.write(f"### {status_emoji} {service.upper()} 模块 ({stats['found']}/{stats['total']} - {rate:.1f}%)\n\n")

            # 该模块的API列表
            module_apis = [r for r in self.results
                          if self.extract_service_info(r['path'])[0] == service]

            if module_apis:
                f.write("| 序号 | API名称 | 请求方式 | API地址 | 状态 |\n")
                f.write("|------|---------|----------|---------|------|\n")

                for i, api in enumerate(module_apis, 1):
                    name = api['name'].replace('|', '\\|')
                    method = api['method']
                    path = api['path'].replace('|', '\\|')
                    status = "✅" if api['status'] == 'found' else "❌"

                    f.write(f"| {i} | {name} | {method} | `{path}` | {status} |\n")

                f.write("\n")

    def generate_summary_report(self, f):
        """生成汇总报告"""
        analysis = self.analyze_service_coverage()

        f.write("## 实现覆盖率分析\n\n")
        f.write(f"🟢 **高覆盖率模块 (≥80%)**: {len(analysis['high_coverage'])} 个\n")
        f.write(f"🟡 **中等覆盖率模块 (50-79%)**: {len(analysis['medium_coverage'])} 个\n")
        f.write(f"🔴 **低覆盖率模块 (<50%)**: {len(analysis['low_coverage'])} 个\n")
        f.write(f"⚫ **零覆盖率模块**: {len(analysis['no_coverage'])} 个\n\n")

        # 优先改进建议
        if analysis['low_coverage']:
            f.write("### 🚀 优先改进建议\n\n")
            f.write("以下模块实现率较低，建议优先完善：\n\n")
            for service in sorted(analysis['low_coverage'], key=lambda x: x['rate'])[:5]:
                f.write(f"- **{service['name']}**: {service['found']}/{service['total']} ({service['rate']:.1f}%)\n")

    def generate_reports(self, total_apis, md_file, json_file):
        """生成报告文件"""
        total_time = time.time() - self.start_time
        print(f"\n处理完成！")
        print(f"- 总API数: {total_apis}")
        print(f"- 找到实现: {self.found_count}")
        print(f"- 实现率: {self.found_count/total_apis*100:.1f}%")
        print(f"- 总耗时: {total_time/60:.1f} 分钟")
        print(f"- 平均速度: {total_apis/total_time:.1f} API/秒")

        # 保存JSON数据
        with open(json_file, 'w', encoding='utf-8') as f:
            json.dump({
                'metadata': {
                    'total_apis': total_apis,
                    'found_count': self.found_count,
                    'implementation_rate': self.found_count/total_apis*100,
                    'processing_time': total_time,
                    'generated_at': datetime.now().isoformat()
                },
                'service_stats': self.service_stats,
                'results': self.results
            }, f, ensure_ascii=False, indent=2)

        # 生成Markdown报告
        print(f"\n生成Markdown报告...")
        with open(md_file, 'w', encoding='utf-8') as f:
            f.write("# 完整API实现映射表\n\n")
            f.write(f"**生成时间**: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}  \n")
            f.write(f"**总API数**: {total_apis}  \n")
            f.write(f"**已实现**: {self.found_count}  \n")
            f.write(f"**实现率**: {self.found_count/total_apis*100:.1f}%  \n")
            f.write(f"**处理耗时**: {total_time/60:.1f} 分钟  \n")
            f.write(f"**处理速度**: {total_apis/total_time:.1f} API/秒  \n\n")

            f.write("| 序号 | API名称 | 请求方式 | API地址 | 文档链接 | 文件路径 | 行号 | 状态 |\n")
            f.write("|------|---------|----------|---------|----------|----------|------|------|\n")

            for i, result in enumerate(self.results, 1):
                raw_name = result['name']
                name = raw_name.replace('|', '\\|')
                method = result['method']
                path = result['path'].replace('|', '\\|')
                file_path = result['file_path'].replace('|', '\\|')
                line_num = result['line_number']
                status = "✅ 已实现" if result['status'] == 'found' else "❌ 未实现"
                doc_link = result.get('doc_link', '') or ''
                doc_cell = doc_link.replace('|', '\\|') if doc_link else "-"
                if doc_link:
                    name_cell = f"[{name}]({doc_cell})"
                else:
                    name_cell = name

                f.write(
                    f"| {i} | {name_cell} | {method} | `{path}` | {doc_cell} | `{file_path}` | {line_num} | {status} |\n"
                )

            # 添加统计信息
            f.write("\n\n## 实现统计\n\n")

            # 改进的排序逻辑：按实现率排序，实现率相同的按服务名排序
            sorted_services = sorted(
                self.service_stats.items(),
                key=lambda x: (-x[1]['rate'], x[0])  # 实现率降序，服务名升序
            )

            # 生成汇总报告
            self.generate_summary_report(f)

            # 生成按模块分组的详细报告
            self.generate_module_grouped_report(f, sorted_services)

            # 未实现的API
            not_found = [r for r in self.results if r['status'] == 'not_found']
            if not_found:
                f.write(f"\n### 未实现的API ({len(not_found)}个)\n\n")
                f.write("以下是前100个未实现的API:\n\n")
                for api in not_found[:100]:
                    f.write(f"- {api['name']} ({api['method']} {api['path']})\n")
                if len(not_found) > 100:
                    f.write(f"- ... 还有 {len(not_found) - 100} 个未实现的API\n")

        print(f"完整API映射表已保存到: {md_file}")
        print(f"详细数据已保存到: {json_file}")

if __name__ == "__main__":
    processor = APIProcessor()
    processor.process_all_apis()
