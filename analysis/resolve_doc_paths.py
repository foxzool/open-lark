#!/usr/bin/env python3
"""
Resolve final documentation URLs for API entries.

The script reads `analysis/data/api_list_export.csv`, opens each entry's
`fullPath` on https://open.feishu.cn via Playwright, waits for any JS
redirects to settle, and writes the final URL into a `docPath` column in
the same CSV. Run `python -m playwright install chromium` once before the
first execution.
"""

from __future__ import annotations

import argparse
import asyncio
import csv
from pathlib import Path
from typing import List, Tuple

from playwright.async_api import async_playwright, TimeoutError as PlaywrightTimeoutError


BASE_URL = "https://open.feishu.cn"
CSV_DEFAULT = Path(__file__).parent / "data" / "api_list_export.csv"
DEFAULT_TIMEOUT_MS = 20_000


def load_rows(csv_path: Path) -> Tuple[List[str], List[dict]]:
    with csv_path.open("r", encoding="utf-8", newline="") as f:
        reader = csv.DictReader(f)
        fieldnames = reader.fieldnames or []
        rows = list(reader)
    return fieldnames, rows


def persist_rows(csv_path: Path, fieldnames: List[str], rows: List[dict]) -> None:
    if "docPath" not in fieldnames:
        fieldnames.append("docPath")

    with csv_path.open("w", encoding="utf-8", newline="") as f:
        writer = csv.DictWriter(f, fieldnames=fieldnames)
        writer.writeheader()
        writer.writerows(rows)


async def resolve_final_url(page, target_url: str, timeout_ms: int) -> str:
    try:
        await page.goto(target_url, wait_until="domcontentloaded", timeout=timeout_ms)
        await page.wait_for_load_state("networkidle", timeout=timeout_ms)
        await page.wait_for_timeout(500)
        return page.url
    except PlaywrightTimeoutError:
        return ""
    except Exception:
        return ""


async def fill_doc_paths(
    csv_path: Path, skip_existing: bool, timeout_ms: int, headless: bool
) -> None:
    fieldnames, rows = load_rows(csv_path)
    if "docPath" not in fieldnames:
        fieldnames.append("docPath")

    async with async_playwright() as p:
        browser = await p.chromium.launch(headless=headless)
        page = await browser.new_page()

        save_every = 50
        for idx, row in enumerate(rows, start=1):
            if skip_existing and row.get("docPath"):
                continue

            full_path = (row.get("fullPath") or "").strip()
            if not full_path:
                row["docPath"] = ""
                continue

            target_url = full_path if full_path.startswith("http") else f"{BASE_URL}{full_path}"
            final_url = await resolve_final_url(page, target_url, timeout_ms)
            row["docPath"] = final_url
            print(f"[{idx}/{len(rows)}] {full_path} -> {final_url}")

            if idx % save_every == 0:
                persist_rows(csv_path, fieldnames, rows)

        await browser.close()

    persist_rows(csv_path, fieldnames, rows)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Populate docPath column via Playwright.")
    parser.add_argument("--csv", type=Path, default=CSV_DEFAULT, help="CSV file to update.")
    parser.add_argument(
        "--skip-existing",
        action="store_true",
        help="Leave rows untouched when docPath already has a value.",
    )
    parser.add_argument(
        "--timeout",
        type=int,
        default=DEFAULT_TIMEOUT_MS,
        help="Per-request timeout in milliseconds.",
    )
    parser.add_argument(
        "--headed",
        action="store_true",
        help="Run browser in headed mode for debugging.",
    )
    return parser.parse_args()


def main() -> None:
    args = parse_args()
    asyncio.run(
        fill_doc_paths(
            csv_path=args.csv,
            skip_existing=args.skip_existing,
            timeout_ms=args.timeout,
            headless=not args.headed,
        )
    )


if __name__ == "__main__":
    main()
