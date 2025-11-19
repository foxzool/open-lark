#!/usr/bin/env python3
"""
API模块迁移工具 - Python版本
更精确的AST分析和替换工具
"""

import os
import re
import ast
import sys
import argparse
from pathlib import Path
from typing import List, Dict, Tuple, Optional
from dataclasses import dataclass

# 颜色输出
class Colors:
    RED = '\033[0;31m'
    GREEN = '\033[0;32m'
    YELLOW = '\033[1;33m'
    BLUE = '\033[0;34m'
    NC = '\033[0m'

def log_info(msg: str):
    print(f"{Colors.BLUE}[INFO]{Colors.NC} {msg}")

def log_success(msg: str):
    print(f"{Colors.GREEN}[SUCCESS]{Colors.NC} {msg}")

def log_warning(msg: str):
    print(f"{Colors.YELLOW}[WARNING]{Colors.NC} {msg}")

def log_error(msg: str):
    print(f"{Colors.RED}[ERROR]{Colors.NC} {msg}")

@dataclass
class MigrationRule:
    """迁移规则定义"""
    old_pattern: str
    new_pattern: str
    rule_type: str  # 'type', 'import', 'function'
    description: str = ""

class APIMigrationTool:
    """API迁移工具"""

    def __init__(self):
        self.total_files = 0
        self.modified_files = 0
        self.errors = 0

        # 迁移规则定义
        self.rules = [
            # 类型映射
            MigrationRule(
                "api_resp::BaseResponse",
                "api::Response",
                "type",
                "基础响应类型"
            ),
            MigrationRule(
                "api_resp::RawResponse",
                "api::RawResponse",
                "type",
                "原始响应类型"
            ),
            MigrationRule(
                "api_resp::ApiResponseTrait",
                "api::ApiResponseTrait",
                "type",
                "API响应特征"
            ),
            MigrationRule(
                "api_resp::ErrorInfo",
                "api::ErrorInfo",
                "type",
                "错误信息类型"
            ),
            MigrationRule(
                "api_resp::ResponseFormat",
                "api::ResponseFormat",
                "type",
                "响应格式枚举"
            ),

            # 请求类型
            MigrationRule(
                "api_req::ApiRequest",
                "api::ApiRequest",
                "type",
                "API请求类型"
            ),
            MigrationRule(
                "api_req::HttpMethod",
                "api::HttpMethod",
                "type",
                "HTTP方法枚举"
            ),
            MigrationRule(
                "api_req::RequestData",
                "api::RequestData",
                "type",
                "请求数据类型"
            ),

            # 完整路径映射
            MigrationRule(
                "crate::api_resp::BaseResponse",
                "api::BaseResponse",
                "type",
                "完整路径基础响应"
            ),
            MigrationRule(
                "crate::api_resp::RawResponse",
                "api::RawResponse",
                "type",
                "完整路径原始响应"
            ),
            MigrationRule(
                "crate::api_req::ApiRequest",
                "api::ApiRequest",
                "type",
                "完整路径API请求"
            ),
        ]

        # 导入模式规则
        self.import_rules = [
            # 优化导入路径，优先使用api::
            (r"use crate::api_resp::([^;]+);", r"use api::\1;"),
            (r"use crate::api_req::([^;]+);", r"use api::\1;"),
            (r"use super::api_resp::([^;]+);", r"use super::api::\1;"),
            (r"use super::api_req::([^;]+);", r"use super::api::\1;"),
            (r"use crate::api_resp::prelude::\*", r"use api::prelude::*;"),
            (r"use crate::api_req::prelude::\*", r"use api::prelude::*;"),
            # 处理响应类型导入
            (r"use api::responses::\{([^}]*)\}", r"use api::{\1}"),
        ]

        # 特殊处理规则
        self.special_rules = [
            (r"\bBaseResponse<", r"Response<"),
            (r"\bapi_resp::", r"api::"),
            (r"\bapi_req::", r"api::"),
        ]

    def apply_rules_to_content(self, content: str) -> Tuple[str, bool]:
        """应用所有规则到内容"""
        modified = False
        new_content = content

        # 应用类型映射规则
        for rule in self.rules:
            if rule.old_pattern in new_content:
                count = new_content.count(rule.old_pattern)
                new_content = new_content.replace(rule.old_pattern, rule.new_pattern)
                if count > 0:
                    modified = True
                    log_info(f"  应用规则 {rule.old_pattern} -> {rule.new_pattern} ({count}次)")

        # 应用导入规则
        for pattern, replacement in self.import_rules:
            if re.search(pattern, new_content):
                old_content = new_content
                new_content = re.sub(pattern, replacement, new_content)
                if old_content != new_content:
                    modified = True
                    log_info(f"  应用导入规则: {pattern}")

        # 应用特殊规则
        for pattern, replacement in self.special_rules:
            if re.search(pattern, new_content):
                old_content = new_content
                new_content = re.sub(pattern, replacement, new_content)
                if old_content != new_content:
                    modified = True
                    log_info(f"  应用特殊规则: {pattern}")

        return new_content, modified

    def process_file(self, file_path: Path, dry_run: bool = False) -> bool:
        """处理单个文件"""
        try:
            self.total_files += 1
            log_info(f"处理文件 ({self.total_files}): {file_path.name}")

            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()

            new_content, modified = self.apply_rules_to_content(content)

            if modified:
                if dry_run:
                    log_success(f"  [预览] 将修改: {file_path.name}")
                    return True

                # 写入修改后的内容
                with open(file_path, 'w', encoding='utf-8') as f:
                    f.write(new_content)

                log_success(f"  已更新: {file_path.name}")
                self.modified_files += 1
                return True
            else:
                log_info(f"  无需修改: {file_path.name}")
                return False

        except Exception as e:
            log_error(f"  处理文件失败 {file_path.name}: {e}")
            self.errors += 1
            return False

    def validate_migration(self, file_path: Path) -> bool:
        """验证迁移结果"""
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()

            valid = True

            # 检查是否还有旧的API引用
            old_patterns = ["api_req::", "api_resp::"]
            for pattern in old_patterns:
                matches = re.findall(pattern, content)
                if matches:
                    log_warning(f"  仍有旧API引用 ({pattern}): {len(matches)}次")
                    valid = False

                    # 显示上下文
                    lines = content.split('\n')
                    for i, line in enumerate(lines):
                        if pattern in line:
                            log_warning(f"    第{i+1}行: {line.strip()}")
                            if len([l for l in log_warning.__self__.calls if l]) >= 3:  # 限制显示数量
                                break

            # 基本语法检查
            try:
                ast.parse(content)
            except SyntaxError as e:
                log_error(f"  语法错误: {e}")
                valid = False

            return valid

        except Exception as e:
            log_error(f"  验证失败: {e}")
            return False

    def process_directory(self, dir_path: Path, dry_run: bool = False):
        """处理目录"""
        log_info(f"处理目录: {dir_path}")

        # 查找所有Rust文件
        rust_files = list(dir_path.rglob("*.rs"))

        for file_path in rust_files:
            if self.process_file(file_path, dry_run):
                if not dry_run:
                    self.validate_migration(file_path)

    def show_statistics(self):
        """显示统计信息"""
        print()
        log_info("========== 迁移统计 ==========")
        log_info(f"总文件数: {self.total_files}")
        log_success(f"已修改文件: {self.modified_files}")
        log_warning(f"错误文件数: {self.errors}")

        if self.errors == 0:
            log_success("🎉 迁移完成！")
        else:
            log_warning(f"⚠️  发现 {self.errors} 个错误，需要手动检查")

    def show_rules(self):
        """显示所有迁移规则"""
        log_info("========== API迁移规则 ==========")

        print("\n类型映射规则:")
        for rule in self.rules:
            if rule.rule_type == "type":
                log_info(f"  {rule.old_pattern} -> {rule.new_pattern}")
                print(f"    描述: {rule.description}")

        print("\n导入规则:")
        for pattern, replacement in self.import_rules:
            log_info(f"  {pattern} -> {replacement}")

        print("\n特殊规则:")
        for pattern, replacement in self.special_rules:
            log_info(f"  {pattern} -> {replacement}")

def main():
    parser = argparse.ArgumentParser(
        description="API模块迁移工具 - 从api_req/api_resp迁移到新api模块",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
示例:
  %(prog)s src/                    # 迁移整个src目录
  %(prog)s src/services/           # 迁移特定目录
  %(prog)s src/lib.rs              # 迁移单个文件
  %(prog)s --dry-run src/          # 预览模式
  %(prog)s --show-rules            # 显示所有规则
        """
    )

    parser.add_argument(
        "target",
        nargs="?",
        help="要迁移的文件或目录路径"
    )

    parser.add_argument(
        "-d", "--dry-run",
        action="store_true",
        help="预览模式，不实际修改文件"
    )

    parser.add_argument(
        "-v", "--verbose",
        action="store_true",
        help="详细输出"
    )

    parser.add_argument(
        "--show-rules",
        action="store_true",
        help="显示所有迁移规则"
    )

    args = parser.parse_args()

    # 创建迁移工具
    tool = APIMigrationTool()

    # 显示规则
    if args.show_rules:
        tool.show_rules()
        return

    # 检查目标
    if not args.target:
        log_error("请指定要迁移的文件或目录")
        parser.print_help()
        sys.exit(1)

    target_path = Path(args.target)

    if not target_path.exists():
        log_error(f"目标不存在: {target_path}")
        sys.exit(1)

    if args.dry_run:
        log_warning("🔍 预览模式 - 不会实际修改文件")

    # 显示规则
    tool.show_rules()

    # 开始迁移
    log_info("🚀 开始API模块迁移...")
    log_info(f"目标: {target_path}")
    print()

    if target_path.is_file():
        # 处理单个文件
        tool.process_file(target_path, args.dry_run)
        if not args.dry_run:
            tool.validate_migration(target_path)
    elif target_path.is_dir():
        # 处理目录
        tool.process_directory(target_path, args.dry_run)
    else:
        log_error(f"无效的目标类型: {target_path}")
        sys.exit(1)

    # 显示统计
    tool.show_statistics()

if __name__ == "__main__":
    main()