import subprocess
import unittest
from pathlib import Path


class OpenlarkMailMissingDocsTests(unittest.TestCase):
    def test_openlark_mail_has_no_missing_docs_warnings(self):
        result = subprocess.run(
            ["cargo", "test", "-p", "openlark-mail", "--all-features", "--no-run"],
            capture_output=True,
            text=True,
            check=False,
        )

        output = result.stdout + result.stderr
        self.assertEqual(result.returncode, 0, msg=output)
        self.assertNotIn("warning: missing documentation for ", output, msg=output)

    def test_openlark_mail_mod_roots_do_not_suppress_missing_docs(self):
        for path in Path("crates/openlark-mail/src").rglob("mod.rs"):
            content = path.read_text(encoding="utf-8")
            self.assertNotIn(
                "#![allow(missing_docs)]",
                content,
                msg=f"{path} should not suppress missing_docs at module root",
            )


if __name__ == "__main__":
    unittest.main()
