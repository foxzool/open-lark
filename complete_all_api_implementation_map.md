# 完整API实现映射表

**生成时间**: 2025-11-05 12:44:40  
**总API数**: 1551  
**已实现**: 864  
**实现率**: 55.7%  
**处理耗时**: 1.3 分钟  
**处理速度**: 20.2 API/秒  

| 序号 | API名称 | 请求方式 | API地址 | 文件路径 | 行号 | 状态 |
|------|---------|----------|---------|----------|------|------|
| 1 | 获取事件出口 IP | GET | `/open-apis/event/v1/outbound_ip` | `未找到` | - | ❌ 未实现 |
| 2 | 获取用户信息 | GET | `/open-apis/authen/v1/user_info` | `../src/service/auth/v1/mod.rs` | 27 | ✅ 已实现 |
| 3 | 批量获取脱敏的用户登录信息 | POST | `/open-apis/passport/v1/sessions/query` | `../src/service/attendance/v1/user_task.rs` | 86 | ✅ 已实现 |
| 4 | 退出登录 | POST | `/open-apis/passport/v1/sessions/logout` | `未找到` | - | ❌ 未实现 |
| 5 | 自建应用获取 tenant_access_token | POST | `/open-apis/auth/v3/tenant_access_token/internal` | `../src/service/auth/v3/mod.rs` | 27 | ✅ 已实现 |
| 6 | 自建应用获取 app_access_token | POST | `/open-apis/auth/v3/app_access_token/internal` | `../src/service/auth/v3/mod.rs` | 27 | ✅ 已实现 |
| 7 | 重新获取 app_ticket | POST | `/open-apis/auth/v3/app_ticket/resend` | `../src/service/auth/v3/mod.rs` | 61 | ✅ 已实现 |
| 8 | 商店应用获取 app_access_token | POST | `/open-apis/auth/v3/app_access_token` | `../src/service/auth/v3/mod.rs` | 44 | ✅ 已实现 |
| 9 | 商店应用获取 tenant_access_token | POST | `/open-apis/auth/v3/tenant_access_token` | `../src/service/auth/v3/mod.rs` | 27 | ✅ 已实现 |
| 10 | 获取通讯录授权范围 | GET | `/open-apis/contact/v3/scopes` | `../src/service/contact/v3/functional_role_member.rs` | 63 | ✅ 已实现 |
| 11 | 创建用户 | POST | `/open-apis/contact/v3/users` | `../src/service/contact/v3/group.rs` | 206 | ✅ 已实现 |
| 12 | 修改用户部分信息 | PATCH | `/open-apis/contact/v3/users/:user_id` | `../src/service/contact/v3/group.rs` | 206 | ✅ 已实现 |
| 13 | 更新用户 ID | PATCH | `/open-apis/contact/v3/users/:user_id/update_user_id` | `../src/service/contact/v3/group.rs` | 206 | ✅ 已实现 |
| 14 | 获取单个用户信息 | GET | `/open-apis/contact/v3/users/:user_id` | `../src/service/contact/v3/group.rs` | 206 | ✅ 已实现 |
| 15 | 批量获取用户信息 | GET | `/open-apis/contact/v3/users/batch` | `../src/service/contact/v3/group_member.rs` | 44 | ✅ 已实现 |
| 16 | 获取部门直属用户列表 | GET | `/open-apis/contact/v3/users/find_by_department` | `../src/service/contact/v3/group.rs` | 206 | ✅ 已实现 |
| 17 | 通过手机号或邮箱获取用户 ID | POST | `/open-apis/contact/v3/users/batch_get_id` | `../src/service/contact/v3/group.rs` | 206 | ✅ 已实现 |
| 18 | 搜索用户 | GET | `/open-apis/search/v1/user` | `../src/service/search/v1/user.rs` | 282 | ✅ 已实现 |
| 19 | 删除用户 | DELETE | `/open-apis/contact/v3/users/:user_id` | `../src/service/contact/v3/group.rs` | 206 | ✅ 已实现 |
| 20 | 恢复已删除用户 | POST | `/open-apis/contact/v3/users/:user_id/resurrect` | `../src/service/contact/v3/group.rs` | 206 | ✅ 已实现 |
| 21 | 创建用户组 | POST | `/open-apis/contact/v3/group` | `../src/service/contact/v3/group.rs` | 206 | ✅ 已实现 |
| 22 | 更新用户组 | PATCH | `/open-apis/contact/v3/group/:group_id` | `../src/service/payroll/v1/paygroup.rs` | 53 | ✅ 已实现 |
| 23 | 查询指定用户组 | GET | `/open-apis/contact/v3/group/:group_id` | `../src/service/payroll/v1/paygroup.rs` | 53 | ✅ 已实现 |
| 24 | 查询用户组列表 | GET | `/open-apis/contact/v3/group/simplelist` | `../src/service/contact/v3/group_member.rs` | 62 | ✅ 已实现 |
| 25 | 查询用户所属用户组 | GET | `/open-apis/contact/v3/group/member_belong` | `未找到` | - | ❌ 未实现 |
| 26 | 删除用户组 | DELETE | `/open-apis/contact/v3/group/:group_id` | `../src/service/payroll/v1/paygroup.rs` | 53 | ✅ 已实现 |
| 27 | 获取企业自定义用户字段 | GET | `/open-apis/contact/v3/custom_attrs` | `未找到` | - | ❌ 未实现 |
| 28 | 新增人员类型 | POST | `/open-apis/contact/v3/employee_type_enums` | `未找到` | - | ❌ 未实现 |
| 29 | 更新人员类型 | PUT | `/open-apis/contact/v3/employee_type_enums/:enum_id` | `未找到` | - | ❌ 未实现 |
| 30 | 查询人员类型 | GET | `/open-apis/contact/v3/employee_type_enums` | `未找到` | - | ❌ 未实现 |
| 31 | 删除人员类型 | DELETE | `/open-apis/contact/v3/employee_type_enums/:enum_id` | `未找到` | - | ❌ 未实现 |
| 32 | 创建部门 | POST | `/open-apis/contact/v3/departments` | `../src/service/contact/v3/department.rs` | 156 | ✅ 已实现 |
| 33 | 修改部门部分信息 | PATCH | `/open-apis/contact/v3/departments/:department_id` | `../src/service/contact/v3/department.rs` | 156 | ✅ 已实现 |
| 34 | 更新部门所有信息 | PUT | `/open-apis/contact/v3/departments/:department_id` | `../src/service/contact/v3/department.rs` | 156 | ✅ 已实现 |
| 35 | 更新部门 ID | PATCH | `/open-apis/contact/v3/departments/:department_id/update_department_id` | `../src/service/contact/v3/department.rs` | 156 | ✅ 已实现 |
| 36 | 部门群转为普通群 | POST | `/open-apis/contact/v3/departments/unbind_department_chat` | `../src/service/contact/v3/department.rs` | 156 | ✅ 已实现 |
| 37 | 获取单个部门信息 | GET | `/open-apis/contact/v3/departments/:department_id` | `../src/service/contact/v3/department.rs` | 156 | ✅ 已实现 |
| 38 | 批量获取部门信息 | GET | `/open-apis/contact/v3/departments/batch` | `../src/service/contact/v3/group_member.rs` | 44 | ✅ 已实现 |
| 39 | 获取子部门列表 | GET | `/open-apis/contact/v3/departments/:department_id/children` | `../src/service/contact/v3/department.rs` | 230 | ✅ 已实现 |
| 40 | 获取父部门信息 | GET | `/open-apis/contact/v3/departments/parent` | `../src/service/contact/v3/department.rs` | 276 | ✅ 已实现 |
| 41 | 搜索部门 | POST | `/open-apis/contact/v3/departments/search` | `../src/service/contact/v3/user.rs` | 485 | ✅ 已实现 |
| 42 | 删除部门 | DELETE | `/open-apis/contact/v3/departments/:department_id` | `../src/service/contact/v3/department.rs` | 156 | ✅ 已实现 |
| 43 | 创建单位 | POST | `/open-apis/contact/v3/unit` | `../src/service/contact/v3/unit.rs` | 130 | ✅ 已实现 |
| 44 | 修改单位信息 | PATCH | `/open-apis/contact/v3/unit/:unit_id` | `../src/service/contact/v3/unit.rs` | 130 | ✅ 已实现 |
| 45 | 建立部门与单位的绑定关系 | POST | `/open-apis/contact/v3/unit/bind_department` | `../src/service/contact/v3/unit.rs` | 62 | ✅ 已实现 |
| 46 | 解除部门与单位的绑定关系 | POST | `/open-apis/contact/v3/unit/unbind_department` | `../src/service/contact/v3/unit.rs` | 83 | ✅ 已实现 |
| 47 | 获取单位绑定的部门列表 | GET | `/open-apis/contact/v3/unit/list_department` | `../src/service/contact/v3/unit.rs` | 104 | ✅ 已实现 |
| 48 | 获取单位信息 | GET | `/open-apis/contact/v3/unit/:unit_id` | `../src/service/contact/v3/unit.rs` | 130 | ✅ 已实现 |
| 49 | 获取单位列表 | GET | `/open-apis/contact/v3/unit` | `../src/service/contact/v3/unit.rs` | 130 | ✅ 已实现 |
| 50 | 删除单位 | DELETE | `/open-apis/contact/v3/unit/:unit_id` | `../src/service/contact/v3/unit.rs` | 130 | ✅ 已实现 |
| 51 | 添加用户组成员 | POST | `/open-apis/contact/v3/group/:group_id/member/add` | `../src/service/contact/v3/group_member.rs` | 26 | ✅ 已实现 |
| 52 | 批量添加用户组成员 | POST | `/open-apis/contact/v3/group/:group_id/member/batch_add` | `../src/service/contact/v3/group_member.rs` | 44 | ✅ 已实现 |
| 53 | 查询用户组成员列表 | GET | `/open-apis/contact/v3/group/:group_id/member/simplelist` | `../src/service/contact/v3/group_member.rs` | 62 | ✅ 已实现 |
| 54 | 移除用户组成员 | POST | `/open-apis/contact/v3/group/:group_id/member/remove` | `../src/service/contact/v3/group_member.rs` | 84 | ✅ 已实现 |
| 55 | 批量移除用户组成员 | POST | `/open-apis/contact/v3/group/:group_id/member/batch_remove` | `../src/service/contact/v3/group_member.rs` | 105 | ✅ 已实现 |
| 56 | 创建角色 | POST | `/open-apis/contact/v3/functional_roles` | `../src/service/contact/v3/functional_role.rs` | 62 | ✅ 已实现 |
| 57 | 修改角色名称 | PUT | `/open-apis/contact/v3/functional_roles/:role_id` | `../src/service/contact/v3/functional_role.rs` | 62 | ✅ 已实现 |
| 58 | 删除角色 | DELETE | `/open-apis/contact/v3/functional_roles/:role_id` | `../src/service/contact/v3/functional_role.rs` | 62 | ✅ 已实现 |
| 59 | 批量添加角色成员 | POST | `/open-apis/contact/v3/functional_roles/:role_id/members/batch_create` | `../src/service/contact/v3/functional_role_member.rs` | 42 | ✅ 已实现 |
| 60 | 批量设置角色成员管理范围 | PATCH | `/open-apis/contact/v3/functional_roles/:role_id/members/scopes` | `../src/service/contact/v3/functional_role_member.rs` | 63 | ✅ 已实现 |
| 61 | 查询角色下某个成员的管理范围 | GET | `/open-apis/contact/v3/functional_roles/:role_id/members/:member_id` | `../src/service/contact/v3/functional_role.rs` | 62 | ✅ 已实现 |
| 62 | 查询角色下的所有成员信息 | GET | `/open-apis/contact/v3/functional_roles/:role_id/members` | `../src/service/contact/v3/functional_role.rs` | 62 | ✅ 已实现 |
| 63 | 删除角色下的成员 | PATCH | `/open-apis/contact/v3/functional_roles/:role_id/members/batch_delete` | `../src/service/contact/v3/functional_role_member.rs` | 131 | ✅ 已实现 |
| 64 | 创建职级 | POST | `/open-apis/contact/v3/job_levels` | `../src/service/corehr/job_management/mod.rs` | 286 | ✅ 已实现 |
| 65 | 更新职级 | PUT | `/open-apis/contact/v3/job_levels/:job_level_id` | `../src/service/contact/v3/job_level.rs` | 65 | ✅ 已实现 |
| 66 | 获取单个职级信息 | GET | `/open-apis/contact/v3/job_levels/:job_level_id` | `../src/service/contact/v3/job_level.rs` | 65 | ✅ 已实现 |
| 67 | 获取租户职级列表 | GET | `/open-apis/contact/v3/job_levels` | `../src/service/corehr/job_management/mod.rs` | 286 | ✅ 已实现 |
| 68 | 删除职级 | DELETE | `/open-apis/contact/v3/job_levels/:job_level_id` | `../src/service/contact/v3/job_level.rs` | 65 | ✅ 已实现 |
| 69 | 创建序列 | POST | `/open-apis/contact/v3/job_families` | `../src/service/corehr/job_management/mod.rs` | 190 | ✅ 已实现 |
| 70 | 更新序列 | PUT | `/open-apis/contact/v3/job_families/:job_family_id` | `../src/service/contact/v3/job_family.rs` | 66 | ✅ 已实现 |
| 71 | 获取单个序列信息 | GET | `/open-apis/contact/v3/job_families/:job_family_id` | `../src/service/contact/v3/job_family.rs` | 66 | ✅ 已实现 |
| 72 | 获取租户序列列表 | GET | `/open-apis/contact/v3/job_families` | `../src/service/corehr/job_management/mod.rs` | 190 | ✅ 已实现 |
| 73 | 删除序列 | DELETE | `/open-apis/contact/v3/job_families/:job_family_id` | `../src/service/contact/v3/job_family.rs` | 66 | ✅ 已实现 |
| 74 | 获取单个职务信息 | GET | `/open-apis/contact/v3/job_titles/:job_title_id` | `../src/service/contact/v3/job_title.rs` | 28 | ✅ 已实现 |
| 75 | 获取租户职务列表 | GET | `/open-apis/contact/v3/job_titles` | `未找到` | - | ❌ 未实现 |
| 76 | 获取单个工作城市信息 | GET | `/open-apis/contact/v3/work_cities/:work_city_id` | `../src/service/contact/v3/work_city.rs` | 28 | ✅ 已实现 |
| 77 | 获取租户工作城市列表 | GET | `/open-apis/contact/v3/work_cities` | `未找到` | - | ❌ 未实现 |
| 78 | 创建员工 | POST | `/open-apis/directory/v1/employees` | `../src/service/ehr/v1/mod.rs` | 263 | ✅ 已实现 |
| 79 | 更新员工信息 | PATCH | `/open-apis/directory/v1/employees/:employee_id` | `../src/service/ehr/v1/mod.rs` | 165 | ✅ 已实现 |
| 80 | 离职员工 | DELETE | `/open-apis/directory/v1/employees/:employee_id` | `../src/service/ehr/v1/mod.rs` | 165 | ✅ 已实现 |
| 81 | 恢复离职员工 | POST | `/open-apis/directory/v1/employees/:employee_id/resurrect` | `未找到` | - | ❌ 未实现 |
| 82 | 更新在职员工为待离职 | PATCH | `/open-apis/directory/v1/employees/:employee_id/to_be_resigned` | `未找到` | - | ❌ 未实现 |
| 83 | 更新待离职成员为在职 | PATCH | `/open-apis/directory/v1/employees/:employee_id/regular` | `未找到` | - | ❌ 未实现 |
| 84 | 批量获取员工信息 | POST | `/open-apis/directory/v1/employees/mget` | `未找到` | - | ❌ 未实现 |
| 85 | 批量获取员工列表 | POST | `/open-apis/directory/v1/employees/filter` | `未找到` | - | ❌ 未实现 |
| 86 | 搜索员工信息 | POST | `/open-apis/directory/v1/employees/search` | `../src/service/attendance/v1/group.rs` | 154 | ✅ 已实现 |
| 87 | 创建部门 | POST | `/open-apis/directory/v1/departments` | `../src/service/corehr/organization/mod.rs` | 161 | ✅ 已实现 |
| 88 | 更新部门 | PATCH | `/open-apis/directory/v1/departments/:department_id` | `../src/service/contact/v3/department.rs` | 156 | ✅ 已实现 |
| 89 | 删除部门 | DELETE | `/open-apis/directory/v1/departments/:department_id` | `../src/service/contact/v3/department.rs` | 156 | ✅ 已实现 |
| 90 | 批量获取部门信息 | POST | `/open-apis/directory/v1/departments/mget` | `未找到` | - | ❌ 未实现 |
| 91 | 获取部门列表 | POST | `/open-apis/directory/v1/departments/filter` | `未找到` | - | ❌ 未实现 |
| 92 | 搜索部门 | POST | `/open-apis/directory/v1/departments/search` | `../src/service/attendance/v1/group.rs` | 154 | ✅ 已实现 |
| 93 | 发送消息 | POST | `/open-apis/im/v1/messages` | `../src/service/aily/message/mod.rs` | 127 | ✅ 已实现 |
| 94 | 回复消息 | POST | `/open-apis/im/v1/messages/:message_id/reply` | `../src/service/cloud_docs/comments/mod.rs` | 171 | ✅ 已实现 |
| 95 | 编辑消息 | PUT | `/open-apis/im/v1/messages/:message_id` | `未找到` | - | ❌ 未实现 |
| 96 | 转发消息 | POST | `/open-apis/im/v1/messages/:message_id/forward` | `未找到` | - | ❌ 未实现 |
| 97 | 合并转发消息 | POST | `/open-apis/im/v1/messages/merge_forward` | `未找到` | - | ❌ 未实现 |
| 98 | 转发话题 | POST | `/open-apis/im/v1/threads/:thread_id/forward` | `未找到` | - | ❌ 未实现 |
| 99 | 撤回消息 | DELETE | `/open-apis/im/v1/messages/:message_id` | `未找到` | - | ❌ 未实现 |
| 100 | 添加跟随气泡 | POST | `/open-apis/im/v1/messages/:message_id/push_follow_up` | `未找到` | - | ❌ 未实现 |
| 101 | 查询消息已读信息 | GET | `/open-apis/im/v1/messages/:message_id/read_users` | `未找到` | - | ❌ 未实现 |
| 102 | 获取会话历史消息 | GET | `/open-apis/im/v1/messages` | `../src/service/aily/message/mod.rs` | 127 | ✅ 已实现 |
| 103 | 获取消息中的资源文件 | GET | `/open-apis/im/v1/messages/:message_id/resources/:file_key` | `未找到` | - | ❌ 未实现 |
| 104 | 获取指定消息的内容 | GET | `/open-apis/im/v1/messages/:message_id` | `未找到` | - | ❌ 未实现 |
| 105 | 批量发送消息 | POST | `/open-apis/message/v4/batch_send/` | `未找到` | - | ❌ 未实现 |
| 106 | 批量撤回消息 | DELETE | `/open-apis/im/v1/batch_messages/:batch_message_id` | `未找到` | - | ❌ 未实现 |
| 107 | 查询批量消息推送和阅读人数 | GET | `/open-apis/im/v1/batch_messages/:batch_message_id/read_user` | `../src/service/im/v1/batch_message/mod.rs` | 134 | ✅ 已实现 |
| 108 | 查询批量消息整体进度 | GET | `/open-apis/im/v1/batch_messages/:batch_message_id/get_progress` | `../src/service/im/v1/batch_message/mod.rs` | 117 | ✅ 已实现 |
| 109 | 上传图片 | POST | `/open-apis/im/v1/images` | `未找到` | - | ❌ 未实现 |
| 110 | 下载图片 | GET | `/open-apis/im/v1/images/:image_key` | `未找到` | - | ❌ 未实现 |
| 111 | 上传文件 | POST | `/open-apis/im/v1/files` | `../src/service/cloud_docs/drive/v1/folder.rs` | 133 | ✅ 已实现 |
| 112 | 下载文件 | GET | `/open-apis/im/v1/files/:file_key` | `未找到` | - | ❌ 未实现 |
| 113 | 发送应用内加急 | PATCH | `/open-apis/im/v1/messages/:message_id/urgent_app` | `../src/service/im/v1/buzz_messages/mod.rs` | 52 | ✅ 已实现 |
| 114 | 发送短信加急 | PATCH | `/open-apis/im/v1/messages/:message_id/urgent_sms` | `../src/service/im/v1/buzz_messages/mod.rs` | 73 | ✅ 已实现 |
| 115 | 发送电话加急 | PATCH | `/open-apis/im/v1/messages/:message_id/urgent_phone` | `../src/service/im/v1/buzz_messages/mod.rs` | 94 | ✅ 已实现 |
| 116 | 添加消息表情回复 | POST | `/open-apis/im/v1/messages/:message_id/reactions` | `未找到` | - | ❌ 未实现 |
| 117 | 获取消息表情回复 | GET | `/open-apis/im/v1/messages/:message_id/reactions` | `未找到` | - | ❌ 未实现 |
| 118 | 删除消息表情回复 | DELETE | `/open-apis/im/v1/messages/:message_id/reactions/:reaction_id` | `未找到` | - | ❌ 未实现 |
| 119 | Pin 消息 | POST | `/open-apis/im/v1/pins` | `未找到` | - | ❌ 未实现 |
| 120 | 移除 Pin 消息 | DELETE | `/open-apis/im/v1/pins/:message_id` | `未找到` | - | ❌ 未实现 |
| 121 | 获取群内 Pin 消息 | GET | `/open-apis/im/v1/pins` | `未找到` | - | ❌ 未实现 |
| 122 | 更新已发送的消息卡片 | PATCH | `/open-apis/im/v1/messages/:message_id` | `未找到` | - | ❌ 未实现 |
| 123 | 延时更新消息卡片 | POST | `/open-apis/interactive/v1/card/update` | `../src/service/lingo/draft/mod.rs` | 44 | ✅ 已实现 |
| 124 | 发送仅特定人可见的消息卡片 | POST | `/open-apis/ephemeral/v1/send` | `../src/service/auth/v3/mod.rs` | 61 | ✅ 已实现 |
| 125 | 删除仅特定人可见的消息卡片 | POST | `/open-apis/ephemeral/v1/delete` | `../src/service/attendance/v1/group.rs` | 101 | ✅ 已实现 |
| 126 | 更新 URL 预览 | POST | `/open-apis/im/v2/url_previews/batch_update` | `../src/service/im/v1/url_preview/mod.rs` | 31 | ✅ 已实现 |
| 127 | 创建群 | POST | `/open-apis/im/v1/chats` | `未找到` | - | ❌ 未实现 |
| 128 | 解散群 | DELETE | `/open-apis/im/v1/chats/:chat_id` | `未找到` | - | ❌ 未实现 |
| 129 | 更新群信息 | PUT | `/open-apis/im/v1/chats/:chat_id` | `未找到` | - | ❌ 未实现 |
| 130 | 更新群发言权限 | PUT | `/open-apis/im/v1/chats/:chat_id/moderation` | `未找到` | - | ❌ 未实现 |
| 131 | 获取群信息 | GET | `/open-apis/im/v1/chats/:chat_id` | `未找到` | - | ❌ 未实现 |
| 132 | 更新群置顶 | POST | `/open-apis/im/v1/chats/:chat_id/top_notice/put_top_notice` | `未找到` | - | ❌ 未实现 |
| 133 | 撤销群置顶 | POST | `/open-apis/im/v1/chats/:chat_id/top_notice/delete_top_notice` | `未找到` | - | ❌ 未实现 |
| 134 | 获取用户或机器人所在的群列表 | GET | `/open-apis/im/v1/chats` | `未找到` | - | ❌ 未实现 |
| 135 | 搜索对用户或机器人可见的群列表 | GET | `/open-apis/im/v1/chats/search` | `../src/service/attendance/v1/group.rs` | 154 | ✅ 已实现 |
| 136 | 获取群成员发言权限 | GET | `/open-apis/im/v1/chats/:chat_id/moderation` | `未找到` | - | ❌ 未实现 |
| 137 | 获取群分享链接 | POST | `/open-apis/im/v1/chats/:chat_id/link` | `未找到` | - | ❌ 未实现 |
| 138 | 指定群管理员 | POST | `/open-apis/im/v1/chats/:chat_id/managers/add_managers` | `未找到` | - | ❌ 未实现 |
| 139 | 删除群管理员 | POST | `/open-apis/im/v1/chats/:chat_id/managers/delete_managers` | `未找到` | - | ❌ 未实现 |
| 140 | 将用户或机器人拉入群聊 | POST | `/open-apis/im/v1/chats/:chat_id/members` | `../src/service/task/v2/tasklist/mod.rs` | 208 | ✅ 已实现 |
| 141 | 用户或机器人主动加入群聊 | PATCH | `/open-apis/im/v1/chats/:chat_id/members/me_join` | `未找到` | - | ❌ 未实现 |
| 142 | 将用户或机器人移出群聊 | DELETE | `/open-apis/im/v1/chats/:chat_id/members` | `../src/service/task/v2/tasklist/mod.rs` | 208 | ✅ 已实现 |
| 143 | 获取群成员列表 | GET | `/open-apis/im/v1/chats/:chat_id/members` | `../src/service/task/v2/tasklist/mod.rs` | 208 | ✅ 已实现 |
| 144 | 判断用户或机器人是否在群里 | GET | `/open-apis/im/v1/chats/:chat_id/members/is_in_chat` | `未找到` | - | ❌ 未实现 |
| 145 | 获取群公告基本信息 | GET | `/open-apis/docx/v1/chats/:chat_id/announcement` | `未找到` | - | ❌ 未实现 |
| 146 | 获取群公告所有块 | GET | `/open-apis/docx/v1/chats/:chat_id/announcement/blocks` | `../src/service/cloud_docs/docx/v1/document.rs` | 106 | ✅ 已实现 |
| 147 | 在群公告中创建块 | POST | `/open-apis/docx/v1/chats/:chat_id/announcement/blocks/:block_id/children` | `../src/service/contact/v3/department.rs` | 230 | ✅ 已实现 |
| 148 | 批量更新群公告块的内容 | PATCH | `/open-apis/docx/v1/chats/:chat_id/announcement/blocks/batch_update` | `../src/service/apass/object/mod.rs` | 304 | ✅ 已实现 |
| 149 | 获取群公告块的内容 | GET | `/open-apis/docx/v1/chats/:chat_id/announcement/blocks/:block_id` | `未找到` | - | ❌ 未实现 |
| 150 | 获取所有子块 | GET | `/open-apis/docx/v1/chats/:chat_id/announcement/blocks/:block_id/children` | `../src/service/contact/v3/department.rs` | 230 | ✅ 已实现 |
| 151 | 删除群公告中的块 | DELETE | `/open-apis/docx/v1/chats/:chat_id/announcement/blocks/:block_id/children/batch_delete` | `../src/service/contact/v3/functional_role_member.rs` | 131 | ✅ 已实现 |
| 152 | 更新群公告信息 | PATCH | `/open-apis/im/v1/chats/:chat_id/announcement` | `未找到` | - | ❌ 未实现 |
| 153 | 获取群公告信息 | GET | `/open-apis/im/v1/chats/:chat_id/announcement` | `未找到` | - | ❌ 未实现 |
| 154 | 添加会话标签页 | POST | `/open-apis/im/v1/chats/:chat_id/chat_tabs` | `未找到` | - | ❌ 未实现 |
| 155 | 删除会话标签页 | DELETE | `/open-apis/im/v1/chats/:chat_id/chat_tabs/delete_tabs` | `未找到` | - | ❌ 未实现 |
| 156 | 更新会话标签页 | POST | `/open-apis/im/v1/chats/:chat_id/chat_tabs/update_tabs` | `未找到` | - | ❌ 未实现 |
| 157 | 会话标签页排序 | POST | `/open-apis/im/v1/chats/:chat_id/chat_tabs/sort_tabs` | `未找到` | - | ❌ 未实现 |
| 158 | 拉取会话标签页 | GET | `/open-apis/im/v1/chats/:chat_id/chat_tabs/list_tabs` | `未找到` | - | ❌ 未实现 |
| 159 | 添加群菜单 | POST | `/open-apis/im/v1/chats/:chat_id/menu_tree` | `未找到` | - | ❌ 未实现 |
| 160 | 删除群菜单 | DELETE | `/open-apis/im/v1/chats/:chat_id/menu_tree` | `未找到` | - | ❌ 未实现 |
| 161 | 修改群菜单元信息 | PATCH | `/open-apis/im/v1/chats/:chat_id/menu_items/:menu_item_id` | `未找到` | - | ❌ 未实现 |
| 162 | 排序群菜单 | POST | `/open-apis/im/v1/chats/:chat_id/menu_tree/sort` | `未找到` | - | ❌ 未实现 |
| 163 | 获取群菜单 | GET | `/open-apis/im/v1/chats/:chat_id/menu_tree` | `未找到` | - | ❌ 未实现 |
| 164 | 创建卡片实体 | POST | `/open-apis/cardkit/v1/cards` | `未找到` | - | ❌ 未实现 |
| 165 | 更新卡片实体配置 | PATCH | `/open-apis/cardkit/v1/cards/:card_id/settings` | `../src/service/hire/recruitment_config/offer_settings/mod.rs` | 161 | ✅ 已实现 |
| 166 | 局部更新卡片实体 | POST | `/open-apis/cardkit/v1/cards/:card_id/batch_update` | `../src/service/apass/object/mod.rs` | 304 | ✅ 已实现 |
| 167 | 全量更新卡片实体 | PUT | `/open-apis/cardkit/v1/cards/:card_id` | `未找到` | - | ❌ 未实现 |
| 168 | 新增组件 | POST | `/open-apis/cardkit/v1/cards/:card_id/elements` | `未找到` | - | ❌ 未实现 |
| 169 | 更新组件 | PUT | `/open-apis/cardkit/v1/cards/:card_id/elements/:element_id` | `未找到` | - | ❌ 未实现 |
| 170 | 更新组件属性 | PATCH | `/open-apis/cardkit/v1/cards/:card_id/elements/:element_id` | `未找到` | - | ❌ 未实现 |
| 171 | 流式更新文本 | PUT | `/open-apis/cardkit/v1/cards/:card_id/elements/:element_id/content` | `../src/service/cloud_docs/docx/v1/document.rs` | 83 | ✅ 已实现 |
| 172 | 删除组件 | DELETE | `/open-apis/cardkit/v1/cards/:card_id/elements/:element_id` | `未找到` | - | ❌ 未实现 |
| 173 | 创建应用消息流卡片 | POST | `/open-apis/im/v2/app_feed_card` | `未找到` | - | ❌ 未实现 |
| 174 | 更新应用消息流卡片 | PUT | `/open-apis/im/v2/app_feed_card/batch` | `../src/service/im/v1/url_preview/mod.rs` | 31 | ✅ 已实现 |
| 175 | 删除应用消息流卡片 | DELETE | `/open-apis/im/v2/app_feed_card/batch` | `../src/service/im/v1/url_preview/mod.rs` | 31 | ✅ 已实现 |
| 176 | 机器人单聊即时提醒 | PATCH | `/open-apis/im/v2/feed_cards/bot_time_sentive` | `未找到` | - | ❌ 未实现 |
| 177 | 更新消息流卡片按钮 | PUT | `/open-apis/im/v2/chat_button` | `未找到` | - | ❌ 未实现 |
| 178 | 即时提醒 | PATCH | `/open-apis/im/v2/feed_cards/:feed_card_id` | `未找到` | - | ❌ 未实现 |
| 179 | 查询实体与标签的绑定关系 | GET | `/open-apis/im/v2/biz_entity_tag_relation` | `未找到` | - | ❌ 未实现 |
| 180 | 创建标签 | POST | `/open-apis/im/v2/tags` | `../src/service/hire/recruitment_config/application.rs` | 67 | ✅ 已实现 |
| 181 | 修改标签 | PATCH | `/open-apis/im/v2/tags/:tag_id` | `未找到` | - | ❌ 未实现 |
| 182 | 绑定标签到群 | POST | `/open-apis/im/v2/biz_entity_tag_relation` | `未找到` | - | ❌ 未实现 |
| 183 | 解绑标签与群 | PUT | `/open-apis/im/v2/biz_entity_tag_relation` | `未找到` | - | ❌ 未实现 |
| 184 | 获取我的空间（根文件夹）元数据 | GET | `/open-apis/drive/explorer/v2/root_folder/meta` | `../src/service/ccm/sheets/v2/spreadsheet.rs` | 37 | ✅ 已实现 |
| 185 | 获取文件夹中的文件清单 | GET | `/open-apis/drive/v1/files` | `../src/service/cloud_docs/drive/v1/folder.rs` | 133 | ✅ 已实现 |
| 186 | 获取文件夹元数据 | GET | `/open-apis/drive/explorer/v2/folder/:folderToken/meta` | `../src/service/ccm/sheets/v2/spreadsheet.rs` | 37 | ✅ 已实现 |
| 187 | 新建文件夹 | POST | `/open-apis/drive/v1/files/create_folder` | `../src/service/cloud_docs/drive/v1/folder.rs` | 237 | ✅ 已实现 |
| 188 | 查询异步任务状态 | GET | `/open-apis/drive/v1/files/task_check` | `未找到` | - | ❌ 未实现 |
| 189 | 获取文件元数据 | POST | `/open-apis/drive/v1/metas/batch_query` | `../src/service/apass/object/mod.rs` | 333 | ✅ 已实现 |
| 190 | 获取文件统计信息 | GET | `/open-apis/drive/v1/files/:file_token/statistics` | `../src/service/elearning/course_registration/mod.rs` | 203 | ✅ 已实现 |
| 191 | 获取文件访问记录 | GET | `/open-apis/drive/v1/files/:file_token/view_records` | `../src/service/cloud_docs/drive/v1/file.rs` | 74 | ✅ 已实现 |
| 192 | 复制文件 | POST | `/open-apis/drive/v1/files/:file_token/copy` | `../src/service/cloud_docs/bitable/v1/app_dashboard/copy.rs` | 67 | ✅ 已实现 |
| 193 | 移动文件或文件夹 | POST | `/open-apis/drive/v1/files/:file_token/move` | `../src/service/contact/v3/group_member.rs` | 84 | ✅ 已实现 |
| 194 | 删除文件或文件夹 | DELETE | `/open-apis/drive/v1/files/:file_token` | `未找到` | - | ❌ 未实现 |
| 195 | 创建文件快捷方式 | POST | `/open-apis/drive/v1/files/create_shortcut` | `未找到` | - | ❌ 未实现 |
| 196 | 搜索云文档 | POST | `/open-apis/suite/docs-api/search/object` | `未找到` | - | ❌ 未实现 |
| 197 | 上传文件 | POST | `/open-apis/drive/v1/files/upload_all` | `../src/service/cloud_docs/drive/v1/media.rs` | 39 | ✅ 已实现 |
| 198 | 分片上传文件-预上传 | POST | `/open-apis/drive/v1/files/upload_prepare` | `../src/service/cloud_docs/drive/v1/media.rs` | 75 | ✅ 已实现 |
| 199 | 分片上传文件-上传分片 | POST | `/open-apis/drive/v1/files/upload_part` | `../src/service/cloud_docs/drive/v1/media.rs` | 98 | ✅ 已实现 |
| 200 | 分片上传文件-完成上传 | POST | `/open-apis/drive/v1/files/upload_finish` | `../src/service/cloud_docs/drive/v1/media.rs` | 119 | ✅ 已实现 |
| 201 | 下载文件 | GET | `/open-apis/drive/v1/files/:file_token/download` | `../src/service/attendance/v1/user_setting.rs` | 117 | ✅ 已实现 |
| 202 | 创建导入任务 | POST | `/open-apis/drive/v1/import_tasks` | `未找到` | - | ❌ 未实现 |
| 203 | 查询导入任务结果 | GET | `/open-apis/drive/v1/import_tasks/:ticket` | `../src/service/auth/v3/mod.rs` | 61 | ✅ 已实现 |
| 204 | 创建导出任务 | POST | `/open-apis/drive/v1/export_tasks` | `未找到` | - | ❌ 未实现 |
| 205 | 查询导出任务结果 | GET | `/open-apis/drive/v1/export_tasks/:ticket` | `../src/service/auth/v3/mod.rs` | 61 | ✅ 已实现 |
| 206 | 下载导出文件 | GET | `/open-apis/drive/export_tasks/file/:file_token/download` | `../src/service/attendance/v1/user_setting.rs` | 117 | ✅ 已实现 |
| 207 | 上传素材 | POST | `/open-apis/drive/v1/medias/upload_all` | `../src/service/cloud_docs/drive/v1/media.rs` | 39 | ✅ 已实现 |
| 208 | 分片上传素材-预上传 | POST | `/open-apis/drive/v1/medias/upload_prepare` | `../src/service/cloud_docs/drive/v1/media.rs` | 75 | ✅ 已实现 |
| 209 | 分片上传素材-上传分片 | POST | `/open-apis/drive/v1/medias/upload_part` | `../src/service/cloud_docs/drive/v1/media.rs` | 98 | ✅ 已实现 |
| 210 | 分片上传素材-完成上传 | POST | `/open-apis/drive/v1/medias/upload_finish` | `../src/service/cloud_docs/drive/v1/media.rs` | 119 | ✅ 已实现 |
| 211 | 下载素材 | GET | `/open-apis/drive/v1/medias/:file_token/download` | `../src/service/attendance/v1/user_setting.rs` | 117 | ✅ 已实现 |
| 212 | 获取素材临时下载链接 | GET | `/open-apis/drive/v1/medias/batch_get_tmp_download_url` | `../src/service/cloud_docs/drive/v1/media.rs` | 165 | ✅ 已实现 |
| 213 | 创建文档版本 | POST | `/open-apis/drive/v1/files/:file_token/versions` | `../src/service/cloud_docs/drive/v1/file_version.rs` | 114 | ✅ 已实现 |
| 214 | 获取文档版本列表 | GET | `/open-apis/drive/v1/files/:file_token/versions` | `../src/service/cloud_docs/drive/v1/file_version.rs` | 114 | ✅ 已实现 |
| 215 | 获取文档版本信息 | GET | `/open-apis/drive/v1/files/:file_token/versions/:version_id` | `未找到` | - | ❌ 未实现 |
| 216 | 删除文档版本 | DELETE | `/open-apis/drive/v1/files/:file_token/versions/:version_id` | `未找到` | - | ❌ 未实现 |
| 217 | 获取云文档的点赞者列表 | GET | `/open-apis/drive/v2/files/:file_token/likes` | `../src/service/cloud_docs/drive/v1/like.rs` | 39 | ✅ 已实现 |
| 218 | 订阅云文档事件 | POST | `/open-apis/drive/v1/files/:file_token/subscribe` | `../src/service/calendar/v4/mod.rs` | 497 | ✅ 已实现 |
| 219 | 查询云文档事件订阅状态 | GET | `/open-apis/drive/v1/files/:file_token/get_subscribe` | `未找到` | - | ❌ 未实现 |
| 220 | 取消云文档事件订阅 | DELETE | `/open-apis/drive/v1/files/:file_token/delete_subscribe` | `未找到` | - | ❌ 未实现 |
| 221 | 获取知识空间列表 | GET | `/open-apis/wiki/v2/spaces` | `../src/service/cloud_docs/wiki/v2/space/list.rs` | 71 | ✅ 已实现 |
| 222 | 获取知识空间信息 | GET | `/open-apis/wiki/v2/spaces/:space_id` | `未找到` | - | ❌ 未实现 |
| 223 | 创建知识空间 | POST | `/open-apis/wiki/v2/spaces` | `../src/service/cloud_docs/wiki/v2/space/list.rs` | 71 | ✅ 已实现 |
| 224 | 获取知识空间成员列表 | GET | `/open-apis/wiki/v2/spaces/:space_id/members` | `../src/service/task/v2/tasklist/mod.rs` | 208 | ✅ 已实现 |
| 225 | 添加知识空间成员 | POST | `/open-apis/wiki/v2/spaces/:space_id/members` | `../src/service/task/v2/tasklist/mod.rs` | 208 | ✅ 已实现 |
| 226 | 删除知识空间成员 | DELETE | `/open-apis/wiki/v2/spaces/:space_id/members/:member_id` | `未找到` | - | ❌ 未实现 |
| 227 | 更新知识空间设置 | PUT | `/open-apis/wiki/v2/spaces/:space_id/setting` | `../src/service/cloud_docs/wiki/v2/space_setting/update.rs` | 57 | ✅ 已实现 |
| 228 | 创建知识空间节点 | POST | `/open-apis/wiki/v2/spaces/:space_id/nodes` | `../src/service/cloud_docs/board/v1/whiteboard_node/list.rs` | 618 | ✅ 已实现 |
| 229 | 获取知识空间节点信息 | GET | `/open-apis/wiki/v2/spaces/get_node` | `未找到` | - | ❌ 未实现 |
| 230 | 获取知识空间子节点列表 | GET | `/open-apis/wiki/v2/spaces/:space_id/nodes` | `../src/service/cloud_docs/board/v1/whiteboard_node/list.rs` | 618 | ✅ 已实现 |
| 231 | 移动知识空间节点 | POST | `/open-apis/wiki/v2/spaces/:space_id/nodes/:node_token/move` | `../src/service/contact/v3/group_member.rs` | 84 | ✅ 已实现 |
| 232 | 更新知识空间节点标题 | POST | `/open-apis/wiki/v2/spaces/:space_id/nodes/:node_token/update_title` | `../src/service/cloud_docs/wiki/v2/space_node/mod.rs` | 116 | ✅ 已实现 |
| 233 | 创建知识空间节点副本 | POST | `/open-apis/wiki/v2/spaces/:space_id/nodes/:node_token/copy` | `../src/service/cloud_docs/bitable/v1/app_dashboard/copy.rs` | 67 | ✅ 已实现 |
| 234 | 移动云空间文档至知识空间 | POST | `/open-apis/wiki/v2/spaces/:space_id/nodes/move_docs_to_wiki` | `../src/service/cloud_docs/wiki/v2/task/mod.rs` | 25 | ✅ 已实现 |
| 235 | 获取任务结果 | GET | `/open-apis/wiki/v2/tasks/:task_id` | `未找到` | - | ❌ 未实现 |
| 236 | 搜索 Wiki | POST | `/open-apis/wiki/v1/nodes/search` | `../src/service/attendance/v1/group.rs` | 154 | ✅ 已实现 |
| 237 | 创建文档 | POST | `/open-apis/docx/v1/documents` | `未找到` | - | ❌ 未实现 |
| 238 | 获取文档基本信息 | GET | `/open-apis/docx/v1/documents/:document_id` | `未找到` | - | ❌ 未实现 |
| 239 | 获取文档纯文本内容 | GET | `/open-apis/docx/v1/documents/:document_id/raw_content` | `../src/service/cloud_docs/docx/v1/document.rs` | 83 | ✅ 已实现 |
| 240 | 获取文档所有块 | GET | `/open-apis/docx/v1/documents/:document_id/blocks` | `../src/service/cloud_docs/docx/v1/document.rs` | 106 | ✅ 已实现 |
| 241 | 创建块 | POST | `/open-apis/docx/v1/documents/:document_id/blocks/:block_id/children` | `../src/service/contact/v3/department.rs` | 230 | ✅ 已实现 |
| 242 | 创建嵌套块 | POST | `/open-apis/docx/v1/documents/:document_id/blocks/:block_id/descendant` | `未找到` | - | ❌ 未实现 |
| 243 | 更新块的内容 | PATCH | `/open-apis/docx/v1/documents/:document_id/blocks/:block_id` | `未找到` | - | ❌ 未实现 |
| 244 | 获取块的内容 | GET | `/open-apis/docx/v1/documents/:document_id/blocks/:block_id` | `未找到` | - | ❌ 未实现 |
| 245 | 批量更新块的内容 | PATCH | `/open-apis/docx/v1/documents/:document_id/blocks/batch_update` | `../src/service/apass/object/mod.rs` | 304 | ✅ 已实现 |
| 246 | 获取所有子块 | GET | `/open-apis/docx/v1/documents/:document_id/blocks/:block_id/children` | `../src/service/contact/v3/department.rs` | 230 | ✅ 已实现 |
| 247 | 删除块 | DELETE | `/open-apis/docx/v1/documents/:document_id/blocks/:block_id/children/batch_delete` | `../src/service/contact/v3/functional_role_member.rs` | 131 | ✅ 已实现 |
| 248 | Markdown/HTML 内容转换为文档块 | POST | `/open-apis/docx/documents/blocks/convert` | `../src/service/corehr/basic_info/mod.rs` | 232 | ✅ 已实现 |
| 249 | 创建电子表格 | POST | `/open-apis/sheets/v3/spreadsheets` | `未找到` | - | ❌ 未实现 |
| 250 | 修改电子表格属性 | PATCH | `/open-apis/sheets/v3/spreadsheets/:spreadsheet_token` | `../src/service/ccm/sheets/v2/spreadsheet.rs` | 129 | ✅ 已实现 |
| 251 | 获取电子表格信息 | GET | `/open-apis/sheets/v3/spreadsheets/:spreadsheet_token` | `../src/service/ccm/sheets/v2/spreadsheet.rs` | 129 | ✅ 已实现 |
| 252 | 操作工作表 | POST | `/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/sheets_batch_update` | `未找到` | - | ❌ 未实现 |
| 253 | 更新工作表属性 | POST | `/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/sheets_batch_update` | `未找到` | - | ❌ 未实现 |
| 254 | 获取工作表 | GET | `/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/query` | `../src/service/attendance/v1/user_task.rs` | 86 | ✅ 已实现 |
| 255 | 查询工作表 | GET | `/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id` | `未找到` | - | ❌ 未实现 |
| 256 | 增加行列 | POST | `/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/dimension_range` | `未找到` | - | ❌ 未实现 |
| 257 | 插入行列 | POST | `/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/insert_dimension_range` | `未找到` | - | ❌ 未实现 |
| 258 | 更新行列 | PUT | `/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/dimension_range` | `未找到` | - | ❌ 未实现 |
| 259 | 移动行列 | POST | `/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/move_dimension` | `未找到` | - | ❌ 未实现 |
| 260 | 删除行列 | DELETE | `/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/dimension_range` | `未找到` | - | ❌ 未实现 |
| 261 | 合并单元格 | POST | `/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/merge_cells` | `未找到` | - | ❌ 未实现 |
| 262 | 拆分单元格 | POST | `/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/unmerge_cells` | `未找到` | - | ❌ 未实现 |
| 263 | 查找单元格 | POST | `/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/find` | `../src/service/performance/stage_task/mod.rs` | 36 | ✅ 已实现 |
| 264 | 替换单元格 | POST | `/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/replace` | `未找到` | - | ❌ 未实现 |
| 265 | 设置单元格样式  | PUT | `/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/style` | `未找到` | - | ❌ 未实现 |
| 266 | 批量设置单元格样式  | PUT | `/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/styles_batch_update` | `未找到` | - | ❌ 未实现 |
| 267 | 插入数据 | POST | `/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/values_prepend` | `未找到` | - | ❌ 未实现 |
| 268 | 追加数据 | POST | `/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/values_append` | `未找到` | - | ❌ 未实现 |
| 269 | 写入图片 | POST | `/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/values_image` | `未找到` | - | ❌ 未实现 |
| 270 | 读取单个范围 | GET | `/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/values/:range` | `../src/service/cloud_docs/sheets/v3/data_operation/reading_single_range.rs` | 70 | ✅ 已实现 |
| 271 | 读取多个范围 | GET | `/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/values_batch_get` | `未找到` | - | ❌ 未实现 |
| 272 | 向单个范围写入数据 | PUT | `/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/values` | `未找到` | - | ❌ 未实现 |
| 273 | 向多个范围写入数据 | POST | `/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/values_batch_update` | `未找到` | - | ❌ 未实现 |
| 274 | 创建筛选 | POST | `/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter` | `未找到` | - | ❌ 未实现 |
| 275 | 更新筛选 | PUT | `/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter` | `未找到` | - | ❌ 未实现 |
| 276 | 获取筛选 | GET | `/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter` | `未找到` | - | ❌ 未实现 |
| 277 | 删除筛选 | DELETE | `/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter` | `未找到` | - | ❌ 未实现 |
| 278 | 创建筛选视图 | POST | `/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter_views` | `未找到` | - | ❌ 未实现 |
| 279 | 更新筛选视图 | PATCH | `/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter_views/:filter_view_id` | `未找到` | - | ❌ 未实现 |
| 280 | 查询筛选视图 | GET | `/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter_views/query` | `../src/service/attendance/v1/user_task.rs` | 86 | ✅ 已实现 |
| 281 | 获取筛选视图 | GET | `/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter_views/:filter_view_id` | `未找到` | - | ❌ 未实现 |
| 282 | 删除筛选视图 | DELETE | `/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter_views/:filter_view_id` | `未找到` | - | ❌ 未实现 |
| 283 | 创建筛选条件 | POST | `/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter_views/:filter_view_id/conditions` | `未找到` | - | ❌ 未实现 |
| 284 | 更新筛选条件 | PUT | `/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter_views/:filter_view_id/conditions/:condition_id` | `未找到` | - | ❌ 未实现 |
| 285 | 查询筛选条件 | GET | `/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter_views/:filter_view_id/conditions/query` | `../src/service/attendance/v1/user_task.rs` | 86 | ✅ 已实现 |
| 286 | 获取筛选条件 | GET | `/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter_views/:filter_view_id/conditions/:condition_id` | `未找到` | - | ❌ 未实现 |
| 287 | 删除筛选条件 | DELETE | `/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter_views/:filter_view_id/conditions/:condition_id` | `未找到` | - | ❌ 未实现 |
| 288 | 增加保护范围 | POST | `/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/protected_dimension` | `未找到` | - | ❌ 未实现 |
| 289 | 修改保护范围 | POST | `/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/protected_range_batch_update` | `未找到` | - | ❌ 未实现 |
| 290 | 获取保护范围 | GET | `/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/protected_range_batch_get` | `未找到` | - | ❌ 未实现 |
| 291 | 删除保护范围 | DELETE | `/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/protected_range_batch_del` | `未找到` | - | ❌ 未实现 |
| 292 | 设置下拉列表 | POST | `/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/dataValidation` | `未找到` | - | ❌ 未实现 |
| 293 | 更新下拉列表设置 | PUT | `/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/dataValidation/:sheetId/:dataValidationId` | `未找到` | - | ❌ 未实现 |
| 294 | 查询下拉列表设置 | GET | `/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/dataValidation` | `未找到` | - | ❌ 未实现 |
| 295 | 删除下拉列表设置 | DELETE | `/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/dataValidation` | `未找到` | - | ❌ 未实现 |
| 296 | 批量创建条件格式 | POST | `/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/condition_formats/batch_create` | `../src/service/contact/v3/functional_role_member.rs` | 42 | ✅ 已实现 |
| 297 | 批量更新条件格式 | POST | `/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/condition_formats/batch_update` | `../src/service/apass/object/mod.rs` | 304 | ✅ 已实现 |
| 298 | 批量获取条件格式 | GET | `/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/condition_formats` | `未找到` | - | ❌ 未实现 |
| 299 | 批量删除条件格式 | DELETE | `/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/condition_formats/batch_delete` | `../src/service/contact/v3/functional_role_member.rs` | 131 | ✅ 已实现 |
| 300 | 创建浮动图片 | POST | `/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/float_images` | `未找到` | - | ❌ 未实现 |
| 301 | 更新浮动图片 | PATCH | `/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/float_images/:float_image_id` | `未找到` | - | ❌ 未实现 |
| 302 | 获取浮动图片 | GET | `/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/float_images/:float_image_id` | `未找到` | - | ❌ 未实现 |
| 303 | 查询浮动图片 | GET | `/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/float_images/query` | `../src/service/attendance/v1/user_task.rs` | 86 | ✅ 已实现 |
| 304 | 删除浮动图片 | DELETE | `/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/float_images/:float_image_id` | `未找到` | - | ❌ 未实现 |
| 305 | 创建多维表格 | POST | `/open-apis/bitable/v1/apps` | `../src/service/workplace/app_recommend/mod.rs` | 47 | ✅ 已实现 |
| 306 | 复制多维表格 | POST | `/open-apis/bitable/v1/apps/:app_token/copy` | `../src/service/cloud_docs/bitable/v1/app_dashboard/copy.rs` | 67 | ✅ 已实现 |
| 307 | 获取多维表格元数据 | GET | `/open-apis/bitable/v1/apps/:app_token` | `../src/service/base/bitable/mod.rs` | 44 | ✅ 已实现 |
| 308 | 更新多维表格元数据 | PUT | `/open-apis/bitable/v1/apps/:app_token` | `../src/service/base/bitable/mod.rs` | 44 | ✅ 已实现 |
| 309 | 新增一个数据表 | POST | `/open-apis/bitable/v1/apps/:app_token/tables` | `../src/service/base/bitable/mod.rs` | 84 | ✅ 已实现 |
| 310 | 新增多个数据表 | POST | `/open-apis/bitable/v1/apps/:app_token/tables/batch_create` | `../src/service/contact/v3/functional_role_member.rs` | 42 | ✅ 已实现 |
| 311 | 更新数据表 | PATCH | `/open-apis/bitable/v1/apps/:app_token/tables/:table_id` | `../src/service/base/bitable/mod.rs` | 93 | ✅ 已实现 |
| 312 | 列出数据表 | GET | `/open-apis/bitable/v1/apps/:app_token/tables` | `../src/service/base/bitable/mod.rs` | 84 | ✅ 已实现 |
| 313 | 删除一个数据表 | DELETE | `/open-apis/bitable/v1/apps/:app_token/tables/:table_id` | `../src/service/base/bitable/mod.rs` | 93 | ✅ 已实现 |
| 314 | 删除多个数据表 | POST | `/open-apis/bitable/v1/apps/:app_token/tables/batch_delete` | `../src/service/contact/v3/functional_role_member.rs` | 131 | ✅ 已实现 |
| 315 | 新增视图 | POST | `/open-apis/bitable/v1/apps/:app_token/tables/:table_id/views` | `../src/service/okr/review/mod.rs` | 82 | ✅ 已实现 |
| 316 | 更新视图 | PATCH | `/open-apis/bitable/v1/apps/:app_token/tables/:table_id/views/:view_id` | `../src/service/performance/v1/reviews.rs` | 31 | ✅ 已实现 |
| 317 | 列出视图 | GET | `/open-apis/bitable/v1/apps/:app_token/tables/:table_id/views` | `../src/service/okr/review/mod.rs` | 82 | ✅ 已实现 |
| 318 | 获取视图 | GET | `/open-apis/bitable/v1/apps/:app_token/tables/:table_id/views/:view_id` | `../src/service/performance/v1/reviews.rs` | 31 | ✅ 已实现 |
| 319 | 删除视图 | DELETE | `/open-apis/bitable/v1/apps/:app_token/tables/:table_id/views/:view_id` | `../src/service/performance/v1/reviews.rs` | 31 | ✅ 已实现 |
| 320 | 新增记录 | POST | `/open-apis/bitable/v1/apps/:app_token/tables/:table_id/records` | `../src/service/okr/v1/mod.rs` | 409 | ✅ 已实现 |
| 321 | 更新记录 | PUT | `/open-apis/bitable/v1/apps/:app_token/tables/:table_id/records/:record_id` | `../src/service/base/bitable/mod.rs` | 135 | ✅ 已实现 |
| 322 | 查询记录 | POST | `/open-apis/bitable/v1/apps/:app_token/tables/:table_id/records/search` | `../src/service/attendance/v1/group.rs` | 154 | ✅ 已实现 |
| 323 | 删除记录 | DELETE | `/open-apis/bitable/v1/apps/:app_token/tables/:table_id/records/:record_id` | `../src/service/base/bitable/mod.rs` | 135 | ✅ 已实现 |
| 324 | 新增多条记录 | POST | `/open-apis/bitable/v1/apps/:app_token/tables/:table_id/records/batch_create` | `../src/service/contact/v3/functional_role_member.rs` | 42 | ✅ 已实现 |
| 325 | 更新多条记录 | POST | `/open-apis/bitable/v1/apps/:app_token/tables/:table_id/records/batch_update` | `../src/service/apass/object/mod.rs` | 304 | ✅ 已实现 |
| 326 | 批量获取记录 | POST | `/open-apis/bitable/v1/apps/:app_token/tables/:table_id/records/batch_get` | `../src/service/contact/v3/department.rs` | 212 | ✅ 已实现 |
| 327 | 删除多条记录 | POST | `/open-apis/bitable/v1/apps/:app_token/tables/:table_id/records/batch_delete` | `../src/service/contact/v3/functional_role_member.rs` | 131 | ✅ 已实现 |
| 328 | 新增字段 | POST | `/open-apis/bitable/v1/apps/:app_token/tables/:table_id/fields` | `../src/service/ai/document_ai/mod.rs` | 265 | ✅ 已实现 |
| 329 | 更新字段 | PUT | `/open-apis/bitable/v1/apps/:app_token/tables/:table_id/fields/:field_id` | `未找到` | - | ❌ 未实现 |
| 330 | 列出字段 | GET | `/open-apis/bitable/v1/apps/:app_token/tables/:table_id/fields` | `../src/service/ai/document_ai/mod.rs` | 265 | ✅ 已实现 |
| 331 | 删除字段 | DELETE | `/open-apis/bitable/v1/apps/:app_token/tables/:table_id/fields/:field_id` | `未找到` | - | ❌ 未实现 |
| 332 | 复制仪表盘 | POST | `/open-apis/bitable/v1/apps/:app_token/dashboards/:block_id/copy` | `../src/service/cloud_docs/bitable/v1/app_dashboard/copy.rs` | 67 | ✅ 已实现 |
| 333 | 列出仪表盘 | GET | `/open-apis/bitable/v1/apps/:app_token/dashboards` | `未找到` | - | ❌ 未实现 |
| 334 | 更新表单元数据 | PATCH | `/open-apis/bitable/v1/apps/:app_token/tables/:table_id/forms/:form_id` | `未找到` | - | ❌ 未实现 |
| 335 | 获取表单元数据 | GET | `/open-apis/bitable/v1/apps/:app_token/tables/:table_id/forms/:form_id` | `未找到` | - | ❌ 未实现 |
| 336 | 更新表单问题 | PATCH | `/open-apis/bitable/v1/apps/:app_token/tables/:table_id/forms/:form_id/fields/:field_id` | `未找到` | - | ❌ 未实现 |
| 337 | 列出表单问题 | GET | `/open-apis/bitable/v1/apps/:app_token/tables/:table_id/forms/:form_id/fields` | `../src/service/ai/document_ai/mod.rs` | 265 | ✅ 已实现 |
| 338 | 新增自定义角色 | POST | `/open-apis/base/v2/apps/:app_token/roles` | `../src/service/cloud_docs/bitable/v1/app_role/list.rs` | 75 | ✅ 已实现 |
| 339 | 更新自定义角色 | PUT | `/open-apis/base/v2/apps/:app_token/roles/:role_id` | `../src/service/contact/v3/functional_role.rs` | 62 | ✅ 已实现 |
| 340 | 列出自定义角色 | GET | `/open-apis/base/v2/apps/:app_token/roles` | `../src/service/cloud_docs/bitable/v1/app_role/list.rs` | 75 | ✅ 已实现 |
| 341 | 删除自定义角色 | DELETE | `/open-apis/bitable/v1/apps/:app_token/roles/:role_id` | `../src/service/contact/v3/functional_role.rs` | 62 | ✅ 已实现 |
| 342 | 新增协作者 | POST | `/open-apis/bitable/v1/apps/:app_token/roles/:role_id/members` | `../src/service/task/v2/tasklist/mod.rs` | 208 | ✅ 已实现 |
| 343 | 批量新增协作者 | POST | `/open-apis/bitable/v1/apps/:app_token/roles/:role_id/members/batch_create` | `../src/service/contact/v3/functional_role_member.rs` | 42 | ✅ 已实现 |
| 344 | 列出协作者 | GET | `/open-apis/bitable/v1/apps/:app_token/roles/:role_id/members` | `../src/service/task/v2/tasklist/mod.rs` | 208 | ✅ 已实现 |
| 345 | 删除协作者 | DELETE | `/open-apis/bitable/v1/apps/:app_token/roles/:role_id/members/:member_id` | `未找到` | - | ❌ 未实现 |
| 346 | 批量删除协作者 | POST | `/open-apis/bitable/v1/apps/:app_token/roles/:role_id/members/batch_delete` | `../src/service/contact/v3/functional_role_member.rs` | 131 | ✅ 已实现 |
| 347 | 列出自动化流程 | GET | `/open-apis/bitable/v1/apps/:app_token/workflows` | `../src/service/cloud_docs/bitable/v1/app_workflow/list.rs` | 92 | ✅ 已实现 |
| 348 | 更新自动化流程状态 | PUT | `/open-apis/bitable/v1/apps/:app_token/workflows/:workflow_id` | `未找到` | - | ❌ 未实现 |
| 349 | 获取画板主题 | GET | `/open-apis/board/v1/whiteboards/:whiteboard_id/theme` | `未找到` | - | ❌ 未实现 |
| 350 | 更新画板主题 | POST | `/open-apis/board/v1/whiteboards/:whiteboard_id/update_theme` | `未找到` | - | ❌ 未实现 |
| 351 | 获取画板缩略图片 | GET | `/open-apis/board/v1/whiteboards/:whiteboard_id/download_as_image` | `未找到` | - | ❌ 未实现 |
| 352 | 解析画板语法 | POST | `/open-apis/board/v1/whiteboards/:whiteboard_id/nodes/plantuml` | `未找到` | - | ❌ 未实现 |
| 353 | 创建节点 | POST | `/open-apis/board/v1/whiteboards/:whiteboard_id/nodes` | `../src/service/cloud_docs/board/v1/whiteboard_node/list.rs` | 618 | ✅ 已实现 |
| 354 | 获取所有节点 | GET | `/open-apis/board/v1/whiteboards/:whiteboard_id/nodes` | `../src/service/cloud_docs/board/v1/whiteboard_node/list.rs` | 618 | ✅ 已实现 |
| 355 | 增加协作者权限 | POST | `/open-apis/drive/v1/permissions/:token/members` | `../src/service/task/v2/tasklist/mod.rs` | 208 | ✅ 已实现 |
| 356 | 批量增加协作者权限 | POST | `/open-apis/drive/v1/permissions/:token/members/batch_create` | `../src/service/contact/v3/functional_role_member.rs` | 42 | ✅ 已实现 |
| 357 | 更新协作者权限 | PUT | `/open-apis/drive/v1/permissions/:token/members/:member_id` | `未找到` | - | ❌ 未实现 |
| 358 | 获取云文档协作者 | GET | `/open-apis/drive/v1/permissions/:token/members` | `../src/service/task/v2/tasklist/mod.rs` | 208 | ✅ 已实现 |
| 359 | 移除云文档协作者权限 | DELETE | `/open-apis/drive/v1/permissions/:token/members/:member_id` | `未找到` | - | ❌ 未实现 |
| 360 | 转移云文档所有者 | POST | `/open-apis/drive/v1/permissions/:token/members/transfer_owner` | `../src/service/cloud_docs/permission/member/transfer_owner.rs` | 68 | ✅ 已实现 |
| 361 | 判断用户云文档权限 | GET | `/open-apis/drive/v1/permissions/:token/members/auth` | `../src/service/contact/v3/scope.rs` | 50 | ✅ 已实现 |
| 362 | 更新云文档权限设置 | PATCH | `/open-apis/drive/v2/permissions/:token/public` | `../src/service/cloud_docs/permission/public_v2/patch.rs` | 103 | ✅ 已实现 |
| 363 | 获取云文档权限设置 | GET | `/open-apis/drive/v2/permissions/:token/public` | `../src/service/cloud_docs/permission/public_v2/patch.rs` | 103 | ✅ 已实现 |
| 364 | 启用云文档密码 | POST | `/open-apis/drive/v1/permissions/:token/public/password` | `../src/service/cloud_docs/permission/mod.rs` | 162 | ✅ 已实现 |
| 365 | 刷新云文档密码 | PUT | `/open-apis/drive/v1/permissions/:token/public/password` | `../src/service/cloud_docs/permission/mod.rs` | 162 | ✅ 已实现 |
| 366 | 停用云文档密码 | DELETE | `/open-apis/drive/v1/permissions/:token/public/password` | `../src/service/cloud_docs/permission/mod.rs` | 162 | ✅ 已实现 |
| 367 | 获取云文档所有评论 | GET | `/open-apis/drive/v1/files/:file_token/comments` | `../src/service/task/v2/mod.rs` | 338 | ✅ 已实现 |
| 368 | 批量获取评论 | POST | `/open-apis/drive/v1/files/:file_token/comments/batch_query` | `../src/service/apass/object/mod.rs` | 333 | ✅ 已实现 |
| 369 | 解决/恢复评论 | PATCH | `/open-apis/drive/v1/files/:file_token/comments/:comment_id` | `未找到` | - | ❌ 未实现 |
| 370 | 添加全文评论 | POST | `/open-apis/drive/v1/files/:file_token/comments` | `../src/service/task/v2/mod.rs` | 338 | ✅ 已实现 |
| 371 | 获取全文评论 | GET | `/open-apis/drive/v1/files/:file_token/comments/:comment_id` | `未找到` | - | ❌ 未实现 |
| 372 | 获取回复信息 | GET | `/open-apis/drive/v1/files/:file_token/comments/:comment_id/replies` | `../src/service/cloud_docs/comments/list_replies.rs` | 307 | ✅ 已实现 |
| 373 | 更新回复的内容 | PUT | `/open-apis/drive/v1/files/:file_token/comments/:comment_id/replies/:reply_id` | `未找到` | - | ❌ 未实现 |
| 374 | 删除回复 | DELETE | `/open-apis/drive/v1/files/:file_token/comments/:comment_id/replies/:reply_id` | `未找到` | - | ❌ 未实现 |
| 375 | 获取订阅状态 | GET | `/open-apis/drive/v1/files/:file_token/subscriptions/:subscription_id` | `未找到` | - | ❌ 未实现 |
| 376 | 创建订阅 | POST | `/open-apis/drive/v1/files/:file_token/subscriptions` | `../src/service/calendar/v4/mod.rs` | 535 | ✅ 已实现 |
| 377 | 更新订阅状态 | PATCH | `/open-apis/drive/v1/files/:file_token/subscriptions/:subscription_id` | `未找到` | - | ❌ 未实现 |
| 378 | 获取云文档内容 | GET | `/open-apis/docs/v1/content` | `../src/service/cloud_docs/docx/v1/document.rs` | 83 | ✅ 已实现 |
| 379 | 创建共享日历 | POST | `/open-apis/calendar/v4/calendars` | `../src/service/calendar/v4/mod.rs` | 213 | ✅ 已实现 |
| 380 | 删除共享日历 | DELETE | `/open-apis/calendar/v4/calendars/:calendar_id` | `../src/service/calendar/v4/mod.rs` | 33 | ✅ 已实现 |
| 381 | 查询主日历信息 | POST | `/open-apis/calendar/v4/calendars/primary` | `../src/service/calendar/v4/mod.rs` | 188 | ✅ 已实现 |
| 382 | 批量获取主日历信息 | POST | `/open-apis/calendar/v4/calendars/primarys` | `../src/service/calendar/v4/mod.rs` | 33 | ✅ 已实现 |
| 383 | 查询日历信息 | GET | `/open-apis/calendar/v4/calendars/:calendar_id` | `../src/service/calendar/v4/mod.rs` | 33 | ✅ 已实现 |
| 384 | 批量查询日历信息 | POST | `/open-apis/calendar/v4/calendars/mget` | `../src/service/calendar/v4/mod.rs` | 33 | ✅ 已实现 |
| 385 | 查询主日历日程忙闲信息 | POST | `/open-apis/calendar/v4/freebusy/list` | `../src/service/calendar/v4/mod.rs` | 128 | ✅ 已实现 |
| 386 | 批量查询主日历日程忙闲信息 | POST | `/open-apis/calendar/v4/freebusy/batch` | `../src/service/calendar/v4/mod.rs` | 33 | ✅ 已实现 |
| 387 | 查询日历列表 | GET | `/open-apis/calendar/v4/calendars` | `../src/service/calendar/v4/mod.rs` | 213 | ✅ 已实现 |
| 388 | 更新日历信息 | PATCH | `/open-apis/calendar/v4/calendars/:calendar_id` | `../src/service/calendar/v4/mod.rs` | 33 | ✅ 已实现 |
| 389 | 搜索日历 | POST | `/open-apis/calendar/v4/calendars/search` | `../src/service/calendar/v4/mod.rs` | 33 | ✅ 已实现 |
| 390 | 订阅日历 | POST | `/open-apis/calendar/v4/calendars/:calendar_id/subscribe` | `../src/service/calendar/v4/mod.rs` | 497 | ✅ 已实现 |
| 391 | 取消订阅日历 | POST | `/open-apis/calendar/v4/calendars/:calendar_id/unsubscribe` | `../src/service/calendar/v4/mod.rs` | 518 | ✅ 已实现 |
| 392 | 订阅日历变更事件 | POST | `/open-apis/calendar/v4/calendars/subscription` | `../src/service/calendar/v4/mod.rs` | 535 | ✅ 已实现 |
| 393 | 取消订阅日历变更事件 | POST | `/open-apis/calendar/v4/calendars/unsubscription` | `../src/service/calendar/v4/mod.rs` | 33 | ✅ 已实现 |
| 394 | 创建访问控制 | POST | `/open-apis/calendar/v4/calendars/:calendar_id/acls` | `../src/service/calendar/v4/mod.rs` | 33 | ✅ 已实现 |
| 395 | 删除访问控制 | DELETE | `/open-apis/calendar/v4/calendars/:calendar_id/acls/:acl_id` | `../src/service/calendar/v4/mod.rs` | 33 | ✅ 已实现 |
| 396 | 获取访问控制列表 | GET | `/open-apis/calendar/v4/calendars/:calendar_id/acls` | `../src/service/calendar/v4/mod.rs` | 33 | ✅ 已实现 |
| 397 | 订阅日历访问控制变更事件 | POST | `/open-apis/calendar/v4/calendars/:calendar_id/acls/subscription` | `../src/service/calendar/v4/mod.rs` | 535 | ✅ 已实现 |
| 398 | 取消订阅日历访问控制变更事件 | POST | `/open-apis/calendar/v4/calendars/:calendar_id/acls/unsubscription` | `../src/service/calendar/v4/mod.rs` | 33 | ✅ 已实现 |
| 399 | 创建日程 | POST | `/open-apis/calendar/v4/calendars/:calendar_id/events` | `../src/service/calendar/v4/mod.rs` | 128 | ✅ 已实现 |
| 400 | 删除日程 | DELETE | `/open-apis/calendar/v4/calendars/:calendar_id/events/:event_id` | `../src/service/calendar/v4/mod.rs` | 33 | ✅ 已实现 |
| 401 | 更新日程 | PATCH | `/open-apis/calendar/v4/calendars/:calendar_id/events/:event_id` | `../src/service/calendar/v4/mod.rs` | 33 | ✅ 已实现 |
| 402 | 获取日程 | GET | `/open-apis/calendar/v4/calendars/:calendar_id/events/:event_id` | `../src/service/calendar/v4/mod.rs` | 33 | ✅ 已实现 |
| 403 | 获取日程列表 | GET | `/open-apis/calendar/v4/calendars/:calendar_id/events` | `../src/service/calendar/v4/mod.rs` | 128 | ✅ 已实现 |
| 404 | 搜索日程 | POST | `/open-apis/calendar/v4/calendars/:calendar_id/events/search` | `../src/service/calendar/v4/mod.rs` | 33 | ✅ 已实现 |
| 405 | 订阅日程变更事件 | POST | `/open-apis/calendar/v4/calendars/:calendar_id/events/subscription` | `../src/service/calendar/v4/mod.rs` | 535 | ✅ 已实现 |
| 406 | 取消订阅日程变更事件 | POST | `/open-apis/calendar/v4/calendars/:calendar_id/events/unsubscription` | `../src/service/calendar/v4/mod.rs` | 33 | ✅ 已实现 |
| 407 | 回复日程 | POST | `/open-apis/calendar/v4/calendars/:calendar_id/events/:event_id/reply` | `../src/service/calendar/v4/mod.rs` | 33 | ✅ 已实现 |
| 408 | 获取重复日程实例 | GET | `/open-apis/calendar/v4/calendars/:calendar_id/events/:event_id/instances` | `../src/service/calendar/v4/mod.rs` | 33 | ✅ 已实现 |
| 409 | 查询日程视图 | GET | `/open-apis/calendar/v4/calendars/:calendar_id/events/instance_view` | `../src/service/calendar/v4/mod.rs` | 33 | ✅ 已实现 |
| 410 | 创建会议群 | POST | `/open-apis/calendar/v4/calendars/:calendar_id/events/:event_id/meeting_chat` | `../src/service/calendar/v4/mod.rs` | 33 | ✅ 已实现 |
| 411 | 解绑会议群 | DELETE | `/open-apis/calendar/v4/calendars/:calendar_id/events/:event_id/meeting_chat` | `../src/service/calendar/v4/mod.rs` | 33 | ✅ 已实现 |
| 412 | 创建会议纪要 | POST | `/open-apis/calendar/v4/calendars/:calendar_id/events/:event_id/meeting_minute` | `../src/service/calendar/v4/mod.rs` | 33 | ✅ 已实现 |
| 413 | 创建请假日程 | POST | `/open-apis/calendar/v4/timeoff_events` | `../src/service/calendar/v4/mod.rs` | 33 | ✅ 已实现 |
| 414 | 删除请假日程 | DELETE | `/open-apis/calendar/v4/timeoff_events/:timeoff_event_id` | `../src/service/calendar/v4/mod.rs` | 33 | ✅ 已实现 |
| 415 | 查询会议室日程主题和会议详情 | POST | `/open-apis/meeting_room/summary/batch_get` | `../src/service/contact/v3/department.rs` | 212 | ✅ 已实现 |
| 416 | 查询会议室忙闲 | GET | `/open-apis/meeting_room/freebusy/batch_get` | `../src/service/contact/v3/department.rs` | 212 | ✅ 已实现 |
| 417 | 回复会议室日程实例 | POST | `/open-apis/meeting_room/instance/reply` | `../src/service/cloud_docs/comments/mod.rs` | 171 | ✅ 已实现 |
| 418 | 添加日程参与人 | POST | `/open-apis/calendar/v4/calendars/:calendar_id/events/:event_id/attendees` | `../src/service/calendar/v4/mod.rs` | 337 | ✅ 已实现 |
| 419 | 删除日程参与人 | POST | `/open-apis/calendar/v4/calendars/:calendar_id/events/:event_id/attendees/batch_delete` | `../src/service/calendar/v4/mod.rs` | 33 | ✅ 已实现 |
| 420 | 获取日程参与人列表 | GET | `/open-apis/calendar/v4/calendars/:calendar_id/events/:event_id/attendees` | `../src/service/calendar/v4/mod.rs` | 337 | ✅ 已实现 |
| 421 | 获取日程参与群成员列表 | GET | `/open-apis/calendar/v4/calendars/:calendar_id/events/:event_id/attendees/:attendee_id/chat_members` | `../src/service/calendar/v4/mod.rs` | 33 | ✅ 已实现 |
| 422 | 生成 CalDAV 配置 | POST | `/open-apis/calendar/v4/settings/generate_caldav_conf` | `../src/service/calendar/v4/mod.rs` | 33 | ✅ 已实现 |
| 423 | 将 Exchange 账户绑定到飞书账户 | POST | `/open-apis/calendar/v4/exchange_bindings` | `../src/service/calendar/v4/mod.rs` | 33 | ✅ 已实现 |
| 424 | 解除 Exchange 账户绑定 | DELETE | `/open-apis/calendar/v4/exchange_bindings/:exchange_binding_id` | `../src/service/calendar/v4/mod.rs` | 33 | ✅ 已实现 |
| 425 | 查询 Exchange 账户的绑定状态 | GET | `/open-apis/calendar/v4/exchange_bindings/:exchange_binding_id` | `../src/service/calendar/v4/mod.rs` | 33 | ✅ 已实现 |
| 426 | 预约会议 | POST | `/open-apis/vc/v1/reserves/apply` | `../src/service/application/v6/scope/mod.rs` | 31 | ✅ 已实现 |
| 427 | 删除预约 | DELETE | `/open-apis/vc/v1/reserves/:reserve_id` | `未找到` | - | ❌ 未实现 |
| 428 | 更新预约 | PUT | `/open-apis/vc/v1/reserves/:reserve_id` | `未找到` | - | ❌ 未实现 |
| 429 | 获取预约 | GET | `/open-apis/vc/v1/reserves/:reserve_id` | `未找到` | - | ❌ 未实现 |
| 430 | 获取活跃会议 | GET | `/open-apis/vc/v1/reserves/:reserve_id/get_active_meeting` | `未找到` | - | ❌ 未实现 |
| 431 | 邀请参会人 | PATCH | `/open-apis/vc/v1/meetings/:meeting_id/invite` | `未找到` | - | ❌ 未实现 |
| 432 | 移除参会人 | POST | `/open-apis/vc/v1/meetings/:meeting_id/kickout` | `未找到` | - | ❌ 未实现 |
| 433 | 设置主持人 | PATCH | `/open-apis/vc/v1/meetings/:meeting_id/set_host` | `未找到` | - | ❌ 未实现 |
| 434 | 结束会议 | PATCH | `/open-apis/vc/v1/meetings/:meeting_id/end` | `../src/service/calendar/v4/mod.rs` | 33 | ✅ 已实现 |
| 435 | 获取会议详情 | GET | `/open-apis/vc/v1/meetings/:meeting_id` | `未找到` | - | ❌ 未实现 |
| 436 | 获取与会议号关联的会议列表 | GET | `/open-apis/vc/v1/meetings/list_by_no` | `未找到` | - | ❌ 未实现 |
| 437 | 开始录制 | PATCH | `/open-apis/vc/v1/meetings/:meeting_id/recording/start` | `../src/service/vc/v1/recording/mod.rs` | 76 | ✅ 已实现 |
| 438 | 停止录制 | PATCH | `/open-apis/vc/v1/meetings/:meeting_id/recording/stop` | `../src/service/vc/v1/recording/mod.rs` | 100 | ✅ 已实现 |
| 439 | 获取录制文件 | GET | `/open-apis/vc/v1/meetings/:meeting_id/recording` | `未找到` | - | ❌ 未实现 |
| 440 | 授权录制文件 | PATCH | `/open-apis/vc/v1/meetings/:meeting_id/recording/set_permission` | `../src/service/vc/v1/recording/mod.rs` | 136 | ✅ 已实现 |
| 441 | 获取会议报告 | GET | `/open-apis/vc/v1/reports/get_daily` | `未找到` | - | ❌ 未实现 |
| 442 | 获取 Top 用户列表 | GET | `/open-apis/vc/v1/reports/get_top_user` | `未找到` | - | ❌ 未实现 |
| 443 | 导出会议明细 | POST | `/open-apis/vc/v1/exports/meeting_list` | `未找到` | - | ❌ 未实现 |
| 444 | 导出参会人明细 | POST | `/open-apis/vc/v1/exports/participant_list` | `未找到` | - | ❌ 未实现 |
| 445 | 导出参会人会议质量数据 | POST | `/open-apis/vc/v1/exports/participant_quality_list` | `未找到` | - | ❌ 未实现 |
| 446 | 导出会议室预定数据 | POST | `/open-apis/vc/v1/exports/resource_reservation_list` | `未找到` | - | ❌ 未实现 |
| 447 | 查询导出任务结果 | GET | `/open-apis/vc/v1/exports/:task_id` | `未找到` | - | ❌ 未实现 |
| 448 | 下载导出文件 | GET | `/open-apis/vc/v1/exports/download` | `../src/service/attendance/v1/user_setting.rs` | 117 | ✅ 已实现 |
| 449 | 创建会议室层级 | POST | `/open-apis/vc/v1/room_levels` | `未找到` | - | ❌ 未实现 |
| 450 | 删除会议室层级 | POST | `/open-apis/vc/v1/room_levels/del` | `../src/service/vc/v1/room/mod.rs` | 155 | ✅ 已实现 |
| 451 | 更新会议室层级 | PATCH | `/open-apis/vc/v1/room_levels/:room_level_id` | `未找到` | - | ❌ 未实现 |
| 452 | 查询会议室层级详情 | GET | `/open-apis/vc/v1/room_levels/:room_level_id` | `未找到` | - | ❌ 未实现 |
| 453 | 批量查询会议室层级详情 | POST | `/open-apis/vc/v1/room_levels/mget` | `未找到` | - | ❌ 未实现 |
| 454 | 查询会议室层级列表 | GET | `/open-apis/vc/v1/room_levels` | `未找到` | - | ❌ 未实现 |
| 455 | 搜索会议室层级 | GET | `/open-apis/vc/v1/room_levels/search` | `../src/service/vc/v1/room/mod.rs` | 223 | ✅ 已实现 |
| 456 | 创建会议室 | POST | `/open-apis/vc/v1/rooms` | `../src/service/calendar/v4/mod.rs` | 434 | ✅ 已实现 |
| 457 | 删除会议室 | DELETE | `/open-apis/vc/v1/rooms/:room_id` | `未找到` | - | ❌ 未实现 |
| 458 | 更新会议室 | PATCH | `/open-apis/vc/v1/rooms/:room_id` | `未找到` | - | ❌ 未实现 |
| 459 | 查询会议室详情 | GET | `/open-apis/vc/v1/rooms/:room_id` | `未找到` | - | ❌ 未实现 |
| 460 | 批量查询会议室详情 | POST | `/open-apis/vc/v1/rooms/mget` | `未找到` | - | ❌ 未实现 |
| 461 | 查询会议室列表 | GET | `/open-apis/vc/v1/rooms` | `../src/service/calendar/v4/mod.rs` | 434 | ✅ 已实现 |
| 462 | 搜索会议室 | POST | `/open-apis/vc/v1/rooms/search` | `../src/service/vc/v1/room/mod.rs` | 223 | ✅ 已实现 |
| 463 | 查询会议室配置 | GET | `/open-apis/vc/v1/scope_config` | `未找到` | - | ❌ 未实现 |
| 464 | 设置会议室配置 | POST | `/open-apis/vc/v1/scope_config` | `未找到` | - | ❌ 未实现 |
| 465 | 查询会议室预定限制 | GET | `/open-apis/vc/v1/reserve_configs/reserve_scope` | `未找到` | - | ❌ 未实现 |
| 466 | 更新会议室预定限制 | PATCH | `/open-apis/vc/v1/reserve_configs/:reserve_config_id` | `未找到` | - | ❌ 未实现 |
| 467 | 查询会议室预定表单 | GET | `/open-apis/vc/v1/reserve_configs/:reserve_config_id/form` | `../src/service/ehr/v1/mod.rs` | 677 | ✅ 已实现 |
| 468 | 更新会议室预定表单 | PATCH | `/open-apis/vc/v1/reserve_configs/:reserve_config_id/form` | `../src/service/ehr/v1/mod.rs` | 677 | ✅ 已实现 |
| 469 | 查询会议室预定管理员 | GET | `/open-apis/vc/v1/reserve_configs/:reserve_config_id/admin` | `../src/service/trust_party/collaboration_organization/mod.rs` | 218 | ✅ 已实现 |
| 470 | 更新会议室预定管理员 | PATCH | `/open-apis/vc/v1/reserve_configs/:reserve_config_id/admin` | `../src/service/trust_party/collaboration_organization/mod.rs` | 218 | ✅ 已实现 |
| 471 | 查询禁用状态变更通知 | GET | `/open-apis/vc/v1/reserve_configs/:reserve_config_id/disable_inform` | `未找到` | - | ❌ 未实现 |
| 472 | 更新禁用状态变更通知 | PATCH | `/open-apis/vc/v1/reserve_configs/:reserve_config_id/disable_inform` | `未找到` | - | ❌ 未实现 |
| 473 | 查询会议明细 | GET | `/open-apis/vc/v1/meeting_list` | `未找到` | - | ❌ 未实现 |
| 474 | 查询参会人明细 | GET | `/open-apis/vc/v1/participant_list` | `未找到` | - | ❌ 未实现 |
| 475 | 查询参会人会议质量数据 | GET | `/open-apis/vc/v1/participant_quality_list` | `未找到` | - | ❌ 未实现 |
| 476 | 查询会议室预定数据 | GET | `/open-apis/vc/v1/resource_reservation_list` | `未找到` | - | ❌ 未实现 |
| 477 | 获取告警记录 | GET | `/open-apis/vc/v1/alerts` | `未找到` | - | ❌ 未实现 |
| 478 | 下载妙记音视频文件 | GET | `/open-apis/minutes/v1/minutes/:minute_token/media` | `未找到` | - | ❌ 未实现 |
| 479 | 导出妙记文字记录 | GET | `/open-apis/minutes/v1/minutes/:minute_token/transcript` | `未找到` | - | ❌ 未实现 |
| 480 | 获取妙记统计数据 | GET | `/open-apis/minutes/v1/minutes/:minute_token/statistics` | `../src/service/elearning/course_registration/mod.rs` | 203 | ✅ 已实现 |
| 481 | 获取妙记信息 | GET | `/open-apis/minutes/v1/minutes/:minute_token` | `未找到` | - | ❌ 未实现 |
| 482 | 创建班次 | POST | `/open-apis/attendance/v1/shifts` | `../src/service/attendance/v1/mod.rs` | 134 | ✅ 已实现 |
| 483 | 删除班次 | DELETE | `/open-apis/attendance/v1/shifts/:shift_id` | `未找到` | - | ❌ 未实现 |
| 484 | 按 ID 查询班次 | GET | `/open-apis/attendance/v1/shifts/:shift_id` | `未找到` | - | ❌ 未实现 |
| 485 | 按名称查询班次 | POST | `/open-apis/attendance/v1/shifts/query` | `../src/service/attendance/v1/user_task.rs` | 86 | ✅ 已实现 |
| 486 | 查询所有班次 | GET | `/open-apis/attendance/v1/shifts` | `../src/service/attendance/v1/mod.rs` | 134 | ✅ 已实现 |
| 487 | 创建或修改排班表 | POST | `/open-apis/attendance/v1/user_daily_shifts/batch_create` | `../src/service/contact/v3/functional_role_member.rs` | 42 | ✅ 已实现 |
| 488 | 查询排班表 | POST | `/open-apis/attendance/v1/user_daily_shifts/query` | `../src/service/attendance/v1/user_task.rs` | 86 | ✅ 已实现 |
| 489 | 创建或修改临时排班 | POST | `/open-apis/attendance/v1/user_daily_shifts/batch_create_temp` | `未找到` | - | ❌ 未实现 |
| 490 | 查询考勤组下所有成员 | GET | `/open-apis/attendance/v1/groups/:group_id/list_user` | `../src/service/okr/v1/mod.rs` | 296 | ✅ 已实现 |
| 491 | 创建或修改考勤组 | POST | `/open-apis/attendance/v1/groups` | `../src/service/contact/v3/group.rs` | 206 | ✅ 已实现 |
| 492 | 删除考勤组 | DELETE | `/open-apis/attendance/v1/groups/:group_id` | `../src/service/payroll/v1/paygroup.rs` | 53 | ✅ 已实现 |
| 493 | 按 ID 查询考勤组 | GET | `/open-apis/attendance/v1/groups/:group_id` | `../src/service/payroll/v1/paygroup.rs` | 53 | ✅ 已实现 |
| 494 | 按名称查询考勤组 | POST | `/open-apis/attendance/v1/groups/search` | `../src/service/attendance/v1/group.rs` | 154 | ✅ 已实现 |
| 495 | 查询所有考勤组 | GET | `/open-apis/attendance/v1/groups` | `../src/service/contact/v3/group.rs` | 206 | ✅ 已实现 |
| 496 | 修改用户人脸识别信息 | POST | `/open-apis/attendance/v1/user_settings/modify` | `未找到` | - | ❌ 未实现 |
| 497 | 批量查询用户人脸识别信息 | GET | `/open-apis/attendance/v1/user_settings/query` | `../src/service/attendance/v1/user_task.rs` | 86 | ✅ 已实现 |
| 498 | 上传用户人脸识别照片 | POST | `/open-apis/attendance/v1/files/upload` | `../src/service/attendance/v1/user_setting.rs` | 77 | ✅ 已实现 |
| 499 | 下载用户人脸识别照片 | GET | `/open-apis/attendance/v1/files/:file_id/download` | `../src/service/attendance/v1/user_setting.rs` | 117 | ✅ 已实现 |
| 500 | 更新统计设置 | PUT | `/open-apis/attendance/v1/user_stats_views/:user_stats_view_id` | `未找到` | - | ❌ 未实现 |
| 501 | 查询统计表头 | POST | `/open-apis/attendance/v1/user_stats_fields/query` | `../src/service/attendance/v1/user_task.rs` | 86 | ✅ 已实现 |
| 502 | 查询统计设置 | POST | `/open-apis/attendance/v1/user_stats_views/query` | `../src/service/attendance/v1/user_task.rs` | 86 | ✅ 已实现 |
| 503 | 查询统计数据 | POST | `/open-apis/attendance/v1/user_stats_datas/query` | `../src/service/attendance/v1/user_task.rs` | 86 | ✅ 已实现 |
| 504 | 获取审批数据 | POST | `/open-apis/attendance/v1/user_approvals/query` | `../src/service/attendance/v1/user_task.rs` | 86 | ✅ 已实现 |
| 505 | 写入审批结果 | POST | `/open-apis/attendance/v1/user_approvals` | `未找到` | - | ❌ 未实现 |
| 506 | 通知审批状态更新 | POST | `/open-apis/attendance/v1/approval_infos/process` | `../src/service/attendance/v1/user_approval.rs` | 85 | ✅ 已实现 |
| 507 | 通知补卡审批发起 | POST | `/open-apis/attendance/v1/user_task_remedys` | `未找到` | - | ❌ 未实现 |
| 508 | 获取可补卡时间 | POST | `/open-apis/attendance/v1/user_task_remedys/query_user_allowed_remedys` | `未找到` | - | ❌ 未实现 |
| 509 | 获取补卡记录 | POST | `/open-apis/attendance/v1/user_task_remedys/query` | `../src/service/attendance/v1/user_task.rs` | 86 | ✅ 已实现 |
| 510 | 查询归档报表表头 | POST | `/open-apis/attendance/v1/archive_rule/user_stats_fields_query` | `未找到` | - | ❌ 未实现 |
| 511 | 写入归档报表结果 | POST | `/open-apis/attendance/v1/archive_rule/upload_report` | `未找到` | - | ❌ 未实现 |
| 512 | 删除归档报表行数据 | POST | `/open-apis/attendance/v1/archive_rule/del_report` | `未找到` | - | ❌ 未实现 |
| 513 | 查询所有归档规则 | GET | `/open-apis/attendance/v1/archive_rule` | `未找到` | - | ❌ 未实现 |
| 514 | 导入打卡流水 | POST | `/open-apis/attendance/v1/user_flows/batch_create` | `../src/service/contact/v3/functional_role_member.rs` | 42 | ✅ 已实现 |
| 515 | 查询打卡流水 | GET | `/open-apis/attendance/v1/user_flows/:user_flow_id` | `未找到` | - | ❌ 未实现 |
| 516 | 批量查询打卡流水 | POST | `/open-apis/attendance/v1/user_flows/query` | `../src/service/attendance/v1/user_task.rs` | 86 | ✅ 已实现 |
| 517 | 删除打卡流水 | POST | `/open-apis/attendance/v1/user_flows/batch_del` | `../src/service/attendance/v1/user_task.rs` | 58 | ✅ 已实现 |
| 518 | 查询打卡结果 | POST | `/open-apis/attendance/v1/user_tasks/query` | `../src/service/attendance/v1/user_task.rs` | 86 | ✅ 已实现 |
| 519 | 通过过期时间获取发放记录 | GET | `/open-apis/attendance/v1/leave_employ_expire_records/:leave_id` | `未找到` | - | ❌ 未实现 |
| 520 | 修改发放记录 | PATCH | `/open-apis/attendance/v1/leave_accrual_record/:leave_id` | `未找到` | - | ❌ 未实现 |
| 521 | 创建审批定义 | POST | `/open-apis/approval/v4/approvals` | `../src/service/approval/v4/instance.rs` | 262 | ✅ 已实现 |
| 522 | 查看指定审批定义 | GET | `/open-apis/approval/v4/approvals/:approval_id` | `../src/service/approval/v4/search/mod.rs` | 361 | ✅ 已实现 |
| 523 | 创建审批实例 | POST | `/open-apis/approval/v4/instances` | `../src/service/approval/v4/search/mod.rs` | 212 | ✅ 已实现 |
| 524 | 撤回审批实例 | POST | `/open-apis/approval/v4/instances/cancel` | `../src/service/approval/v4/instance.rs` | 262 | ✅ 已实现 |
| 525 | 抄送审批实例 | POST | `/open-apis/approval/v4/instances/cc` | `../src/service/approval/v4/search/mod.rs` | 311 | ✅ 已实现 |
| 526 | 预览审批流程 | POST | `/open-apis/approval/v4/instances/preview` | `../src/service/approval/v4/instance.rs` | 262 | ✅ 已实现 |
| 527 | 获取单个审批实例详情 | GET | `/open-apis/approval/v4/instances/:instance_id` | `../src/service/approval/v4/instance.rs` | 262 | ✅ 已实现 |
| 528 | 批量获取审批实例 ID | GET | `/open-apis/approval/v4/instances` | `../src/service/approval/v4/search/mod.rs` | 212 | ✅ 已实现 |
| 529 | 同意审批任务 | POST | `/open-apis/approval/v4/tasks/approve` | `../src/service/approval/v4/task.rs` | 59 | ✅ 已实现 |
| 530 | 拒绝审批任务 | POST | `/open-apis/approval/v4/tasks/reject` | `../src/service/approval/v4/task.rs` | 76 | ✅ 已实现 |
| 531 | 转交审批任务 | POST | `/open-apis/approval/v4/tasks/transfer` | `../src/service/approval/v4/task.rs` | 93 | ✅ 已实现 |
| 532 | 退回审批任务 | POST | `/open-apis/approval/v4/instances/specified_rollback` | `../src/service/approval/v4/instance.rs` | 262 | ✅ 已实现 |
| 533 | 审批任务加签 | POST | `/open-apis/approval/v4/instances/add_sign` | `../src/service/approval/v4/instance.rs` | 262 | ✅ 已实现 |
| 534 | 重新提交审批任务 | POST | `/open-apis/approval/v4/tasks/resubmit` | `../src/service/approval/v4/instance.rs` | 262 | ✅ 已实现 |
| 535 | 上传文件 | POST | `/approval/openapi/v2/file/upload` | `../src/service/approval/v4/file/mod.rs` | 49 | ✅ 已实现 |
| 536 | 创建评论 | POST | `/open-apis/approval/v4/instances/:instance_id/comments` | `../src/service/approval/v4/instance.rs` | 262 | ✅ 已实现 |
| 537 | 删除评论 | DELETE | `/open-apis/approval/v4/instances/:instance_id/comments/:comment_id` | `../src/service/approval/v4/instance.rs` | 262 | ✅ 已实现 |
| 538 | 清空评论 | POST | `/open-apis/approval/v4/instances/:instance_id/comments/remove` | `../src/service/approval/v4/instance_comment/mod.rs` | 105 | ✅ 已实现 |
| 539 | 获取评论 | GET | `/open-apis/approval/v4/instances/:instance_id/comments` | `../src/service/approval/v4/instance.rs` | 262 | ✅ 已实现 |
| 540 | 创建三方审批定义 | POST | `/open-apis/approval/v4/external_approvals` | `../src/service/approval/v4/instance.rs` | 262 | ✅ 已实现 |
| 541 | 查看指定三方审批定义 | GET | `/open-apis/approval/v4/external_approvals/:approval_code` | `../src/service/approval/v4/approval.rs` | 54 | ✅ 已实现 |
| 542 | 三方快捷审批回调 | POST | `/approval/openapi/v2/external/instanceOperate` | `../src/service/approval/v4/instance.rs` | 262 | ✅ 已实现 |
| 543 | 同步三方审批实例 | POST | `/open-apis/approval/v4/external_instances` | `../src/service/approval/v4/instance.rs` | 262 | ✅ 已实现 |
| 544 | 校验三方审批实例 | POST | `/open-apis/approval/v4/external_instances/check` | `../src/service/approval/v4/external_instance/mod.rs` | 70 | ✅ 已实现 |
| 545 | 获取三方审批任务状态 | GET | `/open-apis/approval/v4/external_tasks` | `../src/service/approval/v4/instance.rs` | 262 | ✅ 已实现 |
| 546 | 发送审批 Bot 消息 | POST | `/approval/openapi/v1/message/send` | `../src/service/approval/v4/message/mod.rs` | 52 | ✅ 已实现 |
| 547 | 更新审批 Bot 消息 | POST | `/approval/openapi/v1/message/update` | `../src/service/approval/v4/message/mod.rs` | 78 | ✅ 已实现 |
| 548 | 查询实例列表 | POST | `/open-apis/approval/v4/instances/query` | `../src/service/approval/v4/instance.rs` | 109 | ✅ 已实现 |
| 549 | 查询抄送列表 | POST | `/open-apis/approval/v4/instances/search_cc` | `../src/service/approval/v4/search/mod.rs` | 311 | ✅ 已实现 |
| 550 | 查询任务列表 | POST | `/open-apis/approval/v4/tasks/search` | `../src/service/approval/v4/search/mod.rs` | 212 | ✅ 已实现 |
| 551 | 查询用户的任务列表 | GET | `/open-apis/approval/v4/tasks/query` | `../src/service/approval/v4/instance.rs` | 109 | ✅ 已实现 |
| 552 | 查询审批 ID（专用） | POST | `/approval/openapi/v1/id/get` | `../src/service/approval/v4/instance.rs` | 58 | ✅ 已实现 |
| 553 | 订阅审批事件 | POST | `/open-apis/approval/v4/approvals/:approval_code/subscribe` | `../src/service/approval/v4/instance.rs` | 262 | ✅ 已实现 |
| 554 | 取消订阅审批事件 | POST | `/open-apis/approval/v4/approvals/:approval_code/unsubscribe` | `../src/service/approval/v4/instance.rs` | 262 | ✅ 已实现 |
| 555 | 更新客服信息 | PATCH | `/open-apis/helpdesk/v1/agents/:agent_id` | `未找到` | - | ❌ 未实现 |
| 556 | 获取客服邮箱 | GET | `/open-apis/helpdesk/v1/agent_emails` | `未找到` | - | ❌ 未实现 |
| 557 | 创建客服工作日程 | POST | `/open-apis/helpdesk/v1/agent_schedules` | `未找到` | - | ❌ 未实现 |
| 558 | 删除客服工作日程 | DELETE | `/open-apis/helpdesk/v1/agents/:agent_id/schedules` | `未找到` | - | ❌ 未实现 |
| 559 | 更新客服工作日程 | PATCH | `/open-apis/helpdesk/v1/agents/:agent_id/schedules` | `未找到` | - | ❌ 未实现 |
| 560 | 查询指定客服工作日程 | GET | `/open-apis/helpdesk/v1/agents/:agent_id/schedules` | `未找到` | - | ❌ 未实现 |
| 561 | 查询全部客服工作日程 | GET | `/open-apis/helpdesk/v1/agent_schedules` | `未找到` | - | ❌ 未实现 |
| 562 | 创建客服技能 | POST | `/open-apis/helpdesk/v1/agent_skills` | `未找到` | - | ❌ 未实现 |
| 563 | 删除客服技能 | DELETE | `/open-apis/helpdesk/v1/agent_skills/:agent_skill_id` | `未找到` | - | ❌ 未实现 |
| 564 | 更新客服技能 | PATCH | `/open-apis/helpdesk/v1/agent_skills/:agent_skill_id` | `未找到` | - | ❌ 未实现 |
| 565 | 查询指定客服技能 | GET | `/open-apis/helpdesk/v1/agent_skills/:agent_skill_id` | `未找到` | - | ❌ 未实现 |
| 566 | 查询全部客服技能 | GET | `/open-apis/helpdesk/v1/agent_skills` | `未找到` | - | ❌ 未实现 |
| 567 | 获取客服技能列表 | GET | `/open-apis/helpdesk/v1/agent_skill_rules` | `未找到` | - | ❌ 未实现 |
| 568 | 创建服务台对话 | POST | `/open-apis/helpdesk/v1/start_service` | `未找到` | - | ❌ 未实现 |
| 569 | 查询指定工单详情 | GET | `/open-apis/helpdesk/v1/tickets/:ticket_id` | `未找到` | - | ❌ 未实现 |
| 570 | 更新工单详情 | PUT | `/open-apis/helpdesk/v1/tickets/:ticket_id` | `未找到` | - | ❌ 未实现 |
| 571 | 查询全部工单详情 | GET | `/open-apis/helpdesk/v1/tickets` | `未找到` | - | ❌ 未实现 |
| 572 | 获取工单内图像 | GET | `/open-apis/helpdesk/v1/ticket_images` | `未找到` | - | ❌ 未实现 |
| 573 | 回复用户在工单里的提问 | POST | `/open-apis/helpdesk/v1/tickets/:ticket_id/answer_user_query` | `未找到` | - | ❌ 未实现 |
| 574 | 获取服务台自定义字段 | GET | `/open-apis/helpdesk/v1/customized_fields` | `未找到` | - | ❌ 未实现 |
| 575 | 发送工单消息 | POST | `/open-apis/helpdesk/v1/tickets/:ticket_id/messages` | `../src/service/helpdesk/v1/ticket_message/mod.rs` | 169 | ✅ 已实现 |
| 576 | 获取工单消息详情 | GET | `/open-apis/helpdesk/v1/tickets/:ticket_id/messages` | `../src/service/helpdesk/v1/ticket_message/mod.rs` | 169 | ✅ 已实现 |
| 577 | 服务台机器人向工单绑定的群内发送消息 | POST | `/open-apis/helpdesk/v1/message` | `../src/service/helpdesk/v1/ticket_message/mod.rs` | 169 | ✅ 已实现 |
| 578 | 创建工单自定义字段 | POST | `/open-apis/helpdesk/v1/ticket_customized_fields` | `未找到` | - | ❌ 未实现 |
| 579 | 删除工单自定义字段 | DELETE | `/open-apis/helpdesk/v1/ticket_customized_fields/:ticket_customized_field_id` | `未找到` | - | ❌ 未实现 |
| 580 | 更新工单自定义字段 | PATCH | `/open-apis/helpdesk/v1/ticket_customized_fields/:ticket_customized_field_id` | `未找到` | - | ❌ 未实现 |
| 581 | 获取指定工单自定义字段 | GET | `/open-apis/helpdesk/v1/ticket_customized_fields/:ticket_customized_field_id` | `未找到` | - | ❌ 未实现 |
| 582 | 获取全部工单自定义字段 | GET | `/open-apis/helpdesk/v1/ticket_customized_fields` | `未找到` | - | ❌ 未实现 |
| 583 | 创建知识库 | POST | `/open-apis/helpdesk/v1/faqs` | `未找到` | - | ❌ 未实现 |
| 584 | 删除知识库 | DELETE | `/open-apis/helpdesk/v1/faqs/:id` | `../src/service/contact/v3/job_family.rs` | 66 | ✅ 已实现 |
| 585 | 修改知识库 | PATCH | `/open-apis/helpdesk/v1/faqs/:id` | `../src/service/contact/v3/job_family.rs` | 66 | ✅ 已实现 |
| 586 | 获取指定知识库详情 | GET | `/open-apis/helpdesk/v1/faqs/:id` | `../src/service/contact/v3/job_family.rs` | 66 | ✅ 已实现 |
| 587 | 获取全部知识库详情 | GET | `/open-apis/helpdesk/v1/faqs` | `未找到` | - | ❌ 未实现 |
| 588 | 获取知识库图像 | GET | `/open-apis/helpdesk/v1/faqs/:id/image/:image_key` | `未找到` | - | ❌ 未实现 |
| 589 | 搜索知识库 | GET | `/open-apis/helpdesk/v1/faqs/search` | `../src/service/helpdesk/v1/faq/mod.rs` | 328 | ✅ 已实现 |
| 590 | 创建知识库分类 | POST | `/open-apis/helpdesk/v1/categories` | `../src/service/aily/knowledge/mod.rs` | 284 | ✅ 已实现 |
| 591 | 获取知识库分类 | GET | `/open-apis/helpdesk/v1/categories/:id` | `../src/service/contact/v3/job_family.rs` | 66 | ✅ 已实现 |
| 592 | 更新知识库分类详情 | PATCH | `/open-apis/helpdesk/v1/categories/:id` | `../src/service/contact/v3/job_family.rs` | 66 | ✅ 已实现 |
| 593 | 删除知识库分类详情 | DELETE | `/open-apis/helpdesk/v1/categories/:id` | `../src/service/contact/v3/job_family.rs` | 66 | ✅ 已实现 |
| 594 | 获取全部知识库分类 | GET | `/open-apis/helpdesk/v1/categories` | `../src/service/aily/knowledge/mod.rs` | 284 | ✅ 已实现 |
| 595 | 创建推送 | POST | `/open-apis/helpdesk/v1/notifications` | `未找到` | - | ❌ 未实现 |
| 596 | 更新推送 | PATCH | `/open-apis/helpdesk/v1/notifications/:notification_id` | `未找到` | - | ❌ 未实现 |
| 597 | 查询推送 | GET | `/open-apis/helpdesk/v1/notifications/:notification_id` | `未找到` | - | ❌ 未实现 |
| 598 | 预览推送 | POST | `/open-apis/helpdesk/v1/notifications/:notification_id/preview` | `../src/service/helpdesk/v1/notification/mod.rs` | 151 | ✅ 已实现 |
| 599 | 提交审核 | POST | `/open-apis/helpdesk/v1/notifications/:notification_id/submit_approve` | `../src/service/helpdesk/v1/notification/mod.rs` | 173 | ✅ 已实现 |
| 600 | 取消审核 | POST | `/open-apis/helpdesk/v1/notifications/:notification_id/cancel_approve` | `../src/service/helpdesk/v1/notification/mod.rs` | 195 | ✅ 已实现 |
| 601 | 执行推送 | POST | `/open-apis/helpdesk/v1/notifications/:notification_id/execute_send` | `../src/service/helpdesk/v1/notification/mod.rs` | 217 | ✅ 已实现 |
| 602 | 取消推送 | POST | `/open-apis/helpdesk/v1/notifications/:notification_id/cancel_send` | `../src/service/helpdesk/v1/notification/mod.rs` | 239 | ✅ 已实现 |
| 603 | 订阅服务台事件 | POST | `/open-apis/helpdesk/v1/events/subscribe` | `../src/service/helpdesk/v1/event/mod.rs` | 55 | ✅ 已实现 |
| 604 | 取消订阅服务台事件 | POST | `/open-apis/helpdesk/v1/events/unsubscribe` | `../src/service/helpdesk/v1/event/mod.rs` | 88 | ✅ 已实现 |
| 605 | 创建任务 | POST | `/open-apis/task/v2/tasks` | `../src/service/task/v1/mod.rs` | 54 | ✅ 已实现 |
| 606 | 更新任务 | PATCH | `/open-apis/task/v2/tasks/:task_guid` | `../src/service/task/v1/mod.rs` | 33 | ✅ 已实现 |
| 607 | 获取任务详情 | GET | `/open-apis/task/v2/tasks/:task_guid` | `../src/service/task/v1/mod.rs` | 33 | ✅ 已实现 |
| 608 | 删除任务 | DELETE | `/open-apis/task/v2/tasks/:task_guid` | `../src/service/task/v1/mod.rs` | 33 | ✅ 已实现 |
| 609 | 添加任务成员 | POST | `/open-apis/task/v2/tasks/:task_guid/add_members` | `../src/service/task/v2/tasklist/mod.rs` | 208 | ✅ 已实现 |
| 610 | 移除任务成员 | POST | `/open-apis/task/v2/tasks/:task_guid/remove_members` | `../src/service/task/v2/tasklist/mod.rs` | 228 | ✅ 已实现 |
| 611 | 列取任务列表 | GET | `/open-apis/task/v2/tasks` | `../src/service/task/v1/mod.rs` | 54 | ✅ 已实现 |
| 612 | 列取任务所在清单 | GET | `/open-apis/task/v2/tasks/:task_guid/tasklists` | `../src/service/task/v2/mod.rs` | 215 | ✅ 已实现 |
| 613 | 任务加入清单 | POST | `/open-apis/task/v2/tasks/:task_guid/add_tasklist` | `../src/service/task/v2/mod.rs` | 286 | ✅ 已实现 |
| 614 | 任务移出清单 | POST | `/open-apis/task/v2/tasks/:task_guid/remove_tasklist` | `../src/service/task/v2/mod.rs` | 298 | ✅ 已实现 |
| 615 | 添加任务提醒 | POST | `/open-apis/task/v2/tasks/:task_guid/add_reminders` | `../src/service/task/v2/task/mod.rs` | 412 | ✅ 已实现 |
| 616 | 移除任务提醒 | POST | `/open-apis/task/v2/tasks/:task_guid/remove_reminders` | `../src/service/task/v2/task/mod.rs` | 458 | ✅ 已实现 |
| 617 | 添加依赖 | POST | `/open-apis/task/v2/tasks/:task_guid/add_dependencies` | `../src/service/task/v2/task/mod.rs` | 481 | ✅ 已实现 |
| 618 | 移除依赖 | POST | `/open-apis/task/v2/tasks/:task_guid/remove_dependencies` | `../src/service/task/v2/task/mod.rs` | 504 | ✅ 已实现 |
| 619 | 创建子任务 | POST | `/open-apis/task/v2/tasks/:task_guid/subtasks` | `../src/service/task/v1/mod.rs` | 33 | ✅ 已实现 |
| 620 | 获取任务的子任务列表 | GET | `/open-apis/task/v2/tasks/:task_guid/subtasks` | `../src/service/task/v1/mod.rs` | 33 | ✅ 已实现 |
| 621 | 创建清单 | POST | `/open-apis/task/v2/tasklists` | `../src/service/task/v2/mod.rs` | 215 | ✅ 已实现 |
| 622 | 获取清单详情 | GET | `/open-apis/task/v2/tasklists/:tasklist_guid` | `../src/service/task/v1/mod.rs` | 33 | ✅ 已实现 |
| 623 | 更新清单 | PATCH | `/open-apis/task/v2/tasklists/:tasklist_guid` | `../src/service/task/v1/mod.rs` | 33 | ✅ 已实现 |
| 624 | 删除清单 | DELETE | `/open-apis/task/v2/tasklists/:tasklist_guid` | `../src/service/task/v1/mod.rs` | 33 | ✅ 已实现 |
| 625 | 添加清单成员 | POST | `/open-apis/task/v2/tasklists/:tasklist_guid/add_members` | `../src/service/task/v2/tasklist/mod.rs` | 208 | ✅ 已实现 |
| 626 | 移除清单成员 | POST | `/open-apis/task/v2/tasklists/:tasklist_guid/remove_members` | `../src/service/task/v2/tasklist/mod.rs` | 228 | ✅ 已实现 |
| 627 | 获取清单任务列表 | GET | `/open-apis/task/v2/tasklists/:tasklist_guid/tasks` | `../src/service/task/v1/mod.rs` | 54 | ✅ 已实现 |
| 628 | 获取清单列表 | GET | `/open-apis/task/v2/tasklists` | `../src/service/task/v2/mod.rs` | 215 | ✅ 已实现 |
| 629 | 创建动态订阅 | POST | `/open-apis/task/v2/tasklists/:tasklist_guid/activity_subscriptions` | `../src/service/task/v1/mod.rs` | 33 | ✅ 已实现 |
| 630 | 获取动态订阅 | GET | `/open-apis/task/v2/tasklists/:tasklist_guid/activity_subscriptions/:activity_subscription_guid` | `../src/service/task/v1/mod.rs` | 33 | ✅ 已实现 |
| 631 | 列取动态订阅 | GET | `/open-apis/task/v2/tasklists/:tasklist_guid/activity_subscriptions` | `../src/service/task/v1/mod.rs` | 33 | ✅ 已实现 |
| 632 | 更新动态订阅 | PATCH | `/open-apis/task/v2/tasklists/:tasklist_guid/activity_subscriptions/:activity_subscription_guid` | `../src/service/task/v1/mod.rs` | 33 | ✅ 已实现 |
| 633 | 删除动态订阅 | DELETE | `/open-apis/task/v2/tasklists/:tasklist_guid/activity_subscriptions/:activity_subscription_guid` | `../src/service/task/v1/mod.rs` | 33 | ✅ 已实现 |
| 634 | 创建评论 | POST | `/open-apis/task/v2/comments` | `../src/service/task/v2/mod.rs` | 338 | ✅ 已实现 |
| 635 | 获取评论详情 | GET | `/open-apis/task/v2/comments/:comment_id` | `../src/service/task/v1/mod.rs` | 33 | ✅ 已实现 |
| 636 | 更新评论 | PATCH | `/open-apis/task/v2/comments/:comment_id` | `../src/service/task/v1/mod.rs` | 33 | ✅ 已实现 |
| 637 | 删除评论 | DELETE | `/open-apis/task/v2/comments/:comment_id` | `../src/service/task/v1/mod.rs` | 33 | ✅ 已实现 |
| 638 | 获取评论列表 | GET | `/open-apis/task/v2/comments` | `../src/service/task/v2/mod.rs` | 338 | ✅ 已实现 |
| 639 | 上传附件 | POST | `/open-apis/task/v2/attachments/upload` | `../src/service/task/v2/mod.rs` | 396 | ✅ 已实现 |
| 640 | 列取附件 | GET | `/open-apis/task/v2/attachments` | `../src/service/task/v2/mod.rs` | 422 | ✅ 已实现 |
| 641 | 获取附件 | GET | `/open-apis/task/v2/attachments/:attachment_guid` | `../src/service/task/v1/mod.rs` | 33 | ✅ 已实现 |
| 642 | 删除附件 | DELETE | `/open-apis/task/v2/attachments/:attachment_guid` | `../src/service/task/v1/mod.rs` | 33 | ✅ 已实现 |
| 643 | 创建自定义分组 | POST | `/open-apis/task/v2/sections` | `../src/service/task/v1/mod.rs` | 33 | ✅ 已实现 |
| 644 | 获取自定义分组详情 | GET | `/open-apis/task/v2/sections/:section_guid` | `../src/service/task/v1/mod.rs` | 33 | ✅ 已实现 |
| 645 | 更新自定义分组 | PATCH | `/open-apis/task/v2/sections/:section_guid` | `../src/service/task/v1/mod.rs` | 33 | ✅ 已实现 |
| 646 | 删除自定义分组 | DELETE | `/open-apis/task/v2/sections/:section_guid` | `../src/service/task/v1/mod.rs` | 33 | ✅ 已实现 |
| 647 | 获取自定义分组列表 | GET | `/open-apis/task/v2/sections` | `../src/service/task/v1/mod.rs` | 33 | ✅ 已实现 |
| 648 | 获取自定义分组任务列表 | GET | `/open-apis/task/v2/sections/:section_guid/tasks` | `../src/service/task/v1/mod.rs` | 54 | ✅ 已实现 |
| 649 | 创建自定义字段 | POST | `/open-apis/task/v2/custom_fields` | `../src/service/task/v2/mod.rs` | 489 | ✅ 已实现 |
| 650 | 获取自定义字段 | GET | `/open-apis/task/v2/custom_fields/:custom_field_guid` | `../src/service/task/v1/mod.rs` | 33 | ✅ 已实现 |
| 651 | 更新自定义字段 | PATCH | `/open-apis/task/v2/custom_fields/:custom_field_guid` | `../src/service/task/v1/mod.rs` | 33 | ✅ 已实现 |
| 652 | 列取自定义字段 | GET | `/open-apis/task/v2/custom_fields` | `../src/service/task/v2/mod.rs` | 489 | ✅ 已实现 |
| 653 | 将自定义字段加入资源 | POST | `/open-apis/task/v2/custom_fields/:custom_field_guid/add` | `../src/service/task/v2/custom_field/mod.rs` | 184 | ✅ 已实现 |
| 654 | 将自定义字段移出资源 | POST | `/open-apis/task/v2/custom_fields/:custom_field_guid/remove` | `../src/service/task/v2/custom_field/mod.rs` | 208 | ✅ 已实现 |
| 655 | 创建自定义任务选项 | POST | `/open-apis/task/v2/custom_fields/:custom_field_guid/options` | `../src/service/task/v1/mod.rs` | 33 | ✅ 已实现 |
| 656 | 更新自定义字段选项 | PATCH | `/open-apis/task/v2/custom_fields/:custom_field_guid/options/:option_guid` | `../src/service/task/v1/mod.rs` | 33 | ✅ 已实现 |
| 657 | 创建邮箱文件夹 | POST | `/open-apis/mail/v1/user_mailboxes/:user_mailbox_id/folders` | `未找到` | - | ❌ 未实现 |
| 658 | 删除邮箱文件夹 | DELETE | `/open-apis/mail/v1/user_mailboxes/:user_mailbox_id/folders/:folder_id` | `未找到` | - | ❌ 未实现 |
| 659 | 修改邮箱文件夹 | PATCH | `/open-apis/mail/v1/user_mailboxes/:user_mailbox_id/folders/:folder_id` | `未找到` | - | ❌ 未实现 |
| 660 | 列出邮箱文件夹 | GET | `/open-apis/mail/v1/user_mailboxes/:user_mailbox_id/folders` | `未找到` | - | ❌ 未实现 |
| 661 | 获取邮件卡片的邮件列表 | GET | `/open-apis/mail/v1/user_mailboxes/:user_mailbox_id/messages/get_by_card` | `../src/service/mail/v1/message/mod.rs` | 237 | ✅ 已实现 |
| 662 | 列出邮件 | GET | `/open-apis/mail/v1/user_mailboxes/:user_mailbox_id/messages` | `../src/service/aily/message/mod.rs` | 127 | ✅ 已实现 |
| 663 | 获取邮件详情 | GET | `/open-apis/mail/v1/user_mailboxes/:user_mailbox_id/messages/:message_id` | `未找到` | - | ❌ 未实现 |
| 664 | 发送邮件 | POST | `/open-apis/mail/v1/user_mailboxes/:user_mailbox_id/messages/send` | `../src/service/mail/v1/message/mod.rs` | 106 | ✅ 已实现 |
| 665 | 获取附件下载链接 | GET | `/open-apis/mail/v1/user_mailboxes/:user_mailbox_id/messages/:message_id/attachments/download_url` | `../src/service/mail/v1/attachment/mod.rs` | 52 | ✅ 已实现 |
| 666 | 订阅事件 | POST | `/open-apis/mail/v1/user_mailboxes/:user_mailbox_id/event/subscribe` | `../src/service/mail/v1/event/mod.rs` | 51 | ✅ 已实现 |
| 667 | 获取订阅状态 | GET | `/open-apis/mail/v1/user_mailboxes/:user_mailbox_id/event/subscription` | `../src/service/mail/v1/event/mod.rs` | 75 | ✅ 已实现 |
| 668 | 取消订阅 | POST | `/open-apis/mail/v1/user_mailboxes/:user_mailbox_id/event/unsubscribe` | `../src/service/mail/v1/event/mod.rs` | 97 | ✅ 已实现 |
| 669 | 创建收信规则 | POST | `/open-apis/mail/v1/user_mailboxes/:user_mailbox_id/rules` | `../src/service/workplace/app_recommend/mod.rs` | 125 | ✅ 已实现 |
| 670 | 删除收信规则 | DELETE | `/open-apis/mail/v1/user_mailboxes/:user_mailbox_id/rules/:rule_id` | `未找到` | - | ❌ 未实现 |
| 671 | 更新收信规则 | PUT | `/open-apis/mail/v1/user_mailboxes/:user_mailbox_id/rules/:rule_id` | `未找到` | - | ❌ 未实现 |
| 672 | 列出收信规则 | GET | `/open-apis/mail/v1/user_mailboxes/:user_mailbox_id/rules` | `../src/service/workplace/app_recommend/mod.rs` | 125 | ✅ 已实现 |
| 673 | 对收信规则进行排序 | POST | `/open-apis/mail/v1/user_mailboxes/:user_mailbox_id/rules/reorder` | `../src/service/mail/v1/rule/mod.rs` | 186 | ✅ 已实现 |
| 674 | 创建邮箱联系人 | POST | `/open-apis/mail/v1/user_mailboxes/:user_mailbox_id/mail_contacts` | `未找到` | - | ❌ 未实现 |
| 675 | 删除邮箱联系人 | DELETE | `/open-apis/mail/v1/user_mailboxes/:user_mailbox_id/mail_contacts/:mail_contact_id` | `未找到` | - | ❌ 未实现 |
| 676 | 修改邮箱联系人信息 | PATCH | `/open-apis/mail/v1/user_mailboxes/:user_mailbox_id/mail_contacts/:mail_contact_id` | `未找到` | - | ❌ 未实现 |
| 677 | 列出邮箱联系人 | GET | `/open-apis/mail/v1/user_mailboxes/:user_mailbox_id/mail_contacts` | `未找到` | - | ❌ 未实现 |
| 678 | 创建邮件组 | POST | `/open-apis/mail/v1/mailgroups` | `未找到` | - | ❌ 未实现 |
| 679 | 删除邮件组 | DELETE | `/open-apis/mail/v1/mailgroups/:mailgroup_id` | `未找到` | - | ❌ 未实现 |
| 680 | 修改邮件组部分信息 | PATCH | `/open-apis/mail/v1/mailgroups/:mailgroup_id` | `未找到` | - | ❌ 未实现 |
| 681 | 修改邮件组全部信息 | PUT | `/open-apis/mail/v1/mailgroups/:mailgroup_id` | `未找到` | - | ❌ 未实现 |
| 682 | 查询指定邮件组 | GET | `/open-apis/mail/v1/mailgroups/:mailgroup_id` | `未找到` | - | ❌ 未实现 |
| 683 | 批量获取邮件组 | GET | `/open-apis/mail/v1/mailgroups` | `未找到` | - | ❌ 未实现 |
| 684 | 批量创建邮件组管理员 | POST | `/open-apis/mail/v1/mailgroups/:mailgroup_id/managers/batch_create` | `../src/service/mail/v1/mail_group_manager/mod.rs` | 51 | ✅ 已实现 |
| 685 | 批量删除邮件组管理员 | POST | `/open-apis/mail/v1/mailgroups/:mailgroup_id/managers/batch_delete` | `../src/service/mail/v1/mail_group_manager/mod.rs` | 75 | ✅ 已实现 |
| 686 | 批量获取邮件组管理员 | GET | `/open-apis/mail/v1/mailgroups/:mailgroup_id/managers` | `未找到` | - | ❌ 未实现 |
| 687 | 创建邮件组成员 | POST | `/open-apis/mail/v1/mailgroups/:mailgroup_id/members` | `../src/service/task/v2/tasklist/mod.rs` | 208 | ✅ 已实现 |
| 688 | 删除邮件组成员 | DELETE | `/open-apis/mail/v1/mailgroups/:mailgroup_id/members/:member_id` | `未找到` | - | ❌ 未实现 |
| 689 | 查询指定邮件组成员 | GET | `/open-apis/mail/v1/mailgroups/:mailgroup_id/members/:member_id` | `未找到` | - | ❌ 未实现 |
| 690 | 获取所有邮件组成员 | GET | `/open-apis/mail/v1/mailgroups/:mailgroup_id/members` | `../src/service/task/v2/tasklist/mod.rs` | 208 | ✅ 已实现 |
| 691 | 批量创建邮件组成员 | POST | `/open-apis/mail/v1/mailgroups/:mailgroup_id/members/batch_create` | `../src/service/mail/v1/mail_group_manager/mod.rs` | 51 | ✅ 已实现 |
| 692 | 批量删除邮件组成员 | DELETE | `/open-apis/mail/v1/mailgroups/:mailgroup_id/members/batch_delete` | `../src/service/mail/v1/mail_group_manager/mod.rs` | 75 | ✅ 已实现 |
| 693 | 创建邮件组别名 | POST | `/open-apis/mail/v1/mailgroups/:mailgroup_id/aliases` | `未找到` | - | ❌ 未实现 |
| 694 | 删除邮件组别名 | DELETE | `/open-apis/mail/v1/mailgroups/:mailgroup_id/aliases/:alias_id` | `未找到` | - | ❌ 未实现 |
| 695 | 获取邮件组所有别名 | GET | `/open-apis/mail/v1/mailgroups/:mailgroup_id/aliases` | `未找到` | - | ❌ 未实现 |
| 696 | 创建邮件组权限成员 | POST | `/open-apis/mail/v1/mailgroups/:mailgroup_id/permission_members` | `../src/service/cloud_docs/permission/member/list.rs` | 82 | ✅ 已实现 |
| 697 | 删除邮件组权限成员 | DELETE | `/open-apis/mail/v1/mailgroups/:mailgroup_id/permission_members/:permission_member_id` | `未找到` | - | ❌ 未实现 |
| 698 | 获取邮件组权限成员 | GET | `/open-apis/mail/v1/mailgroups/:mailgroup_id/permission_members/:permission_member_id` | `未找到` | - | ❌ 未实现 |
| 699 | 批量获取邮件组权限成员 | GET | `/open-apis/mail/v1/mailgroups/:mailgroup_id/permission_members` | `../src/service/cloud_docs/permission/member/list.rs` | 82 | ✅ 已实现 |
| 700 | 批量创建邮件组权限成员 | POST | `/open-apis/mail/v1/mailgroups/:mailgroup_id/permission_members/batch_create` | `../src/service/mail/v1/mail_group_manager/mod.rs` | 51 | ✅ 已实现 |
| 701 | 批量删除邮件组权限成员 | DELETE | `/open-apis/mail/v1/mailgroups/:mailgroup_id/permission_members/batch_delete` | `../src/service/mail/v1/mail_group_manager/mod.rs` | 75 | ✅ 已实现 |
| 702 | 创建公共邮箱 | POST | `/open-apis/mail/v1/public_mailboxes` | `未找到` | - | ❌ 未实现 |
| 703 | 修改公共邮箱部分信息 | PATCH | `/open-apis/mail/v1/public_mailboxes/:public_mailbox_id` | `未找到` | - | ❌ 未实现 |
| 704 | 修改公共邮箱全部信息 | PUT | `/open-apis/mail/v1/public_mailboxes/:public_mailbox_id` | `未找到` | - | ❌ 未实现 |
| 705 | 查询指定公共邮箱 | GET | `/open-apis/mail/v1/public_mailboxes/:public_mailbox_id` | `未找到` | - | ❌ 未实现 |
| 706 | 查询所有公共邮箱 | GET | `/open-apis/mail/v1/public_mailboxes` | `未找到` | - | ❌ 未实现 |
| 707 | 将公共邮箱移至回收站 | DELETE | `/open-apis/mail/v1/public_mailboxes/:public_mailbox_id/remove_to_recycle_bin` | `未找到` | - | ❌ 未实现 |
| 708 | 永久删除公共邮箱 | DELETE | `/open-apis/mail/v1/public_mailboxes/:public_mailbox_id` | `未找到` | - | ❌ 未实现 |
| 709 | 添加公共邮箱成员 | POST | `/open-apis/mail/v1/public_mailboxes/:public_mailbox_id/members` | `../src/service/task/v2/tasklist/mod.rs` | 208 | ✅ 已实现 |
| 710 | 删除公共邮箱单个成员 | DELETE | `/open-apis/mail/v1/public_mailboxes/:public_mailbox_id/members/:member_id` | `未找到` | - | ❌ 未实现 |
| 711 | 删除公共邮箱所有成员 | POST | `/open-apis/mail/v1/public_mailboxes/:public_mailbox_id/members/clear` | `未找到` | - | ❌ 未实现 |
| 712 | 查询指定公共邮箱成员信息 | GET | `/open-apis/mail/v1/public_mailboxes/:public_mailbox_id/members/:member_id` | `未找到` | - | ❌ 未实现 |
| 713 | 查询所有公共邮箱成员信息 | GET | `/open-apis/mail/v1/public_mailboxes/:public_mailbox_id/members` | `../src/service/task/v2/tasklist/mod.rs` | 208 | ✅ 已实现 |
| 714 | 批量添加公共邮箱成员 | POST | `/open-apis/mail/v1/public_mailboxes/:public_mailbox_id/members/batch_create` | `../src/service/mail/v1/mail_group_manager/mod.rs` | 51 | ✅ 已实现 |
| 715 | 批量删除公共邮箱成员 | DELETE | `/open-apis/mail/v1/public_mailboxes/:public_mailbox_id/members/batch_delete` | `../src/service/mail/v1/mail_group_manager/mod.rs` | 75 | ✅ 已实现 |
| 716 | 创建公共邮箱别名 | POST | `/open-apis/mail/v1/public_mailboxes/:public_mailbox_id/aliases` | `未找到` | - | ❌ 未实现 |
| 717 | 删除公共邮箱别名 | DELETE | `/open-apis/mail/v1/public_mailboxes/:public_mailbox_id/aliases/:alias_id` | `未找到` | - | ❌ 未实现 |
| 718 | 查询公共邮箱的所有别名 | GET | `/open-apis/mail/v1/public_mailboxes/:public_mailbox_id/aliases` | `未找到` | - | ❌ 未实现 |
| 719 | 从回收站删除用户邮箱地址 | DELETE | `/open-apis/mail/v1/user_mailboxes/:user_mailbox_id` | `未找到` | - | ❌ 未实现 |
| 720 | 创建用户邮箱别名 | POST | `/open-apis/mail/v1/user_mailboxes/:user_mailbox_id/aliases` | `未找到` | - | ❌ 未实现 |
| 721 | 删除用户邮箱别名 | DELETE | `/open-apis/mail/v1/user_mailboxes/:user_mailbox_id/aliases/:alias_id` | `未找到` | - | ❌ 未实现 |
| 722 | 获取用户邮箱所有别名 | GET | `/open-apis/mail/v1/user_mailboxes/:user_mailbox_id/aliases` | `未找到` | - | ❌ 未实现 |
| 723 | 查询邮箱地址状态 | POST | `/open-apis/mail/v1/users/query` | `../src/service/attendance/v1/user_task.rs` | 86 | ✅ 已实现 |
| 724 | 转移应用所有者 | PUT | `/open-apis/application/v6/applications/:app_id/owner` | `../src/service/application/v6/application/mod.rs` | 32 | ✅ 已实现 |
| 725 | 更新应用协作者 | PUT | `/open-apis/application/v6/applications/:app_id/collaborators` | `../src/service/application/v6/application/mod.rs` | 51 | ✅ 已实现 |
| 726 | 获取应用协作者列表 | GET | `/open-apis/application/v6/applications/:app_id/collaborators` | `../src/service/application/v6/application/mod.rs` | 51 | ✅ 已实现 |
| 727 | 获取应用信息 | GET | `/open-apis/application/v6/applications/:app_id` | `未找到` | - | ❌ 未实现 |
| 728 | 获取应用版本信息 | GET | `/open-apis/application/v6/applications/:app_id/app_versions/:version_id` | `未找到` | - | ❌ 未实现 |
| 729 | 获取应用版本列表 | GET | `/open-apis/application/v6/applications/:app_id/app_versions` | `未找到` | - | ❌ 未实现 |
| 730 | 获取应用版本中开发者申请的通讯录权限范围 | GET | `/open-apis/application/v6/applications/:app_id/app_versions/:version_id/contacts_range_suggest` | `../src/service/application/v6/application/mod.rs` | 164 | ✅ 已实现 |
| 731 | 向管理员申请授权 | POST | `/open-apis/application/v6/scopes/apply` | `../src/service/application/v6/scope/mod.rs` | 31 | ✅ 已实现 |
| 732 | 查询租户授权状态 | GET | `/open-apis/application/v6/scopes` | `../src/service/contact/v3/functional_role_member.rs` | 63 | ✅ 已实现 |
| 733 | 获取企业安装的应用 | GET | `/open-apis/application/v6/applications` | `../src/service/hire/candidate_management/application/mod.rs` | 261 | ✅ 已实现 |
| 734 | 获取用户可用的应用 | GET | `/open-apis/application/v1/user/visible_apps` | `未找到` | - | ❌ 未实现 |
| 735 | 查看待审核的应用列表 | GET | `/open-apis/application/v6/applications/underauditlist` | `未找到` | - | ❌ 未实现 |
| 736 | 更新应用审核状态 | PATCH | `/open-apis/application/v6/applications/:app_id/app_versions/:version_id` | `未找到` | - | ❌ 未实现 |
| 737 | 更新应用分组信息 | PATCH | `/open-apis/application/v6/applications/:app_id` | `未找到` | - | ❌ 未实现 |
| 738 | 获取应用通讯录权限范围配置 | GET | `/open-apis/application/v6/applications/:app_id/contacts_range_configuration` | `../src/service/application/v6/admin/mod.rs` | 88 | ✅ 已实现 |
| 739 | 更新应用通讯录权限范围配置 | PATCH | `/open-apis/application/v6/applications/:app_id/contacts_range` | `../src/service/application/v6/admin/mod.rs` | 88 | ✅ 已实现 |
| 740 | 获取应用在企业内的可用范围 | GET | `/open-apis/application/v2/app/visibility` | `未找到` | - | ❌ 未实现 |
| 741 | 查询用户或部门是否在应用的可用或禁用名单 | POST | `/open-apis/application/v6/applications/:app_id/visibility/check_white_black_list` | `../src/service/application/v6/admin/mod.rs` | 174 | ✅ 已实现 |
| 742 | 更新应用可用范围 | PATCH | `/open-apis/application/v6/applications/:app_id/visibility` | `未找到` | - | ❌ 未实现 |
| 743 | 启停用应用 | PUT | `/open-apis/application/v6/applications/:app_id/management` | `未找到` | - | ❌ 未实现 |
| 744 | 查询应用管理员列表 | GET | `/open-apis/user/v4/app_admin_user/list` | `../src/service/attendance/v1/mod.rs` | 134 | ✅ 已实现 |
| 745 | 获取应用管理员管理范围 | GET | `/open-apis/contact/v1/user/admin_scope/get` | `../src/service/contact/v3/job_family.rs` | 66 | ✅ 已实现 |
| 746 | 校验应用管理员 | GET | `/open-apis/application/v3/is_user_admin` | `未找到` | - | ❌ 未实现 |
| 747 | 查询用户是否在应用开通范围 | GET | `/open-apis/pay/v1/paid_scope/check_user` | `../src/service/application/v6/appstore_paid_info/mod.rs` | 31 | ✅ 已实现 |
| 748 | 查询租户购买的付费方案 | GET | `/open-apis/pay/v1/order/list` | `../src/service/attendance/v1/mod.rs` | 134 | ✅ 已实现 |
| 749 | 查询订单详情 | GET | `/open-apis/pay/v1/order/get` | `../src/service/attendance/v1/mod.rs` | 34 | ✅ 已实现 |
| 750 | 获取多部门应用使用概览 | POST | `/open-apis/application/v6/applications/:app_id/app_usage/department_overview` | `../src/service/application/v6/app_usage/mod.rs` | 31 | ✅ 已实现 |
| 751 | 获取消息推送概览 | POST | `/open-apis/application/v6/applications/:app_id/app_usage/message_push_overview` | `../src/service/application/v6/app_usage/mod.rs` | 64 | ✅ 已实现 |
| 752 | 获取应用使用概览 | POST | `/open-apis/application/v6/applications/:app_id/app_usage/overview` | `../src/service/application/v6/app_usage/mod.rs` | 31 | ✅ 已实现 |
| 753 | 更新应用反馈 | PATCH | `/open-apis/application/v6/applications/:app_id/feedbacks/:feedback_id` | `未找到` | - | ❌ 未实现 |
| 754 | 获取应用反馈列表 | GET | `/open-apis/application/v6/applications/:app_id/feedbacks` | `未找到` | - | ❌ 未实现 |
| 755 | 更新应用红点 | POST | `/open-apis/application/v6/app_badge/set` | `../src/service/application/v6/app_badge/mod.rs` | 30 | ✅ 已实现 |
| 756 | 获取企业席位信息接口 | GET | `/open-apis/tenant/v2/tenant/assign_info_list/query` | `../src/service/tenant/v2/tenant_product_assign_info/mod.rs` | 58 | ✅ 已实现 |
| 757 | 获取企业信息 | GET | `/open-apis/tenant/v2/tenant/query` | `../src/service/tenant/v2/tenant_product_assign_info/mod.rs` | 58 | ✅ 已实现 |
| 758 | 获取认证信息 | GET | `/open-apis/verification/v1/verification` | `未找到` | - | ❌ 未实现 |
| 759 | 创建系统状态 | POST | `/open-apis/personal_settings/v1/system_statuses` | `未找到` | - | ❌ 未实现 |
| 760 | 删除系统状态 | DELETE | `/open-apis/personal_settings/v1/system_statuses/:system_status_id` | `未找到` | - | ❌ 未实现 |
| 761 | 修改系统状态 | PATCH | `/open-apis/personal_settings/v1/system_statuses/:system_status_id` | `未找到` | - | ❌ 未实现 |
| 762 | 获取系统状态 | GET | `/open-apis/personal_settings/v1/system_statuses` | `未找到` | - | ❌ 未实现 |
| 763 | 批量开启系统状态 | POST | `/open-apis/personal_settings/v1/system_statuses/:system_status_id/batch_open` | `../src/service/personal_settings/v1/system_status/mod.rs` | 173 | ✅ 已实现 |
| 764 | 批量关闭系统状态 | POST | `/open-apis/personal_settings/v1/system_statuses/:system_status_id/batch_close` | `../src/service/personal_settings/v1/system_status/mod.rs` | 194 | ✅ 已实现 |
| 765 | 搜索消息 | POST | `/open-apis/search/v2/message` | `../src/service/search/v2/suite_search/mod.rs` | 61 | ✅ 已实现 |
| 766 | 搜索应用 | POST | `/open-apis/search/v2/app` | `../src/service/search/v2/suite_search/mod.rs` | 96 | ✅ 已实现 |
| 767 | 创建数据源 | POST | `/open-apis/search/v2/data_sources` | `../src/service/search/v1/user.rs` | 177 | ✅ 已实现 |
| 768 | 删除数据源 | DELETE | `/open-apis/search/v2/data_sources/:data_source_id` | `../src/service/search/v1/user.rs` | 177 | ✅ 已实现 |
| 769 | 修改数据源 | PATCH | `/open-apis/search/v2/data_sources/:data_source_id` | `../src/service/search/v1/user.rs` | 177 | ✅ 已实现 |
| 770 | 获取数据源 | GET | `/open-apis/search/v2/data_sources/:data_source_id` | `../src/service/search/v1/user.rs` | 177 | ✅ 已实现 |
| 771 | 批量获取数据源 | GET | `/open-apis/search/v2/data_sources` | `../src/service/search/v1/user.rs` | 177 | ✅ 已实现 |
| 772 | 为指定数据项创建索引 | POST | `/open-apis/search/v2/data_sources/:data_source_id/items` | `../src/service/search/v1/user.rs` | 177 | ✅ 已实现 |
| 773 | 删除数据项 | DELETE | `/open-apis/search/v2/data_sources/:data_source_id/items/:item_id` | `../src/service/search/v1/user.rs` | 177 | ✅ 已实现 |
| 774 | 查询指定数据项 | GET | `/open-apis/search/v2/data_sources/:data_source_id/items/:item_id` | `../src/service/search/v1/user.rs` | 177 | ✅ 已实现 |
| 775 | 创建数据范式 | POST | `/open-apis/search/v2/schemas` | `../src/service/search/v1/user.rs` | 177 | ✅ 已实现 |
| 776 | 删除数据范式 | DELETE | `/open-apis/search/v2/schemas/:schema_id` | `../src/service/search/v1/user.rs` | 177 | ✅ 已实现 |
| 777 | 修改数据范式 | PATCH | `/open-apis/search/v2/schemas/:schema_id` | `../src/service/search/v1/user.rs` | 177 | ✅ 已实现 |
| 778 | 获取数据范式 | GET | `/open-apis/search/v2/schemas/:schema_id` | `../src/service/search/v1/user.rs` | 177 | ✅ 已实现 |
| 779 | 识别文件中的简历信息 | POST | `/open-apis/document_ai/v1/resume/parse` | `../src/service/ai/document_ai/mod.rs` | 139 | ✅ 已实现 |
| 780 | 识别文件中的机动车发票 | POST | `/open-apis/document_ai/v1/vehicle_invoice/recognize` | `../src/service/ai/speech_to_text/mod.rs` | 61 | ✅ 已实现 |
| 781 | 识别文件中的健康证 | POST | `/open-apis/document_ai/v1/health_certificate/recognize` | `../src/service/ai/speech_to_text/mod.rs` | 61 | ✅ 已实现 |
| 782 | 识别文件中的港澳居民来往内地通行证 | POST | `/open-apis/document_ai/v1/hkm_mainland_travel_permit/recognize` | `../src/service/ai/speech_to_text/mod.rs` | 61 | ✅ 已实现 |
| 783 | 识别文件中的台湾居民来往大陆通行证 | POST | `/open-apis/document_ai/v1/tw_mainland_travel_permit/recognize` | `../src/service/ai/speech_to_text/mod.rs` | 61 | ✅ 已实现 |
| 784 | 识别文件中的中国护照 | POST | `/open-apis/document_ai/v1/chinese_passport/recognize` | `../src/service/ai/speech_to_text/mod.rs` | 61 | ✅ 已实现 |
| 785 | 识别文件中的银行卡 | POST | `/open-apis/document_ai/v1/bank_card/recognize` | `../src/service/ai/speech_to_text/mod.rs` | 61 | ✅ 已实现 |
| 786 | 识别文件中的行驶证 | POST | `/open-apis/document_ai/v1/vehicle_license/recognize` | `../src/service/ai/speech_to_text/mod.rs` | 61 | ✅ 已实现 |
| 787 | 识别文件中的火车票 | POST | `/open-apis/document_ai/v1/train_invoice/recognize` | `../src/service/ai/speech_to_text/mod.rs` | 61 | ✅ 已实现 |
| 788 | 识别文件中的出租车发票 | POST | `/open-apis/document_ai/v1/taxi_invoice/recognize` | `../src/service/ai/speech_to_text/mod.rs` | 61 | ✅ 已实现 |
| 789 | 识别文件中的身份证 | POST | `/open-apis/document_ai/v1/id_card/recognize` | `../src/service/ai/speech_to_text/mod.rs` | 61 | ✅ 已实现 |
| 790 | 识别文件中的食品生产许可证 | POST | `/open-apis/document_ai/v1/food_produce_license/recognize` | `../src/service/ai/speech_to_text/mod.rs` | 61 | ✅ 已实现 |
| 791 | 识别文件中的食品经营许可证 | POST | `/open-apis/document_ai/v1/food_manage_license/recognize` | `../src/service/ai/speech_to_text/mod.rs` | 61 | ✅ 已实现 |
| 792 | 识别文件中的驾驶证 | POST | `/open-apis/document_ai/v1/driving_license/recognize` | `../src/service/ai/speech_to_text/mod.rs` | 61 | ✅ 已实现 |
| 793 | 识别文件中的增值税发票 | POST | `/open-apis/document_ai/v1/vat_invoice/recognize` | `../src/service/ai/speech_to_text/mod.rs` | 61 | ✅ 已实现 |
| 794 | 识别文件中的营业执照 | POST | `/open-apis/document_ai/v1/business_license/recognize` | `../src/service/ai/speech_to_text/mod.rs` | 61 | ✅ 已实现 |
| 795 | 提取文件中的合同字段 | POST | `/open-apis/document_ai/v1/contract/field_extraction` | `未找到` | - | ❌ 未实现 |
| 796 | 识别文件中的名片 | POST | `/open-apis/document_ai/v1/business_card/recognize` | `../src/service/ai/speech_to_text/mod.rs` | 61 | ✅ 已实现 |
| 797 | 识别图片中的文字 | POST | `/open-apis/optical_char_recognition/v1/image/basic_recognize` | `../src/service/ai/optical_char_recognition/mod.rs` | 49 | ✅ 已实现 |
| 798 | 识别语音文件 | POST | `/open-apis/speech_to_text/v1/speech/file_recognize` | `../src/service/ai/speech_to_text/mod.rs` | 61 | ✅ 已实现 |
| 799 | 识别流式语音 | POST | `/open-apis/speech_to_text/v1/speech/stream_recognize` | `../src/service/ai/speech_to_text/mod.rs` | 82 | ✅ 已实现 |
| 800 | 识别文本语种 | POST | `/open-apis/translation/v1/text/detect` | `../src/service/security_and_compliance/v1/security_monitoring.rs` | 249 | ✅ 已实现 |
| 801 | 翻译文本 | POST | `/open-apis/translation/v1/text/translate` | `未找到` | - | ❌ 未实现 |
| 802 | 查看应用基本信息 | GET | `/open-apis/apaas/v1/apps` | `../src/service/workplace/app_recommend/mod.rs` | 47 | ✅ 已实现 |
| 803 | 查询席位分配详情 | GET | `/open-apis/apaas/v1/seat_assignments` | `未找到` | - | ❌ 未实现 |
| 804 | 查询席位活跃详情 | GET | `/open-apis/apaas/v1/seat_activities` | `未找到` | - | ❌ 未实现 |
| 805 | 查询审计日志列表 | GET | `/open-apis/apaas/v1/applications/:namespace/audit_log/audit_log_list` | `未找到` | - | ❌ 未实现 |
| 806 | 查询审计日志详情 | GET | `/open-apis/apaas/v1/applications/:namespace/audit_log` | `../src/service/apass/audit_log/mod.rs` | 91 | ✅ 已实现 |
| 807 | 查询数据变更日志列表 | GET | `/open-apis/apaas/v1/applications/:namespace/audit_log/data_change_logs_list` | `未找到` | - | ❌ 未实现 |
| 808 | 查询数据变更日志详情 | GET | `/open-apis/apaas/v1/applications/:namespace/audit_log/data_change_log_detail` | `../src/service/apass/audit_log/mod.rs` | 194 | ✅ 已实现 |
| 809 | 批量删除记录权限用户授权 | POST | `/open-apis/apaas/v1/applications/:namespace/record_permissions/:record_permission_api_name/member/batch_remove_authorization` | `未找到` | - | ❌ 未实现 |
| 810 | 批量创建记录权限用户授权 | POST | `/open-apis/apaas/v1/applications/:namespace/record_permissions/:record_permission_api_name/member/batch_create_authorization` | `未找到` | - | ❌ 未实现 |
| 811 | 批量删除角色成员授权 | POST | `/open-apis/apaas/v1/applications/:namespace/roles/:role_api_name/member/batch_remove_authorization` | `未找到` | - | ❌ 未实现 |
| 812 | 批量创建角色成员授权 | POST | `/open-apis/apaas/v1/applications/:namespace/roles/:role_api_name/member/batch_create_authorization` | `未找到` | - | ❌ 未实现 |
| 813 | 查询角色成员信息 | GET | `/open-apis/apaas/v1/applications/:namespace/roles/:role_api_name/member` | `../src/service/trust_party/collaboration_organization/mod.rs` | 183 | ✅ 已实现 |
| 814 | 执行 OQL | POST | `/open-apis/apaas/v1/applications/:namespace/objects/oql_query` | `../src/service/apass/object/mod.rs` | 126 | ✅ 已实现 |
| 815 | 搜索记录 | POST | `/open-apis/apaas/v1/applications/:namespace/objects/search` | `../src/service/attendance/v1/group.rs` | 154 | ✅ 已实现 |
| 816 | 获取记录详情 | POST | `/open-apis/apaas/v1/applications/:namespace/objects/:object_api_name/records/:id/query` | `../src/service/attendance/v1/user_task.rs` | 86 | ✅ 已实现 |
| 817 | 编辑记录 | PATCH | `/open-apis/apaas/v1/applications/:namespace/objects/:object_api_name/records/:id` | `../src/service/contact/v3/job_family.rs` | 66 | ✅ 已实现 |
| 818 | 删除记录 | DELETE | `/open-apis/apaas/v1/applications/:namespace/objects/:object_api_name/records/:id` | `../src/service/contact/v3/job_family.rs` | 66 | ✅ 已实现 |
| 819 | 新建记录 | POST | `/open-apis/apaas/v1/applications/:namespace/objects/:object_api_name/records` | `../src/service/okr/v1/mod.rs` | 409 | ✅ 已实现 |
| 820 | 批量编辑记录 | PATCH | `/open-apis/apaas/v1/applications/:namespace/objects/:object_api_name/records/batch_update` | `../src/service/apass/object/mod.rs` | 304 | ✅ 已实现 |
| 821 | 查询记录列表 | POST | `/open-apis/apaas/v1/applications/:namespace/objects/:object_api_name/records/batch_query` | `../src/service/apass/object/mod.rs` | 333 | ✅ 已实现 |
| 822 | 批量删除记录 | DELETE | `/open-apis/apaas/v1/applications/:namespace/objects/:object_api_name/records/batch_delete` | `../src/service/contact/v3/functional_role_member.rs` | 131 | ✅ 已实现 |
| 823 | 批量新建记录 | POST | `/open-apis/apaas/v1/applications/:namespace/objects/:object_api_name/records/batch_create` | `../src/service/contact/v3/functional_role_member.rs` | 42 | ✅ 已实现 |
| 824 | 执行函数 | POST | `/open-apis/apaas/v1/applications/:namespace/functions/:function_api_name/invoke` | `../src/service/apass/function/mod.rs` | 42 | ✅ 已实现 |
| 825 | 查询环境变量列表 | POST | `/open-apis/apaas/v1/applications/:namespace/environment_variables/query` | `../src/service/attendance/v1/user_task.rs` | 86 | ✅ 已实现 |
| 826 | 查询环境变量详情 | GET | `/open-apis/apaas/v1/applications/:namespace/environment_variables/:environment_variable_api_name` | `未找到` | - | ❌ 未实现 |
| 827 | 发起流程 | POST | `/open-apis/apaas/v1/applications/:namespace/flows/:flow_id/execute` | `../src/service/apass/flow/mod.rs` | 94 | ✅ 已实现 |
| 828 | 查询人工任务 | POST | `/open-apis/apaas/v1/user_task/query` | `../src/service/attendance/v1/user_task.rs` | 86 | ✅ 已实现 |
| 829 | 同意人工任务 | POST | `/open-apis/apaas/v1/approval_tasks/:approval_task_id/agree` | `../src/service/apass/flow/mod.rs` | 157 | ✅ 已实现 |
| 830 | 拒绝人工任务 | POST | `/open-apis/apaas/v1/approval_tasks/:approval_task_id/reject` | `../src/service/apass/flow/mod.rs` | 184 | ✅ 已实现 |
| 831 | 转交人工任务 | POST | `/open-apis/apaas/v1/approval_tasks/:approval_task_id/transfer` | `../src/service/apass/flow/mod.rs` | 211 | ✅ 已实现 |
| 832 | 人工任务加签 | POST | `/open-apis/apaas/v1/approval_tasks/:approval_task_id/add_assignee` | `../src/service/apass/flow/mod.rs` | 238 | ✅ 已实现 |
| 833 | 抄送人工任务 | POST | `/open-apis/apaas/v1/user_tasks/:task_id/cc` | `../src/service/auth/v1/mod.rs` | 41 | ✅ 已实现 |
| 834 | 催办人工任务 | POST | `/open-apis/apaas/v1/user_tasks/:task_id/expediting` | `未找到` | - | ❌ 未实现 |
| 835 | 撤销人工任务 | POST | `/open-apis/apaas/v1/approval_instances/:approval_instance_id/cancel` | `../src/service/apass/flow/mod.rs` | 318 | ✅ 已实现 |
| 836 | 查询人工任务可退回的位置 | POST | `/open-apis/apaas/v1/user_tasks/:task_id/rollback_points` | `../src/service/apass/flow/mod.rs` | 345 | ✅ 已实现 |
| 837 | 退回人工任务 | POST | `/open-apis/apaas/v1/user_tasks/:task_id/rollback` | `../src/service/apass/flow/mod.rs` | 345 | ✅ 已实现 |
| 838 | 基于人工任务发起群聊 | POST | `/open-apis/apaas/v1/user_tasks/:task_id/chat_group` | `../src/service/apass/flow/mod.rs` | 397 | ✅ 已实现 |
| 839 | 创建会话 | POST | `/open-apis/aily/v1/sessions` | `../src/service/aily/session/mod.rs` | 81 | ✅ 已实现 |
| 840 | 更新会话 | PUT | `/open-apis/aily/v1/sessions/:aily_session_id` | `../src/service/aily/session/mod.rs` | 81 | ✅ 已实现 |
| 841 | 获取会话 | GET | `/open-apis/aily/v1/sessions/:aily_session_id` | `../src/service/aily/session/mod.rs` | 81 | ✅ 已实现 |
| 842 | 删除会话 | DELETE | `/open-apis/aily/v1/sessions/:aily_session_id` | `../src/service/aily/session/mod.rs` | 81 | ✅ 已实现 |
| 843 | 发送 Aily 消息 | POST | `/open-apis/aily/v1/sessions/:aily_session_id/messages` | `../src/service/aily/message/mod.rs` | 127 | ✅ 已实现 |
| 844 | 获取 Aily 消息 | GET | `/open-apis/aily/v1/sessions/:aily_session_id/messages/:aily_message_id` | `../src/service/aily/message/mod.rs` | 68 | ✅ 已实现 |
| 845 | 列出 Aily 消息 | GET | `/open-apis/aily/v1/sessions/:aily_session_id/messages` | `../src/service/aily/message/mod.rs` | 127 | ✅ 已实现 |
| 846 | 创建运行 | POST | `/open-apis/aily/v1/sessions/:aily_session_id/runs` | `../src/service/aily/run/mod.rs` | 140 | ✅ 已实现 |
| 847 | 获取运行 | GET | `/open-apis/aily/v1/sessions/:aily_session_id/runs/:run_id` | `../src/service/aily/session/mod.rs` | 81 | ✅ 已实现 |
| 848 | 列出运行 | GET | `/open-apis/aily/v1/sessions/:aily_session_id/runs` | `../src/service/aily/run/mod.rs` | 140 | ✅ 已实现 |
| 849 | 取消运行 | POST | `/open-apis/aily/v1/sessions/:aily_session_id/runs/:run_id/cancel` | `../src/service/aily/run/mod.rs` | 175 | ✅ 已实现 |
| 850 | 调用技能 | POST | `/open-apis/aily/v1/apps/:app_id/skills/:skill_id/start` | `../src/service/aily/skill/mod.rs` | 68 | ✅ 已实现 |
| 851 | 获取技能信息 | GET | `/open-apis/aily/v1/apps/:app_id/skills/:skill_id` | `未找到` | - | ❌ 未实现 |
| 852 | 查询技能列表 | GET | `/open-apis/aily/v1/apps/:app_id/skills` | `../src/service/aily/skill/mod.rs` | 124 | ✅ 已实现 |
| 853 | 执行数据知识问答 | POST | `/open-apis/aily/v1/apps/:app_id/knowledges/ask` | `../src/service/aily/knowledge/mod.rs` | 116 | ✅ 已实现 |
| 854 | 上传文件用于数据知识管理 | POST | `/open-apis/aily/v1/apps/:app_id/data_assets/upload_file` | `../src/service/aily/knowledge/mod.rs` | 143 | ✅ 已实现 |
| 855 | 创建数据知识 | POST | `/open-apis/aily/v1/apps/:app_id/data_assets` | `未找到` | - | ❌ 未实现 |
| 856 | 获取数据知识 | GET | `/open-apis/aily/v1/apps/:app_id/data_assets/:data_asset_id` | `未找到` | - | ❌ 未实现 |
| 857 | 删除数据知识 | DELETE | `/open-apis/aily/v1/apps/:app_id/data_assets/:data_asset_id` | `未找到` | - | ❌ 未实现 |
| 858 | 查询数据知识列表 | GET | `/open-apis/aily/v1/apps/:app_id/data_assets` | `未找到` | - | ❌ 未实现 |
| 859 | 获取数据知识分类列表 | GET | `/open-apis/aily/v1/apps/:app_id/data_asset_tags` | `未找到` | - | ❌ 未实现 |
| 860 | 重置用户的企业邮箱密码 | POST | `/open-apis/admin/v1/password/reset` | `未找到` | - | ❌ 未实现 |
| 861 | 获取部门维度的用户活跃和功能使用数据 | GET | `/open-apis/admin/v1/admin_dept_stats` | `未找到` | - | ❌ 未实现 |
| 862 | 获取用户维度的用户活跃和功能使用数据 | GET | `/open-apis/admin/v1/admin_user_stats` | `未找到` | - | ❌ 未实现 |
| 863 | 创建勋章 | POST | `/open-apis/admin/v1/badges` | `未找到` | - | ❌ 未实现 |
| 864 | 修改勋章信息 | PUT | `/open-apis/admin/v1/badges/:badge_id` | `未找到` | - | ❌ 未实现 |
| 865 | 上传勋章图片 | POST | `/open-apis/admin/v1/badge_images` | `未找到` | - | ❌ 未实现 |
| 866 | 获取勋章列表 | GET | `/open-apis/admin/v1/badges` | `未找到` | - | ❌ 未实现 |
| 867 | 获取勋章详情 | GET | `/open-apis/admin/v1/badges/:badge_id` | `未找到` | - | ❌ 未实现 |
| 868 | 创建授予名单 | POST | `/open-apis/admin/v1/badges/:badge_id/grants` | `未找到` | - | ❌ 未实现 |
| 869 | 删除授予名单 | DELETE | `/open-apis/admin/v1/badges/:badge_id/grants/:grant_id` | `未找到` | - | ❌ 未实现 |
| 870 | 修改授予名单 | PUT | `/open-apis/admin/v1/badges/:badge_id/grants/:grant_id` | `未找到` | - | ❌ 未实现 |
| 871 | 获取授予名单列表 | GET | `/open-apis/admin/v1/badges/:badge_id/grants` | `未找到` | - | ❌ 未实现 |
| 872 | 获取授予名单详情 | GET | `/open-apis/admin/v1/badges/:badge_id/grants/:grant_id` | `未找到` | - | ❌ 未实现 |
| 873 | 查询帖子信息 | GET | `/open-apis/moments/v1/posts/:post_id` | `未找到` | - | ❌ 未实现 |
| 874 | 批量获取员工花名册信息 | GET | `/open-apis/ehr/v1/employees` | `../src/service/ehr/v1/mod.rs` | 263 | ✅ 已实现 |
| 875 | 下载人员的附件 | GET | `/open-apis/ehr/v1/attachments/:token` | `../src/service/auth/v1/mod.rs` | 41 | ✅ 已实现 |
| 876 | 获取飞书人事对象列表 | GET | `/open-apis/corehr/v1/custom_fields/list_object_api_name` | `未找到` | - | ❌ 未实现 |
| 877 | 获取自定义字段列表 | GET | `/open-apis/corehr/v1/custom_fields/query` | `../src/service/corehr/job_management/mod.rs` | 383 | ✅ 已实现 |
| 878 | 获取字段详情 | GET | `/open-apis/corehr/v1/custom_fields/get_by_param` | `未找到` | - | ❌ 未实现 |
| 879 | 增加字段枚举值选项 | POST | `/open-apis/corehr/v1/common_data/meta_data/add_enum_option` | `未找到` | - | ❌ 未实现 |
| 880 | 修改字段枚举值选项 | POST | `/open-apis/corehr/v1/common_data/meta_data/edit_enum_option` | `未找到` | - | ❌ 未实现 |
| 881 | 查询枚举信息 | POST | `/open-apis/corehr/v2/enums/search` | `../src/service/corehr/lifecycle/mod.rs` | 191 | ✅ 已实现 |
| 882 | 查询国家/地区信息 | POST | `/open-apis/corehr/v2/basic_info/country_regions/search` | `../src/service/corehr/lifecycle/mod.rs` | 191 | ✅ 已实现 |
| 883 | 查询省份/主要行政区信息 | POST | `/open-apis/corehr/v2/basic_info/country_region_subdivisions/search` | `../src/service/corehr/lifecycle/mod.rs` | 191 | ✅ 已实现 |
| 884 | 查询城市信息 | POST | `/open-apis/corehr/v2/basic_info/cities/search` | `../src/service/corehr/lifecycle/mod.rs` | 191 | ✅ 已实现 |
| 885 | 查询区/县信息 | POST | `/open-apis/corehr/v2/basic_info/districts/search` | `../src/service/corehr/lifecycle/mod.rs` | 191 | ✅ 已实现 |
| 886 | 查询国籍信息 | POST | `/open-apis/corehr/v2/basic_info/nationalities/search` | `../src/service/corehr/lifecycle/mod.rs` | 191 | ✅ 已实现 |
| 887 | 创建国家证件类型 | POST | `/open-apis/corehr/v1/national_id_types` | `未找到` | - | ❌ 未实现 |
| 888 | 删除国家证件类型 | DELETE | `/open-apis/corehr/v1/national_id_types/:national_id_type_id` | `未找到` | - | ❌ 未实现 |
| 889 | 更新国家证件类型 | PATCH | `/open-apis/corehr/v1/national_id_types/:national_id_type_id` | `未找到` | - | ❌ 未实现 |
| 890 | 查询单个国家证件类型 | GET | `/open-apis/corehr/v1/national_id_types/:national_id_type_id` | `未找到` | - | ❌ 未实现 |
| 891 | 批量查询国家证件类型 | GET | `/open-apis/corehr/v1/national_id_types` | `未找到` | - | ❌ 未实现 |
| 892 | 查询银行信息 | POST | `/open-apis/corehr/v2/basic_info/banks/search` | `../src/service/corehr/lifecycle/mod.rs` | 191 | ✅ 已实现 |
| 893 | 查询支行信息 | POST | `/open-apis/corehr/v2/basic_info/bank_branchs/search` | `../src/service/corehr/lifecycle/mod.rs` | 191 | ✅ 已实现 |
| 894 | 查询货币信息 | POST | `/open-apis/corehr/v2/basic_info/currencies/search` | `../src/service/corehr/lifecycle/mod.rs` | 191 | ✅ 已实现 |
| 895 | 查询时区信息 | POST | `/open-apis/corehr/v2/basic_info/time_zones/search` | `../src/service/corehr/lifecycle/mod.rs` | 191 | ✅ 已实现 |
| 896 | 查询语言信息 | POST | `/open-apis/corehr/v2/basic_info/languages/search` | `../src/service/corehr/lifecycle/mod.rs` | 191 | ✅ 已实现 |
| 897 | 创建人员类型 | POST | `/open-apis/corehr/v1/employee_types` | `未找到` | - | ❌ 未实现 |
| 898 | 删除人员类型 | DELETE | `/open-apis/corehr/v1/employee_types/:employee_type_id` | `未找到` | - | ❌ 未实现 |
| 899 | 更新人员类型 | PATCH | `/open-apis/corehr/v1/employee_types/:employee_type_id` | `未找到` | - | ❌ 未实现 |
| 900 | 查询单个人员类型 | GET | `/open-apis/corehr/v1/employee_types/:employee_type_id` | `未找到` | - | ❌ 未实现 |
| 901 | 批量查询人员类型 | GET | `/open-apis/corehr/v1/employee_types` | `未找到` | - | ❌ 未实现 |
| 902 | 创建工时制度 | POST | `/open-apis/corehr/v1/working_hours_types` | `未找到` | - | ❌ 未实现 |
| 903 | 删除工时制度 | DELETE | `/open-apis/corehr/v1/working_hours_types/:working_hours_type_id` | `未找到` | - | ❌ 未实现 |
| 904 | 更新工时制度 | PATCH | `/open-apis/corehr/v1/working_hours_types/:working_hours_type_id` | `未找到` | - | ❌ 未实现 |
| 905 | 查询单个工时制度 | GET | `/open-apis/corehr/v1/working_hours_types/:working_hours_type_id` | `未找到` | - | ❌ 未实现 |
| 906 | 批量查询工时制度 | GET | `/open-apis/corehr/v1/working_hours_types` | `未找到` | - | ❌ 未实现 |
| 907 | ID 转换 | POST | `/open-apis/corehr/v1/common_data/id/convert` | `../src/service/corehr/basic_info/mod.rs` | 232 | ✅ 已实现 |
| 908 | 批量查询员工信息 | POST | `/open-apis/corehr/v2/employees/batch_get` | `../src/service/corehr/organization/mod.rs` | 161 | ✅ 已实现 |
| 909 | 搜索员工信息 | POST | `/open-apis/corehr/v2/employees/search` | `../src/service/corehr/lifecycle/mod.rs` | 191 | ✅ 已实现 |
| 910 | 添加人员 | POST | `/open-apis/corehr/v2/employees` | `../src/service/ehr/v1/mod.rs` | 263 | ✅ 已实现 |
| 911 | 创建个人信息 | POST | `/open-apis/corehr/v2/persons` | `未找到` | - | ❌ 未实现 |
| 912 | 更新个人信息 | PATCH | `/open-apis/corehr/v2/persons/:person_id` | `未找到` | - | ❌ 未实现 |
| 913 | 删除个人信息 | DELETE | `/open-apis/corehr/v1/persons/:person_id` | `未找到` | - | ❌ 未实现 |
| 914 | 上传文件 | POST | `/open-apis/corehr/v1/persons/upload` | `../src/service/attendance/v1/user_setting.rs` | 77 | ✅ 已实现 |
| 915 | 下载文件 | GET | `/open-apis/corehr/v1/files/:id` | `../src/service/corehr/basic_info/mod.rs` | 232 | ✅ 已实现 |
| 916 | 创建雇佣信息 | POST | `/open-apis/corehr/v1/employments` | `未找到` | - | ❌ 未实现 |
| 917 | 更新雇佣信息 | PATCH | `/open-apis/corehr/v1/employments/:employment_id` | `未找到` | - | ❌ 未实现 |
| 918 | 删除雇佣信息 | DELETE | `/open-apis/corehr/v1/employments/:employment_id` | `未找到` | - | ❌ 未实现 |
| 919 | 创建任职信息 | POST | `/open-apis/corehr/v1/job_datas` | `未找到` | - | ❌ 未实现 |
| 920 | 删除任职信息 | DELETE | `/open-apis/corehr/v1/job_datas/:job_data_id` | `未找到` | - | ❌ 未实现 |
| 921 | 更新任职信息 | PATCH | `/open-apis/corehr/v1/job_datas/:job_data_id` | `未找到` | - | ❌ 未实现 |
| 922 | 获取任职信息列表 | POST | `/open-apis/corehr/v2/employees/job_datas/query` | `../src/service/corehr/job_management/mod.rs` | 383 | ✅ 已实现 |
| 923 | 批量查询员工任职信息 | POST | `/open-apis/corehr/v2/employees/job_datas/batch_get` | `../src/service/corehr/organization/mod.rs` | 161 | ✅ 已实现 |
| 924 | 批量查询任职信息 | GET | `/open-apis/corehr/v1/job_datas` | `未找到` | - | ❌ 未实现 |
| 925 | 查询单个任职信息 | GET | `/open-apis/corehr/v1/job_datas/:job_data_id` | `未找到` | - | ❌ 未实现 |
| 926 | 创建外派信息 | POST | `/open-apis/corehr/v2/employees/international_assignments` | `未找到` | - | ❌ 未实现 |
| 927 | 更新外派信息 | PATCH | `/open-apis/corehr/v2/employees/international_assignments/:international_assignment_id` | `未找到` | - | ❌ 未实现 |
| 928 | 批量查询外派信息 | GET | `/open-apis/corehr/v2/employees/international_assignments` | `未找到` | - | ❌ 未实现 |
| 929 | 删除外派信息 | DELETE | `/open-apis/corehr/v2/employees/international_assignments/:international_assignment_id` | `未找到` | - | ❌ 未实现 |
| 930 | 创建兼职 | POST | `/open-apis/corehr/v2/employees/additional_jobs` | `未找到` | - | ❌ 未实现 |
| 931 | 更新兼职 | PATCH | `/open-apis/corehr/v2/employees/additional_jobs/:additional_job_id` | `未找到` | - | ❌ 未实现 |
| 932 | 删除兼职 | DELETE | `/open-apis/corehr/v2/employees/additional_jobs/:additional_job_id` | `未找到` | - | ❌ 未实现 |
| 933 | 批量查询兼职信息 | POST | `/open-apis/corehr/v2/employees/additional_jobs/batch` | `../src/service/corehr/organization/mod.rs` | 161 | ✅ 已实现 |
| 934 | 更新默认成本中心 | POST | `/open-apis/corehr/v2/default_cost_centers/update_version` | `未找到` | - | ❌ 未实现 |
| 935 | 删除默认成本中心 | POST | `/open-apis/corehr/v2/default_cost_centers/remove_version` | `未找到` | - | ❌ 未实现 |
| 936 | 添加默认成本中心 | POST | `/open-apis/corehr/v2/default_cost_centers/create_version` | `../src/service/cloud_docs/drive/v1/file_version.rs` | 39 | ✅ 已实现 |
| 937 | 查询默认成本中心 | POST | `/open-apis/corehr/v2/default_cost_centers/batch_query` | `../src/service/apass/object/mod.rs` | 333 | ✅ 已实现 |
| 938 | 更新成本分摊 | POST | `/open-apis/corehr/v2/cost_allocations/update_version` | `未找到` | - | ❌ 未实现 |
| 939 | 删除成本分摊 | POST | `/open-apis/corehr/v2/cost_allocations/remove_version` | `未找到` | - | ❌ 未实现 |
| 940 | 创建成本分摊 | POST | `/open-apis/corehr/v2/cost_allocations/create_version` | `../src/service/cloud_docs/drive/v1/file_version.rs` | 39 | ✅ 已实现 |
| 941 | 查询成本分摊 | POST | `/open-apis/corehr/v2/cost_allocations/batch_query` | `../src/service/apass/object/mod.rs` | 333 | ✅ 已实现 |
| 942 | 批量查询部门操作日志 | POST | `/open-apis/corehr/departments/query_operation_logs` | `../src/service/corehr/organization/mod.rs` | 124 | ✅ 已实现 |
| 943 | 创建部门 | POST | `/open-apis/corehr/v1/departments` | `../src/service/corehr/organization/mod.rs` | 161 | ✅ 已实现 |
| 944 | 更新部门 | PATCH | `/open-apis/corehr/v2/departments/:department_id` | `../src/service/corehr/organization/mod.rs` | 124 | ✅ 已实现 |
| 945 | 获取父部门信息 | POST | `/open-apis/corehr/v2/departments/parents` | `../src/service/corehr/organization/mod.rs` | 124 | ✅ 已实现 |
| 946 | 批量查询部门 | POST | `/open-apis/corehr/v2/departments/batch_get` | `../src/service/corehr/organization/mod.rs` | 161 | ✅ 已实现 |
| 947 | 查询生效信息变更部门 | GET | `/open-apis/corehr/v2/departments/query_recent_change` | `../src/service/corehr/organization/mod.rs` | 124 | ✅ 已实现 |
| 948 | 查询指定生效日期的部门基本信息 | POST | `/open-apis/corehr/v2/departments/query_timeline` | `../src/service/corehr/organization/mod.rs` | 124 | ✅ 已实现 |
| 949 | 查询指定生效日期的部门架构树 | POST | `/open-apis/corehr/v2/departments/tree` | `../src/service/corehr/organization/mod.rs` | 207 | ✅ 已实现 |
| 950 | 批量查询部门版本信息 | POST | `/open-apis/corehr/v2/departments/query_multi_timeline` | `../src/service/corehr/organization/mod.rs` | 124 | ✅ 已实现 |
| 951 | 搜索部门信息 | POST | `/open-apis/corehr/v2/departments/search` | `../src/service/corehr/lifecycle/mod.rs` | 191 | ✅ 已实现 |
| 952 | 删除部门 V2 | DELETE | `/open-apis/corehr/v2/departments/:department_id` | `../src/service/corehr/organization/mod.rs` | 124 | ✅ 已实现 |
| 953 | 创建地点 | POST | `/open-apis/corehr/v1/locations` | `../src/service/hire/recruitment_config/location/mod.rs` | 62 | ✅ 已实现 |
| 954 | 更新地点 | PATCH | `/open-apis/corehr/v2/locations/:location_id` | `未找到` | - | ❌ 未实现 |
| 955 | 查询单个地点 | GET | `/open-apis/corehr/v1/locations/:location_id` | `未找到` | - | ❌ 未实现 |
| 956 | 查询当前生效信息发生变更的地点 | GET | `/open-apis/corehr/v2/locations/query_recent_change` | `未找到` | - | ❌ 未实现 |
| 957 | 通过地点 ID 批量获取地点信息 | POST | `/open-apis/corehr/v2/locations/batch_get` | `../src/service/corehr/organization/mod.rs` | 161 | ✅ 已实现 |
| 958 | 批量分页查询地点信息 | GET | `/open-apis/corehr/v1/locations` | `../src/service/hire/recruitment_config/location/mod.rs` | 62 | ✅ 已实现 |
| 959 | 启用/停用地点 | POST | `/open-apis/corehr/v2/locations/active` | `未找到` | - | ❌ 未实现 |
| 960 | 删除地点 | DELETE | `/open-apis/corehr/v1/locations/:location_id` | `未找到` | - | ❌ 未实现 |
| 961 | 删除地点地址 | DELETE | `/open-apis/corehr/v2/locations/:location_id/addresses/:address_id` | `未找到` | - | ❌ 未实现 |
| 962 | 更新地点地址 | PATCH | `/open-apis/corehr/v2/locations/:location_id/addresses/:address_id` | `未找到` | - | ❌ 未实现 |
| 963 | 添加地点地址 | POST | `/open-apis/corehr/v2/locations/:location_id/addresses` | `未找到` | - | ❌ 未实现 |
| 964 | 创建公司 | POST | `/open-apis/corehr/v1/companies` | `../src/service/corehr/organization/mod.rs` | 307 | ✅ 已实现 |
| 965 | 更新公司 | PATCH | `/open-apis/corehr/v1/companies/:company_id` | `未找到` | - | ❌ 未实现 |
| 966 | 启用/停用公司 | POST | `/open-apis/corehr/v2/companies/active` | `未找到` | - | ❌ 未实现 |
| 967 | 查询单个公司 | GET | `/open-apis/corehr/v1/companies/:company_id` | `未找到` | - | ❌ 未实现 |
| 968 | 批量查询公司 | GET | `/open-apis/corehr/v1/companies` | `../src/service/corehr/organization/mod.rs` | 307 | ✅ 已实现 |
| 969 | 查询当前生效信息变更公司 | GET | `/open-apis/corehr/v2/companies/query_recent_change` | `未找到` | - | ❌ 未实现 |
| 970 | 通过公司 ID 批量获取公司信息 | POST | `/open-apis/corehr/v2/companies/batch_get` | `../src/service/corehr/organization/mod.rs` | 161 | ✅ 已实现 |
| 971 | 删除公司 | DELETE | `/open-apis/corehr/v1/companies/:company_id` | `未找到` | - | ❌ 未实现 |
| 972 | 创建成本中心 | POST | `/open-apis/corehr/v2/cost_centers` | `未找到` | - | ❌ 未实现 |
| 973 | 启用 / 停用成本中心 | PATCH | `/open-apis/corehr/v2/cost_centers/:cost_center_id` | `未找到` | - | ❌ 未实现 |
| 974 | 查询当前生效信息发生变更的成本中心 | GET | `/open-apis/corehr/v2/cost_centers/query_recent_change` | `未找到` | - | ❌ 未实现 |
| 975 | 搜索成本中心信息 | POST | `/open-apis/corehr/v2/cost_centers/search` | `../src/service/corehr/lifecycle/mod.rs` | 191 | ✅ 已实现 |
| 976 | 删除成本中心 | DELETE | `/open-apis/corehr/v2/cost_centers/:cost_center_id` | `未找到` | - | ❌ 未实现 |
| 977 | 创建成本中心版本 | POST | `/open-apis/corehr/v2/cost_centers/:cost_center_id/versions` | `../src/service/cloud_docs/drive/v1/file_version.rs` | 114 | ✅ 已实现 |
| 978 | 更正成本中心版本 | PATCH | `/open-apis/corehr/v2/cost_centers/:cost_center_id/versions/:version_id` | `未找到` | - | ❌ 未实现 |
| 979 | 撤销成本中心版本 | DELETE | `/open-apis/corehr/v2/cost_centers/:cost_center_id/versions/:version_id` | `未找到` | - | ❌ 未实现 |
| 980 | 创建自定义组织 | POST | `/open-apis/corehr/v2/custom_orgs` | `未找到` | - | ❌ 未实现 |
| 981 | 更新自定义组织信息 | PATCH | `/open-apis/corehr/v2/custom_orgs/:org_id` | `未找到` | - | ❌ 未实现 |
| 982 | 更新自定义组织的匹配规则 | POST | `/open-apis/corehr/v2/custom_orgs/update_rule` | `../src/service/trust_party/searchable_visible_rules/mod.rs` | 72 | ✅ 已实现 |
| 983 | 启用/停用自定义组织 | POST | `/open-apis/corehr/v2/custom_orgs/active` | `未找到` | - | ❌ 未实现 |
| 984 | 查询自定义组织信息 | POST | `/open-apis/corehr/v2/custom_orgs/query` | `../src/service/corehr/job_management/mod.rs` | 383 | ✅ 已实现 |
| 985 | 查询当前生效信息变更的自定义组织 | GET | `/open-apis/corehr/v2/custom_orgs/query_recent_change` | `未找到` | - | ❌ 未实现 |
| 986 | 删除自定义组织 | POST | `/open-apis/corehr/v2/custom_orgs/delete_org` | `未找到` | - | ❌ 未实现 |
| 987 | 根据组织架构调整 ID 查询发起的流程信息 | GET | `/open-apis/corehr/v2/drafts/:draft_id` | `未找到` | - | ❌ 未实现 |
| 988 | 批量查询岗位调整内容 | POST | `/open-apis/corehr/v2/approval_groups/open_query_position_change_list_by_ids` | `未找到` | - | ❌ 未实现 |
| 989 | 根据流程 ID 查询组织架构调整记录 | GET | `/open-apis/corehr/v2/approval_groups/:process_id` | `未找到` | - | ❌ 未实现 |
| 990 | 批量查询部门调整内容 | POST | `/open-apis/corehr/v2/approval_groups/open_query_department_change_list_by_ids` | `未找到` | - | ❌ 未实现 |
| 991 | 批量查询人员调整内容 | POST | `/open-apis/corehr/v2/approval_groups/open_query_job_change_list_by_ids` | `未找到` | - | ❌ 未实现 |
| 992 | 创建序列 | POST | `/open-apis/corehr/v1/job_families` | `../src/service/corehr/job_management/mod.rs` | 190 | ✅ 已实现 |
| 993 | 更新序列 | PATCH | `/open-apis/corehr/v1/job_families/:job_family_id` | `../src/service/contact/v3/job_family.rs` | 66 | ✅ 已实现 |
| 994 | 查询单个序列 | GET | `/open-apis/corehr/v1/job_families/:job_family_id` | `../src/service/contact/v3/job_family.rs` | 66 | ✅ 已实现 |
| 995 | 批量查询序列 | GET | `/open-apis/corehr/v1/job_families` | `../src/service/corehr/job_management/mod.rs` | 190 | ✅ 已实现 |
| 996 | 查询当前生效信息发生变更的序列 | GET | `/open-apis/corehr/v2/job_families/query_recent_change` | `未找到` | - | ❌ 未实现 |
| 997 | 根据条件批量获取序列信息 | POST | `/open-apis/corehr/v2/job_families/batch_get` | `../src/service/corehr/organization/mod.rs` | 161 | ✅ 已实现 |
| 998 | 查询指定时间范围序列版本 | POST | `/open-apis/corehr/v2/job_families/query_multi_timeline` | `未找到` | - | ❌ 未实现 |
| 999 | 删除序列 | DELETE | `/open-apis/corehr/v1/job_families/:job_family_id` | `../src/service/contact/v3/job_family.rs` | 66 | ✅ 已实现 |
| 1000 | 新建职级 | POST | `/open-apis/corehr/v1/job_levels` | `../src/service/corehr/job_management/mod.rs` | 286 | ✅ 已实现 |
| 1001 | 更新单个职级 | PATCH | `/open-apis/corehr/v1/job_levels/:job_level_id` | `../src/service/contact/v3/job_level.rs` | 65 | ✅ 已实现 |
| 1002 | 查询单个职级 | GET | `/open-apis/corehr/v1/job_levels/:job_level_id` | `../src/service/contact/v3/job_level.rs` | 65 | ✅ 已实现 |
| 1003 | 批量查询职级 | GET | `/open-apis/corehr/v1/job_levels` | `../src/service/corehr/job_management/mod.rs` | 286 | ✅ 已实现 |
| 1004 | 查询当前生效信息发生变更的职级 | GET | `/open-apis/corehr/v2/job_levels/query_recent_change` | `未找到` | - | ❌ 未实现 |
| 1005 | 根据条件批量获取职级信息 | POST | `/open-apis/corehr/v2/job_levels/batch_get` | `../src/service/corehr/organization/mod.rs` | 161 | ✅ 已实现 |
| 1006 | 删除职级 | DELETE | `/open-apis/corehr/v1/job_levels/:job_level_id` | `../src/service/contact/v3/job_level.rs` | 65 | ✅ 已实现 |
| 1007 | 创建职等 | POST | `/open-apis/corehr/v2/job_grades` | `../src/service/corehr/job_management/mod.rs` | 383 | ✅ 已实现 |
| 1008 | 更新职等 | PATCH | `/open-apis/corehr/v2/job_grades/:job_grade_id` | `未找到` | - | ❌ 未实现 |
| 1009 | 查询职等 | POST | `/open-apis/corehr/v2/job_grades/query` | `../src/service/corehr/job_management/mod.rs` | 383 | ✅ 已实现 |
| 1010 | 查询当前生效信息发生变更的职等 | GET | `/open-apis/corehr/v2/job_grades/query_recent_change` | `未找到` | - | ❌ 未实现 |
| 1011 | 删除职等 | DELETE | `/open-apis/corehr/v2/job_grades/:job_grade_id` | `未找到` | - | ❌ 未实现 |
| 1012 | 创建通道 | POST | `/open-apis/corehr/v2/pathways` | `未找到` | - | ❌ 未实现 |
| 1013 | 更新通道 | PATCH | `/open-apis/corehr/v2/pathways/:pathway_id` | `未找到` | - | ❌ 未实现 |
| 1014 | 删除通道 | DELETE | `/open-apis/corehr/v2/pathways/:pathway_id` | `未找到` | - | ❌ 未实现 |
| 1015 | 启停用通道 | POST | `/open-apis/corehr/v2/pathways/active` | `未找到` | - | ❌ 未实现 |
| 1016 | 获取通道信息 | POST | `/open-apis/corehr/v2/pathways/batch_get` | `../src/service/corehr/organization/mod.rs` | 161 | ✅ 已实现 |
| 1017 | 创建职务 | POST | `/open-apis/corehr/v1/jobs` | `../src/service/corehr/job_management/mod.rs` | 476 | ✅ 已实现 |
| 1018 | 删除职务 | DELETE | `/open-apis/corehr/v1/jobs/:job_id` | `未找到` | - | ❌ 未实现 |
| 1019 | 更新职务 | PATCH | `/open-apis/corehr/v1/jobs/:job_id` | `未找到` | - | ❌ 未实现 |
| 1020 | 查询单个职务 | GET | `/open-apis/corehr/v2/jobs/:job_id` | `未找到` | - | ❌ 未实现 |
| 1021 | 批量查询职务 | GET | `/open-apis/corehr/v2/jobs` | `../src/service/corehr/job_management/mod.rs` | 476 | ✅ 已实现 |
| 1022 | 根据条件批量获取职务 | POST | `/open-apis/corehr/v2/jobs/batch_get` | `../src/service/corehr/organization/mod.rs` | 161 | ✅ 已实现 |
| 1023 | 查询指定时间范围职务版本 | POST | `/open-apis/corehr/v2/jobs/query_multi_timeline` | `未找到` | - | ❌ 未实现 |
| 1024 | 查询当前生效信息发生变更的职务 | GET | `/open-apis/corehr/v2/jobs/query_recent_change` | `未找到` | - | ❌ 未实现 |
| 1025 | 创建岗位信息 | POST | `/open-apis/corehr/v2/positions` | `../src/service/ehr/v1/mod.rs` | 441 | ✅ 已实现 |
| 1026 | 更新岗位信息 | PATCH | `/open-apis/corehr/v2/positions/:position_id` | `未找到` | - | ❌ 未实现 |
| 1027 | 查询岗位信息 | POST | `/open-apis/corehr/v2/positions/query` | `../src/service/corehr/job_management/mod.rs` | 383 | ✅ 已实现 |
| 1028 | 查询指定时范围内当前版本信息发生变更的岗位 | GET | `/open-apis/corehr/v2/positions/query_recent_change` | `未找到` | - | ❌ 未实现 |
| 1029 | 启停用岗位 | POST | `/open-apis/corehr/v2/positions/active` | `未找到` | - | ❌ 未实现 |
| 1030 | 删除岗位 | POST | `/open-apis/corehr/v2/positions/del_position` | `未找到` | - | ❌ 未实现 |
| 1031 | 撤销入职 | POST | `/open-apis/corehr/v2/pre_hires/withdraw_onboarding` | `未找到` | - | ❌ 未实现 |
| 1032 | 恢复入职 | POST | `/open-apis/corehr/v2/pre_hires/restore_flow_instance` | `未找到` | - | ❌ 未实现 |
| 1033 | 直接创建待入职 | POST | `/open-apis/corehr/v2/pre_hires` | `未找到` | - | ❌ 未实现 |
| 1034 | 更新待入职信息 | PATCH | `/open-apis/corehr/v2/pre_hires/:pre_hire_id` | `未找到` | - | ❌ 未实现 |
| 1035 | 删除待入职信息 | DELETE | `/open-apis/corehr/v2/pre_hires/:pre_hire_id` | `未找到` | - | ❌ 未实现 |
| 1036 | 查询待入职信息 | POST | `/open-apis/corehr/v2/pre_hires/query` | `../src/service/corehr/job_management/mod.rs` | 383 | ✅ 已实现 |
| 1037 | 搜索待入职信息 | POST | `/open-apis/corehr/v2/pre_hires/search` | `../src/service/corehr/lifecycle/mod.rs` | 191 | ✅ 已实现 |
| 1038 | 流转入职任务 | POST | `/open-apis/corehr/v2/pre_hires/:pre_hire_id/transit_task` | `未找到` | - | ❌ 未实现 |
| 1039 | 流转入职任务 | POST | `/open-apis/corehr/v2/pre_hires/transform_onboarding_task` | `未找到` | - | ❌ 未实现 |
| 1040 | 操作员工完成入职 | POST | `/open-apis/corehr/v2/pre_hires/:pre_hire_id/complete` | `未找到` | - | ❌ 未实现 |
| 1041 | 新增试用期考核信息 | POST | `/open-apis/corehr/v2/probation/assessments` | `未找到` | - | ❌ 未实现 |
| 1042 | 启用/停用试用期考核功能 | POST | `/open-apis/corehr/v2/probation/enable_disable_assessment` | `未找到` | - | ❌ 未实现 |
| 1043 | 更新试用期考核信息 | PATCH | `/open-apis/corehr/v2/probation/assessments/:assessment_id` | `未找到` | - | ❌ 未实现 |
| 1044 | 搜索试用期信息 | POST | `/open-apis/corehr/v2/probation/search` | `../src/service/corehr/lifecycle/mod.rs` | 191 | ✅ 已实现 |
| 1045 | 删除试用期考核信息 | DELETE | `/open-apis/corehr/v2/probation/assessments/:assessment_id` | `未找到` | - | ❌ 未实现 |
| 1046 | 发起转正 | POST | `/open-apis/corehr/v2/probation/submit` | `../src/service/hire/ecological_docking/exam/mod.rs` | 512 | ✅ 已实现 |
| 1047 | 撤销转正 | POST | `/open-apis/corehr/v2/probation/withdraw` | `../src/service/hire/candidate_management/offer/mod.rs` | 569 | ✅ 已实现 |
| 1048 | 发起员工异动 | POST | `/open-apis/corehr/v2/job_changes` | `未找到` | - | ❌ 未实现 |
| 1049 | 获取异动类型列表 | GET | `/open-apis/corehr/v1/transfer_types/query` | `../src/service/corehr/job_management/mod.rs` | 383 | ✅ 已实现 |
| 1050 | 获取异动原因列表 | GET | `/open-apis/corehr/v1/transfer_reasons/query` | `../src/service/corehr/job_management/mod.rs` | 383 | ✅ 已实现 |
| 1051 | 搜索员工异动信息 | POST | `/open-apis/corehr/v2/job_changes/search` | `../src/service/corehr/lifecycle/mod.rs` | 191 | ✅ 已实现 |
| 1052 | 撤销异动 | POST | `/open-apis/corehr/v2/job_changes/:job_change_id/revoke` | `未找到` | - | ❌ 未实现 |
| 1053 | 发起员工异动(不推荐) | POST | `/open-apis/corehr/v1/job_changes` | `未找到` | - | ❌ 未实现 |
| 1054 | 查询员工离职原因列表 | POST | `/open-apis/corehr/v1/offboardings/query` | `../src/service/corehr/job_management/mod.rs` | 383 | ✅ 已实现 |
| 1055 | 操作员工离职 | POST | `/open-apis/corehr/v2/offboardings/submit_v2` | `未找到` | - | ❌ 未实现 |
| 1056 | 编辑离职信息 | POST | `/open-apis/corehr/v2/offboardings/edit` | `../src/service/apass/flow/mod.rs` | 292 | ✅ 已实现 |
| 1057 | 撤销离职 | POST | `/open-apis/corehr/v2/offboardings/revoke` | `未找到` | - | ❌ 未实现 |
| 1058 | 搜索离职信息 | POST | `/open-apis/corehr/v1/offboardings/search` | `../src/service/corehr/lifecycle/mod.rs` | 191 | ✅ 已实现 |
| 1059 | 新建合同 | POST | `/open-apis/corehr/v1/contracts` | `../src/service/feishu_people/core/v1/contracts.rs` | 569 | ✅ 已实现 |
| 1060 | 更新合同 | PATCH | `/open-apis/corehr/v1/contracts/:contract_id` | `未找到` | - | ❌ 未实现 |
| 1061 | 删除合同 | DELETE | `/open-apis/corehr/v1/contracts/:contract_id` | `未找到` | - | ❌ 未实现 |
| 1062 | 查询单个合同 | GET | `/open-apis/corehr/v1/contracts/:contract_id` | `未找到` | - | ❌ 未实现 |
| 1063 | 批量查询合同 | GET | `/open-apis/corehr/v1/contracts` | `../src/service/feishu_people/core/v1/contracts.rs` | 569 | ✅ 已实现 |
| 1064 | 搜索合同 | POST | `/open-apis/corehr/v2/contracts/search` | `../src/service/corehr/lifecycle/mod.rs` | 191 | ✅ 已实现 |
| 1065 | 批量创建/更新明细行 | POST | `/open-apis/corehr/v2/workforce_plan_detail_row/batchSave` | `未找到` | - | ❌ 未实现 |
| 1066 | 批量删除明细行 | POST | `/open-apis/corehr/v2/workforce_plan_detail_row/batchDelete` | `未找到` | - | ❌ 未实现 |
| 1067 | 批量创建/更新填报行 | POST | `/open-apis/corehr/v2/report_detail_row/batchSave` | `未找到` | - | ❌ 未实现 |
| 1068 | 批量删除填报行 | POST | `/open-apis/corehr/v2/report_detail_row/batchDelete` | `未找到` | - | ❌ 未实现 |
| 1069 | 查询编制规划方案 | GET | `/open-apis/corehr/v2/workforce_plans` | `未找到` | - | ❌ 未实现 |
| 1070 | 查询编制规划明细信息（不支持自定义组织） | POST | `/open-apis/corehr/v2/workforce_plan_details/batch` | `../src/service/corehr/organization/mod.rs` | 161 | ✅ 已实现 |
| 1071 | 查询编制规划明细信息（支持自定义组织） | POST | `/open-apis/corehr/v2/workforce_plan_details/batch_v2` | `未找到` | - | ❌ 未实现 |
| 1072 | 创建假期发放记录 | POST | `/open-apis/corehr/v1/leave_granting_records` | `未找到` | - | ❌ 未实现 |
| 1073 | 删除假期发放记录 | DELETE | `/open-apis/corehr/v1/leave_granting_records/:leave_granting_record_id` | `未找到` | - | ❌ 未实现 |
| 1074 | 获取假期类型列表 | GET | `/open-apis/corehr/v1/leaves/leave_types` | `未找到` | - | ❌ 未实现 |
| 1075 | 批量查询员工假期余额 | GET | `/open-apis/corehr/v1/leaves/leave_balances` | `未找到` | - | ❌ 未实现 |
| 1076 | 批量查询员工请假记录 | GET | `/open-apis/corehr/v1/leaves/leave_request_history` | `未找到` | - | ❌ 未实现 |
| 1077 | 获取工作日历 | POST | `/open-apis/corehr/v1/leaves/work_calendar` | `未找到` | - | ❌ 未实现 |
| 1078 | 根据适用条件获取工作日历 ID | GET | `/open-apis/corehr/v1/leaves/calendar_by_scope` | `未找到` | - | ❌ 未实现 |
| 1079 | 获取工作日历日期详情 | POST | `/open-apis/corehr/v1/leaves/work_calendar_date` | `未找到` | - | ❌ 未实现 |
| 1080 | 批量查询用户授权 | GET | `/open-apis/corehr/v1/authorizations/query` | `../src/service/corehr/job_management/mod.rs` | 383 | ✅ 已实现 |
| 1081 | 查询单个用户授权 | GET | `/open-apis/corehr/v1/authorizations/get_by_param` | `未找到` | - | ❌ 未实现 |
| 1082 | 批量获取角色列表 | GET | `/open-apis/corehr/v1/security_groups` | `未找到` | - | ❌ 未实现 |
| 1083 | 为用户授权角色 | POST | `/open-apis/corehr/v1/authorizations/add_role_assign` | `未找到` | - | ❌ 未实现 |
| 1084 | 更新用户被授权的数据范围 | POST | `/open-apis/corehr/v1/authorizations/update_role_assign` | `未找到` | - | ❌ 未实现 |
| 1085 | 移除用户被授权的角色 | POST | `/open-apis/corehr/v1/authorizations/remove_role_assign` | `未找到` | - | ❌ 未实现 |
| 1086 | 查询员工 HRBP / 属地 BP | POST | `/open-apis/corehr/v2/employees/bps/batch_get` | `../src/service/corehr/organization/mod.rs` | 161 | ✅ 已实现 |
| 1087 | 查询部门 HRBP | POST | `/open-apis/corehr/v2/bps/get_by_department` | `../src/service/feishu_people/core/v1/positions.rs` | 278 | ✅ 已实现 |
| 1088 | 查询部门 / 地点的 HRBP / 属地 BP | POST | `/open-apis/corehr/v1/security_groups/query` | `../src/service/corehr/job_management/mod.rs` | 383 | ✅ 已实现 |
| 1089 | 获取 HRBP 列表 | GET | `/open-apis/corehr/v2/bps` | `未找到` | - | ❌ 未实现 |
| 1090 | 获取组织类角色授权列表 | POST | `/open-apis/corehr/v1/assigned_users/search` | `../src/service/corehr/lifecycle/mod.rs` | 191 | ✅ 已实现 |
| 1091 | 查询流程实例列表 | GET | `/open-apis/corehr/v2/processes` | `../src/service/hire/recruitment_config/job_process/mod.rs` | 253 | ✅ 已实现 |
| 1092 | 获取单个流程详情 | GET | `/open-apis/corehr/v2/processes/:process_id` | `未找到` | - | ❌ 未实现 |
| 1093 | 获取流程数据 | GET | `/open-apis/corehr/v2/processes/:process_id/flow_variable_data` | `未找到` | - | ❌ 未实现 |
| 1094 | 获取流程表单数据 | GET | `/open-apis/corehr/v2/processes/:process_id/form_variable_data` | `未找到` | - | ❌ 未实现 |
| 1095 | 撤销流程 | PUT | `/open-apis/corehr/v2/process_revoke/:process_id` | `未找到` | - | ❌ 未实现 |
| 1096 | 撤回流程 | PUT | `/open-apis/corehr/v2/process_withdraw/:process_id` | `未找到` | - | ❌ 未实现 |
| 1097 | 获取指定人员审批任务列表 | GET | `/open-apis/corehr/v2/approvers` | `未找到` | - | ❌ 未实现 |
| 1098 | 通过/拒绝审批任务 | PUT | `/open-apis/corehr/v2/processes/:process_id/approvers/:approver_id` | `未找到` | - | ❌ 未实现 |
| 1099 | 加签审批任务 | PUT | `/open-apis/corehr/v2/processes/:process_id/extra` | `../src/service/ai/document_ai/mod.rs` | 265 | ✅ 已实现 |
| 1100 | 转交审批任务 | PUT | `/open-apis/corehr/v2/processes/:process_id/transfer` | `../src/service/apass/flow/mod.rs` | 211 | ✅ 已实现 |
| 1101 | 创建薪资档案 | POST | `/open-apis/compensation/v1/archives` | `未找到` | - | ❌ 未实现 |
| 1102 | 批量查询员工薪资档案 | POST | `/open-apis/compensation/v1/archives/query` | `../src/service/attendance/v1/user_task.rs` | 86 | ✅ 已实现 |
| 1103 | 批量查询薪资项 | GET | `/open-apis/compensation/v1/items` | `../src/service/payroll/v1/acct_item.rs` | 123 | ✅ 已实现 |
| 1104 | 批量查询薪资统计指标 | GET | `/open-apis/compensation/v1/indicators` | `未找到` | - | ❌ 未实现 |
| 1105 | 批量获取薪资项分类信息 | GET | `/open-apis/compensation/v1/item_categories` | `未找到` | - | ❌ 未实现 |
| 1106 | 获取员工薪资标准 | GET | `/open-apis/corehr/v1/compensation_standards/match` | `../src/service/lingo/entity/mod.rs` | 187 | ✅ 已实现 |
| 1107 | 批量查询薪资方案 | GET | `/open-apis/compensation/v1/plans` | `../src/service/application/v6/appstore_paid_info/mod.rs` | 58 | ✅ 已实现 |
| 1108 | 批量查询定调薪原因 | GET | `/open-apis/compensation/v1/change_reasons` | `未找到` | - | ❌ 未实现 |
| 1109 | 获取险种配置列表 | GET | `/open-apis/compensation/v1/social_insurances` | `未找到` | - | ❌ 未实现 |
| 1110 | 根据方案ID和生效日期批量查询参保方案 | POST | `/open-apis/compensation/v1/social_plans/query` | `../src/service/attendance/v1/user_task.rs` | 86 | ✅ 已实现 |
| 1111 | 根据生效日期分页查询参保方案 | GET | `/open-apis/compensation/v1/social_plans` | `未找到` | - | ❌ 未实现 |
| 1112 | 通过员工ID批量获取社保增减员记录 | POST | `/open-apis/compensation/v1/social_archive_adjust_record/query` | `../src/service/attendance/v1/user_task.rs` | 86 | ✅ 已实现 |
| 1113 | 批量获取员工参保档案 | POST | `/open-apis/compensation/v1/social_archive/query` | `../src/service/attendance/v1/user_task.rs` | 86 | ✅ 已实现 |
| 1114 | 批量创建一次性支付记录 | POST | `/open-apis/compensation/v1/lump_sum_payment/batch_create` | `../src/service/contact/v3/functional_role_member.rs` | 42 | ✅ 已实现 |
| 1115 | 批量更正一次性支付记录 | POST | `/open-apis/compensation/v1/lump_sum_payment/batch_update` | `../src/service/apass/object/mod.rs` | 304 | ✅ 已实现 |
| 1116 | 查询一次性支付授予记录 | POST | `/open-apis/compensation/v1/lump_sum_payment/query` | `../src/service/attendance/v1/user_task.rs` | 86 | ✅ 已实现 |
| 1117 | 查询一次性支付授予明细 | POST | `/open-apis/compensation/v1/lump_sum_payment/query_detail` | `../src/service/performance/review_data/mod.rs` | 66 | ✅ 已实现 |
| 1118 | 批量删除一次性支付记录 | POST | `/open-apis/compensation/v1/lump_sum_payment/batch_remove` | `../src/service/contact/v3/group_member.rs` | 105 | ✅ 已实现 |
| 1119 | 查询经常性支付记录 | POST | `/open-apis/compensation/v1/recurring_payment/query` | `../src/service/attendance/v1/user_task.rs` | 86 | ✅ 已实现 |
| 1120 | 批量更正经常性支付记录 | POST | `/open-apis/compensation/v1/recurring_payment/batch_update` | `../src/service/apass/object/mod.rs` | 304 | ✅ 已实现 |
| 1121 | 批量删除经常性支付记录 | POST | `/open-apis/compensation/v1/recurring_payment/batch_remove` | `../src/service/contact/v3/group_member.rs` | 105 | ✅ 已实现 |
| 1122 | 批量创建经常性支付记录 | POST | `/open-apis/compensation/v1/recurring_payment/batch_create` | `../src/service/contact/v3/functional_role_member.rs` | 42 | ✅ 已实现 |
| 1123 | 批量查询算薪项 | GET | `/open-apis/payroll/v1/acct_items` | `../src/service/payroll/v1/acct_item.rs` | 123 | ✅ 已实现 |
| 1124 | 获取薪资组基本信息 | GET | `/open-apis/payroll/v1/paygroups` | `../src/service/payroll/v1/paygroup.rs` | 109 | ✅ 已实现 |
| 1125 | 获取外部数据源配置信息 | GET | `/open-apis/payroll/v1/datasources` | `../src/service/payroll/v1/datasource.rs` | 108 | ✅ 已实现 |
| 1126 | 创建 / 更新外部算薪数据 | POST | `/open-apis/payroll/v1/datasource_records/save` | `未找到` | - | ❌ 未实现 |
| 1127 | 批量查询外部算薪数据记录 | POST | `/open-apis/payroll/v1/datasource_records/query` | `../src/service/payroll/payment_detail/mod.rs` | 163 | ✅ 已实现 |
| 1128 | 封存发薪活动 | POST | `/open-apis/payroll/v1/payment_activitys/archive` | `../src/service/payroll/payment_activity/mod.rs` | 212 | ✅ 已实现 |
| 1129 | 查询发薪活动列表 | GET | `/open-apis/payroll/v1/payment_activitys` | `未找到` | - | ❌ 未实现 |
| 1130 | 查询发薪活动明细列表 | GET | `/open-apis/payroll/v1/payment_activity_details` | `未找到` | - | ❌ 未实现 |
| 1131 | 批量查询发薪明细 | POST | `/open-apis/payroll/v1/payment_detail/query` | `../src/service/payroll/payment_detail/mod.rs` | 163 | ✅ 已实现 |
| 1132 | 查询成本分摊报表明细 | GET | `/open-apis/payroll/v1/cost_allocation_details` | `未找到` | - | ❌ 未实现 |
| 1133 | 查询成本分摊报表汇总数据 | GET | `/open-apis/payroll/v1/cost_allocation_reports` | `未找到` | - | ❌ 未实现 |
| 1134 | 批量查询成本分摊方案 | GET | `/open-apis/payroll/v1/cost_allocation_plans` | `未找到` | - | ❌ 未实现 |
| 1135 | 获取申请表模板列表 | GET | `/open-apis/hire/v1/portal_apply_schemas` | `未找到` | - | ❌ 未实现 |
| 1136 | 查询地点列表 | POST | `/open-apis/hire/locations/query` | `../src/service/hire/recruitment_config/location/mod.rs` | 62 | ✅ 已实现 |
| 1137 | 获取地址列表 | GET | `/open-apis/hire/v1/locations` | `../src/service/hire/recruitment_config/location/mod.rs` | 62 | ✅ 已实现 |
| 1138 | 获取角色详情 | GET | `/open-apis/hire/v1/roles/:role_id` | `../src/service/hire/recruitment_config/auth/mod.rs` | 77 | ✅ 已实现 |
| 1139 | 获取角色列表 | GET | `/open-apis/hire/v1/roles` | `../src/service/hire/recruitment_config/auth/mod.rs` | 112 | ✅ 已实现 |
| 1140 | 获取用户角色列表 | GET | `/open-apis/hire/v1/user_roles` | `../src/service/hire/recruitment_config/auth/mod.rs` | 148 | ✅ 已实现 |
| 1141 | 新建职位 | POST | `/open-apis/hire/v1/jobs/combined_create` | `未找到` | - | ❌ 未实现 |
| 1142 | 更新职位 | POST | `/open-apis/hire/v1/jobs/:job_id/combined_update` | `未找到` | - | ❌ 未实现 |
| 1143 | 更新职位设置 | POST | `/open-apis/hire/v1/jobs/:job_id/update_config` | `未找到` | - | ❌ 未实现 |
| 1144 | 更新职位相关人员 | POST | `/open-apis/hire/v1/jobs/:job_id/managers/batch_update` | `../src/service/apass/object/mod.rs` | 304 | ✅ 已实现 |
| 1145 | 获取职位详情 | GET | `/open-apis/hire/v1/jobs/:job_id/get_detail` | `../src/service/contact/v3/group.rs` | 266 | ✅ 已实现 |
| 1146 | 获取职位信息 | GET | `/open-apis/hire/v1/jobs/:job_id` | `未找到` | - | ❌ 未实现 |
| 1147 | 获取职位上的招聘人员信息 | GET | `/open-apis/hire/v1/jobs/:job_id/recruiter` | `未找到` | - | ❌ 未实现 |
| 1148 | 获取职位设置 | GET | `/open-apis/hire/v1/jobs/:job_id/config` | `../src/service/hire/get_candidates/external_system/mod.rs` | 225 | ✅ 已实现 |
| 1149 | 获取职位列表 | GET | `/open-apis/hire/v1/jobs` | `../src/service/hire/recruitment_config/job/mod.rs` | 262 | ✅ 已实现 |
| 1150 | 关闭职位 | POST | `/open-apis/hire/v1/jobs/:job_id/close` | `../src/service/hire/recruitment_config/job/mod.rs` | 312 | ✅ 已实现 |
| 1151 | 重启职位 | POST | `/open-apis/hire/v1/jobs/:job_id/open` | `../src/service/hire/recruitment_config/job/mod.rs` | 341 | ✅ 已实现 |
| 1152 | 获取职位模板 | GET | `/open-apis/hire/v1/job_schemas` | `未找到` | - | ❌ 未实现 |
| 1153 | 发布职位广告 | POST | `/open-apis/hire/v1/advertisements/:advertisement_id/publish` | `../src/service/hire/get_candidates/website/mod.rs` | 291 | ✅ 已实现 |
| 1154 | 获取职位广告发布记录 | POST | `/open-apis/hire/v1/job_publish_records/search` | `../src/service/attendance/v1/group.rs` | 154 | ✅ 已实现 |
| 1155 | 获取职能分类列表 | GET | `/open-apis/hire/v1/job_functions` | `未找到` | - | ❌ 未实现 |
| 1156 | 获取职位类别列表 | GET | `/open-apis/hire/v1/job_types` | `未找到` | - | ❌ 未实现 |
| 1157 | 创建招聘需求 | POST | `/open-apis/hire/v1/job_requirements` | `未找到` | - | ❌ 未实现 |
| 1158 | 更新招聘需求 | PUT | `/open-apis/hire/v1/job_requirements/:job_requirement_id` | `未找到` | - | ❌ 未实现 |
| 1159 | 获取招聘需求信息 | POST | `/open-apis/hire/job_requirements/search` | `../src/service/attendance/v1/group.rs` | 154 | ✅ 已实现 |
| 1160 | 获取招聘需求列表 | GET | `/open-apis/hire/v1/job_requirements` | `未找到` | - | ❌ 未实现 |
| 1161 | 删除招聘需求 | DELETE | `/open-apis/hire/v1/job_requirements/:job_requirement_id` | `未找到` | - | ❌ 未实现 |
| 1162 | 获取招聘需求模板列表 | GET | `/open-apis/hire/v1/job_requirement_schemas` | `未找到` | - | ❌ 未实现 |
| 1163 | 获取招聘流程信息 | GET | `/open-apis/hire/v1/job_processes` | `未找到` | - | ❌ 未实现 |
| 1164 | 获取项目列表 | GET | `/open-apis/hire/v1/subjects` | `../src/service/hire/recruitment_config/subject/mod.rs` | 286 | ✅ 已实现 |
| 1165 | 获取人才标签信息列表 | GET | `/open-apis/hire/v1/talent_tags` | `../src/service/hire/recruitment_config/application.rs` | 67 | ✅ 已实现 |
| 1166 | 获取信息登记表列表 | GET | `/open-apis/hire/v1/registration_schemas` | `未找到` | - | ❌ 未实现 |
| 1167 | 获取面试评价表列表 | GET | `/open-apis/hire/v1/interview_feedback_forms` | `未找到` | - | ❌ 未实现 |
| 1168 | 获取面试轮次类型列表 | GET | `/open-apis/hire/v1/interview_round_types` | `未找到` | - | ❌ 未实现 |
| 1169 | 获取面试登记表列表 | GET | `/open-apis/hire/v1/interview_registration_schemas` | `未找到` | - | ❌ 未实现 |
| 1170 | 查询面试官信息列表 | GET | `/open-apis/hire/v1/interviewers` | `未找到` | - | ❌ 未实现 |
| 1171 | 更新面试官信息 | PATCH | `/open-apis/hire/v1/interviewers/:interviewer_id` | `未找到` | - | ❌ 未实现 |
| 1172 | 获取 Offer 审批流列表 | GET | `/open-apis/hire/v1/offer_approval_templates` | `未找到` | - | ❌ 未实现 |
| 1173 | 更新 Offer 申请表自定义字段 | PUT | `/open-apis/hire/v1/offer_custom_fields/:offer_custom_field_id` | `未找到` | - | ❌ 未实现 |
| 1174 | 获取 Offer 申请表信息 | GET | `/open-apis/hire/v1/offer_application_forms/:offer_application_form_id` | `未找到` | - | ❌ 未实现 |
| 1175 | 获取 Offer 申请表列表 | GET | `/open-apis/hire/v1/offer_application_forms` | `未找到` | - | ❌ 未实现 |
| 1176 | 查询人才内推信息 | POST | `/open-apis/hire/v1/referrals/search` | `../src/service/attendance/v1/group.rs` | 154 | ✅ 已实现 |
| 1177 | 获取内推官网下职位广告列表 | GET | `/open-apis/hire/v1/referral_websites/job_posts` | `未找到` | - | ❌ 未实现 |
| 1178 | 获取内推官网下职位广告详情 | GET | `/open-apis/hire/v1/referral_websites/job_posts/:job_post_id` | `未找到` | - | ❌ 未实现 |
| 1179 | 获取内推信息 | GET | `/open-apis/hire/v1/referrals/get_by_application` | `未找到` | - | ❌ 未实现 |
| 1180 | 新建招聘官网推广渠道 | POST | `/open-apis/hire/v1/websites/:website_id/channels` | `未找到` | - | ❌ 未实现 |
| 1181 | 删除招聘官网推广渠道 | DELETE | `/open-apis/hire/v1/websites/:website_id/channels/:channel_id` | `未找到` | - | ❌ 未实现 |
| 1182 | 更新招聘官网推广渠道 | PUT | `/open-apis/hire/v1/websites/:website_id/channels/:channel_id` | `未找到` | - | ❌ 未实现 |
| 1183 | 获取招聘官网推广渠道列表 | GET | `/open-apis/hire/v1/websites/:website_id/channels` | `未找到` | - | ❌ 未实现 |
| 1184 | 新建招聘官网用户 | POST | `/open-apis/hire/v1/websites/:website_id/site_users` | `../src/service/hire/recruitment_config/auth/mod.rs` | 148 | ✅ 已实现 |
| 1185 | 获取招聘官网下职位广告详情 | GET | `/open-apis/hire/v1/websites/:website_id/job_posts/:job_post_id` | `未找到` | - | ❌ 未实现 |
| 1186 | 搜索招聘官网下的职位广告列表 | POST | `/open-apis/hire/v1/websites/:website_id/job_posts/search` | `../src/service/attendance/v1/group.rs` | 154 | ✅ 已实现 |
| 1187 | 获取招聘官网下的职位广告列表 | GET | `/open-apis/hire/v1/websites/:website_id/job_posts` | `未找到` | - | ❌ 未实现 |
| 1188 | 新建招聘官网投递 | POST | `/open-apis/hire/v1/websites/:website_id/deliveries/create_by_resume` | `未找到` | - | ❌ 未实现 |
| 1189 | 根据简历附件创建招聘官网投递任务 | POST | `/open-apis/hire/v1/websites/:website_id/deliveries/create_by_attachment` | `未找到` | - | ❌ 未实现 |
| 1190 | 获取招聘官网投递任务结果 | GET | `/open-apis/hire/v1/websites/:website_id/delivery_tasks/:delivery_task_id` | `../src/service/hire/attachment/mod.rs` | 236 | ✅ 已实现 |
| 1191 | 获取招聘官网列表 | GET | `/open-apis/hire/v1/websites` | `未找到` | - | ❌ 未实现 |
| 1192 | 设置猎头保护期 | POST | `/open-apis/hire/v1/agencies/protect` | `未找到` | - | ❌ 未实现 |
| 1193 | 获取猎头供应商信息 | GET | `/open-apis/hire/v1/agencies/:agency_id` | `未找到` | - | ❌ 未实现 |
| 1194 | 查询猎头保护期信息 | POST | `/open-apis/hire/v1/agencies/protection_period/search` | `../src/service/attendance/v1/group.rs` | 154 | ✅ 已实现 |
| 1195 | 查询猎头供应商信息 | GET | `/open-apis/hire/v1/agencies/query` | `../src/service/hire/recruitment_config/location/mod.rs` | 62 | ✅ 已实现 |
| 1196 | 查询猎头供应商下猎头列表 | POST | `/open-apis/hire/v1/agencies/get_agency_account` | `未找到` | - | ❌ 未实现 |
| 1197 | 搜索猎头供应商列表 | POST | `/open-apis/hire/v1/agencies/batch_query` | `../src/service/apass/object/mod.rs` | 333 | ✅ 已实现 |
| 1198 | 禁用/取消禁用猎头 | POST | `/open-apis/hire/v1/agencies/operate_agency_account` | `未找到` | - | ❌ 未实现 |
| 1199 | 创建人才外部信息 | POST | `/open-apis/hire/v1/talents/:talent_id/external_info` | `未找到` | - | ❌ 未实现 |
| 1200 | 更新人才外部信息 | PUT | `/open-apis/hire/v1/talents/:talent_id/external_info` | `未找到` | - | ❌ 未实现 |
| 1201 | 创建外部投递 | POST | `/open-apis/hire/v1/external_applications` | `未找到` | - | ❌ 未实现 |
| 1202 | 更新外部投递 | PUT | `/open-apis/hire/v1/external_applications/:external_application_id` | `未找到` | - | ❌ 未实现 |
| 1203 | 查询外部投递列表 | GET | `/open-apis/hire/v1/external_applications` | `未找到` | - | ❌ 未实现 |
| 1204 | 删除外部投递 | DELETE | `/open-apis/hire/v1/external_applications/:external_application_id` | `未找到` | - | ❌ 未实现 |
| 1205 | 创建外部面试 | POST | `/open-apis/hire/v1/external_interviews` | `未找到` | - | ❌ 未实现 |
| 1206 | 更新外部面试 | PUT | `/open-apis/hire/v1/external_interviews/:external_interview_id` | `未找到` | - | ❌ 未实现 |
| 1207 | 查询外部面试列表 | POST | `/open-apis/hire/v1/external_interviews/batch_query` | `../src/service/apass/object/mod.rs` | 333 | ✅ 已实现 |
| 1208 | 删除外部面试 | DELETE | `/open-apis/hire/v1/external_interviews/:external_interview_id` | `未找到` | - | ❌ 未实现 |
| 1209 | 创建外部面评 | POST | `/open-apis/hire/v1/external_interview_assessments` | `未找到` | - | ❌ 未实现 |
| 1210 | 更新外部面评 | PATCH | `/open-apis/hire/v1/external_interview_assessments/:external_interview_assessment_id` | `未找到` | - | ❌ 未实现 |
| 1211 | 创建外部 Offer | POST | `/open-apis/hire/v1/external_offers` | `未找到` | - | ❌ 未实现 |
| 1212 | 更新外部 Offer | PUT | `/open-apis/hire/v1/external_offers/:external_offer_id` | `未找到` | - | ❌ 未实现 |
| 1213 | 查询外部 Offer 列表 | POST | `/open-apis/hire/v1/external_offers/batch_query` | `../src/service/apass/object/mod.rs` | 333 | ✅ 已实现 |
| 1214 | 删除外部 Offer | DELETE | `/open-apis/hire/v1/external_offers/:external_offer_id` | `未找到` | - | ❌ 未实现 |
| 1215 | 创建外部背调 | POST | `/open-apis/hire/v1/external_background_checks` | `未找到` | - | ❌ 未实现 |
| 1216 | 更新外部背调 | PUT | `/open-apis/hire/v1/external_background_checks/:external_background_check_id` | `未找到` | - | ❌ 未实现 |
| 1217 | 查询外部背调列表 | POST | `/open-apis/hire/v1/external_background_checks/batch_query` | `../src/service/apass/object/mod.rs` | 333 | ✅ 已实现 |
| 1218 | 删除外部背调 | DELETE | `/open-apis/hire/v1/external_background_checks/:external_background_check_id` | `未找到` | - | ❌ 未实现 |
| 1219 | 导入外部内推奖励 | POST | `/open-apis/hire/v1/external_referral_rewards` | `未找到` | - | ❌ 未实现 |
| 1220 | 删除外部内推奖励 | DELETE | `/open-apis/hire/v1/external_referral_rewards/:external_referral_reward_id` | `未找到` | - | ❌ 未实现 |
| 1221 | 批量加入/移除人才库中人才 | POST | `/open-apis/hire/v1/talent_pools/:talent_pool_id/batch_change_talent_pool` | `未找到` | - | ❌ 未实现 |
| 1222 | 获取人才库列表 | GET | `/open-apis/hire/v1/talent_pools` | `未找到` | - | ❌ 未实现 |
| 1223 | 将人才加入人才库 | POST | `/open-apis/hire/v1/talent_pools/:talent_pool_id/talent_relationship` | `未找到` | - | ❌ 未实现 |
| 1224 | 操作人才标签 | POST | `/open-apis/hire/talents/:talent_id/tag` | `../src/service/hire/recruitment_config/application.rs` | 67 | ✅ 已实现 |
| 1225 | 创建人才 | POST | `/open-apis/hire/v1/talents/combined_create` | `未找到` | - | ❌ 未实现 |
| 1226 | 更新人才 | POST | `/open-apis/hire/v1/talents/combined_update` | `未找到` | - | ❌ 未实现 |
| 1227 | 将人才加入指定文件夹 | POST | `/open-apis/hire/v1/talents/add_to_folder` | `未找到` | - | ❌ 未实现 |
| 1228 | 将人才从指定文件夹移除 | POST | `/open-apis/hire/v1/talents/remove_to_folder` | `未找到` | - | ❌ 未实现 |
| 1229 | 获取人才文件夹列表 | GET | `/open-apis/hire/v1/talent_folders` | `未找到` | - | ❌ 未实现 |
| 1230 | 批量获取人才ID | POST | `/open-apis/hire/v1/talents/batch_get_id` | `未找到` | - | ❌ 未实现 |
| 1231 | 获取人才列表 | GET | `/open-apis/hire/v1/talents` | `../src/service/hire/candidate_management/talent_pool/mod.rs` | 377 | ✅ 已实现 |
| 1232 | 获取人才字段 | GET | `/open-apis/hire/v1/talent_objects/query` | `../src/service/hire/recruitment_config/location/mod.rs` | 62 | ✅ 已实现 |
| 1233 | 获取人才信息 | GET | `/open-apis/hire/v1/talents/:talent_id` | `未找到` | - | ❌ 未实现 |
| 1234 | 获取人才详情 | GET | `/open-apis/hire/v2/talents/:talent_id` | `未找到` | - | ❌ 未实现 |
| 1235 | 更新人才在职状态 | POST | `/open-apis/hire/v1/talents/:talent_id/onboard_status` | `未找到` | - | ❌ 未实现 |
| 1236 | 加入/移除屏蔽名单 | POST | `/open-apis/hire/v1/talent_blocklist/change_talent_block` | `未找到` | - | ❌ 未实现 |
| 1237 | 获取投递详情 | GET | `/open-apis/hire/v1/applications/:application_id/get_detail` | `../src/service/contact/v3/group.rs` | 266 | ✅ 已实现 |
| 1238 | 恢复投递 | POST | `/open-apis/hire/v1/applications/:application_id/recover` | `未找到` | - | ❌ 未实现 |
| 1239 | 创建投递 | POST | `/open-apis/hire/v1/applications` | `../src/service/hire/candidate_management/application/mod.rs` | 261 | ✅ 已实现 |
| 1240 | 终止投递 | POST | `/open-apis/hire/v1/applications/:application_id/terminate` | `../src/service/feishu_people/core/v1/contracts.rs` | 440 | ✅ 已实现 |
| 1241 | 转移投递阶段 | POST | `/open-apis/hire/v1/applications/:application_id/transfer_stage` | `未找到` | - | ❌ 未实现 |
| 1242 | 获取终止投递原因 | GET | `/open-apis/hire/v1/termination_reasons` | `未找到` | - | ❌ 未实现 |
| 1243 | 获取投递信息 | GET | `/open-apis/hire/v1/applications/:application_id` | `未找到` | - | ❌ 未实现 |
| 1244 | 获取投递列表 | GET | `/open-apis/hire/v1/applications` | `../src/service/hire/candidate_management/application/mod.rs` | 261 | ✅ 已实现 |
| 1245 | 获取申请表附加信息 | POST | `/open-apis/hire/v1/applications/diversity_inclusions/search` | `../src/service/attendance/v1/group.rs` | 154 | ✅ 已实现 |
| 1246 | 获取简历评估信息列表 | GET | `/open-apis/hire/v1/evaluations` | `../src/service/hire/candidate_management/interview/mod.rs` | 520 | ✅ 已实现 |
| 1247 | 添加笔试结果 | POST | `/open-apis/hire/v1/exams` | `未找到` | - | ❌ 未实现 |
| 1248 | 获取笔试列表 | POST | `/open-apis/hire/v1/tests/search` | `../src/service/attendance/v1/group.rs` | 154 | ✅ 已实现 |
| 1249 | 获取面试信息 | GET | `/open-apis/hire/v1/interviews` | `../src/service/hire/candidate_management/interview/mod.rs` | 365 | ✅ 已实现 |
| 1250 | 获取人才面试信息 | GET | `/open-apis/hire/v1/interviews/get_by_talent` | `未找到` | - | ❌ 未实现 |
| 1251 | 获取面试评价详细信息 | GET | `/open-apis/hire/v1/interview_records/:interview_record_id` | `未找到` | - | ❌ 未实现 |
| 1252 | 获取面试评价详细信息（新版） | GET | `/open-apis/hire/v2/interview_records/:interview_record_id` | `未找到` | - | ❌ 未实现 |
| 1253 | 批量获取面试评价详细信息 | GET | `/open-apis/hire/v1/interview_records` | `未找到` | - | ❌ 未实现 |
| 1254 | 批量获取面试评价详细信息（新版） | GET | `/open-apis/hire/v2/interview_records` | `未找到` | - | ❌ 未实现 |
| 1255 | 获取面试记录附件 | GET | `/open-apis/hire/v1/interview_records/attachments` | `../src/service/hire/attachment/mod.rs` | 342 | ✅ 已实现 |
| 1256 | 获取面试速记明细 | GET | `/open-apis/hire/v1/minutes` | `未找到` | - | ❌ 未实现 |
| 1257 | 获取面试满意度问卷列表 | GET | `/open-apis/hire/v1/questionnaires` | `未找到` | - | ❌ 未实现 |
| 1258 | 创建 Offer | POST | `/open-apis/hire/v1/offers` | `../src/service/hire/candidate_management/offer/mod.rs` | 417 | ✅ 已实现 |
| 1259 | 更新 Offer 信息 | PUT | `/open-apis/hire/v1/offers/:offer_id` | `未找到` | - | ❌ 未实现 |
| 1260 | 获取 Offer 信息 | GET | `/open-apis/hire/v1/applications/:application_id/offer` | `../src/service/hire/candidate_management/offer/mod.rs` | 315 | ✅ 已实现 |
| 1261 | 获取 Offer 详情 | GET | `/open-apis/hire/v1/offers/:offer_id` | `未找到` | - | ❌ 未实现 |
| 1262 | 获取 Offer 列表 | GET | `/open-apis/hire/v1/offers` | `../src/service/hire/candidate_management/offer/mod.rs` | 417 | ✅ 已实现 |
| 1263 | 更新 Offer 状态 | PATCH | `/open-apis/hire/v1/offers/:offer_id/offer_status` | `未找到` | - | ❌ 未实现 |
| 1264 | 更新实习 Offer 入/离职状态 | POST | `/open-apis/hire/v1/offers/:offer_id/intern_offer_status` | `未找到` | - | ❌ 未实现 |
| 1265 | 获取背调信息列表 | GET | `/open-apis/hire/v1/background_check_orders` | `未找到` | - | ❌ 未实现 |
| 1266 | 查询背调信息列表 | POST | `/open-apis/hire/v1/background_check_orders/batch_query` | `../src/service/apass/object/mod.rs` | 333 | ✅ 已实现 |
| 1267 | 创建三方协议 | POST | `/open-apis/hire/v1/tripartite_agreements` | `未找到` | - | ❌ 未实现 |
| 1268 | 获取三方协议 | GET | `/open-apis/hire/v1/tripartite_agreements` | `未找到` | - | ❌ 未实现 |
| 1269 | 更新三方协议 | PUT | `/open-apis/hire/v1/tripartite_agreements/:tripartite_agreement_id` | `未找到` | - | ❌ 未实现 |
| 1270 | 删除三方协议 | DELETE | `/open-apis/hire/v1/tripartite_agreements/:tripartite_agreement_id` | `未找到` | - | ❌ 未实现 |
| 1271 | 更新 e-HR 导入任务结果 | PATCH | `/open-apis/hire/v1/ehr_import_tasks/:ehr_import_task_id` | `../src/service/hire/attachment/mod.rs` | 236 | ✅ 已实现 |
| 1272 | 操作候选人入职 | POST | `/open-apis/hire/v1/applications/:application_id/transfer_onboard` | `未找到` | - | ❌ 未实现 |
| 1273 | 取消候选人入职 | POST | `/open-apis/hire/v1/applications/:application_id/cancel_onboard` | `未找到` | - | ❌ 未实现 |
| 1274 | 更新员工状态 | PATCH | `/open-apis/hire/v1/employees/:employee_id` | `../src/service/ehr/v1/mod.rs` | 165 | ✅ 已实现 |
| 1275 | 通过投递 ID 获取入职信息 | GET | `/open-apis/hire/v1/employees/get_by_application` | `未找到` | - | ❌ 未实现 |
| 1276 | 通过员工 ID 获取入职信息 | GET | `/open-apis/hire/v1/employees/:employee_id` | `../src/service/ehr/v1/mod.rs` | 165 | ✅ 已实现 |
| 1277 | 批量获取待办事项 | GET | `/open-apis/hire/v1/todos` | `未找到` | - | ❌ 未实现 |
| 1278 | 获取简历评估任务列表 | GET | `/open-apis/hire/v1/evaluation_tasks` | `../src/service/hire/attachment/mod.rs` | 236 | ✅ 已实现 |
| 1279 | 获取笔试阅卷任务列表 | GET | `/open-apis/hire/v1/exam_marking_tasks` | `../src/service/hire/attachment/mod.rs` | 236 | ✅ 已实现 |
| 1280 | 获取面试任务列表 | GET | `/open-apis/hire/v1/interview_tasks` | `../src/service/hire/attachment/mod.rs` | 236 | ✅ 已实现 |
| 1281 | 创建备注 | POST | `/open-apis/hire/v1/notes` | `未找到` | - | ❌ 未实现 |
| 1282 | 更新备注 | PATCH | `/open-apis/hire/v1/notes/:note_id` | `未找到` | - | ❌ 未实现 |
| 1283 | 获取备注 | GET | `/open-apis/hire/v1/notes/:note_id` | `未找到` | - | ❌ 未实现 |
| 1284 | 获取备注列表 | GET | `/open-apis/hire/v1/notes` | `未找到` | - | ❌ 未实现 |
| 1285 | 删除备注 | DELETE | `/open-apis/hire/v1/notes/:note_id` | `未找到` | - | ❌ 未实现 |
| 1286 | 获取简历来源列表 | GET | `/open-apis/hire/v1/resume_sources` | `未找到` | - | ❌ 未实现 |
| 1287 | 创建账号自定义字段 | POST | `/open-apis/hire/v1/eco_account_custom_fields` | `未找到` | - | ❌ 未实现 |
| 1288 | 更新账号自定义字段 | PATCH | `/open-apis/hire/v1/eco_account_custom_fields/batch_update` | `../src/service/apass/object/mod.rs` | 304 | ✅ 已实现 |
| 1289 | 删除账号自定义字段 | POST | `/open-apis/hire/v1/eco_account_custom_fields/batch_delete` | `../src/service/hire/attachment/mod.rs` | 594 | ✅ 已实现 |
| 1290 | 创建背调自定义字段 | POST | `/open-apis/hire/v1/eco_background_check_custom_fields` | `未找到` | - | ❌ 未实现 |
| 1291 | 更新背调自定义字段 | PATCH | `/open-apis/hire/v1/eco_background_check_custom_fields/batch_update` | `../src/service/apass/object/mod.rs` | 304 | ✅ 已实现 |
| 1292 | 删除背调自定义字段 | POST | `/open-apis/hire/v1/eco_background_check_custom_fields/batch_delete` | `../src/service/hire/attachment/mod.rs` | 594 | ✅ 已实现 |
| 1293 | 创建背调套餐和附加调查项 | POST | `/open-apis/hire/v1/eco_background_check_packages` | `未找到` | - | ❌ 未实现 |
| 1294 | 更新背调套餐和附加调查项 | PATCH | `/open-apis/hire/v1/eco_background_check_packages/batch_update` | `../src/service/apass/object/mod.rs` | 304 | ✅ 已实现 |
| 1295 | 删除背调套餐和附加调查项 | POST | `/open-apis/hire/v1/eco_background_check_packages/batch_delete` | `../src/service/hire/attachment/mod.rs` | 594 | ✅ 已实现 |
| 1296 | 更新背调订单进度 | POST | `/open-apis/hire/v1/eco_background_checks/update_progress` | `../src/service/okr/progress_record/mod.rs` | 177 | ✅ 已实现 |
| 1297 | 回传背调订单的最终结果 | POST | `/open-apis/hire/v1/eco_background_checks/update_result` | `未找到` | - | ❌ 未实现 |
| 1298 | 终止背调订单 | POST | `/open-apis/hire/v1/eco_background_checks/cancel` | `../src/service/hire/ecological_docking/background_check/mod.rs` | 496 | ✅ 已实现 |
| 1299 | 创建试卷列表 | POST | `/open-apis/hire/v1/eco_exam_papers` | `未找到` | - | ❌ 未实现 |
| 1300 | 更新试卷列表 | PATCH | `/open-apis/hire/v1/eco_exam_papers/batch_update` | `../src/service/apass/object/mod.rs` | 304 | ✅ 已实现 |
| 1301 | 删除试卷列表 | POST | `/open-apis/hire/v1/eco_exam_papers/batch_delete` | `../src/service/hire/attachment/mod.rs` | 594 | ✅ 已实现 |
| 1302 | 回传笔试安排结果 | POST | `/open-apis/hire/v1/eco_exams/:exam_id/login_info` | `未找到` | - | ❌ 未实现 |
| 1303 | 回传笔试结果 | POST | `/open-apis/hire/v1/eco_exams/:exam_id/update_result` | `未找到` | - | ❌ 未实现 |
| 1304 | 启用内推账户 | POST | `/open-apis/hire/v1/referral_account/enable` | `../src/service/hire/recruitment_config/subject/mod.rs` | 401 | ✅ 已实现 |
| 1305 | 查询内推账户 | GET | `/open-apis/hire/v1/referral_account/get_account_assets` | `未找到` | - | ❌ 未实现 |
| 1306 | 注册内推账户 | POST | `/open-apis/hire/v1/referral_account` | `../src/service/hire/get_candidates/referral/mod.rs` | 393 | ✅ 已实现 |
| 1307 | 停用内推账户 | POST | `/open-apis/hire/v1/referral_account/:referral_account_id/deactivate` | `../src/service/payroll/paygroup/mod.rs` | 447 | ✅ 已实现 |
| 1308 | 全额提取内推账户余额 | POST | `/open-apis/hire/v1/referral_account/:referral_account_id/withdraw` | `../src/service/hire/candidate_management/offer/mod.rs` | 569 | ✅ 已实现 |
| 1309 | 内推账户提现数据对账 | POST | `/open-apis/hire/v1/referral_account/reconciliation` | `未找到` | - | ❌ 未实现 |
| 1310 | 创建附件 | POST | `/open-apis/hire/v1/attachments` | `../src/service/hire/attachment/mod.rs` | 342 | ✅ 已实现 |
| 1311 | 获取附件信息 | GET | `/open-apis/hire/v1/attachments/:attachment_id` | `未找到` | - | ❌ 未实现 |
| 1312 | 获取附件 PDF 格式下载链接 | GET | `/open-apis/hire/v1/attachments/:attachment_id/preview` | `../src/service/hire/attachment/mod.rs` | 506 | ✅ 已实现 |
| 1313 | 创建 OKR 周期 | POST | `/open-apis/okr/v1/periods` | `../src/service/okr/v1/mod.rs` | 93 | ✅ 已实现 |
| 1314 | 修改 OKR 周期状态 | PATCH | `/open-apis/okr/v1/periods/:period_id` | `../src/service/okr/v1/mod.rs` | 149 | ✅ 已实现 |
| 1315 | 获取 OKR 周期列表 | GET | `/open-apis/okr/v1/periods` | `../src/service/okr/v1/mod.rs` | 93 | ✅ 已实现 |
| 1316 | 获取 OKR 周期规则 | GET | `/open-apis/okr/v1/period_rules` | `../src/service/okr/period_rule/mod.rs` | 79 | ✅ 已实现 |
| 1317 | 获取用户的 OKR 列表 | GET | `/open-apis/okr/v1/users/:user_id/okrs` | `../src/service/okr/v1/mod.rs` | 296 | ✅ 已实现 |
| 1318 | 批量获取 OKR | GET | `/open-apis/okr/v1/okrs/batch_get` | `../src/service/okr/v1/mod.rs` | 342 | ✅ 已实现 |
| 1319 | 创建 OKR 进展记录 | POST | `/open-apis/okr/v1/progress_records` | `../src/service/okr/v1/mod.rs` | 409 | ✅ 已实现 |
| 1320 | 删除 OKR 进展记录 | DELETE | `/open-apis/okr/v1/progress_records/:progress_id` | `../src/service/okr/v1/mod.rs` | 149 | ✅ 已实现 |
| 1321 | 更新 OKR 进展记录 | PUT | `/open-apis/okr/v1/progress_records/:progress_id` | `../src/service/okr/v1/mod.rs` | 149 | ✅ 已实现 |
| 1322 | 获取 OKR 进展记录 | GET | `/open-apis/okr/v1/progress_records/:progress_id` | `../src/service/okr/v1/mod.rs` | 149 | ✅ 已实现 |
| 1323 | 上传进展记录图片 | POST | `/open-apis/okr/v1/images/upload` | `../src/service/okr/progress_record/mod.rs` | 236 | ✅ 已实现 |
| 1324 | 查询复盘信息 | GET | `/open-apis/okr/v1/reviews/query` | `../src/service/okr/review/mod.rs` | 82 | ✅ 已实现 |
| 1325 | 录入身份信息 | POST | `/open-apis/human_authentication/v1/identities` | `未找到` | - | ❌ 未实现 |
| 1326 | 上传人脸基准图片 | POST | `/open-apis/face_verify/v1/upload_face_image` | `未找到` | - | ❌ 未实现 |
| 1327 | 裁剪人脸图片 | POST | `/open-apis/face_verify/v1/crop_face_image` | `未找到` | - | ❌ 未实现 |
| 1328 | 查询人脸认证结果 | GET | `/open-apis/face_verify/v1/query_auth_result` | `未找到` | - | ❌ 未实现 |
| 1329 | 修改用户部分信息 | PATCH | `/open-apis/acs/v1/users/:user_id` | `未找到` | - | ❌ 未实现 |
| 1330 | 获取单个用户信息 | GET | `/open-apis/acs/v1/users/:user_id` | `未找到` | - | ❌ 未实现 |
| 1331 | 获取用户列表 | GET | `/open-apis/acs/v1/users` | `../src/service/calendar/v4/mod.rs` | 302 | ✅ 已实现 |
| 1332 | 上传人脸图片 | PUT | `/open-apis/acs/v1/users/:user_id/face` | `未找到` | - | ❌ 未实现 |
| 1333 | 下载人脸图片 | GET | `/open-apis/acs/v1/users/:user_id/face` | `未找到` | - | ❌ 未实现 |
| 1334 | 设备绑定权限组 | POST | `/open-apis/acs/v1/rule_external/device_bind` | `未找到` | - | ❌ 未实现 |
| 1335 | 获取权限组信息 | GET | `/open-apis/acs/v1/rule_external` | `未找到` | - | ❌ 未实现 |
| 1336 | 删除权限组 | DELETE | `/open-apis/acs/v1/rule_external` | `未找到` | - | ❌ 未实现 |
| 1337 | 创建或更新权限组 | POST | `/open-apis/acs/v1/rule_external` | `未找到` | - | ❌ 未实现 |
| 1338 | 删除访客 | DELETE | `/open-apis/acs/v1/visitors/:visitor_id` | `未找到` | - | ❌ 未实现 |
| 1339 | 添加访客 | POST | `/open-apis/acs/v1/visitors` | `未找到` | - | ❌ 未实现 |
| 1340 | 获取门禁设备列表 | GET | `/open-apis/acs/v1/devices` | `未找到` | - | ❌ 未实现 |
| 1341 | 获取门禁记录列表 | GET | `/open-apis/acs/v1/access_records` | `未找到` | - | ❌ 未实现 |
| 1342 | 下载开门时的人脸识别图片 | GET | `/open-apis/acs/v1/access_records/:access_record_id/access_photo` | `未找到` | - | ❌ 未实现 |
| 1343 | 获取周期列表 | GET | `/open-apis/performance/v1/semesters` | `../src/service/performance/review_config/mod.rs` | 43 | ✅ 已实现 |
| 1344 | 获取项目列表 | POST | `/open-apis/performance/v2/activity/query` | `../src/service/performance/metric_detail/mod.rs` | 40 | ✅ 已实现 |
| 1345 | 批量查询补充信息 | POST | `/open-apis/performance/v2/additional_informations/query` | `../src/service/performance/metric_detail/mod.rs` | 40 | ✅ 已实现 |
| 1346 | 批量导入补充信息 | POST | `/open-apis/performance/v2/additional_informations/import` | `../src/service/performance/metric_detail/mod.rs` | 65 | ✅ 已实现 |
| 1347 | 批量删除补充信息 | DELETE | `/open-apis/performance/v2/additional_informations/batch` | `../src/service/attendance/v1/user_task.rs` | 58 | ✅ 已实现 |
| 1348 | 更新人员组成员 | POST | `/open-apis/performance/v2/user_group_user_rels/write` | `../src/service/performance/review_config/mod.rs` | 184 | ✅ 已实现 |
| 1349 | 获取被评估人信息 | POST | `/open-apis/performance/v2/reviewees/query` | `../src/service/performance/metric_detail/mod.rs` | 40 | ✅ 已实现 |
| 1350 | 获取绩效模板配置 | POST | `/open-apis/performance/v2/review_templates/query` | `../src/service/performance/metric_detail/mod.rs` | 40 | ✅ 已实现 |
| 1351 | 获取评估项列表 | POST | `/open-apis/performance/v2/indicators/query` | `../src/service/performance/metric_detail/mod.rs` | 40 | ✅ 已实现 |
| 1352 | 获取标签填写题配置 | POST | `/open-apis/performance/v2/questions/query` | `../src/service/performance/metric_detail/mod.rs` | 40 | ✅ 已实现 |
| 1353 | 获取指标列表 | POST | `/open-apis/performance/v2/metric_libs/query` | `../src/service/performance/metric_detail/mod.rs` | 40 | ✅ 已实现 |
| 1354 | 获取指标模板列表 | POST | `/open-apis/performance/v2/metric_templates/query` | `../src/service/performance/metric_detail/mod.rs` | 40 | ✅ 已实现 |
| 1355 | 获取指标字段列表 | POST | `/open-apis/performance/v2/metric_fields/query` | `../src/service/performance/metric_detail/mod.rs` | 40 | ✅ 已实现 |
| 1356 | 获取指标标签列表 | GET | `/open-apis/performance/v2/metric_tags` | `../src/service/performance/review_config/mod.rs` | 384 | ✅ 已实现 |
| 1357 | 获取周期任务（指定用户） | POST | `/open-apis/performance/v1/stage_tasks/find_by_user_list` | `../src/service/performance/stage_task/mod.rs` | 36 | ✅ 已实现 |
| 1358 | 获取周期任务（全部用户） | POST | `/open-apis/performance/v1/stage_tasks/find_by_page` | `../src/service/performance/stage_task/mod.rs` | 36 | ✅ 已实现 |
| 1359 | 获取被评估人关键指标结果 | POST | `/open-apis/performance/v2/metric_details/query` | `../src/service/performance/metric_detail/mod.rs` | 40 | ✅ 已实现 |
| 1360 | 录入被评估人关键指标数据 | POST | `/open-apis/performance/v2/metric_details/import` | `../src/service/performance/metric_detail/mod.rs` | 65 | ✅ 已实现 |
| 1361 | 获取绩效结果 | POST | `/open-apis/performance/v1/review_datas/query` | `../src/service/performance/metric_detail/mod.rs` | 40 | ✅ 已实现 |
| 1362 | 获取绩效详情数据 | POST | `/open-apis/performance/v2/review_datas/query` | `../src/service/performance/metric_detail/mod.rs` | 40 | ✅ 已实现 |
| 1363 | 创建草稿 | POST | `/open-apis/lingo/v1/drafts` | `未找到` | - | ❌ 未实现 |
| 1364 | 更新草稿 | PUT | `/open-apis/lingo/v1/drafts/:draft_id` | `未找到` | - | ❌ 未实现 |
| 1365 | 创建免审词条 | POST | `/open-apis/lingo/v1/entities` | `../src/service/lingo/entity/mod.rs` | 147 | ✅ 已实现 |
| 1366 | 更新免审词条 | PUT | `/open-apis/lingo/v1/entities/:entity_id` | `未找到` | - | ❌ 未实现 |
| 1367 | 删除免审词条 | DELETE | `/open-apis/lingo/v1/entities/:entity_id` | `未找到` | - | ❌ 未实现 |
| 1368 | 获取词条详情 | GET | `/open-apis/lingo/v1/entities/:entity_id` | `未找到` | - | ❌ 未实现 |
| 1369 | 获取词条列表 | GET | `/open-apis/lingo/v1/entities` | `../src/service/lingo/entity/mod.rs` | 147 | ✅ 已实现 |
| 1370 | 精准搜索词条 | POST | `/open-apis/lingo/v1/entities/match` | `../src/service/lingo/entity/mod.rs` | 187 | ✅ 已实现 |
| 1371 | 模糊搜索词条 | POST | `/open-apis/lingo/v1/entities/search` | `../src/service/lingo/entity/mod.rs` | 212 | ✅ 已实现 |
| 1372 | 词条高亮 | POST | `/open-apis/lingo/v1/entities/highlight` | `../src/service/lingo/entity/mod.rs` | 237 | ✅ 已实现 |
| 1373 | 获取词典分类 | GET | `/open-apis/lingo/v1/classifications` | `未找到` | - | ❌ 未实现 |
| 1374 | 获取词库列表 | GET | `/open-apis/lingo/v1/repos` | `../src/service/lingo/repo/mod.rs` | 41 | ✅ 已实现 |
| 1375 | 上传图片 | POST | `/open-apis/lingo/v1/files/upload` | `../src/service/lingo/file/mod.rs` | 40 | ✅ 已实现 |
| 1376 | 下载图片 | GET | `/open-apis/lingo/v1/files/:file_token/download` | `../src/service/lingo/file/mod.rs` | 65 | ✅ 已实现 |
| 1377 | 获取客户端设备认证信息 | GET | `/open-apis/security_and_compliance/v2/device_records/mine` | `未找到` | - | ❌ 未实现 |
| 1378 | 新增设备 | POST | `/open-apis/security_and_compliance/v2/device_records` | `未找到` | - | ❌ 未实现 |
| 1379 | 查询设备信息 | GET | `/open-apis/security_and_compliance/v2/device_records` | `未找到` | - | ❌ 未实现 |
| 1380 | 获取设备信息 | GET | `/open-apis/security_and_compliance/v2/device_records/:device_record_id` | `未找到` | - | ❌ 未实现 |
| 1381 | 更新设备 | PUT | `/open-apis/security_and_compliance/v2/device_records/:device_record_id` | `未找到` | - | ❌ 未实现 |
| 1382 | 删除设备 | DELETE | `/open-apis/security_and_compliance/v2/device_records/:device_record_id` | `未找到` | - | ❌ 未实现 |
| 1383 | 审批设备申报 | PUT | `/open-apis/security_and_compliance/v2/device_apply_records/:device_apply_record_id` | `未找到` | - | ❌ 未实现 |
| 1384 | 获取OpenAPI审计日志数据 | POST | `/open-apis/security_and_compliance/v1/openapi_logs/list_data` | `../src/service/security_and_compliance/openapi_log/mod.rs` | 31 | ✅ 已实现 |
| 1385 | 获取行为审计日志数据 | GET | `/open-apis/admin/v1/audit_infos` | `未找到` | - | ❌ 未实现 |
| 1386 | 获取可见关联组织的列表 | GET | `/open-apis/trust_party/v1/collaboration_tenants` | `未找到` | - | ❌ 未实现 |
| 1387 | 获取关联组织的部门和成员信息 | GET | `/open-apis/trust_party/v1/collaboration_tenants/visible_organization` | `未找到` | - | ❌ 未实现 |
| 1388 | 获取关联组织详情 | GET | `/open-apis/trust_party/v1/collaboration_tenants/:target_tenant_key` | `未找到` | - | ❌ 未实现 |
| 1389 | 获取关联组织成员详情 | GET | `/open-apis/trust_party/v1/collaboration_tenants/:target_tenant_key/collaboration_users/:target_user_id` | `../src/service/trust_party/collaboration_organization/mod.rs` | 131 | ✅ 已实现 |
| 1390 | 获取关联组织部门详情 | GET | `/open-apis/trust_party/v1/collaboration_tenants/:target_tenant_key/collaboration_departments/:target_department_id` | `../src/service/trust_party/collaboration_organization/mod.rs` | 157 | ✅ 已实现 |
| 1391 | 获取关联组织双方共享成员范围 | GET | `/open-apis/directory/v1/share_entities` | `未找到` | - | ❌ 未实现 |
| 1392 | 管理员获取所有关联组织列表 | GET | `/open-apis/directory/v1/collaboration_tenants` | `未找到` | - | ❌ 未实现 |
| 1393 | 新增可搜可见规则 | POST | `/open-apis/directory/v1/collaboration_rules` | `未找到` | - | ❌ 未实现 |
| 1394 | 更新可搜可见规则 | PUT | `/open-apis/directory/v1/collaboration_rules/:collaboration_rule_id` | `未找到` | - | ❌ 未实现 |
| 1395 | 查询可搜可见规则 | GET | `/open-apis/directory/v1/collaboration_rules` | `未找到` | - | ❌ 未实现 |
| 1396 | 删除可搜可见规则 | DELETE | `/open-apis/directory/v1/collaboration_rules/:collaboration_rule_id` | `未找到` | - | ❌ 未实现 |
| 1397 | 获取工作台访问数据 | POST | `/open-apis/workplace/v1/workplace_access_data/search` | `../src/service/workplace/workplace_access_data/mod.rs` | 50 | ✅ 已实现 |
| 1398 | 获取定制工作台访问数据 | POST | `/open-apis/workplace/v1/custom_workplace_access_data/search` | `../src/service/workplace/workplace_access_data/mod.rs` | 50 | ✅ 已实现 |
| 1399 | 获取定制工作台小组件访问数据 | POST | `/open-apis/workplace/v1/workplace_block_access_data/search` | `../src/service/workplace/workplace_access_data/mod.rs` | 50 | ✅ 已实现 |
| 1400 | 获取用户自定义常用的应用 | GET | `/open-apis/application/v5/applications/favourite` | `../src/service/workplace/app_recommend/mod.rs` | 47 | ✅ 已实现 |
| 1401 | 获取管理员推荐的应用 | GET | `/open-apis/application/v5/applications/recommend` | `../src/service/workplace/app_recommend/mod.rs` | 84 | ✅ 已实现 |
| 1402 | 获取当前设置的推荐规则列表 | GET | `/open-apis/application/v6/app_recommend_rules` | `未找到` | - | ❌ 未实现 |
| 1403 | 根据主数据编码批量查询国家/地区 | GET | `/open-apis/mdm/v3/batch_country_region` | `未找到` | - | ❌ 未实现 |
| 1404 | 分页批量查询国家/地区 | GET | `/open-apis/mdm/v3/country_regions` | `未找到` | - | ❌ 未实现 |
| 1405 | 用户数据维度绑定 | POST | `/open-apis/mdm/v1/user_auth_data_relations/bind` | `../src/service/mdm/user_auth_data_relation/mod.rs` | 42 | ✅ 已实现 |
| 1406 | 用户数据维度解绑 | POST | `/open-apis/mdm/v1/user_auth_data_relations/unbind` | `../src/service/mdm/user_auth_data_relation/mod.rs` | 67 | ✅ 已实现 |
| 1407 | 查询规则 | GET | `/open-apis/report/v1/rules/query` | `../src/service/report/rule/mod.rs` | 40 | ✅ 已实现 |
| 1408 | 移除规则看板 | POST | `/open-apis/report/v1/rules/:rule_id/views/remove` | `../src/service/report/rule_view/mod.rs` | 38 | ✅ 已实现 |
| 1409 | 查询任务 | POST | `/open-apis/report/v1/tasks/query` | `../src/service/report/rule/mod.rs` | 40 | ✅ 已实现 |
| 1410 | 创建任务 | POST | `/open-apis/task/v1/tasks` | `../src/service/task/v1/mod.rs` | 54 | ✅ 已实现 |
| 1411 | 删除任务 | DELETE | `/open-apis/task/v1/tasks/:task_id` | `../src/service/task/v1/mod.rs` | 33 | ✅ 已实现 |
| 1412 | 更新任务 | PATCH | `/open-apis/task/v1/tasks/:task_id` | `../src/service/task/v1/mod.rs` | 33 | ✅ 已实现 |
| 1413 | 完成任务 | POST | `/open-apis/task/v1/tasks/:task_id/complete` | `../src/service/task/v1/mod.rs` | 33 | ✅ 已实现 |
| 1414 | 取消完成任务 | POST | `/open-apis/task/v1/tasks/:task_id/uncomplete` | `../src/service/task/v1/mod.rs` | 33 | ✅ 已实现 |
| 1415 | 查询指定任务 | GET | `/open-apis/task/v1/tasks/:task_id` | `../src/service/task/v1/mod.rs` | 33 | ✅ 已实现 |
| 1416 | 查询所有任务 | GET | `/open-apis/task/v1/tasks` | `../src/service/task/v1/mod.rs` | 54 | ✅ 已实现 |
| 1417 | 新增提醒时间 | POST | `/open-apis/task/v1/tasks/:task_id/reminders` | `../src/service/task/v2/task/mod.rs` | 412 | ✅ 已实现 |
| 1418 | 删除提醒时间 | DELETE | `/open-apis/task/v1/tasks/:task_id/reminders/:reminder_id` | `../src/service/task/v1/mod.rs` | 33 | ✅ 已实现 |
| 1419 | 查询提醒时间列表 | GET | `/open-apis/task/v1/tasks/:task_id/reminders` | `../src/service/task/v2/task/mod.rs` | 412 | ✅ 已实现 |
| 1420 | 创建评论 | POST | `/open-apis/task/v1/tasks/:task_id/comments` | `../src/service/task/v2/mod.rs` | 338 | ✅ 已实现 |
| 1421 | 删除评论 | DELETE | `/open-apis/task/v1/tasks/:task_id/comments/:comment_id` | `../src/service/task/v1/mod.rs` | 33 | ✅ 已实现 |
| 1422 | 更新评论 | PUT | `/open-apis/task/v1/tasks/:task_id/comments/:comment_id` | `../src/service/task/v1/mod.rs` | 33 | ✅ 已实现 |
| 1423 | 获取评论详情 | GET | `/open-apis/task/v1/tasks/:task_id/comments/:comment_id` | `../src/service/task/v1/mod.rs` | 33 | ✅ 已实现 |
| 1424 | 获取评论列表 | GET | `/open-apis/task/v1/tasks/:task_id/comments` | `../src/service/task/v2/mod.rs` | 338 | ✅ 已实现 |
| 1425 | 新增关注人 | POST | `/open-apis/task/v1/tasks/:task_id/followers` | `../src/service/task/v1/mod.rs` | 33 | ✅ 已实现 |
| 1426 | 删除指定关注人 | DELETE | `/open-apis/task/v1/tasks/:task_id/followers/:follower_id` | `../src/service/task/v1/mod.rs` | 33 | ✅ 已实现 |
| 1427 | 批量删除关注人 | POST | `/open-apis/task/v1/tasks/:task_id/batch_delete_follower` | `../src/service/task/v1/mod.rs` | 33 | ✅ 已实现 |
| 1428 | 获取关注人列表 | GET | `/open-apis/task/v1/tasks/:task_id/followers` | `../src/service/task/v1/mod.rs` | 33 | ✅ 已实现 |
| 1429 | 新增执行者 | POST | `/open-apis/task/v1/tasks/:task_id/collaborators` | `../src/service/task/v1/mod.rs` | 33 | ✅ 已实现 |
| 1430 | 删除指定执行者 | DELETE | `/open-apis/task/v1/tasks/:task_id/collaborators/:collaborator_id` | `../src/service/task/v1/mod.rs` | 33 | ✅ 已实现 |
| 1431 | 批量删除执行者 | POST | `/open-apis/task/v1/tasks/:task_id/batch_delete_collaborator` | `../src/service/task/v1/mod.rs` | 33 | ✅ 已实现 |
| 1432 | 获取执行者列表 | GET | `/open-apis/task/v1/tasks/:task_id/collaborators` | `../src/service/task/v1/mod.rs` | 33 | ✅ 已实现 |
| 1433 | 获取 user_access_token | POST | `/open-apis/authen/v1/oidc/access_token` | `../src/service/auth/v1/mod.rs` | 41 | ✅ 已实现 |
| 1434 | 刷新 user_access_token | POST | `/open-apis/authen/v1/oidc/refresh_access_token` | `../src/service/auth/v1/mod.rs` | 59 | ✅ 已实现 |
| 1435 | 获取登录预授权码 | GET | `/open-apis/authen/v1/index` | `未找到` | - | ❌ 未实现 |
| 1436 | 获取 user_access_token（v1 版本） | POST | `/open-apis/authen/v1/access_token` | `../src/service/auth/v1/mod.rs` | 41 | ✅ 已实现 |
| 1437 | 刷新 user_access_token（v1 版本） | POST | `/open-apis/authen/v1/refresh_access_token` | `../src/service/auth/v1/mod.rs` | 59 | ✅ 已实现 |
| 1438 | 创建草稿 | POST | `/open-apis/baike/v1/drafts` | `未找到` | - | ❌ 未实现 |
| 1439 | 更新草稿 | PUT | `/open-apis/baike/v1/drafts/:draft_id` | `未找到` | - | ❌ 未实现 |
| 1440 | 创建免审词条 | POST | `/open-apis/baike/v1/entities` | `../src/service/lingo/entity/mod.rs` | 147 | ✅ 已实现 |
| 1441 | 更新免审词条 | PUT | `/open-apis/baike/v1/entities/:entity_id` | `未找到` | - | ❌ 未实现 |
| 1442 | 获取词条详情 | GET | `/open-apis/baike/v1/entities/:entity_id` | `未找到` | - | ❌ 未实现 |
| 1443 | 获取词条列表 | GET | `/open-apis/baike/v1/entities` | `../src/service/lingo/entity/mod.rs` | 147 | ✅ 已实现 |
| 1444 | 精准搜索词条 | POST | `/open-apis/baike/v1/entities/match` | `../src/service/lingo/entity/mod.rs` | 187 | ✅ 已实现 |
| 1445 | 模糊搜索词条 | POST | `/open-apis/baike/v1/entities/search` | `../src/service/attendance/v1/group.rs` | 154 | ✅ 已实现 |
| 1446 | 词条高亮 | POST | `/open-apis/baike/v1/entities/highlight` | `../src/service/lingo/entity/mod.rs` | 237 | ✅ 已实现 |
| 1447 | 提取潜在的词条 | POST | `/open-apis/baike/v1/entities/extract` | `../src/service/ai/document_ai/mod.rs` | 265 | ✅ 已实现 |
| 1448 | 获取词典分类 | GET | `/open-apis/baike/v1/classifications` | `未找到` | - | ❌ 未实现 |
| 1449 | 上传图片 | POST | `/open-apis/baike/v1/files/upload` | `../src/service/attendance/v1/user_setting.rs` | 77 | ✅ 已实现 |
| 1450 | 下载图片 | GET | `/open-apis/baike/v1/files/:file_token/download` | `../src/service/attendance/v1/user_setting.rs` | 117 | ✅ 已实现 |
| 1451 | 获取企业安装的应用 | GET | `/open-apis/application/v3/app/list` | `../src/service/application/v6/appstore_paid_info/mod.rs` | 58 | ✅ 已实现 |
| 1452 | 更新应用可用范围 | POST | `/open-apis/application/v3/app/update_visibility` | `未找到` | - | ❌ 未实现 |
| 1453 | 订阅审批事件 | POST | `/approval/openapi/v2/subscription/subscribe` | `../src/service/approval/v4/instance.rs` | 262 | ✅ 已实现 |
| 1454 | 取消订阅审批事件 | POST | `/approval/openapi/v2/subscription/unsubscribe` | `../src/service/approval/v4/instance.rs` | 262 | ✅ 已实现 |
| 1455 | 查看审批定义 | POST | `/approval/openapi/v2/approval/get` | `../src/service/approval/v4/instance.rs` | 58 | ✅ 已实现 |
| 1456 | 创建审批实例 | POST | `/approval/openapi/v2/instance/create` | `../src/service/approval/v4/instance.rs` | 36 | ✅ 已实现 |
| 1457 | 获取单个审批实例详情 | POST | `/approval/openapi/v2/instance/get` | `../src/service/approval/v4/instance.rs` | 58 | ✅ 已实现 |
| 1458 | 批量获取审批实例ID | POST | `/approval/openapi/v2/instance/list` | `../src/service/approval/v4/external_task/mod.rs` | 51 | ✅ 已实现 |
| 1459 | 审批实例抄送 | POST | `/approval/openapi/v2/instance/cc` | `../src/service/approval/v4/search/mod.rs` | 311 | ✅ 已实现 |
| 1460 | 审批实例撤回 | POST | `/approval/openapi/v2/instance/cancel` | `../src/service/approval/v4/instance.rs` | 262 | ✅ 已实现 |
| 1461 | 审批任务同意 | POST | `/approval/openapi/v2/instance/approve` | `../src/service/approval/v4/task.rs` | 59 | ✅ 已实现 |
| 1462 | 审批任务拒绝 | POST | `/approval/openapi/v2/instance/reject` | `../src/service/approval/v4/task.rs` | 76 | ✅ 已实现 |
| 1463 | 审批任务转交 | POST | `/approval/openapi/v2/instance/transfer` | `../src/service/approval/v4/task.rs` | 93 | ✅ 已实现 |
| 1464 | 三方审批定义创建 | POST | `/approval/openapi/v3/external/approval/create` | `../src/service/approval/v4/instance.rs` | 36 | ✅ 已实现 |
| 1465 | 三方审批实例同步 | POST | `/approval/openapi/v2/external/instance/create` | `../src/service/approval/v4/instance.rs` | 36 | ✅ 已实现 |
| 1466 | 三方审批实例校验 | POST | `/approval/openapi/v3/external/instance/check` | `../src/service/approval/v4/external_instance/mod.rs` | 70 | ✅ 已实现 |
| 1467 | 获取三方审批任务状态 | POST | `/approval/openapi/v2/external/list` | `../src/service/approval/v4/external_task/mod.rs` | 51 | ✅ 已实现 |
| 1468 | 创建审批定义 | POST | `/approval/openapi/v2/approval/create` | `../src/service/approval/v4/instance.rs` | 36 | ✅ 已实现 |
| 1469 | 实例列表查询 | POST | `/approval/openapi/v2/instance/search` | `../src/service/approval/v4/search/mod.rs` | 212 | ✅ 已实现 |
| 1470 | 抄送列表查询 | POST | `/approval/openapi/v2/cc/search` | `../src/service/approval/v4/search/mod.rs` | 212 | ✅ 已实现 |
| 1471 | 任务列表查询 | POST | `/approval/openapi/v2/task/search` | `../src/service/approval/v4/search/mod.rs` | 212 | ✅ 已实现 |
| 1472 | 获取用户列表 | GET | `/open-apis/contact/v3/users` | `../src/service/contact/v3/group.rs` | 206 | ✅ 已实现 |
| 1473 | 获取角色列表 | GET | `/open-apis/contact/v2/role/list` | `../src/service/contact/v3/group_member.rs` | 62 | ✅ 已实现 |
| 1474 | 更新用户所有信息 | PUT | `/open-apis/contact/v3/users/:user_id` | `../src/service/contact/v3/group.rs` | 206 | ✅ 已实现 |
| 1475 | 获取部门信息列表 | GET | `/open-apis/contact/v3/departments` | `../src/service/contact/v3/department.rs` | 156 | ✅ 已实现 |
| 1476 | 批量新增部门 | POST | `/open-apis/contact/v2/department/batch_add` | `../src/service/contact/v3/group_member.rs` | 44 | ✅ 已实现 |
| 1477 | 批量新增用户 | POST | `/open-apis/contact/v2/user/batch_add` | `../src/service/contact/v3/group_member.rs` | 44 | ✅ 已实现 |
| 1478 | 查询批量任务执行状态 | GET | `/open-apis/contact/v2/task/get` | `../src/service/contact/v3/job_family.rs` | 66 | ✅ 已实现 |
| 1479 | 新增自定义角色 | POST | `/open-apis/bitable/v1/apps/:app_token/roles` | `../src/service/cloud_docs/bitable/v1/app_role/list.rs` | 75 | ✅ 已实现 |
| 1480 | 列出自定义角色 | GET | `/open-apis/bitable/v1/apps/:app_token/roles` | `../src/service/cloud_docs/bitable/v1/app_role/list.rs` | 75 | ✅ 已实现 |
| 1481 | 更新自定义角色 | PUT | `/open-apis/bitable/v1/apps/:app_token/roles/:role_id` | `../src/service/contact/v3/functional_role.rs` | 62 | ✅ 已实现 |
| 1482 | 检索记录 | GET | `/open-apis/bitable/v1/apps/:app_token/tables/:table_id/records/:record_id` | `../src/service/base/bitable/mod.rs` | 135 | ✅ 已实现 |
| 1483 | 列出记录 | GET | `/open-apis/bitable/v1/apps/:app_token/tables/:table_id/records` | `../src/service/okr/v1/mod.rs` | 409 | ✅ 已实现 |
| 1484 | 创建旧版文档 | POST | `/open-apis/doc/v2/create` | `../src/service/attendance/v1/user_approval.rs` | 54 | ✅ 已实现 |
| 1485 | 获取旧版文档元信息 | GET | `/open-apis/doc/v2/meta/:docToken` | `未找到` | - | ❌ 未实现 |
| 1486 | 获取旧版文档中的电子表格元数据 | GET | `/open-apis/doc/v2/:docToken/sheet_meta` | `未找到` | - | ❌ 未实现 |
| 1487 | 获取旧版文档纯文本内容 | GET | `/open-apis/doc/v2/:docToken/raw_content` | `../src/service/cloud_docs/docx/v1/document.rs` | 83 | ✅ 已实现 |
| 1488 | 获取旧版文档富文本内容 | GET | `/open-apis/doc/v2/:docToken/content` | `../src/service/cloud_docs/docx/v1/document.rs` | 83 | ✅ 已实现 |
| 1489 | 编辑旧版文档内容 | POST | `/open-apis/doc/v2/:docToken/batch_update` | `../src/service/apass/object/mod.rs` | 304 | ✅ 已实现 |
| 1490 | 获取表格元数据 | GET | `/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/metainfo` | `未找到` | - | ❌ 未实现 |
| 1491 | 更新表格属性 | PUT | `/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/properties` | `../src/service/ccm/sheets/v2/spreadsheet.rs` | 76 | ✅ 已实现 |
| 1492 | 导入表格 | POST | `/open-apis/sheets/v2/import` | `../src/service/cloud_docs/drive/v1/file.rs` | 306 | ✅ 已实现 |
| 1493 | 查询导入结果 | GET | `/open-apis/sheets/v2/import/result` | `../src/service/attendance/v1/user_task.rs` | 86 | ✅ 已实现 |
| 1494 | 新建文件 | POST | `/open-apis/drive/explorer/v2/file/:folderToken` | `未找到` | - | ❌ 未实现 |
| 1495 | 获取元数据 | POST | `/open-apis/suite/docs-api/meta` | `../src/service/ccm/sheets/v2/spreadsheet.rs` | 37 | ✅ 已实现 |
| 1496 | 删除Sheet | DELETE | `/open-apis/drive/explorer/v2/file/spreadsheets/:spreadsheetToken` | `未找到` | - | ❌ 未实现 |
| 1497 | 复制文档 | POST | `/open-apis/drive/explorer/v2/file/copy/files/:fileToken` | `未找到` | - | ❌ 未实现 |
| 1498 | 删除Doc | DELETE | `/open-apis/drive/explorer/v2/file/docs/:docToken` | `未找到` | - | ❌ 未实现 |
| 1499 | 获取文件夹下的文档清单 | GET | `/open-apis/drive/explorer/v2/folder/:folderToken/children` | `../src/service/contact/v3/department.rs` | 230 | ✅ 已实现 |
| 1500 | 新建文件夹 | POST | `/open-apis/drive/explorer/v2/folder/:folderToken` | `未找到` | - | ❌ 未实现 |
| 1501 | 判断协作者是否有某权限 | POST | `/open-apis/drive/permission/member/permitted` | `未找到` | - | ❌ 未实现 |
| 1502 | 转移拥有者 | POST | `/open-apis/drive/permission/member/transfer` | `../src/service/apass/flow/mod.rs` | 211 | ✅ 已实现 |
| 1503 | 获取云文档权限设置V2 | POST | `/open-apis/drive/permission/v2/public/` | `../src/service/cloud_docs/permission/public_v2/patch.rs` | 103 | ✅ 已实现 |
| 1504 | 更新云文档权限设置 | PATCH | `/open-apis/drive/v1/permissions/:token/public` | `../src/service/cloud_docs/permission/public_v2/patch.rs` | 103 | ✅ 已实现 |
| 1505 | 获取云文档权限设置 | GET | `/open-apis/drive/v1/permissions/:token/public` | `../src/service/cloud_docs/permission/public_v2/patch.rs` | 103 | ✅ 已实现 |
| 1506 | 获取面试记录列表 | GET | `/open-apis/hire/v1/applications/:application_id/interviews` | `../src/service/hire/candidate_management/interview/mod.rs` | 365 | ✅ 已实现 |
| 1507 | 查询人才操作记录 | POST | `/open-apis/hire/v1/talent_operation_logs/search` | `../src/service/attendance/v1/group.rs` | 154 | ✅ 已实现 |
| 1508 | 获取职位上的招聘人员信息 | GET | `/open-apis/hire/v1/jobs/:job_id/managers/:manager_id` | `未找到` | - | ❌ 未实现 |
| 1509 | 获取 Offer 申请表详细信息 | GET | `/open-apis/hire/v1/offer_schemas/:offer_schema_id` | `未找到` | - | ❌ 未实现 |
| 1510 | 查询单个待入职信息 | GET | `/open-apis/corehr/v1/pre_hires/:pre_hire_id` | `未找到` | - | ❌ 未实现 |
| 1511 | 批量查询待入职信息 | GET | `/open-apis/corehr/v1/pre_hires` | `未找到` | - | ❌ 未实现 |
| 1512 | 更新待入职信息（不推荐） | PATCH | `/open-apis/corehr/v1/pre_hires/:pre_hire_id` | `未找到` | - | ❌ 未实现 |
| 1513 | 删除待入职（不推荐） | DELETE | `/open-apis/corehr/v1/pre_hires/:pre_hire_id` | `未找到` | - | ❌ 未实现 |
| 1514 | 获取流程表单数据 | GET | `/open-apis/corehr/v1/processes/:process_id/form_variable_data` | `未找到` | - | ❌ 未实现 |
| 1515 | 批量查询城市/区域信息 | GET | `/open-apis/corehr/v1/subregions` | `未找到` | - | ❌ 未实现 |
| 1516 | 查询单条城市/区域信息 | GET | `/open-apis/corehr/v1/subregions/:subregion_id` | `未找到` | - | ❌ 未实现 |
| 1517 | 批量查询省份/行政区信息 | GET | `/open-apis/corehr/v1/subdivisions` | `未找到` | - | ❌ 未实现 |
| 1518 | 查询单条省份/行政区信息 | GET | `/open-apis/corehr/v1/subdivisions/:subdivision_id` | `未找到` | - | ❌ 未实现 |
| 1519 | 批量查询国家/地区信息 | GET | `/open-apis/corehr/v1/country_regions` | `未找到` | - | ❌ 未实现 |
| 1520 | 查询单条国家/地区信息 | GET | `/open-apis/corehr/v1/country_regions/:country_region_id` | `未找到` | - | ❌ 未实现 |
| 1521 | 批量查询货币信息 | GET | `/open-apis/corehr/v1/currencies` | `未找到` | - | ❌ 未实现 |
| 1522 | 查询单个货币信息 | GET | `/open-apis/corehr/v1/currencies/:currency_id` | `未找到` | - | ❌ 未实现 |
| 1523 | 查询单个职务 | GET | `/open-apis/corehr/v1/jobs/:job_id` | `未找到` | - | ❌ 未实现 |
| 1524 | 删除部门 | DELETE | `/open-apis/corehr/v1/departments/:department_id` | `../src/service/corehr/organization/mod.rs` | 124 | ✅ 已实现 |
| 1525 | 更新部门 | PATCH | `/open-apis/corehr/v1/departments/:department_id` | `../src/service/corehr/organization/mod.rs` | 124 | ✅ 已实现 |
| 1526 | 查询单个部门 | GET | `/open-apis/corehr/v1/departments/:department_id` | `../src/service/corehr/organization/mod.rs` | 124 | ✅ 已实现 |
| 1527 | 批量查询职务 | GET | `/open-apis/corehr/v1/jobs` | `../src/service/corehr/job_management/mod.rs` | 476 | ✅ 已实现 |
| 1528 | 批量查询部门 | GET | `/open-apis/corehr/v1/departments` | `../src/service/corehr/organization/mod.rs` | 161 | ✅ 已实现 |
| 1529 | 更新个人信息 | PATCH | `/open-apis/corehr/v1/persons/:person_id` | `未找到` | - | ❌ 未实现 |
| 1530 | 创建个人信息 | POST | `/open-apis/corehr/v1/persons` | `未找到` | - | ❌ 未实现 |
| 1531 | 查询单个个人信息 | GET | `/open-apis/corehr/v1/persons/:person_id` | `未找到` | - | ❌ 未实现 |
| 1532 | 操作员工离职 | POST | `/open-apis/corehr/v1/offboardings/submit` | `../src/service/hire/ecological_docking/exam/mod.rs` | 512 | ✅ 已实现 |
| 1533 | 获取建筑物列表 | GET | `/open-apis/meeting_room/building/list` | `../src/service/attendance/v1/mod.rs` | 134 | ✅ 已实现 |
| 1534 | 查询建筑物详情 | GET | `/open-apis/meeting_room/building/batch_get` | `../src/service/contact/v3/department.rs` | 212 | ✅ 已实现 |
| 1535 | 获取会议室列表 | GET | `/open-apis/meeting_room/room/list` | `../src/service/attendance/v1/mod.rs` | 134 | ✅ 已实现 |
| 1536 | 查询会议室详情 | GET | `/open-apis/meeting_room/room/batch_get` | `../src/service/contact/v3/department.rs` | 212 | ✅ 已实现 |
| 1537 | 创建建筑物 | POST | `/open-apis/meeting_room/building/create` | `../src/service/attendance/v1/user_approval.rs` | 54 | ✅ 已实现 |
| 1538 | 更新建筑物 | POST | `/open-apis/meeting_room/building/update` | `../src/service/lingo/draft/mod.rs` | 44 | ✅ 已实现 |
| 1539 | 删除建筑物 | POST | `/open-apis/meeting_room/building/delete` | `../src/service/attendance/v1/group.rs` | 101 | ✅ 已实现 |
| 1540 | 查询建筑物ID | GET | `/open-apis/meeting_room/building/batch_get_id` | `未找到` | - | ❌ 未实现 |
| 1541 | 创建会议室 | POST | `/open-apis/meeting_room/room/create` | `../src/service/attendance/v1/user_approval.rs` | 54 | ✅ 已实现 |
| 1542 | 更新会议室 | POST | `/open-apis/meeting_room/room/update` | `../src/service/lingo/draft/mod.rs` | 44 | ✅ 已实现 |
| 1543 | 删除会议室 | POST | `/open-apis/meeting_room/room/delete` | `../src/service/attendance/v1/group.rs` | 101 | ✅ 已实现 |
| 1544 | 查询会议室ID | GET | `/open-apis/meeting_room/room/batch_get_id` | `未找到` | - | ❌ 未实现 |
| 1545 | 获取国家地区列表 | GET | `/open-apis/meeting_room/country/list` | `../src/service/attendance/v1/mod.rs` | 134 | ✅ 已实现 |
| 1546 | 获取城市列表 | GET | `/open-apis/meeting_room/district/list` | `../src/service/attendance/v1/mod.rs` | 134 | ✅ 已实现 |
| 1547 | 创建签到板部署码 | POST | `/open-apis/vc/v1/room_configs/set_checkboard_access_code` | `未找到` | - | ❌ 未实现 |
| 1548 | 创建会议室部署码 | POST | `/open-apis/vc/v1/room_configs/set_room_access_code` | `未找到` | - | ❌ 未实现 |
| 1549 | 查询会议室配置 | GET | `/open-apis/vc/v1/room_configs/query` | `../src/service/attendance/v1/user_task.rs` | 86 | ✅ 已实现 |
| 1550 | 设置会议室配置 | POST | `/open-apis/vc/v1/room_configs/set` | `../src/service/vc/v1/recording/mod.rs` | 136 | ✅ 已实现 |
| 1551 | 转换 ID | POST | `/open-apis/cardkit/v1/cards/id_convert` | `未找到` | - | ❌ 未实现 |


## 实现统计

### 按服务分类的实现情况


### 未实现的API (687个)

以下是前100个未实现的API:

- 获取事件出口 IP (GET /open-apis/event/v1/outbound_ip)
- 退出登录 (POST /open-apis/passport/v1/sessions/logout)
- 查询用户所属用户组 (GET /open-apis/contact/v3/group/member_belong)
- 获取企业自定义用户字段 (GET /open-apis/contact/v3/custom_attrs)
- 新增人员类型 (POST /open-apis/contact/v3/employee_type_enums)
- 更新人员类型 (PUT /open-apis/contact/v3/employee_type_enums/:enum_id)
- 查询人员类型 (GET /open-apis/contact/v3/employee_type_enums)
- 删除人员类型 (DELETE /open-apis/contact/v3/employee_type_enums/:enum_id)
- 获取租户职务列表 (GET /open-apis/contact/v3/job_titles)
- 获取租户工作城市列表 (GET /open-apis/contact/v3/work_cities)
- 恢复离职员工 (POST /open-apis/directory/v1/employees/:employee_id/resurrect)
- 更新在职员工为待离职 (PATCH /open-apis/directory/v1/employees/:employee_id/to_be_resigned)
- 更新待离职成员为在职 (PATCH /open-apis/directory/v1/employees/:employee_id/regular)
- 批量获取员工信息 (POST /open-apis/directory/v1/employees/mget)
- 批量获取员工列表 (POST /open-apis/directory/v1/employees/filter)
- 批量获取部门信息 (POST /open-apis/directory/v1/departments/mget)
- 获取部门列表 (POST /open-apis/directory/v1/departments/filter)
- 编辑消息 (PUT /open-apis/im/v1/messages/:message_id)
- 转发消息 (POST /open-apis/im/v1/messages/:message_id/forward)
- 合并转发消息 (POST /open-apis/im/v1/messages/merge_forward)
- 转发话题 (POST /open-apis/im/v1/threads/:thread_id/forward)
- 撤回消息 (DELETE /open-apis/im/v1/messages/:message_id)
- 添加跟随气泡 (POST /open-apis/im/v1/messages/:message_id/push_follow_up)
- 查询消息已读信息 (GET /open-apis/im/v1/messages/:message_id/read_users)
- 获取消息中的资源文件 (GET /open-apis/im/v1/messages/:message_id/resources/:file_key)
- 获取指定消息的内容 (GET /open-apis/im/v1/messages/:message_id)
- 批量发送消息 (POST /open-apis/message/v4/batch_send/)
- 批量撤回消息 (DELETE /open-apis/im/v1/batch_messages/:batch_message_id)
- 上传图片 (POST /open-apis/im/v1/images)
- 下载图片 (GET /open-apis/im/v1/images/:image_key)
- 下载文件 (GET /open-apis/im/v1/files/:file_key)
- 添加消息表情回复 (POST /open-apis/im/v1/messages/:message_id/reactions)
- 获取消息表情回复 (GET /open-apis/im/v1/messages/:message_id/reactions)
- 删除消息表情回复 (DELETE /open-apis/im/v1/messages/:message_id/reactions/:reaction_id)
- Pin 消息 (POST /open-apis/im/v1/pins)
- 移除 Pin 消息 (DELETE /open-apis/im/v1/pins/:message_id)
- 获取群内 Pin 消息 (GET /open-apis/im/v1/pins)
- 更新已发送的消息卡片 (PATCH /open-apis/im/v1/messages/:message_id)
- 创建群 (POST /open-apis/im/v1/chats)
- 解散群 (DELETE /open-apis/im/v1/chats/:chat_id)
- 更新群信息 (PUT /open-apis/im/v1/chats/:chat_id)
- 更新群发言权限 (PUT /open-apis/im/v1/chats/:chat_id/moderation)
- 获取群信息 (GET /open-apis/im/v1/chats/:chat_id)
- 更新群置顶 (POST /open-apis/im/v1/chats/:chat_id/top_notice/put_top_notice)
- 撤销群置顶 (POST /open-apis/im/v1/chats/:chat_id/top_notice/delete_top_notice)
- 获取用户或机器人所在的群列表 (GET /open-apis/im/v1/chats)
- 获取群成员发言权限 (GET /open-apis/im/v1/chats/:chat_id/moderation)
- 获取群分享链接 (POST /open-apis/im/v1/chats/:chat_id/link)
- 指定群管理员 (POST /open-apis/im/v1/chats/:chat_id/managers/add_managers)
- 删除群管理员 (POST /open-apis/im/v1/chats/:chat_id/managers/delete_managers)
- 用户或机器人主动加入群聊 (PATCH /open-apis/im/v1/chats/:chat_id/members/me_join)
- 判断用户或机器人是否在群里 (GET /open-apis/im/v1/chats/:chat_id/members/is_in_chat)
- 获取群公告基本信息 (GET /open-apis/docx/v1/chats/:chat_id/announcement)
- 获取群公告块的内容 (GET /open-apis/docx/v1/chats/:chat_id/announcement/blocks/:block_id)
- 更新群公告信息 (PATCH /open-apis/im/v1/chats/:chat_id/announcement)
- 获取群公告信息 (GET /open-apis/im/v1/chats/:chat_id/announcement)
- 添加会话标签页 (POST /open-apis/im/v1/chats/:chat_id/chat_tabs)
- 删除会话标签页 (DELETE /open-apis/im/v1/chats/:chat_id/chat_tabs/delete_tabs)
- 更新会话标签页 (POST /open-apis/im/v1/chats/:chat_id/chat_tabs/update_tabs)
- 会话标签页排序 (POST /open-apis/im/v1/chats/:chat_id/chat_tabs/sort_tabs)
- 拉取会话标签页 (GET /open-apis/im/v1/chats/:chat_id/chat_tabs/list_tabs)
- 添加群菜单 (POST /open-apis/im/v1/chats/:chat_id/menu_tree)
- 删除群菜单 (DELETE /open-apis/im/v1/chats/:chat_id/menu_tree)
- 修改群菜单元信息 (PATCH /open-apis/im/v1/chats/:chat_id/menu_items/:menu_item_id)
- 排序群菜单 (POST /open-apis/im/v1/chats/:chat_id/menu_tree/sort)
- 获取群菜单 (GET /open-apis/im/v1/chats/:chat_id/menu_tree)
- 创建卡片实体 (POST /open-apis/cardkit/v1/cards)
- 全量更新卡片实体 (PUT /open-apis/cardkit/v1/cards/:card_id)
- 新增组件 (POST /open-apis/cardkit/v1/cards/:card_id/elements)
- 更新组件 (PUT /open-apis/cardkit/v1/cards/:card_id/elements/:element_id)
- 更新组件属性 (PATCH /open-apis/cardkit/v1/cards/:card_id/elements/:element_id)
- 删除组件 (DELETE /open-apis/cardkit/v1/cards/:card_id/elements/:element_id)
- 创建应用消息流卡片 (POST /open-apis/im/v2/app_feed_card)
- 机器人单聊即时提醒 (PATCH /open-apis/im/v2/feed_cards/bot_time_sentive)
- 更新消息流卡片按钮 (PUT /open-apis/im/v2/chat_button)
- 即时提醒 (PATCH /open-apis/im/v2/feed_cards/:feed_card_id)
- 查询实体与标签的绑定关系 (GET /open-apis/im/v2/biz_entity_tag_relation)
- 修改标签 (PATCH /open-apis/im/v2/tags/:tag_id)
- 绑定标签到群 (POST /open-apis/im/v2/biz_entity_tag_relation)
- 解绑标签与群 (PUT /open-apis/im/v2/biz_entity_tag_relation)
- 查询异步任务状态 (GET /open-apis/drive/v1/files/task_check)
- 删除文件或文件夹 (DELETE /open-apis/drive/v1/files/:file_token)
- 创建文件快捷方式 (POST /open-apis/drive/v1/files/create_shortcut)
- 搜索云文档 (POST /open-apis/suite/docs-api/search/object)
- 创建导入任务 (POST /open-apis/drive/v1/import_tasks)
- 创建导出任务 (POST /open-apis/drive/v1/export_tasks)
- 获取文档版本信息 (GET /open-apis/drive/v1/files/:file_token/versions/:version_id)
- 删除文档版本 (DELETE /open-apis/drive/v1/files/:file_token/versions/:version_id)
- 查询云文档事件订阅状态 (GET /open-apis/drive/v1/files/:file_token/get_subscribe)
- 取消云文档事件订阅 (DELETE /open-apis/drive/v1/files/:file_token/delete_subscribe)
- 获取知识空间信息 (GET /open-apis/wiki/v2/spaces/:space_id)
- 删除知识空间成员 (DELETE /open-apis/wiki/v2/spaces/:space_id/members/:member_id)
- 获取知识空间节点信息 (GET /open-apis/wiki/v2/spaces/get_node)
- 获取任务结果 (GET /open-apis/wiki/v2/tasks/:task_id)
- 创建文档 (POST /open-apis/docx/v1/documents)
- 获取文档基本信息 (GET /open-apis/docx/v1/documents/:document_id)
- 创建嵌套块 (POST /open-apis/docx/v1/documents/:document_id/blocks/:block_id/descendant)
- 更新块的内容 (PATCH /open-apis/docx/v1/documents/:document_id/blocks/:block_id)
- 获取块的内容 (GET /open-apis/docx/v1/documents/:document_id/blocks/:block_id)
- 创建电子表格 (POST /open-apis/sheets/v3/spreadsheets)
- ... 还有 587 个未实现的API
