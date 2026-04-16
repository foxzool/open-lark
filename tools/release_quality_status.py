#!/usr/bin/env python3
"""
生成发布说明里的 crate 级质量状态摘要。

输入：
- reports/api_validation/summary.json

输出：
- Markdown 片段，可直接附加到 release notes / GitHub Release body
"""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any, Dict, List

try:
    import tomllib
except ModuleNotFoundError:  # pragma: no cover
    tomllib = None


def load_workspace_crates(repo_root: Path) -> List[Dict[str, str]]:
    if tomllib is None:
        raise RuntimeError("需要 Python 3.11+ 以支持 tomllib")

    root_manifest = tomllib.loads((repo_root / "Cargo.toml").read_text(encoding="utf-8"))
    crates: List[Dict[str, str]] = []

    root_package = root_manifest.get("package")
    if isinstance(root_package, dict):
        crates.append({"name": root_package["name"], "path": "."})

    members = root_manifest.get("workspace", {}).get("members", [])
    for member in members:
        manifest_path = repo_root / member / "Cargo.toml"
        manifest = tomllib.loads(manifest_path.read_text(encoding="utf-8"))
        package = manifest.get("package", {})
        crate_name = package.get("name")
        if crate_name:
            crates.append({"name": crate_name, "path": member})

    return crates


def has_contract_tests(crate_dir: Path) -> bool:
    tests_dir = crate_dir / "tests"
    return tests_dir.exists() and any("contract" in path.name for path in tests_dir.rglob("*.rs"))


def has_snapshot_tests(crate_dir: Path) -> bool:
    tests_dir = crate_dir / "tests"
    if not tests_dir.exists():
        return False
    return any("snapshot" in path.name for path in tests_dir.rglob("*.rs")) or any(
        path.suffix == ".snap" for path in (tests_dir / "snapshots").glob("*.snap")
    )


def format_quality_note(crate_name: str, summary: Dict[str, Any] | None) -> str:
    if summary is None:
        return "非 typed API 覆盖统计对象"

    missing = summary.get("missing", 0)
    if missing == 0:
        return "无 typed API 缺口"

    priority_counts = summary.get("priority_counts", {})
    if priority_counts:
        parts = [f"{priority}={count}" for priority, count in sorted(priority_counts.items())]
        return "缺口：" + ", ".join(parts)

    return f"剩余 {missing} 个缺口"


def render_markdown(crate_rows: List[Dict[str, Any]]) -> str:
    lines = [
        "## Crate 级质量状态",
        "",
        "质量状态维度：",
        "",
        "- **typed coverage**：来自 `reports/api_validation/summary.json` 的 typed API 覆盖率",
        "- **contract tests**：crate 内存在 request/response contract test",
        "- **snapshot tests**：crate 内存在 helper/output snapshot test",
        "",
        "| crate | typed coverage | 剩余缺口 | contract tests | snapshot tests | 备注 |",
        "|------|----------------|----------|----------------|----------------|------|",
    ]

    for row in crate_rows:
        lines.append(
            f"| `{row['crate']}` | {row['typed_coverage']} | {row['missing']} | "
            f"{row['contract_tests']} | {row['snapshot_tests']} | {row['note']} |"
        )

    lines.append("")
    return "\n".join(lines)


def build_rows(repo_root: Path, summary_data: Dict[str, Any]) -> List[Dict[str, Any]]:
    summary_crates = summary_data.get("crates", {})
    rows: List[Dict[str, Any]] = []

    for crate in load_workspace_crates(repo_root):
        crate_name = crate["name"]
        crate_dir = repo_root / crate["path"]
        summary = summary_crates.get(crate_name)
        rows.append(
            {
                "crate": crate_name,
                "typed_coverage": (
                    f"{summary['completion_rate']:.1f}%"
                    if summary is not None and "completion_rate" in summary
                    else "n/a"
                ),
                "missing": summary.get("missing", "n/a") if summary is not None else "n/a",
                "contract_tests": "yes" if has_contract_tests(crate_dir) else "no",
                "snapshot_tests": "yes" if has_snapshot_tests(crate_dir) else "no",
                "note": format_quality_note(crate_name, summary),
            }
        )

    return rows


def main() -> int:
    parser = argparse.ArgumentParser(description="生成 crate 级质量状态 Markdown")
    parser.add_argument(
        "--summary",
        default="reports/api_validation/summary.json",
        help="typed coverage 汇总 JSON 路径",
    )
    parser.add_argument(
        "--repo-root",
        default=".",
        help="仓库根目录（默认当前目录）",
    )
    parser.add_argument(
        "--output",
        default="",
        help="输出文件路径；为空时打印到 stdout",
    )
    args = parser.parse_args()

    repo_root = Path(args.repo_root).resolve()
    summary_path = repo_root / args.summary
    summary_data = json.loads(summary_path.read_text(encoding="utf-8"))
    markdown = render_markdown(build_rows(repo_root, summary_data))

    if args.output:
        output_path = repo_root / args.output
        output_path.write_text(markdown + "\n", encoding="utf-8")
    else:
        print(markdown)

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
