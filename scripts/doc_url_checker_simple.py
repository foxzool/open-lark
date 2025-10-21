#!/usr/bin/env python3
"""
简化版文档 URL 自动化检查脚本

用于全面检查 open-lark 项目中的文档 URL，不依赖外部库。
"""

import os
import re
import sys
import time
import json
from pathlib import Path
from datetime import datetime
from urllib.request import Request, urlopen
from urllib.error import URLError, HTTPError
from urllib.parse import urlparse

class SimpleDocURLChecker:
    """简化版文档 URL 检查器"""

    def __init__(self, project_root: Path):
        self.project_root = project_root
        self.src_dir = project_root / "src" / "service"
        self.results = {
            "timestamp": datetime.now().isoformat(),
            "summary": {
                "total_files": 0,
                "total_methods": 0,
                "methods_with_docs": 0,
                "methods_without_docs": 0,
                "total_doc_urls": 0,
                "accessible_urls": 0,
                "inaccessible_urls": 0,
                "format_errors": 0,
                "standard_format_urls": 0,
                "old_format_urls": 0
            },
            "files": {},
            "errors": [],
            "recommendations": [],
            "inaccessible_urls": []
        }

        # 文档 URL 模式
        self.standard_format_pattern = r'/// # API文档\s*\n\s*///\s*\n\s*///\s*(https://open\.feishu\.cn/document/[^\s\n]+)'
        self.old_format_pattern = r'/// <(https://open\.feishu\.cn/document/[^\s\n]+)>'
        self.generic_url_pattern = r'https://open\.feishu\.cn/document/[^\s\n\)]+'

        # 异步方法模式
        self.async_method_pattern = r'pub async fn\s+(\w+)'

        # 用户代理
        self.user_agent = "open-lark-doc-checker/1.0"

        # 超时设置
        self.timeout = 10

    def find_all_rust_files(self) -> list:
        """查找所有 Rust 源文件"""
        if not self.src_dir.exists():
            print(f"❌ 源码目录不存在: {self.src_dir}")
            return []

        rust_files = list(self.src_dir.rglob("*.rs"))
        print(f"📁 找到 {len(rust_files)} 个 Rust 文件")
        return rust_files

    def analyze_file(self, file_path: Path) -> dict:
        """分析单个文件的文档状态"""
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()
        except Exception as e:
            error_msg = f"读取文件失败 {file_path.name}: {e}"
            self.results["errors"].append(error_msg)
            return {"error": error_msg}

        # 统计异步方法
        async_methods = re.findall(self.async_method_pattern, content)
        method_count = len(async_methods)

        # 查找不同格式的文档 URL
        standard_urls = re.findall(self.standard_format_pattern, content)
        old_format_urls = re.findall(self.old_format_pattern, content)

        # 查找所有文档 URL（避免重复）
        all_doc_urls = re.findall(self.generic_url_pattern, content)

        # 去重
        doc_urls = list(set(all_doc_urls))

        # 格式错误检测
        format_errors = []

        # 检查是否还有其他非标准格式的 URL
        all_urls_with_markers = re.findall(r'///\s*([^\n]*https://open\.feishu\.cn/document/[^\s\n]+)', content)
        for url_with_marker in all_urls_with_markers:
            if not url_with_marker.strip().startswith('/// # API文档') and not url_with_marker.strip().startswith('/// <https'):
                url = re.search(r'https://open\.feishu\.cn/document/[^\s\n]+', url_with_marker)
                if url:
                    format_errors.append(f"非标准格式: {url.group()}")

        # 检查每个方法是否有文档
        methods_with_docs = set()

        # 分析每个方法的文档状态
        for method in async_methods:
            method_pattern = rf'pub async fn\s+{method}'
            method_match = re.search(method_pattern, content)

            if method_match:
                # 检查方法定义前的文档注释
                start_pos = max(0, method_match.start() - 1000)  # 向前查找1000个字符
                text_before = content[start_pos:method_match.start()]

                if re.search(r'/// # API文档', text_before) or re.search(r'/// <https://open\.feishu\.cn/document/', text_before):
                    methods_with_docs.add(method)

        # 计算覆盖率
        doc_count = len(methods_with_docs)
        coverage = (doc_count / method_count * 100) if method_count > 0 else 0

        return {
            "file_path": str(file_path.relative_to(self.project_root)),
            "total_methods": method_count,
            "methods_with_docs": doc_count,
            "methods_without_docs": method_count - doc_count,
            "doc_urls": doc_urls,
            "standard_format_urls": len(standard_urls),
            "old_format_urls": len(old_format_urls),
            "coverage": coverage,
            "format_errors": format_errors,
            "async_methods": async_methods,
            "methods_with_docs_list": list(methods_with_docs)
        }

    def check_url_accessibility(self, url: str) -> tuple:
        """检查单个 URL 的可访问性"""
        try:
            headers = {
                'User-Agent': self.user_agent,
                'Accept': 'text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8',
                'Accept-Language': 'zh-CN,zh;q=0.9,en;q=0.8',
                'Accept-Encoding': 'gzip, deflate',
                'Connection': 'keep-alive',
            }

            req = Request(url, headers=headers)

            with urlopen(req, timeout=self.timeout) as response:
                status_code = response.getcode()
                is_accessible = 200 <= status_code < 400

                # 检查内容长度
                content = response.read(5000).decode('utf-8', errors='ignore')
                if len(content) < 1000 and ("404" in content or "Not Found" in content or "页面不存在" in content or "找不到" in content):
                    is_accessible = False
                    message = f"内容过短且包含错误信息 (status: {status_code})"
                else:
                    message = f"HTTP {status_code}"

                return url, is_accessible, message, status_code

        except HTTPError as e:
            return url, False, f"HTTP错误 {e.code}", e.code
        except URLError as e:
            return url, False, f"URL错误: {str(e)}", 0
        except Exception as e:
            return url, False, f"未知错误: {str(e)}", 0

    def check_urls_sample(self, urls: list, max_check: int = 20) -> list:
        """检查部分 URL 的可访问性（避免过多请求）"""
        if not urls:
            return []

        # 限制检查数量，避免发送过多请求
        urls_to_check = urls[:max_check]
        print(f"   检查前 {len(urls_to_check)} 个 URL 的可访问性...")

        results = []
        for i, url in enumerate(urls_to_check, 1):
            print(f"   检查 URL {i}/{len(urls_to_check)}: {url[:60]}...")
            result = self.check_url_accessibility(url)
            results.append(result)

            # 添加延迟避免请求过快
            time.sleep(0.5)

        return results

    def generate_recommendations(self):
        """生成改进建议"""
        recommendations = []

        summary = self.results["summary"]

        # 文档覆盖率建议
        if summary["total_methods"] > 0:
            coverage_rate = (summary["methods_with_docs"] / summary["total_methods"] * 100)
            if coverage_rate < 80:
                recommendations.append(f"📝 文档覆盖率较低 ({coverage_rate:.1f}%)，建议为所有 API 方法添加文档 URL")
            elif coverage_rate < 95:
                recommendations.append(f"📝 文档覆盖率良好 ({coverage_rate:.1f}%)，建议完善剩余方法的文档")

        # 格式标准化建议
        if summary["old_format_urls"] > 0:
            recommendations.append(f"📐 发现 {summary['old_format_urls']} 个旧格式文档 URL，建议转换为标准格式")

        if summary["format_errors"] > 0:
            recommendations.append(f"📐 发现 {summary['format_errors']} 个格式错误，需要标准化文档 URL 格式")

        # URL 可访问性建议
        total_checked_urls = summary["accessible_urls"] + summary["inaccessible_urls"]
        if total_checked_urls > 0:
            accessibility_rate = (summary["accessible_urls"] / total_checked_urls * 100)
            if accessibility_rate < 90:
                recommendations.append(f"🔗 文档 URL 可访问性较低 ({accessibility_rate:.1f}%)，需要修复失效链接")

        # 按文件分析
        low_coverage_files = []
        for file_path, file_info in self.results["files"].items():
            if file_info.get("coverage", 0) < 50 and file_info.get("total_methods", 0) > 0:
                low_coverage_files.append(file_path)

        if low_coverage_files:
            recommendations.append(f"📁 以下文件文档覆盖率较低，需要重点关注: {', '.join(low_coverage_files[:5])}")

        self.results["recommendations"] = recommendations

    def print_summary(self):
        """打印检查摘要"""
        summary = self.results["summary"]

        print("\n" + "="*80)
        print("📊 文档 URL 检查摘要")
        print("="*80)

        print(f"📁 检查文件数: {summary['total_files']}")
        print(f"🔧 总异步方法数: {summary['total_methods']}")
        print(f"📝 有文档的方法数: {summary['methods_with_docs']}")
        print(f"❌ 无文档的方法数: {summary['methods_without_docs']}")

        if summary['total_methods'] > 0:
            coverage = (summary['methods_with_docs'] / summary['total_methods'] * 100)
            print(f"📈 文档覆盖率: {coverage:.1f}%")

        print(f"🔗 总文档 URL 数: {summary['total_doc_urls']}")
        print(f"✨ 标准格式 URL 数: {summary['standard_format_urls']}")
        print(f"📜 旧格式 URL 数: {summary['old_format_urls']}")
        print(f"✅ 可访问 URL 数: {summary['accessible_urls']}")
        print(f"❌ 不可访问 URL 数: {summary['inaccessible_urls']}")

        if summary["accessible_urls"] + summary["inaccessible_urls"] > 0:
            accessibility = (summary["accessible_urls"] / (summary["accessible_urls"] + summary["inaccessible_urls"]) * 100)
            print(f"🌐 URL 可访问率: {accessibility:.1f}%")

        print(f"📐 格式错误数: {summary['format_errors']}")

        # 推荐建议
        if self.results["recommendations"]:
            print(f"\n💡 改进建议:")
            for i, rec in enumerate(self.results["recommendations"], 1):
                print(f"   {i}. {rec}")

    def print_detailed_report(self):
        """打印详细报告"""
        print("\n" + "="*80)
        print("📋 详细检查报告")
        print("="*80)

        # 按覆盖率排序文件
        sorted_files = sorted(
            self.results["files"].items(),
            key=lambda x: x[1].get("coverage", 0)
        )

        for file_path, file_info in sorted_files:
            if "error" in file_info:
                print(f"\n❌ {file_path}: {file_info['error']}")
                continue

            if file_info.get("total_methods", 0) == 0:
                continue

            coverage = file_info.get("coverage", 0)
            coverage_emoji = "🟢" if coverage >= 80 else "🟡" if coverage >= 50 else "🔴"

            print(f"\n{coverage_emoji} {file_path}")
            print(f"   方法数: {file_info['total_methods']}")
            print(f"   有文档: {file_info['methods_with_docs']}")
            print(f"   无文档: {file_info['methods_without_docs']}")
            print(f"   覆盖率: {coverage:.1f}%")
            print(f"   标准格式: {file_info['standard_format_urls']}")
            print(f"   旧格式: {file_info['old_format_urls']}")

            if file_info.get("format_errors"):
                print(f"   格式错误: {len(file_info['format_errors'])}")
                for error in file_info["format_errors"][:3]:
                    print(f"     - {error}")

            # 显示缺失文档的方法
            if file_info["methods_without_docs"] > 0:
                all_methods = set(file_info.get("async_methods", []))
                methods_with_docs = set(file_info.get("methods_with_docs_list", []))
                missing_docs = all_methods - methods_with_docs

                if missing_docs:
                    print(f"   缺失文档的方法: {', '.join(list(missing_docs)[:5])}")
                    if len(missing_docs) > 5:
                        print(f"     ... 还有 {len(missing_docs) - 5} 个方法")

        # 显示不可访问的 URL
        if self.results["inaccessible_urls"]:
            print(f"\n❌ 不可访问的 URL:")
            for url, is_accessible, message, status_code in self.results["inaccessible_urls"]:
                print(f"   {url} - {message}")

    def save_report(self, output_file: str = None):
        """保存报告到文件"""
        if output_file is None:
            timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
            output_file = f"doc_check_report_{timestamp}.json"

        output_path = self.project_root / "scripts" / output_file

        try:
            with open(output_path, 'w', encoding='utf-8') as f:
                json.dump(self.results, f, ensure_ascii=False, indent=2)
            print(f"\n💾 报告已保存到: {output_path}")
        except Exception as e:
            print(f"\n❌ 保存报告失败: {e}")

    def run(self, check_urls: bool = True, save_report: bool = False):
        """运行完整检查"""
        print("🚀 开始文档 URL 自动化检查...")
        print("="*80)

        # 查找所有文件
        files = self.find_all_rust_files()
        if not files:
            return

        print(f"📊 分析 {len(files)} 个文件...")

        # 分析所有文件
        for file_path in files:
            file_info = self.analyze_file(file_path)
            self.results["files"][str(file_path.relative_to(self.project_root))] = file_info

        # 汇总统计
        for file_info in self.results["files"].values():
            if "error" in file_info:
                continue

            self.results["summary"]["total_methods"] += file_info["total_methods"]
            self.results["summary"]["methods_with_docs"] += file_info["methods_with_docs"]
            self.results["summary"]["methods_without_docs"] += file_info["methods_without_docs"]
            self.results["summary"]["total_doc_urls"] += len(file_info["doc_urls"])
            self.results["summary"]["standard_format_urls"] += file_info["standard_format_urls"]
            self.results["summary"]["old_format_urls"] += file_info["old_format_urls"]
            self.results["summary"]["format_errors"] += len(file_info.get("format_errors", []))

        self.results["summary"]["total_files"] = len(files)

        # 检查 URL 可访问性
        if check_urls:
            all_urls = []
            for file_info in self.results["files"].values():
                all_urls.extend(file_info.get("doc_urls", []))

            # 去重
            unique_urls = list(set(all_urls))
            print(f"\n🌐 发现 {len(unique_urls)} 个唯一文档 URL")

            if unique_urls:
                url_results = self.check_urls_sample(unique_urls, max_check=15)

                for url, is_accessible, message, status_code in url_results:
                    if is_accessible:
                        self.results["summary"]["accessible_urls"] += 1
                    else:
                        self.results["summary"]["inaccessible_urls"] += 1
                        self.results["inaccessible_urls"].append((url, is_accessible, message, status_code))

        # 生成建议
        self.generate_recommendations()

        # 打印结果
        self.print_summary()
        self.print_detailed_report()

        # 保存报告
        if save_report:
            self.save_report()

def main():
    """主函数"""
    import argparse

    parser = argparse.ArgumentParser(description="简化版文档 URL 自动化检查工具")
    parser.add_argument("--no-url-check", action="store_true", help="跳过 URL 可访问性检查")
    parser.add_argument("--save-report", action="store_true", help="保存详细报告到文件")
    parser.add_argument("--project-root", type=str, help="项目根目录路径")

    args = parser.parse_args()

    # 确定项目根目录
    if args.project_root:
        project_root = Path(args.project_root)
    else:
        # 脚本在 scripts 目录下，项目根目录是上级目录
        project_root = Path(__file__).parent.parent

    if not project_root.exists():
        print(f"❌ 项目根目录不存在: {project_root}")
        sys.exit(1)

    # 创建检查器并运行
    checker = SimpleDocURLChecker(project_root)

    try:
        checker.run(check_urls=not args.no_url_check, save_report=args.save_report)
    except KeyboardInterrupt:
        print("\n\n⚠️  检查被用户中断")
        sys.exit(1)
    except Exception as e:
        print(f"\n❌ 检查过程中发生错误: {e}")
        sys.exit(1)

if __name__ == "__main__":
    main()