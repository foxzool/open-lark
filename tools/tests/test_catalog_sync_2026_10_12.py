"""#688 飞书 API 目录同步（2026-10-12）验收测试。

Seams（公开边界）:
1. 3 个 Catalog Entry 的 id → name / bizTag / meta.* / fullPath / url
2. checked-in catalog 行数增至 CURRENT_CATALOG_ROW_COUNT
3. 相邻既有 device_record / spark app 身份字段不变
4. 全表仍按 (bizTag, meta.Project, meta.Version, meta.Resource, meta.Name, id) 有序
5. 三个 Rust Contract Target 均 Resolved，且 HTTP method/path 与 catalog url 一致
"""

from __future__ import annotations

import csv
import unittest
from pathlib import Path

from tools.api_contracts.official import load_api_identities, normalize_endpoint_path
from tools.api_contracts.rust_contract_resolution import Resolved, compose
from tools.api_contracts.rust_source import RustSourceContractAdapter
from tools.tests.catalog_row_count import CURRENT_CATALOG_ROW_COUNT

REPO_ROOT = Path(__file__).resolve().parents[2]
CSV_PATH = REPO_ROOT / "api_list_export.csv"

MINE_ID = "7563551656446263298"
EXPORT_ID = "7692846107466206181"
RELEASE_UPDATE_ID = "7692846107466222565"

NEW_API_EXPECTATIONS = {
    MINE_ID: {
        "name": "通过授权获取设备信息",
        "bizTag": "security_and_compliance",
        "meta.Project": "security_and_compliance",
        "meta.Version": "v2",
        "meta.Resource": "device_record",
        "meta.Name": "mine",
        "fullPath": "/document/uAjLw4CM/ukTMukTMukTM/security_and_compliance-v2/device_record/mine",
        "url": "GET:/open-apis/security_and_compliance/v2/device_records/mine",
    },
    EXPORT_ID: {
        "name": "导出妙搭应用代码包",
        "bizTag": "spark",
        "meta.Project": "spark",
        "meta.Version": "v1",
        "meta.Resource": "app",
        "meta.Name": "export",
        "fullPath": "/document/uAjLw4CM/ukTMukTMukTM/spark-v1/app/export",
        "url": "POST:/open-apis/spark/v1/apps/export",
    },
    RELEASE_UPDATE_ID: {
        "name": "回写发布节点状态",
        "bizTag": "spark",
        "meta.Project": "spark",
        "meta.Version": "v1",
        "meta.Resource": "app.release",
        "meta.Name": "update",
        "fullPath": "/document/uAjLw4CM/ukTMukTMukTM/spark-v1/app-release/update",
        "url": "PUT:/open-apis/spark/v1/apps/:app_id/releases/:release_id",
    },
}

EXISTING_EXPECTATIONS = {
    "7430737008881631234": {
        "name": "查询设备信息",
        "url": "GET:/open-apis/security_and_compliance/v2/device_records",
        "meta.Name": "list",
    },
    "7430737008881664002": {
        "name": "更新设备",
        "url": "PUT:/open-apis/security_and_compliance/v2/device_records/:device_record_id",
        "meta.Name": "update",
    },
    "7642228089434950837": {
        "name": "创建妙搭应用",
        "url": "POST:/open-apis/spark/v1/apps",
        "meta.Name": "create",
    },
    "7642228089434901685": {
        "name": "批量获取妙搭应用",
        "url": "GET:/open-apis/spark/v1/apps",
        "meta.Name": "list",
    },
}

SORT_KEY_FIELDS = (
    "bizTag",
    "meta.Project",
    "meta.Version",
    "meta.Resource",
    "meta.Name",
    "id",
)


def _load_csv_rows() -> list[dict[str, str]]:
    with CSV_PATH.open("r", encoding="utf-8-sig", newline="") as file:
        return [row for row in csv.DictReader(file) if row.get("id")]


def _load_csv_by_id() -> dict[str, dict[str, str]]:
    return {row["id"]: row for row in _load_csv_rows()}


def _sort_key(row: dict[str, str]) -> tuple[str, ...]:
    return tuple(row.get(field, "") for field in SORT_KEY_FIELDS)


class CatalogSync20261012Tests(unittest.TestCase):
    def test_checked_in_catalog_includes_three_new_apis(self) -> None:
        rows = _load_csv_by_id()
        self.assertEqual(len(rows), CURRENT_CATALOG_ROW_COUNT)
        for api_id, expected in NEW_API_EXPECTATIONS.items():
            with self.subTest(api_id=api_id):
                self.assertIn(api_id, rows, f"缺少 API id={api_id}")
                row = rows[api_id]
                for field, value in expected.items():
                    self.assertEqual(row.get(field), value, f"id={api_id} field={field}")

    def test_adjacent_existing_apis_are_unchanged(self) -> None:
        rows = _load_csv_by_id()
        for api_id, expected in EXISTING_EXPECTATIONS.items():
            with self.subTest(api_id=api_id):
                self.assertIn(api_id, rows, f"缺少 API id={api_id}")
                row = rows[api_id]
                for field, value in expected.items():
                    self.assertEqual(row.get(field), value, f"id={api_id} field={field}")

    def test_catalog_remains_sorted_by_meta_key(self) -> None:
        rows = _load_csv_rows()
        self.assertEqual(len(rows), CURRENT_CATALOG_ROW_COUNT)
        keys = [_sort_key(row) for row in rows]
        self.assertEqual(keys, sorted(keys))

    def test_three_contract_targets_are_resolved(self) -> None:
        identities = {
            entry.api_id: entry
            for entry in load_api_identities(REPO_ROOT / "api_list_export.csv")
        }
        resolver = compose(repository_root=REPO_ROOT)
        adapter = RustSourceContractAdapter(REPO_ROOT)

        for api_id in (MINE_ID, EXPORT_ID, RELEASE_UPDATE_ID):
            with self.subTest(api_id=api_id):
                entry = identities[api_id]
                result = resolver.resolve(entry)
                self.assertIsInstance(result, Resolved, f"id={api_id} 未 Resolved: {result}")
                assert isinstance(result, Resolved)
                contract = adapter.scan(result.target)
                self.assertGreaterEqual(len(contract.endpoint_calls), 1, result.target)
                resolved = [
                    call
                    for call in contract.endpoint_calls
                    if call.resolved_path
                ]
                self.assertGreaterEqual(len(resolved), 1, contract.endpoint_calls)
                call = resolved[0]
                catalog_method, catalog_path = entry.url.split(":", 1)
                self.assertEqual(call.method, catalog_method)
                self.assertEqual(
                    normalize_endpoint_path(call.resolved_path),
                    normalize_endpoint_path(catalog_path),
                )


if __name__ == "__main__":
    unittest.main()
