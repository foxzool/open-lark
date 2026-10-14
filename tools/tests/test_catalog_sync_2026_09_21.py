"""#678 飞书 API 目录同步（2026-09-21）验收测试。

Seams（公开边界）: 其余 5 个 device_record 身份（url / meta.Name）不变。
`device_records.mine`（id 7563551656446263298）曾在当日目录中删除；
#688（2026-10-12）将其以新合同重新上架，存在性改由
`test_catalog_sync_2026_10_12` 锁定，本文件不再断言缺席。
"""

from __future__ import annotations

import csv
import unittest
from pathlib import Path

from tools.tests.catalog_row_count import CURRENT_CATALOG_ROW_COUNT

REPO_ROOT = Path(__file__).resolve().parents[2]
CSV_PATH = REPO_ROOT / "api_list_export.csv"

DEVICE_RECORD_EXPECTATIONS = {
    "7430737008881614850": {
        "url": "POST:/open-apis/security_and_compliance/v2/device_records",
        "meta.Name": "create",
    },
    "7430737008881631234": {
        "url": "GET:/open-apis/security_and_compliance/v2/device_records",
        "meta.Name": "list",
    },
    "7430737008881647618": {
        "url": "GET:/open-apis/security_and_compliance/v2/device_records/:device_record_id",
        "meta.Name": "get",
    },
    "7430737008881664002": {
        "url": "PUT:/open-apis/security_and_compliance/v2/device_records/:device_record_id",
        "meta.Name": "update",
    },
    "7430737008881680386": {
        "url": "DELETE:/open-apis/security_and_compliance/v2/device_records/:device_record_id",
        "meta.Name": "delete",
    },
}


def _load_csv_by_id() -> dict[str, dict[str, str]]:
    with CSV_PATH.open("r", encoding="utf-8-sig", newline="") as file:
        return {row["id"]: row for row in csv.DictReader(file) if row.get("id")}


class CatalogSync20260921Tests(unittest.TestCase):
    def test_remaining_device_records_unchanged(self) -> None:
        rows = _load_csv_by_id()
        self.assertEqual(len(rows), CURRENT_CATALOG_ROW_COUNT)
        for api_id, expected in DEVICE_RECORD_EXPECTATIONS.items():
            with self.subTest(api_id=api_id):
                self.assertIn(api_id, rows, f"缺少 API id={api_id}")
                row = rows[api_id]
                for field, value in expected.items():
                    self.assertEqual(row.get(field), value, f"id={api_id} field={field}")


if __name__ == "__main__":
    unittest.main()
