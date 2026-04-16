import json
import tempfile
import unittest
from pathlib import Path
import importlib.util


MODULE_PATH = Path(__file__).resolve().parents[1] / "release_quality_status.py"
SPEC = importlib.util.spec_from_file_location("release_quality_status", MODULE_PATH)
release_quality_status = importlib.util.module_from_spec(SPEC)
assert SPEC.loader is not None
SPEC.loader.exec_module(release_quality_status)


class ReleaseQualityStatusTests(unittest.TestCase):
    def test_render_rows_marks_contract_and_snapshot_signals(self):
        with tempfile.TemporaryDirectory() as temp_dir:
            root = Path(temp_dir)
            (root / "Cargo.toml").write_text(
                """
[workspace]
members = ["crates/openlark-demo"]

[package]
name = "openlark"
version = "0.15.0"
                """.strip(),
                encoding="utf-8",
            )

            crate_dir = root / "crates" / "openlark-demo"
            crate_dir.mkdir(parents=True)
            (crate_dir / "Cargo.toml").write_text(
                """
[package]
name = "openlark-demo"
version = "0.15.0"
                """.strip(),
                encoding="utf-8",
            )
            tests_dir = crate_dir / "tests"
            tests_dir.mkdir()
            (tests_dir / "demo_contract_models.rs").write_text("// contract", encoding="utf-8")
            (tests_dir / "helper_snapshots.rs").write_text("// snapshots", encoding="utf-8")
            snapshots_dir = tests_dir / "snapshots"
            snapshots_dir.mkdir()
            (snapshots_dir / "demo.snap").write_text("snapshot", encoding="utf-8")

            summary = {
                "crates": {
                    "openlark-demo": {
                        "completion_rate": 97.5,
                        "missing": 2,
                        "priority_counts": {"P0": 1, "P1": 1},
                    }
                }
            }

            rows = release_quality_status.build_rows(root, summary)
            demo = next(row for row in rows if row["crate"] == "openlark-demo")
            self.assertEqual(demo["typed_coverage"], "97.5%")
            self.assertEqual(demo["missing"], 2)
            self.assertEqual(demo["contract_tests"], "yes")
            self.assertEqual(demo["snapshot_tests"], "yes")
            self.assertEqual(demo["note"], "缺口：P0=1, P1=1")

    def test_render_rows_marks_non_summary_crate_as_na(self):
        with tempfile.TemporaryDirectory() as temp_dir:
            root = Path(temp_dir)
            (root / "Cargo.toml").write_text(
                """
[workspace]
members = ["crates/openlark-core"]

[package]
name = "openlark"
version = "0.15.0"
                """.strip(),
                encoding="utf-8",
            )
            crate_dir = root / "crates" / "openlark-core"
            crate_dir.mkdir(parents=True)
            (crate_dir / "Cargo.toml").write_text(
                """
[package]
name = "openlark-core"
version = "0.15.0"
                """.strip(),
                encoding="utf-8",
            )

            rows = release_quality_status.build_rows(root, {"crates": {}})
            core = next(row for row in rows if row["crate"] == "openlark-core")
            self.assertEqual(core["typed_coverage"], "n/a")
            self.assertEqual(core["missing"], "n/a")
            self.assertEqual(core["note"], "非 typed API 覆盖统计对象")


if __name__ == "__main__":
    unittest.main()
