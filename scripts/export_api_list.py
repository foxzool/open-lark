#!/usr/bin/env python3
"""Export `api_list.json` entries to CSV, keeping all payload fields."""

from __future__ import annotations

import argparse
import csv
import json
from pathlib import Path
from typing import Dict, Iterable, List, Sequence, Tuple


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Convert api_list.json to a CSV summary."
    )
    parser.add_argument(
        "--input",
        default="api_list.json",
        help="Path to api_list.json (default: %(default)s)",
    )
    parser.add_argument(
        "--output",
        default="api_list_export.csv",
        help="Destination CSV path (default: %(default)s)",
    )
    # no extra arguments needed currently, but keep hook for future options
    return parser.parse_args()


def normalize_value(value) -> str:
    if value is None:
        return ""
    if isinstance(value, bool):
        return "true" if value else "false"
    if isinstance(value, (int, float)):
        return str(value)
    if isinstance(value, (list, dict)):
        return json.dumps(value, ensure_ascii=False)
    return str(value)


def collect_fields(entries: Sequence[dict]) -> Tuple[List[str], List[str]]:
    top_keys = set()
    meta_keys = set()
    for entry in entries:
        for key, value in entry.items():
            if key == "meta" and isinstance(value, dict):
                meta_keys.update(value.keys())
            elif key != "meta":
                top_keys.add(key)

    preferred = [
        "id",
        "name",
        "bizTag",
        "chargingMethod",
        "detail",
        "fullDose",
        "fullPath",
        "url",
        "orderMark",
        "supportAppTypes",
        "tags",
        "updateTime",
    ]

    ordered_top = []
    seen = set()
    for key in preferred:
        if key in top_keys and key not in seen:
            ordered_top.append(key)
            top_keys.remove(key)
            seen.add(key)
    ordered_top.extend(sorted(top_keys))

    ordered_meta = sorted(meta_keys)
    return ordered_top, ordered_meta


def iter_rows(entries: Iterable[dict], top_fields: Sequence[str], meta_fields: Sequence[str]):
    for item in entries:
        row: Dict[str, str] = {}
        meta = item.get("meta") or {}
        for field in top_fields:
            row[field] = normalize_value(item.get(field))
        for meta_key in meta_fields:
            row[f"meta.{meta_key}"] = normalize_value(meta.get(meta_key))
        yield row


def main() -> None:
    args = parse_args()
    source_path = Path(args.input)
    output_path = Path(args.output)

    with source_path.open("r", encoding="utf-8") as handle:
        payload = json.load(handle)

    entries = (payload.get("data") or {}).get("apis") or []
    if not entries:
        raise SystemExit("未在 JSON 中找到 data.apis 列表")

    output_path.parent.mkdir(parents=True, exist_ok=True)

    top_fields, meta_fields = collect_fields(entries)
    fieldnames = list(top_fields) + [f"meta.{k}" for k in meta_fields]

    with output_path.open("w", encoding="utf-8", newline="") as csv_file:
        writer = csv.DictWriter(csv_file, fieldnames=fieldnames)
        writer.writeheader()
        for row in iter_rows(entries, top_fields, meta_fields):
            writer.writerow(row)

    print(f"已导出 {len(entries)} 条接口到 {output_path}")


if __name__ == "__main__":
    main()
