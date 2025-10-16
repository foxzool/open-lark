# Open-Lark API Methods Analysis

This document contains a comprehensive list of all API services and methods in the open-lark project.

## Summary

- **Services with APIs**: 40
- **API versions/modules**: 105
- **Total API methods**: 986

## API Methods by Service

### acs

#### access_record

| Method | File | Line |
|--------|------|------|
| `download_face_image()` | `src/service/acs/access_record/mod.rs` | 129 |
| `list_access_records()` | `src/service/acs/access_record/mod.rs` | 46 |

#### device

| Method | File | Line |
|--------|------|------|
| `list_devices()` | `src/service/acs/device/mod.rs` | 42 |

#### rule_external

| Method | File | Line |
|--------|------|------|
| `bind_device()` | `src/service/acs/rule_external/mod.rs` | 134 |
| `create_or_update_rule()` | `src/service/acs/rule_external/mod.rs` | 42 |
| `delete_rule()` | `src/service/acs/rule_external/mod.rs` | 102 |
| `get_rule()` | `src/service/acs/rule_external/mod.rs` | 70 |

#### user

| Method | File | Line |
|--------|------|------|
| `download_face_image()` | `src/service/acs/user/mod.rs` | 185 |
| `get_user()` | `src/service/acs/user/mod.rs` | 72 |
| `list_users()` | `src/service/acs/user/mod.rs` | 100 |
| `patch_user()` | `src/service/acs/user/mod.rs` | 43 |
| `upload_face_image()` | `src/service/acs/user/mod.rs` | 156 |

#### visitor

| Method | File | Line |
|--------|------|------|
| `create_visitor()` | `src/service/acs/visitor/mod.rs` | 42 |
| `delete_visitor()` | `src/service/acs/visitor/mod.rs` | 70 |

### admin

#### badge

| Method | File | Line |
|--------|------|------|
| `create_badge()` | `src/service/admin/badge/mod.rs` | 114 |
| `get_badge()` | `src/service/admin/badge/mod.rs` | 238 |
| `list_badges()` | `src/service/admin/badge/mod.rs` | 199 |
| `update_badge()` | `src/service/admin/badge/mod.rs` | 138 |
| `upload_badge_image()` | `src/service/admin/badge/mod.rs` | 175 |

#### badge_grant

| Method | File | Line |
|--------|------|------|
| `create_badge_grant()` | `src/service/admin/badge_grant/mod.rs` | 116 |
| `delete_badge_grant()` | `src/service/admin/badge_grant/mod.rs` | 140 |
| `get_badge_grant()` | `src/service/admin/badge_grant/mod.rs` | 243 |
| `list_badge_grants()` | `src/service/admin/badge_grant/mod.rs` | 203 |
| `update_badge_grant()` | `src/service/admin/badge_grant/mod.rs` | 168 |

#### data_report

| Method | File | Line |
|--------|------|------|
| `get_department_data_report()` | `src/service/admin/data_report/mod.rs` | 66 |
| `get_user_data_report()` | `src/service/admin/data_report/mod.rs` | 113 |

#### password

| Method | File | Line |
|--------|------|------|
| `reset_password()` | `src/service/admin/password/mod.rs` | 60 |
| `reset_password_with_builder()` | `src/service/admin/password/mod.rs` | 108 |

### ai

#### document_ai

| Method | File | Line |
|--------|------|------|
| `extract_contract_fields()` | `src/service/ai/document_ai/mod.rs` | 312 |
| `parse_resume()` | `src/service/ai/document_ai/mod.rs` | 168 |
| `recognize_bank_card()` | `src/service/ai/document_ai/mod.rs` | 240 |
| `recognize_business_card()` | `src/service/ai/document_ai/mod.rs` | 336 |
| `recognize_business_license()` | `src/service/ai/document_ai/mod.rs` | 264 |
| `recognize_chinese_passport()` | `src/service/ai/document_ai/mod.rs` | 456 |
| `recognize_driving_license()` | `src/service/ai/document_ai/mod.rs` | 216 |
| `recognize_food_manage_license()` | `src/service/ai/document_ai/mod.rs` | 576 |
| `recognize_food_produce_license()` | `src/service/ai/document_ai/mod.rs` | 552 |
| `recognize_health_certificate()` | `src/service/ai/document_ai/mod.rs` | 384 |
| `recognize_hkm_mainland_travel_permit()` | `src/service/ai/document_ai/mod.rs` | 408 |
| `recognize_id_card()` | `src/service/ai/document_ai/mod.rs` | 192 |
| `recognize_taxi_invoice()` | `src/service/ai/document_ai/mod.rs` | 528 |
| `recognize_train_invoice()` | `src/service/ai/document_ai/mod.rs` | 504 |
| `recognize_tw_mainland_travel_permit()` | `src/service/ai/document_ai/mod.rs` | 432 |
| `recognize_vat_invoice()` | `src/service/ai/document_ai/mod.rs` | 288 |
| `recognize_vehicle_invoice()` | `src/service/ai/document_ai/mod.rs` | 360 |
| `recognize_vehicle_license()` | `src/service/ai/document_ai/mod.rs` | 480 |

#### optical_char_recognition

| Method | File | Line |
|--------|------|------|
| `basic_recognize()` | `src/service/ai/optical_char_recognition/mod.rs` | 51 |

#### speech_to_text

| Method | File | Line |
|--------|------|------|
| `file_recognize()` | `src/service/ai/speech_to_text/mod.rs` | 65 |
| `stream_recognize()` | `src/service/ai/speech_to_text/mod.rs` | 89 |

#### translation

| Method | File | Line |
|--------|------|------|
| `detect()` | `src/service/ai/translation/mod.rs` | 91 |
| `translate()` | `src/service/ai/translation/mod.rs` | 115 |

### aily

#### knowledge

| Method | File | Line |
|--------|------|------|
| `ask_data_knowledge()` | `src/service/aily/knowledge/mod.rs` | 139 |
| `create_data_knowledge()` | `src/service/aily/knowledge/mod.rs` | 198 |
| `delete_data_knowledge()` | `src/service/aily/knowledge/mod.rs` | 260 |
| `get_data_knowledge()` | `src/service/aily/knowledge/mod.rs` | 229 |
| `list_data_knowledge()` | `src/service/aily/knowledge/mod.rs` | 291 |
| `list_data_knowledge_categories()` | `src/service/aily/knowledge/mod.rs` | 333 |
| `upload_file()` | `src/service/aily/knowledge/mod.rs` | 169 |

#### message

| Method | File | Line |
|--------|------|------|
| `create_message()` | `src/service/aily/message/mod.rs` | 80 |
| `get_message()` | `src/service/aily/message/mod.rs` | 113 |
| `list_messages()` | `src/service/aily/message/mod.rs` | 146 |

#### run

| Method | File | Line |
|--------|------|------|
| `cancel_run()` | `src/service/aily/run/mod.rs` | 204 |
| `create_run()` | `src/service/aily/run/mod.rs` | 94 |
| `get_run()` | `src/service/aily/run/mod.rs` | 128 |
| `list_runs()` | `src/service/aily/run/mod.rs` | 161 |

#### session

| Method | File | Line |
|--------|------|------|
| `create_session()` | `src/service/aily/session/mod.rs` | 95 |
| `delete_session()` | `src/service/aily/session/mod.rs` | 186 |
| `get_session()` | `src/service/aily/session/mod.rs` | 155 |
| `update_session()` | `src/service/aily/session/mod.rs` | 123 |

#### skill

| Method | File | Line |
|--------|------|------|
| `get_skill()` | `src/service/aily/skill/mod.rs` | 112 |
| `list_skills()` | `src/service/aily/skill/mod.rs` | 143 |
| `start_skill()` | `src/service/aily/skill/mod.rs` | 80 |

### apass

#### audit_log

| Method | File | Line |
|--------|------|------|
| `get_audit_log()` | `src/service/apass/audit_log/mod.rs` | 158 |
| `get_data_change_log_detail()` | `src/service/apass/audit_log/mod.rs` | 230 |
| `list_audit_events()` | `src/service/apass/audit_log/mod.rs` | 258 |
| `list_audit_logs()` | `src/service/apass/audit_log/mod.rs` | 108 |
| `list_data_change_logs()` | `src/service/apass/audit_log/mod.rs` | 186 |

#### environment_variable

| Method | File | Line |
|--------|------|------|
| `get_environment_variable()` | `src/service/apass/environment_variable/mod.rs` | 105 |
| `query_environment_variables()` | `src/service/apass/environment_variable/mod.rs` | 67 |

#### flow

| Method | File | Line |
|--------|------|------|
| `add_assignee_user_task()` | `src/service/apass/flow/mod.rs` | 280 |
| `agree_user_task()` | `src/service/apass/flow/mod.rs` | 190 |
| `cancel_user_task()` | `src/service/apass/flow/mod.rs` | 369 |
| `cc_user_task()` | `src/service/apass/flow/mod.rs` | 310 |
| `create_user_task_chat_group()` | `src/service/apass/flow/mod.rs` | 457 |
| `execute_flow()` | `src/service/apass/flow/mod.rs` | 117 |
| `expedite_user_task()` | `src/service/apass/flow/mod.rs` | 340 |
| `get_user_task_rollback_points()` | `src/service/apass/flow/mod.rs` | 399 |
| `query_user_tasks()` | `src/service/apass/flow/mod.rs` | 149 |
| `reject_user_task()` | `src/service/apass/flow/mod.rs` | 220 |
| `rollback_user_task()` | `src/service/apass/flow/mod.rs` | 427 |
| `transfer_user_task()` | `src/service/apass/flow/mod.rs` | 250 |

#### function

| Method | File | Line |
|--------|------|------|
| `invoke_function()` | `src/service/apass/function/mod.rs` | 50 |

#### object

| Method | File | Line |
|--------|------|------|
| `batch_create_records()` | `src/service/apass/object/mod.rs` | 472 |
| `batch_delete_records()` | `src/service/apass/object/mod.rs` | 440 |
| `batch_query_records()` | `src/service/apass/object/mod.rs` | 397 |
| `batch_update_records()` | `src/service/apass/object/mod.rs` | 365 |
| `create_record()` | `src/service/apass/object/mod.rs` | 335 |
| `delete_record()` | `src/service/apass/object/mod.rs` | 304 |
| `get_record()` | `src/service/apass/object/mod.rs` | 237 |
| `oql_query()` | `src/service/apass/object/mod.rs` | 164 |
| `search_records()` | `src/service/apass/object/mod.rs` | 195 |
| `update_record()` | `src/service/apass/object/mod.rs` | 273 |

#### permission

| Method | File | Line |
|--------|------|------|
| `batch_create_record_permission_member_authorization()` | `src/service/apass/permission/mod.rs` | 204 |
| `batch_create_role_member_authorization()` | `src/service/apass/permission/mod.rs` | 101 |
| `batch_remove_record_permission_member_authorization()` | `src/service/apass/permission/mod.rs` | 168 |
| `batch_remove_role_member_authorization()` | `src/service/apass/permission/mod.rs` | 69 |
| `get_role_member()` | `src/service/apass/permission/mod.rs` | 135 |

#### seat

| Method | File | Line |
|--------|------|------|
| `list_seat_activity()` | `src/service/apass/seat/mod.rs` | 101 |
| `list_seat_assignment()` | `src/service/apass/seat/mod.rs` | 67 |

### application

#### v6

| Method | File | Line |
|--------|------|------|
| `apply()` | `src/service/application/v6/scope/mod.rs` | 30 |
| `check_user_access()` | `src/service/application/v6/appstore_paid_info/mod.rs` | 30 |
| `check_white_black_list()` | `src/service/application/v6/admin/mod.rs` | 207 |
| `contacts_range_configuration()` | `src/service/application/v6/admin/mod.rs` | 103 |
| `contacts_range_suggest()` | `src/service/application/v6/application/mod.rs` | 191 |
| `department_overview()` | `src/service/application/v6/app_usage/mod.rs` | 30 |
| `enable_disable_app()` | `src/service/application/v6/admin/mod.rs` | 279 |
| `get()` | `src/service/application/v6/application/mod.rs` | 102 |
| `get_app_admin_permissions()` | `src/service/application/v6/admin/mod.rs` | 336 |
| `get_app_availability()` | `src/service/application/v6/admin/mod.rs` | 173 |
| `get_collaborators()` | `src/service/application/v6/application/mod.rs` | 75 |
| `get_order_info()` | `src/service/application/v6/appstore_paid_info/mod.rs` | 93 |
| `get_user_available_apps()` | `src/service/application/v6/admin/mod.rs` | 64 |
| `get_version()` | `src/service/application/v6/application/mod.rs` | 129 |
| `list()` | `src/service/application/v6/application_feedback/mod.rs` | 60 |
| `list()` | `src/service/application/v6/scope/mod.rs` | 52 |
| `list_app_admins()` | `src/service/application/v6/admin/mod.rs` | 301 |
| `list_installed_apps()` | `src/service/application/v6/admin/mod.rs` | 30 |
| `list_tenant_paid_plans()` | `src/service/application/v6/appstore_paid_info/mod.rs` | 62 |
| `list_versions()` | `src/service/application/v6/application/mod.rs` | 156 |
| `message_push_overview()` | `src/service/application/v6/app_usage/mod.rs` | 70 |
| `overview()` | `src/service/application/v6/app_usage/mod.rs` | 97 |
| `set()` | `src/service/application/v6/app_badge/mod.rs` | 30 |
| `transfer_owner()` | `src/service/application/v6/application/mod.rs` | 31 |
| `underaudit_list()` | `src/service/application/v6/application/mod.rs` | 218 |
| `update()` | `src/service/application/v6/application_feedback/mod.rs` | 30 |
| `update_app_availability()` | `src/service/application/v6/admin/mod.rs` | 243 |
| `update_audit_status()` | `src/service/application/v6/application/mod.rs` | 249 |
| `update_collaborators()` | `src/service/application/v6/application/mod.rs` | 53 |
| `update_contacts_range_configuration()` | `src/service/application/v6/admin/mod.rs` | 137 |
| `update_group()` | `src/service/application/v6/application/mod.rs` | 271 |
| `verify_app_admin()` | `src/service/application/v6/admin/mod.rs` | 370 |

### approval

#### v4

| Method | File | Line |
|--------|------|------|
| `add_sign()` | `src/service/approval/v4/task/mod.rs` | 199 |
| `approval_id()` | `src/service/approval/v4/search/mod.rs` | 273 |
| `approve()` | `src/service/approval/v4/task/mod.rs` | 95 |
| `cancel()` | `src/service/approval/v4/instance/mod.rs` | 186 |
| `cc()` | `src/service/approval/v4/search/mod.rs` | 234 |
| `cc()` | `src/service/approval/v4/instance/mod.rs` | 213 |
| `check()` | `src/service/approval/v4/external_instance/mod.rs` | 112 |
| `create()` | `src/service/approval/v4/instance_comment/mod.rs` | 70 |
| `create()` | `src/service/approval/v4/external_instance/mod.rs` | 81 |
| `create()` | `src/service/approval/v4/instance/mod.rs` | 155 |
| `create()` | `src/service/approval/v4/external_approval/mod.rs` | 88 |
| `create()` | `src/service/approval/v4/approval/mod.rs` | 75 |
| `delete()` | `src/service/approval/v4/instance_comment/mod.rs` | 99 |
| `get()` | `src/service/approval/v4/instance/mod.rs` | 273 |
| `get()` | `src/service/approval/v4/external_approval/mod.rs` | 119 |
| `get()` | `src/service/approval/v4/approval/mod.rs` | 106 |
| `instances()` | `src/service/approval/v4/search/mod.rs` | 141 |
| `list()` | `src/service/approval/v4/external_task/mod.rs` | 59 |
| `list()` | `src/service/approval/v4/instance_comment/mod.rs` | 157 |
| `list()` | `src/service/approval/v4/instance/mod.rs` | 300 |
| `preview()` | `src/service/approval/v4/instance/mod.rs` | 242 |
| `reject()` | `src/service/approval/v4/task/mod.rs` | 120 |
| `remove_all()` | `src/service/approval/v4/instance_comment/mod.rs` | 130 |
| `resubmit()` | `src/service/approval/v4/task/mod.rs` | 224 |
| `rollback()` | `src/service/approval/v4/task/mod.rs` | 170 |
| `tasks()` | `src/service/approval/v4/search/mod.rs` | 186 |
| `transfer()` | `src/service/approval/v4/task/mod.rs` | 145 |
| `update()` | `src/service/approval/v4/message/mod.rs` | 91 |
| `upload()` | `src/service/approval/v4/file/mod.rs` | 43 |
| `user_tasks()` | `src/service/approval/v4/search/mod.rs` | 299 |

### attendance

#### v1

| Method | File | Line |
|--------|------|------|
| `batch_create()` | `src/service/attendance/v1/user_task.rs` | 32 |
| `batch_create()` | `src/service/attendance/v1/user_daily_shift.rs` | 28 |
| `batch_create_temp()` | `src/service/attendance/v1/user_daily_shift.rs` | 93 |
| `batch_del()` | `src/service/attendance/v1/user_task.rs` | 148 |
| `create()` | `src/service/attendance/v1/user_task_remedy.rs` | 29 |
| `create()` | `src/service/attendance/v1/user_approval.rs` | 81 |
| `create()` | `src/service/attendance/v1/group.rs` | 76 |
| `create()` | `src/service/attendance/v1/shift.rs` | 34 |
| `del_report()` | `src/service/attendance/v1/archive_rule.rs` | 98 |
| `delete()` | `src/service/attendance/v1/group.rs` | 142 |
| `delete()` | `src/service/attendance/v1/shift.rs` | 116 |
| `download_photo()` | `src/service/attendance/v1/user_setting.rs` | 156 |
| `get()` | `src/service/attendance/v1/user_task.rs` | 63 |
| `get()` | `src/service/attendance/v1/leave_employ_expire_record.rs` | 26 |
| `get()` | `src/service/attendance/v1/group.rs` | 165 |
| `get()` | `src/service/attendance/v1/shift.rs` | 139 |
| `list()` | `src/service/attendance/v1/archive_rule.rs` | 133 |
| `list()` | `src/service/attendance/v1/group.rs` | 232 |
| `list()` | `src/service/attendance/v1/shift.rs` | 205 |
| `list_user()` | `src/service/attendance/v1/group.rs` | 32 |
| `modify()` | `src/service/attendance/v1/user_setting.rs` | 32 |
| `patch()` | `src/service/attendance/v1/leave_accrual_record.rs` | 32 |
| `process()` | `src/service/attendance/v1/user_approval.rs` | 117 |
| `query()` | `src/service/attendance/v1/user_task.rs` | 94 |
| `query()` | `src/service/attendance/v1/user_task_remedy.rs` | 98 |
| `query()` | `src/service/attendance/v1/user_approval.rs` | 31 |
| `query()` | `src/service/attendance/v1/user_setting.rs` | 78 |
| `query()` | `src/service/attendance/v1/user_daily_shift.rs` | 59 |
| `query()` | `src/service/attendance/v1/shift.rs` | 159 |
| `query_data()` | `src/service/attendance/v1/user_stats_data.rs` | 113 |
| `query_fields()` | `src/service/attendance/v1/user_stats_data.rs` | 85 |
| `query_result()` | `src/service/attendance/v1/user_task.rs` | 179 |
| `query_settings()` | `src/service/attendance/v1/user_stats_data.rs` | 61 |
| `query_user_allowed_remedys()` | `src/service/attendance/v1/user_task_remedy.rs` | 65 |
| `query_user_stats_fields()` | `src/service/attendance/v1/archive_rule.rs` | 35 |
| `search()` | `src/service/attendance/v1/group.rs` | 197 |
| `update()` | `src/service/attendance/v1/user_stats_data.rs` | 30 |
| `upload_photo()` | `src/service/attendance/v1/user_setting.rs` | 109 |
| `upload_report()` | `src/service/attendance/v1/archive_rule.rs` | 63 |

### authentication

#### v1

| Method | File | Line |
|--------|------|------|
| `get()` | `src/service/authentication/v1/auth.rs` | 25 |

### bot

#### v3

| Method | File | Line |
|--------|------|------|
| `get()` | `src/service/bot/v3/info/mod.rs` | 54 |

### calendar

#### v4

| Method | File | Line |
|--------|------|------|
| `create()` | `src/service/calendar/v4/meeting_chat/create.rs` | 45 |
| `create()` | `src/service/calendar/v4/calendar/create.rs` | 160 |
| `delete()` | `src/service/calendar/v4/meeting_chat/delete.rs` | 45 |
| `get()` | `src/service/calendar/v4/calendar/get.rs` | 111 |
| `list()` | `src/service/calendar/v4/calendar/list.rs` | 155 |

### cardkit

#### v1

| Method | File | Line |
|--------|------|------|
| `batch_update()` | `src/service/cardkit/v1/card/batch_update.rs` | 140 |
| `create()` | `src/service/cardkit/v1/card/create.rs` | 148 |
| `create()` | `src/service/cardkit/v1/card_element/create.rs` | 165 |
| `settings()` | `src/service/cardkit/v1/card/settings.rs` | 153 |
| `update()` | `src/service/cardkit/v1/card/update.rs` | 154 |

### cloud_docs

#### assistant

| Method | File | Line |
|--------|------|------|
| `activate()` | `src/service/cloud_docs/assistant/v1/subscription/mod.rs` | 131 |
| `cancel()` | `src/service/cloud_docs/assistant/v1/subscription/mod.rs` | 163 |
| `create()` | `src/service/cloud_docs/assistant/v1/subscription/mod.rs` | 43 |
| `create_subscription()` | `src/service/cloud_docs/assistant/v1/subscription/create.rs` | 246 |
| `eco_activate()` | `src/service/cloud_docs/assistant/v1/subscription/mod.rs` | 195 |
| `get()` | `src/service/cloud_docs/assistant/v1/subscription/mod.rs` | 29 |
| `get_subscription()` | `src/service/cloud_docs/assistant/v1/subscription/get.rs` | 241 |
| `is_subscribed()` | `src/service/cloud_docs/assistant/v1/subscription/mod.rs` | 227 |
| `patch()` | `src/service/cloud_docs/assistant/v1/subscription/mod.rs` | 57 |
| `patch_subscription()` | `src/service/cloud_docs/assistant/v1/subscription/patch.rs` | 335 |
| `pause()` | `src/service/cloud_docs/assistant/v1/subscription/mod.rs` | 147 |
| `quick_activate()` | `src/service/cloud_docs/assistant/v1/subscription/mod.rs` | 179 |
| `quick_subscribe_bitable()` | `src/service/cloud_docs/assistant/v1/subscription/mod.rs` | 86 |
| `quick_subscribe_doc()` | `src/service/cloud_docs/assistant/v1/subscription/mod.rs` | 71 |
| `quick_subscribe_sheet()` | `src/service/cloud_docs/assistant/v1/subscription/mod.rs` | 101 |
| `quick_subscribe_wiki()` | `src/service/cloud_docs/assistant/v1/subscription/mod.rs` | 116 |
| `safe_pause()` | `src/service/cloud_docs/assistant/v1/subscription/mod.rs` | 211 |

#### bitable

| Method | File | Line |
|--------|------|------|
| `batch_create()` | `src/service/cloud_docs/bitable/v1/app_table/batch_create.rs` | 21 |
| `batch_create_record()` | `src/service/cloud_docs/bitable/v1/app_table_record/batch_create.rs` | 136 |
| `batch_create_role_members()` | `src/service/cloud_docs/bitable/v1/app_role_member/batch_create.rs` | 135 |
| `batch_delete()` | `src/service/cloud_docs/bitable/v1/app_table/batch_delete.rs` | 21 |
| `batch_delete_record()` | `src/service/cloud_docs/bitable/v1/app_table_record/batch_delete.rs` | 112 |
| `batch_delete_role_members()` | `src/service/cloud_docs/bitable/v1/app_role_member/batch_delete.rs` | 131 |
| `batch_get_record()` | `src/service/cloud_docs/bitable/v1/app_table_record/batch_get.rs` | 151 |
| `batch_update_record()` | `src/service/cloud_docs/bitable/v1/app_table_record/batch_update.rs` | 134 |
| `copy()` | `src/service/cloud_docs/bitable/v1/app/copy.rs` | 21 |
| `copy_dashboard()` | `src/service/cloud_docs/bitable/v1/app_dashboard/copy.rs` | 108 |
| `create()` | `src/service/cloud_docs/bitable/v1/app/create.rs` | 23 |
| `create()` | `src/service/cloud_docs/bitable/v1/app_role_member/create.rs` | 21 |
| `create()` | `src/service/cloud_docs/bitable/v1/app_table/create.rs` | 21 |
| `create()` | `src/service/cloud_docs/bitable/v1/app_table_view/create.rs` | 21 |
| `create()` | `src/service/cloud_docs/bitable/v1/app_role/create.rs` | 21 |
| `create_field()` | `src/service/cloud_docs/bitable/v1/app_table_field/create.rs` | 175 |
| `create_record()` | `src/service/cloud_docs/bitable/v1/app_table_record/create.rs` | 153 |
| `delete()` | `src/service/cloud_docs/bitable/v1/app_table/delete.rs` | 21 |
| `delete()` | `src/service/cloud_docs/bitable/v1/app_table_view/delete.rs` | 22 |
| `delete_app_role()` | `src/service/cloud_docs/bitable/v1/app_role/delete.rs` | 93 |
| `delete_field()` | `src/service/cloud_docs/bitable/v1/app_table_field/delete.rs` | 101 |
| `delete_record()` | `src/service/cloud_docs/bitable/v1/app_table_record/delete.rs` | 104 |
| `delete_role_member()` | `src/service/cloud_docs/bitable/v1/app_role_member/delete.rs` | 118 |
| `delete_view()` | `src/service/cloud_docs/bitable/v1/app_table_view/delete.rs` | 32 |
| `get()` | `src/service/cloud_docs/bitable/v1/app/get.rs` | 23 |
| `get()` | `src/service/cloud_docs/bitable/v1/app_table_view/get.rs` | 21 |
| `get_form()` | `src/service/cloud_docs/bitable/v1/form/get.rs` | 114 |
| `list()` | `src/service/cloud_docs/bitable/v1/app_role_member/list.rs` | 21 |
| `list()` | `src/service/cloud_docs/bitable/v1/app_table/list.rs` | 18 |
| `list()` | `src/service/cloud_docs/bitable/v1/app_dashboard/list.rs` | 127 |
| `list()` | `src/service/cloud_docs/bitable/v1/app_table_view/list.rs` | 18 |
| `list_app_roles()` | `src/service/cloud_docs/bitable/v1/app_role/list.rs` | 117 |
| `list_dashboard()` | `src/service/cloud_docs/bitable/v1/app_dashboard/list.rs` | 143 |
| `list_field()` | `src/service/cloud_docs/bitable/v1/app_table_field/list.rs` | 157 |
| `list_form_questions()` | `src/service/cloud_docs/bitable/v1/form/list.rs` | 145 |
| `list_workflows()` | `src/service/cloud_docs/bitable/v1/app_workflow/list.rs` | 139 |
| `patch()` | `src/service/cloud_docs/bitable/v1/app_table/patch.rs` | 21 |
| `patch()` | `src/service/cloud_docs/bitable/v1/app_table_view/patch.rs` | 21 |
| `patch_form_meta()` | `src/service/cloud_docs/bitable/v1/form/patch_meta.rs` | 155 |
| `patch_form_question()` | `src/service/cloud_docs/bitable/v1/form/patch.rs` | 152 |
| `search_record()` | `src/service/cloud_docs/bitable/v1/app_table_record/search.rs` | 214 |
| `update()` | `src/service/cloud_docs/bitable/v1/app/update.rs` | 23 |
| `update_app_role()` | `src/service/cloud_docs/bitable/v1/app_role/update.rs` | 120 |
| `update_field()` | `src/service/cloud_docs/bitable/v1/app_table_field/update.rs` | 165 |
| `update_record()` | `src/service/cloud_docs/bitable/v1/app_table_record/update.rs` | 139 |
| `update_workflow()` | `src/service/cloud_docs/bitable/v1/app_workflow/update.rs` | 115 |

#### board

| Method | File | Line |
|--------|------|------|
| `get_thumbnail()` | `src/service/cloud_docs/board/v1/whiteboard.rs` | 30 |
| `list_nodes()` | `src/service/cloud_docs/board/mod.rs` | 35 |
| `list_whiteboard_nodes()` | `src/service/cloud_docs/board/v1/whiteboard_node/list.rs` | 228 |

#### comments

| Method | File | Line |
|--------|------|------|
| `batch_query()` | `src/service/cloud_docs/comments/mod.rs` | 55 |
| `batch_query_comments()` | `src/service/cloud_docs/comments/batch_query.rs` | 172 |
| `create()` | `src/service/cloud_docs/comments/mod.rs` | 73 |
| `create_comment()` | `src/service/cloud_docs/comments/create.rs` | 234 |
| `delete_reply()` | `src/service/cloud_docs/comments/mod.rs` | 109 |
| `delete_reply()` | `src/service/cloud_docs/comments/delete_reply.rs` | 181 |
| `get()` | `src/service/cloud_docs/comments/mod.rs` | 82 |
| `get_comment()` | `src/service/cloud_docs/comments/get.rs` | 159 |
| `list()` | `src/service/cloud_docs/comments/mod.rs` | 46 |
| `list_comments()` | `src/service/cloud_docs/comments/list.rs` | 267 |
| `list_replies()` | `src/service/cloud_docs/comments/list_replies.rs` | 181 |
| `list_replies()` | `src/service/cloud_docs/comments/mod.rs` | 91 |
| `patch()` | `src/service/cloud_docs/comments/mod.rs` | 64 |
| `patch_comment()` | `src/service/cloud_docs/comments/patch.rs` | 203 |
| `update_reply()` | `src/service/cloud_docs/comments/mod.rs` | 100 |
| `update_reply()` | `src/service/cloud_docs/comments/update_reply.rs` | 195 |

#### docx

| Method | File | Line |
|--------|------|------|
| `batch_delete()` | `src/service/cloud_docs/docx/v1/document_block.rs` | 129 |
| `batch_update()` | `src/service/cloud_docs/docx/v1/document_block.rs` | 105 |
| `convert_to_docx()` | `src/service/cloud_docs/docx/v1/document.rs` | 128 |
| `create()` | `src/service/cloud_docs/docx/v1/document_block.rs` | 32 |
| `create()` | `src/service/cloud_docs/docx/v1/document.rs` | 33 |
| `execute_with_options()` | `src/service/cloud_docs/docx/v1/document_block.rs` | 558 |
| `get()` | `src/service/cloud_docs/docx/v1/document_block.rs` | 55 |
| `get()` | `src/service/cloud_docs/docx/v1/document.rs` | 55 |
| `get_raw_content()` | `src/service/cloud_docs/docx/v1/document.rs` | 76 |
| `list_blocks()` | `src/service/cloud_docs/docx/v1/document.rs` | 97 |
| `list_children()` | `src/service/cloud_docs/docx/v1/document_block.rs` | 156 |
| `patch()` | `src/service/cloud_docs/docx/v1/document_block.rs` | 79 |

#### drive

| Method | File | Line |
|--------|------|------|
| `batch_get_tmp_download_url()` | `src/service/cloud_docs/drive/v1/media.rs` | 153 |
| `check_async_task()` | `src/service/cloud_docs/drive/v1/folder.rs` | 161 |
| `copy_file()` | `src/service/cloud_docs/drive/v1/file.rs` | 126 |
| `create_file()` | `src/service/cloud_docs/drive/v1/file.rs` | 105 |
| `create_file_shortcut()` | `src/service/cloud_docs/drive/v1/file.rs` | 174 |
| `create_folder()` | `src/service/cloud_docs/drive/v1/folder.rs` | 111 |
| `create_folder()` | `src/service/cloud_docs/drive/v2/explorer.rs` | 71 |
| `create_import_task()` | `src/service/cloud_docs/drive/v1/file.rs` | 299 |
| `create_version()` | `src/service/cloud_docs/drive/v1/file_version.rs` | 33 |
| `delete_file()` | `src/service/cloud_docs/drive/v1/file.rs` | 154 |
| `delete_version()` | `src/service/cloud_docs/drive/v1/file_version.rs` | 60 |
| `download()` | `src/service/cloud_docs/drive/v1/media.rs` | 132 |
| `download()` | `src/service/cloud_docs/drive/v1/files.rs` | 277 |
| `folder_meta()` | `src/service/cloud_docs/drive/v2/explorer.rs` | 52 |
| `get()` | `src/service/cloud_docs/drive/v1/permissions.rs` | 28 |
| `get_file_meta()` | `src/service/cloud_docs/drive/v1/file.rs` | 34 |
| `get_file_statistics()` | `src/service/cloud_docs/drive/v1/file.rs` | 55 |
| `get_file_subscription()` | `src/service/cloud_docs/drive/v1/event.rs` | 55 |
| `get_folder_meta()` | `src/service/cloud_docs/drive/v1/folder.rs` | 90 |
| `get_import_task()` | `src/service/cloud_docs/drive/v1/file.rs` | 322 |
| `get_root_folder_meta()` | `src/service/cloud_docs/drive/v1/folder.rs` | 33 |
| `get_version()` | `src/service/cloud_docs/drive/v1/file_version.rs` | 83 |
| `list_file_likes()` | `src/service/cloud_docs/drive/v1/like.rs` | 33 |
| `list_file_view_records()` | `src/service/cloud_docs/drive/v1/file.rs` | 75 |
| `list_files()` | `src/service/cloud_docs/drive/v1/folder.rs` | 53 |
| `list_folder()` | `src/service/cloud_docs/drive/v2/explorer.rs` | 89 |
| `list_versions()` | `src/service/cloud_docs/drive/v1/file_version.rs` | 106 |
| `move_or_delete_folder()` | `src/service/cloud_docs/drive/v1/folder.rs` | 133 |
| `patch()` | `src/service/cloud_docs/drive/v1/permissions.rs` | 44 |
| `root_folder_meta()` | `src/service/cloud_docs/drive/v2/explorer.rs` | 33 |
| `search_files()` | `src/service/cloud_docs/drive/v1/file.rs` | 195 |
| `subscribe_file_events()` | `src/service/cloud_docs/drive/v1/event.rs` | 33 |
| `unsubscribe_file_events()` | `src/service/cloud_docs/drive/v1/event.rs` | 78 |
| `upload_all()` | `src/service/cloud_docs/drive/v1/media.rs` | 50 |
| `upload_all()` | `src/service/cloud_docs/drive/v1/files.rs` | 260 |
| `upload_all_with_builder()` | `src/service/cloud_docs/drive/v1/media.rs` | 36 |
| `upload_all_with_builder()` | `src/service/cloud_docs/drive/v1/files.rs` | 250 |
| `upload_finish()` | `src/service/cloud_docs/drive/v1/media.rs` | 110 |
| `upload_finish()` | `src/service/cloud_docs/drive/v1/file.rs` | 276 |
| `upload_part()` | `src/service/cloud_docs/drive/v1/media.rs` | 91 |
| `upload_part()` | `src/service/cloud_docs/drive/v1/file.rs` | 256 |
| `upload_prepare()` | `src/service/cloud_docs/drive/v1/media.rs` | 69 |
| `upload_prepare()` | `src/service/cloud_docs/drive/v1/file.rs` | 233 |

#### permission

| Method | File | Line |
|--------|------|------|
| `auth_permission()` | `src/service/cloud_docs/permission/mod.rs` | 51 |
| `auth_permission()` | `src/service/cloud_docs/permission/member/auth.rs` | 161 |
| `batch_create_member()` | `src/service/cloud_docs/permission/mod.rs` | 33 |
| `batch_create_permission_member()` | `src/service/cloud_docs/permission/member/batch_create.rs` | 222 |
| `create_member()` | `src/service/cloud_docs/permission/mod.rs` | 69 |
| `create_password()` | `src/service/cloud_docs/permission/mod.rs` | 114 |
| `create_password()` | `src/service/cloud_docs/permission/public_v1/password/create.rs` | 165 |
| `create_permission_member()` | `src/service/cloud_docs/permission/member/create.rs` | 253 |
| `delete_member()` | `src/service/cloud_docs/permission/mod.rs` | 87 |
| `delete_password()` | `src/service/cloud_docs/permission/mod.rs` | 132 |
| `delete_password()` | `src/service/cloud_docs/permission/public_v1/password/delete.rs` | 138 |
| `delete_permission_member()` | `src/service/cloud_docs/permission/member/delete.rs` | 202 |
| `get_permission_public()` | `src/service/cloud_docs/permission/mod.rs` | 96 |
| `get_permission_public()` | `src/service/cloud_docs/permission/public_v1/get.rs` | 146 |
| `get_permission_public_v2()` | `src/service/cloud_docs/permission/mod.rs` | 141 |
| `get_permission_public_v2()` | `src/service/cloud_docs/permission/public_v2/get.rs` | 162 |
| `list_members()` | `src/service/cloud_docs/permission/mod.rs` | 60 |
| `list_permission_members()` | `src/service/cloud_docs/permission/member/list.rs` | 157 |
| `patch_permission_public()` | `src/service/cloud_docs/permission/mod.rs` | 105 |
| `patch_permission_public()` | `src/service/cloud_docs/permission/public_v1/patch.rs` | 285 |
| `patch_permission_public_v2()` | `src/service/cloud_docs/permission/mod.rs` | 150 |
| `patch_permission_public_v2()` | `src/service/cloud_docs/permission/public_v2/patch.rs` | 396 |
| `transfer_owner()` | `src/service/cloud_docs/permission/mod.rs` | 42 |
| `transfer_owner()` | `src/service/cloud_docs/permission/member/transfer_owner.rs` | 205 |
| `update_member()` | `src/service/cloud_docs/permission/mod.rs` | 78 |
| `update_password()` | `src/service/cloud_docs/permission/mod.rs` | 123 |
| `update_password()` | `src/service/cloud_docs/permission/public_v1/password/update.rs` | 177 |
| `update_permission_member()` | `src/service/cloud_docs/permission/member/update.rs` | 249 |

#### sheets

| Method | File | Line |
|--------|------|------|
| `add_dimension_range()` | `src/service/cloud_docs/sheets/v2/sheet_row_col/add_dimension_range.rs` | 84 |
| `add_protect_range()` | `src/service/cloud_docs/sheets/v3/protect_range/create.rs` | 20 |
| `add_rows_or_columns()` | `src/service/cloud_docs/sheets/v3/sheet_row_col/add_rows_or_columns.rs` | 19 |
| `append_data()` | `src/service/cloud_docs/sheets/v3/data_operation/append_data.rs` | 23 |
| `append_data()` | `src/service/cloud_docs/sheets/v2/data_operation/append_data.rs` | 81 |
| `batch_set_cell_style()` | `src/service/cloud_docs/sheets/v3/data_operation/batch_set_cell_style.rs` | 23 |
| `batch_set_cell_style()` | `src/service/cloud_docs/sheets/v2/data_operation/batch_set_cell_style.rs` | 90 |
| `create()` | `src/service/cloud_docs/sheets/v3/spreadsheet/create.rs` | 22 |
| `create()` | `src/service/cloud_docs/sheets/v3/spreadsheet_sheet_filter/create.rs` | 85 |
| `create()` | `src/service/cloud_docs/sheets/v3/spreadsheet_sheet_filter_view/create.rs` | 20 |
| `create_condition()` | `src/service/cloud_docs/sheets/v3/spreadsheet_sheet_filter_view_condition/create.rs` | 19 |
| `create_condition_formats()` | `src/service/cloud_docs/sheets/v3/condition_format/create.rs` | 20 |
| `create_float_image()` | `src/service/cloud_docs/sheets/v3/float_image/create.rs` | 20 |
| `delete()` | `src/service/cloud_docs/sheets/v3/spreadsheet_sheet_filter/delete.rs` | 52 |
| `delete()` | `src/service/cloud_docs/sheets/v3/spreadsheet_sheet_filter_view/delete.rs` | 19 |
| `delete_condition()` | `src/service/cloud_docs/sheets/v3/spreadsheet_sheet_filter_view_condition/delete.rs` | 19 |
| `delete_condition_formats()` | `src/service/cloud_docs/sheets/v3/condition_format/delete.rs` | 19 |
| `delete_data_validation()` | `src/service/cloud_docs/sheets/v3/data_validation/delete.rs` | 19 |
| `delete_dimension_range()` | `src/service/cloud_docs/sheets/v2/sheet_row_col/delete_dimension_range.rs` | 88 |
| `delete_float_image()` | `src/service/cloud_docs/sheets/v3/float_image/delete.rs` | 20 |
| `delete_protect_range()` | `src/service/cloud_docs/sheets/v3/protect_range/delete.rs` | 19 |
| `delete_rows_or_columns()` | `src/service/cloud_docs/sheets/v3/sheet_row_col/delete_rows_or_columns.rs` | 22 |
| `find_cells()` | `src/service/cloud_docs/sheets/v3/data_operation/find_cells.rs` | 236 |
| `get()` | `src/service/cloud_docs/sheets/v3/spreadsheet/get.rs` | 22 |
| `get()` | `src/service/cloud_docs/sheets/v3/spreadsheet_sheet_filter/get.rs` | 87 |
| `get()` | `src/service/cloud_docs/sheets/v3/spreadsheet_sheet/get.rs` | 55 |
| `get()` | `src/service/cloud_docs/sheets/v3/spreadsheet_sheet_filter_view/get.rs` | 21 |
| `get_condition()` | `src/service/cloud_docs/sheets/v3/spreadsheet_sheet_filter_view_condition/get.rs` | 21 |
| `get_condition_formats()` | `src/service/cloud_docs/sheets/v3/condition_format/get.rs` | 21 |
| `get_float_image()` | `src/service/cloud_docs/sheets/v3/float_image/get.rs` | 22 |
| `get_protect_ranges()` | `src/service/cloud_docs/sheets/v3/protect_range/get.rs` | 21 |
| `insert_dimension_range()` | `src/service/cloud_docs/sheets/v2/sheet_row_col/insert_dimension_range.rs` | 104 |
| `insert_rows_or_columns()` | `src/service/cloud_docs/sheets/v3/sheet_row_col/insert_rows_or_columns.rs` | 20 |
| `merge_cells()` | `src/service/cloud_docs/sheets/v3/data_operation/merge_cells.rs` | 22 |
| `merge_cells()` | `src/service/cloud_docs/sheets/v2/data_operation/merge_cells.rs` | 80 |
| `move_dimension()` | `src/service/cloud_docs/sheets/v3/sheet_row_col/move_dimension.rs` | 113 |
| `operate()` | `src/service/cloud_docs/sheets/v2/spreadsheet_sheet/operate_sheets.rs` | 155 |
| `patch()` | `src/service/cloud_docs/sheets/v3/spreadsheet/patch.rs` | 20 |
| `patch()` | `src/service/cloud_docs/sheets/v3/spreadsheet_sheet_filter_view/patch.rs` | 19 |
| `prepend_data()` | `src/service/cloud_docs/sheets/v3/data_operation/prepend_data.rs` | 23 |
| `prepend_data()` | `src/service/cloud_docs/sheets/v2/data_operation/prepend_data.rs` | 68 |
| `query()` | `src/service/cloud_docs/sheets/v3/spreadsheet_sheet/query.rs` | 28 |
| `query()` | `src/service/cloud_docs/sheets/v3/spreadsheet_sheet_filter_view/query.rs` | 19 |
| `query_conditions()` | `src/service/cloud_docs/sheets/v3/spreadsheet_sheet_filter_view_condition/query.rs` | 21 |
| `query_data_validations()` | `src/service/cloud_docs/sheets/v3/data_validation/query.rs` | 21 |
| `query_float_images()` | `src/service/cloud_docs/sheets/v3/float_image/query.rs` | 22 |
| `reading_a_single_range()` | `src/service/cloud_docs/sheets/v2/data_operation/reading_a_single_range.rs` | 158 |
| `reading_multi_ranges()` | `src/service/cloud_docs/sheets/v2/data_operation/reading_multiple_range.rs` | 157 |
| `reading_multiple_ranges()` | `src/service/cloud_docs/sheets/v3/data_operation/reading_multiple_ranges.rs` | 24 |
| `reading_single_range()` | `src/service/cloud_docs/sheets/v3/data_operation/reading_single_range.rs` | 24 |
| `replace_cells()` | `src/service/cloud_docs/sheets/v3/data_operation/replace_cells.rs` | 133 |
| `set_cell_style()` | `src/service/cloud_docs/sheets/v3/data_operation/set_cell_style.rs` | 21 |
| `set_cell_style()` | `src/service/cloud_docs/sheets/v2/data_operation/set_cell_style.rs` | 127 |
| `set_data_validation()` | `src/service/cloud_docs/sheets/v3/data_validation/create.rs` | 20 |
| `split_cells()` | `src/service/cloud_docs/sheets/v3/data_operation/split_cells.rs` | 21 |
| `split_cells()` | `src/service/cloud_docs/sheets/v2/data_operation/split_cells.rs` | 69 |
| `update()` | `src/service/cloud_docs/sheets/v3/spreadsheet_sheet_filter/update.rs` | 76 |
| `update_condition()` | `src/service/cloud_docs/sheets/v3/spreadsheet_sheet_filter_view_condition/update.rs` | 21 |
| `update_condition_formats()` | `src/service/cloud_docs/sheets/v3/condition_format/update.rs` | 21 |
| `update_data_validation()` | `src/service/cloud_docs/sheets/v3/data_validation/update.rs` | 21 |
| `update_dimension_range()` | `src/service/cloud_docs/sheets/v2/sheet_row_col/update_dimension_range.rs` | 114 |
| `update_float_image()` | `src/service/cloud_docs/sheets/v3/float_image/patch.rs` | 22 |
| `update_protect_range()` | `src/service/cloud_docs/sheets/v3/protect_range/update.rs` | 21 |
| `update_rows_or_columns()` | `src/service/cloud_docs/sheets/v3/sheet_row_col/update_rows_or_columns.rs` | 21 |
| `update_sheet_properties()` | `src/service/cloud_docs/sheets/v2/spreadsheet_sheet/update_sheet_properties.rs` | 124 |
| `write_data_multi_ranges()` | `src/service/cloud_docs/sheets/v2/data_operation/write_data_to_multi_ranges.rs` | 104 |
| `write_data_to_multiple_ranges()` | `src/service/cloud_docs/sheets/v3/data_operation/write_data_to_multiple_ranges.rs` | 22 |
| `write_data_to_single_range()` | `src/service/cloud_docs/sheets/v2/data_operation/write_data_to_a_single_range.rs` | 73 |
| `write_image()` | `src/service/cloud_docs/sheets/v2/data_operation/write_image.rs` | 90 |
| `write_images()` | `src/service/cloud_docs/sheets/v3/data_operation/write_images.rs` | 21 |

#### wiki

| Method | File | Line |
|--------|------|------|
| `copy()` | `src/service/cloud_docs/wiki/v2/space_node/mod.rs` | 100 |
| `copy_space_node()` | `src/service/cloud_docs/wiki/v2/space_node/copy.rs` | 155 |
| `create()` | `src/service/cloud_docs/wiki/v2/space_node/mod.rs` | 30 |
| `create_space()` | `src/service/cloud_docs/wiki/v2/space/create.rs` | 105 |
| `create_space_member()` | `src/service/cloud_docs/wiki/v2/space_member/create.rs` | 142 |
| `create_space_node()` | `src/service/cloud_docs/wiki/v2/space_node/create.rs` | 172 |
| `delete_space_member()` | `src/service/cloud_docs/wiki/v2/space_member/delete.rs` | 96 |
| `get()` | `src/service/cloud_docs/wiki/v2/space_node/mod.rs` | 44 |
| `get()` | `src/service/cloud_docs/wiki/v2/task/mod.rs` | 36 |
| `get_space()` | `src/service/cloud_docs/wiki/v2/space/get.rs` | 89 |
| `get_space_node()` | `src/service/cloud_docs/wiki/v2/space_node/get.rs` | 109 |
| `get_task()` | `src/service/cloud_docs/wiki/v2/task/get.rs` | 129 |
| `list()` | `src/service/cloud_docs/wiki/v2/space_node/mod.rs` | 58 |
| `list_space_members()` | `src/service/cloud_docs/wiki/v2/space_member/list.rs` | 117 |
| `list_space_node()` | `src/service/cloud_docs/wiki/v2/space_node/list.rs` | 147 |
| `list_spaces()` | `src/service/cloud_docs/wiki/v2/space/list.rs` | 108 |
| `move_docs_to_wiki()` | `src/service/cloud_docs/wiki/v2/task/mod.rs` | 22 |
| `move_docs_to_wiki()` | `src/service/cloud_docs/wiki/v2/task/move_docs_to_wiki.rs` | 114 |
| `move_space_node()` | `src/service/cloud_docs/wiki/v2/space_node/move.rs` | 129 |
| `search_wiki()` | `src/service/cloud_docs/wiki/v2/search_wiki.rs` | 153 |
| `search_wiki()` | `src/service/cloud_docs/wiki/v2/mod.rs` | 114 |
| `update_space_node_title()` | `src/service/cloud_docs/wiki/v2/space_node/update_title.rs` | 108 |
| `update_space_setting()` | `src/service/cloud_docs/wiki/v2/space_setting/update.rs` | 123 |
| `update_title()` | `src/service/cloud_docs/wiki/v2/space_node/mod.rs` | 86 |

### corehr

#### basic_info

| Method | File | Line |
|--------|------|------|
| `convert_id()` | `src/service/corehr/basic_info/mod.rs` | 257 |
| `search_country_region()` | `src/service/corehr/basic_info/mod.rs` | 164 |
| `search_enum()` | `src/service/corehr/basic_info/mod.rs` | 117 |
| `search_nationality()` | `src/service/corehr/basic_info/mod.rs` | 210 |

#### employee

| Method | File | Line |
|--------|------|------|
| `batch_get()` | `src/service/corehr/employee/mod.rs` | 101 |
| `search()` | `src/service/corehr/employee/mod.rs` | 165 |

#### job_management

| Method | File | Line |
|--------|------|------|
| `create_job()` | `src/service/corehr/job_management/mod.rs` | 494 |
| `create_job_family()` | `src/service/corehr/job_management/mod.rs` | 177 |
| `create_job_grade()` | `src/service/corehr/job_management/mod.rs` | 391 |
| `create_job_level()` | `src/service/corehr/job_management/mod.rs` | 284 |
| `list_job_families()` | `src/service/corehr/job_management/mod.rs` | 219 |
| `list_job_levels()` | `src/service/corehr/job_management/mod.rs` | 326 |
| `list_jobs()` | `src/service/corehr/job_management/mod.rs` | 536 |
| `query_job_grades()` | `src/service/corehr/job_management/mod.rs` | 434 |

#### lifecycle

| Method | File | Line |
|--------|------|------|
| `create_job_change()` | `src/service/corehr/lifecycle/mod.rs` | 272 |
| `create_offboarding()` | `src/service/corehr/lifecycle/mod.rs` | 376 |
| `create_pre_hire()` | `src/service/corehr/lifecycle/mod.rs` | 163 |
| `search_job_change()` | `src/service/corehr/lifecycle/mod.rs` | 325 |
| `search_offboarding()` | `src/service/corehr/lifecycle/mod.rs` | 429 |
| `search_pre_hire()` | `src/service/corehr/lifecycle/mod.rs` | 215 |

#### organization

| Method | File | Line |
|--------|------|------|
| `batch_get_departments()` | `src/service/corehr/organization/mod.rs` | 181 |
| `create_company()` | `src/service/corehr/organization/mod.rs` | 292 |
| `create_department()` | `src/service/corehr/organization/mod.rs` | 140 |
| `get_department_tree()` | `src/service/corehr/organization/mod.rs` | 231 |
| `list_companies()` | `src/service/corehr/organization/mod.rs` | 341 |

### directory

#### v1

| Method | File | Line |
|--------|------|------|
| `create()` | `src/service/directory/v1/department/create.rs` | 186 |
| `create()` | `src/service/directory/v1/employee/create.rs` | 259 |
| `delete()` | `src/service/directory/v1/department/delete.rs` | 111 |
| `delete()` | `src/service/directory/v1/employee/delete.rs` | 158 |
| `filter()` | `src/service/directory/v1/department/filter.rs` | 186 |
| `filter()` | `src/service/directory/v1/employee/filter.rs` | 188 |
| `mget()` | `src/service/directory/v1/department/mget.rs` | 142 |
| `mget()` | `src/service/directory/v1/employee/mget.rs` | 137 |
| `patch()` | `src/service/directory/v1/department/patch.rs` | 199 |
| `patch()` | `src/service/directory/v1/employee/patch.rs` | 254 |
| `regular()` | `src/service/directory/v1/employee/regular.rs` | 117 |
| `resurrect()` | `src/service/directory/v1/employee/resurrect.rs` | 194 |
| `search()` | `src/service/directory/v1/department/search.rs` | 168 |
| `search()` | `src/service/directory/v1/employee/search.rs` | 181 |
| `to_be_resigned()` | `src/service/directory/v1/employee/to_be_resigned.rs` | 156 |

### ehr

#### attachment

| Method | File | Line |
|--------|------|------|
| `download_attachment()` | `src/service/ehr/attachment/mod.rs` | 89 |

#### employee

| Method | File | Line |
|--------|------|------|
| `list_employees()` | `src/service/ehr/employee/mod.rs` | 97 |

### elearning

#### course_registration

| Method | File | Line |
|--------|------|------|
| `create()` | `src/service/elearning/course_registration/mod.rs` | 41 |
| `delete()` | `src/service/elearning/course_registration/mod.rs` | 204 |
| `get()` | `src/service/elearning/course_registration/mod.rs` | 138 |
| `get_statistics()` | `src/service/elearning/course_registration/mod.rs` | 236 |
| `list()` | `src/service/elearning/course_registration/mod.rs` | 69 |
| `update()` | `src/service/elearning/course_registration/mod.rs` | 171 |

### helpdesk

#### v1

| Method | File | Line |
|--------|------|------|
| `agent_email()` | `src/service/helpdesk/v1/agent/mod.rs` | 121 |
| `cancel_approve()` | `src/service/helpdesk/v1/notification/mod.rs` | 248 |
| `cancel_send()` | `src/service/helpdesk/v1/notification/mod.rs` | 302 |
| `create()` | `src/service/helpdesk/v1/category/mod.rs` | 115 |
| `create()` | `src/service/helpdesk/v1/notification/mod.rs` | 114 |
| `create()` | `src/service/helpdesk/v1/faq/mod.rs` | 156 |
| `create()` | `src/service/helpdesk/v1/agent_skill/mod.rs` | 85 |
| `create()` | `src/service/helpdesk/v1/agent_schedule/mod.rs` | 114 |
| `create()` | `src/service/helpdesk/v1/ticket_customized_field/mod.rs` | 137 |
| `create()` | `src/service/helpdesk/v1/ticket_message/mod.rs` | 110 |
| `create_group_message()` | `src/service/helpdesk/v1/ticket_message/mod.rs` | 206 |
| `delete()` | `src/service/helpdesk/v1/category/mod.rs` | 195 |
| `delete()` | `src/service/helpdesk/v1/faq/mod.rs` | 194 |
| `delete()` | `src/service/helpdesk/v1/agent_skill/mod.rs` | 109 |
| `delete()` | `src/service/helpdesk/v1/agent_schedule/mod.rs` | 143 |
| `delete()` | `src/service/helpdesk/v1/ticket_customized_field/mod.rs` | 175 |
| `execute_send()` | `src/service/helpdesk/v1/notification/mod.rs` | 275 |
| `faq_image()` | `src/service/helpdesk/v1/faq/mod.rs` | 363 |
| `get()` | `src/service/helpdesk/v1/category/mod.rs` | 139 |
| `get()` | `src/service/helpdesk/v1/notification/mod.rs` | 167 |
| `get()` | `src/service/helpdesk/v1/faq/mod.rs` | 271 |
| `get()` | `src/service/helpdesk/v1/agent_skill/mod.rs` | 136 |
| `get()` | `src/service/helpdesk/v1/ticket/mod.rs` | 143 |
| `get()` | `src/service/helpdesk/v1/agent_schedule/mod.rs` | 209 |
| `get()` | `src/service/helpdesk/v1/ticket_customized_field/mod.rs` | 260 |
| `list()` | `src/service/helpdesk/v1/category/mod.rs` | 222 |
| `list()` | `src/service/helpdesk/v1/faq/mod.rs` | 310 |
| `list()` | `src/service/helpdesk/v1/agent_skill/mod.rs` | 163 |
| `list()` | `src/service/helpdesk/v1/ticket/mod.rs` | 199 |
| `list()` | `src/service/helpdesk/v1/agent_schedule/mod.rs` | 241 |
| `list()` | `src/service/helpdesk/v1/agent_skill_rule/mod.rs` | 114 |
| `list()` | `src/service/helpdesk/v1/ticket_customized_field/mod.rs` | 301 |
| `list()` | `src/service/helpdesk/v1/ticket_message/mod.rs` | 155 |
| `operator_options()` | `src/service/helpdesk/v1/agent_skill_rule/mod.rs` | 78 |
| `patch()` | `src/service/helpdesk/v1/category/mod.rs` | 166 |
| `patch()` | `src/service/helpdesk/v1/notification/mod.rs` | 138 |
| `patch()` | `src/service/helpdesk/v1/faq/mod.rs` | 232 |
| `patch()` | `src/service/helpdesk/v1/agent/mod.rs` | 82 |
| `patch()` | `src/service/helpdesk/v1/agent_schedule/mod.rs` | 175 |
| `preview()` | `src/service/helpdesk/v1/notification/mod.rs` | 194 |
| `search()` | `src/service/helpdesk/v1/faq/mod.rs` | 407 |
| `start_service()` | `src/service/helpdesk/v1/ticket/mod.rs` | 119 |
| `submit_approve()` | `src/service/helpdesk/v1/notification/mod.rs` | 221 |
| `subscribe()` | `src/service/helpdesk/v1/event/mod.rs` | 73 |
| `unsubscribe()` | `src/service/helpdesk/v1/event/mod.rs` | 111 |
| `update()` | `src/service/helpdesk/v1/ticket/mod.rs` | 170 |
| `update()` | `src/service/helpdesk/v1/ticket_customized_field/mod.rs` | 217 |

### hire

#### attachment

| Method | File | Line |
|--------|------|------|
| `batch_delete()` | `src/service/hire/attachment/mod.rs` | 661 |
| `batch_download()` | `src/service/hire/attachment/mod.rs` | 624 |
| `create_upload_task()` | `src/service/hire/attachment/mod.rs` | 260 |
| `delete_attachment()` | `src/service/hire/attachment/mod.rs` | 494 |
| `get_attachment_detail()` | `src/service/hire/attachment/mod.rs` | 306 |
| `get_attachment_statistics()` | `src/service/hire/attachment/mod.rs` | 710 |
| `get_download_url()` | `src/service/hire/attachment/mod.rs` | 530 |
| `get_preview_url()` | `src/service/hire/attachment/mod.rs` | 566 |
| `list_attachments()` | `src/service/hire/attachment/mod.rs` | 374 |
| `update_attachment()` | `src/service/hire/attachment/mod.rs` | 457 |

#### candidate_management

| Method | File | Line |
|--------|------|------|
| `add_application_evaluation()` | `src/service/hire/candidate_management/application/mod.rs` | 631 |
| `add_talent_to_pool()` | `src/service/hire/candidate_management/talent_pool/mod.rs` | 484 |
| `advance_application()` | `src/service/hire/candidate_management/application/mod.rs` | 364 |
| `arrange_interview()` | `src/service/hire/candidate_management/interview/mod.rs` | 510 |
| `batch_import_talents()` | `src/service/hire/candidate_management/talent/mod.rs` | 535 |
| `batch_import_talents_with_builder()` | `src/service/hire/candidate_management/talent/mod.rs` | 661 |
| `cancel_interview()` | `src/service/hire/candidate_management/interview/mod.rs` | 659 |
| `create_application()` | `src/service/hire/candidate_management/application/mod.rs` | 168 |
| `create_interview()` | `src/service/hire/candidate_management/interview/mod.rs` | 310 |
| `create_offer()` | `src/service/hire/candidate_management/offer/mod.rs` | 340 |
| `create_offer()` | `src/service/hire/candidate_management/application/mod.rs` | 540 |
| `create_onboarding()` | `src/service/hire/candidate_management/offer/mod.rs` | 685 |
| `create_pool()` | `src/service/hire/candidate_management/talent_pool/mod.rs` | 228 |
| `create_talent()` | `src/service/hire/candidate_management/talent/mod.rs` | 175 |
| `create_talent_with_builder()` | `src/service/hire/candidate_management/talent/mod.rs` | 587 |
| `delete_pool()` | `src/service/hire/candidate_management/talent_pool/mod.rs` | 608 |
| `delete_talent()` | `src/service/hire/candidate_management/talent/mod.rs` | 418 |
| `get_application_detail()` | `src/service/hire/candidate_management/application/mod.rs` | 216 |
| `get_application_interviews()` | `src/service/hire/candidate_management/application/mod.rs` | 479 |
| `get_application_offer()` | `src/service/hire/candidate_management/application/mod.rs` | 586 |
| `get_interview_detail()` | `src/service/hire/candidate_management/interview/mod.rs` | 357 |
| `get_offer_detail()` | `src/service/hire/candidate_management/offer/mod.rs` | 386 |
| `get_pool_detail()` | `src/service/hire/candidate_management/talent_pool/mod.rs` | 275 |
| `get_talent_application_history()` | `src/service/hire/candidate_management/talent/mod.rs` | 470 |
| `get_talent_detail()` | `src/service/hire/candidate_management/talent/mod.rs` | 224 |
| `list_applications()` | `src/service/hire/candidate_management/application/mod.rs` | 285 |
| `list_interview_evaluations()` | `src/service/hire/candidate_management/interview/mod.rs` | 603 |
| `list_interviews()` | `src/service/hire/candidate_management/interview/mod.rs` | 425 |
| `list_offers()` | `src/service/hire/candidate_management/offer/mod.rs` | 450 |
| `list_onboardings()` | `src/service/hire/candidate_management/offer/mod.rs` | 725 |
| `list_pool_talents()` | `src/service/hire/candidate_management/talent_pool/mod.rs` | 420 |
| `list_pools()` | `src/service/hire/candidate_management/talent_pool/mod.rs` | 335 |
| `list_talents()` | `src/service/hire/candidate_management/talent/mod.rs` | 291 |
| `list_talents_with_builder()` | `src/service/hire/candidate_management/talent/mod.rs` | 746 |
| `reject_application()` | `src/service/hire/candidate_management/application/mod.rs` | 410 |
| `remove_talent_from_pool()` | `src/service/hire/candidate_management/talent_pool/mod.rs` | 523 |
| `reschedule_interview()` | `src/service/hire/candidate_management/interview/mod.rs` | 715 |
| `send_offer()` | `src/service/hire/candidate_management/offer/mod.rs` | 582 |
| `submit_interview_evaluation()` | `src/service/hire/candidate_management/interview/mod.rs` | 564 |
| `update_offer()` | `src/service/hire/candidate_management/offer/mod.rs` | 539 |
| `update_onboarding_progress()` | `src/service/hire/candidate_management/offer/mod.rs` | 792 |
| `update_pool()` | `src/service/hire/candidate_management/talent_pool/mod.rs` | 575 |
| `update_talent()` | `src/service/hire/candidate_management/talent/mod.rs` | 385 |
| `update_talent_with_builder()` | `src/service/hire/candidate_management/talent/mod.rs` | 623 |
| `withdraw_offer()` | `src/service/hire/candidate_management/offer/mod.rs` | 630 |

#### ecological_docking

| Method | File | Line |
|--------|------|------|
| `arrange_exam()` | `src/service/hire/ecological_docking/exam/mod.rs` | 435 |
| `batch_create_orders()` | `src/service/hire/ecological_docking/background_check/mod.rs` | 710 |
| `cancel_exam()` | `src/service/hire/ecological_docking/exam/mod.rs` | 667 |
| `cancel_order()` | `src/service/hire/ecological_docking/background_check/mod.rs` | 616 |
| `create_order()` | `src/service/hire/ecological_docking/background_check/mod.rs` | 438 |
| `get_exam_statistics()` | `src/service/hire/ecological_docking/exam/mod.rs` | 782 |
| `get_order_detail()` | `src/service/hire/ecological_docking/background_check/mod.rs` | 484 |
| `get_record_detail()` | `src/service/hire/ecological_docking/exam/mod.rs` | 481 |
| `get_report()` | `src/service/hire/ecological_docking/background_check/mod.rs` | 662 |
| `list_orders()` | `src/service/hire/ecological_docking/background_check/mod.rs` | 550 |
| `list_packages()` | `src/service/hire/ecological_docking/background_check/mod.rs` | 339 |
| `list_papers()` | `src/service/hire/ecological_docking/exam/mod.rs` | 346 |
| `list_records()` | `src/service/hire/ecological_docking/exam/mod.rs` | 549 |
| `reschedule_exam()` | `src/service/hire/ecological_docking/exam/mod.rs` | 723 |
| `submit_exam()` | `src/service/hire/ecological_docking/exam/mod.rs` | 632 |

#### get_candidates

| Method | File | Line |
|--------|------|------|
| `add_consultant()` | `src/service/hire/get_candidates/agency/mod.rs` | 616 |
| `confirm_recommendation()` | `src/service/hire/get_candidates/agency/mod.rs` | 706 |
| `convert_external_candidate()` | `src/service/hire/get_candidates/external_system/mod.rs` | 679 |
| `convert_website_application()` | `src/service/hire/get_candidates/website/mod.rs` | 656 |
| `create_agency()` | `src/service/hire/get_candidates/agency/mod.rs` | 355 |
| `create_recommendation()` | `src/service/hire/get_candidates/agency/mod.rs` | 495 |
| `create_referral()` | `src/service/hire/get_candidates/referral/mod.rs` | 248 |
| `create_reward_settings()` | `src/service/hire/get_candidates/referral/mod.rs` | 593 |
| `create_sync_task()` | `src/service/hire/get_candidates/external_system/mod.rs` | 415 |
| `create_system_config()` | `src/service/hire/get_candidates/external_system/mod.rs` | 292 |
| `get_referral_account()` | `src/service/hire/get_candidates/referral/mod.rs` | 488 |
| `get_referral_detail()` | `src/service/hire/get_candidates/referral/mod.rs` | 295 |
| `get_website_configuration()` | `src/service/hire/get_candidates/website/mod.rs` | 557 |
| `get_website_job_statistics()` | `src/service/hire/get_candidates/website/mod.rs` | 702 |
| `grant_referral_reward()` | `src/service/hire/get_candidates/referral/mod.rs` | 534 |
| `import_external_candidates()` | `src/service/hire/get_candidates/external_system/mod.rs` | 558 |
| `list_agencies()` | `src/service/hire/get_candidates/agency/mod.rs` | 417 |
| `list_consultants()` | `src/service/hire/get_candidates/agency/mod.rs` | 650 |
| `list_external_candidates()` | `src/service/hire/get_candidates/external_system/mod.rs` | 615 |
| `list_recommendations()` | `src/service/hire/get_candidates/agency/mod.rs` | 538 |
| `list_referrals()` | `src/service/hire/get_candidates/referral/mod.rs` | 363 |
| `list_reward_settings()` | `src/service/hire/get_candidates/referral/mod.rs` | 638 |
| `list_sync_records()` | `src/service/hire/get_candidates/external_system/mod.rs` | 473 |
| `list_system_configs()` | `src/service/hire/get_candidates/external_system/mod.rs` | 347 |
| `list_website_applications()` | `src/service/hire/get_candidates/website/mod.rs` | 469 |
| `list_website_jobs()` | `src/service/hire/get_candidates/website/mod.rs` | 285 |
| `publish_job_to_website()` | `src/service/hire/get_candidates/website/mod.rs` | 368 |
| `register_referral_account()` | `src/service/hire/get_candidates/referral/mod.rs` | 442 |
| `reject_recommendation()` | `src/service/hire/get_candidates/agency/mod.rs` | 760 |
| `test_system_connection()` | `src/service/hire/get_candidates/external_system/mod.rs` | 718 |
| `unpublish_job_from_website()` | `src/service/hire/get_candidates/website/mod.rs` | 400 |
| `update_website_configuration()` | `src/service/hire/get_candidates/website/mod.rs` | 624 |

#### recruitment_config

| Method | File | Line |
|--------|------|------|
| `close_job()` | `src/service/hire/recruitment_config/job/mod.rs` | 350 |
| `create_job()` | `src/service/hire/recruitment_config/job/mod.rs` | 119 |
| `create_process()` | `src/service/hire/recruitment_config/job_process/mod.rs` | 240 |
| `create_requirement()` | `src/service/hire/recruitment_config/job_requirement/mod.rs` | 125 |
| `create_settings()` | `src/service/hire/recruitment_config/offer_settings/mod.rs` | 249 |
| `create_settings()` | `src/service/hire/recruitment_config/interview_settings/mod.rs` | 231 |
| `create_subject()` | `src/service/hire/recruitment_config/subject/mod.rs` | 203 |
| `delete_process()` | `src/service/hire/recruitment_config/job_process/mod.rs` | 452 |
| `delete_requirement()` | `src/service/hire/recruitment_config/job_requirement/mod.rs` | 332 |
| `delete_settings()` | `src/service/hire/recruitment_config/offer_settings/mod.rs` | 465 |
| `delete_settings()` | `src/service/hire/recruitment_config/interview_settings/mod.rs` | 449 |
| `delete_subject()` | `src/service/hire/recruitment_config/subject/mod.rs` | 413 |
| `disable_subject()` | `src/service/hire/recruitment_config/subject/mod.rs` | 481 |
| `enable_subject()` | `src/service/hire/recruitment_config/subject/mod.rs` | 445 |
| `get_job_detail()` | `src/service/hire/recruitment_config/job/mod.rs` | 220 |
| `get_process_detail()` | `src/service/hire/recruitment_config/job_process/mod.rs` | 286 |
| `get_requirement_detail()` | `src/service/hire/recruitment_config/job_requirement/mod.rs` | 171 |
| `get_role_detail()` | `src/service/hire/recruitment_config/auth/mod.rs` | 86 |
| `get_settings_detail()` | `src/service/hire/recruitment_config/offer_settings/mod.rs` | 298 |
| `get_settings_detail()` | `src/service/hire/recruitment_config/interview_settings/mod.rs` | 279 |
| `get_subject_detail()` | `src/service/hire/recruitment_config/subject/mod.rs` | 250 |
| `get_user_roles()` | `src/service/hire/recruitment_config/auth/mod.rs` | 167 |
| `list_jobs()` | `src/service/hire/recruitment_config/job/mod.rs` | 283 |
| `list_locations()` | `src/service/hire/recruitment_config/location/mod.rs` | 119 |
| `list_processes()` | `src/service/hire/recruitment_config/job_process/mod.rs` | 348 |
| `list_registration_forms()` | `src/service/hire/recruitment_config/application.rs` | 116 |
| `list_requirements()` | `src/service/hire/recruitment_config/job_requirement/mod.rs` | 233 |
| `list_roles()` | `src/service/hire/recruitment_config/auth/mod.rs` | 124 |
| `list_settings()` | `src/service/hire/recruitment_config/offer_settings/mod.rs` | 360 |
| `list_settings()` | `src/service/hire/recruitment_config/interview_settings/mod.rs` | 341 |
| `list_subjects()` | `src/service/hire/recruitment_config/subject/mod.rs` | 310 |
| `list_talent_tags()` | `src/service/hire/recruitment_config/application.rs` | 71 |
| `open_job()` | `src/service/hire/recruitment_config/job/mod.rs` | 382 |
| `query_locations()` | `src/service/hire/recruitment_config/location/mod.rs` | 70 |
| `update_job()` | `src/service/hire/recruitment_config/job/mod.rs` | 168 |
| `update_process()` | `src/service/hire/recruitment_config/job_process/mod.rs` | 415 |
| `update_requirement()` | `src/service/hire/recruitment_config/job_requirement/mod.rs` | 295 |
| `update_settings()` | `src/service/hire/recruitment_config/offer_settings/mod.rs` | 428 |
| `update_settings()` | `src/service/hire/recruitment_config/interview_settings/mod.rs` | 412 |
| `update_subject()` | `src/service/hire/recruitment_config/subject/mod.rs` | 380 |

#### referral_account

| Method | File | Line |
|--------|------|------|
| `apply_withdrawal()` | `src/service/hire/referral_account/mod.rs` | 605 |
| `approve_withdrawal()` | `src/service/hire/referral_account/mod.rs` | 740 |
| `create_account()` | `src/service/hire/referral_account/mod.rs` | 321 |
| `disable_account()` | `src/service/hire/referral_account/mod.rs` | 825 |
| `enable_account()` | `src/service/hire/referral_account/mod.rs` | 786 |
| `get_balance()` | `src/service/hire/referral_account/mod.rs` | 445 |
| `get_referral_statistics()` | `src/service/hire/referral_account/mod.rs` | 878 |
| `list_accounts()` | `src/service/hire/referral_account/mod.rs` | 379 |
| `list_income_records()` | `src/service/hire/referral_account/mod.rs` | 511 |
| `list_withdrawal_records()` | `src/service/hire/referral_account/mod.rs` | 667 |

### human_authentication

#### root

| Method | File | Line |
|--------|------|------|
| `create_identity()` | `src/service/human_authentication/mod.rs` | 459 |
| `crop_face_image()` | `src/service/human_authentication/mod.rs` | 569 |
| `query_authentication_result()` | `src/service/human_authentication/mod.rs` | 620 |
| `upload_face_image()` | `src/service/human_authentication/mod.rs` | 512 |

### im

#### v1

| Method | File | Line |
|--------|------|------|
| `batch_update()` | `src/service/im/v1/url_preview/mod.rs` | 58 |
| `create()` | `src/service/im/v1/pin/mod.rs` | 80 |
| `create()` | `src/service/im/v1/file/mod.rs` | 59 |
| `create()` | `src/service/im/v1/message/send.rs` | 19 |
| `create()` | `src/service/im/v1/image/mod.rs` | 57 |
| `create()` | `src/service/im/v1/message_reaction/mod.rs` | 58 |
| `delay_update()` | `src/service/im/v1/message_card/mod.rs` | 97 |
| `delete()` | `src/service/im/v1/pin/mod.rs` | 106 |
| `delete()` | `src/service/im/v1/message/send.rs` | 39 |
| `delete()` | `src/service/im/v1/message_reaction/mod.rs` | 128 |
| `delete()` | `src/service/im/v1/batch_message/mod.rs` | 146 |
| `delete_visible()` | `src/service/im/v1/message_card/mod.rs` | 141 |
| `get()` | `src/service/im/v1/file/mod.rs` | 85 |
| `get()` | `src/service/im/v1/image/mod.rs` | 78 |
| `get_progress()` | `src/service/im/v1/batch_message/mod.rs` | 168 |
| `list()` | `src/service/im/v1/chats.rs` | 22 |
| `list()` | `src/service/im/v1/pin/mod.rs` | 136 |
| `list()` | `src/service/im/v1/message/list.rs` | 206 |
| `list()` | `src/service/im/v1/message_reaction/mod.rs` | 91 |
| `list_with_validated_pagination()` | `src/service/im/v1/message/list.rs` | 236 |
| `patch()` | `src/service/im/v1/message_card/mod.rs` | 75 |
| `read_user()` | `src/service/im/v1/batch_message/mod.rs` | 190 |
| `reply()` | `src/service/im/v1/message/send.rs` | 84 |
| `send_visible()` | `src/service/im/v1/message_card/mod.rs` | 119 |
| `update()` | `src/service/im/v1/message/send.rs` | 60 |
| `urgent_app()` | `src/service/im/v1/buzz_messages/mod.rs` | 69 |
| `urgent_phone()` | `src/service/im/v1/buzz_messages/mod.rs` | 121 |
| `urgent_sms()` | `src/service/im/v1/buzz_messages/mod.rs` | 95 |

#### v2

| Method | File | Line |
|--------|------|------|
| `bot_time_sentive()` | `src/service/im/v2/groups_bots/mod.rs` | 115 |
| `create()` | `src/service/im/v2/app_feed_card/mod.rs` | 85 |
| `delete()` | `src/service/im/v2/app_feed_card/mod.rs` | 124 |
| `patch()` | `src/service/im/v2/groups_bots/mod.rs` | 159 |
| `update()` | `src/service/im/v2/groups_bots/mod.rs` | 137 |
| `update()` | `src/service/im/v2/app_feed_card/mod.rs` | 102 |

### lingo

#### classification

| Method | File | Line |
|--------|------|------|
| `list_classifications()` | `src/service/lingo/classification/mod.rs` | 39 |

#### draft

| Method | File | Line |
|--------|------|------|
| `create_draft()` | `src/service/lingo/draft/mod.rs` | 39 |
| `update_draft()` | `src/service/lingo/draft/mod.rs` | 71 |

#### entity

| Method | File | Line |
|--------|------|------|
| `create_entity()` | `src/service/lingo/entity/mod.rs` | 47 |
| `delete_entity()` | `src/service/lingo/entity/mod.rs` | 105 |
| `get_entity()` | `src/service/lingo/entity/mod.rs` | 133 |
| `highlight_entities()` | `src/service/lingo/entity/mod.rs` | 270 |
| `list_entities()` | `src/service/lingo/entity/mod.rs` | 161 |
| `match_entities()` | `src/service/lingo/entity/mod.rs` | 214 |
| `search_entities()` | `src/service/lingo/entity/mod.rs` | 242 |
| `update_entity()` | `src/service/lingo/entity/mod.rs` | 76 |

#### file

| Method | File | Line |
|--------|------|------|
| `download_image()` | `src/service/lingo/file/mod.rs` | 69 |
| `upload_image()` | `src/service/lingo/file/mod.rs` | 41 |

#### repo

| Method | File | Line |
|--------|------|------|
| `list_repos()` | `src/service/lingo/repo/mod.rs` | 41 |

### mail

#### v1

| Method | File | Line |
|--------|------|------|
| `batch_create()` | `src/service/mail/v1/mail_group_manager/mod.rs` | 76 |
| `batch_delete()` | `src/service/mail/v1/mail_group_manager/mod.rs` | 105 |
| `create()` | `src/service/mail/v1/contact/mod.rs` | 101 |
| `create()` | `src/service/mail/v1/folder/mod.rs` | 111 |
| `create()` | `src/service/mail/v1/rule/mod.rs` | 113 |
| `create()` | `src/service/mail/v1/mail_group/mod.rs` | 117 |
| `delete()` | `src/service/mail/v1/contact/mod.rs` | 130 |
| `delete()` | `src/service/mail/v1/folder/mod.rs` | 155 |
| `delete()` | `src/service/mail/v1/rule/mod.rs` | 142 |
| `delete()` | `src/service/mail/v1/mail_group/mod.rs` | 141 |
| `download_url()` | `src/service/mail/v1/attachment/mod.rs` | 62 |
| `get()` | `src/service/mail/v1/message/mod.rs` | 173 |
| `get()` | `src/service/mail/v1/mail_group/mod.rs` | 226 |
| `get_by_card()` | `src/service/mail/v1/message/mod.rs` | 278 |
| `list()` | `src/service/mail/v1/contact/mod.rs` | 196 |
| `list()` | `src/service/mail/v1/folder/mod.rs` | 253 |
| `list()` | `src/service/mail/v1/message/mod.rs` | 222 |
| `list()` | `src/service/mail/v1/rule/mod.rs` | 208 |
| `list()` | `src/service/mail/v1/mail_group/mod.rs` | 253 |
| `list()` | `src/service/mail/v1/mail_group_manager/mod.rs` | 134 |
| `patch()` | `src/service/mail/v1/contact/mod.rs` | 162 |
| `patch()` | `src/service/mail/v1/folder/mod.rs` | 203 |
| `patch()` | `src/service/mail/v1/mail_group/mod.rs` | 168 |
| `reorder()` | `src/service/mail/v1/rule/mod.rs` | 243 |
| `subscribe()` | `src/service/mail/v1/event/mod.rs` | 65 |
| `subscription()` | `src/service/mail/v1/event/mod.rs` | 94 |
| `unsubscribe()` | `src/service/mail/v1/event/mod.rs` | 121 |
| `update()` | `src/service/mail/v1/rule/mod.rs` | 174 |
| `update()` | `src/service/mail/v1/mail_group/mod.rs` | 197 |

### mdm

#### country_region

| Method | File | Line |
|--------|------|------|
| `get()` | `src/service/mdm/country_region/mod.rs` | 42 |
| `list()` | `src/service/mdm/country_region/mod.rs` | 70 |

#### user_auth_data_relation

| Method | File | Line |
|--------|------|------|
| `bind()` | `src/service/mdm/user_auth_data_relation/mod.rs` | 42 |
| `unbind()` | `src/service/mdm/user_auth_data_relation/mod.rs` | 70 |

### minutes

#### v1

| Method | File | Line |
|--------|------|------|
| `get()` | `src/service/minutes/v1/minute/mod.rs` | 44 |
| `get()` | `src/service/minutes/v1/statistics/mod.rs` | 43 |
| `get()` | `src/service/minutes/v1/transcript/mod.rs` | 43 |
| `get()` | `src/service/minutes/v1/media/mod.rs` | 43 |

### moments

#### post

| Method | File | Line |
|--------|------|------|
| `get_post()` | `src/service/moments/post/mod.rs` | 58 |

### okr

#### okr

| Method | File | Line |
|--------|------|------|
| `batch_get_okrs()` | `src/service/okr/okr/mod.rs` | 154 |
| `list_user_okrs()` | `src/service/okr/okr/mod.rs` | 103 |

#### period

| Method | File | Line |
|--------|------|------|
| `create_period()` | `src/service/okr/period/mod.rs` | 112 |
| `list_periods()` | `src/service/okr/period/mod.rs` | 174 |
| `update_period_status()` | `src/service/okr/period/mod.rs` | 141 |

#### period_rule

| Method | File | Line |
|--------|------|------|
| `list_period_rules()` | `src/service/okr/period_rule/mod.rs` | 83 |

#### progress_record

| Method | File | Line |
|--------|------|------|
| `create_progress_record()` | `src/service/okr/progress_record/mod.rs` | 134 |
| `delete_progress_record()` | `src/service/okr/progress_record/mod.rs` | 162 |
| `get_progress_record()` | `src/service/okr/progress_record/mod.rs` | 228 |
| `update_progress_record()` | `src/service/okr/progress_record/mod.rs` | 195 |
| `upload_progress_image()` | `src/service/okr/progress_record/mod.rs` | 260 |

#### review

| Method | File | Line |
|--------|------|------|
| `query_reviews()` | `src/service/okr/review/mod.rs` | 86 |

### payroll

#### acct_item

| Method | File | Line |
|--------|------|------|
| `list_acct_items()` | `src/service/payroll/acct_item/mod.rs` | 83 |

#### cost_allocation_plan

| Method | File | Line |
|--------|------|------|
| `list_plans()` | `src/service/payroll/cost_allocation_plan/mod.rs` | 82 |

#### cost_allocation_report

| Method | File | Line |
|--------|------|------|
| `list_reports()` | `src/service/payroll/cost_allocation_report/mod.rs` | 88 |

#### datasource

| Method | File | Line |
|--------|------|------|
| `list_datasources()` | `src/service/payroll/datasource/mod.rs` | 79 |

#### datasource_record

| Method | File | Line |
|--------|------|------|
| `query_records()` | `src/service/payroll/datasource_record/mod.rs` | 197 |
| `save_records()` | `src/service/payroll/datasource_record/mod.rs` | 127 |

#### paygroup

| Method | File | Line |
|--------|------|------|
| `list_paygroups()` | `src/service/payroll/paygroup/mod.rs` | 81 |

#### payment_activity

| Method | File | Line |
|--------|------|------|
| `archive_activity()` | `src/service/payroll/payment_activity/mod.rs` | 186 |
| `list_activities()` | `src/service/payroll/payment_activity/mod.rs` | 104 |

#### payment_detail

| Method | File | Line |
|--------|------|------|
| `list_details()` | `src/service/payroll/payment_detail/mod.rs` | 99 |
| `query_details()` | `src/service/payroll/payment_detail/mod.rs` | 189 |

### performance

#### metric_detail

| Method | File | Line |
|--------|------|------|
| `import_metric_details()` | `src/service/performance/metric_detail/mod.rs` | 69 |
| `query_metric_details()` | `src/service/performance/metric_detail/mod.rs` | 41 |

#### review_config

| Method | File | Line |
|--------|------|------|
| `delete_additional_information()` | `src/service/performance/review_config/mod.rs` | 183 |
| `import_additional_information()` | `src/service/performance/review_config/mod.rs` | 155 |
| `list_metric_tags()` | `src/service/performance/review_config/mod.rs` | 435 |
| `list_semesters()` | `src/service/performance/review_config/mod.rs` | 45 |
| `query_activities()` | `src/service/performance/review_config/mod.rs` | 84 |
| `query_additional_information()` | `src/service/performance/review_config/mod.rs` | 127 |
| `query_metric_fields()` | `src/service/performance/review_config/mod.rs` | 407 |
| `query_metric_templates()` | `src/service/performance/review_config/mod.rs` | 379 |
| `query_metrics()` | `src/service/performance/review_config/mod.rs` | 351 |
| `query_review_items()` | `src/service/performance/review_config/mod.rs` | 295 |
| `query_review_templates()` | `src/service/performance/review_config/mod.rs` | 267 |
| `query_reviewees()` | `src/service/performance/review_config/mod.rs` | 239 |
| `query_tag_question_configs()` | `src/service/performance/review_config/mod.rs` | 323 |
| `write_user_group_members()` | `src/service/performance/review_config/mod.rs` | 211 |

#### review_data

| Method | File | Line |
|--------|------|------|
| `query_details()` | `src/service/performance/review_data/mod.rs` | 70 |
| `query_results()` | `src/service/performance/review_data/mod.rs` | 42 |

#### stage_task

| Method | File | Line |
|--------|------|------|
| `find_tasks_by_page()` | `src/service/performance/stage_task/mod.rs` | 69 |
| `find_tasks_by_user_list()` | `src/service/performance/stage_task/mod.rs` | 41 |

### personal_settings

#### v1

| Method | File | Line |
|--------|------|------|
| `batch_close()` | `src/service/personal_settings/v1/system_status/mod.rs` | 227 |
| `batch_open()` | `src/service/personal_settings/v1/system_status/mod.rs` | 203 |
| `create()` | `src/service/personal_settings/v1/system_status/mod.rs` | 85 |
| `delete()` | `src/service/personal_settings/v1/system_status/mod.rs` | 109 |
| `list()` | `src/service/personal_settings/v1/system_status/mod.rs` | 166 |
| `patch()` | `src/service/personal_settings/v1/system_status/mod.rs` | 137 |

### report

#### rule

| Method | File | Line |
|--------|------|------|
| `query()` | `src/service/report/rule/mod.rs` | 41 |

#### rule_view

| Method | File | Line |
|--------|------|------|
| `remove()` | `src/service/report/rule_view/mod.rs` | 38 |

#### task

| Method | File | Line |
|--------|------|------|
| `query()` | `src/service/report/task/mod.rs` | 41 |

### search

#### v1

| Method | File | Line |
|--------|------|------|
| `search_user()` | `src/service/search/v1/user.rs` | 30 |
| `search_user_with_validated_pagination()` | `src/service/search/v1/user.rs` | 75 |

#### v2

| Method | File | Line |
|--------|------|------|
| `batch_create()` | `src/service/search/v2/data_item/mod.rs` | 127 |
| `create()` | `src/service/search/v2/data_source/mod.rs` | 99 |
| `create()` | `src/service/search/v2/schema/mod.rs` | 90 |
| `create()` | `src/service/search/v2/data_item/mod.rs` | 93 |
| `delete()` | `src/service/search/v2/data_source/mod.rs` | 123 |
| `delete()` | `src/service/search/v2/schema/mod.rs` | 120 |
| `delete()` | `src/service/search/v2/data_item/mod.rs` | 157 |
| `get()` | `src/service/search/v2/data_source/mod.rs` | 180 |
| `get()` | `src/service/search/v2/schema/mod.rs` | 179 |
| `get()` | `src/service/search/v2/data_item/mod.rs` | 188 |
| `list()` | `src/service/search/v2/data_source/mod.rs` | 207 |
| `patch()` | `src/service/search/v2/data_source/mod.rs` | 151 |
| `patch()` | `src/service/search/v2/schema/mod.rs` | 149 |
| `search_app()` | `src/service/search/v2/suite_search/mod.rs` | 109 |
| `search_message()` | `src/service/search/v2/suite_search/mod.rs` | 68 |

### security_and_compliance

#### audit_log

| Method | File | Line |
|--------|------|------|
| `audit_data_get()` | `src/service/security_and_compliance/audit_log/mod.rs` | 33 |

#### openapi_log

| Method | File | Line |
|--------|------|------|
| `list_data()` | `src/service/security_and_compliance/openapi_log/mod.rs` | 33 |

### task

#### v2

| Method | File | Line |
|--------|------|------|
| `add()` | `src/service/task/v2/custom_field/mod.rs` | 245 |
| `add_dependencies()` | `src/service/task/v2/task/mod.rs` | 645 |
| `add_members()` | `src/service/task/v2/tasklist/mod.rs` | 273 |
| `add_members()` | `src/service/task/v2/task/mod.rs` | 500 |
| `add_reminders()` | `src/service/task/v2/task/mod.rs` | 558 |
| `add_tasklist()` | `src/service/task/v2/task/mod.rs` | 587 |
| `create()` | `src/service/task/v2/task_subtask/mod.rs` | 72 |
| `create()` | `src/service/task/v2/custom_field/mod.rs` | 127 |
| `create()` | `src/service/task/v2/section/mod.rs` | 123 |
| `create()` | `src/service/task/v2/comment/mod.rs` | 106 |
| `create()` | `src/service/task/v2/tasklist_activity_subscription/mod.rs` | 115 |
| `create()` | `src/service/task/v2/custom_field_option/mod.rs` | 77 |
| `create()` | `src/service/task/v2/tasklist/mod.rs` | 166 |
| `create()` | `src/service/task/v2/task/mod.rs` | 334 |
| `delete()` | `src/service/task/v2/section/mod.rs` | 203 |
| `delete()` | `src/service/task/v2/comment/mod.rs` | 195 |
| `delete()` | `src/service/task/v2/attachment/mod.rs` | 174 |
| `delete()` | `src/service/task/v2/tasklist_activity_subscription/mod.rs` | 245 |
| `delete()` | `src/service/task/v2/tasklist/mod.rs` | 246 |
| `delete()` | `src/service/task/v2/task/mod.rs` | 414 |
| `get()` | `src/service/task/v2/custom_field/mod.rs` | 151 |
| `get()` | `src/service/task/v2/section/mod.rs` | 147 |
| `get()` | `src/service/task/v2/comment/mod.rs` | 135 |
| `get()` | `src/service/task/v2/attachment/mod.rs` | 147 |
| `get()` | `src/service/task/v2/tasklist_activity_subscription/mod.rs` | 144 |
| `get()` | `src/service/task/v2/tasklist/mod.rs` | 190 |
| `get()` | `src/service/task/v2/task/mod.rs` | 387 |
| `list()` | `src/service/task/v2/task_subtask/mod.rs` | 101 |
| `list()` | `src/service/task/v2/custom_field/mod.rs` | 207 |
| `list()` | `src/service/task/v2/section/mod.rs` | 230 |
| `list()` | `src/service/task/v2/comment/mod.rs` | 224 |
| `list()` | `src/service/task/v2/attachment/mod.rs` | 113 |
| `list()` | `src/service/task/v2/tasklist_activity_subscription/mod.rs` | 176 |
| `list()` | `src/service/task/v2/tasklist/mod.rs` | 395 |
| `list()` | `src/service/task/v2/task/mod.rs` | 442 |
| `patch()` | `src/service/task/v2/custom_field/mod.rs` | 178 |
| `patch()` | `src/service/task/v2/section/mod.rs` | 174 |
| `patch()` | `src/service/task/v2/comment/mod.rs` | 164 |
| `patch()` | `src/service/task/v2/tasklist_activity_subscription/mod.rs` | 211 |
| `patch()` | `src/service/task/v2/custom_field_option/mod.rs` | 106 |
| `patch()` | `src/service/task/v2/tasklist/mod.rs` | 217 |
| `patch()` | `src/service/task/v2/task/mod.rs` | 358 |
| `remove()` | `src/service/task/v2/custom_field/mod.rs` | 274 |
| `remove_dependencies()` | `src/service/task/v2/task/mod.rs` | 674 |
| `remove_members()` | `src/service/task/v2/tasklist/mod.rs` | 302 |
| `remove_members()` | `src/service/task/v2/task/mod.rs` | 529 |
| `remove_reminders()` | `src/service/task/v2/task/mod.rs` | 616 |
| `tasks()` | `src/service/task/v2/section/mod.rs` | 263 |
| `tasks()` | `src/service/task/v2/tasklist/mod.rs` | 332 |
| `upload()` | `src/service/task/v2/attachment/mod.rs` | 89 |

### tenant

#### v2

| Method | File | Line |
|--------|------|------|
| `query()` | `src/service/tenant/v2/tenant/mod.rs` | 67 |
| `query()` | `src/service/tenant/v2/tenant_product_assign_info/mod.rs` | 61 |

### tenant_tag

#### tag

| Method | File | Line |
|--------|------|------|
| `create()` | `src/service/tenant_tag/tag/mod.rs` | 99 |
| `list()` | `src/service/tenant_tag/tag/mod.rs` | 148 |
| `patch()` | `src/service/tenant_tag/tag/mod.rs` | 123 |

#### tag_binding

| Method | File | Line |
|--------|------|------|
| `create()` | `src/service/tenant_tag/tag_binding/mod.rs` | 146 |
| `get()` | `src/service/tenant_tag/tag_binding/mod.rs` | 112 |
| `update()` | `src/service/tenant_tag/tag_binding/mod.rs` | 170 |

### trust_party

#### collaboration_organization

| Method | File | Line |
|--------|------|------|
| `admin_list_organizations()` | `src/service/trust_party/collaboration_organization/mod.rs` | 263 |
| `get_organization()` | `src/service/trust_party/collaboration_organization/mod.rs` | 118 |
| `get_organization_department()` | `src/service/trust_party/collaboration_organization/mod.rs` | 184 |
| `get_organization_structure()` | `src/service/trust_party/collaboration_organization/mod.rs` | 86 |
| `get_organization_user()` | `src/service/trust_party/collaboration_organization/mod.rs` | 151 |
| `list_organizations()` | `src/service/trust_party/collaboration_organization/mod.rs` | 45 |
| `list_shared_member_scope()` | `src/service/trust_party/collaboration_organization/mod.rs` | 217 |

#### searchable_visible_rules

| Method | File | Line |
|--------|------|------|
| `create_rule()` | `src/service/trust_party/searchable_visible_rules/mod.rs` | 42 |
| `delete_rule()` | `src/service/trust_party/searchable_visible_rules/mod.rs` | 159 |
| `list_rules()` | `src/service/trust_party/searchable_visible_rules/mod.rs` | 104 |
| `update_rule()` | `src/service/trust_party/searchable_visible_rules/mod.rs` | 71 |

### vc

#### v1

| Method | File | Line |
|--------|------|------|
| `apply()` | `src/service/vc/v1/reserve/mod.rs` | 130 |
| `create()` | `src/service/vc/v1/room/mod.rs` | 153 |
| `delete()` | `src/service/vc/v1/room/mod.rs` | 206 |
| `delete()` | `src/service/vc/v1/reserve/mod.rs` | 154 |
| `end()` | `src/service/vc/v1/meeting/mod.rs` | 227 |
| `get()` | `src/service/vc/v1/room/mod.rs` | 233 |
| `get()` | `src/service/vc/v1/recording/mod.rs` | 144 |
| `get()` | `src/service/vc/v1/meeting/mod.rs` | 250 |
| `get()` | `src/service/vc/v1/reserve/mod.rs` | 202 |
| `get_active_meeting()` | `src/service/vc/v1/reserve/mod.rs` | 225 |
| `invite()` | `src/service/vc/v1/meeting/mod.rs` | 144 |
| `kickout()` | `src/service/vc/v1/meeting/mod.rs` | 169 |
| `list()` | `src/service/vc/v1/room/mod.rs` | 260 |
| `list_by_no()` | `src/service/vc/v1/meeting/mod.rs` | 273 |
| `search()` | `src/service/vc/v1/room/mod.rs` | 294 |
| `set_host()` | `src/service/vc/v1/meeting/mod.rs` | 198 |
| `set_permission()` | `src/service/vc/v1/recording/mod.rs` | 167 |
| `start()` | `src/service/vc/v1/recording/mod.rs` | 92 |
| `stop()` | `src/service/vc/v1/recording/mod.rs` | 121 |
| `update()` | `src/service/vc/v1/room/mod.rs` | 177 |
| `update()` | `src/service/vc/v1/reserve/mod.rs` | 177 |

### verification

#### v1

| Method | File | Line |
|--------|------|------|
| `get()` | `src/service/verification/v1/mod.rs` | 54 |

### workplace

#### app_recommend

| Method | File | Line |
|--------|------|------|
| `get_favourite_apps()` | `src/service/workplace/app_recommend/mod.rs` | 43 |
| `get_recommended_apps()` | `src/service/workplace/app_recommend/mod.rs` | 90 |
| `list_recommend_rules()` | `src/service/workplace/app_recommend/mod.rs` | 143 |

#### workplace_access_data

| Method | File | Line |
|--------|------|------|
| `search()` | `src/service/workplace/workplace_access_data/mod.rs` | 47 |
| `search_custom()` | `src/service/workplace/workplace_access_data/mod.rs` | 118 |
| `search_custom_widget()` | `src/service/workplace/workplace_access_data/mod.rs` | 183 |

