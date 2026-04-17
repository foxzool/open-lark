#!/usr/bin/env python3
"""Audit TODO/FIXME comments across the working tree and summarize tracking buckets."""

from __future__ import annotations

import argparse
import json
import re
from collections import Counter, defaultdict
from dataclasses import asdict, dataclass
from pathlib import Path
from typing import Iterable

ROOT = Path(__file__).resolve().parents[1]
SCAN_ROOTS = ("crates", "tests", "examples", "src", "tools")
EXCLUDE_PARTS = {"target", ".git", ".omx", "reports", ".serena", ".agents"}
EXCLUDE_FILES = {"audit_todos.py", "test_audit_todos.py"}
TODO_PATTERN = re.compile(r"\b(TODO|FIXME)\b")


@dataclass(frozen=True)
class TodoEntry:
    path: str
    line: int
    text: str
    category: str
    priority: str
    rationale: str


CATEGORY_META = {
    "websocket_test_placeholders": {
        "priority": "p1",
        "rationale": "Large WebSocket integration test files are still placeholders and hide real coverage gaps.",
    },
    "contact_test_placeholders": {
        "priority": "p1",
        "rationale": "Contact test suites contain many TODO placeholders and block trustworthy behavior coverage.",
    },
    "hr_hire_generated_stubs": {
        "priority": "p1",
        "rationale": "Generated HR hire APIs still contain TODO field / implementation stubs inside shipped source.",
    },
    "source_api_stubs": {
        "priority": "p1",
        "rationale": "User/platform/analytics source files still advertise TODO API implementations in runtime code.",
    },
    "internal_tooling": {
        "priority": "p3",
        "rationale": "Tooling TODOs are useful to keep, but they are not directly user-facing runtime debt.",
    },
    "other": {
        "priority": "p2",
        "rationale": "Remaining TODO/FIXME comments should stay visible until explicitly triaged.",
    },
}


def classify(path: str) -> tuple[str, str, str]:
    if path.startswith("tests/unit/websocket/"):
        category = "websocket_test_placeholders"
    elif path.startswith("tests/unit/contact/"):
        category = "contact_test_placeholders"
    elif path.startswith("crates/openlark-hr/src/hire/"):
        category = "hr_hire_generated_stubs"
    elif path.startswith("crates/openlark-user/src/") or path.startswith(
        "crates/openlark-platform/src/"
    ) or path.startswith("crates/openlark-analytics/src/"):
        category = "source_api_stubs"
    elif path.startswith("tools/"):
        category = "internal_tooling"
    else:
        category = "other"

    meta = CATEGORY_META[category]
    return category, meta["priority"], meta["rationale"]


def iter_source_files(root: Path) -> Iterable[Path]:
    for scan_root in SCAN_ROOTS:
        base = root / scan_root
        if not base.exists():
            continue
        for path in base.rglob("*"):
            if path.is_dir():
                continue
            if any(part in EXCLUDE_PARTS for part in path.parts):
                continue
            if "__pycache__" in path.parts:
                continue
            if path.name in EXCLUDE_FILES or path.suffix == ".pyc":
                continue
            yield path


def collect_entries(root: Path) -> list[TodoEntry]:
    entries: list[TodoEntry] = []
    for path in iter_source_files(root):
        rel = path.relative_to(root).as_posix()
        try:
            lines = path.read_text(encoding="utf-8", errors="ignore").splitlines()
        except OSError:
            continue
        for line_no, line in enumerate(lines, 1):
            if not TODO_PATTERN.search(line):
                continue
            category, priority, rationale = classify(rel)
            entries.append(
                TodoEntry(
                    path=rel,
                    line=line_no,
                    text=line.strip(),
                    category=category,
                    priority=priority,
                    rationale=rationale,
                )
            )
    return entries


def build_summary(entries: list[TodoEntry]) -> dict:
    by_category: dict[str, list[TodoEntry]] = defaultdict(list)
    for entry in entries:
        by_category[entry.category].append(entry)

    top_files = Counter(entry.path for entry in entries).most_common(20)
    categories = []
    for category, items in sorted(
        by_category.items(),
        key=lambda kv: (-len(kv[1]), kv[0]),
    ):
        categories.append(
            {
                "category": category,
                "priority": CATEGORY_META[category]["priority"],
                "count": len(items),
                "rationale": CATEGORY_META[category]["rationale"],
                "sample_paths": sorted({entry.path for entry in items})[:10],
            }
        )

    return {
        "total": len(entries),
        "categories": categories,
        "top_files": [{"path": path, "count": count} for path, count in top_files],
        "sample_entries": [asdict(entry) for entry in entries[:80]],
    }


def render_markdown(summary: dict) -> str:
    lines = [
        "# TODO / FIXME Audit Summary",
        "",
        f"- Total TODO/FIXME matches in tracked source roots: **{summary['total']}**",
        "- Scan roots: `crates`, `tests`, `examples`, `src`, `tools`",
        "- Excluded: `target`, `.git`, `.omx`, `reports`, `.serena`, `.agents`",
        "",
        "## Category summary",
        "",
        "| Category | Priority | Count | Why it matters |",
        "| --- | --- | ---: | --- |",
    ]

    for item in summary["categories"]:
        lines.append(
            f"| `{item['category']}` | `{item['priority']}` | {item['count']} | {item['rationale']} |"
        )

    lines.extend(
        [
            "",
            "## Top files",
            "",
            "| Count | File |",
            "| ---: | --- |",
        ]
    )
    for item in summary["top_files"]:
        lines.append(f"| {item['count']} | `{item['path']}` |")

    lines.extend(
        [
            "",
            "## Recommended tracking split",
            "",
            "- `websocket_test_placeholders`: follow-up issue for executable WebSocket integration tests",
            "- `contact_test_placeholders`: follow-up issue for executable contact test coverage",
            "- `hr_hire_generated_stubs`: follow-up issue for generated HR hire source stubs",
            "- `source_api_stubs`: follow-up issue for user/platform/analytics runtime TODO implementations",
            "- `internal_tooling` / `other`: keep visible unless they block release or API trust",
        ]
    )
    return "\n".join(lines) + "\n"


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output-json", type=Path, default=None)
    parser.add_argument("--output-md", type=Path, default=None)
    args = parser.parse_args()

    entries = collect_entries(ROOT)
    summary = build_summary(entries)

    if args.output_json:
        args.output_json.parent.mkdir(parents=True, exist_ok=True)
        args.output_json.write_text(
            json.dumps(summary, ensure_ascii=False, indent=2) + "\n",
            encoding="utf-8",
        )
    if args.output_md:
        args.output_md.parent.mkdir(parents=True, exist_ok=True)
        args.output_md.write_text(render_markdown(summary), encoding="utf-8")

    print(json.dumps(summary, ensure_ascii=False, indent=2))


if __name__ == "__main__":
    main()
