import tomllib
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
POLICY_PATH = ROOT / "tools" / "typed_coverage_release.toml"


class TypedCoverageReleasePolicyTests(unittest.TestCase):
    def test_release_policy_toml_is_parseable_and_has_required_sections(self):
        payload = tomllib.loads(POLICY_PATH.read_text(encoding="utf-8"))

        self.assertEqual(payload["policy_name"], "typed-coverage-stable-release")
        self.assertIn("inputs", payload)
        self.assertIn("stable_release", payload)

        stable_release = payload["stable_release"]
        self.assertIn("hard_gates", stable_release)
        self.assertIn("waiver_gates", stable_release)
        self.assertIn("waiver", stable_release)
        self.assertIn("reporting", stable_release)

        hard_gates = stable_release["hard_gates"]
        self.assertGreaterEqual(hard_gates["summary_completion_rate_min"], 0)
        self.assertGreaterEqual(hard_gates["core_business_completion_rate_min"], 0)
        self.assertGreaterEqual(hard_gates["core_crate_completion_rate_min"], 0)

        waiver = stable_release["waiver"]
        self.assertIn("maintainer", waiver["required_approvers"])
        self.assertIn("domain-owner", waiver["required_approvers"])
        self.assertIn("target_release", waiver["required_fields"])


if __name__ == "__main__":
    unittest.main()
