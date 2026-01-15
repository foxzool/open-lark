#!/usr/bin/env python3
"""
Calendar API 自动化重构工具

用途：
1. 分析 Calendar API 文件，识别需要重构的 API
2. 自动生成强类型响应结构
3. 自动应用常见修复模式
4. 生成重构报告

使用方法：
    python3 scripts/refactor_calendar_apis.py --analyze
    python3 scripts/refactor_calendar_apis.py --refactor calendar/v4/event/list
    python3 scripts/refactor_calendar_apis.py --report
"""

import os
import re
import sys
import json
import argparse
from pathlib import Path
from typing import Dict, List, Tuple, Optional

# 颜色输出
class Colors:
    GREEN = '\033[92m'
    RED = '\033[91m'
    YELLOW = '\033[93m'
    BLUE = '\033[94m'
    END = '\033[0m'

def print_success(msg: str):
    print(f"{Colors.GREEN}✓{Colors.END} {msg}")

def print_error(msg: str):
    print(f"{Colors.RED}✗{Colors.END} {msg}")

def print_warning(msg: str):
    print(f"{Colors.YELLOW}⚠{Colors.END} {msg}")

def print_info(msg: str):
    print(f"{Colors.BLUE}ℹ{Colors.END} {msg}")

class CalendarAPIAnalyzer:
    """Calendar API 分析器"""

    def __init__(self, base_path: str = "crates/openlark-meeting/src"):
        self.base_path = Path(base_path)
        self.api_files = []
        self.analysis_results = []

    def find_api_files(self) -> List[Path]:
        """查找所有 Calendar API 文件"""
        print_info("扫描 Calendar API 文件...")
        calendar_path = self.base_path / "calendar" / "v4"

        if not calendar_path.exists():
            print_error(f"路径不存在: {calendar_path}")
            return []

        # 查找所有 .rs 文件
        api_files = []
        for rs_file in calendar_path.rglob("*.rs"):
            # 跳过 mod.rs 和 common.rs
            if rs_file.name not in ["mod.rs", "common.rs", "models.rs", "responses_new.rs"]:
                api_files.append(rs_file)

        self.api_files = api_files
        print_success(f"找到 {len(api_files)} 个 API 文件")
        return api_files

    def analyze_file(self, file_path: Path) -> Dict:
        """分析单个 API 文件"""
        content = file_path.read_text(encoding='utf-8')

        result = {
            'file_path': str(file_path.relative_to(self.base_path)),
            'api_name': file_path.stem,
            'issues': [],
            'recommendations': [],
            'uses_strong_types': False,
            'uses_api_endpoint_enum': False,
            'uses_weak_type': False,
            'uses_unwrap_default': False,
            'uses_custom_format': False,
            'response_type': None,
        }

        # 检查是否使用强类型
        if re.search(r'SDKResult<\w+Response>', content):
            result['uses_strong_types'] = True
            result['response_type'] = re.search(r'SDKResult<(\w+Response)>', content).group(1)

        # 检查是否使用弱类型
        if 'SDKResult<serde_json::Value>' in content:
            result['uses_weak_type'] = True
            result['issues'].append("使用弱类型 serde_json::Value")
            result['recommendations'].append("建议转换为强类型")

        # 检查是否使用 API endpoint 枚举
        if 'CalendarApiV4::' in content:
            result['uses_api_endpoint_enum'] = True
        else:
            result['issues'].append("未使用 CalendarApiV4 枚举")
            result['recommendations'].append("建议迁移到 CalendarApiV4 枚举")

        # 检查 unwrap_or_default 使用
        if 'unwrap_or_default()' in content:
            result['uses_unwrap_default'] = True
            result['issues'].append("使用 unwrap_or_default()")
            result['recommendations'].append("建议改为 ok_or_else(validation_error)")

        # 检查 ResponseFormat::Custom 使用
        if 'ResponseFormat::Custom' in content:
            result['uses_custom_format'] = True
            result['issues'].append("使用无效的 ResponseFormat::Custom")
            result['recommendations'].append("改为 ResponseFormat::Data")

        return result

    def analyze_all(self) -> List[Dict]:
        """分析所有 API 文件"""
        results = []
        for file_path in self.api_files:
            result = self.analyze_file(file_path)
            results.append(result)
        self.analysis_results = results
        return results

    def print_summary(self):
        """打印分析摘要"""
        total = len(self.analysis_results)
        strong_typed = sum(1 for r in self.analysis_results if r['uses_strong_types'])
        weak_typed = sum(1 for r in self.analysis_results if r['uses_weak_type'])

        print("\n" + "="*60)
        print_info("Calendar API 重构状态摘要")
        print("="*60)
        print(f"总 API 数量: {total}")
        print_success(f"已使用强类型: {strong_typed}")
        print_warning(f"使用弱类型: {weak_typed}")
        print(f"进度: {strong_typed}/{total} ({strong_typed*100//total}%)" if total > 0 else "进度: 0%")

        # 按模块统计
        modules = {}
        for r in self.analysis_results:
            parts = r['file_path'].split('/')
            if len(parts) >= 4:
                module = '/'.join(parts[2:4])
                if module not in modules:
                    modules[module] = {'total': 0, 'strong': 0}
                modules[module]['total'] += 1
                if r['uses_strong_types']:
                    modules[module]['strong'] += 1

        print("\n按模块统计:")
        for module, stats in sorted(modules.items()):
            progress = stats['strong'] * 100 // stats['total'] if stats['total'] > 0 else 0
            status = Colors.GREEN + "✓" + Colors.END if progress == 100 else Colors.YELLOW + str(progress) + "%" + Colors.END
            print(f"  {module:30s} {stats['strong']:2d}/{stats['total']:2d} {status}")

    def print_details(self):
        """打印详细分析结果"""
        print("\n" + "="*60)
        print_info("详细分析结果")
        print("="*60)

        for result in sorted(self.analysis_results, key=lambda x: x['file_path']):
            status = Colors.GREEN + "✓ 强类型" + Colors.END if result['uses_strong_types'] else Colors.RED + "✗ 弱类型" + Colors.END
            print(f"\n{status} {result['file_path']}")

            if result['response_type']:
                print(f"  响应类型: {result['response_type']}")

            if result['issues']:
                print_warning("  发现的问题:")
                for issue in result['issues']:
                    print(f"    - {issue}")

            if result['recommendations']:
                print_info("  建议:")
                for rec in result['recommendations']:
                    print(f"    - {rec}")

    def save_report(self, output_path: str = "/tmp/calendar_api_refactor_report.json"):
        """保存分析报告到 JSON"""
        report = {
            'total_apis': len(self.analysis_results),
            'strong_typed': sum(1 for r in self.analysis_results if r['uses_strong_types']),
            'weak_typed': sum(1 for r in self.analysis_results if r['uses_weak_type']),
            'progress': f"{sum(1 for r in self.analysis_results if r['uses_strong_types']) * 100 // len(self.analysis_results)}%",
            'details': self.analysis_results,
        }

        Path(output_path).write_text(json.dumps(report, indent=2, ensure_ascii=False), encoding='utf-8')
        print_success(f"报告已保存到: {output_path}")

class CalendarAPIRefactorer:
    """Calendar API 重构器"""

    def __init__(self, base_path: str = "crates/openlark-meeting/src"):
        self.base_path = Path(base_path)

    def generate_response_template(self, file_path: Path, api_doc_url: str) -> str:
        """生成响应结构模板"""
        api_name = file_path.stem
        response_name = f"{api_name.capitalize()}Response"

        template = f'''/// {api_name.capitalize()} 响应数据
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct {response_name} {{
    // TODO: 根据在线文档填充字段
    // 文档链接: {api_doc_url}
    pub data: serde_json::Value,  // 临时占位，请替换为实际结构
}}

impl ApiResponseTrait for {response_name} {{
    fn data_format() -> ResponseFormat {{
        ResponseFormat::Data
    }}
}}
'''
        return template

    def fix_common_issues(self, file_path: Path, content: str) -> Tuple[str, List[str]]:
        """修复常见问题"""
        issues_fixed = []

        # 修复 ResponseFormat::Custom -> ResponseFormat::Data
        if 'ResponseFormat::Custom' in content:
            content = re.sub(r'ResponseFormat::Custom\([^)]+\)', 'ResponseFormat::Data', content)
            issues_fixed.append("修复 ResponseFormat::Custom")

        # 修复 ResponseFormat::None -> ResponseFormat::Data
        if 'ResponseFormat::None' in content:
            content = content.replace('ResponseFormat::None', 'ResponseFormat::Data')
            issues_fixed.append("修复 ResponseFormat::None")

        # 修复 unwrap_or_default() -> ok_or_else(validation_error)
        # 注意：这里只是标记，实际修复需要上下文判断
        if 'unwrap_or_default()' in content and 'validation_error' not in content:
            # 添加注释，提示需要修复
            content = re.sub(
                r'(\.unwrap_or_default\(\))',
                r'// TODO: 需要改为 .ok_or_else(|| validation_error("响应数据为空", "服务器没有返回有效的数据"))\1',
                content
            )
            issues_fixed.append("标记 unwrap_or_default 需要修复")

        return content, issues_fixed

def main():
    parser = argparse.ArgumentParser(description='Calendar API 自动化重构工具')
    parser.add_argument('--analyze', action='store_true', help='分析所有 API 文件')
    parser.add_argument('--refactor', type=str, help='重构指定 API 文件')
    parser.add_argument('--report', action='store_true', help='生成并保存报告')
    parser.add_argument('--base-path', type=str, default='crates/openlark-meeting/src', help='基础路径')

    args = parser.parse_args()

    if args.analyze:
        # 分析模式
        analyzer = CalendarAPIAnalyzer(args.base_path)
        analyzer.find_api_files()
        analyzer.analyze_all()
        analyzer.print_summary()
        analyzer.print_details()

        if args.report:
            analyzer.save_report()

    elif args.refactor:
        # 重构模式
        file_path = Path(args.refactor)
        if not file_path.exists():
            print_error(f"文件不存在: {file_path}")
            sys.exit(1)

        print_info(f"准备重构: {file_path}")

        # 这里可以添加自动重构逻辑
        print_warning("自动重构功能开发中，请手动完成重构")

    else:
        parser.print_help()

if __name__ == '__main__':
    main()
