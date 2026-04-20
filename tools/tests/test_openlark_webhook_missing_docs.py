import subprocess
import unittest


class OpenlarkWebhookMissingDocsTests(unittest.TestCase):
    def test_openlark_webhook_has_no_missing_docs_warnings(self):
        result = subprocess.run(
            ["cargo", "test", "-p", "openlark-webhook", "--all-features", "--no-run"],
            capture_output=True,
            text=True,
            check=False,
        )

        output = result.stdout + result.stderr
        self.assertEqual(result.returncode, 0, msg=output)
        self.assertNotIn("warning: missing documentation for ", output, msg=output)


if __name__ == "__main__":
    unittest.main()
