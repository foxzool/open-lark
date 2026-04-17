# TODO / FIXME Audit Summary

- Total TODO/FIXME matches in tracked source roots: **519**
- Scan roots: `crates`, `tests`, `examples`, `src`, `tools`
- Excluded: `target`, `.git`, `.omx`, `reports`, `.serena`, `.agents`

## Category summary

| Category | Priority | Count | Why it matters |
| --- | --- | ---: | --- |
| `hr_hire_generated_stubs` | `p1` | 243 | Generated HR hire APIs still contain TODO field / implementation stubs inside shipped source. |
| `other` | `p2` | 96 | Remaining TODO/FIXME comments should stay visible until explicitly triaged. |
| `websocket_test_placeholders` | `p1` | 96 | Large WebSocket integration test files are still placeholders and hide real coverage gaps. |
| `contact_test_placeholders` | `p1` | 60 | Contact test suites contain many TODO placeholders and block trustworthy behavior coverage. |
| `source_api_stubs` | `p1` | 18 | User/platform/analytics source files still advertise TODO API implementations in runtime code. |
| `internal_tooling` | `p3` | 6 | Tooling TODOs are useful to keep, but they are not directly user-facing runtime debt. |

## Top files

| Count | File |
| ---: | --- |
| 96 | `tests/unit/websocket/websocket_integration_tests.rs` |
| 32 | `tests/unit/contact/department_tests.rs` |
| 28 | `tests/unit/contact/batch_operations_tests.rs` |
| 6 | `tools/restructure_hr.py` |
| 4 | `crates/openlark-hr/src/hire/hire/v2/interview_record/list.rs` |
| 4 | `crates/openlark-hr/src/hire/hire/v1/exam_marking_task/list.rs` |
| 4 | `crates/openlark-hr/src/hire/hire/v1/referral/search.rs` |
| 4 | `crates/openlark-hr/src/hire/hire/v1/role/list.rs` |
| 4 | `crates/openlark-hr/src/hire/hire/v1/talent_tag/list.rs` |
| 4 | `crates/openlark-hr/src/hire/hire/v1/location/list.rs` |
| 4 | `crates/openlark-hr/src/hire/hire/v1/website/list.rs` |
| 4 | `crates/openlark-hr/src/hire/hire/v1/user_role/list.rs` |
| 4 | `crates/openlark-hr/src/hire/hire/v1/subject/list.rs` |
| 4 | `crates/openlark-hr/src/hire/hire/v1/talent_folder/list.rs` |
| 4 | `crates/openlark-hr/src/hire/hire/v1/minutes/get.rs` |
| 4 | `crates/openlark-hr/src/hire/hire/v1/todo/list.rs` |
| 4 | `crates/openlark-hr/src/hire/hire/v1/evaluation_task/list.rs` |
| 4 | `crates/openlark-hr/src/hire/hire/v1/talent_object/query.rs` |
| 4 | `crates/openlark-hr/src/hire/hire/v1/evaluation/list.rs` |
| 4 | `crates/openlark-hr/src/hire/hire/v1/termination_reason/list.rs` |

## Recommended tracking split

- `websocket_test_placeholders`: follow-up issue for executable WebSocket integration tests
- `contact_test_placeholders`: follow-up issue for executable contact test coverage
- `hr_hire_generated_stubs`: follow-up issue for generated HR hire source stubs
- `source_api_stubs`: follow-up issue for user/platform/analytics runtime TODO implementations
- `internal_tooling` / `other`: keep visible unless they block release or API trust

## Filed follow-up issues

- #104 Replace WebSocket integration test TODO placeholders with executable coverage
- #105 Replace contact test TODO placeholders with executable coverage
- #106 Audit generated HR hire API TODO stubs in shipped source
- #107 Track remaining runtime TODO API stubs in user/platform/analytics
