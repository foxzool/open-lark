import subprocess
import unittest


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


if __name__ == "__main__":
    unittest.main()
