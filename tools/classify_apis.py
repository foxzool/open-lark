#!/usr/bin/env python3
"""
Open-Lark API分类工具

分析飞书开放平台API数据，按业务领域进行分类，生成模块映射关系。
"""

import csv
import json
import os
from collections import defaultdict, Counter
from typing import Dict, List, Tuple, Any
from dataclasses import dataclass, asdict
from pathlib import Path


@dataclass
class APInfo:
    """API信息数据类"""
    id: str
    name: str
    biz_tag: str
    charging_method: str
    detail: str
    full_dose: str
    full_path: str
    url: str
    order_mark: str
    support_app_types: str
    tags: str
    update_time: str
    is_charge: str
    meta_name: str
    meta_project: str
    meta_resource: str
    meta_type: str
    meta_version: str


class APIClassifier:
    """API分类器"""

    def __init__(self, csv_file: str):
        """初始化分类器

        Args:
            csv_file: API数据CSV文件路径
        """
        self.csv_file = csv_file
        self.apis = []
        self.classification = {}

    def load_apis(self) -> None:
        """加载API数据"""
        apis = []
        try:
            with open(self.csv_file, 'r', encoding='utf-8') as f:
                reader = csv.DictReader(f)
                for row in reader:
                    # 清理和验证数据
                    if not row.get('name') or not row.get('bizTag'):
                        continue

                    api = APInfo(
                        id=row.get('id', ''),
                        name=row.get('name', ''),
                        biz_tag=row.get('bizTag', ''),
                        charging_method=row.get('chargingMethod', ''),
                        detail=row.get('detail', ''),
                        full_dose=row.get('fullDose', ''),
                        full_path=row.get('fullPath', ''),
                        url=row.get('url', ''),
                        order_mark=row.get('orderMark', ''),
                        support_app_types=row.get('supportAppTypes', ''),
                        tags=row.get('tags', ''),
                        update_time=row.get('updateTime', ''),
                        is_charge=row.get('isCharge', ''),
                        meta_name=row.get('meta.Name', ''),
                        meta_project=row.get('meta.Project', ''),
                        meta_resource=row.get('meta.Resource', ''),
                        meta_type=row.get('meta.Type', ''),
                        meta_version=row.get('meta.Version', '')
                    )
                    apis.append(api)

            self.apis = apis
            print(f"✅ 成功加载 {len(apis)} 个API")

        except Exception as e:
            print(f"❌ 加载API数据失败: {e}")
            raise

    def classify_by_biztag(self) -> Dict[str, List[APInfo]]:
        """按bizTag分类"""
        by_biztag = defaultdict(list)
        for api in self.apis:
            if api.biz_tag:
                by_biztag[api.biz_tag].append(api)
        return dict(by_biztag)

    def classify_by_project(self) -> Dict[str, List[APInfo]]:
        """按meta.Project分类"""
        by_project = defaultdict(list)
        for api in self.apis:
            if api.meta_project:
                by_project[api.meta_project].append(api)
        return dict(by_project)

    def classify_by_version(self) -> Dict[str, List[APInfo]]:
        """按meta.Version分类"""
        by_version = defaultdict(list)
        for api in self.apis:
            if api.meta_version:
                by_version[api.meta_version].append(api)
        return dict(by_version)

    def classify_by_resource(self) -> Dict[str, List[APInfo]]:
        """按meta.Resource分类"""
        by_resource = defaultdict(list)
        for api in self.apis:
            if api.meta_resource:
                by_resource[api.meta_resource].append(api)
        return dict(by_resource)

    def suggest_modules(self) -> Dict[str, Dict[str, Any]]:
        """建议的模块分组"""
        # 定义业务相似性映射
        module_mapping = {
            # HR人力管理模块
            "hr": {
                "biztags": ["hire", "corehr", "feishu_people", "attendance", "payroll", "ehr"],
                "name": "HR人力管理",
                "description": "完整的人力资源管理生态",
                "priority": "P0"
            },
            # 通讯协作模块
            "communication": {
                "biztags": ["contact", "im", "moments"],
                "name": "通讯协作",
                "description": "即时通讯和联系人管理",
                "priority": "P0"
            },
            # 文档协作模块
            "docs": {
                "biztags": ["ccm", "base", "baike", "minutes"],
                "name": "文档协作",
                "description": "云文档、知识库、企业知识管理",
                "priority": "P0"
            },
            # 任务审批模块
            "workflow": {
                "biztags": ["task", "approval", "board"],
                "name": "任务审批",
                "description": "任务管理、审批流程、看板",
                "priority": "P1"
            },
            # 会议日程模块
            "meeting": {
                "biztags": ["calendar", "vc", "meeting_room"],
                "name": "会议日程",
                "description": "日历、视频会议、会议室管理",
                "priority": "P1"
            },
            # 邮件服务模块
            "mail": {
                "biztags": ["mail"],
                "name": "邮件服务",
                "description": "邮件发送和管理",
                "priority": "P2"
            },
            # 帮助台模块
            "helpdesk": {
                "biztags": ["helpdesk"],
                "name": "帮助台",
                "description": "客服和工单系统",
                "priority": "P3"
            },
            # 应用管理模块
            "platform": {
                "biztags": ["application", "app_engine", "admin"],
                "name": "应用管理",
                "description": "应用平台和管理工具",
                "priority": "P2"
            },
            # AI智能模块
            "ai": {
                "biztags": ["ai"],
                "name": "AI智能",
                "description": "AI服务和智能功能",
                "priority": "P3"
            },
            # 安全认证模块
            "security": {
                "biztags": ["auth", "passport", "security_and_compliance", "trust_party", "acs", "human_authentication"],
                "name": "安全认证",
                "description": "认证、安全、合规",
                "priority": "P0"
            },
            # 数据分析模块
            "analytics": {
                "biztags": ["report", "search", "directory"],
                "name": "数据分析",
                "description": "搜索、报表、目录服务",
                "priority": "P3"
            },
            # 个人设置模块
            "user": {
                "biztags": ["personal_settings", "workplace", "cardkit", "tenant"],
                "name": "个人设置",
                "description": "个人化设置和用户体验",
                "priority": "P3"
            }
        }

        # 统计每个模块的API数量
        by_biztag = self.classify_by_biztag()
        module_stats = {}

        for module_id, module_config in module_mapping.items():
            api_count = 0
            biztags_with_count = []

            for biztag in module_config["biztags"]:
                count = len(by_biztag.get(biztag, []))
                if count > 0:
                    api_count += count
                    biztags_with_count.append(f"{biztag}({count})")

            module_stats[module_id] = {
                **module_config,
                "api_count": api_count,
                "biztags_with_count": biztags_with_count,
                "crate_name": f"openlark-{module_id}"
            }

        return module_stats

    def generate_rust_code(self, modules: Dict[str, Dict[str, Any]]) -> str:
        """生成Rust代码"""
        lines = []
        lines.append("// 自动生成的模块映射代码")
        lines.append("// 请勿手动编辑，由 tools/classify_apis.py 生成")
        lines.append("")
        lines.append("use std::collections::HashMap;")
        lines.append("")
        lines.append("/// 模块映射信息")
        lines.append("pub fn get_module_mapping() -> HashMap<String, ModuleInfo> {")
        lines.append("    let mut modules = HashMap::new();")
        lines.append("")

        for module_id, module_info in modules.items():
            lines.append(f"    // {module_info['name']}")
            lines.append(f"    modules.insert(")
            lines.append(f"        \"{module_id}\".to_string(),")
            lines.append(f"        ModuleInfo {{")
            lines.append(f"            name: \"{module_info['name']}\".to_string(),")
            lines.append(f"            description: \"{module_info['description']}\".to_string(),")
            lines.append(f"            api_count: {module_info['api_count']},")
            lines.append(f"            priority: \"{module_info['priority']}\".to_string(),")
            lines.append(f"            crate_name: \"{module_info['crate_name']}\".to_string(),")
            lines.append(f"            biztags: vec![{', '.join([f'\"{bt}\"' for bt in module_info['biztags']])}],")
            lines.append(f"        }},")
            lines.append(f"    );")
            lines.append("")

        lines.append("    modules")
        lines.append("}")
        lines.append("")
        lines.append("/// 模块信息结构")
        lines.append("#[derive(Debug, Clone)]")
        lines.append("pub struct ModuleInfo {")
        lines.append("    /// 模块名称")
        lines.append("    pub name: String,")
        lines.append("    /// 模块描述")
        lines.append("    pub description: String,")
        lines.append("    /// API数量")
        lines.append("    pub api_count: usize,")
        lines.append("    /// 开发优先级")
        lines.append("    pub priority: String,")
        lines.append("    /// Crate名称")
        lines.append("    pub crate_name: String,")
        lines.append("    /// 包含的bizTags")
        lines.append("    pub biztags: Vec<String>,")
        lines.append("}")

        return "\n".join(lines)

    def generate_report(self) -> str:
        """生成分类报告"""
        lines = []
        lines.append("# Open-Lark API分类报告")
        lines.append("")
        lines.append(f"**生成时间**: {self._get_current_time()}")
        lines.append(f"**总API数量**: {len(self.apis)}")
        lines.append("")

        # 按bizTag统计
        by_biztag = self.classify_by_biztag()
        lines.append("## 按bizTag分类统计")
        lines.append("")
        lines.append("| bizTag | API数量 | 描述 |")
        lines.append("|--------|---------|------|")

        for biztag, apis in sorted(by_biztag.items(), key=lambda x: len(x[1]), reverse=True):
            sample_api = apis[0] if apis else None
            description = sample_api.name[:50] + "..." if sample_api and len(sample_api.name) > 50 else (sample_api.name if sample_api else "")
            lines.append(f"| {biztag} | {len(apis)} | {description} |")

        lines.append("")

        # 按项目统计
        by_project = self.classify_by_project()
        lines.append("## 按meta.Project分类统计")
        lines.append("")
        lines.append("| Project | API数量 |")
        lines.append("|---------|---------|")

        for project, apis in sorted(by_project.items(), key=lambda x: len(x[1]), reverse=True):
            lines.append(f"| {project} | {len(apis)} |")

        lines.append("")

        # 按版本统计
        by_version = self.classify_by_version()
        lines.append("## 按meta.Version分类统计")
        lines.append("")
        lines.append("| Version | API数量 |")
        lines.append("|---------|---------|")

        for version, apis in sorted(by_version.items(), key=lambda x: len(x[1]), reverse=True):
            lines.append(f"| {version} | {len(apis)} |")

        lines.append("")

        # 建议的模块分组
        modules = self.suggest_modules()
        lines.append("## 建议的模块分组")
        lines.append("")

        total_api_count = sum(m['api_count'] for m in modules.values())
        lines.append(f"**模块总数**: {len(modules)}")
        lines.append(f"**覆盖API数**: {total_api_count}")
        lines.append(f"**覆盖率**: {total_api_count/len(self.apis)*100:.1f}%")
        lines.append("")

        lines.append("| 模块标识 | 模块名称 | API数量 | 优先级 | Crate名称 | 包含的bizTag |")
        lines.append("|---------|---------|---------|--------|-----------|-------------|")

        for module_id, module_info in sorted(modules.items(), key=lambda x: x[1]['api_count'], reverse=True):
            biztags_str = ", ".join(module_info['biztags_with_count'])
            lines.append(f"| {module_id} | {module_info['name']} | {module_info['api_count']} | {module_info['priority']} | {module_info['crate_name']} | {biztags_str} |")

        return "\n".join(lines)

    def _get_current_time(self) -> str:
        """获取当前时间"""
        from datetime import datetime
        return datetime.now().strftime("%Y-%m-%d %H:%M:%S")

    def save_outputs(self, output_dir: str = "tools/output") -> None:
        """保存输出文件"""
        output_path = Path(output_dir)
        output_path.mkdir(exist_ok=True)

        # 生成分类
        by_biztag = self.classify_by_biztag()
        by_project = self.classify_by_project()
        by_version = self.classify_by_version()
        by_resource = self.classify_by_resource()
        modules = self.suggest_modules()

        classification = {
            "total_apis": len(self.apis),
            "by_biztag": {k: len(v) for k, v in by_biztag.items()},
            "by_project": {k: len(v) for k, v in by_project.items()},
            "by_version": {k: len(v) for k, v in by_version.items()},
            "by_resource": {k: len(v) for k, v in by_resource.items()},
            "suggested_modules": modules
        }

        # 保存JSON数据
        with open(output_path / "classification.json", "w", encoding="utf-8") as f:
            json.dump(classification, f, ensure_ascii=False, indent=2)

        # 保存详细API数据
        api_data = [asdict(api) for api in self.apis]
        with open(output_path / "apis.json", "w", encoding="utf-8") as f:
            json.dump(api_data, f, ensure_ascii=False, indent=2)

        # 保存模块映射代码
        rust_code = self.generate_rust_code(modules)
        with open(output_path / "module_mapping.rs", "w", encoding="utf-8") as f:
            f.write(rust_code)

        # 保存分类报告
        report = self.generate_report()
        with open(output_path / "classification_report.md", "w", encoding="utf-8") as f:
            f.write(report)

        print(f"✅ 输出文件已保存到: {output_path.absolute()}")
        print(f"   📊 分类数据: classification.json")
        print(f"   📋 API数据: apis.json")
        print(f"   🦀 Rust代码: module_mapping.rs")
        print(f"   📄 分类报告: classification_report.md")


def main():
    """主函数"""
    import argparse

    parser = argparse.ArgumentParser(description="Open-Lark API分类工具")
    parser.add_argument(
        "csv_file",
        nargs="?",
        default="analysis/data/api_list_export.csv",
        help="API数据CSV文件路径"
    )
    parser.add_argument(
        "--output",
        "-o",
        default="tools/output",
        help="输出目录路径"
    )

    args = parser.parse_args()

    # 检查输入文件
    if not os.path.exists(args.csv_file):
        print(f"❌ 找不到文件: {args.csv_file}")
        return 1

    # 创建分类器
    classifier = APIClassifier(args.csv_file)

    try:
        # 加载数据
        classifier.load_apis()

        # 保存输出
        classifier.save_outputs(args.output)

        # 打印统计信息
        modules = classifier.suggest_modules()
        total_apis = len(classifier.apis)
        covered_apis = sum(m['api_count'] for m in modules.values())

        print(f"\n📈 分类统计:")
        print(f"   总API数: {total_apis}")
        print(f"   建议模块数: {len(modules)}")
        print(f"   覆盖API数: {covered_apis}")
        print(f"   覆盖率: {covered_apis/total_apis*100:.1f}%")

        return 0

    except Exception as e:
        print(f"❌ 处理失败: {e}")
        return 1


if __name__ == "__main__":
    exit(main())