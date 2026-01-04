#!/usr/bin/env python3
"""
API 验证脚本

对比 CSV 文件中的 API 列表与实际代码实现，生成完成度报告。
"""

import csv
import os
import re
from pathlib import Path
from typing import Dict, List, Tuple, Set
from dataclasses import dataclass, field
from collections import defaultdict


@dataclass
class APIInfo:
    """API 信息"""
    api_id: str
    name: str
    biz_tag: str
    meta_project: str
    meta_version: str
    meta_resource: str
    meta_name: str
    url: str
    doc_path: str
    expected_file: str = ""
    is_implemented: bool = False


class APIValidator:
    """API 验证器"""

    def __init__(self, csv_path: str, src_path: str, filter_tags: List[str] = None):
        self.csv_path = csv_path
        self.src_path = Path(src_path)
        self.filter_tags = filter_tags
        self.apis: List[APIInfo] = []
        self.implemented_files: Set[str] = set()
        self.missing_apis: List[APIInfo] = []
        self.extra_files: Set[str] = set()

    def parse_csv(self):
        """解析 CSV 文件"""
        print(f"📄 读取 CSV 文件: {self.csv_path}")

        if self.filter_tags:
            print(f"🏷️  过滤业务标签: {', '.join(self.filter_tags)}")

        with open(self.csv_path, 'r', encoding='utf-8') as f:
            reader = csv.DictReader(f)

            for row in reader:
                # 如果设置了过滤器，只处理匹配的业务标签
                if self.filter_tags and row['bizTag'] not in self.filter_tags:
                    continue

                api = APIInfo(
                    api_id=row['id'],
                    name=row['name'],
                    biz_tag=row['bizTag'],
                    meta_project=row['meta.Project'],
                    meta_version=row['meta.Version'],
                    meta_resource=row['meta.Resource'],
                    meta_name=row['meta.Name'],
                    url=row['url'],
                    doc_path=row['docPath']
                )

                # 生成预期的文件路径
                api.expected_file = self._generate_expected_file_path(api)

                self.apis.append(api)

        print(f"✅ 解析完成，共 {len(self.apis)} 个 API")

    def _generate_expected_file_path(self, api: APIInfo) -> str:
        """根据 API 信息生成预期的文件路径"""

        # 提取 HTTP 方法和路径
        match = re.match(r'(\w+):(.+)', api.url)
        if not match:
            return ""

        method = match.group(1)
        path = match.group(2)

        # 根据不同的业务标签和项目生成路径
        if api.biz_tag == 'calendar':
            # calendar/calendar/v4/{resource}/{operation}.rs
            operation = self._get_operation_from_url(method, path, api.meta_name)
            # meta.Resource 可能是 "calendar" 或 "calendar.event" 或 "calendar.acl"
            # 需要将点号替换为斜杠
            resource_path = api.meta_resource.replace('.', '/')
            return f"calendar/calendar/v4/{resource_path}/{operation}.rs"

        elif api.biz_tag == 'vc':
            # vc/vc/v1/{resource}/{operation}.rs
            operation = self._get_operation_from_url(method, path, api.meta_name)
            # meta.Resource 可能包含嵌套路径，如 "meeting.recording" -> "meeting/recording"
            resource_path = api.meta_resource.replace('.', '/')
            return f"vc/vc/v1/{resource_path}/{operation}.rs"

        elif api.biz_tag == 'meeting_room':
            # meeting_room/vc_meeting/old/default/{resource}/{operation}.rs
            # 对于 meeting_room，meta.Name 格式为 "building/create"，"room/list" 等
            # 需要从 meta.Name 中提取资源和操作
            name_parts = api.meta_name.split('/')
            if len(name_parts) >= 2:
                resource = name_parts[0].lower()
                operation = name_parts[1].lower()
                return f"meeting_room/vc_meeting/old/default/{resource}/{operation}.rs"

            # 回退到从 URL 提取
            operation = self._get_operation_from_url(method, path, api.meta_name, old_version=True)
            return f"meeting_room/vc_meeting/old/default/{operation}.rs"

        return ""

    def _get_operation_from_url(self, method: str, path: str, meta_name: str = "", old_version: bool = False) -> str:
        """从 URL 提取操作类型"""

        # 如果 meta_name 存在且不是通用名称，直接使用 meta_name 作为操作名
        # 'default' 是唯一需要跳过的通用名称，'list' 和 'query' 都是有效的操作名
        if meta_name and meta_name.lower() != 'default':
            operation = meta_name.lower()

            # 特殊处理某些操作名
            special_mappings = {
                'batch': 'batch_get',
                'primarys': 'primarys',
                'mget': 'mget',
                'primary': 'primary',
                'subscription': 'subscription',
                'unsubscription': 'unsubscription',
                'instance_view': 'instance_view',
                'instances': 'instances',
                'reply': 'reply',
                'batch_delete': 'batch_delete',
                'meeting_chat': 'meeting_chat',
                'meeting_minute': 'meeting_minute',
                'chat_member': 'chat_member',
            }

            if operation in special_mappings:
                return special_mappings[operation]

            # 根据方法推断操作类型（只在 meta_name 没有明确操作名时）
            # 注意：list, search 等操作名应该直接使用，不根据方法推断
            if operation not in ['list', 'search', 'query', 'primary', 'primarys', 'mget']:
                if method == 'POST' and not old_version:
                    return 'create'
                elif method == 'GET' and not old_version:
                    return 'get'

            return operation

        # 移除路径参数，例如 :calendar_id
        path = re.sub(r'/:[^/]+', '', path)

        # 分割路径
        parts = [p for p in path.split('/') if p]

        # 获取最后一个部分作为操作
        if parts:
            operation = parts[-1].lower()

            # 特殊映射
            if operation == 'primarys':
                return 'primarys'
            elif operation == 'mget':
                return 'mget'
            elif operation == 'primary':
                return 'primary'
            elif operation == 'calendars':
                return method.lower() if method.lower() in ['post', 'get'] else 'list'
            elif operation == 'subscription':
                return 'subscription'
            elif operation == 'unsubscription':
                return 'unsubscription'
            elif method == 'POST' and not old_version:
                return 'create'
            elif method == 'GET' and not old_version:
                return 'get'
            elif method == 'PATCH':
                return 'patch'
            elif method == 'DELETE':
                return 'delete'

            return operation

        return ""

    def scan_implementations(self):
        """扫描实际实现的文件"""
        print(f"🔍 扫描代码实现目录: {self.src_path}")

        for root, dirs, files in os.walk(self.src_path):
            # 跳过 __pycache__ 等目录
            dirs[:] = [d for d in dirs if not d.startswith('.') and d != '__pycache__']

            for file in files:
                if file.endswith('.rs') and file != 'mod.rs':
                    # 获取相对路径
                    full_path = os.path.join(root, file)
                    rel_path = os.path.relpath(full_path, self.src_path)

                    # 将路径分隔符转换为 /
                    rel_path = rel_path.replace('\\', '/')
                    self.implemented_files.add(rel_path)

        print(f"✅ 扫描完成，找到 {len(self.implemented_files)} 个实现文件")

    def compare(self):
        """对比 CSV 和实际实现"""
        print("🔬 开始对比分析...")

        for api in self.apis:
            if api.expected_file and api.expected_file in self.implemented_files:
                api.is_implemented = True
            else:
                api.is_implemented = False
                self.missing_apis.append(api)

        # 找出额外的文件（不在 CSV 中的）
        expected_files = set(api.expected_file for api in self.apis if api.expected_file)
        self.extra_files = self.implemented_files - expected_files

        print(f"✅ 对比完成")
        print(f"   - 已实现: {len([a for a in self.apis if a.is_implemented])}")
        print(f"   - 未实现: {len(self.missing_apis)}")
        print(f"   - 额外文件: {len(self.extra_files)}")

    def generate_report(self, output_path: str):
        """生成报告"""
        print(f"📝 生成报告: {output_path}")

        with open(output_path, 'w', encoding='utf-8') as f:
            # 标题
            f.write("# API 验证报告\n\n")
            f.write(f"**生成时间**: {self._get_timestamp()}\n")
            f.write(f"**CSV 文件**: {self.csv_path}\n")
            f.write(f"**源码目录**: {self.src_path}\n\n")

            # 总体统计
            f.write("## 一、总体统计\n\n")
            total_apis = len(self.apis)
            implemented = len([a for a in self.apis if a.is_implemented])
            missing = len(self.missing_apis)
            completion_rate = (implemented / total_apis * 100) if total_apis > 0 else 0

            f.write(f"| 指标 | 数量 |\n")
            f.write(f"|------|------|\n")
            f.write(f"| **API 总数** | {total_apis} |\n")
            f.write(f"| **已实现** | {implemented} |\n")
            f.write(f"| **未实现** | {missing} |\n")
            f.write(f"| **完成率** | {completion_rate:.1f}% |\n")
            f.write(f"| **额外文件** | {len(self.extra_files)} |\n\n")

            # 按模块统计
            f.write("## 二、模块统计\n\n")

            module_stats = self._calculate_module_stats()

            f.write("| 模块 | API 数量 | 已实现 | 未实现 | 完成率 |\n")
            f.write("|------|---------|--------|--------|--------|\n")

            for module_name, stats in sorted(module_stats.items()):
                f.write(f"| {module_name} | {stats['total']} | {stats['implemented']} | "
                       f"{stats['missing']} | {stats['rate']:.1f}% |\n")

            f.write("\n")

            # 未实现的 API
            if self.missing_apis:
                f.write("## 三、未实现的 API\n\n")

                # 按模块分组
                missing_by_module = defaultdict(list)
                for api in self.missing_apis:
                    module_name = api.biz_tag.upper()
                    missing_by_module[module_name].append(api)

                for module_name in sorted(missing_by_module.keys()):
                    f.write(f"### {module_name} ({len(missing_by_module[module_name])} 个)\n\n")

                    for api in sorted(missing_by_module[module_name], key=lambda x: x.name):
                        f.write(f"#### {api.name}\n\n")
                        f.write(f"- **API ID**: {api.api_id}\n")
                        f.write(f"- **预期文件**: `{api.expected_file}`\n")
                        f.write(f"- **URL**: {api.url}\n")
                        f.write(f"- **文档**: {api.doc_path}\n\n")

            # 额外的文件
            if self.extra_files:
                f.write("## 四、额外的实现文件\n\n")
                f.write("这些文件存在于代码中，但不在 CSV API 列表中：\n\n")

                for file in sorted(self.extra_files):
                    f.write(f"- `{file}`\n")

                f.write("\n")

            # 完成 API 列表
            f.write("## 五、已实现的 API\n\n")

            implemented_by_module = defaultdict(list)
            for api in self.apis:
                if api.is_implemented:
                    module_name = api.biz_tag.upper()
                    implemented_by_module[module_name].append(api)

            for module_name in sorted(implemented_by_module.keys()):
                f.write(f"### {module_name} ({len(implemented_by_module[module_name])} 个)\n\n")

                for api in sorted(implemented_by_module[module_name], key=lambda x: x.name):
                    f.write(f"- ✅ {api.name} (`{api.expected_file}`)\n")

                f.write("\n")

            print(f"✅ 报告生成完成")

    def _calculate_module_stats(self) -> Dict[str, Dict]:
        """计算各模块的统计数据"""
        module_stats = defaultdict(lambda: {'total': 0, 'implemented': 0, 'missing': 0, 'rate': 0.0})

        for api in self.apis:
            module_name = api.biz_tag.upper()
            module_stats[module_name]['total'] += 1

            if api.is_implemented:
                module_stats[module_name]['implemented'] += 1
            else:
                module_stats[module_name]['missing'] += 1

        # 计算完成率
        for stats in module_stats.values():
            if stats['total'] > 0:
                stats['rate'] = (stats['implemented'] / stats['total']) * 100

        return dict(module_stats)

    @staticmethod
    def _get_timestamp() -> str:
        """获取当前时间戳"""
        from datetime import datetime
        return datetime.now().strftime("%Y-%m-%d %H:%M:%S")


def main():
    """主函数"""
    import argparse

    parser = argparse.ArgumentParser(description='API 验证脚本')
    parser.add_argument('--csv', default='api_list_export.csv',
                       help='CSV 文件路径 (默认: api_list_export.csv)')
    parser.add_argument('--src', default='crates/openlark-meeting/src',
                       help='源码目录路径 (默认: crates/openlark-meeting/src)')
    parser.add_argument('--output', default='API_VALIDATION_REPORT.md',
                       help='报告输出路径 (默认: API_VALIDATION_REPORT.md)')
    parser.add_argument('--filter', nargs='+',
                       help='过滤业务标签 (例如: --filter calendar vc meeting_room)')

    args = parser.parse_args()

    print("=" * 60)
    print("🚀 API 验证工具")
    print("=" * 60)
    print()

    # 验证输入
    if not os.path.exists(args.csv):
        print(f"❌ 错误: CSV 文件不存在: {args.csv}")
        return 1

    if not os.path.exists(args.src):
        print(f"❌ 错误: 源码目录不存在: {args.src}")
        return 1

    # 执行验证
    validator = APIValidator(args.csv, args.src, args.filter)

    validator.parse_csv()
    validator.scan_implementations()
    validator.compare()
    validator.generate_report(args.output)

    print()
    print("=" * 60)
    print("✅ 验证完成！")
    print(f"📄 报告已保存到: {args.output}")
    print("=" * 60)

    return 0


if __name__ == '__main__':
    exit(main())
