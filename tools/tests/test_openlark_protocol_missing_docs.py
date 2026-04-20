import subprocess
import unittest
from pathlib import Path


class OpenlarkProtocolMissingDocsTests(unittest.TestCase):
    def test_openlark_protocol_has_no_missing_docs_warnings(self):
        result = subprocess.run(
            ["cargo", "test", "-p", "openlark-protocol", "--all-features", "--no-run"],
            capture_output=True,
            text=True,
            check=False,
        )

        output = result.stdout + result.stderr
        self.assertEqual(result.returncode, 0, msg=output)
        self.assertNotIn("warning: missing documentation for ", output, msg=output)

    def test_openlark_protocol_only_keeps_generated_module_exception(self):
        hits = []
        for path in Path("crates/openlark-protocol").rglob("*.rs"):
            content = path.read_text(encoding="utf-8")
            if "#[allow(missing_docs)]" in content:
                hits.append(path.as_posix())
            self.assertNotIn(
                "#![allow(missing_docs)]",
                content,
                msg=f"{path} should not use crate-level missing_docs suppression",
            )

        self.assertEqual(hits, ["crates/openlark-protocol/src/lib.rs"])


if __name__ == "__main__":
    unittest.main()
