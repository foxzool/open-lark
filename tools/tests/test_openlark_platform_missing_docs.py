import subprocess
import unittest
from pathlib import Path


class OpenlarkPlatformMissingDocsTests(unittest.TestCase):
    def test_openlark_platform_has_no_missing_docs_warnings(self):
        result = subprocess.run(
            ["cargo", "check", "-p", "openlark-platform", "--features", "v1"],
            capture_output=True,
            text=True,
            check=False,
        )

        output = result.stdout + result.stderr
        self.assertEqual(result.returncode, 0, msg=output)
        self.assertNotIn("warning: missing documentation for ", output, msg=output)

    def test_v1_root_modules_do_not_use_missing_docs_allow(self):
        root_modules = [
            Path("crates/openlark-platform/src/app_engine/apaas/v1/mod.rs"),
            Path("crates/openlark-platform/src/admin/admin/v1/mod.rs"),
            Path("crates/openlark-platform/src/directory/directory/v1/mod.rs"),
        ]

        for path in root_modules:
            content = path.read_text(encoding="utf-8")
            self.assertNotIn("#![allow(missing_docs)]", content, msg=f"{path} still suppresses the full subtree")


if __name__ == "__main__":
    unittest.main()
