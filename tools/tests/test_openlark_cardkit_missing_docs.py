import subprocess
import unittest


class OpenlarkCardkitMissingDocsTests(unittest.TestCase):
    def test_openlark_cardkit_has_no_missing_docs_warnings(self):
        result = subprocess.run(
            ["cargo", "check", "-p", "openlark-cardkit", "--all-features"],
            capture_output=True,
            text=True,
            check=False,
        )

        output = result.stdout + result.stderr
        self.assertEqual(result.returncode, 0, msg=output)
        self.assertNotIn("warning: missing documentation for ", output, msg=output)


if __name__ == "__main__":
    unittest.main()
