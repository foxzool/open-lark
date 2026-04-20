import subprocess
import unittest
from pathlib import Path


class OpenlarkDocsMissingDocsTests(unittest.TestCase):
    def test_openlark_docs_has_no_missing_docs_warnings(self):
        result = subprocess.run(
            ["cargo", "test", "-p", "openlark-docs", "--all-features", "--no-run"],
            capture_output=True,
            text=True,
            check=False,
        )

        output = result.stdout + result.stderr
        self.assertEqual(result.returncode, 0, msg=output)
        self.assertNotIn("warning: missing documentation for ", output, msg=output)

    def test_openlark_docs_files_do_not_suppress_missing_docs(self):
        for path in Path("crates/openlark-docs").rglob("*.rs"):
            content = path.read_text(encoding="utf-8")
            self.assertNotIn(
                "#![allow(missing_docs)]",
                content,
                msg=f"{path} should not suppress missing_docs in docs crate",
            )


if __name__ == "__main__":
    unittest.main()
