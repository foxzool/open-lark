import importlib.util
import unittest
from pathlib import Path


MODULE_PATH = Path(__file__).resolve().parents[1] / "validate_apis.py"
SPEC = importlib.util.spec_from_file_location("validate_apis", MODULE_PATH)
validate_apis = importlib.util.module_from_spec(SPEC)
assert SPEC.loader is not None
SPEC.loader.exec_module(validate_apis)


class PriorityModelTests(unittest.TestCase):
    def _build_model(self):
        return validate_apis.PriorityModel.from_data(
            {
                "defaults": {
                    "business_value": 2,
                    "usage_frequency": 2,
                    "implementation_effort": 4,
                },
                "weights": {
                    "business_value": 0.5,
                    "usage_frequency": 0.3,
                    "implementation_effort": 0.2,
                },
                "priority_tiers": [
                    {"name": "P0", "min_score": 4.4},
                    {"name": "P1", "min_score": 3.7},
                    {"name": "P2", "min_score": 3.0},
                    {"name": "P3", "min_score": 0.0},
                ],
                "rules": [
                    {
                        "name": "task-baseline",
                        "biz_tags": ["task"],
                        "business_value": 4,
                        "usage_frequency": 4,
                    },
                    {
                        "name": "read-is-cheap",
                        "methods": ["GET"],
                        "implementation_effort": 2,
                    },
                    {
                        "name": "task-v2-core",
                        "expected_file_prefixes": ["task/task/v2/task/"],
                        "business_value": 5,
                    },
                ],
            },
            "<inline>",
        )

    def _build_api(self, expected_file: str, url: str, name: str = "列取任务列表"):
        return validate_apis.APIInfo(
            api_id="1",
            name=name,
            biz_tag="task",
            meta_project="task",
            meta_version="v2",
            meta_resource="task",
            meta_name="list",
            url=url,
            doc_path="",
            expected_file=expected_file,
        )

    def test_later_rule_overrides_generic_dimension(self):
        model = self._build_model()
        api = self._build_api("task/task/v2/task/list.rs", "GET:/open-apis/task/v2/tasks")

        model.evaluate(api)

        self.assertEqual(api.business_value, 5)
        self.assertEqual(api.usage_frequency, 4)
        self.assertEqual(api.implementation_effort, 2)
        self.assertEqual(api.priority_level, "P0")
        self.assertEqual(api.priority_reasons, ["task-baseline", "read-is-cheap", "task-v2-core"])

    def test_sort_key_prefers_higher_priority_and_score(self):
        model = self._build_model()
        high = self._build_api("task/task/v2/task/list.rs", "GET:/open-apis/task/v2/tasks", "高优先")
        low = self._build_api("task/task/v1/task/create.rs", "POST:/open-apis/task/v1/tasks", "低优先")
        low.expected_file = "task/task/v1/task/create.rs"
        low.meta_version = "v1"

        model.evaluate(high)
        model.evaluate(low)

        ordered = sorted([low, high], key=model.sort_key)
        self.assertEqual([api.name for api in ordered], ["高优先", "低优先"])


if __name__ == "__main__":
    unittest.main()
