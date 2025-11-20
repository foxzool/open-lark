# 增强版URL-Based API匹配报告

生成时间: 2025-11-08 21:24:18

## 📊 总体统计

- **总API数**: 1550
- **成功匹配**: 1
- **匹配率**: 0.1%
- **代码中URL总数**: 20
- **未匹配的代码URL**: 19

## 🏢 服务级别统计

| 服务 | 总数 | 已匹配 | 未匹配 | 匹配率 |
|------|------|--------|--------|--------|
| acs | 14 | 0 | 14 | 0.0% |
| admin | 14 | 0 | 14 | 0.0% |
| aily | 21 | 0 | 21 | 0.0% |
| apaas | 37 | 0 | 37 | 0.0% |
| application | 32 | 0 | 32 | 0.0% |
| approval | 29 | 0 | 29 | 0.0% |
| attendance | 39 | 0 | 39 | 0.0% |
| auth | 5 | 0 | 5 | 0.0% |
| authen | 6 | 0 | 6 | 0.0% |
| baike | 13 | 0 | 13 | 0.0% |
| base | 3 | 0 | 3 | 0.0% |
| bitable | 46 | 0 | 46 | 0.0% |
| board | 6 | 0 | 6 | 0.0% |
| calendar | 44 | 0 | 44 | 0.0% |
| cardkit | 10 | 0 | 10 | 0.0% |
| compensation | 21 | 0 | 21 | 0.0% |
| contact | 75 | 0 | 75 | 0.0% |
| corehr | 249 | 0 | 249 | 0.0% |
| directory | 21 | 0 | 21 | 0.0% |
| doc | 6 | 0 | 6 | 0.0% |
| docs | 1 | 0 | 1 | 0.0% |
| document_ai | 18 | 0 | 18 | 0.0% |
| docx | 19 | 0 | 19 | 0.0% |
| drive | 70 | 0 | 70 | 0.0% |
| ehr | 2 | 0 | 2 | 0.0% |
| ephemeral | 2 | 0 | 2 | 0.0% |
| event | 1 | 0 | 1 | 0.0% |
| face_verify | 3 | 0 | 3 | 0.0% |
| helpdesk | 50 | 0 | 50 | 0.0% |
| hire | 182 | 0 | 182 | 0.0% |
| human_authentication | 1 | 0 | 1 | 0.0% |
| im | 71 | 0 | 71 | 0.0% |
| interactive | 1 | 0 | 1 | 0.0% |
| lingo | 14 | 0 | 14 | 0.0% |
| mail | 67 | 0 | 67 | 0.0% |
| mdm | 4 | 0 | 4 | 0.0% |
| meeting_room | 17 | 0 | 17 | 0.0% |
| message | 1 | 0 | 1 | 0.0% |
| minutes | 4 | 0 | 4 | 0.0% |
| moments | 1 | 0 | 1 | 0.0% |
| okr | 12 | 0 | 12 | 0.0% |
| optical_char_recognition | 1 | 0 | 1 | 0.0% |
| passport | 2 | 0 | 2 | 0.0% |
| pay | 3 | 0 | 3 | 0.0% |
| payroll | 12 | 0 | 12 | 0.0% |
| performance | 20 | 0 | 20 | 0.0% |
| personal_settings | 6 | 0 | 6 | 0.0% |
| report | 3 | 0 | 3 | 0.0% |
| search | 15 | 0 | 15 | 0.0% |
| security_and_compliance | 8 | 0 | 8 | 0.0% |
| sheets | 59 | 1 | 58 | 1.7% |
| speech_to_text | 2 | 0 | 2 | 0.0% |
| suite | 2 | 0 | 2 | 0.0% |
| task | 75 | 0 | 75 | 0.0% |
| tenant | 2 | 0 | 2 | 0.0% |
| translation | 2 | 0 | 2 | 0.0% |
| trust_party | 5 | 0 | 5 | 0.0% |
| unknown | 24 | 0 | 24 | 0.0% |
| user | 1 | 0 | 1 | 0.0% |
| vc | 56 | 0 | 56 | 0.0% |
| verification | 1 | 0 | 1 | 0.0% |
| wiki | 16 | 0 | 16 | 0.0% |
| workplace | 3 | 0 | 3 | 0.0% |

## ✅ 成功匹配的API

| API ID | 方法 | 路径 | 实现位置 | 置信度 |
|--------|------|------|----------|--------|
| 创建电子表格 | POST | /open-apis/sheets/v3/spreadsheets | src/service/sheets/v3/spreadsheet_create.rs:349 | 1.00 |

## ❌ 未匹配的API

⚠️ 共有 1549 个未匹配的API，只显示前20个

| API ID | 方法 | 路径 | 描述 |
|--------|------|------|------|
| 获取事件出口 IP | GET | /open-apis/event/v1/outbound_ip | 飞书开放平台向应用配置的回调地址推送事件时，是通过特定的 IP 发送出去的，应用可以通过本接口获取所... |
| 获取用户信息 | GET | /open-apis/authen/v1/user_info | 通过 `user_access_token` 获取登录用户的信息。 |
| 批量获取脱敏的用户登录信息 | POST | /open-apis/passport/v1/sessions/query | 该接口用于查询用户的登录信息。 |
| 退出登录 | POST | /open-apis/passport/v1/sessions/logout |  |
| 自建应用获取 tenant_access_token | POST | /open-apis/auth/v3/tenant_access_token/internal | 企业自建应用通过此接口获取 tenant_access_token，调用接口获取企业资源时，需要使用... |
| 自建应用获取 app_access_token | POST | /open-apis/auth/v3/app_access_token/internal | 企业自建应用通过此接口获取 app_access_token，调用接口获取应用资源时，需要使用 ap... |
| 重新获取 app_ticket | POST | /open-apis/auth/v3/app_ticket/resend | 飞书每隔 1 小时会给应用推送一次最新的 app_ticket，应用也可以主动调用此接口，触发飞书进... |
| 商店应用获取 app_access_token | POST | /open-apis/auth/v3/app_access_token | 应用商店应用通过此接口获取 app_access_token，调用接口获取应用资源时，需要使用 ap... |
| 商店应用获取 tenant_access_token | POST | /open-apis/auth/v3/tenant_access_token | 应用商店应用通过此接口获取 tenant_access_token，调用接口获取企业资源时，需要使用... |
| 获取通讯录授权范围 | GET | /open-apis/contact/v3/scopes | 该接口用于获取应用被授权可访问的通讯录范围，包括可访问的部门列表、用户列表和用户组列表。 授权范围为... |
| 创建用户 | POST | /open-apis/contact/v3/users | 使用该接口向通讯录创建一个用户，可以理解为员工入职。创建用户后只返回有数据权限的数据。具体的数据权限... |
| 修改用户部分信息 | PATCH | /open-apis/contact/v3/users/:user_id | 该接口用于更新通讯录中用户的字段，未传递的参数不会更新。 |
| 更新用户 ID | PATCH | /open-apis/contact/v3/users/:user_id/update_user_id |  |
| 获取单个用户信息 | GET | /open-apis/contact/v3/users/:user_id | 该接口用于获取通讯录中单个用户的信息。 |
| 批量获取用户信息 | GET | /open-apis/contact/v3/users/batch |  |
| 获取部门直属用户列表 | GET | /open-apis/contact/v3/users/find_by_department | 基于部门ID获取部门直属用户列表。 |
| 通过手机号或邮箱获取用户 ID | POST | /open-apis/contact/v3/users/batch_get_id | 通过该接口，可使用手机号/邮箱获取用户的 ID 信息，具体获取支持的 ID 类型包括 open_id... |
| 搜索用户 | GET | /open-apis/search/v1/user | 以用户身份搜索其他用户的信息，无法搜索到外部企业或已离职的用户。 |
| 删除用户 | DELETE | /open-apis/contact/v3/users/:user_id | 该接口用于从通讯录删除一个用户信息，可以理解为员工离职。 |
| 恢复已删除用户 | POST | /open-apis/contact/v3/users/:user_id/resurrect | 该接口用于恢复已删除用户（已离职的成员），仅自建应用可申请，应用商店应用无权调用接口。 |

## 🔍 代码中额外的URL

这些URL在代码中找到，但不在API列表中：

- **POST**: /open-apis/sheets/v2/spreadsheets/:/merge_cells
  - 位置: src/service/sheets/v2/merge_cells.rs:319
  - 原因: Not in API list

- **POST**: /open-apis/sheets/v2/spreadsheets/:/unmerge_cells
  - 位置: src/service/sheets/v2/merge_cells.rs:370
  - 原因: Not in API list

- **POST**: /open-apis/sheets/v2/spreadsheets/:/sheets_batch_update
  - 位置: src/service/sheets/v2/sheet_management.rs:283
  - 原因: Not in API list

- **POST**: /open-apis/sheets/v2/spreadsheets/:/metainfo
  - 位置: src/service/sheets/v2/metainfo.rs:298
  - 原因: Not in API list

- **GET**: /open-apis/sheets/v2/spreadsheets/:/metainfo?:
  - 位置: src/service/sheets/v2/metainfo.rs:303
  - 原因: Not in API list

- **POST**: /open-apis/sheets/v2/spreadsheets/:/insert_dimension_range
  - 位置: src/service/sheets/v2/dimension_operations.rs:573
  - 原因: Not in API list

- **DELETE**: /open-apis/sheets/v2/spreadsheets/:/dimension_range
  - 位置: src/service/sheets/v2/dimension_operations.rs:627
  - 原因: Not in API list

- **POST**: /open-apis/sheets/v2/spreadsheets/:/dimension_range
  - 位置: src/service/sheets/v2/dimension_operations.rs:687
  - 原因: Not in API list

- **PUT**: /open-apis/sheets/v2/spreadsheets/:/dimension_range
  - 位置: src/service/sheets/v2/dimension_operations.rs:747
  - 原因: Not in API list

- **PUT**: /open-apis/sheets/v2/spreadsheets/:/styles_batch_update
  - 位置: src/service/sheets/v2/style_operations.rs:552
  - 原因: Not in API list

...还有 9 个额外的URL

