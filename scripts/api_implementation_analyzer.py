#!/usr/bin/env python3
"""
完整的API实现覆盖分析工具
分析feishu_server_apis.csv中的所有API在open-lark中的实现情况
"""

import csv
import os
from pathlib import Path
from typing import Dict, List, Tuple, Set, Optional
from dataclasses import dataclass
from collections import defaultdict

@dataclass
class APIInfo:
    """API信息结构"""
    module: str
    method: str
    endpoint: str
    name: str
    description: str
    doc_url: str
    project: str
    resource: str
    version: str
    api_type: str

@dataclass
class ImplementationResult:
    """实现结果"""
    api_info: APIInfo
    implemented: bool
    file_path: Optional[str] = None
    function_name: Optional[str] = None
    coverage_type: str = "missing"  # missing, partial, full

class APIImplementationAnalyzer:
    """API实现分析器"""

    def __init__(self, project_root: Path):
        self.project_root = project_root
        self.service_dir = project_root / "src" / "service"

        # 模块名称映射（CSV -> open-lark）
        self.module_mapping = {
            "im": "im",
            "contact": "contact",
            "drive": "cloud_docs",
            "calendar": "calendar",
            "approval": "approval",
            "hr": "hire",
            "attendance": "attendance",
            "payroll": "payroll",
            "meeting": "meeting",
            "wiki": "wiki",
            "email": "email",
            "application": "application",
            "department": "contact",  # department在contact中
            "group": "group",
            "message": "im",  # message在im中
            "chat": "im",  # chat在im中
            "user": "contact",  # user在contact中
            "sheet": "cloud_docs",  # sheet在cloud_docs中
            "doc": "cloud_docs",  # doc在cloud_docs中
            "bitable": "cloud_docs",  # bitable在cloud_docs中
            "mindnote": "cloud_docs",  # mindnote在cloud_docs中
            "file": "cloud_docs",  # file在cloud_docs中
            "wiki_space": "wiki",
            "tasklist": "tasklist",
            "ai": "ai",
            "search": "search",
            "auth": "authentication",
            "authentication": "authentication",
            "tenant": "tenant",
            "mdm": "mdm",
            "helpdesk": "helpdesk",
            "performance": "performance",
            "openid": "authentication",
            "captcha": "authentication",
            "ticket": "helpdesk",
            "survey": "survey",
            "quality": "quality",
            "safety": "safety",
            "finance": "finance",
            "ecnomics": "finance",  # 拼写错误修正
            "shop": "shop",
            "live": "live",
            "fintech": "fintech",
            "industry": "industry",
            "education": "education",
            "healthcare": "healthcare",
            "government": "government",
            "community": "community",
            "game": "game",
            "blockchain": "blockchain",
            "iot": "iot",
            "ark": "ark",
            "gke": "gke",
            "anti_fraud": "anti_fraud",
            "data_market": "data_market",
            "activity": "activity",
            "badge": "badge",
            "coordinate": "coordinate",
            "calendar_v2": "calendar",
            "drive_v3": "cloud_docs",
            "im_v1": "im",
            "contact_v3": "contact",
            "approval_v4": "approval",
            "hr_v4": "hire",
            "attendance_v1": "attendance",
            "meeting_v1": "meeting",
            "wiki_v2": "wiki",
            "email_v1": "email",
            "application_v6": "application",
            "group_v1": "group",
            "message_v1": "im",
            "chat_v1": "im",
            "user_v1": "contact",
            "sheet_v1": "cloud_docs",
            "doc_v1": "cloud_docs",
            "bitable_v1": "cloud_docs",
            "mindnote_v1": "cloud_docs",
            "file_v1": "cloud_docs",
            "wiki_space_v1": "wiki",
            "tasklist_v1": "tasklist",
            "ai_v1": "ai",
            "search_v1": "search",
            "auth_v1": "authentication",
            "authentication_v1": "authentication",
            "tenant_v2": "tenant",
            "mdm_v3": "mdm",
            "helpdesk_v1": "helpdesk",
            "performance_v1": "performance",
            "openid_v1": "authentication",
            "captcha_v1": "authentication",
            "ticket_v1": "helpdesk",
            "survey_v1": "survey",
            "quality_v1": "quality",
            "safety_v1": "safety",
            "finance_v1": "finance",
            "ecnomics_v1": "finance",
            "shop_v1": "shop",
            "live_v1": "live",
            "fintech_v1": "fintech",
            "industry_v1": "industry",
            "education_v1": "education",
            "healthcare_v1": "healthcare",
            "government_v1": "government",
            "community_v1": "community",
            "game_v1": "game",
            "blockchain_v1": "blockchain",
            "iot_v1": "iot",
            "ark_v1": "ark",
            "gke_v1": "gke",
            "anti_fraud_v1": "anti_fraud",
            "data_market_v1": "data_market",
            "activity_v1": "activity",
            "badge_v1": "badge",
            "coordinate_v1": "coordinate"
        }

        # 版本映射
        self.version_mapping = {
            "v1": "v1",
            "v2": "v2",
            "v3": "v3",
            "v4": "v4",
            "v5": "v5",
            "v6": "v6",
            "latest": "v1"
        }

    def load_apis_from_csv(self) -> List[APIInfo]:
        """从CSV加载所有API信息"""
        apis = []
        csv_path = self.project_root / "reports" / "feishu_server_apis.csv"

        if not csv_path.exists():
            print(f"❌ CSV文件不存在: {csv_path}")
            return apis

        with open(csv_path, 'r', encoding='utf-8') as f:
            reader = csv.DictReader(f)
            for row in reader:
                api_info = APIInfo(
                    module=row.get('module', '').strip(),
                    method=row.get('method', '').strip(),
                    endpoint=row.get('endpoint', '').strip(),
                    name=row.get('name', '').strip(),
                    description=row.get('description', '').strip(),
                    doc_url=row.get('doc_url', '').strip(),
                    project=row.get('project', '').strip(),
                    resource=row.get('resource', '').strip(),
                    version=row.get('version', '').strip(),
                    api_type=row.get('type', '').strip()
                )
                apis.append(api_info)

        return apis

    def find_service_files(self, service_name: str) -> List[Path]:
        """查找指定服务的所有Rust文件"""
        if not self.service_dir.exists():
            return []

        service_path = self.service_dir / service_name
        if not service_path.exists():
            return []

        # 递归查找所有.rs文件
        rust_files = list(service_path.rglob("*.rs"))
        return rust_files

    def find_api_implementation(self, api_info: APIInfo) -> ImplementationResult:
        """查找API实现"""
        # 映射模块名称
        service_name = self.module_mapping.get(api_info.module)
        if not service_name:
            return ImplementationResult(api_info, False, None, None, "module_not_found")

        # 查找服务文件
        service_files = self.find_service_files(service_name)
        if not service_files:
            return ImplementationResult(api_info, False, None, None, "service_not_found")

        # 确定版本目录
        version = self.version_mapping.get(api_info.version, "v1")
        version_dir = self.service_dir / service_name / version

        # 提取方法特征
        method_name = self.extract_method_name(api_info)
        if not method_name:
            return ImplementationResult(api_info, False, None, None, "method_extraction_failed")

        # 搜索实现
        for file_path in service_files:
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()

                # 查找方法定义
                if self.is_method_implemented(content, method_name, api_info):
                    return ImplementationResult(
                        api_info,
                        True,
                        str(file_path.relative_to(self.project_root)),
                        method_name,
                        "full"
                    )

                # 查找部分实现
                if self.is_partially_implemented(content, method_name, api_info):
                    return ImplementationResult(
                        api_info,
                        True,
                        str(file_path.relative_to(self.project_root)),
                        method_name,
                        "partial"
                    )

        return ImplementationResult(api_info, False, None, None, "missing")

    def extract_method_name(self, api_info: APIInfo) -> Optional[str]:
        """提取方法名称"""
        # 从endpoint提取方法名称
        endpoint = api_info.endpoint

        # 移除参数
        if '?' in endpoint:
            endpoint = endpoint.split('?')[0]

        # 移除版本前缀
        if endpoint.startswith('/'):
            parts = endpoint.strip('/').split('/')
            if len(parts) >= 2 and parts[0].startswith('v'):
                parts = parts[1:]
            endpoint = '/'.join(parts)

        # 转换为snake_case方法名
        method_name = self.endpoint_to_method_name(endpoint, api_info.method)

        return method_name

    def endpoint_to_method_name(self, endpoint: str, http_method: str) -> str:
        """将endpoint转换为方法名"""
        # 移除开头的斜杠
        endpoint = endpoint.lstrip('/')

        # 按斜杠分割
        parts = endpoint.split('/')

        # 过滤掉常见的路径参数
        filtered_parts = []
        for part in parts:
            # 跳过参数占位符
            if part.startswith('{') and part.endswith('}'):
                continue
            # 跳过常见的ID字段
            if part in ['user_id', 'chat_id', 'message_id', 'file_id', 'document_id']:
                continue
            filtered_parts.append(part)

        if not filtered_parts:
            # 如果没有有意义的部分，使用HTTP方法
            return http_method.lower()

        # 组合方法名
        method_name = '_'.join(filtered_parts)

        # 添加HTTP方法前缀（如果有冲突）
        if http_method.upper() != 'GET':
            method_name = f"{http_method.lower()}_{method_name}"

        return method_name

    def is_method_implemented(self, content: str, method_name: str, api_info: APIInfo) -> bool:
        """检查方法是否完整实现"""
        # 查找函数定义
        patterns = [
            f"pub async fn {method_name}",
            f"fn {method_name}",
            f"pub fn {method_name}",
        ]

        found_definition = False
        for pattern in patterns:
            if pattern in content:
                found_definition = True
                break

        if not found_definition:
            return False

        # 检查是否包含Transport::request调用
        if "Transport::request" not in content:
            return False

        # 检查HTTP方法匹配
        http_method_upper = api_info.method.upper()
        if f"Method::{http_method_upper}" not in content:
            return False

        return True

    def is_partially_implemented(self, content: str, method_name: str, api_info: APIInfo) -> bool:
        """检查方法是否部分实现"""
        # 查找函数定义
        if f"pub async fn {method_name}" not in content:
            return False

        # 检查是否包含TODO或注释
        if "TODO" in content or "FIXME" in content:
            return True

        # 检查是否有占位符实现
        if "todo!(" in content or "unimplemented!" in content:
            return True

        return False

    def analyze_all_apis(self) -> Dict:
        """分析所有API的实现情况"""
        apis = self.load_apis_from_csv()
        print(f"📊 从CSV加载了 {len(apis)} 个API")

        if not apis:
            return {"error": "No APIs loaded from CSV"}

        results = []
        implemented_count = 0
        missing_count = 0
        partial_count = 0

        for i, api_info in enumerate(apis):
            if i % 100 == 0:
                print(f"  进度: {i}/{len(apis)} ({i/len(apis)*100:.1f}%)")

            result = self.find_api_implementation(api_info)
            results.append(result)

            if result.coverage_type == "full":
                implemented_count += 1
            elif result.coverage_type == "partial":
                partial_count += 1
            else:
                missing_count += 1

        # 按模块统计
        module_stats = defaultdict(lambda: {"total": 0, "implemented": 0, "missing": 0, "partial": 0})

        for result in results:
            module = result.api_info.module
            module_stats[module]["total"] += 1

            if result.coverage_type == "full":
                module_stats[module]["implemented"] += 1
            elif result.coverage_type == "partial":
                module_stats[module]["partial"] += 1
            else:
                module_stats[module]["missing"] += 1

        return {
            "total_apis": len(apis),
            "implemented_count": implemented_count,
            "missing_count": missing_count,
            "partial_count": partial_count,
            "implementation_rate": implemented_count / len(apis) * 100 if apis else 0,
            "module_stats": dict(module_stats),
            "results": results
        }

    def generate_report(self, analysis: Dict) -> str:
        """生成分析报告"""
        report = []
        report.append("# API实现覆盖分析报告")
        report.append("=" * 80)
        report.append("")

        # 总体统计
        report.append("## 📊 总体统计")
        report.append(f"- 总API数量: {analysis['total_apis']}")
        report.append(f"- ✅ 已实现: {analysis['implemented_count']}")
        report.append(f"- ⚠️  部分实现: {analysis['partial_count']}")
        report.append(f"- ❌ 未实现: {analysis['missing_count']}")
        report.append(f"- 📈 实现覆盖率: {analysis['implementation_rate']:.1f}%")
        report.append("")

        # 按模块统计
        report.append("## 📋 按模块统计")
        report.append("")
        report.append("| 模块 | 总数 | 已实现 | 部分实现 | 未实现 | 覆盖率 |")
        report.append("|------|------|--------|----------|--------|--------|")

        # 按覆盖率排序
        sorted_modules = sorted(
            analysis['module_stats'].items(),
            key=lambda x: x[1]['implemented'] / x[1]['total'] if x[1]['total'] > 0 else 0,
            reverse=True
        )

        for module, stats in sorted_modules:
            total = stats['total']
            implemented = stats['implemented']
            partial = stats['partial']
            missing = stats['missing']
            coverage = implemented / total * 100 if total > 0 else 0

            report.append(f"| {module} | {total} | {implemented} | {partial} | {missing} | {coverage:.1f}% |")

        report.append("")

        # 未实现的API
        missing_apis = [r for r in analysis['results'] if r.coverage_type == "missing"]
        if missing_apis:
            report.append("## ❌ 未实现的API")
            report.append("")

            # 按模块分组
            missing_by_module = defaultdict(list)
            for result in missing_apis:
                missing_by_module[result.api_info.module].append(result)

            for module, apis in sorted(missing_by_module.items()):
                report.append(f"### {module} ({len(apis)}个)")
                for api in apis:
                    report.append(f"- **{api.api_info.name}** - `{api.api_info.method} {api.api_info.endpoint}`")
                    report.append(f"  - 描述: {api.api_info.description}")
                    report.append(f"  - 文档: {api.api_info.doc_url}")
                    report.append("")

        # 部分实现的API
        partial_apis = [r for r in analysis['results'] if r.coverage_type == "partial"]
        if partial_apis:
            report.append("## ⚠️ 部分实现的API")
            report.append("")

            for result in partial_apis:
                api = result.api_info
                report.append(f"### {api.name}")
                report.append(f"- **模块**: {api.module}")
                report.append(f"- **方法**: `{api.method} {api.endpoint}`")
                report.append(f"- **实现文件**: {result.file_path}")
                report.append(f"- **函数名**: {result.function_name}")
                report.append("")

        # 实现建议
        report.append("## 💡 实现建议")
        report.append("")

        # 找出覆盖率最低的模块
        low_coverage_modules = [
            (module, stats) for module, stats in analysis['module_stats'].items()
            if stats['implemented'] / stats['total'] < 0.5 and stats['total'] > 5
        ]

        if low_coverage_modules:
            report.append("### 优先实现模块（覆盖率 < 50%）")
            low_coverage_modules.sort(key=lambda x: x[1]['implemented'] / x[1]['total'])

            for module, stats in low_coverage_modules:
                coverage = stats['implemented'] / stats['total'] * 100
                report.append(f"- **{module}**: {coverage:.1f}% 覆盖率 ({stats['implemented']}/{stats['total']})")
                missing = stats['missing']
                if missing > 0:
                    report.append(f"  - 需要实现 {missing} 个API")
                report.append("")

        return "\n".join(report)

    def save_detailed_csv(self, analysis: Dict, output_path: Path):
        """保存详细的CSV结果"""
        with open(output_path, 'w', newline='', encoding='utf-8') as f:
            writer = csv.writer(f)

            # 写入标题
            writer.writerow([
                'module', 'method', 'endpoint', 'name', 'description', 'doc_url',
                'implemented', 'coverage_type', 'file_path', 'function_name',
                'project', 'resource', 'version', 'api_type'
            ])

            # 写入数据
            for result in analysis['results']:
                api = result.api_info
                writer.writerow([
                    api.module, api.method, api.endpoint, api.name,
                    api.description, api.doc_url,
                    result.implemented, result.coverage_type,
                    result.file_path or '', result.function_name or '',
                    api.project, api.resource, api.version, api.api_type
                ])

def main():
    """主函数"""
    project_root = Path.cwd()
    print("🔍 开始分析API实现覆盖情况...")

    analyzer = APIImplementationAnalyzer(project_root)

    # 分析所有API
    print("📊 正在分析API实现情况...")
    analysis = analyzer.analyze_all_apis()

    if "error" in analysis:
        print(f"❌ 分析失败: {analysis['error']}")
        return

    # 生成报告
    print("📝 生成分析报告...")
    report = analyzer.generate_report(analysis)

    # 保存报告
    report_path = project_root / "reports" / "api_implementation_coverage.md"
    with open(report_path, 'w', encoding='utf-8') as f:
        f.write(report)

    print(f"✅ 报告已保存到: {report_path}")

    # 保存详细CSV
    csv_path = project_root / "reports" / "api_implementation_detailed.csv"
    analyzer.save_detailed_csv(analysis, csv_path)
    print(f"📊 详细数据已保存到: {csv_path}")

    # 打印关键统计
    print("\n" + "="*80)
    print("🎯 关键统计结果:")
    print("="*80)
    print(f"总API数量: {analysis['total_apis']}")
    print(f"✅ 已实现: {analysis['implemented_count']} ({analysis['implementation_rate']:.1f}%)")
    print(f"⚠️  部分实现: {analysis['partial_count']}")
    print(f"❌ 未实现: {analysis['missing_count']}")

    # 显示覆盖率最低的5个模块
    module_stats = analysis['module_stats']
    sorted_modules = sorted(
        module_stats.items(),
        key=lambda x: x[1]['implemented'] / x[1]['total'] if x[1]['total'] > 0 else 0
    )

    print("\n📉 覆盖率最低的模块:")
    for module, stats in sorted_modules[:5]:
        coverage = stats['implemented'] / stats['total'] * 100 if stats['total'] > 0 else 0
        print(f"  {module}: {coverage:.1f}% ({stats['implemented']}/{stats['total']})")

if __name__ == "__main__":
    main()