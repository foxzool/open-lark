#!/usr/bin/env python3
"""
增强的API实现覆盖分析工具 v2.0
基于实际代码模式精确匹配API实现
"""

import csv
import os
import re
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
    confidence: float = 0.0  # 匹配置信度 0-1

class EnhancedAPIAnalyzer:
    """增强的API实现分析器"""

    def __init__(self, project_root: Path):
        self.project_root = project_root
        self.service_dir = project_root / "src" / "service"

        # 基于实际文件的模块映射
        self.existing_services = self._discover_existing_services()

        # API端点到方法的映射模式（基于实际代码分析）
        self.endpoint_patterns = {
            # IM 模块
            "/open-apis/im/v1/messages": ["create", "send"],
            "/open-apis/im/v1/messages/{message_id}": ["delete", "get", "update"],
            "/open-apis/im/v1/messages/{message_id}/reply": ["reply"],
            "/open-apis/im/v1/chats/{chat_id}/messages": ["create", "send"],

            # Contact 模块
            "/open-apis/contact/v3/users": ["create", "list", "batch"],
            "/open-apis/contact/v3/users/{user_id}": ["get", "delete", "patch", "update"],
            "/open-apis/contact/v3/users/batch/get_id": ["batch_get_id"],
            "/open-apis/contact/v3/users/search": ["search"],
            "/open-apis/contact/v3/users/find_by_department": ["find_by_department"],
            "/open-apis/contact/v3/departments": ["create", "list"],
            "/open-apis/contact/v3/departments/{department_id}": ["get", "delete", "update"],

            # Auth 模块
            "/open-apis/auth/v3/tenant_access_token/internal": ["get_tenant_access_token_internal"],
            "/open-apis/auth/v3/app_access_token/internal": ["get_app_access_token_internal"],
            "/open-apis/auth/v3/app_access_token": ["get_app_access_token"],
            "/open-apis/auth/v3/tenant_access_token": ["get_tenant_access_token"],
            "/open-apis/authen/v1/user_info": ["get_user_info"],

            # 云文档模块
            "/open-apis/drive/v1/files": ["upload", "create"],
            "/open-apis/drive/v1/files/{file_token}": ["get", "delete", "update"],
            "/open-apis/drive/v1/permissions/{file_token}/members": ["batch_create"],
            "/open-apis/drive/v1/permissions/{file_token}/public": ["update", "get"],

            # 其他常见模式
            "/open-apis/{module}/v{version}/{resource}": ["create", "list", "get", "update", "delete"],
            "/open-apis/{module}/v{version}/{resource}/batch": ["batch_create", "batch_get"],
            "/open-apis/{module}/v{version}/{resource}/search": ["search"],
        }

    def _discover_existing_services(self) -> Dict[str, Dict]:
        """发现实际存在的服务和文件"""
        services = {}

        if not self.service_dir.exists():
            return services

        for service_path in self.service_dir.iterdir():
            if not service_path.is_dir():
                continue

            service_name = service_path.name
            service_info = {
                "path": service_path,
                "versions": {},
                "files": []
            }

            # 查找版本目录
            for item in service_path.iterdir():
                if item.is_dir() and re.match(r'v\d+', item.name):
                    service_info["versions"][item.name] = item

                # 查找所有.rs文件
                if item.suffix == '.rs':
                    service_info["files"].append(item)

            # 递归查找子目录中的.rs文件
            for rust_file in service_path.rglob("*.rs"):
                if rust_file not in service_info["files"]:
                    service_info["files"].append(rust_file)

            if service_info["files"]:  # 只包含有文件的模块
                services[service_name] = service_info

        return services

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

    def normalize_module_name(self, csv_module: str) -> Optional[str]:
        """规范化模块名称"""
        # CSV模块名到代码模块名的映射
        module_mapping = {
            "auth": "authentication",
            "authen": "authentication",
            "passport": "authentication",
            "im": "im",
            "contact": "contact",
            "drive": "cloud_docs",
            "ccm": "cloud_docs",
            "calendar": "calendar",
            "approval": "approval",
            "attendance": "attendance",
            "helpdesk": "helpdesk",
            "search": "search",
            "ai": "ai",
            "mail": "mail",
            "application": "application",
            "tenant": "tenant",
            "group": "group",
            "task": "task",
            "meeting": "meeting",
            "vc": "vc",
            "minutes": "minutes",
            "cardkit": "cardkit",
            "directory": "directory",
            "personal_settings": "personal_settings",
            "verification": "verification",
            "event": "event",
            "base": "base",
            "board": "board",
            "meeting_room": "meeting_room",
            "verification_information": "verification",
            "app_engine": "app_engine",
            "aily": "aily",
            "admin": "admin",
            "moments": "moments",
            "ehr": "ehr",
            "feishu_people": "feishu_people",
            "corehr": "corehr",
            "compensation_management": "compensation_management",
            "payroll": "payroll",
            "hire": "hire",
            "okr": "okr",
            "human_authentication": "human_authentication",
            "acs": "acs",
            "performance": "performance",
            "baike": "baike",
            "security_and_compliance": "security_and_compliance",
            "trust_party": "trust_party",
            "workplace": "workplace",
            "mdm": "mdm",
            "report": "report"
        }

        mapped = module_mapping.get(csv_module)
        return mapped if mapped and mapped in self.existing_services else None

    def extract_method_candidates(self, api_info: APIInfo) -> List[Tuple[str, float]]:
        """提取可能的方法名和置信度"""
        candidates = []

        # 基于端点的直接映射
        endpoint = api_info.endpoint
        method = api_info.method.upper()

        # 处理常见的端点模式
        if "{message_id}" in endpoint:
            if method == "DELETE":
                candidates.append(("delete", 0.9))
            elif method == "PATCH":
                candidates.append(("update", 0.9))
            elif method == "GET":
                candidates.append(("get", 0.9))
        elif endpoint.endswith("/messages"):
            if method == "POST":
                candidates.append(("create", 0.9))
                candidates.append(("send", 0.8))
        elif endpoint.endswith("/batch"):
            if method == "POST":
                candidates.append(("batch", 0.8))
                candidates.append(("batch_create", 0.7))
        elif endpoint.endswith("/search"):
            if method == "POST":
                candidates.append(("search", 0.9))
        elif "/batch_get_id" in endpoint:
            candidates.append(("batch_get_id", 0.9))
        elif "/find_by_department" in endpoint:
            candidates.append(("find_by_department", 0.9))
        elif "/tenant_access_token" in endpoint:
            candidates.append(("get_tenant_access_token", 0.9))
        elif "/app_access_token" in endpoint:
            candidates.append(("get_app_access_token", 0.9))
        elif "/user_info" in endpoint:
            candidates.append(("get_user_info", 0.9))

        # 基于资源名的一般模式
        resource = api_info.resource
        if resource and not candidates:
            if method == "GET":
                if "{user_id}" in endpoint or "{department_id}" in endpoint:
                    candidates.append(("get", 0.7))
                else:
                    candidates.append(("list", 0.6))
                    candidates.append(("get", 0.5))
            elif method == "POST":
                candidates.append(("create", 0.7))
            elif method == "DELETE":
                candidates.append(("delete", 0.8))
            elif method == "PATCH":
                candidates.append(("update", 0.7))
            elif method == "PUT":
                candidates.append(("update", 0.7))

        # 基于endpoint最后一段的启发式方法
        if not candidates:
            parts = endpoint.strip('/').split('/')
            if parts:
                last_part = parts[-1]
                if last_part in ['users', 'departments', 'files', 'messages']:
                    if method == "GET" and "{" not in endpoint:
                        candidates.append(("list", 0.5))
                    elif method == "POST":
                        candidates.append(("create", 0.5))

        return candidates

    def search_in_service_files(self, service_name: str, version: str, method_candidates: List[str]) -> Tuple[bool, Optional[Path], Optional[str], float]:
        """在服务文件中搜索方法实现"""
        if service_name not in self.existing_services:
            return False, None, None, 0.0

        service_info = self.existing_services[service_name]

        # 优先搜索特定版本的文件
        search_files = []
        if version in service_info["versions"]:
            version_dir = service_info["versions"][version]
            search_files.extend(version_dir.rglob("*.rs"))

        # 搜索所有服务文件
        search_files.extend(service_info["files"])

        for method_name in method_candidates:
            for file_path in search_files:
                try:
                    with open(file_path, 'r', encoding='utf-8') as f:
                        content = f.read()

                    # 检查方法定义
                    method_patterns = [
                        rf"pub async fn {method_name}\s*\(",
                        rf"pub fn {method_name}\s*\(",
                        rf"async fn {method_name}\s*\(",
                        rf"fn {method_name}\s*\("
                    ]

                    for pattern in method_patterns:
                        if re.search(pattern, content):
                            # 验证包含Transport::request
                            if "Transport::request" in content:
                                confidence = self._calculate_confidence(content, method_name)
                                return True, file_path, method_name, confidence

                except Exception as e:
                    print(f"⚠️  读取文件失败 {file_path}: {e}")
                    continue

        return False, None, None, 0.0

    def _calculate_confidence(self, content: str, method_name: str) -> float:
        """计算匹配置信度"""
        confidence = 0.5  # 基础置信度

        # 包含完整的async方法签名
        if re.search(rf"pub async fn {method_name}\s*\([^)]+\)\s*->", content):
            confidence += 0.2

        # 包含Transport::request调用
        if "Transport::request" in content:
            confidence += 0.2

        # 包含适当的HTTP方法
        if "Method::" in content:
            confidence += 0.1

        # 包含适当的endpoint常量
        if re.search(r"[A-Z_]*ENDPOINT", content):
            confidence += 0.1

        return min(confidence, 1.0)

    def analyze_api_implementation(self, api_info: APIInfo) -> ImplementationResult:
        """分析单个API的实现情况"""
        # 规范化模块名称
        service_name = self.normalize_module_name(api_info.module)
        if not service_name:
            return ImplementationResult(api_info, False, None, None, "module_not_found", 0.0)

        # 提取方法候选
        method_candidates = self.extract_method_candidates(api_info)
        if not method_candidates:
            return ImplementationResult(api_info, False, None, None, "no_method_candidates", 0.0)

        # 搜索实现
        version = api_info.version if api_info.version else "v1"  # 默认版本
        implemented, file_path, method_name, confidence = self.search_in_service_files(
            service_name, version, [m[0] for m in method_candidates]
        )

        if implemented:
            coverage_type = "full" if confidence > 0.8 else "partial"
            return ImplementationResult(api_info, True, str(file_path), method_name, coverage_type, confidence)

        return ImplementationResult(api_info, False, None, None, "missing", 0.0)

    def analyze_all_apis(self) -> Dict:
        """分析所有API的实现情况"""
        apis = self.load_apis_from_csv()
        print(f"📊 从CSV加载了 {len(apis)} 个API")
        print(f"🔍 发现 {len(self.existing_services)} 个服务模块: {list(self.existing_services.keys())}")

        if not apis:
            return {"error": "No APIs loaded from CSV"}

        results = []
        implemented_count = 0
        missing_count = 0
        partial_count = 0

        for i, api_info in enumerate(apis):
            if i % 100 == 0:
                print(f"  进度: {i}/{len(apis)} ({i/len(apis)*100:.1f}%)")

            result = self.analyze_api_implementation(api_info)
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
            "results": results,
            "existing_services": self.existing_services
        }

    def generate_report(self, analysis: Dict) -> str:
        """生成分析报告"""
        report = []
        report.append("# API实现覆盖分析报告 v2.0")
        report.append("=" * 80)
        report.append("")

        # 总体统计
        report.append("## 📊 总体统计")
        report.append(f"- 总API数量: {analysis['total_apis']}")
        report.append(f"- ✅ 已完整实现: {analysis['implemented_count']}")
        report.append(f"- ⚠️  部分实现: {analysis['partial_count']}")
        report.append(f"- ❌ 未实现: {analysis['missing_count']}")
        report.append(f"- 📈 实现覆盖率: {analysis['implementation_rate']:.1f}%")
        report.append("")

        # 现有服务模块
        report.append("## 🔍 发现的服务模块")
        report.append("")
        for service_name, service_info in analysis['existing_services'].items():
            file_count = len(service_info['files'])
            versions = list(service_info['versions'].keys())
            report.append(f"- **{service_name}**: {file_count} 个文件, 版本: {versions}")
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

        # 已实现的API示例
        implemented_apis = [r for r in analysis['results'] if r.coverage_type in ["full", "partial"]]
        if implemented_apis:
            report.append("## ✅ 已实现的API示例")
            report.append("")

            # 按置信度排序
            implemented_apis.sort(key=lambda x: x.confidence, reverse=True)

            for result in implemented_apis[:20]:  # 显示前20个
                api = result.api_info
                report.append(f"### {api.name}")
                report.append(f"- **模块**: {api.module}")
                report.append(f"- **方法**: `{api.method} {api.endpoint}`")
                report.append(f"- **实现文件**: {result.file_path}")
                report.append(f"- **函数名**: {result.function_name}")
                report.append(f"- **置信度**: {result.confidence:.2f}")
                report.append("")

        # 高优先级未实现API
        missing_apis = [r for r in analysis['results'] if r.coverage_type == "missing"]
        if missing_apis:
            report.append("## ❌ 高优先级未实现API")
            report.append("")

            # 按模块分组，只显示有服务模块的API
            missing_by_module = defaultdict(list)
            for result in missing_apis:
                service_name = self.normalize_module_name(result.api_info.module)
                if service_name:  # 只显示有对应服务的
                    missing_by_module[result.api_info.module].append(result)

            # 按重要性排序（核心模块优先）
            priority_modules = ["auth", "authen", "im", "contact", "drive", "ccm"]

            for module in priority_modules:
                if module in missing_by_module:
                    apis = missing_by_module[module]
                    report.append(f"### {module} ({len(apis)}个)")
                    for api in apis[:5]:  # 每个模块最多显示5个
                        report.append(f"- **{api.api_info.name}** - `{api.api_info.method} {api.api_info.endpoint}`")
                        report.append(f"  - 描述: {api.api_info.description}")
                        report.append("")
                    if len(apis) > 5:
                        report.append(f"  - ... 还有 {len(apis) - 5} 个API未实现")
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
                'confidence', 'project', 'resource', 'version', 'api_type'
            ])

            # 写入数据
            for result in analysis['results']:
                api = result.api_info
                writer.writerow([
                    api.module, api.method, api.endpoint, api.name,
                    api.description, api.doc_url,
                    result.implemented, result.coverage_type,
                    result.file_path or '', result.function_name or '',
                    result.confidence,
                    api.project, api.resource, api.version, api.api_type
                ])

def main():
    """主函数"""
    project_root = Path.cwd()
    print("🔍 开始增强API实现覆盖分析...")

    analyzer = EnhancedAPIAnalyzer(project_root)

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
    report_path = project_root / "reports" / "enhanced_api_implementation_coverage.md"
    with open(report_path, 'w', encoding='utf-8') as f:
        f.write(report)

    print(f"✅ 增强报告已保存到: {report_path}")

    # 保存详细CSV
    csv_path = project_root / "reports" / "enhanced_api_implementation_detailed.csv"
    analyzer.save_detailed_csv(analysis, csv_path)
    print(f"📊 详细数据已保存到: {csv_path}")

    # 打印关键统计
    print("\n" + "="*80)
    print("🎯 增强分析结果:")
    print("="*80)
    print(f"总API数量: {analysis['total_apis']}")
    print(f"✅ 已完整实现: {analysis['implemented_count']} ({analysis['implementation_rate']:.1f}%)")
    print(f"⚠️  部分实现: {analysis['partial_count']}")
    print(f"❌ 未实现: {analysis['missing_count']}")
    print(f"🔍 发现服务模块: {len(analysis['existing_services'])}")

    # 显示实现率最高的模块
    module_stats = analysis['module_stats']
    implemented_modules = {k: v for k, v in module_stats.items() if v['implemented'] > 0}

    if implemented_modules:
        print("\n🏆 实现率最高的模块:")
        sorted_implemented = sorted(
            implemented_modules.items(),
            key=lambda x: x[1]['implemented'] / x[1]['total'] if x[1]['total'] > 0 else 0,
            reverse=True
        )
        for module, stats in sorted_implemented[:5]:
            coverage = stats['implemented'] / stats['total'] * 100 if stats['total'] > 0 else 0
            print(f"  {module}: {coverage:.1f}% ({stats['implemented']}/{stats['total']})")

if __name__ == "__main__":
    main()