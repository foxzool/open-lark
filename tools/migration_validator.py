#!/usr/bin/env python3
"""
Open-Lark 迁移验证工具

验证模块化迁移的完整性和一致性。
"""

import os
import json
import re
from pathlib import Path
from typing import Dict, List, Set, Tuple, Any, Optional
from dataclasses import dataclass
import argparse


@dataclass
class ValidationResult:
    """验证结果"""
    is_valid: bool
    issues: List[str]
    warnings: List[str]
    stats: Dict[str, Any]


class MigrationValidator:
    """迁移验证器"""

    def __init__(self, project_root: str):
        """初始化验证器

        Args:
            project_root: 项目根目录路径
        """
        self.project_root = Path(project_root)
        self.crates_dir = self.project_root / "crates"
        self.issues = []
        self.warnings = []
        self.stats = {}

    def validate_crate_structure(self) -> None:
        """验证Crate结构"""
        print("🔍 验证Crate结构...")

        # 检查必需的核心crate
        required_crates = [
            "openlark-core",
            "openlark-client",
            "openlark-protocol"
        ]

        for crate_name in required_crates:
            crate_path = self.crates_dir / crate_name
            if not crate_path.exists():
                self.issues.append(f"缺少必需的核心crate: {crate_name}")
            else:
                self._validate_crate_basic_structure(crate_path)

        # 检查业务crate
        expected_business_crates = [
            "openlark-hr",
            "openlark-communication",
            "openlark-docs",
            "openlark-workflow",
            "openlark-meeting",
            "openlark-mail",
            "openlark-platform",
            "openlark-ai",
            "openlark-security",
            "openlark-analytics",
            "openlark-helpdesk",
            "openlark-user"
        ]

        existing_crates = []
        for crate_name in expected_business_crates:
            crate_path = self.crates_dir / crate_name
            if crate_path.exists():
                existing_crates.append(crate_name)
                self._validate_crate_basic_structure(crate_path)

        self.stats["expected_business_crates"] = len(expected_business_crates)
        self.stats["existing_business_crates"] = len(existing_crates)

        print(f"   ✅ 找到 {len(existing_crates)}/{len(expected_business_crates)} 个业务crate")

    def _validate_crate_basic_structure(self, crate_path: Path) -> None:
        """验证单个crate的基本结构"""
        crate_name = crate_path.name

        # 检查必需文件
        required_files = ["Cargo.toml", "src/lib.rs"]
        for file_name in required_files:
            file_path = crate_path / file_name
            if not file_path.exists():
                self.issues.append(f"{crate_name}: 缺少必需文件 {file_name}")

        # 检查src目录结构
        src_dir = crate_path / "src"
        if src_dir.exists():
            # 检查是否有过时的src目录结构
            old_patterns = ["src/v1", "src/v2", "src/v3"]
            for pattern in old_patterns:
                if (src_dir / pattern).exists():
                    self.warnings.append(f"{crate_name}: 发现可能过时的目录结构 {pattern}")

    def validate_workspace_config(self) -> None:
        """验证工作空间配置"""
        print("🔍 验证工作空间配置...")

        cargo_toml = self.project_root / "Cargo.toml"
        if not cargo_toml.exists():
            self.issues.append("缺少根目录 Cargo.toml")
            return

        with open(cargo_toml, 'r', encoding='utf-8') as f:
            content = f.read()

        # 检查workspace members
        members_match = re.search(r'members\s*=\s*\[(.*?)\]', content, re.DOTALL)
        if members_match:
            members_text = members_match.group(1)
            # 提取crate名称
            crate_matches = re.findall(r'"([^"]+)"', members_text)
            self.stats["workspace_members"] = len(crate_matches)

            # 验证每个member目录是否存在
            for member in crate_matches:
                member_path = self.project_root / member
                if not member_path.exists():
                    self.issues.append(f"工作空间成员目录不存在: {member}")

        # 检查feature配置
        features_match = re.search(r'\[features\].*?(?=\n\[|\Z)', content, re.DOTALL)
        if features_match:
            features_text = features_match.group(0)
            # 检查是否包含预期的feature
            expected_features = [
                "default", "minimal", "full", "core", "client", "protocol",
                "communication", "docs", "hr", "workflow", "meeting",
                "mail", "platform", "ai", "security", "analytics", "helpdesk", "user"
            ]

            missing_features = []
            for feature in expected_features:
                if feature not in features_text:
                    missing_features.append(feature)

            if missing_features:
                self.warnings.append(f"可能缺少的feature配置: {', '.join(missing_features)}")

    def validate_api_coverage(self) -> None:
        """验证API覆盖率"""
        print("🔍 验证API覆盖率...")

        # 检查是否有API分类结果
        classification_file = self.project_root / "tools" / "output" / "classification.json"
        if not classification_file.exists():
            self.warnings.append("没有找到API分类结果，请先运行 classify_apis.py")
            return

        try:
            with open(classification_file, 'r', encoding='utf-8') as f:
                classification = json.load(f)

            total_apis = classification.get("total_apis", 0)
            suggested_modules = classification.get("suggested_modules", {})

            if total_apis > 0:
                covered_apis = sum(m.get("api_count", 0) for m in suggested_modules.values())
                coverage_rate = covered_apis / total_apis * 100

                self.stats["total_apis"] = total_apis
                self.stats["covered_apis"] = covered_apis
                self.stats["coverage_rate"] = coverage_rate

                if coverage_rate < 90:
                    self.warnings.append(f"API覆盖率较低: {coverage_rate:.1f}%")

        except Exception as e:
            self.warnings.append(f"读取API分类结果失败: {e}")

    def validate_naming_conventions(self) -> None:
        """验证命名规范"""
        print("🔍 验证命名规范...")

        # 验证crate命名
        for crate_path in self.crates_dir.iterdir():
            if not crate_path.is_dir() or not crate_path.name.startswith("openlark-"):
                continue

            crate_name = crate_path.name

            # 检查命名规范
            if not re.match(r'^openlark-[a-z]+(-[a-z]+)*$', crate_name):
                self.issues.append(f"crate命名不符合规范: {crate_name}")

            # 检查Cargo.toml中的命名
            cargo_toml = crate_path / "Cargo.toml"
            if cargo_toml.exists():
                with open(cargo_toml, 'r', encoding='utf-8') as f:
                    content = f.read()

                # 检查包名
                name_match = re.search(r'name\s*=\s*"([^"]+)"', content)
                if name_match:
                    package_name = name_match.group(1)
                    if package_name != crate_name:
                        self.issues.append(f"{crate_name}: 包名与目录名不匹配")

    def validate_documentation(self) -> None:
        """验证文档完整性"""
        print("🔍 验证文档完整性...")

        # 检查架构文档
        required_docs = [
            "ARCHITECTURE.md",
            "docs/design-guide.md",
            "docs/module-mapping.md"
        ]

        for doc_path in required_docs:
            full_path = self.project_root / doc_path
            if not full_path.exists():
                self.issues.append(f"缺少重要文档: {doc_path}")

        # 检查crate级别的README
        readme_count = 0
        for crate_path in self.crates_dir.iterdir():
            if not crate_path.is_dir():
                continue

            readme_file = crate_path / "README.md"
            if readme_file.exists():
                readme_count += 1
            else:
                self.warnings.append(f"{crate_path.name}: 缺少README.md")

        self.stats["crates_with_readme"] = readme_count

    def validate_dependencies(self) -> None:
        """验证依赖关系"""
        print("🔍 验证依赖关系...")

        # 检查循环依赖（简单检查）
        dependency_graph = {}

        for crate_path in self.crates_dir.iterdir():
            if not crate_path.is_dir() or not crate_path.name.startswith("openlark-"):
                continue

            crate_name = crate_path.name
            cargo_toml = crate_path / "Cargo.toml"

            if cargo_toml.exists():
                with open(cargo_toml, 'r', encoding='utf-8') as f:
                    content = f.read()

                # 提取依赖
                deps = []
                dep_matches = re.findall(r'openlark-[a-z-]+\s*=\s*\{[^}]*path\s*=\s*"([^"]+)"', content)
                for dep_path in dep_matches:
                    dep_name = Path(dep_path).name
                    deps.append(dep_name)

                dependency_graph[crate_name] = deps

        # 简单的循环依赖检测
        for crate, deps in dependency_graph.items():
            if crate in deps:
                self.issues.append(f"{crate}: 存在自依赖")

        self.stats["dependency_graph"] = dependency_graph

    def validate_tests(self) -> None:
        """验证测试覆盖"""
        print("🔍 验证测试覆盖...")

        test_crates = 0
        total_crates = 0

        for crate_path in self.crates_dir.iterdir():
            if not crate_path.is_dir() or not crate_path.name.startswith("openlark-"):
                continue

            total_crates += 1

            # 检查是否有测试目录或文件
            has_tests = False

            # 检查tests目录
            tests_dir = crate_path / "tests"
            if tests_dir.exists() and any(tests_dir.iterdir()):
                has_tests = True

            # 检查src目录中的测试文件
            if not has_tests:
                for rust_file in crate_path.rglob("*.rs"):
                    if "test" in rust_file.name or rust_file.name.endswith("_test.rs"):
                        has_tests = True
                        break

            if has_tests:
                test_crates += 1
            else:
                self.warnings.append(f"{crate_path.name}: 缺少测试文件")

        self.stats["total_crates"] = total_crates
        self.stats["crates_with_tests"] = test_crates

        if total_crates > 0:
            test_coverage = test_crates / total_crates * 100
            self.stats["test_coverage"] = test_coverage

            if test_coverage < 70:
                self.warnings.append(f"测试覆盖率较低: {test_coverage:.1f}%")

    def run_validation(self) -> ValidationResult:
        """运行完整验证"""
        print("🚀 开始Open-Lark迁移验证...")
        print("=" * 50)

        # 执行各项验证
        self.validate_crate_structure()
        self.validate_workspace_config()
        self.validate_api_coverage()
        self.validate_naming_conventions()
        self.validate_documentation()
        self.validate_dependencies()
        self.validate_tests()

        print("=" * 50)

        # 汇总结果
        is_valid = len(self.issues) == 0

        # 输出结果
        if is_valid:
            print("✅ 迁移验证通过！")
        else:
            print(f"❌ 发现 {len(self.issues)} 个问题，{len(self.warnings)} 个警告")

        if self.issues:
            print("\n🚨 问题:")
            for i, issue in enumerate(self.issues, 1):
                print(f"   {i}. {issue}")

        if self.warnings:
            print("\n⚠️ 警告:")
            for i, warning in enumerate(self.warnings, 1):
                print(f"   {i}. {warning}")

        # 输出统计信息
        print(f"\n📊 统计信息:")
        for key, value in self.stats.items():
            if isinstance(value, float):
                print(f"   {key}: {value:.1f}")
            else:
                print(f"   {key}: {value}")

        return ValidationResult(
            is_valid=is_valid,
            issues=self.issues,
            warnings=self.warnings,
            stats=self.stats
        )

    def save_report(self, result: ValidationResult, output_file: str = None) -> None:
        """保存验证报告"""
        if output_file is None:
            output_file = self.project_root / "tools" / "output" / "migration_validation_report.json"

        output_path = Path(output_file)
        output_path.parent.mkdir(exist_ok=True)

        report = {
            "timestamp": self._get_current_time(),
            "is_valid": result.is_valid,
            "issues": result.issues,
            "warnings": result.warnings,
            "stats": result.stats,
            "summary": {
                "total_issues": len(result.issues),
                "total_warnings": len(result.warnings),
                "validation_passed": result.is_valid
            }
        }

        with open(output_path, 'w', encoding='utf-8') as f:
            json.dump(report, f, ensure_ascii=False, indent=2)

        print(f"\n📄 验证报告已保存到: {output_path}")

    def _get_current_time(self) -> str:
        """获取当前时间"""
        from datetime import datetime
        return datetime.now().strftime("%Y-%m-%d %H:%M:%S")


def main():
    """主函数"""
    parser = argparse.ArgumentParser(description="Open-Lark 迁移验证工具")
    parser.add_argument(
        "project_root",
        nargs="?",
        default=".",
        help="项目根目录路径"
    )
    parser.add_argument(
        "--output",
        "-o",
        help="输出报告文件路径"
    )
    parser.add_argument(
        "--verbose",
        "-v",
        action="store_true",
        help="详细输出"
    )

    args = parser.parse_args()

    # 检查项目根目录
    project_root = Path(args.project_root)
    if not (project_root / "Cargo.toml").exists():
        print(f"❌ 指定的目录不是有效的Rust项目: {project_root}")
        return 1

    # 创建验证器
    validator = MigrationValidator(args.project_root)

    try:
        # 运行验证
        result = validator.run_validation()

        # 保存报告
        validator.save_report(result, args.output)

        # 返回退出码
        return 0 if result.is_valid else 1

    except Exception as e:
        print(f"❌ 验证失败: {e}")
        return 1


if __name__ == "__main__":
    exit(main())