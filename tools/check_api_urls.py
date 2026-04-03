#!/usr/bin/env python3
"""
检查已实现 API 的调用方法与地址是否和 CSV 一致。

复用 tools/validate_apis.py 中的落盘映射逻辑，只对已实现文件做进一步校验：
- 解析实现文件中的 ApiRequest::<...>::get/post/put/patch/delete(...)
- 尽量展开字面量、端点常量、api_endpoints 枚举 to_url/path、format!、replace、字符串拼接
- 与 CSV 的 url 列（METHOD:/open-apis/...）逐项对比
"""

from __future__ import annotations

import argparse
import contextlib
import io
import json
import re
from collections import Counter, defaultdict
from dataclasses import dataclass
from importlib.util import module_from_spec, spec_from_file_location
from pathlib import Path
from typing import Dict, Iterable, List, Optional, Tuple

try:
    import tomllib  # py>=3.11
except ModuleNotFoundError:  # pragma: no cover
    tomllib = None


ROOT = Path(__file__).resolve().parent.parent
DEFAULT_CSV = ROOT / "api_list_export.csv"
DEFAULT_CONFIG = ROOT / "tools" / "api_coverage.toml"
DEFAULT_JSON = ROOT / "reports" / "api_url_validation" / "summary.json"
DEFAULT_MD = ROOT / "reports" / "api_url_validation" / "summary.md"


@dataclass
class ImplRecord:
    crate: str
    file_path: Path
    api: object


class EndpointIndex:
    def __init__(self, root: Path):
        self.root = root
        self.consts: Dict[str, str] = {}
        self.enum_routes: Dict[str, str] = {}

    def build(self) -> None:
        endpoint_files = set(self.root.glob("crates/openlark-*/src/**/api_endpoints.rs"))
        endpoint_files.update(self.root.glob("crates/openlark-*/src/endpoints/*.rs"))
        endpoint_files.update(self.root.glob("crates/openlark-*/src/endpoints/**/*.rs"))

        for path in sorted(endpoint_files):
            text = path.read_text(encoding="utf-8")
            self._load_consts(text)
            self._load_enum_routes(text)

        for _ in range(8):
            changed = False
            for key, value in list(self.consts.items()):
                if value in self.consts and self.consts[value] != value:
                    self.consts[key] = self.consts[value]
                    changed = True
            if not changed:
                break

    def _load_consts(self, text: str) -> None:
        literal_pat = re.compile(r'pub const\s+([A-Z0-9_]+)\s*:\s*&str\s*=\s*"([^"]+)";')
        alias_pat = re.compile(r"pub const\s+([A-Z0-9_]+)\s*:\s*&str\s*=\s*([A-Z0-9_]+);")
        for name, value in literal_pat.findall(text):
            self.consts[name] = value
        for name, alias in alias_pat.findall(text):
            self.consts.setdefault(name, alias)

    def _load_enum_routes(self, text: str) -> None:
        arm_pattern = re.compile(
            r"([A-Za-z0-9_]+::[A-Za-z0-9_]+)(?:\([^)]*\))?\s*=>\s*(?:\{\s*)?(?:format!\(\s*\"([^\"]+)\"|\"([^\"]+)\")",
            re.S,
        )
        for key, fmt_literal, literal in arm_pattern.findall(text):
            route = fmt_literal or literal
            if route.startswith("/open-apis/"):
                self.enum_routes[key] = route

        match_blocks = re.findall(
            r"impl\s+([A-Za-z0-9_]+)\s*\{.*?pub fn (?:to_url|path)\s*\([^)]*\)\s*(?:->\s*[^{]+)?\{(.*?)\n\s*\}",
            text,
            re.S,
        )
        for enum_name, body in match_blocks:
            match_match = re.search(r"match\s+self\s*\{(.*)\}", body, re.S)
            if not match_match:
                continue
            arms = match_match.group(1)
            for variant, expr in re.findall(rf"{enum_name}::([A-Za-z0-9_]+)(?:\([^)]*\))?\s*=>\s*(.*?)(?=,\s*{enum_name}::|\s*$)", arms, re.S):
                route = self._extract_route_expr(expr.strip())
                if route:
                    self.enum_routes[f"{enum_name}::{variant}"] = route

    def _extract_route_expr(self, expr: str) -> Optional[str]:
        expr = expr.strip().rstrip(",")
        if expr.startswith("{") and expr.endswith("}"):
            expr = expr[1:-1].strip()
        literal = re.search(r'"(/open-apis/[^"]*)"', expr)
        if literal:
            return literal.group(1)
        format_match = re.search(r'format!\(\s*"([^"]+)"', expr)
        if format_match:
            return format_match.group(1)
        return None


class ExprResolver:
    def __init__(self, endpoint_index: EndpointIndex):
        self.endpoint_index = endpoint_index

    def resolve(self, expr: str, compact: str) -> Optional[str]:
        if not expr:
            return None

        expr = self._strip_wrappers(expr.strip())

        if expr.startswith("self."):
            return ":{}"

        # format!("/open-apis/...", ...)
        format_value = self._resolve_format_expr(expr, compact)
        if format_value:
            return format_value

        replace_value = self._resolve_replace_chain(expr, compact)
        if replace_value:
            return replace_value

        # "a" + "b" + var
        concat_value = self._resolve_concat_expr(expr, compact)
        if concat_value:
            return concat_value

        # path/to_url
        route_value = self._resolve_route_call(expr, compact)
        if route_value:
            return route_value

        # 直接字面量
        literal = self._extract_first_open_api_literal(expr)
        if literal:
            return self._normalize_template(literal)

        # 常量
        if re.fullmatch(r"[A-Z0-9_]+", expr):
            const_value = self.endpoint_index.consts.get(expr)
            if const_value:
                return self._normalize_template(const_value)

        # 变量
        if re.fullmatch(r"[a-z_][A-Za-z0-9_]*", expr):
            assigned = self._resolve_variable(compact, expr)
            if assigned and assigned != expr:
                return self.resolve(assigned, compact)

        return None

    def _strip_wrappers(self, expr: str) -> str:
        while True:
            original = expr
            expr = expr.strip().rstrip(",").strip()
            if expr.startswith("&"):
                expr = expr[1:].strip()
            if expr.endswith(".to_string()"):
                expr = expr[:-12].strip()
            if expr.endswith(".clone()"):
                expr = expr[:-8].strip()
            if expr.startswith("(") and expr.endswith(")") and self._balanced(expr[1:-1]):
                expr = expr[1:-1].strip()
            if expr == original:
                return expr

    def _resolve_variable(self, compact: str, name: str) -> Optional[str]:
        patterns = [
            rf"let\s+mut\s+{re.escape(name)}\s*=\s*(.*?);",
            rf"let\s+{re.escape(name)}\s*=\s*(.*?);",
        ]
        for pattern in patterns:
            match = re.search(pattern, compact)
            if match:
                return match.group(1).strip()
        return None

    def _resolve_format_expr(self, expr: str, compact: str) -> Optional[str]:
        format_match = re.search(r"format!\((.*)\)$", expr)
        if format_match:
            inner = format_match.group(1).strip()
            parts = [part.strip() for part in self._split_top_level(inner, ",")]
            if parts:
                fmt_literal = self._extract_string_literal(parts[0])
                if fmt_literal is not None:
                    args = parts[1:]
                    rendered = fmt_literal
                    for arg in args:
                        replacement = self.resolve(arg, compact)
                        if replacement is None:
                            replacement = ":{}"
                        rendered = rendered.replace("{}", replacement, 1)
                    return self._normalize_template(rendered)

        # 变量 = format!(...)
        var_match = re.fullmatch(r"[a-z_][A-Za-z0-9_]*", expr)
        if var_match:
            assigned = self._resolve_variable(compact, expr)
            if assigned and "format!(" in assigned:
                return self._resolve_format_expr(assigned, compact)

        return None

    def _resolve_replace_chain(self, expr: str, compact: str) -> Optional[str]:
        if ".replace" not in expr:
            return None

        match = re.match(r"(.+?)(\s*(?:\.\s*replace\([^)]*\)\s*)+)$", expr)
        if not match:
            return None

        base_expr = match.group(1).strip()
        suffix = match.group(2)
        base_value = self.resolve(base_expr, compact)
        if not base_value:
            return None

        value = base_value
        for old, new in re.findall(r'\.\s*replace\(\s*"([^"]+)"\s*,\s*([^)]+)\)', suffix):
            replacement = self._resolve_replace_arg(new.strip(), compact)
            value = value.replace(old, replacement)
        return self._normalize_template(value)

    def _resolve_concat_expr(self, expr: str, compact: str) -> Optional[str]:
        if "+" not in expr:
            return None

        parts = self._split_top_level(expr, "+")
        if len(parts) <= 1:
            return None

        resolved_parts: List[str] = []
        for part in parts:
            part = part.strip()
            if not part:
                continue
            resolved = self.resolve(part, compact)
            if resolved:
                resolved_parts.append(resolved)
                continue

            # "/" 或其他静态字面量
            string_match = re.fullmatch(r'"([^"]*)"', self._strip_wrappers(part))
            if string_match:
                resolved_parts.append(string_match.group(1))
                continue

            return None

        return self._normalize_template("".join(resolved_parts))

    def _resolve_route_call(self, expr: str, compact: str) -> Optional[str]:
        route_match = re.search(r"(.+?)\.(to_url|path)\(\)$", expr)
        if route_match:
            target = route_match.group(1).strip()
            if re.fullmatch(r"[a-z_][A-Za-z0-9_]*", target):
                assigned = self._resolve_variable(compact, target)
                if assigned:
                    return self._resolve_route_call(assigned + f".{route_match.group(2)}()", compact)

            key = target.split("(", 1)[0].strip()
            route = self.endpoint_index.enum_routes.get(key)
            if route is None and "::" in key:
                parts = [part for part in key.split("::") if part]
                if len(parts) >= 2:
                    route = self.endpoint_index.enum_routes.get("::".join(parts[-2:]))
            if route:
                return self._normalize_template(route)

        return None

    def _extract_first_open_api_literal(self, expr: str) -> Optional[str]:
        match = re.search(r'"(/open-apis/[^"]*)"', expr)
        if match:
            return match.group(1)

        # 常量.replace(...).replace(...)
        replace_match = re.match(r"([A-Z0-9_]+)\s*(.*)?$", expr)
        if replace_match:
            const_name = replace_match.group(1)
            suffix = replace_match.group(2) or ""
            base = self.endpoint_index.consts.get(const_name)
            if base:
                value = base
                for old, new in re.findall(r'\.\s*replace\(\s*"([^"]+)"\s*,\s*([^)]+)\)', suffix):
                    replacement = self._resolve_replace_arg(new.strip(), expr)
                    value = value.replace(old, replacement)
                return value

        return None

    def _resolve_replace_arg(self, expr: str, compact: str) -> str:
        expr = self._strip_wrappers(expr)
        literal_match = re.fullmatch(r'"([^"]*)"', expr)
        if literal_match:
            return literal_match.group(1)
        if re.fullmatch(r"[a-z_][A-Za-z0-9_]*", expr):
            return "{" + expr + "}"
        if expr.startswith("self."):
            return "{" + expr.split(".")[-1] + "}"
        return "{}"

    def _normalize_template(self, path: str) -> str:
        path = path.strip()
        if "?" in path:
            path = path.split("?", 1)[0]
        path = re.sub(r"\{[^}]+\}", ":{}", path)
        path = path.replace("{}", ":{}")
        path = re.sub(r":[A-Za-z0-9_]+", ":{}", path)
        path = re.sub(r":+\{\}", ":{}", path)
        path = path.replace(":{}}", ":{}")
        path = path.replace(":{}}}", ":{}")
        path = re.sub(r"/:+", "/:", path)
        path = path.replace("//", "/")
        return path

    def _extract_string_literal(self, expr: str) -> Optional[str]:
        expr = self._strip_wrappers(expr)
        match = re.fullmatch(r'"([^"]*)"', expr)
        if match:
            return match.group(1)
        return None

    def _balanced(self, text: str) -> bool:
        depth = 0
        in_str = False
        escape = False
        for ch in text:
            if in_str:
                if escape:
                    escape = False
                elif ch == "\\":
                    escape = True
                elif ch == '"':
                    in_str = False
                continue
            if ch == '"':
                in_str = True
            elif ch == "(":
                depth += 1
            elif ch == ")":
                depth -= 1
                if depth < 0:
                    return False
        return depth == 0 and not in_str

    def _split_top_level(self, text: str, separator: str) -> List[str]:
        parts: List[str] = []
        buf: List[str] = []
        depth_paren = 0
        in_str = False
        escape = False
        for ch in text:
            if in_str:
                buf.append(ch)
                if escape:
                    escape = False
                elif ch == "\\":
                    escape = True
                elif ch == '"':
                    in_str = False
                continue
            if ch == '"':
                in_str = True
                buf.append(ch)
                continue
            if ch == "(":
                depth_paren += 1
            elif ch == ")":
                depth_paren -= 1
            if ch == separator and depth_paren == 0:
                parts.append("".join(buf))
                buf = []
            else:
                buf.append(ch)
        parts.append("".join(buf))
        return parts


def load_validator():
    spec = spec_from_file_location("validate_apis", ROOT / "tools" / "validate_apis.py")
    module = module_from_spec(spec)
    spec.loader.exec_module(module)
    return module.APIValidator


def parse_args():
    parser = argparse.ArgumentParser(description="检查已实现 API 的方法和地址是否与 CSV 一致")
    parser.add_argument("--csv", default=str(DEFAULT_CSV), help="CSV 文件路径")
    parser.add_argument("--config", default=str(DEFAULT_CONFIG), help="crate 映射配置")
    parser.add_argument("--crate", dest="crate_name", help="仅检查单个 crate")
    parser.add_argument("--json", default=str(DEFAULT_JSON), help="JSON 输出路径")
    parser.add_argument("--markdown", default=str(DEFAULT_MD), help="Markdown 输出路径")
    parser.add_argument("--limit", type=int, default=80, help="Markdown 中每个 crate 展示的最大问题数")
    return parser.parse_args()


def find_request(text: str) -> Tuple[Optional[str], Optional[str], str]:
    compact = re.sub(r"\s+", " ", text)
    match = re.search(r"ApiRequest(?:::<.+?>)?::(get|post|put|patch|delete)\(", compact)
    if not match:
        return None, None, compact
    method = match.group(1).upper()
    arg = parse_balanced_arg(compact, match.end())
    return method, arg, compact


def parse_balanced_arg(text: str, start: int) -> Optional[str]:
    depth = 1
    idx = start
    buf: List[str] = []
    in_str = False
    escape = False
    while idx < len(text):
        ch = text[idx]
        buf.append(ch)
        if in_str:
            if escape:
                escape = False
            elif ch == "\\":
                escape = True
            elif ch == '"':
                in_str = False
        else:
            if ch == '"':
                in_str = True
            elif ch == "(":
                depth += 1
            elif ch == ")":
                depth -= 1
                if depth == 0:
                    buf.pop()
                    return "".join(buf).strip()
        idx += 1
    return None


def collect_implemented_apis(
    validator_cls,
    csv_path: Path,
    config_path: Path,
    crate_name: Optional[str] = None,
) -> List[ImplRecord]:
    if tomllib is None:
        raise RuntimeError("当前 Python 不支持 tomllib，请使用 Python 3.11+")

    config = tomllib.loads(config_path.read_text(encoding="utf-8"))
    crates = config["crates"]
    if crate_name:
        if crate_name not in crates:
            raise KeyError(f"未找到 crate: {crate_name}")
        crates = {crate_name: crates[crate_name]}

    records: List[ImplRecord] = []
    for crate, info in crates.items():
        validator = validator_cls(str(csv_path), str(ROOT / info["src"]), info["biz_tags"])
        with contextlib.redirect_stdout(io.StringIO()):
            validator.parse_csv()
            validator.scan_implementations()
            validator.compare()

        implemented = {
            api.expected_file: api for api in validator.apis if api.is_implemented
        }
        for rel in sorted(validator.implemented_files):
            api = implemented.get(rel)
            if api:
                records.append(ImplRecord(crate=crate, file_path=ROOT / info["src"] / rel, api=api))

    return records


def compare(records: Iterable[ImplRecord], resolver: ExprResolver) -> Dict[str, object]:
    resolved = []
    unresolved = []

    for record in records:
        text = record.file_path.read_text(encoding="utf-8")
        actual_method, arg, compact = find_request(text)
        actual_path = resolver.resolve(arg, compact) if arg else None

        if not actual_method or not actual_path:
            comment_match = re.search(r"url:\s*(GET|POST|PUT|PATCH|DELETE):([^\s]+)", text)
            if comment_match:
                actual_method = comment_match.group(1)
                actual_path = resolver._normalize_template(comment_match.group(2))

        expected_method, expected_path = record.api.url.split(":", 1)
        item = {
            "crate": record.crate,
            "file": str(record.file_path.relative_to(ROOT)),
            "name": record.api.name,
            "expected_method": expected_method,
            "expected_path": resolver._normalize_template(expected_path),
            "actual_method": actual_method,
            "actual_path": actual_path,
            "csv_url": record.api.url,
            "doc_path": record.api.doc_path,
        }

        if not actual_method or not actual_path:
            unresolved.append(item)
            continue

        item["match"] = (
            item["expected_method"] == item["actual_method"]
            and item["expected_path"] == item["actual_path"]
        )
        resolved.append(item)

    mismatches = [item for item in resolved if not item["match"]]

    return {
        "summary": {
            "implemented_total": len(list(records)) if not isinstance(records, list) else len(records),
            "resolved_total": len(resolved),
            "unresolved_total": len(unresolved),
            "matched_total": len(resolved) - len(mismatches),
            "mismatched_total": len(mismatches),
        },
        "mismatches": mismatches,
        "unresolved": unresolved,
    }


def build_markdown(result: Dict[str, object], limit: int) -> str:
    summary = result["summary"]
    mismatches = result["mismatches"]
    unresolved = result["unresolved"]

    lines = [
        "# API 地址校验报告",
        "",
        "## 总体统计",
        "",
        "| 指标 | 数量 |",
        "|---|---:|",
        f"| 已实现文件 | {summary['implemented_total']} |",
        f"| 成功解析 | {summary['resolved_total']} |",
        f"| 一致 | {summary['matched_total']} |",
        f"| 不一致 | {summary['mismatched_total']} |",
        f"| 未解析 | {summary['unresolved_total']} |",
        "",
        "## 不一致分布",
        "",
    ]

    mismatch_counter = Counter(item["crate"] for item in mismatches)
    unresolved_counter = Counter(item["crate"] for item in unresolved)
    lines.append("| crate | 不一致 | 未解析 |")
    lines.append("|---|---:|---:|")
    for crate in sorted(set(mismatch_counter) | set(unresolved_counter)):
        lines.append(f"| {crate} | {mismatch_counter[crate]} | {unresolved_counter[crate]} |")

    by_crate: Dict[str, List[dict]] = defaultdict(list)
    for item in mismatches:
        by_crate[item["crate"]].append(item)

    for crate in sorted(by_crate):
        lines.extend(["", f"## {crate}", ""])
        for item in by_crate[crate][:limit]:
            lines.append(f"- `{item['file']}`")
            lines.append(f"  期望: `{item['expected_method']}:{item['expected_path']}`")
            lines.append(f"  实际: `{item['actual_method']}:{item['actual_path']}`")
        remain = len(by_crate[crate]) - limit
        if remain > 0:
            lines.append(f"- 其余 {remain} 项已省略")

    return "\n".join(lines) + "\n"


def main():
    args = parse_args()
    validator_cls = load_validator()

    records = collect_implemented_apis(
        validator_cls,
        csv_path=Path(args.csv),
        config_path=Path(args.config),
        crate_name=args.crate_name,
    )

    endpoint_index = EndpointIndex(ROOT)
    endpoint_index.build()
    resolver = ExprResolver(endpoint_index)
    result = compare(records, resolver)

    json_path = Path(args.json)
    md_path = Path(args.markdown)
    json_path.parent.mkdir(parents=True, exist_ok=True)
    md_path.parent.mkdir(parents=True, exist_ok=True)

    json_path.write_text(json.dumps(result, ensure_ascii=False, indent=2), encoding="utf-8")
    md_path.write_text(build_markdown(result, args.limit), encoding="utf-8")

    print(json.dumps(result["summary"], ensure_ascii=False, indent=2))
    print(f"JSON: {json_path}")
    print(f"Markdown: {md_path}")


if __name__ == "__main__":
    main()
