import sys
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "tools"))

import audit_todos


class AuditTodosTests(unittest.TestCase):
    def test_classify_known_buckets(self):
        self.assertEqual(
            audit_todos.classify("tests/unit/websocket/example.rs")[0],
            "websocket_test_placeholders",
        )
        self.assertEqual(
            audit_todos.classify("tests/unit/contact/example.rs")[0],
            "contact_test_placeholders",
        )
        self.assertEqual(
            audit_todos.classify("crates/openlark-hr/src/hire/hire/v1/x.rs")[0],
            "hr_hire_generated_stubs",
        )
        self.assertEqual(
            audit_todos.classify("crates/openlark-user/src/settings/v1.rs")[0],
            "source_api_stubs",
        )
        self.assertEqual(
            audit_todos.classify("tools/restructure_hr.py")[0],
            "internal_tooling",
        )
        self.assertEqual(audit_todos.classify("examples/foo.rs")[0], "other")

    def test_build_summary_counts(self):
        entries = [
            audit_todos.TodoEntry(
                path="tests/unit/websocket/a.rs",
                line=1,
                text="// TODO one",
                category="websocket_test_placeholders",
                priority="p1",
                rationale="x",
            ),
            audit_todos.TodoEntry(
                path="tests/unit/websocket/a.rs",
                line=2,
                text="// TODO two",
                category="websocket_test_placeholders",
                priority="p1",
                rationale="x",
            ),
            audit_todos.TodoEntry(
                path="tools/x.py",
                line=3,
                text="# TODO three",
                category="internal_tooling",
                priority="p3",
                rationale="y",
            ),
        ]
        summary = audit_todos.build_summary(entries)
        self.assertEqual(summary["total"], 3)
        self.assertEqual(summary["categories"][0]["category"], "websocket_test_placeholders")
        self.assertEqual(summary["categories"][0]["count"], 2)
        self.assertEqual(summary["top_files"][0], {"path": "tests/unit/websocket/a.rs", "count": 2})


if __name__ == "__main__":
    unittest.main()
