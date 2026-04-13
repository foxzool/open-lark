#!/usr/bin/env python3
"""
API 验证脚本

对比 CSV 文件中的 API 列表与实际代码实现，生成完成度报告。

命名规范：src/bizTag/meta.project/meta.version/meta.resource/meta.name.rs
- meta.resource 中的 '.' 转换为 '/' 作为子目录
- meta.name 中的 '/' 转换为 '/' 作为子目录
- meta.name 中的 ':' 替换为 '_'
"""

import csv
import os
import re
from pathlib import Path
from typing import Any, Dict, List, Optional, Set, Tuple
from dataclasses import dataclass
from collections import defaultdict

try:
    import tomllib  # py>=3.11
except ModuleNotFoundError:  # pragma: no cover
    tomllib = None


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

    def __init__(
        self,
        csv_path: str,
        src_path: str,
        filter_tags: List[str] = None,
        skip_old_versions: bool = True,
        with_timestamp: bool = False,
    ):
        self.csv_path = csv_path
        self.src_path = Path(src_path)
        self.filter_tags = filter_tags
        self.skip_old_versions = skip_old_versions
        self.with_timestamp = with_timestamp
        self.apis: List[APIInfo] = []
        self.implemented_files: Set[str] = set()
        self.missing_apis: List[APIInfo] = []
        self.extra_files: Set[str] = set()
        self.skipped_old_count: int = 0

    @staticmethod
    def _camel_to_snake(name: str) -> str:
        """将 camelCase/PascalCase 转为 snake_case（尽量兼容缩写）。"""
        if not name:
            return name
        # 先处理缩写边界："HTTPServer" -> "HTTP_Server"
        s1 = re.sub(r"([A-Z]+)([A-Z][a-z])", r"\1_\2", name)
        # 再处理常规边界："dataValidation" -> "data_Validation"
        s2 = re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", s1)
        return s2.lower()

    def _normalize_name_path(self, name_path: str) -> str:
        """
        将 meta.name 转为仓库当前实际使用的文件/目录命名风格：
        - 将 '#': `post#spreadsheets` -> `post_spreadsheets`
        - 将每个 path segment 做 snake_case：`dataValidation` -> `data_validation`
        - 对路径参数（已被 ':' 替换为 '_'）也做 snake_case：`_docToken` -> `_doc_token`
        """
        name_path = name_path.replace("#", "_")
        segments = [s for s in name_path.split("/") if s]
        normalized: List[str] = []
        for seg in segments:
            if seg.startswith("_") and len(seg) > 1:
                normalized.append("_" + self._camel_to_snake(seg[1:]))
            else:
                normalized.append(self._camel_to_snake(seg))
        return "/".join(normalized)

    def parse_csv(self):
        """解析 CSV 文件"""
        print(f"📄 读取 CSV 文件: {self.csv_path}")

        if self.filter_tags:
            print(f"🏷️  过滤业务标签: {', '.join(self.filter_tags)}")

        if self.skip_old_versions:
            print(f"🔧 跳过旧版本 API: version='old'")

        with open(self.csv_path, 'r', encoding='utf-8') as f:
            reader = csv.DictReader(f)

            for row in reader:
                # 如果设置了过滤器，只处理匹配的业务标签
                if self.filter_tags and row['bizTag'] not in self.filter_tags:
                    continue

                # 跳过旧版本 API
                if self.skip_old_versions and row['meta.Version'] == 'old':
                    self.skipped_old_count += 1
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
        if self.skip_old_versions and self.skipped_old_count > 0:
            print(f"   📋 已跳过 {self.skipped_old_count} 个旧版本 API")

    def _generate_expected_file_path(self, api: APIInfo) -> str:
        """
        根据 API 信息生成预期的文件路径

        项目命名规范（见 AGENTS.md）：
        - API Path: src/{bizTag}/{project}/{version}/{resource}/{name}.rs
        - Example: src/im/im/v1/message/create.rs
        - 当 meta.project == bizTag 时，仍然保留 project 层级（如 im/im）
        - meta.resource 中的 '.' 转换为 '/' 作为子目录
        - meta.name 中的 '/' 转换为 '/' 作为子目录
        - meta.name 中的 ':' 替换为 '_'（路径参数）
        """

        # 特殊规则: meeting_room 的 old/default 组合完全省略
        if api.biz_tag == 'meeting_room' and api.meta_version == 'old' and api.meta_resource == 'default':
            # 直接使用 meta.name 作为路径
            name_path = api.meta_name.replace(':', '_')
            name_path = self._normalize_name_path(name_path)
            if '/' in name_path:
                name_with_path = name_path.replace('/', '/')
            else:
                name_with_path = name_path
            return f"meeting_room/{name_with_path}.rs"

        # 基础路径：{bizTag}/{project}
        # 始终保留 project 层级，即使 project == bizTag
        base = f"{api.biz_tag}/{api.meta_project}"

        # 处理 meta.version
        version = api.meta_version

        # 处理 meta.resource：将 '.' 替换为 '/'
        resource_path = api.meta_resource.replace('.', '/')

        # 处理 meta.name：
        # 1. 去除末尾的斜杠（处理 meta.name 以 '/' 结尾的情况）
        # 2. 将 '/' 转换为 '/'（保持为子目录分隔符）
        # 3. 将 ':' 替换为 '_'（处理路径参数）
        name_path = api.meta_name.replace(':', '_').rstrip('/')
        name_path = self._normalize_name_path(name_path)

        # 如果 meta.name 包含 '/'，则创建子目录
        if '/' in name_path:
            # 例如: "building/list" -> "building/list.rs"
            # 例如: "summary/batch_get" -> "summary/batch_get.rs"
            name_with_path = name_path.replace('/', '/')
        else:
            # 简单名称，直接使用
            name_with_path = name_path

        # 构建完整路径
        # {bizTag}/{project}/{version}/{resource}/{name}.rs
        full_path = f"{base}/{version}/{resource_path}/{name_with_path}.rs"

        return full_path

    def scan_implementations(self):
        """扫描实际实现的文件"""
        print(f"🔍 扫描代码实现目录: {self.src_path}")

        for root, dirs, files in os.walk(self.src_path):
            # 跳过 __pycache__ 等目录
            dirs[:] = [d for d in dirs if not d.startswith('.') and d != '__pycache__']

            for file in files:
                # 排除非 API 文件：
                # - mod.rs（模块声明）
                # - models.rs（数据模型）
                # - macros.rs（宏定义）
                # - service.rs（服务入口/Fluent Interface）
                # - responses.rs（响应类型定义）
                exclude_files = ('mod.rs', 'models.rs', 'macros.rs', 'service.rs', 'responses.rs')
                if file.endswith('.rs') and file not in exclude_files:
                    # 获取相对路径
                    full_path = os.path.join(root, file)
                    rel_path = os.path.relpath(full_path, self.src_path)

                    # 将路径分隔符转换为 /
                    rel_path = rel_path.replace('\\', '/')

                    # 排除 lib.rs 和 common 目录下的文件
                    if not rel_path.startswith('lib.rs') and not rel_path.startswith('common/'):
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
            if self.with_timestamp:
                f.write(f"**生成时间**: {self._get_timestamp()}\n")
            f.write(f"**CSV 文件**: {self.csv_path}\n")
            f.write(f"**源码目录**: {self.src_path}\n")
            f.write(f"**命名规范**: `src/bizTag/meta.project/meta.version/meta.resource/meta.name.rs`\n\n")

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

            # 已实现的 API 列表
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

    def calculate_summary(self) -> Dict[str, Any]:
        """生成可序列化的统计摘要。"""
        total_apis = len(self.apis)
        implemented = len([a for a in self.apis if a.is_implemented])
        missing = len(self.missing_apis)
        completion_rate = (implemented / total_apis * 100) if total_apis > 0 else 0.0

        return {
            "total_apis": total_apis,
            "implemented": implemented,
            "missing": missing,
            "completion_rate": round(completion_rate, 1),
            "extra_files": len(self.extra_files),
            "skipped_old_versions": self.skipped_old_count if self.skip_old_versions else 0,
            "module_stats": self._calculate_module_stats(),
        }

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

    parser = argparse.ArgumentParser(description='API 验证脚本（基于 strict 命名规范）')
    parser.add_argument('--csv', default='api_list_export.csv',
                       help='CSV 文件路径 (默认: api_list_export.csv)')
    parser.add_argument('--src', default=None,
                       help='源码目录路径（默认: crates/openlark-meeting/src；也可用 --crate 自动设置）')
    parser.add_argument('--output', default=None,
                       help='报告输出路径（默认: API_VALIDATION_REPORT.md；--crate 时默认: reports/api_validation/<crate>.md）')
    parser.add_argument('--filter', nargs='+',
                       help='过滤业务标签 (例如: --filter calendar vc meeting_room)')
    parser.add_argument('--crate',
                        help='按 crate 自动设置 --src/--filter（来源: tools/api_coverage.toml）')
    parser.add_argument('--mapping', default='tools/api_coverage.toml',
                        help='crate→bizTag 映射文件路径 (默认: tools/api_coverage.toml)')
    parser.add_argument('--list-crates', action='store_true',
                        help='列出映射文件中的 crate 与 bizTag，然后退出')
    parser.add_argument('--all-crates', action='store_true',
                        help='按映射文件批量验证所有 crate，并生成汇总报告')
    parser.add_argument('--report-dir', default='reports/api_validation',
                        help='批量模式报告目录 (默认: reports/api_validation)')
    parser.add_argument('--skip-old', dest='skip_old', action='store_true', default=True,
                        help='跳过旧版本 API (version=old，默认启用)')
    parser.add_argument('--include-old', dest='skip_old', action='store_false',
                        help='包含旧版本 API (version=old)')
    parser.add_argument('--with-timestamp', action='store_true',
                        help='在报告中写入生成时间（默认关闭，以支持稳定复现）')

    args = parser.parse_args()

    print("=" * 60)
    print("🚀 API 验证工具（Strict 命名规范）")
    print("=" * 60)
    print()

    def _load_mapping(path: str) -> dict:
        if tomllib is None:
            print("❌ 错误: 当前 Python 不支持 tomllib，请使用 Python 3.11+")
            raise SystemExit(1)
        mapping_path = Path(path)
        if not mapping_path.exists():
            print(f"❌ 错误: 映射文件不存在: {mapping_path}")
            raise SystemExit(1)
        data = tomllib.loads(mapping_path.read_text(encoding="utf-8"))
        crates = data.get("crates", {})
        if not isinstance(crates, dict) or not crates:
            print(f"❌ 错误: 映射文件缺少 [crates.*] 配置: {mapping_path}")
            raise SystemExit(1)
        return crates

    if args.list_crates:
        crates = _load_mapping(args.mapping)
        print(f"📄 映射文件: {args.mapping}\n")
        for crate_name in sorted(crates.keys()):
            cfg = crates.get(crate_name, {})
            src = cfg.get("src", "")
            tags = cfg.get("biz_tags", [])
            tags_text = ", ".join(tags) if isinstance(tags, list) else str(tags)
            print(f"- {crate_name}: src={src} biz_tags=[{tags_text}]")
        return 0

    def _run_validator(
        csv_path: str,
        src_path: str,
        filter_tags: Optional[List[str]],
        skip_old: bool,
        with_timestamp: bool,
    ) -> APIValidator:
        validator = APIValidator(csv_path, src_path, filter_tags, skip_old, with_timestamp)
        validator.parse_csv()
        validator.scan_implementations()
        validator.compare()
        return validator

    def _write_summary_markdown(
        output_path: Path,
        crate_rows: List[Tuple[str, Dict[str, Any], str, List[str]]],
        skip_old: bool,
    ) -> None:
        output_path.parent.mkdir(parents=True, exist_ok=True)
        with output_path.open("w", encoding="utf-8") as f:
            f.write("# Typed API 覆盖率汇总报告（按 crate）\n\n")
            f.write("## 统计口径\n\n")
            if skip_old:
                f.write("- 默认排除 `meta.Version=old`。\n")
            else:
                f.write("- 包含 `meta.Version=old`。\n")
            f.write("- 数据来源：`api_list_export.csv` 对比 crate 源码目录。\n\n")

            total_apis = sum(row[1]["total_apis"] for row in crate_rows)
            total_impl = sum(row[1]["implemented"] for row in crate_rows)
            total_missing = sum(row[1]["missing"] for row in crate_rows)
            total_extra = sum(row[1]["extra_files"] for row in crate_rows)
            total_rate = (total_impl / total_apis * 100) if total_apis > 0 else 0.0

            f.write("## 总览\n\n")
            f.write("| 指标 | 数量 |\n")
            f.write("|------|------|\n")
            f.write(f"| crate 数量 | {len(crate_rows)} |\n")
            f.write(f"| API 总数 | {total_apis} |\n")
            f.write(f"| 已实现 | {total_impl} |\n")
            f.write(f"| 未实现 | {total_missing} |\n")
            f.write(f"| 完成率 | {total_rate:.1f}% |\n")
            f.write(f"| 额外文件 | {total_extra} |\n\n")

            f.write("## 各 crate 覆盖率\n\n")
            f.write("| crate | bizTag | 总数 | 已实现 | 未实现 | 完成率 | 额外文件 | 报告 |\n")
            f.write("|-------|--------|------|--------|--------|--------|----------|------|\n")
            for crate_name, stats, report_rel, tags in sorted(crate_rows, key=lambda x: x[0]):
                tags_text = ", ".join(tags)
                f.write(
                    f"| {crate_name} | `{tags_text}` | {stats['total_apis']} | "
                    f"{stats['implemented']} | {stats['missing']} | {stats['completion_rate']:.1f}% | "
                    f"{stats['extra_files']} | [{crate_name}]({report_rel}) |\n"
                )

            f.write("\n")

    def _write_summary_json(output_path: Path, payload: Dict[str, Any]) -> None:
        import json

        output_path.parent.mkdir(parents=True, exist_ok=True)
        output_path.write_text(
            json.dumps(payload, ensure_ascii=False, indent=2, sort_keys=True) + "\n",
            encoding="utf-8",
        )

    if args.all_crates:
        if not os.path.exists(args.csv):
            print(f"❌ 错误: CSV 文件不存在: {args.csv}")
            return 1
        crates = _load_mapping(args.mapping)
        report_dir = Path(args.report_dir)
        crate_dir = report_dir / "crates"
        crate_rows: List[Tuple[str, Dict[str, Any], str, List[str]]] = []
        crate_summaries: Dict[str, Any] = {}

        for crate_name in sorted(crates.keys()):
            cfg = crates[crate_name]
            src = cfg.get("src")
            tags = cfg.get("biz_tags", [])
            if not src or not os.path.exists(src):
                print(f"⚠️ 跳过 {crate_name}: 源码目录不存在 ({src})")
                continue
            report_path = crate_dir / f"{crate_name}.md"
            print()
            print(f"📦 处理 {crate_name}")
            validator = _run_validator(args.csv, src, tags, args.skip_old, args.with_timestamp)
            report_path.parent.mkdir(parents=True, exist_ok=True)
            validator.generate_report(str(report_path))
            stats = validator.calculate_summary()
            report_rel = report_path.relative_to(report_dir).as_posix()
            crate_rows.append((crate_name, stats, report_rel, tags))
            crate_summaries[crate_name] = {
                "source_dir": src,
                "biz_tags": tags,
                "report": report_rel,
                **stats,
            }

        summary_md = report_dir / "summary.md"
        summary_json = report_dir / "summary.json"
        _write_summary_markdown(summary_md, crate_rows, args.skip_old)

        total_apis = sum(item["total_apis"] for item in crate_summaries.values())
        total_impl = sum(item["implemented"] for item in crate_summaries.values())
        total_missing = sum(item["missing"] for item in crate_summaries.values())
        total_extra = sum(item["extra_files"] for item in crate_summaries.values())
        total_rate = (total_impl / total_apis * 100) if total_apis > 0 else 0.0

        summary_payload = {
            "csv_path": args.csv,
            "mapping_path": args.mapping,
            "skip_old_versions": args.skip_old,
            "crates_total": len(crate_summaries),
            "total_apis": total_apis,
            "implemented": total_impl,
            "missing": total_missing,
            "completion_rate": round(total_rate, 1),
            "extra_files": total_extra,
            "crates": crate_summaries,
        }
        _write_summary_json(summary_json, summary_payload)

        print()
        print("=" * 60)
        print("✅ 批量验证完成！")
        print(f"📄 汇总报告: {summary_md}")
        print(f"📄 机器可读: {summary_json}")
        print(f"📁 各 crate 报告目录: {crate_dir}")
        print("=" * 60)
        return 0

    # 当指定 --crate 时，自动补齐 src/filter（显式参数优先）
    if args.crate:
        crates = _load_mapping(args.mapping)
        if args.crate not in crates:
            print(f"❌ 错误: 映射文件中不存在 crate: {args.crate}")
            print(f"   提示：运行 `python3 tools/validate_apis.py --list-crates` 查看可用项")
            return 1
        cfg = crates[args.crate]
        if args.src is None:
            args.src = cfg.get("src")
        if args.filter is None:
            args.filter = cfg.get("biz_tags")

    # 输出路径默认值：
    # - --crate：默认写入 reports/api_validation/<crate>.md（集中存放，且根 .gitignore 已忽略 /reports/）
    # - 否则：保持旧默认 API_VALIDATION_REPORT.md
    if args.output is None:
        if args.crate:
            args.output = f"reports/api_validation/{args.crate}.md"
        else:
            args.output = "API_VALIDATION_REPORT.md"

    # 兼容旧默认值
    if args.src is None:
        args.src = 'crates/openlark-meeting/src'

    # 验证输入
    if not os.path.exists(args.csv):
        print(f"❌ 错误: CSV 文件不存在: {args.csv}")
        return 1

    if not os.path.exists(args.src):
        print(f"❌ 错误: 源码目录不存在: {args.src}")
        return 1

    # 执行验证
    validator = _run_validator(args.csv, args.src, args.filter, args.skip_old, args.with_timestamp)

    Path(args.output).parent.mkdir(parents=True, exist_ok=True)
    validator.generate_report(args.output)

    print()
    print("=" * 60)
    print("✅ 验证完成！")
    print(f"📄 报告已保存到: {args.output}")
    print("=" * 60)

    return 0


if __name__ == '__main__':
    exit(main())
