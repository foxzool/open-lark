# Rust版本API实现映射表

**生成时间**: 2025年11月09日 10:48:11
**匹配方法**: Rust版本高精度URL提取与匹配
**总API数**: 1550
**已实现**: 18
**实现率**: 1.2%

## 实现统计摘要

- **已实现接口总数**: 1550
- **有文档地址**: 18 (1.2%)
- **无文档地址**: 1532 (98.8%)

## 按服务统计

| 服务 | 总数 | 已实现 | 实现率 |
|------|------|--------|--------|
| acs | 14 | 0 | 0.0% |
| admin | 14 | 0 | 0.0% |
| aily | 21 | 0 | 0.0% |
| apaas | 37 | 0 | 0.0% |
| application | 32 | 0 | 0.0% |
| approval | 29 | 0 | 0.0% |
| attendance | 39 | 0 | 0.0% |
| auth | 5 | 0 | 0.0% |
| authen | 6 | 0 | 0.0% |
| baike | 13 | 0 | 0.0% |
| base | 3 | 0 | 0.0% |
| bitable | 46 | 0 | 0.0% |
| board | 6 | 0 | 0.0% |
| calendar | 44 | 0 | 0.0% |
| cardkit | 10 | 0 | 0.0% |
| compensation | 21 | 0 | 0.0% |
| contact | 75 | 0 | 0.0% |
| corehr | 249 | 0 | 0.0% |
| directory | 21 | 0 | 0.0% |
| doc | 6 | 0 | 0.0% |
| docs | 1 | 0 | 0.0% |
| document_ai | 18 | 0 | 0.0% |
| docx | 19 | 0 | 0.0% |
| drive | 70 | 0 | 0.0% |
| ehr | 2 | 0 | 0.0% |
| ephemeral | 2 | 0 | 0.0% |
| event | 1 | 0 | 0.0% |
| face_verify | 3 | 0 | 0.0% |
| helpdesk | 50 | 0 | 0.0% |
| hire | 182 | 0 | 0.0% |
| human_authentication | 1 | 0 | 0.0% |
| im | 71 | 0 | 0.0% |
| interactive | 1 | 0 | 0.0% |
| lingo | 14 | 0 | 0.0% |
| mail | 67 | 0 | 0.0% |
| mdm | 4 | 0 | 0.0% |
| meeting_room | 17 | 0 | 0.0% |
| message | 1 | 0 | 0.0% |
| minutes | 4 | 0 | 0.0% |
| moments | 1 | 0 | 0.0% |
| okr | 12 | 0 | 0.0% |
| optical_char_recognition | 1 | 0 | 0.0% |
| passport | 2 | 0 | 0.0% |
| pay | 3 | 0 | 0.0% |
| payroll | 12 | 0 | 0.0% |
| performance | 20 | 0 | 0.0% |
| personal_settings | 6 | 0 | 0.0% |
| report | 3 | 0 | 0.0% |
| search | 15 | 0 | 0.0% |
| security_and_compliance | 8 | 0 | 0.0% |
| sheets | 59 | 18 | 30.5% |
| speech_to_text | 2 | 0 | 0.0% |
| suite | 2 | 0 | 0.0% |
| task | 75 | 0 | 0.0% |
| tenant | 2 | 0 | 0.0% |
| translation | 2 | 0 | 0.0% |
| trust_party | 5 | 0 | 0.0% |
| user | 1 | 0 | 0.0% |
| vc | 56 | 0 | 0.0% |
| verification | 1 | 0 | 0.0% |
| wiki | 16 | 0 | 0.0% |
| workplace | 3 | 0 | 0.0% |

## 详细映射表（按模块排序）

| 序号 | API名称 | HTTP方法 | 路径 | 状态 | 文档地址 | 函数名 | 文件路径 | 行号 |
|------|---------|----------|------|------|----------|--------|----------|------|

### 📦 acs 模块

| 1 | 下载人脸图片 | GET | /open-apis/acs/v1/users/:user_id/face | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/acs-v1/user-face/get) | - | 未找到 | - |
| 2 | 下载开门时的人脸识别图片 | GET | /open-apis/acs/v1/access_records/:access_record_id/access_photo | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/acs-v1/access_record-access_photo/get) | - | 未找到 | - |
| 3 | 获取单个用户信息 | GET | /open-apis/acs/v1/users/:user_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/acs-v1/user/get) | - | 未找到 | - |
| 4 | 获取权限组信息 | GET | /open-apis/acs/v1/rule_external | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/acs-v1/rule_external/get) | - | 未找到 | - |
| 5 | 获取用户列表 | GET | /open-apis/acs/v1/users | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/acs-v1/user/list) | - | 未找到 | - |
| 6 | 获取门禁记录列表 | GET | /open-apis/acs/v1/access_records | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/acs-v1/access_record/list) | - | 未找到 | - |
| 7 | 获取门禁设备列表 | GET | /open-apis/acs/v1/devices | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/acs-v1/device/list) | - | 未找到 | - |
| 8 | 创建或更新权限组 | POST | /open-apis/acs/v1/rule_external | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/acs-v1/rule_external/create) | - | 未找到 | - |
| 9 | 添加访客 | POST | /open-apis/acs/v1/visitors | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/acs-v1/visitor/create) | - | 未找到 | - |
| 10 | 设备绑定权限组 | POST | /open-apis/acs/v1/rule_external/device_bind | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/acs-v1/rule_external/device_bind) | - | 未找到 | - |
| 11 | 上传人脸图片 | PUT | /open-apis/acs/v1/users/:user_id/face | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/acs-v1/user-face/update) | - | 未找到 | - |
| 12 | 修改用户部分信息 | PATCH | /open-apis/acs/v1/users/:user_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/acs-v1/user/patch) | - | 未找到 | - |
| 13 | 删除权限组 | DELETE | /open-apis/acs/v1/rule_external | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/acs-v1/rule_external/delete) | - | 未找到 | - |
| 14 | 删除访客 | DELETE | /open-apis/acs/v1/visitors/:visitor_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/acs-v1/visitor/delete) | - | 未找到 | - |

### 📦 admin 模块

| 15 | 获取勋章列表 | GET | /open-apis/admin/v1/badges | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/admin-v1/badge/list) | - | 未找到 | - |
| 16 | 获取勋章详情 | GET | /open-apis/admin/v1/badges/:badge_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/admin-v1/badge/get) | - | 未找到 | - |
| 17 | 获取授予名单列表 | GET | /open-apis/admin/v1/badges/:badge_id/grants | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/admin-v1/badge-grant/list) | - | 未找到 | - |
| 18 | 获取授予名单详情 | GET | /open-apis/admin/v1/badges/:badge_id/grants/:grant_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/admin-v1/badge-grant/get) | - | 未找到 | - |
| 19 | 获取用户维度的用户活跃和功能使用数据 | GET | /open-apis/admin/v1/admin_user_stats | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/admin-v1/admin_user_stat/list) | - | 未找到 | - |
| 20 | 获取行为审计日志数据 | GET | /open-apis/admin/v1/audit_infos | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uQjM5YjL0ITO24CNykjN/audit_log/audit_data_get) | - | 未找到 | - |
| 21 | 获取部门维度的用户活跃和功能使用数据 | GET | /open-apis/admin/v1/admin_dept_stats | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/admin-v1/admin_dept_stat/list) | - | 未找到 | - |
| 22 | 上传勋章图片 | POST | /open-apis/admin/v1/badge_images | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/admin-v1/badge_image/create) | - | 未找到 | - |
| 23 | 创建勋章 | POST | /open-apis/admin/v1/badges | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/admin-v1/badge/create) | - | 未找到 | - |
| 24 | 创建授予名单 | POST | /open-apis/admin/v1/badges/:badge_id/grants | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/admin-v1/badge-grant/create) | - | 未找到 | - |
| 25 | 重置用户的企业邮箱密码 | POST | /open-apis/admin/v1/password/reset | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/admin-v1/password/reset) | - | 未找到 | - |
| 26 | 修改勋章信息 | PUT | /open-apis/admin/v1/badges/:badge_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/admin-v1/badge/update) | - | 未找到 | - |
| 27 | 修改授予名单 | PUT | /open-apis/admin/v1/badges/:badge_id/grants/:grant_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/admin-v1/badge-grant/update) | - | 未找到 | - |
| 28 | 删除授予名单 | DELETE | /open-apis/admin/v1/badges/:badge_id/grants/:grant_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/admin-v1/badge-grant/delete) | - | 未找到 | - |

### 📦 aily 模块

| 29 | 列出 Aily 消息 | GET | /open-apis/aily/v1/sessions/:aily_session_id/messages | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/aily-v1/aily_session-aily_message/list) | - | 未找到 | - |
| 30 | 列出运行 | GET | /open-apis/aily/v1/sessions/:aily_session_id/runs | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/aily-v1/aily_session-run/list) | - | 未找到 | - |
| 31 | 查询技能列表 | GET | /open-apis/aily/v1/apps/:app_id/skills | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/aily-v1/app-skill/list) | - | 未找到 | - |
| 32 | 查询数据知识列表 | GET | /open-apis/aily/v1/apps/:app_id/data_assets | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/aily-v1/app-data_asset/list) | - | 未找到 | - |
| 33 | 获取 Aily 消息 | GET | /open-apis/aily/v1/sessions/:aily_session_id/messages/:aily_message_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/aily-v1/aily_session-aily_message/get) | - | 未找到 | - |
| 34 | 获取会话 | GET | /open-apis/aily/v1/sessions/:aily_session_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/aily-v1/aily_session/get) | - | 未找到 | - |
| 35 | 获取技能信息 | GET | /open-apis/aily/v1/apps/:app_id/skills/:skill_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/aily-v1/app-skill/get) | - | 未找到 | - |
| 36 | 获取数据知识 | GET | /open-apis/aily/v1/apps/:app_id/data_assets/:data_asset_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/aily-v1/app-data_asset/get) | - | 未找到 | - |
| 37 | 获取数据知识分类列表 | GET | /open-apis/aily/v1/apps/:app_id/data_asset_tags | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/aily-v1/app-data_asset_tag/list) | - | 未找到 | - |
| 38 | 获取运行 | GET | /open-apis/aily/v1/sessions/:aily_session_id/runs/:run_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/aily-v1/aily_session-run/get) | - | 未找到 | - |
| 39 | 上传文件用于数据知识管理 | POST | /open-apis/aily/v1/apps/:app_id/data_assets/upload_file | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/aily-v1/app-data_asset/upload_file) | - | 未找到 | - |
| 40 | 创建会话 | POST | /open-apis/aily/v1/sessions | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/aily-v1/aily_session/create) | - | 未找到 | - |
| 41 | 创建数据知识 | POST | /open-apis/aily/v1/apps/:app_id/data_assets | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/aily-v1/app-data_asset/create) | - | 未找到 | - |
| 42 | 创建运行 | POST | /open-apis/aily/v1/sessions/:aily_session_id/runs | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/aily-v1/aily_session-run/create) | - | 未找到 | - |
| 43 | 发送 Aily 消息 | POST | /open-apis/aily/v1/sessions/:aily_session_id/messages | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/aily-v1/aily_session-aily_message/create) | - | 未找到 | - |
| 44 | 取消运行 | POST | /open-apis/aily/v1/sessions/:aily_session_id/runs/:run_id/cancel | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/aily-v1/aily_session-run/cancel) | - | 未找到 | - |
| 45 | 执行数据知识问答 | POST | /open-apis/aily/v1/apps/:app_id/knowledges/ask | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/aily-v1/app-knowledge/ask) | - | 未找到 | - |
| 46 | 调用技能 | POST | /open-apis/aily/v1/apps/:app_id/skills/:skill_id/start | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/aily-v1/app-skill/start) | - | 未找到 | - |
| 47 | 更新会话 | PUT | /open-apis/aily/v1/sessions/:aily_session_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/aily-v1/aily_session/update) | - | 未找到 | - |
| 48 | 删除会话 | DELETE | /open-apis/aily/v1/sessions/:aily_session_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/aily-v1/aily_session/delete) | - | 未找到 | - |
| 49 | 删除数据知识 | DELETE | /open-apis/aily/v1/apps/:app_id/data_assets/:data_asset_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/aily-v1/app-data_asset/delete) | - | 未找到 | - |

### 📦 apaas 模块

| 50 | 查看应用基本信息 | GET | /open-apis/apaas/v1/apps | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/apaas-v1/app/list) | - | 未找到 | - |
| 51 | 查询审计日志列表 | GET | /open-apis/apaas/v1/applications/:namespace/audit_log/audit_log_list | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/apaas-v1/application-audit_log/audit_log_list) | - | 未找到 | - |
| 52 | 查询审计日志详情 | GET | /open-apis/apaas/v1/applications/:namespace/audit_log | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/apaas-v1/application-audit_log/get) | - | 未找到 | - |
| 53 | 查询席位分配详情 | GET | /open-apis/apaas/v1/seat_assignments | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/apaas-v1/seat_assignment/list) | - | 未找到 | - |
| 54 | 查询席位活跃详情 | GET | /open-apis/apaas/v1/seat_activities | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/apaas-v1/seat_activity/list) | - | 未找到 | - |
| 55 | 查询数据变更日志列表 | GET | /open-apis/apaas/v1/applications/:namespace/audit_log/data_change_logs_list | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/apaas-v1/application-audit_log/data_change_logs_list) | - | 未找到 | - |
| 56 | 查询数据变更日志详情 | GET | /open-apis/apaas/v1/applications/:namespace/audit_log/data_change_log_detail | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/apaas-v1/application-audit_log/data_change_log_detail) | - | 未找到 | - |
| 57 | 查询环境变量详情 | GET | /open-apis/apaas/v1/applications/:namespace/environment_variables/:environment_variable_api_name | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/apaas-v1/application-environment_variable/get) | - | 未找到 | - |
| 58 | 查询角色成员信息 | GET | /open-apis/apaas/v1/applications/:namespace/roles/:role_api_name/member | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/apaas-v1/application-role-member/get) | - | 未找到 | - |
| 59 | 人工任务加签 | POST | /open-apis/apaas/v1/approval_tasks/:approval_task_id/add_assignee | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/apaas-v1/approval_task/add_assignee) | - | 未找到 | - |
| 60 | 催办人工任务 | POST | /open-apis/apaas/v1/user_tasks/:task_id/expediting | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/apaas-v1/user_task/expediting) | - | 未找到 | - |
| 61 | 发起流程 | POST | /open-apis/apaas/v1/applications/:namespace/flows/:flow_id/execute | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/apaas-v1/application-flow/execute) | - | 未找到 | - |
| 62 | 同意人工任务 | POST | /open-apis/apaas/v1/approval_tasks/:approval_task_id/agree | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/apaas-v1/approval_task/agree) | - | 未找到 | - |
| 63 | 基于人工任务发起群聊 | POST | /open-apis/apaas/v1/user_tasks/:task_id/chat_group | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/apaas-v1/user_task/chat_group) | - | 未找到 | - |
| 64 | 执行 OQL | POST | /open-apis/apaas/v1/applications/:namespace/objects/oql_query | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/apaas-v1/application-object/oql_query) | - | 未找到 | - |
| 65 | 执行函数 | POST | /open-apis/apaas/v1/applications/:namespace/functions/:function_api_name/invoke | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/apaas-v1/application-function/invoke) | - | 未找到 | - |
| 66 | 批量创建角色成员授权 | POST | /open-apis/apaas/v1/applications/:namespace/roles/:role_api_name/member/batch_create_authorization | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/apaas-v1/application-role-member/batch_create_authorization) | - | 未找到 | - |
| 67 | 批量创建记录权限用户授权 | POST | /open-apis/apaas/v1/applications/:namespace/record_permissions/:record_permission_api_name/member/batch_create_authorization | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/apaas-v1/application-record_permission-member/batch_create_authorization) | - | 未找到 | - |
| 68 | 批量删除角色成员授权 | POST | /open-apis/apaas/v1/applications/:namespace/roles/:role_api_name/member/batch_remove_authorization | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/apaas-v1/application-role-member/batch_remove_authorization) | - | 未找到 | - |
| 69 | 批量删除记录权限用户授权 | POST | /open-apis/apaas/v1/applications/:namespace/record_permissions/:record_permission_api_name/member/batch_remove_authorization | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/apaas-v1/application-record_permission-member/batch_remove_authorization) | - | 未找到 | - |
| 70 | 批量新建记录 | POST | /open-apis/apaas/v1/applications/:namespace/objects/:object_api_name/records/batch_create | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/apaas-v1/application-object-record/batch_create) | - | 未找到 | - |
| 71 | 抄送人工任务 | POST | /open-apis/apaas/v1/user_tasks/:task_id/cc | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/apaas-v1/user_task/cc) | - | 未找到 | - |
| 72 | 拒绝人工任务 | POST | /open-apis/apaas/v1/approval_tasks/:approval_task_id/reject | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/apaas-v1/approval_task/reject) | - | 未找到 | - |
| 73 | 搜索记录 | POST | /open-apis/apaas/v1/applications/:namespace/objects/search | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/apaas-v1/application-object/search) | - | 未找到 | - |
| 74 | 撤销人工任务 | POST | /open-apis/apaas/v1/approval_instances/:approval_instance_id/cancel | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/apaas-v1/approval_instance/cancel) | - | 未找到 | - |
| 75 | 新建记录 | POST | /open-apis/apaas/v1/applications/:namespace/objects/:object_api_name/records | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/apaas-v1/application-object-record/create) | - | 未找到 | - |
| 76 | 查询人工任务 | POST | /open-apis/apaas/v1/user_task/query | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/apaas-v1/user_task/query) | - | 未找到 | - |
| 77 | 查询人工任务可退回的位置 | POST | /open-apis/apaas/v1/user_tasks/:task_id/rollback_points | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/apaas-v1/user_task/rollback_points) | - | 未找到 | - |
| 78 | 查询环境变量列表 | POST | /open-apis/apaas/v1/applications/:namespace/environment_variables/query | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/apaas-v1/application-environment_variable/query) | - | 未找到 | - |
| 79 | 查询记录列表 | POST | /open-apis/apaas/v1/applications/:namespace/objects/:object_api_name/records/batch_query | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/apaas-v1/application-object-record/batch_query) | - | 未找到 | - |
| 80 | 获取记录详情 | POST | /open-apis/apaas/v1/applications/:namespace/objects/:object_api_name/records/:id/query | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/apaas-v1/application-object-record/query) | - | 未找到 | - |
| 81 | 转交人工任务 | POST | /open-apis/apaas/v1/approval_tasks/:approval_task_id/transfer | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/apaas-v1/approval_task/transfer) | - | 未找到 | - |
| 82 | 退回人工任务 | POST | /open-apis/apaas/v1/user_tasks/:task_id/rollback | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/apaas-v1/user_task/rollback) | - | 未找到 | - |
| 83 | 批量编辑记录 | PATCH | /open-apis/apaas/v1/applications/:namespace/objects/:object_api_name/records/batch_update | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/apaas-v1/application-object-record/batch_update) | - | 未找到 | - |
| 84 | 编辑记录 | PATCH | /open-apis/apaas/v1/applications/:namespace/objects/:object_api_name/records/:id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/apaas-v1/application-object-record/patch) | - | 未找到 | - |
| 85 | 删除记录 | DELETE | /open-apis/apaas/v1/applications/:namespace/objects/:object_api_name/records/:id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/apaas-v1/application-object-record/delete) | - | 未找到 | - |
| 86 | 批量删除记录 | DELETE | /open-apis/apaas/v1/applications/:namespace/objects/:object_api_name/records/batch_delete | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/apaas-v1/application-object-record/batch_delete) | - | 未找到 | - |

### 📦 application 模块

| 87 | 查看待审核的应用列表 | GET | /open-apis/application/v6/applications/underauditlist | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/application-v6/application/underauditlist) | - | 未找到 | - |
| 88 | 查询租户授权状态 | GET | /open-apis/application/v6/scopes | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/application-v6/scope/list) | - | 未找到 | - |
| 89 | 校验应用管理员 | GET | /open-apis/application/v3/is_user_admin | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uITN1EjLyUTNx4iM1UTM) | - | 未找到 | - |
| 90 | 获取企业安装的应用 | GET | /open-apis/application/v6/applications | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/application-v6/application/list) | - | 未找到 | - |
| 91 | 获取企业安装的应用 | GET | /open-apis/application/v3/app/list | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uYDN3UjL2QzN14iN0cTN) | - | 未找到 | - |
| 92 | 获取应用信息 | GET | /open-apis/application/v6/applications/:app_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/application-v6/application/get) | - | 未找到 | - |
| 93 | 获取应用协作者列表 | GET | /open-apis/application/v6/applications/:app_id/collaborators | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/application-v6/application-collaborators/get) | - | 未找到 | - |
| 94 | 获取应用反馈列表 | GET | /open-apis/application/v6/applications/:app_id/feedbacks | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/application-v6/application-feedback/list) | - | 未找到 | - |
| 95 | 获取应用在企业内的可用范围 | GET | /open-apis/application/v2/app/visibility | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uIjM3UjLyIzN14iMycTN) | - | 未找到 | - |
| 96 | 获取应用版本中开发者申请的通讯录权限范围 | GET | /open-apis/application/v6/applications/:app_id/app_versions/:version_id/contacts_range_suggest | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/application-v6/application-app_version/contacts_range_suggest) | - | 未找到 | - |
| 97 | 获取应用版本信息 | GET | /open-apis/application/v6/applications/:app_id/app_versions/:version_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/application-v6/application-app_version/get) | - | 未找到 | - |
| 98 | 获取应用版本列表 | GET | /open-apis/application/v6/applications/:app_id/app_versions | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/application-v6/application-app_version/list) | - | 未找到 | - |
| 99 | 获取应用通讯录权限范围配置 | GET | /open-apis/application/v6/applications/:app_id/contacts_range_configuration | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/application-v6/application/contacts_range_configuration) | - | 未找到 | - |
| 100 | 获取当前设置的推荐规则列表 | GET | /open-apis/application/v6/app_recommend_rules | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/application-v6/app_recommend_rule/list) | - | 未找到 | - |
| 101 | 获取用户可用的应用 | GET | /open-apis/application/v1/user/visible_apps | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMjM3UjLzIzN14yMycTN) | - | 未找到 | - |
| 102 | 获取用户自定义常用的应用 | GET | /open-apis/application/v5/applications/favourite | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/application-v5/application/favourite) | - | 未找到 | - |
| 103 | 获取管理员推荐的应用 | GET | /open-apis/application/v5/applications/recommend | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/application-v5/application/recommend) | - | 未找到 | - |
| 104 | 向管理员申请授权 | POST | /open-apis/application/v6/scopes/apply | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/application-v6/scope/apply) | - | 未找到 | - |
| 105 | 更新应用可用范围 | POST | /open-apis/application/v3/app/update_visibility | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/ucDN3UjL3QzN14yN0cTN) | - | 未找到 | - |
| 106 | 更新应用红点 | POST | /open-apis/application/v6/app_badge/set | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/application-v6/app_badge/set) | - | 未找到 | - |
| 107 | 查询用户或部门是否在应用的可用或禁用名单 | POST | /open-apis/application/v6/applications/:app_id/visibility/check_white_black_list | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/application-v6/application-visibility/check_white_black_list) | - | 未找到 | - |
| 108 | 获取多部门应用使用概览 | POST | /open-apis/application/v6/applications/:app_id/app_usage/department_overview | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/application-v6/application-app_usage/department_overview) | - | 未找到 | - |
| 109 | 获取应用使用概览 | POST | /open-apis/application/v6/applications/:app_id/app_usage/overview | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/application-v6/application-app_usage/overview) | - | 未找到 | - |
| 110 | 获取消息推送概览 | POST | /open-apis/application/v6/applications/:app_id/app_usage/message_push_overview | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/application-v6/application-app_usage/message_push_overview) | - | 未找到 | - |
| 111 | 启停用应用 | PUT | /open-apis/application/v6/applications/:app_id/management | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/application-v6/application-management/update) | - | 未找到 | - |
| 112 | 更新应用协作者 | PUT | /open-apis/application/v6/applications/:app_id/collaborators | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/application-v6/application-collaborators/update) | - | 未找到 | - |
| 113 | 转移应用所有者 | PUT | /open-apis/application/v6/applications/:app_id/owner | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/application-v6/application-owner/update) | - | 未找到 | - |
| 114 | 更新应用分组信息 | PATCH | /open-apis/application/v6/applications/:app_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/application-v6/application/patch) | - | 未找到 | - |
| 115 | 更新应用反馈 | PATCH | /open-apis/application/v6/applications/:app_id/feedbacks/:feedback_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/application-v6/application-feedback/patch) | - | 未找到 | - |
| 116 | 更新应用可用范围 | PATCH | /open-apis/application/v6/applications/:app_id/visibility | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/application-v6/application-visibility/patch) | - | 未找到 | - |
| 117 | 更新应用审核状态 | PATCH | /open-apis/application/v6/applications/:app_id/app_versions/:version_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/application-v6/application-app_version/patch) | - | 未找到 | - |
| 118 | 更新应用通讯录权限范围配置 | PATCH | /open-apis/application/v6/applications/:app_id/contacts_range | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/application-v6/application-contacts_range/patch) | - | 未找到 | - |

### 📦 approval 模块

| 119 | 批量获取审批实例 ID | GET | /open-apis/approval/v4/instances | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/instance/list) | - | 未找到 | - |
| 120 | 查看指定三方审批定义 | GET | /open-apis/approval/v4/external_approvals/:approval_code | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/external_approval/get) | - | 未找到 | - |
| 121 | 查看指定审批定义 | GET | /open-apis/approval/v4/approvals/:approval_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/approval/get) | - | 未找到 | - |
| 122 | 查询用户的任务列表 | GET | /open-apis/approval/v4/tasks/query | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/task/query) | - | 未找到 | - |
| 123 | 获取三方审批任务状态 | GET | /open-apis/approval/v4/external_tasks | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/external_task/list) | - | 未找到 | - |
| 124 | 获取单个审批实例详情 | GET | /open-apis/approval/v4/instances/:instance_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/instance/get) | - | 未找到 | - |
| 125 | 获取评论 | GET | /open-apis/approval/v4/instances/:instance_id/comments | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/instance-comment/list) | - | 未找到 | - |
| 126 | 创建三方审批定义 | POST | /open-apis/approval/v4/external_approvals | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/external_approval/create) | - | 未找到 | - |
| 127 | 创建审批定义 | POST | /open-apis/approval/v4/approvals | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/approval/create) | - | 未找到 | - |
| 128 | 创建审批实例 | POST | /open-apis/approval/v4/instances | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/instance/create) | - | 未找到 | - |
| 129 | 创建评论 | POST | /open-apis/approval/v4/instances/:instance_id/comments | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/instance-comment/create) | - | 未找到 | - |
| 130 | 取消订阅审批事件 | POST | /open-apis/approval/v4/approvals/:approval_code/unsubscribe | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/approval/unsubscribe) | - | 未找到 | - |
| 131 | 同意审批任务 | POST | /open-apis/approval/v4/tasks/approve | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/task/approve) | - | 未找到 | - |
| 132 | 同步三方审批实例 | POST | /open-apis/approval/v4/external_instances | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/external_instance/create) | - | 未找到 | - |
| 133 | 审批任务加签 | POST | /open-apis/approval/v4/instances/add_sign | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/ukTM5UjL5ETO14SOxkTN/approval-task-addsign) | - | 未找到 | - |
| 134 | 抄送审批实例 | POST | /open-apis/approval/v4/instances/cc | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/instance/cc) | - | 未找到 | - |
| 135 | 拒绝审批任务 | POST | /open-apis/approval/v4/tasks/reject | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/task/reject) | - | 未找到 | - |
| 136 | 撤回审批实例 | POST | /open-apis/approval/v4/instances/cancel | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/instance/cancel) | - | 未找到 | - |
| 137 | 查询任务列表 | POST | /open-apis/approval/v4/tasks/search | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/task/search) | - | 未找到 | - |
| 138 | 查询实例列表 | POST | /open-apis/approval/v4/instances/query | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/instance/query) | - | 未找到 | - |
| 139 | 查询抄送列表 | POST | /open-apis/approval/v4/instances/search_cc | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/instance/search_cc) | - | 未找到 | - |
| 140 | 校验三方审批实例 | POST | /open-apis/approval/v4/external_instances/check | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/external_instance/check) | - | 未找到 | - |
| 141 | 清空评论 | POST | /open-apis/approval/v4/instances/:instance_id/comments/remove | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/instance-comment/remove) | - | 未找到 | - |
| 142 | 订阅审批事件 | POST | /open-apis/approval/v4/approvals/:approval_code/subscribe | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/approval/subscribe) | - | 未找到 | - |
| 143 | 转交审批任务 | POST | /open-apis/approval/v4/tasks/transfer | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/task/transfer) | - | 未找到 | - |
| 144 | 退回审批任务 | POST | /open-apis/approval/v4/instances/specified_rollback | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/instance/specified_rollback) | - | 未找到 | - |
| 145 | 重新提交审批任务 | POST | /open-apis/approval/v4/tasks/resubmit | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/task/resubmit) | - | 未找到 | - |
| 146 | 预览审批流程 | POST | /open-apis/approval/v4/instances/preview | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/ukTM5UjL5ETO14SOxkTN/approval-preview) | - | 未找到 | - |
| 147 | 删除评论 | DELETE | /open-apis/approval/v4/instances/:instance_id/comments/:comment_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/instance-comment/delete) | - | 未找到 | - |

### 📦 attendance 模块

| 148 | 下载用户人脸识别照片 | GET | /open-apis/attendance/v1/files/:file_id/download | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/file/download) | - | 未找到 | - |
| 149 | 批量查询用户人脸识别信息 | GET | /open-apis/attendance/v1/user_settings/query | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_setting/query) | - | 未找到 | - |
| 150 | 按 ID 查询班次 | GET | /open-apis/attendance/v1/shifts/:shift_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/shift/get) | - | 未找到 | - |
| 151 | 按 ID 查询考勤组 | GET | /open-apis/attendance/v1/groups/:group_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/group/get) | - | 未找到 | - |
| 152 | 查询所有归档规则 | GET | /open-apis/attendance/v1/archive_rule | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/archive_rule/list) | - | 未找到 | - |
| 153 | 查询所有班次 | GET | /open-apis/attendance/v1/shifts | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/shift/list) | - | 未找到 | - |
| 154 | 查询所有考勤组 | GET | /open-apis/attendance/v1/groups | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/group/list) | - | 未找到 | - |
| 155 | 查询打卡流水 | GET | /open-apis/attendance/v1/user_flows/:user_flow_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_flow/get) | - | 未找到 | - |
| 156 | 查询考勤组下所有成员 | GET | /open-apis/attendance/v1/groups/:group_id/list_user | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/group/list_user) | - | 未找到 | - |
| 157 | 通过过期时间获取发放记录 | GET | /open-apis/attendance/v1/leave_employ_expire_records/:leave_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/leave_employ_expire_record/get) | - | 未找到 | - |
| 158 | 上传用户人脸识别照片 | POST | /open-apis/attendance/v1/files/upload | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/file/upload) | - | 未找到 | - |
| 159 | 修改用户人脸识别信息 | POST | /open-apis/attendance/v1/user_settings/modify | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_setting/modify) | - | 未找到 | - |
| 160 | 写入审批结果 | POST | /open-apis/attendance/v1/user_approvals | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_approval/create) | - | 未找到 | - |
| 161 | 写入归档报表结果 | POST | /open-apis/attendance/v1/archive_rule/upload_report | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/archive_rule/upload_report) | - | 未找到 | - |
| 162 | 创建或修改临时排班 | POST | /open-apis/attendance/v1/user_daily_shifts/batch_create_temp | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_daily_shift/batch_create_temp) | - | 未找到 | - |
| 163 | 创建或修改排班表 | POST | /open-apis/attendance/v1/user_daily_shifts/batch_create | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_daily_shift/batch_create) | - | 未找到 | - |
| 164 | 创建或修改考勤组 | POST | /open-apis/attendance/v1/groups | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/group/create) | - | 未找到 | - |
| 165 | 创建班次 | POST | /open-apis/attendance/v1/shifts | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/shift/create) | - | 未找到 | - |
| 166 | 删除归档报表行数据 | POST | /open-apis/attendance/v1/archive_rule/del_report | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/archive_rule/del_report) | - | 未找到 | - |
| 167 | 删除打卡流水 | POST | /open-apis/attendance/v1/user_flows/batch_del | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_flow/batch_del) | - | 未找到 | - |
| 168 | 导入打卡流水 | POST | /open-apis/attendance/v1/user_flows/batch_create | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_flow/batch_create) | - | 未找到 | - |
| 169 | 批量查询打卡流水 | POST | /open-apis/attendance/v1/user_flows/query | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_flow/query) | - | 未找到 | - |
| 170 | 按名称查询班次 | POST | /open-apis/attendance/v1/shifts/query | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/shift/query) | - | 未找到 | - |
| 171 | 按名称查询考勤组 | POST | /open-apis/attendance/v1/groups/search | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/group/search) | - | 未找到 | - |
| 172 | 查询归档报表表头 | POST | /open-apis/attendance/v1/archive_rule/user_stats_fields_query | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/archive_rule/user_stats_fields_query) | - | 未找到 | - |
| 173 | 查询打卡结果 | POST | /open-apis/attendance/v1/user_tasks/query | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_task/query) | - | 未找到 | - |
| 174 | 查询排班表 | POST | /open-apis/attendance/v1/user_daily_shifts/query | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_daily_shift/query) | - | 未找到 | - |
| 175 | 查询统计数据 | POST | /open-apis/attendance/v1/user_stats_datas/query | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_stats_data/query) | - | 未找到 | - |
| 176 | 查询统计表头 | POST | /open-apis/attendance/v1/user_stats_fields/query | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_stats_field/query) | - | 未找到 | - |
| 177 | 查询统计设置 | POST | /open-apis/attendance/v1/user_stats_views/query | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_stats_view/query) | - | 未找到 | - |
| 178 | 获取可补卡时间 | POST | /open-apis/attendance/v1/user_task_remedys/query_user_allowed_remedys | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_task_remedy/query_user_allowed_remedys) | - | 未找到 | - |
| 179 | 获取审批数据 | POST | /open-apis/attendance/v1/user_approvals/query | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_approval/query) | - | 未找到 | - |
| 180 | 获取补卡记录 | POST | /open-apis/attendance/v1/user_task_remedys/query | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_task_remedy/query) | - | 未找到 | - |
| 181 | 通知审批状态更新 | POST | /open-apis/attendance/v1/approval_infos/process | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/approval_info/process) | - | 未找到 | - |
| 182 | 通知补卡审批发起 | POST | /open-apis/attendance/v1/user_task_remedys | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_task_remedy/create) | - | 未找到 | - |
| 183 | 更新统计设置 | PUT | /open-apis/attendance/v1/user_stats_views/:user_stats_view_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_stats_view/update) | - | 未找到 | - |
| 184 | 修改发放记录 | PATCH | /open-apis/attendance/v1/leave_accrual_record/:leave_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/leave_accrual_record/patch) | - | 未找到 | - |
| 185 | 删除班次 | DELETE | /open-apis/attendance/v1/shifts/:shift_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/shift/delete) | - | 未找到 | - |
| 186 | 删除考勤组 | DELETE | /open-apis/attendance/v1/groups/:group_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/group/delete) | - | 未找到 | - |

### 📦 auth 模块

| 187 | 商店应用获取 app_access_token | POST | /open-apis/auth/v3/app_access_token | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/ukDNz4SO0MjL5QzM/auth-v3/auth/app_access_token) | - | 未找到 | - |
| 188 | 商店应用获取 tenant_access_token | POST | /open-apis/auth/v3/tenant_access_token | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/ukDNz4SO0MjL5QzM/auth-v3/auth/tenant_access_token) | - | 未找到 | - |
| 189 | 自建应用获取 app_access_token | POST | /open-apis/auth/v3/app_access_token/internal | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/ukDNz4SO0MjL5QzM/auth-v3/auth/app_access_token_internal) | - | 未找到 | - |
| 190 | 自建应用获取 tenant_access_token | POST | /open-apis/auth/v3/tenant_access_token/internal | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/ukDNz4SO0MjL5QzM/auth-v3/auth/tenant_access_token_internal) | - | 未找到 | - |
| 191 | 重新获取 app_ticket | POST | /open-apis/auth/v3/app_ticket/resend | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/ukDNz4SO0MjL5QzM/auth-v3/auth/app_ticket_resend) | - | 未找到 | - |

### 📦 authen 模块

| 192 | 获取用户信息 | GET | /open-apis/authen/v1/user_info | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/authen-v1/user_info/get) | - | 未找到 | - |
| 193 | 获取登录预授权码 | GET | /open-apis/authen/v1/index | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/ukzN4UjL5cDO14SO3gTN) | - | 未找到 | - |
| 194 | 刷新 user_access_token | POST | /open-apis/authen/v1/oidc/refresh_access_token | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/authen-v1/oidc-refresh_access_token/create) | - | 未找到 | - |
| 195 | 刷新 user_access_token（v1 版本） | POST | /open-apis/authen/v1/refresh_access_token | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/authen-v1/refresh_access_token/create) | - | 未找到 | - |
| 196 | 获取 user_access_token | POST | /open-apis/authen/v1/oidc/access_token | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/authen-v1/oidc-access_token/create) | - | 未找到 | - |
| 197 | 获取 user_access_token（v1 版本） | POST | /open-apis/authen/v1/access_token | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/authen-v1/access_token/create) | - | 未找到 | - |

### 📦 baike 模块

| 198 | 下载图片 | GET | /open-apis/baike/v1/files/:file_token/download | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/baike-v1/file/download) | - | 未找到 | - |
| 199 | 获取词典分类 | GET | /open-apis/baike/v1/classifications | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/baike-v1/classification/list) | - | 未找到 | - |
| 200 | 获取词条列表 | GET | /open-apis/baike/v1/entities | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/baike-v1/entity/list) | - | 未找到 | - |
| 201 | 获取词条详情 | GET | /open-apis/baike/v1/entities/:entity_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/baike-v1/entity/get) | - | 未找到 | - |
| 202 | 上传图片 | POST | /open-apis/baike/v1/files/upload | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/baike-v1/file/upload) | - | 未找到 | - |
| 203 | 创建免审词条 | POST | /open-apis/baike/v1/entities | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/baike-v1/entity/create) | - | 未找到 | - |
| 204 | 创建草稿 | POST | /open-apis/baike/v1/drafts | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/baike-v1/draft/create) | - | 未找到 | - |
| 205 | 提取潜在的词条 | POST | /open-apis/baike/v1/entities/extract | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/baike-v1/entity/extract) | - | 未找到 | - |
| 206 | 模糊搜索词条 | POST | /open-apis/baike/v1/entities/search | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/baike-v1/entity/search) | - | 未找到 | - |
| 207 | 精准搜索词条 | POST | /open-apis/baike/v1/entities/match | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/baike-v1/entity/match) | - | 未找到 | - |
| 208 | 词条高亮 | POST | /open-apis/baike/v1/entities/highlight | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/baike-v1/entity/highlight) | - | 未找到 | - |
| 209 | 更新免审词条 | PUT | /open-apis/baike/v1/entities/:entity_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/baike-v1/entity/update) | - | 未找到 | - |
| 210 | 更新草稿 | PUT | /open-apis/baike/v1/drafts/:draft_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/baike-v1/draft/update) | - | 未找到 | - |

### 📦 base 模块

| 211 | 列出自定义角色 | GET | /open-apis/base/v2/apps/:app_token/roles | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/advanced-permission/base-v2/app-role/list) | - | 未找到 | - |
| 212 | 新增自定义角色 | POST | /open-apis/base/v2/apps/:app_token/roles | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/advanced-permission/base-v2/app-role/create) | - | 未找到 | - |
| 213 | 更新自定义角色 | PUT | /open-apis/base/v2/apps/:app_token/roles/:role_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/advanced-permission/base-v2/app-role/update) | - | 未找到 | - |

### 📦 bitable 模块

| 214 | 列出仪表盘 | GET | /open-apis/bitable/v1/apps/:app_token/dashboards | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-dashboard/list) | - | 未找到 | - |
| 215 | 列出协作者 | GET | /open-apis/bitable/v1/apps/:app_token/roles/:role_id/members | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-role-member/list) | - | 未找到 | - |
| 216 | 列出字段 | GET | /open-apis/bitable/v1/apps/:app_token/tables/:table_id/fields | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-field/list) | - | 未找到 | - |
| 217 | 列出数据表 | GET | /open-apis/bitable/v1/apps/:app_token/tables | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table/list) | - | 未找到 | - |
| 218 | 列出自动化流程 | GET | /open-apis/bitable/v1/apps/:app_token/workflows | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-workflow/list) | - | 未找到 | - |
| 219 | 列出自定义角色 | GET | /open-apis/bitable/v1/apps/:app_token/roles | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-role/list) | - | 未找到 | - |
| 220 | 列出表单问题 | GET | /open-apis/bitable/v1/apps/:app_token/tables/:table_id/forms/:form_id/fields | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-form-field/list) | - | 未找到 | - |
| 221 | 列出视图 | GET | /open-apis/bitable/v1/apps/:app_token/tables/:table_id/views | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-view/list) | - | 未找到 | - |
| 222 | 列出记录 | GET | /open-apis/bitable/v1/apps/:app_token/tables/:table_id/records | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-record/list) | - | 未找到 | - |
| 223 | 检索记录 | GET | /open-apis/bitable/v1/apps/:app_token/tables/:table_id/records/:record_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-record/get) | - | 未找到 | - |
| 224 | 获取多维表格元数据 | GET | /open-apis/bitable/v1/apps/:app_token | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app/get) | - | 未找到 | - |
| 225 | 获取表单元数据 | GET | /open-apis/bitable/v1/apps/:app_token/tables/:table_id/forms/:form_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-form/get) | - | 未找到 | - |
| 226 | 获取视图 | GET | /open-apis/bitable/v1/apps/:app_token/tables/:table_id/views/:view_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-view/get) | - | 未找到 | - |
| 227 | 创建多维表格 | POST | /open-apis/bitable/v1/apps | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app/create) | - | 未找到 | - |
| 228 | 删除多个数据表 | POST | /open-apis/bitable/v1/apps/:app_token/tables/batch_delete | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table/batch_delete) | - | 未找到 | - |
| 229 | 删除多条记录 | POST | /open-apis/bitable/v1/apps/:app_token/tables/:table_id/records/batch_delete | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-record/batch_delete) | - | 未找到 | - |
| 230 | 复制仪表盘 | POST | /open-apis/bitable/v1/apps/:app_token/dashboards/:block_id/copy | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-dashboard/copy) | - | 未找到 | - |
| 231 | 复制多维表格 | POST | /open-apis/bitable/v1/apps/:app_token/copy | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app/copy) | - | 未找到 | - |
| 232 | 批量删除协作者 | POST | /open-apis/bitable/v1/apps/:app_token/roles/:role_id/members/batch_delete | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-role-member/batch_delete) | - | 未找到 | - |
| 233 | 批量新增协作者 | POST | /open-apis/bitable/v1/apps/:app_token/roles/:role_id/members/batch_create | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-role-member/batch_create) | - | 未找到 | - |
| 234 | 批量获取记录 | POST | /open-apis/bitable/v1/apps/:app_token/tables/:table_id/records/batch_get | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-record/batch_get) | - | 未找到 | - |
| 235 | 新增一个数据表 | POST | /open-apis/bitable/v1/apps/:app_token/tables | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table/create) | - | 未找到 | - |
| 236 | 新增协作者 | POST | /open-apis/bitable/v1/apps/:app_token/roles/:role_id/members | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-role-member/create) | - | 未找到 | - |
| 237 | 新增多个数据表 | POST | /open-apis/bitable/v1/apps/:app_token/tables/batch_create | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table/batch_create) | - | 未找到 | - |
| 238 | 新增多条记录 | POST | /open-apis/bitable/v1/apps/:app_token/tables/:table_id/records/batch_create | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-record/batch_create) | - | 未找到 | - |
| 239 | 新增字段 | POST | /open-apis/bitable/v1/apps/:app_token/tables/:table_id/fields | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-field/create) | - | 未找到 | - |
| 240 | 新增自定义角色 | POST | /open-apis/bitable/v1/apps/:app_token/roles | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-role/create) | - | 未找到 | - |
| 241 | 新增视图 | POST | /open-apis/bitable/v1/apps/:app_token/tables/:table_id/views | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-view/create) | - | 未找到 | - |
| 242 | 新增记录 | POST | /open-apis/bitable/v1/apps/:app_token/tables/:table_id/records | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-record/create) | - | 未找到 | - |
| 243 | 更新多条记录 | POST | /open-apis/bitable/v1/apps/:app_token/tables/:table_id/records/batch_update | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-record/batch_update) | - | 未找到 | - |
| 244 | 查询记录 | POST | /open-apis/bitable/v1/apps/:app_token/tables/:table_id/records/search | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-record/search) | - | 未找到 | - |
| 245 | 更新多维表格元数据 | PUT | /open-apis/bitable/v1/apps/:app_token | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app/update) | - | 未找到 | - |
| 246 | 更新字段 | PUT | /open-apis/bitable/v1/apps/:app_token/tables/:table_id/fields/:field_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-field/update) | - | 未找到 | - |
| 247 | 更新自动化流程状态 | PUT | /open-apis/bitable/v1/apps/:app_token/workflows/:workflow_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-workflow/update) | - | 未找到 | - |
| 248 | 更新自定义角色 | PUT | /open-apis/bitable/v1/apps/:app_token/roles/:role_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-role/update) | - | 未找到 | - |
| 249 | 更新记录 | PUT | /open-apis/bitable/v1/apps/:app_token/tables/:table_id/records/:record_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-record/update) | - | 未找到 | - |
| 250 | 更新数据表 | PATCH | /open-apis/bitable/v1/apps/:app_token/tables/:table_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table/patch) | - | 未找到 | - |
| 251 | 更新表单元数据 | PATCH | /open-apis/bitable/v1/apps/:app_token/tables/:table_id/forms/:form_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-form/patch) | - | 未找到 | - |
| 252 | 更新表单问题 | PATCH | /open-apis/bitable/v1/apps/:app_token/tables/:table_id/forms/:form_id/fields/:field_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-form-field/patch) | - | 未找到 | - |
| 253 | 更新视图 | PATCH | /open-apis/bitable/v1/apps/:app_token/tables/:table_id/views/:view_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-view/patch) | - | 未找到 | - |
| 254 | 删除一个数据表 | DELETE | /open-apis/bitable/v1/apps/:app_token/tables/:table_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table/delete) | - | 未找到 | - |
| 255 | 删除协作者 | DELETE | /open-apis/bitable/v1/apps/:app_token/roles/:role_id/members/:member_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-role-member/delete) | - | 未找到 | - |
| 256 | 删除字段 | DELETE | /open-apis/bitable/v1/apps/:app_token/tables/:table_id/fields/:field_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-field/delete) | - | 未找到 | - |
| 257 | 删除自定义角色 | DELETE | /open-apis/bitable/v1/apps/:app_token/roles/:role_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-role/delete) | - | 未找到 | - |
| 258 | 删除视图 | DELETE | /open-apis/bitable/v1/apps/:app_token/tables/:table_id/views/:view_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-view/delete) | - | 未找到 | - |
| 259 | 删除记录 | DELETE | /open-apis/bitable/v1/apps/:app_token/tables/:table_id/records/:record_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-record/delete) | - | 未找到 | - |

### 📦 board 模块

| 260 | 获取所有节点 | GET | /open-apis/board/v1/whiteboards/:whiteboard_id/nodes | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/board-v1/whiteboard-node/list) | - | 未找到 | - |
| 261 | 获取画板主题 | GET | /open-apis/board/v1/whiteboards/:whiteboard_id/theme | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/board-v1/whiteboard/theme) | - | 未找到 | - |
| 262 | 获取画板缩略图片 | GET | /open-apis/board/v1/whiteboards/:whiteboard_id/download_as_image | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/board-v1/whiteboard/download_as_image) | - | 未找到 | - |
| 263 | 创建节点 | POST | /open-apis/board/v1/whiteboards/:whiteboard_id/nodes | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/board-v1/whiteboard-node/create) | - | 未找到 | - |
| 264 | 更新画板主题 | POST | /open-apis/board/v1/whiteboards/:whiteboard_id/update_theme | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/board-v1/whiteboard/update_theme) | - | 未找到 | - |
| 265 | 解析画板语法 | POST | /open-apis/board/v1/whiteboards/:whiteboard_id/nodes/plantuml | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/board-v1/whiteboard-node/create_plantuml) | - | 未找到 | - |

### 📦 calendar 模块

| 266 | 查询 Exchange 账户的绑定状态 | GET | /open-apis/calendar/v4/exchange_bindings/:exchange_binding_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/exchange_binding/get) | - | 未找到 | - |
| 267 | 查询日历信息 | GET | /open-apis/calendar/v4/calendars/:calendar_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar/get) | - | 未找到 | - |
| 268 | 查询日历列表 | GET | /open-apis/calendar/v4/calendars | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar/list) | - | 未找到 | - |
| 269 | 查询日程视图 | GET | /open-apis/calendar/v4/calendars/:calendar_id/events/instance_view | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar-event/instance_view) | - | 未找到 | - |
| 270 | 获取日程 | GET | /open-apis/calendar/v4/calendars/:calendar_id/events/:event_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar-event/get) | - | 未找到 | - |
| 271 | 获取日程列表 | GET | /open-apis/calendar/v4/calendars/:calendar_id/events | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar-event/list) | - | 未找到 | - |
| 272 | 获取日程参与人列表 | GET | /open-apis/calendar/v4/calendars/:calendar_id/events/:event_id/attendees | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar-event-attendee/list) | - | 未找到 | - |
| 273 | 获取日程参与群成员列表 | GET | /open-apis/calendar/v4/calendars/:calendar_id/events/:event_id/attendees/:attendee_id/chat_members | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar-event-attendee-chat_member/list) | - | 未找到 | - |
| 274 | 获取访问控制列表 | GET | /open-apis/calendar/v4/calendars/:calendar_id/acls | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar-acl/list) | - | 未找到 | - |
| 275 | 获取重复日程实例 | GET | /open-apis/calendar/v4/calendars/:calendar_id/events/:event_id/instances | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar-event/instances) | - | 未找到 | - |
| 276 | 创建会议纪要 | POST | /open-apis/calendar/v4/calendars/:calendar_id/events/:event_id/meeting_minute | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar-event-meeting_minute/create) | - | 未找到 | - |
| 277 | 创建会议群 | POST | /open-apis/calendar/v4/calendars/:calendar_id/events/:event_id/meeting_chat | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar-event-meeting_chat/create) | - | 未找到 | - |
| 278 | 创建共享日历 | POST | /open-apis/calendar/v4/calendars | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar/create) | - | 未找到 | - |
| 279 | 创建日程 | POST | /open-apis/calendar/v4/calendars/:calendar_id/events | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar-event/create) | - | 未找到 | - |
| 280 | 创建访问控制 | POST | /open-apis/calendar/v4/calendars/:calendar_id/acls | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar-acl/create) | - | 未找到 | - |
| 281 | 创建请假日程 | POST | /open-apis/calendar/v4/timeoff_events | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/timeoff_event/create) | - | 未找到 | - |
| 282 | 删除日程参与人 | POST | /open-apis/calendar/v4/calendars/:calendar_id/events/:event_id/attendees/batch_delete | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar-event-attendee/batch_delete) | - | 未找到 | - |
| 283 | 取消订阅日历 | POST | /open-apis/calendar/v4/calendars/:calendar_id/unsubscribe | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar/unsubscribe) | - | 未找到 | - |
| 284 | 取消订阅日历变更事件 | POST | /open-apis/calendar/v4/calendars/unsubscription | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar/unsubscription) | - | 未找到 | - |
| 285 | 取消订阅日历访问控制变更事件 | POST | /open-apis/calendar/v4/calendars/:calendar_id/acls/unsubscription | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar-acl/unsubscription) | - | 未找到 | - |
| 286 | 取消订阅日程变更事件 | POST | /open-apis/calendar/v4/calendars/:calendar_id/events/unsubscription | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar-event/unsubscription) | - | 未找到 | - |
| 287 | 回复日程 | POST | /open-apis/calendar/v4/calendars/:calendar_id/events/:event_id/reply | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar-event/reply) | - | 未找到 | - |
| 288 | 将 Exchange 账户绑定到飞书账户 | POST | /open-apis/calendar/v4/exchange_bindings | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/exchange_binding/create) | - | 未找到 | - |
| 289 | 批量查询主日历日程忙闲信息 | POST | /open-apis/calendar/v4/freebusy/batch | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/freebusy/batch) | - | 未找到 | - |
| 290 | 批量查询日历信息 | POST | /open-apis/calendar/v4/calendars/mget | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar/mget) | - | 未找到 | - |
| 291 | 批量获取主日历信息 | POST | /open-apis/calendar/v4/calendars/primarys | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar/primarys) | - | 未找到 | - |
| 292 | 搜索日历 | POST | /open-apis/calendar/v4/calendars/search | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar/search) | - | 未找到 | - |
| 293 | 搜索日程 | POST | /open-apis/calendar/v4/calendars/:calendar_id/events/search | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar-event/search) | - | 未找到 | - |
| 294 | 查询主日历信息 | POST | /open-apis/calendar/v4/calendars/primary | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar/primary) | - | 未找到 | - |
| 295 | 查询主日历日程忙闲信息 | POST | /open-apis/calendar/v4/freebusy/list | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/freebusy/list) | - | 未找到 | - |
| 296 | 添加日程参与人 | POST | /open-apis/calendar/v4/calendars/:calendar_id/events/:event_id/attendees | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar-event-attendee/create) | - | 未找到 | - |
| 297 | 生成 CalDAV 配置 | POST | /open-apis/calendar/v4/settings/generate_caldav_conf | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/setting/generate_caldav_conf) | - | 未找到 | - |
| 298 | 订阅日历 | POST | /open-apis/calendar/v4/calendars/:calendar_id/subscribe | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar/subscribe) | - | 未找到 | - |
| 299 | 订阅日历变更事件 | POST | /open-apis/calendar/v4/calendars/subscription | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar/subscription) | - | 未找到 | - |
| 300 | 订阅日历访问控制变更事件 | POST | /open-apis/calendar/v4/calendars/:calendar_id/acls/subscription | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar-acl/subscription) | - | 未找到 | - |
| 301 | 订阅日程变更事件 | POST | /open-apis/calendar/v4/calendars/:calendar_id/events/subscription | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar-event/subscription) | - | 未找到 | - |
| 302 | 更新日历信息 | PATCH | /open-apis/calendar/v4/calendars/:calendar_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar/patch) | - | 未找到 | - |
| 303 | 更新日程 | PATCH | /open-apis/calendar/v4/calendars/:calendar_id/events/:event_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar-event/patch) | - | 未找到 | - |
| 304 | 删除共享日历 | DELETE | /open-apis/calendar/v4/calendars/:calendar_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar/delete) | - | 未找到 | - |
| 305 | 删除日程 | DELETE | /open-apis/calendar/v4/calendars/:calendar_id/events/:event_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar-event/delete) | - | 未找到 | - |
| 306 | 删除访问控制 | DELETE | /open-apis/calendar/v4/calendars/:calendar_id/acls/:acl_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar-acl/delete) | - | 未找到 | - |
| 307 | 删除请假日程 | DELETE | /open-apis/calendar/v4/timeoff_events/:timeoff_event_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/timeoff_event/delete) | - | 未找到 | - |
| 308 | 解绑会议群 | DELETE | /open-apis/calendar/v4/calendars/:calendar_id/events/:event_id/meeting_chat | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar-event-meeting_chat/delete) | - | 未找到 | - |
| 309 | 解除 Exchange 账户绑定 | DELETE | /open-apis/calendar/v4/exchange_bindings/:exchange_binding_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/exchange_binding/delete) | - | 未找到 | - |

### 📦 cardkit 模块

| 310 | 创建卡片实体 | POST | /open-apis/cardkit/v1/cards | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/cardkit-v1/card/create) | - | 未找到 | - |
| 311 | 局部更新卡片实体 | POST | /open-apis/cardkit/v1/cards/:card_id/batch_update | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/cardkit-v1/card/batch_update) | - | 未找到 | - |
| 312 | 新增组件 | POST | /open-apis/cardkit/v1/cards/:card_id/elements | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/cardkit-v1/card-element/create) | - | 未找到 | - |
| 313 | 转换 ID | POST | /open-apis/cardkit/v1/cards/id_convert | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/cardkit-v1/card/id_convert) | - | 未找到 | - |
| 314 | 全量更新卡片实体 | PUT | /open-apis/cardkit/v1/cards/:card_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/cardkit-v1/card/update) | - | 未找到 | - |
| 315 | 更新组件 | PUT | /open-apis/cardkit/v1/cards/:card_id/elements/:element_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/cardkit-v1/card-element/update) | - | 未找到 | - |
| 316 | 流式更新文本 | PUT | /open-apis/cardkit/v1/cards/:card_id/elements/:element_id/content | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/cardkit-v1/card-element/content) | - | 未找到 | - |
| 317 | 更新卡片实体配置 | PATCH | /open-apis/cardkit/v1/cards/:card_id/settings | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/cardkit-v1/card/settings) | - | 未找到 | - |
| 318 | 更新组件属性 | PATCH | /open-apis/cardkit/v1/cards/:card_id/elements/:element_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/cardkit-v1/card-element/patch) | - | 未找到 | - |
| 319 | 删除组件 | DELETE | /open-apis/cardkit/v1/cards/:card_id/elements/:element_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/cardkit-v1/card-element/delete) | - | 未找到 | - |

### 📦 compensation 模块

| 320 | 批量查询定调薪原因 | GET | /open-apis/compensation/v1/change_reasons | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/compensation-v1/change_reason/list) | - | 未找到 | - |
| 321 | 批量查询薪资方案 | GET | /open-apis/compensation/v1/plans | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/compensation-v1/plan/list) | - | 未找到 | - |
| 322 | 批量查询薪资统计指标 | GET | /open-apis/compensation/v1/indicators | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/compensation-v1/indicator/list) | - | 未找到 | - |
| 323 | 批量查询薪资项 | GET | /open-apis/compensation/v1/items | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/compensation-v1/item/list) | - | 未找到 | - |
| 324 | 批量获取薪资项分类信息 | GET | /open-apis/compensation/v1/item_categories | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/compensation-v1/item_category/list) | - | 未找到 | - |
| 325 | 根据生效日期分页查询参保方案 | GET | /open-apis/compensation/v1/social_plans | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/compensation-v1/social_plan/list) | - | 未找到 | - |
| 326 | 获取险种配置列表 | GET | /open-apis/compensation/v1/social_insurances | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/compensation-v1/social_insurance/list) | - | 未找到 | - |
| 327 | 创建薪资档案 | POST | /open-apis/compensation/v1/archives | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/compensation-v1/archive/create) | - | 未找到 | - |
| 328 | 批量创建一次性支付记录 | POST | /open-apis/compensation/v1/lump_sum_payment/batch_create | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/compensation-v1/lump_sum_payment/batch_create) | - | 未找到 | - |
| 329 | 批量创建经常性支付记录 | POST | /open-apis/compensation/v1/recurring_payment/batch_create | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/compensation-v1/recurring_payment/batch_create) | - | 未找到 | - |
| 330 | 批量删除一次性支付记录 | POST | /open-apis/compensation/v1/lump_sum_payment/batch_remove | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/compensation-v1/lump_sum_payment/batch_remove) | - | 未找到 | - |
| 331 | 批量删除经常性支付记录 | POST | /open-apis/compensation/v1/recurring_payment/batch_remove | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/compensation-v1/recurring_payment/batch_remove) | - | 未找到 | - |
| 332 | 批量更正一次性支付记录 | POST | /open-apis/compensation/v1/lump_sum_payment/batch_update | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/compensation-v1/lump_sum_payment/batch_update) | - | 未找到 | - |
| 333 | 批量更正经常性支付记录 | POST | /open-apis/compensation/v1/recurring_payment/batch_update | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/compensation-v1/recurring_payment/batch_update) | - | 未找到 | - |
| 334 | 批量查询员工薪资档案 | POST | /open-apis/compensation/v1/archives/query | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/compensation-v1/archive/query) | - | 未找到 | - |
| 335 | 批量获取员工参保档案 | POST | /open-apis/compensation/v1/social_archive/query | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/compensation-v1/social_archive/query) | - | 未找到 | - |
| 336 | 查询一次性支付授予明细 | POST | /open-apis/compensation/v1/lump_sum_payment/query_detail | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/compensation-v1/lump_sum_payment/query_detail) | - | 未找到 | - |
| 337 | 查询一次性支付授予记录 | POST | /open-apis/compensation/v1/lump_sum_payment/query | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/compensation-v1/lump_sum_payment/query) | - | 未找到 | - |
| 338 | 查询经常性支付记录 | POST | /open-apis/compensation/v1/recurring_payment/query | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/compensation-v1/recurring_payment/query) | - | 未找到 | - |
| 339 | 根据方案ID和生效日期批量查询参保方案 | POST | /open-apis/compensation/v1/social_plans/query | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/compensation-v1/social_plan/query) | - | 未找到 | - |
| 340 | 通过员工ID批量获取社保增减员记录 | POST | /open-apis/compensation/v1/social_archive_adjust_record/query | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/compensation-v1/social_archive_adjust_record/query) | - | 未找到 | - |

### 📦 contact 模块

| 341 | 批量获取用户信息 | GET | /open-apis/contact/v3/users/batch | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/user/batch) | - | 未找到 | - |
| 342 | 批量获取部门信息 | GET | /open-apis/contact/v3/departments/batch | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/batch) | - | 未找到 | - |
| 343 | 查询人员类型 | GET | /open-apis/contact/v3/employee_type_enums | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/employee_type_enum/list) | - | 未找到 | - |
| 344 | 查询批量任务执行状态 | GET | /open-apis/contact/v2/task/get | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uUDOwUjL1gDM14SN4ATN) | - | 未找到 | - |
| 345 | 查询指定用户组 | GET | /open-apis/contact/v3/group/:group_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/group/get) | - | 未找到 | - |
| 346 | 查询用户所属用户组 | GET | /open-apis/contact/v3/group/member_belong | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/group/member_belong) | - | 未找到 | - |
| 347 | 查询用户组列表 | GET | /open-apis/contact/v3/group/simplelist | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/group/simplelist) | - | 未找到 | - |
| 348 | 查询用户组成员列表 | GET | /open-apis/contact/v3/group/:group_id/member/simplelist | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/group-member/simplelist) | - | 未找到 | - |
| 349 | 查询角色下某个成员的管理范围 | GET | /open-apis/contact/v3/functional_roles/:role_id/members/:member_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/functional_role-member/get) | - | 未找到 | - |
| 350 | 查询角色下的所有成员信息 | GET | /open-apis/contact/v3/functional_roles/:role_id/members | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/functional_role-member/list) | - | 未找到 | - |
| 351 | 获取企业自定义用户字段 | GET | /open-apis/contact/v3/custom_attrs | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/custom_attr/list) | - | 未找到 | - |
| 352 | 获取单个工作城市信息 | GET | /open-apis/contact/v3/work_cities/:work_city_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/work_city/get) | - | 未找到 | - |
| 353 | 获取单个序列信息 | GET | /open-apis/contact/v3/job_families/:job_family_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/job_family/get) | - | 未找到 | - |
| 354 | 获取单个用户信息 | GET | /open-apis/contact/v3/users/:user_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/user/get) | - | 未找到 | - |
| 355 | 获取单个职务信息 | GET | /open-apis/contact/v3/job_titles/:job_title_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/job_title/get) | - | 未找到 | - |
| 356 | 获取单个职级信息 | GET | /open-apis/contact/v3/job_levels/:job_level_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/job_level/get) | - | 未找到 | - |
| 357 | 获取单个部门信息 | GET | /open-apis/contact/v3/departments/:department_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/get) | - | 未找到 | - |
| 358 | 获取单位信息 | GET | /open-apis/contact/v3/unit/:unit_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/unit/get) | - | 未找到 | - |
| 359 | 获取单位列表 | GET | /open-apis/contact/v3/unit | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/unit/list) | - | 未找到 | - |
| 360 | 获取单位绑定的部门列表 | GET | /open-apis/contact/v3/unit/list_department | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/unit/list_department) | - | 未找到 | - |
| 361 | 获取子部门列表 | GET | /open-apis/contact/v3/departments/:department_id/children | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/children) | - | 未找到 | - |
| 362 | 获取应用管理员管理范围 | GET | /open-apis/contact/v1/user/admin_scope/get | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzN3QjLzczN04yM3cDN) | - | 未找到 | - |
| 363 | 获取父部门信息 | GET | /open-apis/contact/v3/departments/parent | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/parent) | - | 未找到 | - |
| 364 | 获取用户列表 | GET | /open-apis/contact/v3/users | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/user/list) | - | 未找到 | - |
| 365 | 获取租户工作城市列表 | GET | /open-apis/contact/v3/work_cities | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/work_city/list) | - | 未找到 | - |
| 366 | 获取租户序列列表 | GET | /open-apis/contact/v3/job_families | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/job_family/list) | - | 未找到 | - |
| 367 | 获取租户职务列表 | GET | /open-apis/contact/v3/job_titles | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/job_title/list) | - | 未找到 | - |
| 368 | 获取租户职级列表 | GET | /open-apis/contact/v3/job_levels | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/job_level/list) | - | 未找到 | - |
| 369 | 获取角色列表 | GET | /open-apis/contact/v2/role/list | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uYzMwUjL2MDM14iNzATN) | - | 未找到 | - |
| 370 | 获取通讯录授权范围 | GET | /open-apis/contact/v3/scopes | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/scope/list) | - | 未找到 | - |
| 371 | 获取部门信息列表 | GET | /open-apis/contact/v3/departments | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/list) | - | 未找到 | - |
| 372 | 获取部门直属用户列表 | GET | /open-apis/contact/v3/users/find_by_department | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/user/find_by_department) | - | 未找到 | - |
| 373 | 创建单位 | POST | /open-apis/contact/v3/unit | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/unit/create) | - | 未找到 | - |
| 374 | 创建序列 | POST | /open-apis/contact/v3/job_families | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/job_family/create) | - | 未找到 | - |
| 375 | 创建用户 | POST | /open-apis/contact/v3/users | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/user/create) | - | 未找到 | - |
| 376 | 创建用户组 | POST | /open-apis/contact/v3/group | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/group/create) | - | 未找到 | - |
| 377 | 创建职级 | POST | /open-apis/contact/v3/job_levels | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/job_level/create) | - | 未找到 | - |
| 378 | 创建角色 | POST | /open-apis/contact/v3/functional_roles | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/functional_role/create) | - | 未找到 | - |
| 379 | 创建部门 | POST | /open-apis/contact/v3/departments | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/create) | - | 未找到 | - |
| 380 | 建立部门与单位的绑定关系 | POST | /open-apis/contact/v3/unit/bind_department | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/unit/bind_department) | - | 未找到 | - |
| 381 | 恢复已删除用户 | POST | /open-apis/contact/v3/users/:user_id/resurrect | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/user/resurrect) | - | 未找到 | - |
| 382 | 批量新增用户 | POST | /open-apis/contact/v2/user/batch_add | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uIDOwUjLygDM14iM4ATN) | - | 未找到 | - |
| 383 | 批量新增部门 | POST | /open-apis/contact/v2/department/batch_add | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMDOwUjLzgDM14yM4ATN) | - | 未找到 | - |
| 384 | 批量添加用户组成员 | POST | /open-apis/contact/v3/group/:group_id/member/batch_add | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/group-member/batch_add) | - | 未找到 | - |
| 385 | 批量添加角色成员 | POST | /open-apis/contact/v3/functional_roles/:role_id/members/batch_create | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/functional_role-member/batch_create) | - | 未找到 | - |
| 386 | 批量移除用户组成员 | POST | /open-apis/contact/v3/group/:group_id/member/batch_remove | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/group-member/batch_remove) | - | 未找到 | - |
| 387 | 搜索部门 | POST | /open-apis/contact/v3/departments/search | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/search) | - | 未找到 | - |
| 388 | 新增人员类型 | POST | /open-apis/contact/v3/employee_type_enums | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/employee_type_enum/create) | - | 未找到 | - |
| 389 | 添加用户组成员 | POST | /open-apis/contact/v3/group/:group_id/member/add | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/group-member/add) | - | 未找到 | - |
| 390 | 移除用户组成员 | POST | /open-apis/contact/v3/group/:group_id/member/remove | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/group-member/remove) | - | 未找到 | - |
| 391 | 解除部门与单位的绑定关系 | POST | /open-apis/contact/v3/unit/unbind_department | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/unit/unbind_department) | - | 未找到 | - |
| 392 | 通过手机号或邮箱获取用户 ID | POST | /open-apis/contact/v3/users/batch_get_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/user/batch_get_id) | - | 未找到 | - |
| 393 | 部门群转为普通群 | POST | /open-apis/contact/v3/departments/unbind_department_chat | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/unbind_department_chat) | - | 未找到 | - |
| 394 | 修改角色名称 | PUT | /open-apis/contact/v3/functional_roles/:role_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/functional_role/update) | - | 未找到 | - |
| 395 | 更新人员类型 | PUT | /open-apis/contact/v3/employee_type_enums/:enum_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/employee_type_enum/update) | - | 未找到 | - |
| 396 | 更新序列 | PUT | /open-apis/contact/v3/job_families/:job_family_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/job_family/update) | - | 未找到 | - |
| 397 | 更新用户所有信息 | PUT | /open-apis/contact/v3/users/:user_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/user/update) | - | 未找到 | - |
| 398 | 更新职级 | PUT | /open-apis/contact/v3/job_levels/:job_level_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/job_level/update) | - | 未找到 | - |
| 399 | 更新部门所有信息 | PUT | /open-apis/contact/v3/departments/:department_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/update) | - | 未找到 | - |
| 400 | 修改单位信息 | PATCH | /open-apis/contact/v3/unit/:unit_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/unit/patch) | - | 未找到 | - |
| 401 | 修改用户部分信息 | PATCH | /open-apis/contact/v3/users/:user_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/user/patch) | - | 未找到 | - |
| 402 | 修改部门部分信息 | PATCH | /open-apis/contact/v3/departments/:department_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/patch) | - | 未找到 | - |
| 403 | 删除角色下的成员 | PATCH | /open-apis/contact/v3/functional_roles/:role_id/members/batch_delete | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/functional_role-member/batch_delete) | - | 未找到 | - |
| 404 | 批量设置角色成员管理范围 | PATCH | /open-apis/contact/v3/functional_roles/:role_id/members/scopes | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/functional_role-member/scopes) | - | 未找到 | - |
| 405 | 更新用户 ID | PATCH | /open-apis/contact/v3/users/:user_id/update_user_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/user/update_user_id) | - | 未找到 | - |
| 406 | 更新用户组 | PATCH | /open-apis/contact/v3/group/:group_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/group/patch) | - | 未找到 | - |
| 407 | 更新部门 ID | PATCH | /open-apis/contact/v3/departments/:department_id/update_department_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/update_department_id) | - | 未找到 | - |
| 408 | 删除人员类型 | DELETE | /open-apis/contact/v3/employee_type_enums/:enum_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/employee_type_enum/delete) | - | 未找到 | - |
| 409 | 删除单位 | DELETE | /open-apis/contact/v3/unit/:unit_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/unit/delete) | - | 未找到 | - |
| 410 | 删除序列 | DELETE | /open-apis/contact/v3/job_families/:job_family_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/job_family/delete) | - | 未找到 | - |
| 411 | 删除用户 | DELETE | /open-apis/contact/v3/users/:user_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/user/delete) | - | 未找到 | - |
| 412 | 删除用户组 | DELETE | /open-apis/contact/v3/group/:group_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/group/delete) | - | 未找到 | - |
| 413 | 删除职级 | DELETE | /open-apis/contact/v3/job_levels/:job_level_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/job_level/delete) | - | 未找到 | - |
| 414 | 删除角色 | DELETE | /open-apis/contact/v3/functional_roles/:role_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/functional_role/delete) | - | 未找到 | - |
| 415 | 删除部门 | DELETE | /open-apis/contact/v3/departments/:department_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/department/delete) | - | 未找到 | - |

### 📦 corehr 模块

| 416 | 下载文件 | GET | /open-apis/corehr/v1/files/:id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/file/get) | - | 未找到 | - |
| 417 | 批量分页查询地点信息 | GET | /open-apis/corehr/v1/locations | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/location/list) | - | 未找到 | - |
| 418 | 批量查询人员类型 | GET | /open-apis/corehr/v1/employee_types | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/employee_type/list) | - | 未找到 | - |
| 419 | 批量查询任职信息 | GET | /open-apis/corehr/v1/job_datas | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/job_data/list) | - | 未找到 | - |
| 420 | 批量查询公司 | GET | /open-apis/corehr/v1/companies | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/company/list) | - | 未找到 | - |
| 421 | 批量查询合同 | GET | /open-apis/corehr/v1/contracts | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/contract/list) | - | 未找到 | - |
| 422 | 批量查询员工假期余额 | GET | /open-apis/corehr/v1/leaves/leave_balances | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/leave/leave_balances) | - | 未找到 | - |
| 423 | 批量查询员工请假记录 | GET | /open-apis/corehr/v1/leaves/leave_request_history | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/leave/leave_request_history) | - | 未找到 | - |
| 424 | 批量查询国家/地区信息 | GET | /open-apis/corehr/v1/country_regions | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/country_region/list) | - | 未找到 | - |
| 425 | 批量查询国家证件类型 | GET | /open-apis/corehr/v1/national_id_types | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/national_id_type/list) | - | 未找到 | - |
| 426 | 批量查询城市/区域信息 | GET | /open-apis/corehr/v1/subregions | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/subregion/list) | - | 未找到 | - |
| 427 | 批量查询外派信息 | GET | /open-apis/corehr/v2/employees/international_assignments | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/employees-international_assignment/list) | - | 未找到 | - |
| 428 | 批量查询工时制度 | GET | /open-apis/corehr/v1/working_hours_types | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/working_hours_type/list) | - | 未找到 | - |
| 429 | 批量查询序列 | GET | /open-apis/corehr/v1/job_families | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/job_family/list) | - | 未找到 | - |
| 430 | 批量查询待入职信息 | GET | /open-apis/corehr/v1/pre_hires | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/pre_hire/list) | - | 未找到 | - |
| 431 | 批量查询用户授权 | GET | /open-apis/corehr/v1/authorizations/query | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/authorization/query) | - | 未找到 | - |
| 432 | 批量查询省份/行政区信息 | GET | /open-apis/corehr/v1/subdivisions | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/subdivision/list) | - | 未找到 | - |
| 433 | 批量查询职务 | GET | /open-apis/corehr/v2/jobs | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/job/list) | - | 未找到 | - |
| 434 | 批量查询职务 | GET | /open-apis/corehr/v1/jobs | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/job/list) | - | 未找到 | - |
| 435 | 批量查询职级 | GET | /open-apis/corehr/v1/job_levels | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/job_level/list) | - | 未找到 | - |
| 436 | 批量查询货币信息 | GET | /open-apis/corehr/v1/currencies | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/currency/list) | - | 未找到 | - |
| 437 | 批量查询部门 | GET | /open-apis/corehr/v1/departments | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/department/list) | - | 未找到 | - |
| 438 | 批量获取角色列表 | GET | /open-apis/corehr/v1/security_groups | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/security_group/list) | - | 未找到 | - |
| 439 | 查询单个个人信息 | GET | /open-apis/corehr/v1/persons/:person_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/person/get) | - | 未找到 | - |
| 440 | 查询单个人员类型 | GET | /open-apis/corehr/v1/employee_types/:employee_type_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/employee_type/get) | - | 未找到 | - |
| 441 | 查询单个任职信息 | GET | /open-apis/corehr/v1/job_datas/:job_data_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/job_data/get) | - | 未找到 | - |
| 442 | 查询单个公司 | GET | /open-apis/corehr/v1/companies/:company_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/company/get) | - | 未找到 | - |
| 443 | 查询单个合同 | GET | /open-apis/corehr/v1/contracts/:contract_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/contract/get) | - | 未找到 | - |
| 444 | 查询单个国家证件类型 | GET | /open-apis/corehr/v1/national_id_types/:national_id_type_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/national_id_type/get) | - | 未找到 | - |
| 445 | 查询单个地点 | GET | /open-apis/corehr/v1/locations/:location_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/location/get) | - | 未找到 | - |
| 446 | 查询单个工时制度 | GET | /open-apis/corehr/v1/working_hours_types/:working_hours_type_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/working_hours_type/get) | - | 未找到 | - |
| 447 | 查询单个序列 | GET | /open-apis/corehr/v1/job_families/:job_family_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/job_family/get) | - | 未找到 | - |
| 448 | 查询单个待入职信息 | GET | /open-apis/corehr/v1/pre_hires/:pre_hire_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/pre_hire/get) | - | 未找到 | - |
| 449 | 查询单个用户授权 | GET | /open-apis/corehr/v1/authorizations/get_by_param | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/authorization/get_by_param) | - | 未找到 | - |
| 450 | 查询单个职务 | GET | /open-apis/corehr/v2/jobs/:job_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/job/get) | - | 未找到 | - |
| 451 | 查询单个职务 | GET | /open-apis/corehr/v1/jobs/:job_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/job/get) | - | 未找到 | - |
| 452 | 查询单个职级 | GET | /open-apis/corehr/v1/job_levels/:job_level_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/job_level/get) | - | 未找到 | - |
| 453 | 查询单个货币信息 | GET | /open-apis/corehr/v1/currencies/:currency_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/currency/get) | - | 未找到 | - |
| 454 | 查询单个部门 | GET | /open-apis/corehr/v1/departments/:department_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/department/get) | - | 未找到 | - |
| 455 | 查询单条国家/地区信息 | GET | /open-apis/corehr/v1/country_regions/:country_region_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/country_region/get) | - | 未找到 | - |
| 456 | 查询单条城市/区域信息 | GET | /open-apis/corehr/v1/subregions/:subregion_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/subregion/get) | - | 未找到 | - |
| 457 | 查询单条省份/行政区信息 | GET | /open-apis/corehr/v1/subdivisions/:subdivision_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/subdivision/get) | - | 未找到 | - |
| 458 | 查询当前生效信息发生变更的地点 | GET | /open-apis/corehr/v2/locations/query_recent_change | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/location/query_recent_change) | - | 未找到 | - |
| 459 | 查询当前生效信息发生变更的序列 | GET | /open-apis/corehr/v2/job_families/query_recent_change | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/job_family/query_recent_change) | - | 未找到 | - |
| 460 | 查询当前生效信息发生变更的成本中心 | GET | /open-apis/corehr/v2/cost_centers/query_recent_change | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/cost_center/query_recent_change) | - | 未找到 | - |
| 461 | 查询当前生效信息发生变更的职务 | GET | /open-apis/corehr/v2/jobs/query_recent_change | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/job/query_recent_change) | - | 未找到 | - |
| 462 | 查询当前生效信息发生变更的职等 | GET | /open-apis/corehr/v2/job_grades/query_recent_change | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/job_grade/query_recent_change) | - | 未找到 | - |
| 463 | 查询当前生效信息发生变更的职级 | GET | /open-apis/corehr/v2/job_levels/query_recent_change | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/job_level/query_recent_change) | - | 未找到 | - |
| 464 | 查询当前生效信息变更公司 | GET | /open-apis/corehr/v2/companies/query_recent_change | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/company/query_recent_change) | - | 未找到 | - |
| 465 | 查询当前生效信息变更的自定义组织 | GET | /open-apis/corehr/v2/custom_orgs/query_recent_change | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/custom_org/query_recent_change) | - | 未找到 | - |
| 466 | 查询指定时范围内当前版本信息发生变更的岗位 | GET | /open-apis/corehr/v2/positions/query_recent_change | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/position/query_recent_change) | - | 未找到 | - |
| 467 | 查询流程实例列表 | GET | /open-apis/corehr/v2/processes | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/process/list) | - | 未找到 | - |
| 468 | 查询生效信息变更部门 | GET | /open-apis/corehr/v2/departments/query_recent_change | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/department/query_recent_change) | - | 未找到 | - |
| 469 | 查询编制规划方案 | GET | /open-apis/corehr/v2/workforce_plans | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/workforce_plan/list) | - | 未找到 | - |
| 470 | 根据流程 ID 查询组织架构调整记录 | GET | /open-apis/corehr/v2/approval_groups/:process_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/approval_groups/get) | - | 未找到 | - |
| 471 | 根据组织架构调整 ID 查询发起的流程信息 | GET | /open-apis/corehr/v2/drafts/:draft_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/draft/get) | - | 未找到 | - |
| 472 | 根据适用条件获取工作日历 ID | GET | /open-apis/corehr/v1/leaves/calendar_by_scope | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/leave/calendar_by_scope) | - | 未找到 | - |
| 473 | 获取 HRBP 列表 | GET | /open-apis/corehr/v2/bps | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/bp/list) | - | 未找到 | - |
| 474 | 获取假期类型列表 | GET | /open-apis/corehr/v1/leaves/leave_types | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/leave/leave_types) | - | 未找到 | - |
| 475 | 获取单个流程详情 | GET | /open-apis/corehr/v2/processes/:process_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/process/get) | - | 未找到 | - |
| 476 | 获取员工薪资标准 | GET | /open-apis/corehr/v1/compensation_standards/match | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/compensation_standard/match) | - | 未找到 | - |
| 477 | 获取字段详情 | GET | /open-apis/corehr/v1/custom_fields/get_by_param | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/custom_field/get_by_param) | - | 未找到 | - |
| 478 | 获取异动原因列表 | GET | /open-apis/corehr/v1/transfer_reasons/query | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/transfer_reason/query) | - | 未找到 | - |
| 479 | 获取异动类型列表 | GET | /open-apis/corehr/v1/transfer_types/query | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/transfer_type/query) | - | 未找到 | - |
| 480 | 获取指定人员审批任务列表 | GET | /open-apis/corehr/v2/approvers | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/approver/list) | - | 未找到 | - |
| 481 | 获取流程数据 | GET | /open-apis/corehr/v2/processes/:process_id/flow_variable_data | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/process/flow_variable_data) | - | 未找到 | - |
| 482 | 获取流程表单数据 | GET | /open-apis/corehr/v2/processes/:process_id/form_variable_data | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/process-form_variable_data/get) | - | 未找到 | - |
| 483 | 获取流程表单数据 | GET | /open-apis/corehr/v1/processes/:process_id/form_variable_data | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/process-form_variable_data/get) | - | 未找到 | - |
| 484 | 获取自定义字段列表 | GET | /open-apis/corehr/v1/custom_fields/query | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/custom_field/query) | - | 未找到 | - |
| 485 | 获取飞书人事对象列表 | GET | /open-apis/corehr/v1/custom_fields/list_object_api_name | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/custom_field/list_object_api_name) | - | 未找到 | - |
| 486 | ID 转换 | POST | /open-apis/corehr/v1/common_data/id/convert | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/common_data-id/convert) | - | 未找到 | - |
| 487 | 上传文件 | POST | /open-apis/corehr/v1/persons/upload | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/person/upload) | - | 未找到 | - |
| 488 | 为用户授权角色 | POST | /open-apis/corehr/v1/authorizations/add_role_assign | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/authorization/add_role_assign) | - | 未找到 | - |
| 489 | 修改字段枚举值选项 | POST | /open-apis/corehr/v1/common_data/meta_data/edit_enum_option | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/common_data-meta_data/edit_enum_option) | - | 未找到 | - |
| 490 | 创建个人信息 | POST | /open-apis/corehr/v1/persons | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/person/create) | - | 未找到 | - |
| 491 | 创建个人信息 | POST | /open-apis/corehr/v2/persons | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/person/create) | - | 未找到 | - |
| 492 | 创建人员类型 | POST | /open-apis/corehr/v1/employee_types | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/employee_type/create) | - | 未找到 | - |
| 493 | 创建任职信息 | POST | /open-apis/corehr/v1/job_datas | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/job_data/create) | - | 未找到 | - |
| 494 | 创建假期发放记录 | POST | /open-apis/corehr/v1/leave_granting_records | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/leave_granting_record/create) | - | 未找到 | - |
| 495 | 创建公司 | POST | /open-apis/corehr/v1/companies | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/company/create) | - | 未找到 | - |
| 496 | 创建兼职 | POST | /open-apis/corehr/v2/employees/additional_jobs | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/employees-additional_job/create) | - | 未找到 | - |
| 497 | 创建国家证件类型 | POST | /open-apis/corehr/v1/national_id_types | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/national_id_type/create) | - | 未找到 | - |
| 498 | 创建地点 | POST | /open-apis/corehr/v1/locations | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/location/create) | - | 未找到 | - |
| 499 | 创建外派信息 | POST | /open-apis/corehr/v2/employees/international_assignments | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/employees-international_assignment/create) | - | 未找到 | - |
| 500 | 创建岗位信息 | POST | /open-apis/corehr/v2/positions | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/position/create) | - | 未找到 | - |
| 501 | 创建工时制度 | POST | /open-apis/corehr/v1/working_hours_types | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/working_hours_type/create) | - | 未找到 | - |
| 502 | 创建序列 | POST | /open-apis/corehr/v1/job_families | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/job_family/create) | - | 未找到 | - |
| 503 | 创建成本中心 | POST | /open-apis/corehr/v2/cost_centers | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/cost_center/create) | - | 未找到 | - |
| 504 | 创建成本中心版本 | POST | /open-apis/corehr/v2/cost_centers/:cost_center_id/versions | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/cost_center-version/create) | - | 未找到 | - |
| 505 | 创建成本分摊 | POST | /open-apis/corehr/v2/cost_allocations/create_version | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/cost_allocation/create_version) | - | 未找到 | - |
| 506 | 创建职务 | POST | /open-apis/corehr/v1/jobs | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/job/create) | - | 未找到 | - |
| 507 | 创建职等 | POST | /open-apis/corehr/v2/job_grades | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/job_grade/create) | - | 未找到 | - |
| 508 | 创建自定义组织 | POST | /open-apis/corehr/v2/custom_orgs | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/custom_org/create) | - | 未找到 | - |
| 509 | 创建通道 | POST | /open-apis/corehr/v2/pathways | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/pathway/create) | - | 未找到 | - |
| 510 | 创建部门 | POST | /open-apis/corehr/v1/departments | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/department/create) | - | 未找到 | - |
| 511 | 创建雇佣信息 | POST | /open-apis/corehr/v1/employments | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/employment/create) | - | 未找到 | - |
| 512 | 删除岗位 | POST | /open-apis/corehr/v2/positions/del_position | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/position/del_position) | - | 未找到 | - |
| 513 | 删除成本分摊 | POST | /open-apis/corehr/v2/cost_allocations/remove_version | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/cost_allocation/remove_version) | - | 未找到 | - |
| 514 | 删除自定义组织 | POST | /open-apis/corehr/v2/custom_orgs/delete_org | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/custom_org/delete_org) | - | 未找到 | - |
| 515 | 删除默认成本中心 | POST | /open-apis/corehr/v2/default_cost_centers/remove_version | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/default_cost_center/remove_version) | - | 未找到 | - |
| 516 | 发起员工异动 | POST | /open-apis/corehr/v2/job_changes | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/job_change/create) | - | 未找到 | - |
| 517 | 发起员工异动(不推荐) | POST | /open-apis/corehr/v1/job_changes | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/job_change/create) | - | 未找到 | - |
| 518 | 发起转正 | POST | /open-apis/corehr/v2/probation/submit | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/probation/submit) | - | 未找到 | - |
| 519 | 启停用岗位 | POST | /open-apis/corehr/v2/positions/active | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/position/active) | - | 未找到 | - |
| 520 | 启停用通道 | POST | /open-apis/corehr/v2/pathways/active | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/pathway/active) | - | 未找到 | - |
| 521 | 启用/停用公司 | POST | /open-apis/corehr/v2/companies/active | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/company/active) | - | 未找到 | - |
| 522 | 启用/停用地点 | POST | /open-apis/corehr/v2/locations/active | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/location/active) | - | 未找到 | - |
| 523 | 启用/停用自定义组织 | POST | /open-apis/corehr/v2/custom_orgs/active | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/custom_org/active) | - | 未找到 | - |
| 524 | 启用/停用试用期考核功能 | POST | /open-apis/corehr/v2/probation/enable_disable_assessment | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/probation/enable_disable_assessment) | - | 未找到 | - |
| 525 | 增加字段枚举值选项 | POST | /open-apis/corehr/v1/common_data/meta_data/add_enum_option | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/common_data-meta_data/add_enum_option) | - | 未找到 | - |
| 526 | 恢复入职 | POST | /open-apis/corehr/v2/pre_hires/restore_flow_instance | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/pre_hire/restore_flow_instance) | - | 未找到 | - |
| 527 | 批量创建/更新填报行 | POST | /open-apis/corehr/v2/report_detail_row/batchSave | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/report_detail_row/batchSave) | - | 未找到 | - |
| 528 | 批量创建/更新明细行 | POST | /open-apis/corehr/v2/workforce_plan_detail_row/batchSave | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/workforce_plan_detail_row/batchSave) | - | 未找到 | - |
| 529 | 批量删除填报行 | POST | /open-apis/corehr/v2/report_detail_row/batchDelete | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/report_detail_row/batchDelete) | - | 未找到 | - |
| 530 | 批量删除明细行 | POST | /open-apis/corehr/v2/workforce_plan_detail_row/batchDelete | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/workforce_plan_detail_row/batchDelete) | - | 未找到 | - |
| 531 | 批量查询人员调整内容 | POST | /open-apis/corehr/v2/approval_groups/open_query_job_change_list_by_ids | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/approval_groups/open_query_job_change_list_by_ids) | - | 未找到 | - |
| 532 | 批量查询兼职信息 | POST | /open-apis/corehr/v2/employees/additional_jobs/batch | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/employees-additional_job/batch) | - | 未找到 | - |
| 533 | 批量查询员工任职信息 | POST | /open-apis/corehr/v2/employees/job_datas/batch_get | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/employees-job_data/batch_get) | - | 未找到 | - |
| 534 | 批量查询员工信息 | POST | /open-apis/corehr/v2/employees/batch_get | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/employee/batch_get) | - | 未找到 | - |
| 535 | 批量查询岗位调整内容 | POST | /open-apis/corehr/v2/approval_groups/open_query_position_change_list_by_ids | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/approval_groups/open_query_position_change_list_by_ids) | - | 未找到 | - |
| 536 | 批量查询部门 | POST | /open-apis/corehr/v2/departments/batch_get | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/department/batch_get) | - | 未找到 | - |
| 537 | 批量查询部门操作日志 | POST | /open-apis/corehr/departments/query_operation_logs | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/department/query_operation_logs) | - | 未找到 | - |
| 538 | 批量查询部门版本信息 | POST | /open-apis/corehr/v2/departments/query_multi_timeline | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/department/query_multi_timeline) | - | 未找到 | - |
| 539 | 批量查询部门调整内容 | POST | /open-apis/corehr/v2/approval_groups/open_query_department_change_list_by_ids | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/approval_groups/open_query_department_change_list_by_ids) | - | 未找到 | - |
| 540 | 搜索合同 | POST | /open-apis/corehr/v2/contracts/search | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/contract/search) | - | 未找到 | - |
| 541 | 搜索员工信息 | POST | /open-apis/corehr/v2/employees/search | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/employee/search) | - | 未找到 | - |
| 542 | 搜索员工异动信息 | POST | /open-apis/corehr/v2/job_changes/search | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/job_change/search) | - | 未找到 | - |
| 543 | 搜索待入职信息 | POST | /open-apis/corehr/v2/pre_hires/search | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/pre_hire/search) | - | 未找到 | - |
| 544 | 搜索成本中心信息 | POST | /open-apis/corehr/v2/cost_centers/search | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/cost_center/search) | - | 未找到 | - |
| 545 | 搜索离职信息 | POST | /open-apis/corehr/v1/offboardings/search | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/offboarding/search) | - | 未找到 | - |
| 546 | 搜索试用期信息 | POST | /open-apis/corehr/v2/probation/search | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/probation/search) | - | 未找到 | - |
| 547 | 搜索部门信息 | POST | /open-apis/corehr/v2/departments/search | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/department/search) | - | 未找到 | - |
| 548 | 撤销入职 | POST | /open-apis/corehr/v2/pre_hires/withdraw_onboarding | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/pre_hire/withdraw_onboarding) | - | 未找到 | - |
| 549 | 撤销异动 | POST | /open-apis/corehr/v2/job_changes/:job_change_id/revoke | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/job_change/revoke) | - | 未找到 | - |
| 550 | 撤销离职 | POST | /open-apis/corehr/v2/offboardings/revoke | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/offboarding/revoke) | - | 未找到 | - |
| 551 | 撤销转正 | POST | /open-apis/corehr/v2/probation/withdraw | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/probation/withdraw) | - | 未找到 | - |
| 552 | 操作员工完成入职 | POST | /open-apis/corehr/v2/pre_hires/:pre_hire_id/complete | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/pre_hire/complete) | - | 未找到 | - |
| 553 | 操作员工离职 | POST | /open-apis/corehr/v2/offboardings/submit_v2 | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/offboarding/submit_v2) | - | 未找到 | - |
| 554 | 操作员工离职 | POST | /open-apis/corehr/v1/offboardings/submit | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/offboarding/submit) | - | 未找到 | - |
| 555 | 新增试用期考核信息 | POST | /open-apis/corehr/v2/probation/assessments | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/probation-assessment/create) | - | 未找到 | - |
| 556 | 新建合同 | POST | /open-apis/corehr/v1/contracts | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/contract/create) | - | 未找到 | - |
| 557 | 新建职级 | POST | /open-apis/corehr/v1/job_levels | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/job_level/create) | - | 未找到 | - |
| 558 | 更新成本分摊 | POST | /open-apis/corehr/v2/cost_allocations/update_version | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/cost_allocation/update_version) | - | 未找到 | - |
| 559 | 更新用户被授权的数据范围 | POST | /open-apis/corehr/v1/authorizations/update_role_assign | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/authorization/update_role_assign) | - | 未找到 | - |
| 560 | 更新自定义组织的匹配规则 | POST | /open-apis/corehr/v2/custom_orgs/update_rule | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/custom_org/update_rule) | - | 未找到 | - |
| 561 | 更新默认成本中心 | POST | /open-apis/corehr/v2/default_cost_centers/update_version | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/default_cost_center/update_version) | - | 未找到 | - |
| 562 | 查询区/县信息 | POST | /open-apis/corehr/v2/basic_info/districts/search | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/basic_info-district/search) | - | 未找到 | - |
| 563 | 查询员工 HRBP / 属地 BP | POST | /open-apis/corehr/v2/employees/bps/batch_get | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/employees-bp/batch_get) | - | 未找到 | - |
| 564 | 查询员工离职原因列表 | POST | /open-apis/corehr/v1/offboardings/query | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/offboarding/query) | - | 未找到 | - |
| 565 | 查询国家/地区信息 | POST | /open-apis/corehr/v2/basic_info/country_regions/search | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/basic_info-country_region/search) | - | 未找到 | - |
| 566 | 查询国籍信息 | POST | /open-apis/corehr/v2/basic_info/nationalities/search | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/basic_info-nationality/search) | - | 未找到 | - |
| 567 | 查询城市信息 | POST | /open-apis/corehr/v2/basic_info/cities/search | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/basic_info-city/search) | - | 未找到 | - |
| 568 | 查询岗位信息 | POST | /open-apis/corehr/v2/positions/query | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/position/query) | - | 未找到 | - |
| 569 | 查询待入职信息 | POST | /open-apis/corehr/v2/pre_hires/query | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/pre_hire/query) | - | 未找到 | - |
| 570 | 查询成本分摊 | POST | /open-apis/corehr/v2/cost_allocations/batch_query | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/cost_allocation/batch_query) | - | 未找到 | - |
| 571 | 查询指定时间范围序列版本 | POST | /open-apis/corehr/v2/job_families/query_multi_timeline | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/job_family/query_multi_timeline) | - | 未找到 | - |
| 572 | 查询指定时间范围职务版本 | POST | /open-apis/corehr/v2/jobs/query_multi_timeline | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/job/query_multi_timeline) | - | 未找到 | - |
| 573 | 查询指定生效日期的部门基本信息 | POST | /open-apis/corehr/v2/departments/query_timeline | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/department/query_timeline) | - | 未找到 | - |
| 574 | 查询指定生效日期的部门架构树 | POST | /open-apis/corehr/v2/departments/tree | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/department/tree) | - | 未找到 | - |
| 575 | 查询支行信息 | POST | /open-apis/corehr/v2/basic_info/bank_branchs/search | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/basic_info-bank_branch/search) | - | 未找到 | - |
| 576 | 查询时区信息 | POST | /open-apis/corehr/v2/basic_info/time_zones/search | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/basic_info-time_zone/search) | - | 未找到 | - |
| 577 | 查询枚举信息 | POST | /open-apis/corehr/v2/enums/search | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/enum/search) | - | 未找到 | - |
| 578 | 查询省份/主要行政区信息 | POST | /open-apis/corehr/v2/basic_info/country_region_subdivisions/search | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/basic_info-country_region_subdivision/search) | - | 未找到 | - |
| 579 | 查询编制规划明细信息（不支持自定义组织） | POST | /open-apis/corehr/v2/workforce_plan_details/batch | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/workforce_plan_detail/batch) | - | 未找到 | - |
| 580 | 查询编制规划明细信息（支持自定义组织） | POST | /open-apis/corehr/v2/workforce_plan_details/batch_v2 | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/workforce_plan_detail/batch_v2) | - | 未找到 | - |
| 581 | 查询职等 | POST | /open-apis/corehr/v2/job_grades/query | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/job_grade/query) | - | 未找到 | - |
| 582 | 查询自定义组织信息 | POST | /open-apis/corehr/v2/custom_orgs/query | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/custom_org/query) | - | 未找到 | - |
| 583 | 查询语言信息 | POST | /open-apis/corehr/v2/basic_info/languages/search | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/basic_info-language/search) | - | 未找到 | - |
| 584 | 查询货币信息 | POST | /open-apis/corehr/v2/basic_info/currencies/search | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/basic_info-currency/search) | - | 未找到 | - |
| 585 | 查询部门 / 地点的 HRBP / 属地 BP | POST | /open-apis/corehr/v1/security_groups/query | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/security_group/query) | - | 未找到 | - |
| 586 | 查询部门 HRBP | POST | /open-apis/corehr/v2/bps/get_by_department | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/bp/get_by_department) | - | 未找到 | - |
| 587 | 查询银行信息 | POST | /open-apis/corehr/v2/basic_info/banks/search | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/basic_info-bank/search) | - | 未找到 | - |
| 588 | 查询默认成本中心 | POST | /open-apis/corehr/v2/default_cost_centers/batch_query | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/default_cost_center/batch_query) | - | 未找到 | - |
| 589 | 根据条件批量获取序列信息 | POST | /open-apis/corehr/v2/job_families/batch_get | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/job_family/batch_get) | - | 未找到 | - |
| 590 | 根据条件批量获取职务 | POST | /open-apis/corehr/v2/jobs/batch_get | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/job/batch_get) | - | 未找到 | - |
| 591 | 根据条件批量获取职级信息 | POST | /open-apis/corehr/v2/job_levels/batch_get | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/job_level/batch_get) | - | 未找到 | - |
| 592 | 流转入职任务 | POST | /open-apis/corehr/v2/pre_hires/transform_onboarding_task | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/pre_hire/transform_onboarding_task) | - | 未找到 | - |
| 593 | 流转入职任务 | POST | /open-apis/corehr/v2/pre_hires/:pre_hire_id/transit_task | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/pre_hire/transit_task) | - | 未找到 | - |
| 594 | 添加人员 | POST | /open-apis/corehr/v2/employees | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/employee/create) | - | 未找到 | - |
| 595 | 添加地点地址 | POST | /open-apis/corehr/v2/locations/:location_id/addresses | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/location-address/create) | - | 未找到 | - |
| 596 | 添加默认成本中心 | POST | /open-apis/corehr/v2/default_cost_centers/create_version | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/default_cost_center/create_version) | - | 未找到 | - |
| 597 | 直接创建待入职 | POST | /open-apis/corehr/v2/pre_hires | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/pre_hire/create) | - | 未找到 | - |
| 598 | 移除用户被授权的角色 | POST | /open-apis/corehr/v1/authorizations/remove_role_assign | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/authorization/remove_role_assign) | - | 未找到 | - |
| 599 | 编辑离职信息 | POST | /open-apis/corehr/v2/offboardings/edit | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/offboarding/edit) | - | 未找到 | - |
| 600 | 获取任职信息列表 | POST | /open-apis/corehr/v2/employees/job_datas/query | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/employees-job_data/query) | - | 未找到 | - |
| 601 | 获取工作日历 | POST | /open-apis/corehr/v1/leaves/work_calendar | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/leave/work_calendar) | - | 未找到 | - |
| 602 | 获取工作日历日期详情 | POST | /open-apis/corehr/v1/leaves/work_calendar_date | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/leave/work_calendar_date) | - | 未找到 | - |
| 603 | 获取父部门信息 | POST | /open-apis/corehr/v2/departments/parents | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/department/parents) | - | 未找到 | - |
| 604 | 获取组织类角色授权列表 | POST | /open-apis/corehr/v1/assigned_users/search | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/assigned_user/search) | - | 未找到 | - |
| 605 | 获取通道信息 | POST | /open-apis/corehr/v2/pathways/batch_get | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/pathway/batch_get) | - | 未找到 | - |
| 606 | 通过公司 ID 批量获取公司信息 | POST | /open-apis/corehr/v2/companies/batch_get | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/company/batch_get) | - | 未找到 | - |
| 607 | 通过地点 ID 批量获取地点信息 | POST | /open-apis/corehr/v2/locations/batch_get | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/location/batch_get) | - | 未找到 | - |
| 608 | 加签审批任务 | PUT | /open-apis/corehr/v2/processes/:process_id/extra | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/process-extra/update) | - | 未找到 | - |
| 609 | 撤回流程 | PUT | /open-apis/corehr/v2/process_withdraw/:process_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/process_withdraw/update) | - | 未找到 | - |
| 610 | 撤销流程 | PUT | /open-apis/corehr/v2/process_revoke/:process_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/process_revoke/update) | - | 未找到 | - |
| 611 | 转交审批任务 | PUT | /open-apis/corehr/v2/processes/:process_id/transfer | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/process-transfer/update) | - | 未找到 | - |
| 612 | 通过/拒绝审批任务 | PUT | /open-apis/corehr/v2/processes/:process_id/approvers/:approver_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/process-approver/update) | - | 未找到 | - |
| 613 | 启用 / 停用成本中心 | PATCH | /open-apis/corehr/v2/cost_centers/:cost_center_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/cost_center/patch) | - | 未找到 | - |
| 614 | 更新个人信息 | PATCH | /open-apis/corehr/v2/persons/:person_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/person/patch) | - | 未找到 | - |
| 615 | 更新个人信息 | PATCH | /open-apis/corehr/v1/persons/:person_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/person/patch) | - | 未找到 | - |
| 616 | 更新人员类型 | PATCH | /open-apis/corehr/v1/employee_types/:employee_type_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/employee_type/patch) | - | 未找到 | - |
| 617 | 更新任职信息 | PATCH | /open-apis/corehr/v1/job_datas/:job_data_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/job_data/patch) | - | 未找到 | - |
| 618 | 更新公司 | PATCH | /open-apis/corehr/v1/companies/:company_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/company/patch) | - | 未找到 | - |
| 619 | 更新兼职 | PATCH | /open-apis/corehr/v2/employees/additional_jobs/:additional_job_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/employees-additional_job/patch) | - | 未找到 | - |
| 620 | 更新单个职级 | PATCH | /open-apis/corehr/v1/job_levels/:job_level_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/job_level/patch) | - | 未找到 | - |
| 621 | 更新合同 | PATCH | /open-apis/corehr/v1/contracts/:contract_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/contract/patch) | - | 未找到 | - |
| 622 | 更新国家证件类型 | PATCH | /open-apis/corehr/v1/national_id_types/:national_id_type_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/national_id_type/patch) | - | 未找到 | - |
| 623 | 更新地点 | PATCH | /open-apis/corehr/v2/locations/:location_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/location/patch) | - | 未找到 | - |
| 624 | 更新地点地址 | PATCH | /open-apis/corehr/v2/locations/:location_id/addresses/:address_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/location-address/patch) | - | 未找到 | - |
| 625 | 更新外派信息 | PATCH | /open-apis/corehr/v2/employees/international_assignments/:international_assignment_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/employees-international_assignment/patch) | - | 未找到 | - |
| 626 | 更新岗位信息 | PATCH | /open-apis/corehr/v2/positions/:position_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/position/patch) | - | 未找到 | - |
| 627 | 更新工时制度 | PATCH | /open-apis/corehr/v1/working_hours_types/:working_hours_type_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/working_hours_type/patch) | - | 未找到 | - |
| 628 | 更新序列 | PATCH | /open-apis/corehr/v1/job_families/:job_family_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/job_family/patch) | - | 未找到 | - |
| 629 | 更新待入职信息 | PATCH | /open-apis/corehr/v2/pre_hires/:pre_hire_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/pre_hire/patch) | - | 未找到 | - |
| 630 | 更新待入职信息（不推荐） | PATCH | /open-apis/corehr/v1/pre_hires/:pre_hire_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/pre_hire/patch) | - | 未找到 | - |
| 631 | 更新职务 | PATCH | /open-apis/corehr/v1/jobs/:job_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/job/patch) | - | 未找到 | - |
| 632 | 更新职等 | PATCH | /open-apis/corehr/v2/job_grades/:job_grade_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/job_grade/patch) | - | 未找到 | - |
| 633 | 更新自定义组织信息 | PATCH | /open-apis/corehr/v2/custom_orgs/:org_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/custom_org/patch) | - | 未找到 | - |
| 634 | 更新试用期考核信息 | PATCH | /open-apis/corehr/v2/probation/assessments/:assessment_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/probation-assessment/patch) | - | 未找到 | - |
| 635 | 更新通道 | PATCH | /open-apis/corehr/v2/pathways/:pathway_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/pathway/patch) | - | 未找到 | - |
| 636 | 更新部门 | PATCH | /open-apis/corehr/v2/departments/:department_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/department/patch) | - | 未找到 | - |
| 637 | 更新部门 | PATCH | /open-apis/corehr/v1/departments/:department_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/department/patch) | - | 未找到 | - |
| 638 | 更新雇佣信息 | PATCH | /open-apis/corehr/v1/employments/:employment_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/employment/patch) | - | 未找到 | - |
| 639 | 更正成本中心版本 | PATCH | /open-apis/corehr/v2/cost_centers/:cost_center_id/versions/:version_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/cost_center-version/patch) | - | 未找到 | - |
| 640 | 删除个人信息 | DELETE | /open-apis/corehr/v1/persons/:person_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/person/delete) | - | 未找到 | - |
| 641 | 删除人员类型 | DELETE | /open-apis/corehr/v1/employee_types/:employee_type_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/employee_type/delete) | - | 未找到 | - |
| 642 | 删除任职信息 | DELETE | /open-apis/corehr/v1/job_datas/:job_data_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/job_data/delete) | - | 未找到 | - |
| 643 | 删除假期发放记录 | DELETE | /open-apis/corehr/v1/leave_granting_records/:leave_granting_record_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/leave_granting_record/delete) | - | 未找到 | - |
| 644 | 删除公司 | DELETE | /open-apis/corehr/v1/companies/:company_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/company/delete) | - | 未找到 | - |
| 645 | 删除兼职 | DELETE | /open-apis/corehr/v2/employees/additional_jobs/:additional_job_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/employees-additional_job/delete) | - | 未找到 | - |
| 646 | 删除合同 | DELETE | /open-apis/corehr/v1/contracts/:contract_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/contract/delete) | - | 未找到 | - |
| 647 | 删除国家证件类型 | DELETE | /open-apis/corehr/v1/national_id_types/:national_id_type_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/national_id_type/delete) | - | 未找到 | - |
| 648 | 删除地点 | DELETE | /open-apis/corehr/v1/locations/:location_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/location/delete) | - | 未找到 | - |
| 649 | 删除地点地址 | DELETE | /open-apis/corehr/v2/locations/:location_id/addresses/:address_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/location-address/delete) | - | 未找到 | - |
| 650 | 删除外派信息 | DELETE | /open-apis/corehr/v2/employees/international_assignments/:international_assignment_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/employees-international_assignment/delete) | - | 未找到 | - |
| 651 | 删除工时制度 | DELETE | /open-apis/corehr/v1/working_hours_types/:working_hours_type_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/working_hours_type/delete) | - | 未找到 | - |
| 652 | 删除序列 | DELETE | /open-apis/corehr/v1/job_families/:job_family_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/job_family/delete) | - | 未找到 | - |
| 653 | 删除待入职信息 | DELETE | /open-apis/corehr/v2/pre_hires/:pre_hire_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/pre_hire/delete) | - | 未找到 | - |
| 654 | 删除待入职（不推荐） | DELETE | /open-apis/corehr/v1/pre_hires/:pre_hire_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/pre_hire/delete) | - | 未找到 | - |
| 655 | 删除成本中心 | DELETE | /open-apis/corehr/v2/cost_centers/:cost_center_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/cost_center/delete) | - | 未找到 | - |
| 656 | 删除职务 | DELETE | /open-apis/corehr/v1/jobs/:job_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/job/delete) | - | 未找到 | - |
| 657 | 删除职等 | DELETE | /open-apis/corehr/v2/job_grades/:job_grade_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/job_grade/delete) | - | 未找到 | - |
| 658 | 删除职级 | DELETE | /open-apis/corehr/v1/job_levels/:job_level_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/job_level/delete) | - | 未找到 | - |
| 659 | 删除试用期考核信息 | DELETE | /open-apis/corehr/v2/probation/assessments/:assessment_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/probation-assessment/delete) | - | 未找到 | - |
| 660 | 删除通道 | DELETE | /open-apis/corehr/v2/pathways/:pathway_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/pathway/delete) | - | 未找到 | - |
| 661 | 删除部门 | DELETE | /open-apis/corehr/v1/departments/:department_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/department/delete) | - | 未找到 | - |
| 662 | 删除部门 V2 | DELETE | /open-apis/corehr/v2/departments/:department_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/department/delete) | - | 未找到 | - |
| 663 | 删除雇佣信息 | DELETE | /open-apis/corehr/v1/employments/:employment_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/employment/delete) | - | 未找到 | - |
| 664 | 撤销成本中心版本 | DELETE | /open-apis/corehr/v2/cost_centers/:cost_center_id/versions/:version_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/cost_center-version/delete) | - | 未找到 | - |

### 📦 directory 模块

| 665 | 查询可搜可见规则 | GET | /open-apis/directory/v1/collaboration_rules | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/directory-v1/collaboration_rule/list) | - | 未找到 | - |
| 666 | 管理员获取所有关联组织列表 | GET | /open-apis/directory/v1/collaboration_tenants | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/directory-v1/collaboration_tenant/list) | - | 未找到 | - |
| 667 | 获取关联组织双方共享成员范围 | GET | /open-apis/directory/v1/share_entities | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/directory-v1/collboration_share_entity/list) | - | 未找到 | - |
| 668 | 创建员工 | POST | /open-apis/directory/v1/employees | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/directory-v1/employee/create) | - | 未找到 | - |
| 669 | 创建部门 | POST | /open-apis/directory/v1/departments | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/directory-v1/department/create) | - | 未找到 | - |
| 670 | 恢复离职员工 | POST | /open-apis/directory/v1/employees/:employee_id/resurrect | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/directory-v1/employee/resurrect) | - | 未找到 | - |
| 671 | 批量获取员工信息 | POST | /open-apis/directory/v1/employees/mget | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/directory-v1/employee/mget) | - | 未找到 | - |
| 672 | 批量获取员工列表 | POST | /open-apis/directory/v1/employees/filter | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/directory-v1/employee/filter) | - | 未找到 | - |
| 673 | 批量获取部门信息 | POST | /open-apis/directory/v1/departments/mget | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/directory-v1/department/mget) | - | 未找到 | - |
| 674 | 搜索员工信息 | POST | /open-apis/directory/v1/employees/search | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/directory-v1/employee/search) | - | 未找到 | - |
| 675 | 搜索部门 | POST | /open-apis/directory/v1/departments/search | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/directory-v1/department/search) | - | 未找到 | - |
| 676 | 新增可搜可见规则 | POST | /open-apis/directory/v1/collaboration_rules | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/directory-v1/collaboration_rule/create) | - | 未找到 | - |
| 677 | 获取部门列表 | POST | /open-apis/directory/v1/departments/filter | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/directory-v1/department/filter) | - | 未找到 | - |
| 678 | 更新可搜可见规则 | PUT | /open-apis/directory/v1/collaboration_rules/:collaboration_rule_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/directory-v1/collaboration_rule/update) | - | 未找到 | - |
| 679 | 更新员工信息 | PATCH | /open-apis/directory/v1/employees/:employee_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/directory-v1/employee/patch) | - | 未找到 | - |
| 680 | 更新在职员工为待离职 | PATCH | /open-apis/directory/v1/employees/:employee_id/to_be_resigned | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/directory-v1/employee/to_be_resigned) | - | 未找到 | - |
| 681 | 更新待离职成员为在职 | PATCH | /open-apis/directory/v1/employees/:employee_id/regular | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/directory-v1/employee/regular) | - | 未找到 | - |
| 682 | 更新部门 | PATCH | /open-apis/directory/v1/departments/:department_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/directory-v1/department/patch) | - | 未找到 | - |
| 683 | 删除可搜可见规则 | DELETE | /open-apis/directory/v1/collaboration_rules/:collaboration_rule_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/directory-v1/collaboration_rule/delete) | - | 未找到 | - |
| 684 | 删除部门 | DELETE | /open-apis/directory/v1/departments/:department_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/directory-v1/department/delete) | - | 未找到 | - |
| 685 | 离职员工 | DELETE | /open-apis/directory/v1/employees/:employee_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/directory-v1/employee/delete) | - | 未找到 | - |

### 📦 doc 模块

| 686 | 获取旧版文档中的电子表格元数据 | GET | /open-apis/doc/v2/:docToken/sheet_meta | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uADOzUjLwgzM14CM4MTN) | - | 未找到 | - |
| 687 | 获取旧版文档元信息 | GET | /open-apis/doc/v2/meta/:docToken | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uczN3UjL3czN14yN3cTN) | - | 未找到 | - |
| 688 | 获取旧版文档富文本内容 | GET | /open-apis/doc/v2/:docToken/content | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uUDM2YjL1AjN24SNwYjN) | - | 未找到 | - |
| 689 | 获取旧版文档纯文本内容 | GET | /open-apis/doc/v2/:docToken/raw_content | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/ukzNzUjL5czM14SO3MTN) | - | 未找到 | - |
| 690 | 创建旧版文档 | POST | /open-apis/doc/v2/create | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/ugDM2YjL4AjN24COwYjN) | - | 未找到 | - |
| 691 | 编辑旧版文档内容 | POST | /open-apis/doc/v2/:docToken/batch_update | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uYDM2YjL2AjN24iNwYjN) | - | 未找到 | - |

### 📦 docs 模块

| 692 | 获取云文档内容 | GET | /open-apis/docs/v1/content | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/docs-v1/content/get) | - | 未找到 | - |

### 📦 document_ai 模块

| 693 | 提取文件中的合同字段 | POST | /open-apis/document_ai/v1/contract/field_extraction | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/ai/document_ai-v1/contract/field_extraction) | - | 未找到 | - |
| 694 | 识别文件中的中国护照 | POST | /open-apis/document_ai/v1/chinese_passport/recognize | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/ai/document_ai-v1/chinese_passport/recognize) | - | 未找到 | - |
| 695 | 识别文件中的健康证 | POST | /open-apis/document_ai/v1/health_certificate/recognize | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/ai/document_ai-v1/health_certificate/recognize) | - | 未找到 | - |
| 696 | 识别文件中的出租车发票 | POST | /open-apis/document_ai/v1/taxi_invoice/recognize | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/ai/document_ai-v1/taxi_invoice/recognize) | - | 未找到 | - |
| 697 | 识别文件中的台湾居民来往大陆通行证 | POST | /open-apis/document_ai/v1/tw_mainland_travel_permit/recognize | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/ai/document_ai-v1/tw_mainland_travel_permit/recognize) | - | 未找到 | - |
| 698 | 识别文件中的名片 | POST | /open-apis/document_ai/v1/business_card/recognize | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/ai/document_ai-v1/business_card/recognize) | - | 未找到 | - |
| 699 | 识别文件中的增值税发票 | POST | /open-apis/document_ai/v1/vat_invoice/recognize | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/ai/document_ai-v1/vat_invoice/recognize) | - | 未找到 | - |
| 700 | 识别文件中的机动车发票 | POST | /open-apis/document_ai/v1/vehicle_invoice/recognize | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/ai/document_ai-v1/vehicle_invoice/recognize) | - | 未找到 | - |
| 701 | 识别文件中的港澳居民来往内地通行证 | POST | /open-apis/document_ai/v1/hkm_mainland_travel_permit/recognize | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/ai/document_ai-v1/hkm_mainland_travel_permit/recognize) | - | 未找到 | - |
| 702 | 识别文件中的火车票 | POST | /open-apis/document_ai/v1/train_invoice/recognize | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/ai/document_ai-v1/train_invoice/recognize) | - | 未找到 | - |
| 703 | 识别文件中的简历信息 | POST | /open-apis/document_ai/v1/resume/parse | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/ai/document_ai-v1/resume/parse) | - | 未找到 | - |
| 704 | 识别文件中的营业执照 | POST | /open-apis/document_ai/v1/business_license/recognize | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/ai/document_ai-v1/business_license/recognize) | - | 未找到 | - |
| 705 | 识别文件中的行驶证 | POST | /open-apis/document_ai/v1/vehicle_license/recognize | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/ai/document_ai-v1/vehicle_license/recognize) | - | 未找到 | - |
| 706 | 识别文件中的身份证 | POST | /open-apis/document_ai/v1/id_card/recognize | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/ai/document_ai-v1/id_card/recognize) | - | 未找到 | - |
| 707 | 识别文件中的银行卡 | POST | /open-apis/document_ai/v1/bank_card/recognize | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/ai/document_ai-v1/bank_card/recognize) | - | 未找到 | - |
| 708 | 识别文件中的食品生产许可证 | POST | /open-apis/document_ai/v1/food_produce_license/recognize | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/ai/document_ai-v1/food_produce_license/recognize) | - | 未找到 | - |
| 709 | 识别文件中的食品经营许可证 | POST | /open-apis/document_ai/v1/food_manage_license/recognize | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/ai/document_ai-v1/food_manage_license/recognize) | - | 未找到 | - |
| 710 | 识别文件中的驾驶证 | POST | /open-apis/document_ai/v1/driving_license/recognize | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/ai/document_ai-v1/driving_license/recognize) | - | 未找到 | - |

### 📦 docx 模块

| 711 | 获取块的内容 | GET | /open-apis/docx/v1/documents/:document_id/blocks/:block_id | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document-block/get) | - | 未找到 | - |
| 712 | 获取所有子块 | GET | /open-apis/docx/v1/documents/:document_id/blocks/:block_id/children | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document-block-children/get) | - | 未找到 | - |
| 713 | 获取所有子块 | GET | /open-apis/docx/v1/chats/:chat_id/announcement/blocks/:block_id/children | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/chat-announcement-block-children/get) | - | 未找到 | - |
| 714 | 获取文档基本信息 | GET | /open-apis/docx/v1/documents/:document_id | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document/get) | - | 未找到 | - |
| 715 | 获取文档所有块 | GET | /open-apis/docx/v1/documents/:document_id/blocks | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document-block/list) | - | 未找到 | - |
| 716 | 获取文档纯文本内容 | GET | /open-apis/docx/v1/documents/:document_id/raw_content | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document/raw_content) | - | 未找到 | - |
| 717 | 获取群公告块的内容 | GET | /open-apis/docx/v1/chats/:chat_id/announcement/blocks/:block_id | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/chat-announcement-block/get) | - | 未找到 | - |
| 718 | 获取群公告基本信息 | GET | /open-apis/docx/v1/chats/:chat_id/announcement | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/chat-announcement/get) | - | 未找到 | - |
| 719 | 获取群公告所有块 | GET | /open-apis/docx/v1/chats/:chat_id/announcement/blocks | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/chat-announcement-block/list) | - | 未找到 | - |
| 720 | Markdown/HTML 内容转换为文档块 | POST | /open-apis/docx/documents/blocks/convert | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document/convert) | - | 未找到 | - |
| 721 | 创建块 | POST | /open-apis/docx/v1/documents/:document_id/blocks/:block_id/children | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document-block-children/create) | - | 未找到 | - |
| 722 | 创建嵌套块 | POST | /open-apis/docx/v1/documents/:document_id/blocks/:block_id/descendant | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document-block-descendant/create) | - | 未找到 | - |
| 723 | 创建文档 | POST | /open-apis/docx/v1/documents | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document/create) | - | 未找到 | - |
| 724 | 在群公告中创建块 | POST | /open-apis/docx/v1/chats/:chat_id/announcement/blocks/:block_id/children | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/chat-announcement-block-children/create) | - | 未找到 | - |
| 725 | 批量更新块的内容 | PATCH | /open-apis/docx/v1/documents/:document_id/blocks/batch_update | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document-block/batch_update) | - | 未找到 | - |
| 726 | 批量更新群公告块的内容 | PATCH | /open-apis/docx/v1/chats/:chat_id/announcement/blocks/batch_update | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/chat-announcement-block/batch_update) | - | 未找到 | - |
| 727 | 更新块的内容 | PATCH | /open-apis/docx/v1/documents/:document_id/blocks/:block_id | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document-block/patch) | - | 未找到 | - |
| 728 | 删除块 | DELETE | /open-apis/docx/v1/documents/:document_id/blocks/:block_id/children/batch_delete | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document-block-children/batch_delete) | - | 未找到 | - |
| 729 | 删除群公告中的块 | DELETE | /open-apis/docx/v1/chats/:chat_id/announcement/blocks/:block_id/children/batch_delete | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/chat-announcement-block-children/batch_delete) | - | 未找到 | - |

### 📦 drive 模块

| 730 | 下载导出文件 | GET | /open-apis/drive/export_tasks/file/:file_token/download | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/export_task/download) | - | 未找到 | - |
| 731 | 下载文件 | GET | /open-apis/drive/v1/files/:file_token/download | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/download) | - | 未找到 | - |
| 732 | 下载素材 | GET | /open-apis/drive/v1/medias/:file_token/download | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/media/download) | - | 未找到 | - |
| 733 | 判断用户云文档权限 | GET | /open-apis/drive/v1/permissions/:token/members/auth | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/permission-member/auth) | - | 未找到 | - |
| 734 | 查询云文档事件订阅状态 | GET | /open-apis/drive/v1/files/:file_token/get_subscribe | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/get_subscribe) | - | 未找到 | - |
| 735 | 查询导入任务结果 | GET | /open-apis/drive/v1/import_tasks/:ticket | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/import_task/get) | - | 未找到 | - |
| 736 | 查询导出任务结果 | GET | /open-apis/drive/v1/export_tasks/:ticket | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/export_task/get) | - | 未找到 | - |
| 737 | 查询异步任务状态 | GET | /open-apis/drive/v1/files/task_check | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/task_check) | - | 未找到 | - |
| 738 | 获取云文档协作者 | GET | /open-apis/drive/v1/permissions/:token/members | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/permission-member/list) | - | 未找到 | - |
| 739 | 获取云文档所有评论 | GET | /open-apis/drive/v1/files/:file_token/comments | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file-comment/list) | - | 未找到 | - |
| 740 | 获取云文档权限设置 | GET | /open-apis/drive/v2/permissions/:token/public | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uIzNzUjLyczM14iM3MTN/drive-v2/permission-public/get) | - | 未找到 | - |
| 741 | 获取云文档权限设置 | GET | /open-apis/drive/v1/permissions/:token/public | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/permission-public/get) | - | 未找到 | - |
| 742 | 获取云文档的点赞者列表 | GET | /open-apis/drive/v2/files/:file_token/likes | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uIzNzUjLyczM14iM3MTN/drive-v2/file-like/list) | - | 未找到 | - |
| 743 | 获取全文评论 | GET | /open-apis/drive/v1/files/:file_token/comments/:comment_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file-comment/get) | - | 未找到 | - |
| 744 | 获取回复信息 | GET | /open-apis/drive/v1/files/:file_token/comments/:comment_id/replies | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file-comment-reply/list) | - | 未找到 | - |
| 745 | 获取我的空间（根文件夹）元数据 | GET | /open-apis/drive/explorer/v2/root_folder/meta | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/ugTNzUjL4UzM14CO1MTN/get-root-folder-meta) | - | 未找到 | - |
| 746 | 获取文件夹下的文档清单 | GET | /open-apis/drive/explorer/v2/folder/:folderToken/children | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uEjNzUjLxYzM14SM2MTN) | - | 未找到 | - |
| 747 | 获取文件夹中的文件清单 | GET | /open-apis/drive/v1/files | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/list) | - | 未找到 | - |
| 748 | 获取文件夹元数据 | GET | /open-apis/drive/explorer/v2/folder/:folderToken/meta | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uAjNzUjLwYzM14CM2MTN) | - | 未找到 | - |
| 749 | 获取文件统计信息 | GET | /open-apis/drive/v1/files/:file_token/statistics | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file-statistics/get) | - | 未找到 | - |
| 750 | 获取文件访问记录 | GET | /open-apis/drive/v1/files/:file_token/view_records | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file-view_record/list) | - | 未找到 | - |
| 751 | 获取文档版本信息 | GET | /open-apis/drive/v1/files/:file_token/versions/:version_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file-version/get) | - | 未找到 | - |
| 752 | 获取文档版本列表 | GET | /open-apis/drive/v1/files/:file_token/versions | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file-version/list) | - | 未找到 | - |
| 753 | 获取素材临时下载链接 | GET | /open-apis/drive/v1/medias/batch_get_tmp_download_url | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/media/batch_get_tmp_download_url) | - | 未找到 | - |
| 754 | 获取订阅状态 | GET | /open-apis/drive/v1/files/:file_token/subscriptions/:subscription_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file-subscription/get) | - | 未找到 | - |
| 755 | 上传文件 | POST | /open-apis/drive/v1/files/upload_all | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/upload_all) | - | 未找到 | - |
| 756 | 上传素材 | POST | /open-apis/drive/v1/medias/upload_all | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/media/upload_all) | - | 未找到 | - |
| 757 | 分片上传文件-上传分片 | POST | /open-apis/drive/v1/files/upload_part | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/upload_part) | - | 未找到 | - |
| 758 | 分片上传文件-完成上传 | POST | /open-apis/drive/v1/files/upload_finish | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/upload_finish) | - | 未找到 | - |
| 759 | 分片上传文件-预上传 | POST | /open-apis/drive/v1/files/upload_prepare | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/upload_prepare) | - | 未找到 | - |
| 760 | 分片上传素材-上传分片 | POST | /open-apis/drive/v1/medias/upload_part | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/media/upload_part) | - | 未找到 | - |
| 761 | 分片上传素材-完成上传 | POST | /open-apis/drive/v1/medias/upload_finish | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/media/upload_finish) | - | 未找到 | - |
| 762 | 分片上传素材-预上传 | POST | /open-apis/drive/v1/medias/upload_prepare | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/media/upload_prepare) | - | 未找到 | - |
| 763 | 创建导入任务 | POST | /open-apis/drive/v1/import_tasks | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/import_task/create) | - | 未找到 | - |
| 764 | 创建导出任务 | POST | /open-apis/drive/v1/export_tasks | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/export_task/create) | - | 未找到 | - |
| 765 | 创建文件快捷方式 | POST | /open-apis/drive/v1/files/create_shortcut | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/create_shortcut) | - | 未找到 | - |
| 766 | 创建文档版本 | POST | /open-apis/drive/v1/files/:file_token/versions | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file-version/create) | - | 未找到 | - |
| 767 | 创建订阅 | POST | /open-apis/drive/v1/files/:file_token/subscriptions | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file-subscription/create) | - | 未找到 | - |
| 768 | 判断协作者是否有某权限 | POST | /open-apis/drive/permission/member/permitted | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uYzN3UjL2czN14iN3cTN) | - | 未找到 | - |
| 769 | 启用云文档密码 | POST | /open-apis/drive/v1/permissions/:token/public/password | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/permission-public-password/create) | - | 未找到 | - |
| 770 | 增加协作者权限 | POST | /open-apis/drive/v1/permissions/:token/members | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/permission-member/create) | - | 未找到 | - |
| 771 | 复制文件 | POST | /open-apis/drive/v1/files/:file_token/copy | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/copy) | - | 未找到 | - |
| 772 | 复制文档 | POST | /open-apis/drive/explorer/v2/file/copy/files/:fileToken | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uYTNzUjL2UzM14iN1MTN) | - | 未找到 | - |
| 773 | 批量增加协作者权限 | POST | /open-apis/drive/v1/permissions/:token/members/batch_create | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/permission-member/batch_create) | - | 未找到 | - |
| 774 | 批量获取评论 | POST | /open-apis/drive/v1/files/:file_token/comments/batch_query | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file-comment/batch_query) | - | 未找到 | - |
| 775 | 新建文件 | POST | /open-apis/drive/explorer/v2/file/:folderToken | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uQTNzUjL0UzM14CN1MTN) | - | 未找到 | - |
| 776 | 新建文件夹 | POST | /open-apis/drive/explorer/v2/folder/:folderToken | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/ukTNzUjL5UzM14SO1MTN) | - | 未找到 | - |
| 777 | 新建文件夹 | POST | /open-apis/drive/v1/files/create_folder | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/create_folder) | - | 未找到 | - |
| 778 | 添加全文评论 | POST | /open-apis/drive/v1/files/:file_token/comments | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file-comment/create) | - | 未找到 | - |
| 779 | 移动文件或文件夹 | POST | /open-apis/drive/v1/files/:file_token/move | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/move) | - | 未找到 | - |
| 780 | 获取云文档权限设置V2 | POST | /open-apis/drive/permission/v2/public | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uITM3YjLyEzN24iMxcjN) | - | 未找到 | - |
| 781 | 获取文件元数据 | POST | /open-apis/drive/v1/metas/batch_query | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/meta/batch_query) | - | 未找到 | - |
| 782 | 订阅云文档事件 | POST | /open-apis/drive/v1/files/:file_token/subscribe | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/subscribe) | - | 未找到 | - |
| 783 | 转移云文档所有者 | POST | /open-apis/drive/v1/permissions/:token/members/transfer_owner | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/permission-member/transfer_owner) | - | 未找到 | - |
| 784 | 转移拥有者 | POST | /open-apis/drive/permission/member/transfer | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uQzNzUjL0czM14CN3MTN) | - | 未找到 | - |
| 785 | 刷新云文档密码 | PUT | /open-apis/drive/v1/permissions/:token/public/password | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/permission-public-password/update) | - | 未找到 | - |
| 786 | 更新协作者权限 | PUT | /open-apis/drive/v1/permissions/:token/members/:member_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/permission-member/update) | - | 未找到 | - |
| 787 | 更新回复的内容 | PUT | /open-apis/drive/v1/files/:file_token/comments/:comment_id/replies/:reply_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file-comment-reply/update) | - | 未找到 | - |
| 788 | 更新云文档权限设置 | PATCH | /open-apis/drive/v2/permissions/:token/public | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uIzNzUjLyczM14iM3MTN/drive-v2/permission-public/patch) | - | 未找到 | - |
| 789 | 更新云文档权限设置 | PATCH | /open-apis/drive/v1/permissions/:token/public | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/permission-public/patch) | - | 未找到 | - |
| 790 | 更新订阅状态 | PATCH | /open-apis/drive/v1/files/:file_token/subscriptions/:subscription_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file-subscription/patch) | - | 未找到 | - |
| 791 | 解决/恢复评论 | PATCH | /open-apis/drive/v1/files/:file_token/comments/:comment_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file-comment/patch) | - | 未找到 | - |
| 792 | 停用云文档密码 | DELETE | /open-apis/drive/v1/permissions/:token/public/password | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/permission-public-password/delete) | - | 未找到 | - |
| 793 | 删除Doc | DELETE | /open-apis/drive/explorer/v2/file/docs/:docToken | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uATM2UjLwEjN14CMxYTN) | - | 未找到 | - |
| 794 | 删除Sheet | DELETE | /open-apis/drive/explorer/v2/file/spreadsheets/:spreadsheetToken | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uUTNzUjL1UzM14SN1MTN/delete-sheet) | - | 未找到 | - |
| 795 | 删除回复 | DELETE | /open-apis/drive/v1/files/:file_token/comments/:comment_id/replies/:reply_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file-comment-reply/delete) | - | 未找到 | - |
| 796 | 删除文件或文件夹 | DELETE | /open-apis/drive/v1/files/:file_token | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/delete) | - | 未找到 | - |
| 797 | 删除文档版本 | DELETE | /open-apis/drive/v1/files/:file_token/versions/:version_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file-version/delete) | - | 未找到 | - |
| 798 | 取消云文档事件订阅 | DELETE | /open-apis/drive/v1/files/:file_token/delete_subscribe | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/delete_subscribe) | - | 未找到 | - |
| 799 | 移除云文档协作者权限 | DELETE | /open-apis/drive/v1/permissions/:token/members/:member_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/permission-member/delete) | - | 未找到 | - |

### 📦 ehr 模块

| 800 | 下载人员的附件 | GET | /open-apis/ehr/v1/attachments/:token | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/ehr/ehr-v1/attachment/get) | - | 未找到 | - |
| 801 | 批量获取员工花名册信息 | GET | /open-apis/ehr/v1/employees | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/ehr/ehr-v1/employee/list) | - | 未找到 | - |

### 📦 ephemeral 模块

| 802 | 删除仅特定人可见的消息卡片 | POST | /open-apis/ephemeral/v1/delete | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uITOyYjLykjM24iM5IjN) | - | 未找到 | - |
| 803 | 发送仅特定人可见的消息卡片 | POST | /open-apis/ephemeral/v1/send | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uETOyYjLxkjM24SM5IjN) | - | 未找到 | - |

### 📦 event 模块

| 804 | 获取事件出口 IP | GET | /open-apis/event/v1/outbound_ip | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uYDNxYjL2QTM24iN0EjN/event-v1/outbound_ip/list) | - | 未找到 | - |

### 📦 face_verify 模块

| 805 | 查询人脸认证结果 | GET | /open-apis/face_verify/v1/query_auth_result | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/human_authentication-v1/face/query-recognition-result) | - | 未找到 | - |
| 806 | 上传人脸基准图片 | POST | /open-apis/face_verify/v1/upload_face_image | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/human_authentication-v1/face/upload-facial-reference-image) | - | 未找到 | - |
| 807 | 裁剪人脸图片 | POST | /open-apis/face_verify/v1/crop_face_image | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/human_authentication-v1/face/facial-image-cropping) | - | 未找到 | - |

### 📦 helpdesk 模块

| 808 | 搜索知识库 | GET | /open-apis/helpdesk/v1/faqs/search | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/faq/search) | - | 未找到 | - |
| 809 | 查询全部客服工作日程 | GET | /open-apis/helpdesk/v1/agent_schedules | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/agent_schedule/list) | - | 未找到 | - |
| 810 | 查询全部客服技能 | GET | /open-apis/helpdesk/v1/agent_skills | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/agent_skill/list) | - | 未找到 | - |
| 811 | 查询全部工单详情 | GET | /open-apis/helpdesk/v1/tickets | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/ticket/list) | - | 未找到 | - |
| 812 | 查询指定客服工作日程 | GET | /open-apis/helpdesk/v1/agents/:agent_id/schedules | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/agent-schedules/get) | - | 未找到 | - |
| 813 | 查询指定客服技能 | GET | /open-apis/helpdesk/v1/agent_skills/:agent_skill_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/agent_skill/get) | - | 未找到 | - |
| 814 | 查询指定工单详情 | GET | /open-apis/helpdesk/v1/tickets/:ticket_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/ticket/get) | - | 未找到 | - |
| 815 | 查询推送 | GET | /open-apis/helpdesk/v1/notifications/:notification_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/notification/get) | - | 未找到 | - |
| 816 | 获取全部工单自定义字段 | GET | /open-apis/helpdesk/v1/ticket_customized_fields | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/ticket_customized_field/list-ticket-customized-fields) | - | 未找到 | - |
| 817 | 获取全部知识库分类 | GET | /open-apis/helpdesk/v1/categories | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/category/list-categories) | - | 未找到 | - |
| 818 | 获取全部知识库详情 | GET | /open-apis/helpdesk/v1/faqs | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/faq/list) | - | 未找到 | - |
| 819 | 获取客服技能列表 | GET | /open-apis/helpdesk/v1/agent_skill_rules | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/agent_skill_rule/list) | - | 未找到 | - |
| 820 | 获取客服邮箱 | GET | /open-apis/helpdesk/v1/agent_emails | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/agent/agent_email) | - | 未找到 | - |
| 821 | 获取工单内图像 | GET | /open-apis/helpdesk/v1/ticket_images | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/ticket/ticket_image) | - | 未找到 | - |
| 822 | 获取工单消息详情 | GET | /open-apis/helpdesk/v1/tickets/:ticket_id/messages | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/ticket-message/list) | - | 未找到 | - |
| 823 | 获取指定工单自定义字段 | GET | /open-apis/helpdesk/v1/ticket_customized_fields/:ticket_customized_field_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/ticket_customized_field/get-ticket-customized-field) | - | 未找到 | - |
| 824 | 获取指定知识库详情 | GET | /open-apis/helpdesk/v1/faqs/:id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/faq/get) | - | 未找到 | - |
| 825 | 获取服务台自定义字段 | GET | /open-apis/helpdesk/v1/customized_fields | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/ticket/customized_fields) | - | 未找到 | - |
| 826 | 获取知识库分类 | GET | /open-apis/helpdesk/v1/categories/:id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/category/get) | - | 未找到 | - |
| 827 | 获取知识库图像 | GET | /open-apis/helpdesk/v1/faqs/:id/image/:image_key | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/faq/faq_image) | - | 未找到 | - |
| 828 | 创建客服工作日程 | POST | /open-apis/helpdesk/v1/agent_schedules | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/agent_schedule/create) | - | 未找到 | - |
| 829 | 创建客服技能 | POST | /open-apis/helpdesk/v1/agent_skills | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/agent_skill/create) | - | 未找到 | - |
| 830 | 创建工单自定义字段 | POST | /open-apis/helpdesk/v1/ticket_customized_fields | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/ticket_customized_field/create-ticket-customized-field) | - | 未找到 | - |
| 831 | 创建推送 | POST | /open-apis/helpdesk/v1/notifications | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/notification/create) | - | 未找到 | - |
| 832 | 创建服务台对话 | POST | /open-apis/helpdesk/v1/start_service | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/ticket/start_service) | - | 未找到 | - |
| 833 | 创建知识库 | POST | /open-apis/helpdesk/v1/faqs | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/faq/create) | - | 未找到 | - |
| 834 | 创建知识库分类 | POST | /open-apis/helpdesk/v1/categories | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/category/create) | - | 未找到 | - |
| 835 | 发送工单消息 | POST | /open-apis/helpdesk/v1/tickets/:ticket_id/messages | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/ticket-message/create) | - | 未找到 | - |
| 836 | 取消审核 | POST | /open-apis/helpdesk/v1/notifications/:notification_id/cancel_approve | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/notification/cancel_approve) | - | 未找到 | - |
| 837 | 取消推送 | POST | /open-apis/helpdesk/v1/notifications/:notification_id/cancel_send | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/notification/cancel_send) | - | 未找到 | - |
| 838 | 取消订阅服务台事件 | POST | /open-apis/helpdesk/v1/events/unsubscribe | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/event/unsubscribe) | - | 未找到 | - |
| 839 | 回复用户在工单里的提问 | POST | /open-apis/helpdesk/v1/tickets/:ticket_id/answer_user_query | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/ticket/answer_user_query) | - | 未找到 | - |
| 840 | 执行推送 | POST | /open-apis/helpdesk/v1/notifications/:notification_id/execute_send | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/notification/execute_send) | - | 未找到 | - |
| 841 | 提交审核 | POST | /open-apis/helpdesk/v1/notifications/:notification_id/submit_approve | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/notification/submit_approve) | - | 未找到 | - |
| 842 | 服务台机器人向工单绑定的群内发送消息 | POST | /open-apis/helpdesk/v1/message | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/bot-message/create) | - | 未找到 | - |
| 843 | 订阅服务台事件 | POST | /open-apis/helpdesk/v1/events/subscribe | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/event/subscribe) | - | 未找到 | - |
| 844 | 预览推送 | POST | /open-apis/helpdesk/v1/notifications/:notification_id/preview | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/notification/preview) | - | 未找到 | - |
| 845 | 更新工单详情 | PUT | /open-apis/helpdesk/v1/tickets/:ticket_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/ticket/update) | - | 未找到 | - |
| 846 | 修改知识库 | PATCH | /open-apis/helpdesk/v1/faqs/:id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/faq/patch) | - | 未找到 | - |
| 847 | 更新客服信息 | PATCH | /open-apis/helpdesk/v1/agents/:agent_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/agent/patch) | - | 未找到 | - |
| 848 | 更新客服工作日程 | PATCH | /open-apis/helpdesk/v1/agents/:agent_id/schedules | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/agent-schedules/patch) | - | 未找到 | - |
| 849 | 更新客服技能 | PATCH | /open-apis/helpdesk/v1/agent_skills/:agent_skill_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/agent_skill/patch) | - | 未找到 | - |
| 850 | 更新工单自定义字段 | PATCH | /open-apis/helpdesk/v1/ticket_customized_fields/:ticket_customized_field_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/ticket_customized_field/update-ticket-customized-field) | - | 未找到 | - |
| 851 | 更新推送 | PATCH | /open-apis/helpdesk/v1/notifications/:notification_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/notification/patch) | - | 未找到 | - |
| 852 | 更新知识库分类详情 | PATCH | /open-apis/helpdesk/v1/categories/:id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/category/patch) | - | 未找到 | - |
| 853 | 删除客服工作日程 | DELETE | /open-apis/helpdesk/v1/agents/:agent_id/schedules | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/agent-schedules/delete) | - | 未找到 | - |
| 854 | 删除客服技能 | DELETE | /open-apis/helpdesk/v1/agent_skills/:agent_skill_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/agent_skill/delete) | - | 未找到 | - |
| 855 | 删除工单自定义字段 | DELETE | /open-apis/helpdesk/v1/ticket_customized_fields/:ticket_customized_field_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/ticket_customized_field/delete) | - | 未找到 | - |
| 856 | 删除知识库 | DELETE | /open-apis/helpdesk/v1/faqs/:id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/faq/delete) | - | 未找到 | - |
| 857 | 删除知识库分类详情 | DELETE | /open-apis/helpdesk/v1/categories/:id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/category/delete) | - | 未找到 | - |

### 📦 hire 模块

| 858 | 批量获取待办事项 | GET | /open-apis/hire/v1/todos | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/todo/list) | - | 未找到 | - |
| 859 | 批量获取面试评价详细信息 | GET | /open-apis/hire/v1/interview_records | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/interview_record/list) | - | 未找到 | - |
| 860 | 批量获取面试评价详细信息（新版） | GET | /open-apis/hire/v2/interview_records | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/hire-v2/interview_record/list) | - | 未找到 | - |
| 861 | 查询内推账户 | GET | /open-apis/hire/v1/referral_account/get_account_assets | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/referral_account/get_account_assets) | - | 未找到 | - |
| 862 | 查询外部投递列表 | GET | /open-apis/hire/v1/external_applications | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/external_application/list) | - | 未找到 | - |
| 863 | 查询猎头供应商信息 | GET | /open-apis/hire/v1/agencies/query | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/agency/query) | - | 未找到 | - |
| 864 | 查询面试官信息列表 | GET | /open-apis/hire/v1/interviewers | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/interviewer/list) | - | 未找到 | - |
| 865 | 获取 Offer 信息 | GET | /open-apis/hire/v1/applications/:application_id/offer | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/application/offer) | - | 未找到 | - |
| 866 | 获取 Offer 列表 | GET | /open-apis/hire/v1/offers | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/offer/list) | - | 未找到 | - |
| 867 | 获取 Offer 审批流列表 | GET | /open-apis/hire/v1/offer_approval_templates | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/offer_approval_template/list) | - | 未找到 | - |
| 868 | 获取 Offer 申请表信息 | GET | /open-apis/hire/v1/offer_application_forms/:offer_application_form_id | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/offer_application_form/get) | - | 未找到 | - |
| 869 | 获取 Offer 申请表列表 | GET | /open-apis/hire/v1/offer_application_forms | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/offer_application_form/list) | - | 未找到 | - |
| 870 | 获取 Offer 申请表详细信息 | GET | /open-apis/hire/v1/offer_schemas/:offer_schema_id | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/offer_schema/get) | - | 未找到 | - |
| 871 | 获取 Offer 详情 | GET | /open-apis/hire/v1/offers/:offer_id | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/offer/get) | - | 未找到 | - |
| 872 | 获取三方协议 | GET | /open-apis/hire/v1/tripartite_agreements | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/tripartite_agreement/list) | - | 未找到 | - |
| 873 | 获取人才信息 | GET | /open-apis/hire/v1/talents/:talent_id | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/talent/get) | - | 未找到 | - |
| 874 | 获取人才列表 | GET | /open-apis/hire/v1/talents | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/talent/list) | - | 未找到 | - |
| 875 | 获取人才字段 | GET | /open-apis/hire/v1/talent_objects/query | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/talent_object/query) | - | 未找到 | - |
| 876 | 获取人才库列表 | GET | /open-apis/hire/v1/talent_pools | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/talent_pool/search) | - | 未找到 | - |
| 877 | 获取人才文件夹列表 | GET | /open-apis/hire/v1/talent_folders | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/talent_folder/list) | - | 未找到 | - |
| 878 | 获取人才标签信息列表 | GET | /open-apis/hire/v1/talent_tags | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/talent_tag/list) | - | 未找到 | - |
| 879 | 获取人才详情 | GET | /open-apis/hire/v2/talents/:talent_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/hire-v2/talent/get) | - | 未找到 | - |
| 880 | 获取人才面试信息 | GET | /open-apis/hire/v1/interviews/get_by_talent | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/interview/get_by_talent) | - | 未找到 | - |
| 881 | 获取信息登记表列表 | GET | /open-apis/hire/v1/registration_schemas | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/registration_schema/list) | - | 未找到 | - |
| 882 | 获取内推信息 | GET | /open-apis/hire/v1/referrals/get_by_application | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/referral/get_by_application) | - | 未找到 | - |
| 883 | 获取内推官网下职位广告列表 | GET | /open-apis/hire/v1/referral_websites/job_posts | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/referral_website-job_post/list) | - | 未找到 | - |
| 884 | 获取内推官网下职位广告详情 | GET | /open-apis/hire/v1/referral_websites/job_posts/:job_post_id | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/referral_website-job_post/get) | - | 未找到 | - |
| 885 | 获取地址列表 | GET | /open-apis/hire/v1/locations | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/location/list) | - | 未找到 | - |
| 886 | 获取备注 | GET | /open-apis/hire/v1/notes/:note_id | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/note/get) | - | 未找到 | - |
| 887 | 获取备注列表 | GET | /open-apis/hire/v1/notes | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/note/list) | - | 未找到 | - |
| 888 | 获取投递信息 | GET | /open-apis/hire/v1/applications/:application_id | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/application/get) | - | 未找到 | - |
| 889 | 获取投递列表 | GET | /open-apis/hire/v1/applications | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/application/list) | - | 未找到 | - |
| 890 | 获取投递详情 | GET | /open-apis/hire/v1/applications/:application_id/get_detail | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/application/get_detail) | - | 未找到 | - |
| 891 | 获取招聘官网下的职位广告列表 | GET | /open-apis/hire/v1/websites/:website_id/job_posts | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/website-job_post/list) | - | 未找到 | - |
| 892 | 获取招聘官网下职位广告详情 | GET | /open-apis/hire/v1/websites/:website_id/job_posts/:job_post_id | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/website-job_post/get) | - | 未找到 | - |
| 893 | 获取招聘官网列表 | GET | /open-apis/hire/v1/websites | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/website/list) | - | 未找到 | - |
| 894 | 获取招聘官网投递任务结果 | GET | /open-apis/hire/v1/websites/:website_id/delivery_tasks/:delivery_task_id | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/website-delivery_task/get) | - | 未找到 | - |
| 895 | 获取招聘官网推广渠道列表 | GET | /open-apis/hire/v1/websites/:website_id/channels | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/website-channel/list) | - | 未找到 | - |
| 896 | 获取招聘流程信息 | GET | /open-apis/hire/v1/job_processes | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/job_process/list) | - | 未找到 | - |
| 897 | 获取招聘需求列表 | GET | /open-apis/hire/v1/job_requirements | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/job_requirement/list) | - | 未找到 | - |
| 898 | 获取招聘需求模板列表 | GET | /open-apis/hire/v1/job_requirement_schemas | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/job_requirement_schema/list) | - | 未找到 | - |
| 899 | 获取猎头供应商信息 | GET | /open-apis/hire/v1/agencies/:agency_id | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/agency/get) | - | 未找到 | - |
| 900 | 获取用户角色列表 | GET | /open-apis/hire/v1/user_roles | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/user_role/list) | - | 未找到 | - |
| 901 | 获取申请表模板列表 | GET | /open-apis/hire/v1/portal_apply_schemas | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/portal_apply_schema/list) | - | 未找到 | - |
| 902 | 获取笔试阅卷任务列表 | GET | /open-apis/hire/v1/exam_marking_tasks | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/exam_marking_task/list) | - | 未找到 | - |
| 903 | 获取简历来源列表 | GET | /open-apis/hire/v1/resume_sources | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/resume_source/list) | - | 未找到 | - |
| 904 | 获取简历评估任务列表 | GET | /open-apis/hire/v1/evaluation_tasks | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/evaluation_task/list) | - | 未找到 | - |
| 905 | 获取简历评估信息列表 | GET | /open-apis/hire/v1/evaluations | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/evaluation/list) | - | 未找到 | - |
| 906 | 获取终止投递原因 | GET | /open-apis/hire/v1/termination_reasons | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/termination_reason/list) | - | 未找到 | - |
| 907 | 获取职位上的招聘人员信息 | GET | /open-apis/hire/v1/jobs/:job_id/recruiter | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/job/recruiter) | - | 未找到 | - |
| 908 | 获取职位上的招聘人员信息 | GET | /open-apis/hire/v1/jobs/:job_id/managers/:manager_id | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/job-manager/get) | - | 未找到 | - |
| 909 | 获取职位信息 | GET | /open-apis/hire/v1/jobs/:job_id | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/job/get) | - | 未找到 | - |
| 910 | 获取职位列表 | GET | /open-apis/hire/v1/jobs | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/job/list) | - | 未找到 | - |
| 911 | 获取职位模板 | GET | /open-apis/hire/v1/job_schemas | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/job_schema/list) | - | 未找到 | - |
| 912 | 获取职位类别列表 | GET | /open-apis/hire/v1/job_types | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/job_type/list) | - | 未找到 | - |
| 913 | 获取职位设置 | GET | /open-apis/hire/v1/jobs/:job_id/config | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/job/config) | - | 未找到 | - |
| 914 | 获取职位详情 | GET | /open-apis/hire/v1/jobs/:job_id/get_detail | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/job/get_detail) | - | 未找到 | - |
| 915 | 获取职能分类列表 | GET | /open-apis/hire/v1/job_functions | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/job_function/list) | - | 未找到 | - |
| 916 | 获取背调信息列表 | GET | /open-apis/hire/v1/background_check_orders | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/background_check_order/list) | - | 未找到 | - |
| 917 | 获取角色列表 | GET | /open-apis/hire/v1/roles | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/role/list) | - | 未找到 | - |
| 918 | 获取角色详情 | GET | /open-apis/hire/v1/roles/:role_id | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/role/get) | - | 未找到 | - |
| 919 | 获取附件 PDF 格式下载链接 | GET | /open-apis/hire/v1/attachments/:attachment_id/preview | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/attachment/preview) | - | 未找到 | - |
| 920 | 获取附件信息 | GET | /open-apis/hire/v1/attachments/:attachment_id | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/attachment/get) | - | 未找到 | - |
| 921 | 获取面试任务列表 | GET | /open-apis/hire/v1/interview_tasks | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/interview_task/list) | - | 未找到 | - |
| 922 | 获取面试信息 | GET | /open-apis/hire/v1/interviews | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/interview/list) | - | 未找到 | - |
| 923 | 获取面试满意度问卷列表 | GET | /open-apis/hire/v1/questionnaires | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/questionnaire/list) | - | 未找到 | - |
| 924 | 获取面试登记表列表 | GET | /open-apis/hire/v1/interview_registration_schemas | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/interview_registration_schema/list) | - | 未找到 | - |
| 925 | 获取面试记录列表 | GET | /open-apis/hire/v1/applications/:application_id/interviews | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/application-interview/list) | - | 未找到 | - |
| 926 | 获取面试记录附件 | GET | /open-apis/hire/v1/interview_records/attachments | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/interview_record-attachment/get) | - | 未找到 | - |
| 927 | 获取面试评价表列表 | GET | /open-apis/hire/v1/interview_feedback_forms | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/interview_feedback_form/list) | - | 未找到 | - |
| 928 | 获取面试评价详细信息 | GET | /open-apis/hire/v1/interview_records/:interview_record_id | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/interview_record/get) | - | 未找到 | - |
| 929 | 获取面试评价详细信息（新版） | GET | /open-apis/hire/v2/interview_records/:interview_record_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/hire-v2/interview_record/get) | - | 未找到 | - |
| 930 | 获取面试轮次类型列表 | GET | /open-apis/hire/v1/interview_round_types | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/interview_round_type/list) | - | 未找到 | - |
| 931 | 获取面试速记明细 | GET | /open-apis/hire/v1/minutes | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/minutes/get) | - | 未找到 | - |
| 932 | 获取项目列表 | GET | /open-apis/hire/v1/subjects | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/subject/list) | - | 未找到 | - |
| 933 | 通过员工 ID 获取入职信息 | GET | /open-apis/hire/v1/employees/:employee_id | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/employee/get) | - | 未找到 | - |
| 934 | 通过投递 ID 获取入职信息 | GET | /open-apis/hire/v1/employees/get_by_application | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/employee/get_by_application) | - | 未找到 | - |
| 935 | 停用内推账户 | POST | /open-apis/hire/v1/referral_account/:referral_account_id/deactivate | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/referral_account/deactivate) | - | 未找到 | - |
| 936 | 全额提取内推账户余额 | POST | /open-apis/hire/v1/referral_account/:referral_account_id/withdraw | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/referral_account/withdraw) | - | 未找到 | - |
| 937 | 关闭职位 | POST | /open-apis/hire/v1/jobs/:job_id/close | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/job/close) | - | 未找到 | - |
| 938 | 内推账户提现数据对账 | POST | /open-apis/hire/v1/referral_account/reconciliation | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/referral_account/reconciliation) | - | 未找到 | - |
| 939 | 创建 Offer | POST | /open-apis/hire/v1/offers | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/offer/create) | - | 未找到 | - |
| 940 | 创建三方协议 | POST | /open-apis/hire/v1/tripartite_agreements | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/tripartite_agreement/create) | - | 未找到 | - |
| 941 | 创建人才 | POST | /open-apis/hire/v1/talents/combined_create | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/talent/combined_create) | - | 未找到 | - |
| 942 | 创建人才外部信息 | POST | /open-apis/hire/v1/talents/:talent_id/external_info | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/talent-external_info/create) | - | 未找到 | - |
| 943 | 创建备注 | POST | /open-apis/hire/v1/notes | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/note/create) | - | 未找到 | - |
| 944 | 创建外部 Offer | POST | /open-apis/hire/v1/external_offers | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/external_offer/create) | - | 未找到 | - |
| 945 | 创建外部投递 | POST | /open-apis/hire/v1/external_applications | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/external_application/create) | - | 未找到 | - |
| 946 | 创建外部背调 | POST | /open-apis/hire/v1/external_background_checks | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/external_background_check/create) | - | 未找到 | - |
| 947 | 创建外部面评 | POST | /open-apis/hire/v1/external_interview_assessments | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/external_interview_assessment/create) | - | 未找到 | - |
| 948 | 创建外部面试 | POST | /open-apis/hire/v1/external_interviews | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/external_interview/create) | - | 未找到 | - |
| 949 | 创建投递 | POST | /open-apis/hire/v1/applications | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/application/create) | - | 未找到 | - |
| 950 | 创建招聘需求 | POST | /open-apis/hire/v1/job_requirements | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/job_requirement/create) | - | 未找到 | - |
| 951 | 创建背调套餐和附加调查项 | POST | /open-apis/hire/v1/eco_background_check_packages | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/eco_background_check_package/create) | - | 未找到 | - |
| 952 | 创建背调自定义字段 | POST | /open-apis/hire/v1/eco_background_check_custom_fields | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/eco_background_check_custom_field/create) | - | 未找到 | - |
| 953 | 创建试卷列表 | POST | /open-apis/hire/v1/eco_exam_papers | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/eco_exam_paper/create) | - | 未找到 | - |
| 954 | 创建账号自定义字段 | POST | /open-apis/hire/v1/eco_account_custom_fields | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/eco_account_custom_field/create) | - | 未找到 | - |
| 955 | 创建附件 | POST | /open-apis/hire/v1/attachments | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uIDN1YjLyQTN24iM0UjN/create_attachment) | - | 未找到 | - |
| 956 | 删除背调套餐和附加调查项 | POST | /open-apis/hire/v1/eco_background_check_packages/batch_delete | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/eco_background_check_package/batch_delete) | - | 未找到 | - |
| 957 | 删除背调自定义字段 | POST | /open-apis/hire/v1/eco_background_check_custom_fields/batch_delete | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/eco_background_check_custom_field/batch_delete) | - | 未找到 | - |
| 958 | 删除试卷列表 | POST | /open-apis/hire/v1/eco_exam_papers/batch_delete | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/eco_exam_paper/batch_delete) | - | 未找到 | - |
| 959 | 删除账号自定义字段 | POST | /open-apis/hire/v1/eco_account_custom_fields/batch_delete | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/eco_account_custom_field/batch_delete) | - | 未找到 | - |
| 960 | 加入/移除屏蔽名单 | POST | /open-apis/hire/v1/talent_blocklist/change_talent_block | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/talent_blocklist/change_talent_block) | - | 未找到 | - |
| 961 | 发布职位广告 | POST | /open-apis/hire/v1/advertisements/:advertisement_id/publish | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/advertisement/publish) | - | 未找到 | - |
| 962 | 取消候选人入职 | POST | /open-apis/hire/v1/applications/:application_id/cancel_onboard | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/application/cancel_onboard) | - | 未找到 | - |
| 963 | 启用内推账户 | POST | /open-apis/hire/v1/referral_account/enable | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/referral_account/enable) | - | 未找到 | - |
| 964 | 回传笔试安排结果 | POST | /open-apis/hire/v1/eco_exams/:exam_id/login_info | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/eco_exam/login_info) | - | 未找到 | - |
| 965 | 回传笔试结果 | POST | /open-apis/hire/v1/eco_exams/:exam_id/update_result | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/eco_exam/update_result) | - | 未找到 | - |
| 966 | 回传背调订单的最终结果 | POST | /open-apis/hire/v1/eco_background_checks/update_result | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/eco_background_check/update_result) | - | 未找到 | - |
| 967 | 导入外部内推奖励 | POST | /open-apis/hire/v1/external_referral_rewards | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/external_referral_reward/create) | - | 未找到 | - |
| 968 | 将人才从指定文件夹移除 | POST | /open-apis/hire/v1/talents/remove_to_folder | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/talent/remove_to_folder) | - | 未找到 | - |
| 969 | 将人才加入人才库 | POST | /open-apis/hire/v1/talent_pools/:talent_pool_id/talent_relationship | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/talent_pool/move_talent) | - | 未找到 | - |
| 970 | 将人才加入指定文件夹 | POST | /open-apis/hire/v1/talents/add_to_folder | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/talent/add_to_folder) | - | 未找到 | - |
| 971 | 恢复投递 | POST | /open-apis/hire/v1/applications/:application_id/recover | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/application/recover) | - | 未找到 | - |
| 972 | 批量加入/移除人才库中人才 | POST | /open-apis/hire/v1/talent_pools/:talent_pool_id/batch_change_talent_pool | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/talent_pool/batch_change_talent_pool) | - | 未找到 | - |
| 973 | 批量获取人才ID | POST | /open-apis/hire/v1/talents/batch_get_id | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/talent/batch_get_id) | - | 未找到 | - |
| 974 | 搜索招聘官网下的职位广告列表 | POST | /open-apis/hire/v1/websites/:website_id/job_posts/search | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/website-job_post/search) | - | 未找到 | - |
| 975 | 搜索猎头供应商列表 | POST | /open-apis/hire/v1/agencies/batch_query | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/agency/batch_query) | - | 未找到 | - |
| 976 | 操作人才标签 | POST | /open-apis/hire/talents/:talent_id/tag | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/talent/tag) | - | 未找到 | - |
| 977 | 操作候选人入职 | POST | /open-apis/hire/v1/applications/:application_id/transfer_onboard | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/application/transfer_onboard) | - | 未找到 | - |
| 978 | 新建招聘官网投递 | POST | /open-apis/hire/v1/websites/:website_id/deliveries/create_by_resume | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/website-delivery/create_by_resume) | - | 未找到 | - |
| 979 | 新建招聘官网推广渠道 | POST | /open-apis/hire/v1/websites/:website_id/channels | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/website-channel/create) | - | 未找到 | - |
| 980 | 新建招聘官网用户 | POST | /open-apis/hire/v1/websites/:website_id/site_users | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/website-site_user/create) | - | 未找到 | - |
| 981 | 新建职位 | POST | /open-apis/hire/v1/jobs/combined_create | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/job/combined_create) | - | 未找到 | - |
| 982 | 更新人才 | POST | /open-apis/hire/v1/talents/combined_update | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/talent/combined_update) | - | 未找到 | - |
| 983 | 更新人才在职状态 | POST | /open-apis/hire/v1/talents/:talent_id/onboard_status | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/talent/onboard_status) | - | 未找到 | - |
| 984 | 更新实习 Offer 入/离职状态 | POST | /open-apis/hire/v1/offers/:offer_id/intern_offer_status | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/offer/intern_offer_status) | - | 未找到 | - |
| 985 | 更新职位 | POST | /open-apis/hire/v1/jobs/:job_id/combined_update | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/job/combined_update) | - | 未找到 | - |
| 986 | 更新职位相关人员 | POST | /open-apis/hire/v1/jobs/:job_id/managers/batch_update | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/job-manager/batch_update) | - | 未找到 | - |
| 987 | 更新职位设置 | POST | /open-apis/hire/v1/jobs/:job_id/update_config | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/job/update_config) | - | 未找到 | - |
| 988 | 更新背调订单进度 | POST | /open-apis/hire/v1/eco_background_checks/update_progress | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/eco_background_check/update_progress) | - | 未找到 | - |
| 989 | 查询人才内推信息 | POST | /open-apis/hire/v1/referrals/search | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/referral/search) | - | 未找到 | - |
| 990 | 查询人才操作记录 | POST | /open-apis/hire/v1/talent_operation_logs/search | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/talent/talent_operation_log/search) | - | 未找到 | - |
| 991 | 查询地点列表 | POST | /open-apis/hire/locations/query | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/location/query) | - | 未找到 | - |
| 992 | 查询外部 Offer 列表 | POST | /open-apis/hire/v1/external_offers/batch_query | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/external_offer/batch_query) | - | 未找到 | - |
| 993 | 查询外部背调列表 | POST | /open-apis/hire/v1/external_background_checks/batch_query | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/external_background_check/batch_query) | - | 未找到 | - |
| 994 | 查询外部面试列表 | POST | /open-apis/hire/v1/external_interviews/batch_query | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/external_interview/batch_query) | - | 未找到 | - |
| 995 | 查询猎头供应商下猎头列表 | POST | /open-apis/hire/v1/agencies/get_agency_account | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/agency/get_agency_account) | - | 未找到 | - |
| 996 | 查询猎头保护期信息 | POST | /open-apis/hire/v1/agencies/protection_period/search | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/agency/protect_search) | - | 未找到 | - |
| 997 | 查询背调信息列表 | POST | /open-apis/hire/v1/background_check_orders/batch_query | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/background_check_order/batch_query) | - | 未找到 | - |
| 998 | 根据简历附件创建招聘官网投递任务 | POST | /open-apis/hire/v1/websites/:website_id/deliveries/create_by_attachment | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/website-delivery/create_by_attachment) | - | 未找到 | - |
| 999 | 注册内推账户 | POST | /open-apis/hire/v1/referral_account | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/referral_account/create) | - | 未找到 | - |
| 1000 | 添加笔试结果 | POST | /open-apis/hire/v1/exams | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/exam/create) | - | 未找到 | - |
| 1001 | 禁用/取消禁用猎头 | POST | /open-apis/hire/v1/agencies/operate_agency_account | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/agency/operate_agency_account) | - | 未找到 | - |
| 1002 | 终止投递 | POST | /open-apis/hire/v1/applications/:application_id/terminate | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/application/terminate) | - | 未找到 | - |
| 1003 | 终止背调订单 | POST | /open-apis/hire/v1/eco_background_checks/cancel | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/eco_background_check/cancel) | - | 未找到 | - |
| 1004 | 获取招聘需求信息 | POST | /open-apis/hire/job_requirements/search | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/job_requirement/list_by_id) | - | 未找到 | - |
| 1005 | 获取申请表附加信息 | POST | /open-apis/hire/v1/applications/diversity_inclusions/search | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/diversity_inclusion/search) | - | 未找到 | - |
| 1006 | 获取笔试列表 | POST | /open-apis/hire/v1/tests/search | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/test/search) | - | 未找到 | - |
| 1007 | 获取职位广告发布记录 | POST | /open-apis/hire/v1/job_publish_records/search | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/job_publish_record/search) | - | 未找到 | - |
| 1008 | 设置猎头保护期 | POST | /open-apis/hire/v1/agencies/protect | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/agency/protect) | - | 未找到 | - |
| 1009 | 转移投递阶段 | POST | /open-apis/hire/v1/applications/:application_id/transfer_stage | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/application/transfer_stage) | - | 未找到 | - |
| 1010 | 重启职位 | POST | /open-apis/hire/v1/jobs/:job_id/open | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/job/open) | - | 未找到 | - |
| 1011 | 更新 Offer 信息 | PUT | /open-apis/hire/v1/offers/:offer_id | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/offer/update) | - | 未找到 | - |
| 1012 | 更新 Offer 申请表自定义字段 | PUT | /open-apis/hire/v1/offer_custom_fields/:offer_custom_field_id | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/offer_custom_field/update) | - | 未找到 | - |
| 1013 | 更新三方协议 | PUT | /open-apis/hire/v1/tripartite_agreements/:tripartite_agreement_id | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/tripartite_agreement/update) | - | 未找到 | - |
| 1014 | 更新人才外部信息 | PUT | /open-apis/hire/v1/talents/:talent_id/external_info | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/talent-external_info/update) | - | 未找到 | - |
| 1015 | 更新外部 Offer | PUT | /open-apis/hire/v1/external_offers/:external_offer_id | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/external_offer/update) | - | 未找到 | - |
| 1016 | 更新外部投递 | PUT | /open-apis/hire/v1/external_applications/:external_application_id | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/external_application/update) | - | 未找到 | - |
| 1017 | 更新外部背调 | PUT | /open-apis/hire/v1/external_background_checks/:external_background_check_id | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/external_background_check/update) | - | 未找到 | - |
| 1018 | 更新外部面试 | PUT | /open-apis/hire/v1/external_interviews/:external_interview_id | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/external_interview/update) | - | 未找到 | - |
| 1019 | 更新招聘官网推广渠道 | PUT | /open-apis/hire/v1/websites/:website_id/channels/:channel_id | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/website-channel/update) | - | 未找到 | - |
| 1020 | 更新招聘需求 | PUT | /open-apis/hire/v1/job_requirements/:job_requirement_id | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/job_requirement/update) | - | 未找到 | - |
| 1021 | 更新 Offer 状态 | PATCH | /open-apis/hire/v1/offers/:offer_id/offer_status | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/offer/offer_status) | - | 未找到 | - |
| 1022 | 更新 e-HR 导入任务结果 | PATCH | /open-apis/hire/v1/ehr_import_tasks/:ehr_import_task_id | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/ehr_import_task/patch) | - | 未找到 | - |
| 1023 | 更新员工状态 | PATCH | /open-apis/hire/v1/employees/:employee_id | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/employee/patch) | - | 未找到 | - |
| 1024 | 更新备注 | PATCH | /open-apis/hire/v1/notes/:note_id | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/note/patch) | - | 未找到 | - |
| 1025 | 更新外部面评 | PATCH | /open-apis/hire/v1/external_interview_assessments/:external_interview_assessment_id | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/external_interview_assessment/patch) | - | 未找到 | - |
| 1026 | 更新背调套餐和附加调查项 | PATCH | /open-apis/hire/v1/eco_background_check_packages/batch_update | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/eco_background_check_package/batch_update) | - | 未找到 | - |
| 1027 | 更新背调自定义字段 | PATCH | /open-apis/hire/v1/eco_background_check_custom_fields/batch_update | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/eco_background_check_custom_field/batch_update) | - | 未找到 | - |
| 1028 | 更新试卷列表 | PATCH | /open-apis/hire/v1/eco_exam_papers/batch_update | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/eco_exam_paper/batch_update) | - | 未找到 | - |
| 1029 | 更新账号自定义字段 | PATCH | /open-apis/hire/v1/eco_account_custom_fields/batch_update | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/eco_account_custom_field/batch_update) | - | 未找到 | - |
| 1030 | 更新面试官信息 | PATCH | /open-apis/hire/v1/interviewers/:interviewer_id | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/interviewer/patch) | - | 未找到 | - |
| 1031 | 删除三方协议 | DELETE | /open-apis/hire/v1/tripartite_agreements/:tripartite_agreement_id | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/tripartite_agreement/delete) | - | 未找到 | - |
| 1032 | 删除备注 | DELETE | /open-apis/hire/v1/notes/:note_id | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/note/delete) | - | 未找到 | - |
| 1033 | 删除外部 Offer | DELETE | /open-apis/hire/v1/external_offers/:external_offer_id | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/external_offer/delete) | - | 未找到 | - |
| 1034 | 删除外部内推奖励 | DELETE | /open-apis/hire/v1/external_referral_rewards/:external_referral_reward_id | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/external_referral_reward/delete) | - | 未找到 | - |
| 1035 | 删除外部投递 | DELETE | /open-apis/hire/v1/external_applications/:external_application_id | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/external_application/delete) | - | 未找到 | - |
| 1036 | 删除外部背调 | DELETE | /open-apis/hire/v1/external_background_checks/:external_background_check_id | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/external_background_check/delete) | - | 未找到 | - |
| 1037 | 删除外部面试 | DELETE | /open-apis/hire/v1/external_interviews/:external_interview_id | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/external_interview/delete) | - | 未找到 | - |
| 1038 | 删除招聘官网推广渠道 | DELETE | /open-apis/hire/v1/websites/:website_id/channels/:channel_id | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/website-channel/delete) | - | 未找到 | - |
| 1039 | 删除招聘需求 | DELETE | /open-apis/hire/v1/job_requirements/:job_requirement_id | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/job_requirement/delete) | - | 未找到 | - |

### 📦 human_authentication 模块

| 1040 | 录入身份信息 | POST | /open-apis/human_authentication/v1/identities | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/human_authentication-v1/identity/create) | - | 未找到 | - |

### 📦 im 模块

| 1041 | 下载图片 | GET | /open-apis/im/v1/images/:image_key | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/image/get) | - | 未找到 | - |
| 1042 | 下载文件 | GET | /open-apis/im/v1/files/:file_key | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/file/get) | - | 未找到 | - |
| 1043 | 判断用户或机器人是否在群里 | GET | /open-apis/im/v1/chats/:chat_id/members/is_in_chat | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-members/is_in_chat) | - | 未找到 | - |
| 1044 | 拉取会话标签页 | GET | /open-apis/im/v1/chats/:chat_id/chat_tabs/list_tabs | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-tab/list_tabs) | - | 未找到 | - |
| 1045 | 搜索对用户或机器人可见的群列表 | GET | /open-apis/im/v1/chats/search | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat/search) | - | 未找到 | - |
| 1046 | 查询实体与标签的绑定关系 | GET | /open-apis/im/v2/biz_entity_tag_relation | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/group/im-v2/biz_entity_tag_relation/get) | - | 未找到 | - |
| 1047 | 查询批量消息推送和阅读人数 | GET | /open-apis/im/v1/batch_messages/:batch_message_id/read_user | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/batch_message/read_user) | - | 未找到 | - |
| 1048 | 查询批量消息整体进度 | GET | /open-apis/im/v1/batch_messages/:batch_message_id/get_progress | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/batch_message/get_progress) | - | 未找到 | - |
| 1049 | 查询消息已读信息 | GET | /open-apis/im/v1/messages/:message_id/read_users | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/read_users) | - | 未找到 | - |
| 1050 | 获取会话历史消息 | GET | /open-apis/im/v1/messages | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/list) | - | 未找到 | - |
| 1051 | 获取指定消息的内容 | GET | /open-apis/im/v1/messages/:message_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/get) | - | 未找到 | - |
| 1052 | 获取消息中的资源文件 | GET | /open-apis/im/v1/messages/:message_id/resources/:file_key | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message-resource/get) | - | 未找到 | - |
| 1053 | 获取消息表情回复 | GET | /open-apis/im/v1/messages/:message_id/reactions | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message-reaction/list) | - | 未找到 | - |
| 1054 | 获取用户或机器人所在的群列表 | GET | /open-apis/im/v1/chats | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat/list) | - | 未找到 | - |
| 1055 | 获取群信息 | GET | /open-apis/im/v1/chats/:chat_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat/get) | - | 未找到 | - |
| 1056 | 获取群公告信息 | GET | /open-apis/im/v1/chats/:chat_id/announcement | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-announcement/get) | - | 未找到 | - |
| 1057 | 获取群内 Pin 消息 | GET | /open-apis/im/v1/pins | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/pin/list) | - | 未找到 | - |
| 1058 | 获取群成员列表 | GET | /open-apis/im/v1/chats/:chat_id/members | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-members/get) | - | 未找到 | - |
| 1059 | 获取群成员发言权限 | GET | /open-apis/im/v1/chats/:chat_id/moderation | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-moderation/get) | - | 未找到 | - |
| 1060 | 获取群菜单 | GET | /open-apis/im/v1/chats/:chat_id/menu_tree | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-menu_tree/get) | - | 未找到 | - |
| 1061 | Pin 消息 | POST | /open-apis/im/v1/pins | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/pin/create) | - | 未找到 | - |
| 1062 | 上传图片 | POST | /open-apis/im/v1/images | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/image/create) | - | 未找到 | - |
| 1063 | 上传文件 | POST | /open-apis/im/v1/files | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/file/create) | - | 未找到 | - |
| 1064 | 会话标签页排序 | POST | /open-apis/im/v1/chats/:chat_id/chat_tabs/sort_tabs | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-tab/sort_tabs) | - | 未找到 | - |
| 1065 | 创建应用消息流卡片 | POST | /open-apis/im/v2/app_feed_card | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/group/im-v2/app_feed_card/create) | - | 未找到 | - |
| 1066 | 创建标签 | POST | /open-apis/im/v2/tags | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/group/im-v2/tag/create) | - | 未找到 | - |
| 1067 | 创建群 | POST | /open-apis/im/v1/chats | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat/create) | - | 未找到 | - |
| 1068 | 删除群管理员 | POST | /open-apis/im/v1/chats/:chat_id/managers/delete_managers | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-managers/delete_managers) | - | 未找到 | - |
| 1069 | 发送消息 | POST | /open-apis/im/v1/messages | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/create) | - | 未找到 | - |
| 1070 | 合并转发消息 | POST | /open-apis/im/v1/messages/merge_forward | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/merge_forward) | - | 未找到 | - |
| 1071 | 回复消息 | POST | /open-apis/im/v1/messages/:message_id/reply | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/reply) | - | 未找到 | - |
| 1072 | 将用户或机器人拉入群聊 | POST | /open-apis/im/v1/chats/:chat_id/members | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-members/create) | - | 未找到 | - |
| 1073 | 指定群管理员 | POST | /open-apis/im/v1/chats/:chat_id/managers/add_managers | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-managers/add_managers) | - | 未找到 | - |
| 1074 | 排序群菜单 | POST | /open-apis/im/v1/chats/:chat_id/menu_tree/sort | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-menu_tree/sort) | - | 未找到 | - |
| 1075 | 撤销群置顶 | POST | /open-apis/im/v1/chats/:chat_id/top_notice/delete_top_notice | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-top_notice/delete_top_notice) | - | 未找到 | - |
| 1076 | 更新 URL 预览 | POST | /open-apis/im/v2/url_previews/batch_update | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/group/im-v2/url_preview/batch_update) | - | 未找到 | - |
| 1077 | 更新会话标签页 | POST | /open-apis/im/v1/chats/:chat_id/chat_tabs/update_tabs | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-tab/update_tabs) | - | 未找到 | - |
| 1078 | 更新群置顶 | POST | /open-apis/im/v1/chats/:chat_id/top_notice/put_top_notice | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-top_notice/put_top_notice) | - | 未找到 | - |
| 1079 | 添加会话标签页 | POST | /open-apis/im/v1/chats/:chat_id/chat_tabs | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-tab/create) | - | 未找到 | - |
| 1080 | 添加消息表情回复 | POST | /open-apis/im/v1/messages/:message_id/reactions | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message-reaction/create) | - | 未找到 | - |
| 1081 | 添加群菜单 | POST | /open-apis/im/v1/chats/:chat_id/menu_tree | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-menu_tree/create) | - | 未找到 | - |
| 1082 | 添加跟随气泡 | POST | /open-apis/im/v1/messages/:message_id/push_follow_up | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/push_follow_up) | - | 未找到 | - |
| 1083 | 绑定标签到群 | POST | /open-apis/im/v2/biz_entity_tag_relation | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/group/im-v2/biz_entity_tag_relation/create) | - | 未找到 | - |
| 1084 | 获取群分享链接 | POST | /open-apis/im/v1/chats/:chat_id/link | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat/link) | - | 未找到 | - |
| 1085 | 转发消息 | POST | /open-apis/im/v1/messages/:message_id/forward | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/forward) | - | 未找到 | - |
| 1086 | 转发话题 | POST | /open-apis/im/v1/threads/:thread_id/forward | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/thread/forward) | - | 未找到 | - |
| 1087 | 更新应用消息流卡片 | PUT | /open-apis/im/v2/app_feed_card/batch | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/group/im-v2/app_feed_card-batch/update) | - | 未找到 | - |
| 1088 | 更新消息流卡片按钮 | PUT | /open-apis/im/v2/chat_button | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/group/im-v2/chat_button/update) | - | 未找到 | - |
| 1089 | 更新群信息 | PUT | /open-apis/im/v1/chats/:chat_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat/update) | - | 未找到 | - |
| 1090 | 更新群发言权限 | PUT | /open-apis/im/v1/chats/:chat_id/moderation | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-moderation/update) | - | 未找到 | - |
| 1091 | 编辑消息 | PUT | /open-apis/im/v1/messages/:message_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/update) | - | 未找到 | - |
| 1092 | 解绑标签与群 | PUT | /open-apis/im/v2/biz_entity_tag_relation | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/group/im-v2/biz_entity_tag_relation/update) | - | 未找到 | - |
| 1093 | 修改标签 | PATCH | /open-apis/im/v2/tags/:tag_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/group/im-v2/tag/patch) | - | 未找到 | - |
| 1094 | 修改群菜单元信息 | PATCH | /open-apis/im/v1/chats/:chat_id/menu_items/:menu_item_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-menu_item/patch) | - | 未找到 | - |
| 1095 | 即时提醒 | PATCH | /open-apis/im/v2/feed_cards/:feed_card_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/group/im-v2/feed_card/patch) | - | 未找到 | - |
| 1096 | 发送应用内加急 | PATCH | /open-apis/im/v1/messages/:message_id/urgent_app | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/urgent_app) | - | 未找到 | - |
| 1097 | 发送电话加急 | PATCH | /open-apis/im/v1/messages/:message_id/urgent_phone | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/urgent_phone) | - | 未找到 | - |
| 1098 | 发送短信加急 | PATCH | /open-apis/im/v1/messages/:message_id/urgent_sms | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/urgent_sms) | - | 未找到 | - |
| 1099 | 更新已发送的消息卡片 | PATCH | /open-apis/im/v1/messages/:message_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/patch) | - | 未找到 | - |
| 1100 | 更新群公告信息 | PATCH | /open-apis/im/v1/chats/:chat_id/announcement | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-announcement/patch) | - | 未找到 | - |
| 1101 | 机器人单聊即时提醒 | PATCH | /open-apis/im/v2/feed_cards/bot_time_sentive | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/group/im-v2/feed_card/bot_time_sentive) | - | 未找到 | - |
| 1102 | 用户或机器人主动加入群聊 | PATCH | /open-apis/im/v1/chats/:chat_id/members/me_join | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-members/me_join) | - | 未找到 | - |
| 1103 | 删除会话标签页 | DELETE | /open-apis/im/v1/chats/:chat_id/chat_tabs/delete_tabs | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-tab/delete_tabs) | - | 未找到 | - |
| 1104 | 删除应用消息流卡片 | DELETE | /open-apis/im/v2/app_feed_card/batch | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/group/im-v2/app_feed_card-batch/delete) | - | 未找到 | - |
| 1105 | 删除消息表情回复 | DELETE | /open-apis/im/v1/messages/:message_id/reactions/:reaction_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message-reaction/delete) | - | 未找到 | - |
| 1106 | 删除群菜单 | DELETE | /open-apis/im/v1/chats/:chat_id/menu_tree | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-menu_tree/delete) | - | 未找到 | - |
| 1107 | 将用户或机器人移出群聊 | DELETE | /open-apis/im/v1/chats/:chat_id/members | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-members/delete) | - | 未找到 | - |
| 1108 | 批量撤回消息 | DELETE | /open-apis/im/v1/batch_messages/:batch_message_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/batch_message/delete) | - | 未找到 | - |
| 1109 | 撤回消息 | DELETE | /open-apis/im/v1/messages/:message_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/delete) | - | 未找到 | - |
| 1110 | 移除 Pin 消息 | DELETE | /open-apis/im/v1/pins/:message_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/pin/delete) | - | 未找到 | - |
| 1111 | 解散群 | DELETE | /open-apis/im/v1/chats/:chat_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat/delete) | - | 未找到 | - |

### 📦 interactive 模块

| 1112 | 延时更新消息卡片 | POST | /open-apis/interactive/v1/card/update | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMDO1YjLzgTN24yM4UjN) | - | 未找到 | - |

### 📦 lingo 模块

| 1113 | 下载图片 | GET | /open-apis/lingo/v1/files/:file_token/download | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/lingo-v1/file/download) | - | 未找到 | - |
| 1114 | 获取词典分类 | GET | /open-apis/lingo/v1/classifications | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/lingo-v1/classification/list) | - | 未找到 | - |
| 1115 | 获取词库列表 | GET | /open-apis/lingo/v1/repos | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/lingo-v1/repo/list) | - | 未找到 | - |
| 1116 | 获取词条列表 | GET | /open-apis/lingo/v1/entities | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/lingo-v1/entity/list) | - | 未找到 | - |
| 1117 | 获取词条详情 | GET | /open-apis/lingo/v1/entities/:entity_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/lingo-v1/entity/get) | - | 未找到 | - |
| 1118 | 上传图片 | POST | /open-apis/lingo/v1/files/upload | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/lingo-v1/file/upload) | - | 未找到 | - |
| 1119 | 创建免审词条 | POST | /open-apis/lingo/v1/entities | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/lingo-v1/entity/create) | - | 未找到 | - |
| 1120 | 创建草稿 | POST | /open-apis/lingo/v1/drafts | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/lingo-v1/draft/create) | - | 未找到 | - |
| 1121 | 模糊搜索词条 | POST | /open-apis/lingo/v1/entities/search | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/lingo-v1/entity/search) | - | 未找到 | - |
| 1122 | 精准搜索词条 | POST | /open-apis/lingo/v1/entities/match | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/lingo-v1/entity/match) | - | 未找到 | - |
| 1123 | 词条高亮 | POST | /open-apis/lingo/v1/entities/highlight | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/lingo-v1/entity/highlight) | - | 未找到 | - |
| 1124 | 更新免审词条 | PUT | /open-apis/lingo/v1/entities/:entity_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/lingo-v1/entity/update) | - | 未找到 | - |
| 1125 | 更新草稿 | PUT | /open-apis/lingo/v1/drafts/:draft_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/lingo-v1/draft/update) | - | 未找到 | - |
| 1126 | 删除免审词条 | DELETE | /open-apis/lingo/v1/entities/:entity_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/lingo-v1/entity/delete) | - | 未找到 | - |

### 📦 mail 模块

| 1127 | 列出收信规则 | GET | /open-apis/mail/v1/user_mailboxes/:user_mailbox_id/rules | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/user_mailbox-rule/list) | - | 未找到 | - |
| 1128 | 列出邮件 | GET | /open-apis/mail/v1/user_mailboxes/:user_mailbox_id/messages | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/user_mailbox-message/list) | - | 未找到 | - |
| 1129 | 列出邮箱文件夹 | GET | /open-apis/mail/v1/user_mailboxes/:user_mailbox_id/folders | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/user_mailbox-folder/list) | - | 未找到 | - |
| 1130 | 列出邮箱联系人 | GET | /open-apis/mail/v1/user_mailboxes/:user_mailbox_id/mail_contacts | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/user_mailbox-mail_contact/list) | - | 未找到 | - |
| 1131 | 批量获取邮件组 | GET | /open-apis/mail/v1/mailgroups | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/mailgroup/list) | - | 未找到 | - |
| 1132 | 批量获取邮件组权限成员 | GET | /open-apis/mail/v1/mailgroups/:mailgroup_id/permission_members | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/mailgroup-permission_member/list) | - | 未找到 | - |
| 1133 | 批量获取邮件组管理员 | GET | /open-apis/mail/v1/mailgroups/:mailgroup_id/managers | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/mailgroup-manager/list) | - | 未找到 | - |
| 1134 | 查询公共邮箱的所有别名 | GET | /open-apis/mail/v1/public_mailboxes/:public_mailbox_id/aliases | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/public_mailbox-alias/list) | - | 未找到 | - |
| 1135 | 查询所有公共邮箱 | GET | /open-apis/mail/v1/public_mailboxes | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/public_mailbox/list) | - | 未找到 | - |
| 1136 | 查询所有公共邮箱成员信息 | GET | /open-apis/mail/v1/public_mailboxes/:public_mailbox_id/members | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/public_mailbox-member/list) | - | 未找到 | - |
| 1137 | 查询指定公共邮箱 | GET | /open-apis/mail/v1/public_mailboxes/:public_mailbox_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/public_mailbox/get) | - | 未找到 | - |
| 1138 | 查询指定公共邮箱成员信息 | GET | /open-apis/mail/v1/public_mailboxes/:public_mailbox_id/members/:member_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/public_mailbox-member/get) | - | 未找到 | - |
| 1139 | 查询指定邮件组 | GET | /open-apis/mail/v1/mailgroups/:mailgroup_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/mailgroup/get) | - | 未找到 | - |
| 1140 | 查询指定邮件组成员 | GET | /open-apis/mail/v1/mailgroups/:mailgroup_id/members/:member_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/mailgroup-member/get) | - | 未找到 | - |
| 1141 | 获取所有邮件组成员 | GET | /open-apis/mail/v1/mailgroups/:mailgroup_id/members | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/mailgroup-member/list) | - | 未找到 | - |
| 1142 | 获取用户邮箱所有别名 | GET | /open-apis/mail/v1/user_mailboxes/:user_mailbox_id/aliases | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/user_mailbox-alias/list) | - | 未找到 | - |
| 1143 | 获取订阅状态 | GET | /open-apis/mail/v1/user_mailboxes/:user_mailbox_id/event/subscription | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/user_mailbox-event/subscription) | - | 未找到 | - |
| 1144 | 获取邮件卡片的邮件列表 | GET | /open-apis/mail/v1/user_mailboxes/:user_mailbox_id/messages/get_by_card | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/user_mailbox-message/get_by_card) | - | 未找到 | - |
| 1145 | 获取邮件组所有别名 | GET | /open-apis/mail/v1/mailgroups/:mailgroup_id/aliases | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/mailgroup-alias/list) | - | 未找到 | - |
| 1146 | 获取邮件组权限成员 | GET | /open-apis/mail/v1/mailgroups/:mailgroup_id/permission_members/:permission_member_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/mailgroup-permission_member/get) | - | 未找到 | - |
| 1147 | 获取邮件详情 | GET | /open-apis/mail/v1/user_mailboxes/:user_mailbox_id/messages/:message_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/user_mailbox-message/get) | - | 未找到 | - |
| 1148 | 获取附件下载链接 | GET | /open-apis/mail/v1/user_mailboxes/:user_mailbox_id/messages/:message_id/attachments/download_url | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/user_mailbox-message-attachment/download_url) | - | 未找到 | - |
| 1149 | 创建公共邮箱 | POST | /open-apis/mail/v1/public_mailboxes | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/public_mailbox/create) | - | 未找到 | - |
| 1150 | 创建公共邮箱别名 | POST | /open-apis/mail/v1/public_mailboxes/:public_mailbox_id/aliases | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/public_mailbox-alias/create) | - | 未找到 | - |
| 1151 | 创建收信规则 | POST | /open-apis/mail/v1/user_mailboxes/:user_mailbox_id/rules | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/user_mailbox-rule/create) | - | 未找到 | - |
| 1152 | 创建用户邮箱别名 | POST | /open-apis/mail/v1/user_mailboxes/:user_mailbox_id/aliases | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/user_mailbox-alias/create) | - | 未找到 | - |
| 1153 | 创建邮件组 | POST | /open-apis/mail/v1/mailgroups | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/mailgroup/create) | - | 未找到 | - |
| 1154 | 创建邮件组别名 | POST | /open-apis/mail/v1/mailgroups/:mailgroup_id/aliases | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/mailgroup-alias/create) | - | 未找到 | - |
| 1155 | 创建邮件组成员 | POST | /open-apis/mail/v1/mailgroups/:mailgroup_id/members | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/mailgroup-member/create) | - | 未找到 | - |
| 1156 | 创建邮件组权限成员 | POST | /open-apis/mail/v1/mailgroups/:mailgroup_id/permission_members | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/mailgroup-permission_member/create) | - | 未找到 | - |
| 1157 | 创建邮箱文件夹 | POST | /open-apis/mail/v1/user_mailboxes/:user_mailbox_id/folders | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/user_mailbox-folder/create) | - | 未找到 | - |
| 1158 | 创建邮箱联系人 | POST | /open-apis/mail/v1/user_mailboxes/:user_mailbox_id/mail_contacts | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/user_mailbox-mail_contact/create) | - | 未找到 | - |
| 1159 | 删除公共邮箱所有成员 | POST | /open-apis/mail/v1/public_mailboxes/:public_mailbox_id/members/clear | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/public_mailbox-member/clear) | - | 未找到 | - |
| 1160 | 发送邮件 | POST | /open-apis/mail/v1/user_mailboxes/:user_mailbox_id/messages/send | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/user_mailbox-message/send) | - | 未找到 | - |
| 1161 | 取消订阅 | POST | /open-apis/mail/v1/user_mailboxes/:user_mailbox_id/event/unsubscribe | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/user_mailbox-event/unsubscribe) | - | 未找到 | - |
| 1162 | 对收信规则进行排序 | POST | /open-apis/mail/v1/user_mailboxes/:user_mailbox_id/rules/reorder | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/user_mailbox-rule/reorder) | - | 未找到 | - |
| 1163 | 批量创建邮件组成员 | POST | /open-apis/mail/v1/mailgroups/:mailgroup_id/members/batch_create | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/mailgroup-member/batch_create) | - | 未找到 | - |
| 1164 | 批量创建邮件组权限成员 | POST | /open-apis/mail/v1/mailgroups/:mailgroup_id/permission_members/batch_create | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/mailgroup-permission_member/batch_create) | - | 未找到 | - |
| 1165 | 批量创建邮件组管理员 | POST | /open-apis/mail/v1/mailgroups/:mailgroup_id/managers/batch_create | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/mailgroup-manager/batch_create) | - | 未找到 | - |
| 1166 | 批量删除邮件组管理员 | POST | /open-apis/mail/v1/mailgroups/:mailgroup_id/managers/batch_delete | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/mailgroup-manager/batch_delete) | - | 未找到 | - |
| 1167 | 批量添加公共邮箱成员 | POST | /open-apis/mail/v1/public_mailboxes/:public_mailbox_id/members/batch_create | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/public_mailbox-member/batch_create) | - | 未找到 | - |
| 1168 | 查询邮箱地址状态 | POST | /open-apis/mail/v1/users/query | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/user/query) | - | 未找到 | - |
| 1169 | 添加公共邮箱成员 | POST | /open-apis/mail/v1/public_mailboxes/:public_mailbox_id/members | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/public_mailbox-member/create) | - | 未找到 | - |
| 1170 | 订阅事件 | POST | /open-apis/mail/v1/user_mailboxes/:user_mailbox_id/event/subscribe | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/user_mailbox-event/subscribe) | - | 未找到 | - |
| 1171 | 修改公共邮箱全部信息 | PUT | /open-apis/mail/v1/public_mailboxes/:public_mailbox_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/public_mailbox/update) | - | 未找到 | - |
| 1172 | 修改邮件组全部信息 | PUT | /open-apis/mail/v1/mailgroups/:mailgroup_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/mailgroup/update) | - | 未找到 | - |
| 1173 | 更新收信规则 | PUT | /open-apis/mail/v1/user_mailboxes/:user_mailbox_id/rules/:rule_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/user_mailbox-rule/update) | - | 未找到 | - |
| 1174 | 修改公共邮箱部分信息 | PATCH | /open-apis/mail/v1/public_mailboxes/:public_mailbox_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/public_mailbox/patch) | - | 未找到 | - |
| 1175 | 修改邮件组部分信息 | PATCH | /open-apis/mail/v1/mailgroups/:mailgroup_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/mailgroup/patch) | - | 未找到 | - |
| 1176 | 修改邮箱文件夹 | PATCH | /open-apis/mail/v1/user_mailboxes/:user_mailbox_id/folders/:folder_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/user_mailbox-folder/patch) | - | 未找到 | - |
| 1177 | 修改邮箱联系人信息 | PATCH | /open-apis/mail/v1/user_mailboxes/:user_mailbox_id/mail_contacts/:mail_contact_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/user_mailbox-mail_contact/patch) | - | 未找到 | - |
| 1178 | 从回收站删除用户邮箱地址 | DELETE | /open-apis/mail/v1/user_mailboxes/:user_mailbox_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/user_mailbox/delete) | - | 未找到 | - |
| 1179 | 删除公共邮箱别名 | DELETE | /open-apis/mail/v1/public_mailboxes/:public_mailbox_id/aliases/:alias_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/public_mailbox-alias/delete) | - | 未找到 | - |
| 1180 | 删除公共邮箱单个成员 | DELETE | /open-apis/mail/v1/public_mailboxes/:public_mailbox_id/members/:member_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/public_mailbox-member/delete) | - | 未找到 | - |
| 1181 | 删除收信规则 | DELETE | /open-apis/mail/v1/user_mailboxes/:user_mailbox_id/rules/:rule_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/user_mailbox-rule/delete) | - | 未找到 | - |
| 1182 | 删除用户邮箱别名 | DELETE | /open-apis/mail/v1/user_mailboxes/:user_mailbox_id/aliases/:alias_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/user_mailbox-alias/delete) | - | 未找到 | - |
| 1183 | 删除邮件组 | DELETE | /open-apis/mail/v1/mailgroups/:mailgroup_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/mailgroup/delete) | - | 未找到 | - |
| 1184 | 删除邮件组别名 | DELETE | /open-apis/mail/v1/mailgroups/:mailgroup_id/aliases/:alias_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/mailgroup-alias/delete) | - | 未找到 | - |
| 1185 | 删除邮件组成员 | DELETE | /open-apis/mail/v1/mailgroups/:mailgroup_id/members/:member_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/mailgroup-member/delete) | - | 未找到 | - |
| 1186 | 删除邮件组权限成员 | DELETE | /open-apis/mail/v1/mailgroups/:mailgroup_id/permission_members/:permission_member_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/mailgroup-permission_member/delete) | - | 未找到 | - |
| 1187 | 删除邮箱文件夹 | DELETE | /open-apis/mail/v1/user_mailboxes/:user_mailbox_id/folders/:folder_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/user_mailbox-folder/delete) | - | 未找到 | - |
| 1188 | 删除邮箱联系人 | DELETE | /open-apis/mail/v1/user_mailboxes/:user_mailbox_id/mail_contacts/:mail_contact_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/user_mailbox-mail_contact/delete) | - | 未找到 | - |
| 1189 | 将公共邮箱移至回收站 | DELETE | /open-apis/mail/v1/public_mailboxes/:public_mailbox_id/remove_to_recycle_bin | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/public_mailbox/remove_to_recycle_bin) | - | 未找到 | - |
| 1190 | 批量删除公共邮箱成员 | DELETE | /open-apis/mail/v1/public_mailboxes/:public_mailbox_id/members/batch_delete | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/public_mailbox-member/batch_delete) | - | 未找到 | - |
| 1191 | 批量删除邮件组成员 | DELETE | /open-apis/mail/v1/mailgroups/:mailgroup_id/members/batch_delete | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/mailgroup-member/batch_delete) | - | 未找到 | - |
| 1192 | 批量删除邮件组权限成员 | DELETE | /open-apis/mail/v1/mailgroups/:mailgroup_id/permission_members/batch_delete | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/mailgroup-permission_member/batch_delete) | - | 未找到 | - |
| 1193 | 永久删除公共邮箱 | DELETE | /open-apis/mail/v1/public_mailboxes/:public_mailbox_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/public_mailbox/delete) | - | 未找到 | - |

### 📦 mdm 模块

| 1194 | 分页批量查询国家/地区 | GET | /open-apis/mdm/v3/country_regions | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/mdm-v3/country_region/list) | - | 未找到 | - |
| 1195 | 根据主数据编码批量查询国家/地区 | GET | /open-apis/mdm/v3/batch_country_region | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/mdm-v3/batch_country_region/get) | - | 未找到 | - |
| 1196 | 用户数据维度绑定 | POST | /open-apis/mdm/v1/user_auth_data_relations/bind | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/mdm-v1/user_auth_data_relation/bind) | - | 未找到 | - |
| 1197 | 用户数据维度解绑 | POST | /open-apis/mdm/v1/user_auth_data_relations/unbind | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/mdm-v1/user_auth_data_relation/unbind) | - | 未找到 | - |

### 📦 meeting_room 模块

| 1198 | 查询会议室ID | GET | /open-apis/meeting_room/room/batch_get_id | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uYzMxYjL2MTM24iNzEjN) | - | 未找到 | - |
| 1199 | 查询会议室忙闲 | GET | /open-apis/meeting_room/freebusy/batch_get | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uIDOyUjLygjM14iM4ITN) | - | 未找到 | - |
| 1200 | 查询会议室详情 | GET | /open-apis/meeting_room/room/batch_get | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uEDOyUjLxgjM14SM4ITN) | - | 未找到 | - |
| 1201 | 查询建筑物ID | GET | /open-apis/meeting_room/building/batch_get_id | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uQzMxYjL0MTM24CNzEjN) | - | 未找到 | - |
| 1202 | 查询建筑物详情 | GET | /open-apis/meeting_room/building/batch_get | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/ukzNyUjL5cjM14SO3ITN) | - | 未找到 | - |
| 1203 | 获取会议室列表 | GET | /open-apis/meeting_room/room/list | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uADOyUjLwgjM14CM4ITN) | - | 未找到 | - |
| 1204 | 获取国家地区列表 | GET | /open-apis/meeting_room/country/list | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uQTNwYjL0UDM24CN1AjN) | - | 未找到 | - |
| 1205 | 获取城市列表 | GET | /open-apis/meeting_room/district/list | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uUTNwYjL1UDM24SN1AjN) | - | 未找到 | - |
| 1206 | 获取建筑物列表 | GET | /open-apis/meeting_room/building/list | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/ugzNyUjL4cjM14CO3ITN) | - | 未找到 | - |
| 1207 | 创建会议室 | POST | /open-apis/meeting_room/room/create | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uITNwYjLyUDM24iM1AjN) | - | 未找到 | - |
| 1208 | 创建建筑物 | POST | /open-apis/meeting_room/building/create | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uATNwYjLwUDM24CM1AjN) | - | 未找到 | - |
| 1209 | 删除会议室 | POST | /open-apis/meeting_room/room/delete | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uUzMxYjL1MTM24SNzEjN) | - | 未找到 | - |
| 1210 | 删除建筑物 | POST | /open-apis/meeting_room/building/delete | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMzMxYjLzMTM24yMzEjN) | - | 未找到 | - |
| 1211 | 回复会议室日程实例 | POST | /open-apis/meeting_room/instance/reply | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uYzN4UjL2cDO14iN3gTN) | - | 未找到 | - |
| 1212 | 更新会议室 | POST | /open-apis/meeting_room/room/update | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMTNwYjLzUDM24yM1AjN) | - | 未找到 | - |
| 1213 | 更新建筑物 | POST | /open-apis/meeting_room/building/update | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uETNwYjLxUDM24SM1AjN) | - | 未找到 | - |
| 1214 | 查询会议室日程主题和会议详情 | POST | /open-apis/meeting_room/summary/batch_get | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uIjM5UjLyITO14iMykTN/) | - | 未找到 | - |

### 📦 message 模块

| 1215 | 批量发送消息 | POST | /open-apis/message/v4/batch_send | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/ucDO1EjL3gTNx4yN4UTM) | - | 未找到 | - |

### 📦 minutes 模块

| 1216 | 下载妙记音视频文件 | GET | /open-apis/minutes/v1/minutes/:minute_token/media | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/minutes-v1/minute-media/get) | - | 未找到 | - |
| 1217 | 导出妙记文字记录 | GET | /open-apis/minutes/v1/minutes/:minute_token/transcript | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/minutes-v1/minute-transcript/get) | - | 未找到 | - |
| 1218 | 获取妙记信息 | GET | /open-apis/minutes/v1/minutes/:minute_token | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/minutes-v1/minute/get) | - | 未找到 | - |
| 1219 | 获取妙记统计数据 | GET | /open-apis/minutes/v1/minutes/:minute_token/statistics | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/minutes-v1/minute-statistics/get) | - | 未找到 | - |

### 📦 moments 模块

| 1220 | 查询帖子信息 | GET | /open-apis/moments/v1/posts/:post_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/moments-v1/post/get) | - | 未找到 | - |

### 📦 okr 模块

| 1221 | 批量获取 OKR | GET | /open-apis/okr/v1/okrs/batch_get | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/okr-v1/okr/batch_get) | - | 未找到 | - |
| 1222 | 查询复盘信息 | GET | /open-apis/okr/v1/reviews/query | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/okr-v1/review/query) | - | 未找到 | - |
| 1223 | 获取 OKR 周期列表 | GET | /open-apis/okr/v1/periods | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/okr-v1/period/list) | - | 未找到 | - |
| 1224 | 获取 OKR 周期规则 | GET | /open-apis/okr/v1/period_rules | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/okr-v1/period_rule/list) | - | 未找到 | - |
| 1225 | 获取 OKR 进展记录 | GET | /open-apis/okr/v1/progress_records/:progress_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/okr-v1/progress_record/get) | - | 未找到 | - |
| 1226 | 获取用户的 OKR 列表 | GET | /open-apis/okr/v1/users/:user_id/okrs | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/okr-v1/user-okr/list) | - | 未找到 | - |
| 1227 | 上传进展记录图片 | POST | /open-apis/okr/v1/images/upload | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/okr-v1/image/upload) | - | 未找到 | - |
| 1228 | 创建 OKR 周期 | POST | /open-apis/okr/v1/periods | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/okr-v1/period/create) | - | 未找到 | - |
| 1229 | 创建 OKR 进展记录 | POST | /open-apis/okr/v1/progress_records | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/okr-v1/progress_record/create) | - | 未找到 | - |
| 1230 | 更新 OKR 进展记录 | PUT | /open-apis/okr/v1/progress_records/:progress_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/okr-v1/progress_record/update) | - | 未找到 | - |
| 1231 | 修改 OKR 周期状态 | PATCH | /open-apis/okr/v1/periods/:period_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/okr-v1/period/patch) | - | 未找到 | - |
| 1232 | 删除 OKR 进展记录 | DELETE | /open-apis/okr/v1/progress_records/:progress_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/okr-v1/progress_record/delete) | - | 未找到 | - |

### 📦 optical_char_recognition 模块

| 1233 | 识别图片中的文字 | POST | /open-apis/optical_char_recognition/v1/image/basic_recognize | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/ai/optical_char_recognition-v1/image/basic_recognize) | - | 未找到 | - |

### 📦 passport 模块

| 1234 | 批量获取脱敏的用户登录信息 | POST | /open-apis/passport/v1/sessions/query | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/passport-v1/session/query) | - | 未找到 | - |
| 1235 | 退出登录 | POST | /open-apis/passport/v1/sessions/logout | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/passport-v1/session/logout) | - | 未找到 | - |

### 📦 pay 模块

| 1236 | 查询用户是否在应用开通范围 | GET | /open-apis/pay/v1/paid_scope/check_user | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uATNwUjLwUDM14CM1ATN) | - | 未找到 | - |
| 1237 | 查询租户购买的付费方案 | GET | /open-apis/pay/v1/order/list | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uETNwUjLxUDM14SM1ATN) | - | 未找到 | - |
| 1238 | 查询订单详情 | GET | /open-apis/pay/v1/order/get | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uITNwUjLyUDM14iM1ATN) | - | 未找到 | - |

### 📦 payroll 模块

| 1239 | 批量查询成本分摊方案 | GET | /open-apis/payroll/v1/cost_allocation_plans | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/payroll-v1/cost_allocation_plan/list) | - | 未找到 | - |
| 1240 | 批量查询算薪项 | GET | /open-apis/payroll/v1/acct_items | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/payroll-v1/acct_item/list) | - | 未找到 | - |
| 1241 | 查询发薪活动列表 | GET | /open-apis/payroll/v1/payment_activitys | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/payroll-v1/payment_activity/list) | - | 未找到 | - |
| 1242 | 查询发薪活动明细列表 | GET | /open-apis/payroll/v1/payment_activity_details | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/payroll-v1/payment_activity_detail/list) | - | 未找到 | - |
| 1243 | 查询成本分摊报表明细 | GET | /open-apis/payroll/v1/cost_allocation_details | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/payroll-v1/cost_allocation_detail/list) | - | 未找到 | - |
| 1244 | 查询成本分摊报表汇总数据 | GET | /open-apis/payroll/v1/cost_allocation_reports | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/payroll-v1/cost_allocation_report/list) | - | 未找到 | - |
| 1245 | 获取外部数据源配置信息 | GET | /open-apis/payroll/v1/datasources | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/payroll-v1/datasource/list) | - | 未找到 | - |
| 1246 | 获取薪资组基本信息 | GET | /open-apis/payroll/v1/paygroups | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/payroll-v1/paygroup/list) | - | 未找到 | - |
| 1247 | 创建 / 更新外部算薪数据 | POST | /open-apis/payroll/v1/datasource_records/save | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/payroll-v1/datasource_record/save) | - | 未找到 | - |
| 1248 | 封存发薪活动 | POST | /open-apis/payroll/v1/payment_activitys/archive | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/payroll-v1/payment_activity/archive) | - | 未找到 | - |
| 1249 | 批量查询发薪明细 | POST | /open-apis/payroll/v1/payment_detail/query | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/payroll-v1/payment_detail/query) | - | 未找到 | - |
| 1250 | 批量查询外部算薪数据记录 | POST | /open-apis/payroll/v1/datasource_records/query | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/payroll-v1/datasource_record/query) | - | 未找到 | - |

### 📦 performance 模块

| 1251 | 获取周期列表 | GET | /open-apis/performance/v1/semesters | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/performance-v1/semester/list) | - | 未找到 | - |
| 1252 | 获取指标标签列表 | GET | /open-apis/performance/v2/metric_tags | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/performance-v2/metric_tag/list) | - | 未找到 | - |
| 1253 | 录入被评估人关键指标数据 | POST | /open-apis/performance/v2/metric_details/import | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/performance-v2/metric_detail/import) | - | 未找到 | - |
| 1254 | 批量导入补充信息 | POST | /open-apis/performance/v2/additional_informations/import | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/performance-v2/additional_information/import) | - | 未找到 | - |
| 1255 | 批量查询补充信息 | POST | /open-apis/performance/v2/additional_informations/query | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/performance-v2/additional_information/query) | - | 未找到 | - |
| 1256 | 更新人员组成员 | POST | /open-apis/performance/v2/user_group_user_rels/write | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/performance-v2/user_group_user_rel/write) | - | 未找到 | - |
| 1257 | 获取周期任务（全部用户） | POST | /open-apis/performance/v1/stage_tasks/find_by_page | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/performance-v1/stage_task/find_by_page) | - | 未找到 | - |
| 1258 | 获取周期任务（指定用户） | POST | /open-apis/performance/v1/stage_tasks/find_by_user_list | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/performance-v1/stage_task/find_by_user_list) | - | 未找到 | - |
| 1259 | 获取指标列表 | POST | /open-apis/performance/v2/metric_libs/query | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/performance-v2/metric_lib/query) | - | 未找到 | - |
| 1260 | 获取指标字段列表 | POST | /open-apis/performance/v2/metric_fields/query | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/performance-v2/metric_field/query) | - | 未找到 | - |
| 1261 | 获取指标模板列表 | POST | /open-apis/performance/v2/metric_templates/query | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/performance-v2/metric_template/query) | - | 未找到 | - |
| 1262 | 获取标签填写题配置 | POST | /open-apis/performance/v2/questions/query | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/performance-v2/question/query) | - | 未找到 | - |
| 1263 | 获取绩效模板配置 | POST | /open-apis/performance/v2/review_templates/query | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/performance-v2/review_template/query) | - | 未找到 | - |
| 1264 | 获取绩效结果 | POST | /open-apis/performance/v1/review_datas/query | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/performance-v1/review_data/query) | - | 未找到 | - |
| 1265 | 获取绩效详情数据 | POST | /open-apis/performance/v2/review_datas/query | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/performance-v2/review_data/query) | - | 未找到 | - |
| 1266 | 获取被评估人信息 | POST | /open-apis/performance/v2/reviewees/query | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/performance-v2/reviewee/query) | - | 未找到 | - |
| 1267 | 获取被评估人关键指标结果 | POST | /open-apis/performance/v2/metric_details/query | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/performance-v2/metric_detail/query) | - | 未找到 | - |
| 1268 | 获取评估项列表 | POST | /open-apis/performance/v2/indicators/query | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/performance-v2/indicator/query) | - | 未找到 | - |
| 1269 | 获取项目列表 | POST | /open-apis/performance/v2/activity/query | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/performance-v2/activity/query) | - | 未找到 | - |
| 1270 | 批量删除补充信息 | DELETE | /open-apis/performance/v2/additional_informations/batch | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/performance-v2/additional_informations-batch/delete) | - | 未找到 | - |

### 📦 personal_settings 模块

| 1271 | 获取系统状态 | GET | /open-apis/personal_settings/v1/system_statuses | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/personal_settings-v1/system_status/list) | - | 未找到 | - |
| 1272 | 创建系统状态 | POST | /open-apis/personal_settings/v1/system_statuses | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/personal_settings-v1/system_status/create) | - | 未找到 | - |
| 1273 | 批量关闭系统状态 | POST | /open-apis/personal_settings/v1/system_statuses/:system_status_id/batch_close | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/personal_settings-v1/system_status/batch_close) | - | 未找到 | - |
| 1274 | 批量开启系统状态 | POST | /open-apis/personal_settings/v1/system_statuses/:system_status_id/batch_open | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/personal_settings-v1/system_status/batch_open) | - | 未找到 | - |
| 1275 | 修改系统状态 | PATCH | /open-apis/personal_settings/v1/system_statuses/:system_status_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/personal_settings-v1/system_status/patch) | - | 未找到 | - |
| 1276 | 删除系统状态 | DELETE | /open-apis/personal_settings/v1/system_statuses/:system_status_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/personal_settings-v1/system_status/delete) | - | 未找到 | - |

### 📦 report 模块

| 1277 | 查询规则 | GET | /open-apis/report/v1/rules/query | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/report/report-v1/rule/query) | - | 未找到 | - |
| 1278 | 查询任务 | POST | /open-apis/report/v1/tasks/query | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/report/report-v1/task/query) | - | 未找到 | - |
| 1279 | 移除规则看板 | POST | /open-apis/report/v1/rules/:rule_id/views/remove | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/report/report-v1/rule-view/remove) | - | 未找到 | - |

### 📦 search 模块

| 1280 | 批量获取数据源 | GET | /open-apis/search/v2/data_sources | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/search-v2/data_source/list) | - | 未找到 | - |
| 1281 | 搜索用户 | GET | /open-apis/search/v1/user | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMTM4UjLzEDO14yMxgTN) | - | 未找到 | - |
| 1282 | 查询指定数据项 | GET | /open-apis/search/v2/data_sources/:data_source_id/items/:item_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/search-v2/data_source-item/get) | - | 未找到 | - |
| 1283 | 获取数据源 | GET | /open-apis/search/v2/data_sources/:data_source_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/search-v2/data_source/get) | - | 未找到 | - |
| 1284 | 获取数据范式 | GET | /open-apis/search/v2/schemas/:schema_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/search-v2/schema/get) | - | 未找到 | - |
| 1285 | 为指定数据项创建索引 | POST | /open-apis/search/v2/data_sources/:data_source_id/items | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/search-v2/data_source-item/create) | - | 未找到 | - |
| 1286 | 创建数据源 | POST | /open-apis/search/v2/data_sources | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/search-v2/data_source/create) | - | 未找到 | - |
| 1287 | 创建数据范式 | POST | /open-apis/search/v2/schemas | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/search-v2/schema/create) | - | 未找到 | - |
| 1288 | 搜索应用 | POST | /open-apis/search/v2/app | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/search-v2/app/create) | - | 未找到 | - |
| 1289 | 搜索消息 | POST | /open-apis/search/v2/message | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/search-v2/message/create) | - | 未找到 | - |
| 1290 | 修改数据源 | PATCH | /open-apis/search/v2/data_sources/:data_source_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/search-v2/data_source/patch) | - | 未找到 | - |
| 1291 | 修改数据范式 | PATCH | /open-apis/search/v2/schemas/:schema_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/search-v2/schema/patch) | - | 未找到 | - |
| 1292 | 删除数据源 | DELETE | /open-apis/search/v2/data_sources/:data_source_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/search-v2/data_source/delete) | - | 未找到 | - |
| 1293 | 删除数据范式 | DELETE | /open-apis/search/v2/schemas/:schema_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/search-v2/schema/delete) | - | 未找到 | - |
| 1294 | 删除数据项 | DELETE | /open-apis/search/v2/data_sources/:data_source_id/items/:item_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/search-v2/data_source-item/delete) | - | 未找到 | - |

### 📦 security_and_compliance 模块

| 1295 | 查询设备信息 | GET | /open-apis/security_and_compliance/v2/device_records | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/security_and_compliance-v2/device_record/list) | - | 未找到 | - |
| 1296 | 获取客户端设备认证信息 | GET | /open-apis/security_and_compliance/v2/device_records/mine | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/security_and_compliance-v2/device_record/mine) | - | 未找到 | - |
| 1297 | 获取设备信息 | GET | /open-apis/security_and_compliance/v2/device_records/:device_record_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/security_and_compliance-v2/device_record/get) | - | 未找到 | - |
| 1298 | 新增设备 | POST | /open-apis/security_and_compliance/v2/device_records | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/security_and_compliance-v2/device_record/create) | - | 未找到 | - |
| 1299 | 获取OpenAPI审计日志数据 | POST | /open-apis/security_and_compliance/v1/openapi_logs/list_data | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/security_and_compliance-v1/openapi_log/list_data) | - | 未找到 | - |
| 1300 | 审批设备申报 | PUT | /open-apis/security_and_compliance/v2/device_apply_records/:device_apply_record_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/security_and_compliance-v2/device_apply_record/update) | - | 未找到 | - |
| 1301 | 更新设备 | PUT | /open-apis/security_and_compliance/v2/device_records/:device_record_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/security_and_compliance-v2/device_record/update) | - | 未找到 | - |
| 1302 | 删除设备 | DELETE | /open-apis/security_and_compliance/v2/device_records/:device_record_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/security_and_compliance-v2/device_record/delete) | - | 未找到 | - |

### 📦 sheets 模块

| 1303 | 批量获取条件格式 | GET | /open-apis/sheets/v2/spreadsheets/:spreadsheetToken/condition_formats | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/conditionformat/condition-format-get) | - | 未找到 | - |
| 1304 | 查询下拉列表设置 | GET | /open-apis/sheets/v2/spreadsheets/:spreadsheetToken/dataValidation | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/datavalidation/query-datavalidation) | - | 未找到 | - |
| 1305 | 查询导入结果 | GET | /open-apis/sheets/v2/import/result | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uETO2YjLxkjN24SM5YjN) | - | 未找到 | - |
| 1306 | 查询工作表 | GET | /open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet/get) | - | 未找到 | - |
| 1307 | 查询浮动图片 | GET | /open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/float_images/query | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-float_image/query) | - | 未找到 | - |
| 1308 | 查询筛选条件 | GET | /open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter_views/:filter_view_id/conditions/query | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter_view-condition/query) | - | 未找到 | - |
| 1309 | 查询筛选视图 | GET | /open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter_views/query | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter_view/query) | - | 未找到 | - |
| 1310 | 获取保护范围 | GET | /open-apis/sheets/v2/spreadsheets/:spreadsheetToken/protected_range_batch_get | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uQTM5YjL0ETO24CNxkjN) | - | 未找到 | - |
| 1311 | 获取工作表 | GET | /open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/query | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet/query) | - | 未找到 | - |
| 1312 | 获取浮动图片 | GET | /open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/float_images/:float_image_id | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-float_image/get) | - | 未找到 | - |
| 1313 | 获取电子表格信息 | GET | /open-apis/sheets/v3/spreadsheets/:spreadsheet_token | ✅ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet/get) | - | ../service/sheets/v3/spreadsheet_info.rs | 98 |
| 1314 | 获取筛选 | GET | /open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter/get) | - | 未找到 | - |
| 1315 | 获取筛选条件 | GET | /open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter_views/:filter_view_id/conditions/:condition_id | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter_view-condition/get) | - | 未找到 | - |
| 1316 | 获取筛选视图 | GET | /open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter_views/:filter_view_id | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter_view/get) | - | 未找到 | - |
| 1317 | 获取表格元数据 | GET | /open-apis/sheets/v2/spreadsheets/:spreadsheetToken/metainfo | ✅ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uETMzUjLxEzM14SMxMTN) | - | ../service/sheets/v2/metainfo.rs | 301 |
| 1318 | 读取单个范围 | GET | /open-apis/sheets/v2/spreadsheets/:spreadsheetToken/values/:range | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/ugTMzUjL4EzM14COxMTN) | - | 未找到 | - |
| 1319 | 读取多个范围 | GET | /open-apis/sheets/v2/spreadsheets/:spreadsheetToken/values_batch_get | ✅ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/ukTMzUjL5EzM14SOxMTN) | - | ../service/sheets/v2/batch_range_read.rs | 98 |
| 1320 | 修改保护范围 | POST | /open-apis/sheets/v2/spreadsheets/:spreadsheetToken/protected_range_batch_update | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uUTM5YjL1ETO24SNxkjN) | - | 未找到 | - |
| 1321 | 写入图片 | POST | /open-apis/sheets/v2/spreadsheets/:spreadsheetToken/values_image | ✅ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uUDNxYjL1QTM24SN0EjN) | - | ../service/sheets/v2/image_write_enhanced.rs | 529 |
| 1322 | 创建浮动图片 | POST | /open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/float_images | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-float_image/create) | - | 未找到 | - |
| 1323 | 创建电子表格 | POST | /open-apis/sheets/v3/spreadsheets | ✅ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet/create) | - | ../service/sheets/v3/spreadsheet_create.rs | 349 |
| 1324 | 创建筛选 | POST | /open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter/create) | - | 未找到 | - |
| 1325 | 创建筛选条件 | POST | /open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter_views/:filter_view_id/conditions | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter_view-condition/create) | - | 未找到 | - |
| 1326 | 创建筛选视图 | POST | /open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter_views | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter_view/create) | - | 未找到 | - |
| 1327 | 合并单元格 | POST | /open-apis/sheets/v2/spreadsheets/:spreadsheetToken/merge_cells | ✅ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/ukDNzUjL5QzM14SO0MTN) | - | ../service/sheets/v2/merge_cells.rs | 319 |
| 1328 | 向多个范围写入数据 | POST | /open-apis/sheets/v2/spreadsheets/:spreadsheetToken/values_batch_update | ✅ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uEjMzUjLxIzM14SMyMTN) | - | ../service/sheets/v2/values_batch_write.rs | 398 |
| 1329 | 增加保护范围 | POST | /open-apis/sheets/v2/spreadsheets/:spreadsheetToken/protected_dimension | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/ugDNzUjL4QzM14CO0MTN) | - | 未找到 | - |
| 1330 | 增加行列 | POST | /open-apis/sheets/v2/spreadsheets/:spreadsheetToken/dimension_range | ✅ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uUjMzUjL1IzM14SNyMTN) | - | ../service/sheets/v2/dimension_operations.rs | 687 |
| 1331 | 导入表格 | POST | /open-apis/sheets/v2/import | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uATO2YjLwkjN24CM5YjN) | - | 未找到 | - |
| 1332 | 批量创建条件格式 | POST | /open-apis/sheets/v2/spreadsheets/:spreadsheetToken/condition_formats/batch_create | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/conditionformat/condition-format-set) | - | 未找到 | - |
| 1333 | 批量更新条件格式 | POST | /open-apis/sheets/v2/spreadsheets/:spreadsheetToken/condition_formats/batch_update | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/conditionformat/condition-format-update) | - | 未找到 | - |
| 1334 | 拆分单元格 | POST | /open-apis/sheets/v2/spreadsheets/:spreadsheetToken/unmerge_cells | ✅ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uATNzUjLwUzM14CM1MTN) | - | ../service/sheets/v2/merge_cells.rs | 370 |
| 1335 | 插入数据 | POST | /open-apis/sheets/v2/spreadsheets/:spreadsheetToken/values_prepend | ✅ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uIjMzUjLyIzM14iMyMTN) | - | ../service/sheets/v2/values_prepend.rs | 455 |
| 1336 | 插入行列 | POST | /open-apis/sheets/v2/spreadsheets/:spreadsheetToken/insert_dimension_range | ✅ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uQjMzUjL0IzM14CNyMTN) | - | ../service/sheets/v2/dimension_operations.rs | 573 |
| 1337 | 更新工作表属性 | POST | /open-apis/sheets/v2/spreadsheets/:spreadsheetToken/sheets_batch_update | ✅ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/ugjMzUjL4IzM14COyMTN) | - | ../service/sheets/v2/sheets_batch_update.rs | 420 |
| 1338 | 替换单元格 | POST | /open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/replace | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet/replace) | - | 未找到 | - |
| 1339 | 查找单元格 | POST | /open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/find | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet/find) | - | 未找到 | - |
| 1340 | 移动行列 | POST | /open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/move_dimension | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet/move_dimension) | - | 未找到 | - |
| 1341 | 设置下拉列表 | POST | /open-apis/sheets/v2/spreadsheets/:spreadsheetToken/dataValidation | ✅ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/datavalidation/set-dropdown) | - | ../service/sheets/v2/data_validation.rs | 600 |
| 1342 | 追加数据 | POST | /open-apis/sheets/v2/spreadsheets/:spreadsheetToken/values_append | ✅ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMjMzUjLzIzM14yMyMTN) | - | ../service/sheets/v2/values_append.rs | 455 |
| 1343 | 向单个范围写入数据 | PUT | /open-apis/sheets/v2/spreadsheets/:spreadsheetToken/values | ✅ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uAjMzUjLwIzM14CMyMTN) | - | ../service/sheets/v2/values_single_write.rs | 337 |
| 1344 | 批量设置单元格样式 | PUT | /open-apis/sheets/v2/spreadsheets/:spreadsheetToken/styles_batch_update | ✅ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uAzMzUjLwMzM14CMzMTN) | - | ../service/sheets/v2/style_operations.rs | 552 |
| 1345 | 更新下拉列表设置 | PUT | /open-apis/sheets/v2/spreadsheets/:spreadsheetToken/dataValidation/:sheetId/:dataValidationId | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/datavalidation/update-datavalidation) | - | 未找到 | - |
| 1346 | 更新筛选 | PUT | /open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter/update) | - | 未找到 | - |
| 1347 | 更新筛选条件 | PUT | /open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter_views/:filter_view_id/conditions/:condition_id | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter_view-condition/update) | - | 未找到 | - |
| 1348 | 更新行列 | PUT | /open-apis/sheets/v2/spreadsheets/:spreadsheetToken/dimension_range | ✅ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uYjMzUjL2IzM14iNyMTN) | - | ../service/sheets/v2/dimension_operations.rs | 747 |
| 1349 | 更新表格属性 | PUT | /open-apis/sheets/v2/spreadsheets/:spreadsheetToken/properties | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/ucTMzUjL3EzM14yNxMTN) | - | 未找到 | - |
| 1350 | 设置单元格样式 | PUT | /open-apis/sheets/v2/spreadsheets/:spreadsheetToken/style | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/ukjMzUjL5IzM14SOyMTN) | - | 未找到 | - |
| 1351 | 修改电子表格属性 | PATCH | /open-apis/sheets/v3/spreadsheets/:spreadsheet_token | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet/patch) | - | 未找到 | - |
| 1352 | 更新浮动图片 | PATCH | /open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/float_images/:float_image_id | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-float_image/patch) | - | 未找到 | - |
| 1353 | 更新筛选视图 | PATCH | /open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter_views/:filter_view_id | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter_view/patch) | - | 未找到 | - |
| 1354 | 删除下拉列表设置 | DELETE | /open-apis/sheets/v2/spreadsheets/:spreadsheetToken/dataValidation | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/datavalidation/delete-datavalidation) | - | 未找到 | - |
| 1355 | 删除保护范围 | DELETE | /open-apis/sheets/v2/spreadsheets/:spreadsheetToken/protected_range_batch_del | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uYTM5YjL2ETO24iNxkjN) | - | 未找到 | - |
| 1356 | 删除浮动图片 | DELETE | /open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/float_images/:float_image_id | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-float_image/delete) | - | 未找到 | - |
| 1357 | 删除筛选 | DELETE | /open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter/delete) | - | 未找到 | - |
| 1358 | 删除筛选条件 | DELETE | /open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter_views/:filter_view_id/conditions/:condition_id | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter_view-condition/delete) | - | 未找到 | - |
| 1359 | 删除筛选视图 | DELETE | /open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter_views/:filter_view_id | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-filter_view/delete) | - | 未找到 | - |
| 1360 | 删除行列 | DELETE | /open-apis/sheets/v2/spreadsheets/:spreadsheetToken/dimension_range | ✅ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/ucjMzUjL3IzM14yNyMTN) | - | ../service/sheets/v2/dimension_operations.rs | 627 |
| 1361 | 批量删除条件格式 | DELETE | /open-apis/sheets/v2/spreadsheets/:spreadsheetToken/condition_formats/batch_delete | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/conditionformat/condition-format-delete) | - | 未找到 | - |

### 📦 speech_to_text 模块

| 1362 | 识别流式语音 | POST | /open-apis/speech_to_text/v1/speech/stream_recognize | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/ai/speech_to_text-v1/speech/stream_recognize) | - | 未找到 | - |
| 1363 | 识别语音文件 | POST | /open-apis/speech_to_text/v1/speech/file_recognize | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/ai/speech_to_text-v1/speech/file_recognize) | - | 未找到 | - |

### 📦 suite 模块

| 1364 | 搜索云文档 | POST | /open-apis/suite/docs-api/search/object | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/ugDM4UjL4ADO14COwgTN) | - | 未找到 | - |
| 1365 | 获取元数据 | POST | /open-apis/suite/docs-api/meta | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMjN3UjLzYzN14yM2cTN) | - | 未找到 | - |

### 📦 task 模块

| 1366 | 列取任务列表 | GET | /open-apis/task/v2/tasks | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/task-v2/task/list) | - | 未找到 | - |
| 1367 | 列取任务所在清单 | GET | /open-apis/task/v2/tasks/:task_guid/tasklists | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/task-v2/task/tasklists) | - | 未找到 | - |
| 1368 | 列取动态订阅 | GET | /open-apis/task/v2/tasklists/:tasklist_guid/activity_subscriptions | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/task-v2/tasklist-activity_subscription/list) | - | 未找到 | - |
| 1369 | 列取自定义字段 | GET | /open-apis/task/v2/custom_fields | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/task-v2/custom_field/list) | - | 未找到 | - |
| 1370 | 列取附件 | GET | /open-apis/task/v2/attachments | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/task-v2/attachment/list) | - | 未找到 | - |
| 1371 | 查询所有任务 | GET | /open-apis/task/v1/tasks | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/task-v1/task/list) | - | 未找到 | - |
| 1372 | 查询指定任务 | GET | /open-apis/task/v1/tasks/:task_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/task-v1/task/get) | - | 未找到 | - |
| 1373 | 查询提醒时间列表 | GET | /open-apis/task/v1/tasks/:task_id/reminders | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/task-v1/task-reminder/list) | - | 未找到 | - |
| 1374 | 获取任务的子任务列表 | GET | /open-apis/task/v2/tasks/:task_guid/subtasks | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/task-v2/task-subtask/list) | - | 未找到 | - |
| 1375 | 获取任务详情 | GET | /open-apis/task/v2/tasks/:task_guid | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/task-v2/task/get) | - | 未找到 | - |
| 1376 | 获取关注人列表 | GET | /open-apis/task/v1/tasks/:task_id/followers | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/task-v1/task-follower/list) | - | 未找到 | - |
| 1377 | 获取动态订阅 | GET | /open-apis/task/v2/tasklists/:tasklist_guid/activity_subscriptions/:activity_subscription_guid | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/task-v2/tasklist-activity_subscription/get) | - | 未找到 | - |
| 1378 | 获取执行者列表 | GET | /open-apis/task/v1/tasks/:task_id/collaborators | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/task-v1/task-collaborator/list) | - | 未找到 | - |
| 1379 | 获取清单任务列表 | GET | /open-apis/task/v2/tasklists/:tasklist_guid/tasks | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/task-v2/tasklist/tasks) | - | 未找到 | - |
| 1380 | 获取清单列表 | GET | /open-apis/task/v2/tasklists | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/task-v2/tasklist/list) | - | 未找到 | - |
| 1381 | 获取清单详情 | GET | /open-apis/task/v2/tasklists/:tasklist_guid | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/task-v2/tasklist/get) | - | 未找到 | - |
| 1382 | 获取自定义分组任务列表 | GET | /open-apis/task/v2/sections/:section_guid/tasks | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/task-v2/section/tasks) | - | 未找到 | - |
| 1383 | 获取自定义分组列表 | GET | /open-apis/task/v2/sections | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/task-v2/section/list) | - | 未找到 | - |
| 1384 | 获取自定义分组详情 | GET | /open-apis/task/v2/sections/:section_guid | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/task-v2/section/get) | - | 未找到 | - |
| 1385 | 获取自定义字段 | GET | /open-apis/task/v2/custom_fields/:custom_field_guid | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/task-v2/custom_field/get) | - | 未找到 | - |
| 1386 | 获取评论列表 | GET | /open-apis/task/v1/tasks/:task_id/comments | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/task-v1/task-comment/list) | - | 未找到 | - |
| 1387 | 获取评论列表 | GET | /open-apis/task/v2/comments | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/task-v2/comment/list) | - | 未找到 | - |
| 1388 | 获取评论详情 | GET | /open-apis/task/v2/comments/:comment_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/task-v2/comment/get) | - | 未找到 | - |
| 1389 | 获取评论详情 | GET | /open-apis/task/v1/tasks/:task_id/comments/:comment_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/task-v1/task-comment/get) | - | 未找到 | - |
| 1390 | 获取附件 | GET | /open-apis/task/v2/attachments/:attachment_guid | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/task-v2/attachment/get) | - | 未找到 | - |
| 1391 | 上传附件 | POST | /open-apis/task/v2/attachments/upload | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/task-v2/attachment/upload) | - | 未找到 | - |
| 1392 | 任务加入清单 | POST | /open-apis/task/v2/tasks/:task_guid/add_tasklist | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/task-v2/task/add_tasklist) | - | 未找到 | - |
| 1393 | 任务移出清单 | POST | /open-apis/task/v2/tasks/:task_guid/remove_tasklist | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/task-v2/task/remove_tasklist) | - | 未找到 | - |
| 1394 | 创建任务 | POST | /open-apis/task/v1/tasks | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/task-v1/task/create) | - | 未找到 | - |
| 1395 | 创建任务 | POST | /open-apis/task/v2/tasks | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/task-v2/task/create) | - | 未找到 | - |
| 1396 | 创建动态订阅 | POST | /open-apis/task/v2/tasklists/:tasklist_guid/activity_subscriptions | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/task-v2/tasklist-activity_subscription/create) | - | 未找到 | - |
| 1397 | 创建子任务 | POST | /open-apis/task/v2/tasks/:task_guid/subtasks | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/task-v2/task-subtask/create) | - | 未找到 | - |
| 1398 | 创建清单 | POST | /open-apis/task/v2/tasklists | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/task-v2/tasklist/create) | - | 未找到 | - |
| 1399 | 创建自定义任务选项 | POST | /open-apis/task/v2/custom_fields/:custom_field_guid/options | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/task-v2/custom_field-option/create) | - | 未找到 | - |
| 1400 | 创建自定义分组 | POST | /open-apis/task/v2/sections | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/task-v2/section/create) | - | 未找到 | - |
| 1401 | 创建自定义字段 | POST | /open-apis/task/v2/custom_fields | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/task-v2/custom_field/create) | - | 未找到 | - |
| 1402 | 创建评论 | POST | /open-apis/task/v1/tasks/:task_id/comments | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/task-v1/task-comment/create) | - | 未找到 | - |
| 1403 | 创建评论 | POST | /open-apis/task/v2/comments | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/task-v2/comment/create) | - | 未找到 | - |
| 1404 | 取消完成任务 | POST | /open-apis/task/v1/tasks/:task_id/uncomplete | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/task-v1/task/uncomplete) | - | 未找到 | - |
| 1405 | 完成任务 | POST | /open-apis/task/v1/tasks/:task_id/complete | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/task-v1/task/complete) | - | 未找到 | - |
| 1406 | 将自定义字段加入资源 | POST | /open-apis/task/v2/custom_fields/:custom_field_guid/add | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/task-v2/custom_field/add) | - | 未找到 | - |
| 1407 | 将自定义字段移出资源 | POST | /open-apis/task/v2/custom_fields/:custom_field_guid/remove | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/task-v2/custom_field/remove) | - | 未找到 | - |
| 1408 | 批量删除关注人 | POST | /open-apis/task/v1/tasks/:task_id/batch_delete_follower | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/task-v1/task/batch_delete_follower) | - | 未找到 | - |
| 1409 | 批量删除执行者 | POST | /open-apis/task/v1/tasks/:task_id/batch_delete_collaborator | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/task-v1/task/batch_delete_collaborator) | - | 未找到 | - |
| 1410 | 新增关注人 | POST | /open-apis/task/v1/tasks/:task_id/followers | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/task-v1/task-follower/create) | - | 未找到 | - |
| 1411 | 新增执行者 | POST | /open-apis/task/v1/tasks/:task_id/collaborators | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/task-v1/task-collaborator/create) | - | 未找到 | - |
| 1412 | 新增提醒时间 | POST | /open-apis/task/v1/tasks/:task_id/reminders | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/task-v1/task-reminder/create) | - | 未找到 | - |
| 1413 | 添加任务成员 | POST | /open-apis/task/v2/tasks/:task_guid/add_members | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/task-v2/task/add_members) | - | 未找到 | - |
| 1414 | 添加任务提醒 | POST | /open-apis/task/v2/tasks/:task_guid/add_reminders | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/task-v2/task/add_reminders) | - | 未找到 | - |
| 1415 | 添加依赖 | POST | /open-apis/task/v2/tasks/:task_guid/add_dependencies | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/task-v2/task/add_dependencies) | - | 未找到 | - |
| 1416 | 添加清单成员 | POST | /open-apis/task/v2/tasklists/:tasklist_guid/add_members | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/task-v2/tasklist/add_members) | - | 未找到 | - |
| 1417 | 移除任务成员 | POST | /open-apis/task/v2/tasks/:task_guid/remove_members | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/task-v2/task/remove_members) | - | 未找到 | - |
| 1418 | 移除任务提醒 | POST | /open-apis/task/v2/tasks/:task_guid/remove_reminders | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/task-v2/task/remove_reminders) | - | 未找到 | - |
| 1419 | 移除依赖 | POST | /open-apis/task/v2/tasks/:task_guid/remove_dependencies | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/task-v2/task/remove_dependencies) | - | 未找到 | - |
| 1420 | 移除清单成员 | POST | /open-apis/task/v2/tasklists/:tasklist_guid/remove_members | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/task-v2/tasklist/remove_members) | - | 未找到 | - |
| 1421 | 更新评论 | PUT | /open-apis/task/v1/tasks/:task_id/comments/:comment_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/task-v1/task-comment/update) | - | 未找到 | - |
| 1422 | 更新任务 | PATCH | /open-apis/task/v1/tasks/:task_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/task-v1/task/patch) | - | 未找到 | - |
| 1423 | 更新任务 | PATCH | /open-apis/task/v2/tasks/:task_guid | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/task-v2/task/patch) | - | 未找到 | - |
| 1424 | 更新动态订阅 | PATCH | /open-apis/task/v2/tasklists/:tasklist_guid/activity_subscriptions/:activity_subscription_guid | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/task-v2/tasklist-activity_subscription/patch) | - | 未找到 | - |
| 1425 | 更新清单 | PATCH | /open-apis/task/v2/tasklists/:tasklist_guid | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/task-v2/tasklist/patch) | - | 未找到 | - |
| 1426 | 更新自定义分组 | PATCH | /open-apis/task/v2/sections/:section_guid | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/task-v2/section/patch) | - | 未找到 | - |
| 1427 | 更新自定义字段 | PATCH | /open-apis/task/v2/custom_fields/:custom_field_guid | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/task-v2/custom_field/patch) | - | 未找到 | - |
| 1428 | 更新自定义字段选项 | PATCH | /open-apis/task/v2/custom_fields/:custom_field_guid/options/:option_guid | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/task-v2/custom_field-option/patch) | - | 未找到 | - |
| 1429 | 更新评论 | PATCH | /open-apis/task/v2/comments/:comment_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/task-v2/comment/patch) | - | 未找到 | - |
| 1430 | 删除任务 | DELETE | /open-apis/task/v2/tasks/:task_guid | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/task-v2/task/delete) | - | 未找到 | - |
| 1431 | 删除任务 | DELETE | /open-apis/task/v1/tasks/:task_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/task-v1/task/delete) | - | 未找到 | - |
| 1432 | 删除动态订阅 | DELETE | /open-apis/task/v2/tasklists/:tasklist_guid/activity_subscriptions/:activity_subscription_guid | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/task-v2/tasklist-activity_subscription/delete) | - | 未找到 | - |
| 1433 | 删除指定关注人 | DELETE | /open-apis/task/v1/tasks/:task_id/followers/:follower_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/task-v1/task-follower/delete) | - | 未找到 | - |
| 1434 | 删除指定执行者 | DELETE | /open-apis/task/v1/tasks/:task_id/collaborators/:collaborator_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/task-v1/task-collaborator/delete) | - | 未找到 | - |
| 1435 | 删除提醒时间 | DELETE | /open-apis/task/v1/tasks/:task_id/reminders/:reminder_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/task-v1/task-reminder/delete) | - | 未找到 | - |
| 1436 | 删除清单 | DELETE | /open-apis/task/v2/tasklists/:tasklist_guid | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/task-v2/tasklist/delete) | - | 未找到 | - |
| 1437 | 删除自定义分组 | DELETE | /open-apis/task/v2/sections/:section_guid | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/task-v2/section/delete) | - | 未找到 | - |
| 1438 | 删除评论 | DELETE | /open-apis/task/v2/comments/:comment_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/task-v2/comment/delete) | - | 未找到 | - |
| 1439 | 删除评论 | DELETE | /open-apis/task/v1/tasks/:task_id/comments/:comment_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/task-v1/task-comment/delete) | - | 未找到 | - |
| 1440 | 删除附件 | DELETE | /open-apis/task/v2/attachments/:attachment_guid | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/task-v2/attachment/delete) | - | 未找到 | - |

### 📦 tenant 模块

| 1441 | 获取企业信息 | GET | /open-apis/tenant/v2/tenant/query | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/tenant-v2/tenant/query) | - | 未找到 | - |
| 1442 | 获取企业席位信息接口 | GET | /open-apis/tenant/v2/tenant/assign_info_list/query | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/tenant-v2/tenant-product_assign_info/query) | - | 未找到 | - |

### 📦 translation 模块

| 1443 | 翻译文本 | POST | /open-apis/translation/v1/text/translate | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/ai/translation-v1/text/translate) | - | 未找到 | - |
| 1444 | 识别文本语种 | POST | /open-apis/translation/v1/text/detect | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/ai/translation-v1/text/detect) | - | 未找到 | - |

### 📦 trust_party 模块

| 1445 | 获取关联组织成员详情 | GET | /open-apis/trust_party/v1/collaboration_tenants/:target_tenant_key/collaboration_users/:target_user_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/trust_party-v1/collaboration_tenant-collaboration_user/get) | - | 未找到 | - |
| 1446 | 获取关联组织的部门和成员信息 | GET | /open-apis/trust_party/v1/collaboration_tenants/visible_organization | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/trust_party-v1/collaboration_tenant/visible_organization) | - | 未找到 | - |
| 1447 | 获取关联组织详情 | GET | /open-apis/trust_party/v1/collaboration_tenants/:target_tenant_key | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/trust_party-v1/collaboration_tenant/get) | - | 未找到 | - |
| 1448 | 获取关联组织部门详情 | GET | /open-apis/trust_party/v1/collaboration_tenants/:target_tenant_key/collaboration_departments/:target_department_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/trust_party-v1/collaboration_tenant-collaboration_department/get) | - | 未找到 | - |
| 1449 | 获取可见关联组织的列表 | GET | /open-apis/trust_party/v1/collaboration_tenants | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/trust_party-v1/collaboration_tenant/list) | - | 未找到 | - |

### 📦 unknown 模块

| 1450 | 三方审批定义创建 | POST | /approval/openapi/v3/external/approval/create | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uIDNyYjLyQjM24iM0IjN) | - | 未找到 | - |
| 1451 | 三方审批实例同步 | POST | /approval/openapi/v2/external/instance/create | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uczM3UjL3MzN14yNzcTN) | - | 未找到 | - |
| 1452 | 三方审批实例校验 | POST | /approval/openapi/v3/external/instance/check | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uUDNyYjL1QjM24SN0IjN) | - | 未找到 | - |
| 1453 | 三方快捷审批回调 | POST | /approval/openapi/v2/external/instanceOperate | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/ukjNyYjL5YjM24SO2IjN/quick-approval-callback) | - | 未找到 | - |
| 1454 | 上传文件 | POST | /approval/openapi/v2/file/upload | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uUDOyUjL1gjM14SN4ITN) | - | 未找到 | - |
| 1455 | 任务列表查询 | POST | /approval/openapi/v2/task/search | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uYjMxYjL2ITM24iNyEjN) | - | 未找到 | - |
| 1456 | 创建审批定义 | POST | /approval/openapi/v2/approval/create | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uUzNyYjL1cjM24SN3IjN) | - | 未找到 | - |
| 1457 | 创建审批实例 | POST | /approval/openapi/v2/instance/create | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uIDNyUjLyQjM14iM0ITN) | - | 未找到 | - |
| 1458 | 发送审批 Bot 消息 | POST | /approval/openapi/v1/message/send | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/ugDNyYjL4QjM24CO0IjN) | - | 未找到 | - |
| 1459 | 取消订阅审批事件 | POST | /approval/openapi/v2/subscription/unsubscribe | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/ugDOyUjL4gjM14CO4ITN) | - | 未找到 | - |
| 1460 | 实例列表查询 | POST | /approval/openapi/v2/instance/search | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uQjMxYjL0ITM24CNyEjN) | - | 未找到 | - |
| 1461 | 审批任务同意 | POST | /approval/openapi/v2/instance/approve | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uMDNyUjLzQjM14yM0ITN) | - | 未找到 | - |
| 1462 | 审批任务拒绝 | POST | /approval/openapi/v2/instance/reject | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uQDNyUjL0QjM14CN0ITN) | - | 未找到 | - |
| 1463 | 审批任务转交 | POST | /approval/openapi/v2/instance/transfer | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uUDNyUjL1QjM14SN0ITN) | - | 未找到 | - |
| 1464 | 审批实例抄送 | POST | /approval/openapi/v2/instance/cc | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uADOzYjLwgzM24CM4MjN) | - | 未找到 | - |
| 1465 | 审批实例撤回 | POST | /approval/openapi/v2/instance/cancel | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uYDNyUjL2QjM14iN0ITN) | - | 未找到 | - |
| 1466 | 批量获取审批实例ID | POST | /approval/openapi/v2/instance/list | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uQDOyUjL0gjM14CN4ITN) | - | 未找到 | - |
| 1467 | 抄送列表查询 | POST | /approval/openapi/v2/cc/search | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uUjMxYjL1ITM24SNyEjN) | - | 未找到 | - |
| 1468 | 更新审批 Bot 消息 | POST | /approval/openapi/v1/message/update | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uAjNyYjLwYjM24CM2IjN) | - | 未找到 | - |
| 1469 | 查看审批定义 | POST | /approval/openapi/v2/approval/get | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uADNyUjLwQjM14CM0ITN) | - | 未找到 | - |
| 1470 | 查询审批 ID（专用） | POST | /approval/openapi/v1/id/get | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uEDN5UjLxQTO14SM0kTN) | - | 未找到 | - |
| 1471 | 获取三方审批任务状态 | POST | /approval/openapi/v2/external/list | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/ukjNyYjL5YjM24SO2IjN/external_status) | - | 未找到 | - |
| 1472 | 获取单个审批实例详情 | POST | /approval/openapi/v2/instance/get | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uEDNyUjLxQjM14SM0ITN) | - | 未找到 | - |
| 1473 | 订阅审批事件 | POST | /approval/openapi/v2/subscription/subscribe | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/ucDOyUjL3gjM14yN4ITN) | - | 未找到 | - |

### 📦 user 模块

| 1474 | 查询应用管理员列表 | GET | /open-apis/user/v4/app_admin_user/list | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/ucDOwYjL3gDM24yN4AjN) | - | 未找到 | - |

### 📦 vc 模块

| 1475 | 下载导出文件 | GET | /open-apis/vc/v1/exports/download | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/export/download) | - | 未找到 | - |
| 1476 | 搜索会议室层级 | GET | /open-apis/vc/v1/room_levels/search | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/room_level/search) | - | 未找到 | - |
| 1477 | 查询会议室列表 | GET | /open-apis/vc/v1/rooms | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/room/list) | - | 未找到 | - |
| 1478 | 查询会议室层级列表 | GET | /open-apis/vc/v1/room_levels | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/room_level/list) | - | 未找到 | - |
| 1479 | 查询会议室层级详情 | GET | /open-apis/vc/v1/room_levels/:room_level_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/room_level/get) | - | 未找到 | - |
| 1480 | 查询会议室详情 | GET | /open-apis/vc/v1/rooms/:room_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/room/get) | - | 未找到 | - |
| 1481 | 查询会议室配置 | GET | /open-apis/vc/v1/scope_config | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/scope_config/get) | - | 未找到 | - |
| 1482 | 查询会议室配置 | GET | /open-apis/vc/v1/room_configs/query | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/room_config/query) | - | 未找到 | - |
| 1483 | 查询会议室预定数据 | GET | /open-apis/vc/v1/resource_reservation_list | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/resource_reservation_list/get) | - | 未找到 | - |
| 1484 | 查询会议室预定管理员 | GET | /open-apis/vc/v1/reserve_configs/:reserve_config_id/admin | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/reserve_config-admin/get) | - | 未找到 | - |
| 1485 | 查询会议室预定表单 | GET | /open-apis/vc/v1/reserve_configs/:reserve_config_id/form | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/reserve_config-form/get) | - | 未找到 | - |
| 1486 | 查询会议室预定限制 | GET | /open-apis/vc/v1/reserve_configs/reserve_scope | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/reserve_config/reserve_scope) | - | 未找到 | - |
| 1487 | 查询会议明细 | GET | /open-apis/vc/v1/meeting_list | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/meeting_list/get) | - | 未找到 | - |
| 1488 | 查询参会人会议质量数据 | GET | /open-apis/vc/v1/participant_quality_list | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/participant_quality_list/get) | - | 未找到 | - |
| 1489 | 查询参会人明细 | GET | /open-apis/vc/v1/participant_list | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/participant_list/get) | - | 未找到 | - |
| 1490 | 查询导出任务结果 | GET | /open-apis/vc/v1/exports/:task_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/export/get) | - | 未找到 | - |
| 1491 | 查询禁用状态变更通知 | GET | /open-apis/vc/v1/reserve_configs/:reserve_config_id/disable_inform | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/reserve_config-disable_inform/get) | - | 未找到 | - |
| 1492 | 获取 Top 用户列表 | GET | /open-apis/vc/v1/reports/get_top_user | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/report/get_top_user) | - | 未找到 | - |
| 1493 | 获取与会议号关联的会议列表 | GET | /open-apis/vc/v1/meetings/list_by_no | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/meeting/list_by_no) | - | 未找到 | - |
| 1494 | 获取会议报告 | GET | /open-apis/vc/v1/reports/get_daily | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/report/get_daily) | - | 未找到 | - |
| 1495 | 获取会议详情 | GET | /open-apis/vc/v1/meetings/:meeting_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/meeting/get) | - | 未找到 | - |
| 1496 | 获取告警记录 | GET | /open-apis/vc/v1/alerts | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/alert/list) | - | 未找到 | - |
| 1497 | 获取录制文件 | GET | /open-apis/vc/v1/meetings/:meeting_id/recording | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/meeting-recording/get) | - | 未找到 | - |
| 1498 | 获取活跃会议 | GET | /open-apis/vc/v1/reserves/:reserve_id/get_active_meeting | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/reserve/get_active_meeting) | - | 未找到 | - |
| 1499 | 获取预约 | GET | /open-apis/vc/v1/reserves/:reserve_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/reserve/get) | - | 未找到 | - |
| 1500 | 创建会议室 | POST | /open-apis/vc/v1/rooms | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/room/create) | - | 未找到 | - |
| 1501 | 创建会议室层级 | POST | /open-apis/vc/v1/room_levels | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/room_level/create) | - | 未找到 | - |
| 1502 | 创建会议室部署码 | POST | /open-apis/vc/v1/room_configs/set_room_access_code | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/room_config/set_room_access_code) | - | 未找到 | - |
| 1503 | 创建签到板部署码 | POST | /open-apis/vc/v1/room_configs/set_checkboard_access_code | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/room_config/set_checkboard_access_code) | - | 未找到 | - |
| 1504 | 删除会议室层级 | POST | /open-apis/vc/v1/room_levels/del | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/room_level/del) | - | 未找到 | - |
| 1505 | 导出会议室预定数据 | POST | /open-apis/vc/v1/exports/resource_reservation_list | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/export/resource_reservation_list) | - | 未找到 | - |
| 1506 | 导出会议明细 | POST | /open-apis/vc/v1/exports/meeting_list | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/export/meeting_list) | - | 未找到 | - |
| 1507 | 导出参会人会议质量数据 | POST | /open-apis/vc/v1/exports/participant_quality_list | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/export/participant_quality_list) | - | 未找到 | - |
| 1508 | 导出参会人明细 | POST | /open-apis/vc/v1/exports/participant_list | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/export/participant_list) | - | 未找到 | - |
| 1509 | 批量查询会议室层级详情 | POST | /open-apis/vc/v1/room_levels/mget | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/room_level/mget) | - | 未找到 | - |
| 1510 | 批量查询会议室详情 | POST | /open-apis/vc/v1/rooms/mget | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/room/mget) | - | 未找到 | - |
| 1511 | 搜索会议室 | POST | /open-apis/vc/v1/rooms/search | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/room/search) | - | 未找到 | - |
| 1512 | 移除参会人 | POST | /open-apis/vc/v1/meetings/:meeting_id/kickout | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/meeting/kickout) | - | 未找到 | - |
| 1513 | 设置会议室配置 | POST | /open-apis/vc/v1/scope_config | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/scope_config/create) | - | 未找到 | - |
| 1514 | 设置会议室配置 | POST | /open-apis/vc/v1/room_configs/set | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/room_config/set) | - | 未找到 | - |
| 1515 | 预约会议 | POST | /open-apis/vc/v1/reserves/apply | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/reserve/apply) | - | 未找到 | - |
| 1516 | 更新预约 | PUT | /open-apis/vc/v1/reserves/:reserve_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/reserve/update) | - | 未找到 | - |
| 1517 | 停止录制 | PATCH | /open-apis/vc/v1/meetings/:meeting_id/recording/stop | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/meeting-recording/stop) | - | 未找到 | - |
| 1518 | 开始录制 | PATCH | /open-apis/vc/v1/meetings/:meeting_id/recording/start | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/meeting-recording/start) | - | 未找到 | - |
| 1519 | 授权录制文件 | PATCH | /open-apis/vc/v1/meetings/:meeting_id/recording/set_permission | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/meeting-recording/set_permission) | - | 未找到 | - |
| 1520 | 更新会议室 | PATCH | /open-apis/vc/v1/rooms/:room_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/room/patch) | - | 未找到 | - |
| 1521 | 更新会议室层级 | PATCH | /open-apis/vc/v1/room_levels/:room_level_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/room_level/patch) | - | 未找到 | - |
| 1522 | 更新会议室预定管理员 | PATCH | /open-apis/vc/v1/reserve_configs/:reserve_config_id/admin | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/reserve_config-admin/patch) | - | 未找到 | - |
| 1523 | 更新会议室预定表单 | PATCH | /open-apis/vc/v1/reserve_configs/:reserve_config_id/form | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/reserve_config-form/patch) | - | 未找到 | - |
| 1524 | 更新会议室预定限制 | PATCH | /open-apis/vc/v1/reserve_configs/:reserve_config_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/reserve_config/patch) | - | 未找到 | - |
| 1525 | 更新禁用状态变更通知 | PATCH | /open-apis/vc/v1/reserve_configs/:reserve_config_id/disable_inform | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/reserve_config-disable_inform/patch) | - | 未找到 | - |
| 1526 | 结束会议 | PATCH | /open-apis/vc/v1/meetings/:meeting_id/end | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/meeting/end) | - | 未找到 | - |
| 1527 | 设置主持人 | PATCH | /open-apis/vc/v1/meetings/:meeting_id/set_host | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/meeting/set_host) | - | 未找到 | - |
| 1528 | 邀请参会人 | PATCH | /open-apis/vc/v1/meetings/:meeting_id/invite | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/meeting/invite) | - | 未找到 | - |
| 1529 | 删除会议室 | DELETE | /open-apis/vc/v1/rooms/:room_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/room/delete) | - | 未找到 | - |
| 1530 | 删除预约 | DELETE | /open-apis/vc/v1/reserves/:reserve_id | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/reserve/delete) | - | 未找到 | - |

### 📦 verification 模块

| 1531 | 获取认证信息 | GET | /open-apis/verification/v1/verification | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/verification-v1/verification/get) | - | 未找到 | - |

### 📦 wiki 模块

| 1532 | 获取任务结果 | GET | /open-apis/wiki/v2/tasks/:task_id | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/wiki-v2/task/get) | - | 未找到 | - |
| 1533 | 获取知识空间信息 | GET | /open-apis/wiki/v2/spaces/:space_id | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/wiki-v2/space/get) | - | 未找到 | - |
| 1534 | 获取知识空间列表 | GET | /open-apis/wiki/v2/spaces | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/wiki-v2/space/list) | - | 未找到 | - |
| 1535 | 获取知识空间子节点列表 | GET | /open-apis/wiki/v2/spaces/:space_id/nodes | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/wiki-v2/space-node/list) | - | 未找到 | - |
| 1536 | 获取知识空间成员列表 | GET | /open-apis/wiki/v2/spaces/:space_id/members | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/wiki-v2/space-member/list) | - | 未找到 | - |
| 1537 | 获取知识空间节点信息 | GET | /open-apis/wiki/v2/spaces/get_node | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/wiki-v2/space/get_node) | - | 未找到 | - |
| 1538 | 创建知识空间 | POST | /open-apis/wiki/v2/spaces | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/wiki-v2/space/create) | - | 未找到 | - |
| 1539 | 创建知识空间节点 | POST | /open-apis/wiki/v2/spaces/:space_id/nodes | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/wiki-v2/space-node/create) | - | 未找到 | - |
| 1540 | 创建知识空间节点副本 | POST | /open-apis/wiki/v2/spaces/:space_id/nodes/:node_token/copy | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/wiki-v2/space-node/copy) | - | 未找到 | - |
| 1541 | 搜索 Wiki | POST | /open-apis/wiki/v1/nodes/search | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uEzN0YjLxcDN24SM3QjN/search_wiki) | - | 未找到 | - |
| 1542 | 更新知识空间节点标题 | POST | /open-apis/wiki/v2/spaces/:space_id/nodes/:node_token/update_title | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/wiki-v2/space-node/update_title) | - | 未找到 | - |
| 1543 | 添加知识空间成员 | POST | /open-apis/wiki/v2/spaces/:space_id/members | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/wiki-v2/space-member/create) | - | 未找到 | - |
| 1544 | 移动云空间文档至知识空间 | POST | /open-apis/wiki/v2/spaces/:space_id/nodes/move_docs_to_wiki | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/wiki-v2/space-node/move_docs_to_wiki) | - | 未找到 | - |
| 1545 | 移动知识空间节点 | POST | /open-apis/wiki/v2/spaces/:space_id/nodes/:node_token/move | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/wiki-v2/space-node/move) | - | 未找到 | - |
| 1546 | 更新知识空间设置 | PUT | /open-apis/wiki/v2/spaces/:space_id/setting | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/wiki-v2/space-setting/update) | - | 未找到 | - |
| 1547 | 删除知识空间成员 | DELETE | /open-apis/wiki/v2/spaces/:space_id/members/:member_id | ❌ | [文档](https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/wiki-v2/space-member/delete) | - | 未找到 | - |

### 📦 workplace 模块

| 1548 | 获取定制工作台小组件访问数据 | POST | /open-apis/workplace/v1/workplace_block_access_data/search | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/workplace-v1/workplace_block_access_data/search) | - | 未找到 | - |
| 1549 | 获取定制工作台访问数据 | POST | /open-apis/workplace/v1/custom_workplace_access_data/search | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/workplace-v1/custom_workplace_access_data/search) | - | 未找到 | - |
| 1550 | 获取工作台访问数据 | POST | /open-apis/workplace/v1/workplace_access_data/search | ❌ | [文档](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/workplace-v1/workplace_access_data/search) | - | 未找到 | - |
