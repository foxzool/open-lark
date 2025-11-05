# 核心API实现映射表

**生成时间**: 2025-11-05  
**核心API总数**: 300  
**已实现**: 144  
**实现率**: 48.0%  

| 序号 | API名称 | 请求方式 | API地址 | 文件路径 | 行号 | 实现预览 |
|------|---------|----------|---------|----------|------|----------|
| 1 | ❌ 获取事件出口 IP | GET | `/open-apis/event/v1/outbound_ip` | `未找到` | - | - |
| 2 | ❌ 获取用户信息 | GET | `/open-apis/authen/v1/user_info` | `未找到` | - | - |
| 3 | ❌ 批量获取脱敏的用户登录信息 | POST | `/open-apis/passport/v1/sessions/query` | `未找到` | - | - |
| 4 | ✅ 自建应用获取 tenant_access_token | POST | `/open-apis/auth/v3/tenant_access_token/internal` | `src/service/auth/v3/mod.rs` | 27 | pub async fn tenant_access_token_internal( |
| 5 | ✅ 自建应用获取 app_access_token | POST | `/open-apis/auth/v3/app_access_token/internal` | `src/service/auth/v3/mod.rs` | 27 | pub async fn tenant_access_token_internal( |
| 6 | ✅ 重新获取 app_ticket | POST | `/open-apis/auth/v3/app_ticket/resend` | `src/service/auth/v3/mod.rs` | 61 | pub async fn app_ticket_resend( |
| 7 | ✅ 商店应用获取 app_access_token | POST | `/open-apis/auth/v3/app_access_token` | `src/service/auth/v3/mod.rs` | 44 | pub async fn app_access_token_internal( |
| 8 | ✅ 商店应用获取 tenant_access_token | POST | `/open-apis/auth/v3/tenant_access_token` | `src/service/auth/v3/mod.rs` | 27 | pub async fn tenant_access_token_internal( |
| 9 | ✅ 获取通讯录授权范围 | GET | `/open-apis/contact/v3/scopes` | `src/service/contact/v3/functional_role_member.rs` | 63 | pub async fn scopes( |
| 10 | ✅ 创建用户 | POST | `/open-apis/contact/v3/users` | `src/service/contact/v3/group_member.rs` | 26 | pub async fn add( |
| 11 | ✅ 修改用户部分信息 | PATCH | `/open-apis/contact/v3/users/:user_id` | `src/service/contact/v3/group_member.rs` | 26 | pub async fn add( |
| 12 | ✅ 更新用户 ID | PATCH | `/open-apis/contact/v3/users/:user_id/update_user_id` | `src/service/contact/v3/group_member.rs` | 26 | pub async fn add( |
| 13 | ✅ 获取单个用户信息 | GET | `/open-apis/contact/v3/users/:user_id` | `src/service/contact/v3/group_member.rs` | 26 | pub async fn add( |
| 14 | ✅ 批量获取用户信息 | GET | `/open-apis/contact/v3/users/batch` | `src/service/contact/v3/group_member.rs` | 44 | pub async fn batch_add( |
| 15 | ✅ 获取部门直属用户列表 | GET | `/open-apis/contact/v3/users/find_by_department` | `src/service/contact/v3/group_member.rs` | 26 | pub async fn add( |
| 16 | ✅ 通过手机号或邮箱获取用户 ID | POST | `/open-apis/contact/v3/users/batch_get_id` | `src/service/contact/v3/group_member.rs` | 26 | pub async fn add( |
| 17 | ✅ 搜索用户 | GET | `/open-apis/search/v1/user` | `src/service/search/v1/user.rs` | 282 | pub async fn search_user_with_validated_pagination( |
| 18 | ✅ 删除用户 | DELETE | `/open-apis/contact/v3/users/:user_id` | `src/service/contact/v3/group_member.rs` | 26 | pub async fn add( |
| 19 | ✅ 恢复已删除用户 | POST | `/open-apis/contact/v3/users/:user_id/resurrect` | `src/service/contact/v3/group_member.rs` | 26 | pub async fn add( |
| 20 | ✅ 创建用户组 | POST | `/open-apis/contact/v3/group` | `src/service/contact/v3/group.rs` | 206 | pub async fn get_user_groups( |
| 21 | ✅ 更新用户组 | PATCH | `/open-apis/contact/v3/group/:group_id` | `src/service/contact/v3/group_member.rs` | 26 | pub async fn add( |
| 22 | ✅ 查询指定用户组 | GET | `/open-apis/contact/v3/group/:group_id` | `src/service/contact/v3/group_member.rs` | 26 | pub async fn add( |
| 23 | ✅ 查询用户组列表 | GET | `/open-apis/contact/v3/group/simplelist` | `src/service/contact/v3/group_member.rs` | 62 | pub async fn simplelist( |
| 24 | ✅ 查询用户所属用户组 | GET | `/open-apis/contact/v3/group/member_belong` | `src/service/contact/v3/group_member.rs` | 26 | pub async fn add( |
| 25 | ✅ 删除用户组 | DELETE | `/open-apis/contact/v3/group/:group_id` | `src/service/contact/v3/group_member.rs` | 26 | pub async fn add( |
| 26 | ✅ 获取企业自定义用户字段 | GET | `/open-apis/contact/v3/custom_attrs` | `src/service/contact/v3/group_member.rs` | 26 | pub async fn add( |
| 27 | ✅ 新增人员类型 | POST | `/open-apis/contact/v3/employee_type_enums` | `src/service/contact/v3/group_member.rs` | 26 | pub async fn add( |
| 28 | ✅ 更新人员类型 | PUT | `/open-apis/contact/v3/employee_type_enums/:enum_id` | `src/service/contact/v3/group_member.rs` | 26 | pub async fn add( |
| 29 | ✅ 查询人员类型 | GET | `/open-apis/contact/v3/employee_type_enums` | `src/service/contact/v3/group_member.rs` | 26 | pub async fn add( |
| 30 | ✅ 删除人员类型 | DELETE | `/open-apis/contact/v3/employee_type_enums/:enum_id` | `src/service/contact/v3/group_member.rs` | 26 | pub async fn add( |
| 31 | ✅ 创建部门 | POST | `/open-apis/contact/v3/departments` | `src/service/contact/v3/group_member.rs` | 26 | pub async fn add( |
| 32 | ✅ 修改部门部分信息 | PATCH | `/open-apis/contact/v3/departments/:department_id` | `src/service/contact/v3/department.rs` | 156 | pub async fn update_department_id( |
| 33 | ✅ 更新部门所有信息 | PUT | `/open-apis/contact/v3/departments/:department_id` | `src/service/contact/v3/department.rs` | 156 | pub async fn update_department_id( |
| 34 | ✅ 更新部门 ID | PATCH | `/open-apis/contact/v3/departments/:department_id/update_department_id` | `src/service/contact/v3/department.rs` | 156 | pub async fn update_department_id( |
| 35 | ✅ 部门群转为普通群 | POST | `/open-apis/contact/v3/departments/unbind_department_chat` | `src/service/contact/v3/group_member.rs` | 26 | pub async fn add( |
| 36 | ✅ 获取单个部门信息 | GET | `/open-apis/contact/v3/departments/:department_id` | `src/service/contact/v3/department.rs` | 156 | pub async fn update_department_id( |
| 37 | ✅ 批量获取部门信息 | GET | `/open-apis/contact/v3/departments/batch` | `src/service/contact/v3/group_member.rs` | 44 | pub async fn batch_add( |
| 38 | ✅ 获取子部门列表 | GET | `/open-apis/contact/v3/departments/:department_id/children` | `src/service/contact/v3/department.rs` | 230 | pub async fn get_children( |
| 39 | ✅ 获取父部门信息 | GET | `/open-apis/contact/v3/departments/parent` | `src/service/contact/v3/department.rs` | 276 | pub async fn get_parent( |
| 40 | ✅ 搜索部门 | POST | `/open-apis/contact/v3/departments/search` | `src/service/contact/v3/user.rs` | 485 | pub async fn search( |
| 41 | ✅ 删除部门 | DELETE | `/open-apis/contact/v3/departments/:department_id` | `src/service/contact/v3/department.rs` | 156 | pub async fn update_department_id( |
| 42 | ✅ 创建单位 | POST | `/open-apis/contact/v3/unit` | `src/service/contact/v3/unit.rs` | 130 | pub async fn get(&self, unit_id: &str) -> crate::core::SDKResult<GetUnitResponse... |
| 43 | ✅ 修改单位信息 | PATCH | `/open-apis/contact/v3/unit/:unit_id` | `src/service/contact/v3/unit.rs` | 130 | pub async fn get(&self, unit_id: &str) -> crate::core::SDKResult<GetUnitResponse... |
| 44 | ✅ 建立部门与单位的绑定关系 | POST | `/open-apis/contact/v3/unit/bind_department` | `src/service/contact/v3/unit.rs` | 62 | pub async fn bind_department( |
| 45 | ✅ 解除部门与单位的绑定关系 | POST | `/open-apis/contact/v3/unit/unbind_department` | `src/service/contact/v3/unit.rs` | 83 | pub async fn unbind_department( |
| 46 | ✅ 获取单位绑定的部门列表 | GET | `/open-apis/contact/v3/unit/list_department` | `src/service/contact/v3/unit.rs` | 104 | pub async fn list_department( |
| 47 | ✅ 获取单位信息 | GET | `/open-apis/contact/v3/unit/:unit_id` | `src/service/contact/v3/unit.rs` | 130 | pub async fn get(&self, unit_id: &str) -> crate::core::SDKResult<GetUnitResponse... |
| 48 | ✅ 获取单位列表 | GET | `/open-apis/contact/v3/unit` | `src/service/contact/v3/unit.rs` | 130 | pub async fn get(&self, unit_id: &str) -> crate::core::SDKResult<GetUnitResponse... |
| 49 | ✅ 删除单位 | DELETE | `/open-apis/contact/v3/unit/:unit_id` | `src/service/contact/v3/unit.rs` | 130 | pub async fn get(&self, unit_id: &str) -> crate::core::SDKResult<GetUnitResponse... |
| 50 | ✅ 添加用户组成员 | POST | `/open-apis/contact/v3/group/:group_id/member/add` | `src/service/contact/v3/group_member.rs` | 26 | pub async fn add( |
| 51 | ✅ 批量添加用户组成员 | POST | `/open-apis/contact/v3/group/:group_id/member/batch_add` | `src/service/contact/v3/group_member.rs` | 44 | pub async fn batch_add( |
| 52 | ✅ 查询用户组成员列表 | GET | `/open-apis/contact/v3/group/:group_id/member/simplelist` | `src/service/contact/v3/group_member.rs` | 62 | pub async fn simplelist( |
| 53 | ✅ 移除用户组成员 | POST | `/open-apis/contact/v3/group/:group_id/member/remove` | `src/service/contact/v3/group_member.rs` | 84 | pub async fn remove( |
| 54 | ✅ 批量移除用户组成员 | POST | `/open-apis/contact/v3/group/:group_id/member/batch_remove` | `src/service/contact/v3/group_member.rs` | 105 | pub async fn batch_remove( |
| 55 | ✅ 创建角色 | POST | `/open-apis/contact/v3/functional_roles` | `src/service/contact/v3/group_member.rs` | 26 | pub async fn add( |
| 56 | ✅ 修改角色名称 | PUT | `/open-apis/contact/v3/functional_roles/:role_id` | `src/service/contact/v3/functional_role.rs` | 62 | pub async fn get(&self, role_id: &str) -> crate::core::SDKResult<GetFunctionalRo... |
| 57 | ✅ 删除角色 | DELETE | `/open-apis/contact/v3/functional_roles/:role_id` | `src/service/contact/v3/functional_role.rs` | 62 | pub async fn get(&self, role_id: &str) -> crate::core::SDKResult<GetFunctionalRo... |
| 58 | ✅ 批量添加角色成员 | POST | `/open-apis/contact/v3/functional_roles/:role_id/members/batch_create` | `src/service/contact/v3/functional_role_member.rs` | 42 | pub async fn batch_create( |
| 59 | ✅ 批量设置角色成员管理范围 | PATCH | `/open-apis/contact/v3/functional_roles/:role_id/members/scopes` | `src/service/contact/v3/functional_role_member.rs` | 63 | pub async fn scopes( |
| 60 | ✅ 查询角色下某个成员的管理范围 | GET | `/open-apis/contact/v3/functional_roles/:role_id/members/:member_id` | `src/service/contact/v3/group_member.rs` | 26 | pub async fn add( |
| 61 | ✅ 查询角色下的所有成员信息 | GET | `/open-apis/contact/v3/functional_roles/:role_id/members` | `src/service/contact/v3/group_member.rs` | 26 | pub async fn add( |
| 62 | ✅ 删除角色下的成员 | PATCH | `/open-apis/contact/v3/functional_roles/:role_id/members/batch_delete` | `src/service/contact/v3/functional_role_member.rs` | 131 | pub async fn batch_delete( |
| 63 | ✅ 创建职级 | POST | `/open-apis/contact/v3/job_levels` | `src/service/contact/v3/group_member.rs` | 26 | pub async fn add( |
| 64 | ✅ 更新职级 | PUT | `/open-apis/contact/v3/job_levels/:job_level_id` | `src/service/contact/v3/job_level.rs` | 65 | pub async fn get(&self, job_level_id: &str) -> crate::core::SDKResult<GetJobLeve... |
| 65 | ✅ 获取单个职级信息 | GET | `/open-apis/contact/v3/job_levels/:job_level_id` | `src/service/contact/v3/job_level.rs` | 65 | pub async fn get(&self, job_level_id: &str) -> crate::core::SDKResult<GetJobLeve... |
| 66 | ✅ 获取租户职级列表 | GET | `/open-apis/contact/v3/job_levels` | `src/service/contact/v3/group_member.rs` | 26 | pub async fn add( |
| 67 | ✅ 删除职级 | DELETE | `/open-apis/contact/v3/job_levels/:job_level_id` | `src/service/contact/v3/job_level.rs` | 65 | pub async fn get(&self, job_level_id: &str) -> crate::core::SDKResult<GetJobLeve... |
| 68 | ✅ 创建序列 | POST | `/open-apis/contact/v3/job_families` | `src/service/contact/v3/group_member.rs` | 26 | pub async fn add( |
| 69 | ✅ 更新序列 | PUT | `/open-apis/contact/v3/job_families/:job_family_id` | `src/service/contact/v3/job_family.rs` | 66 | pub async fn get(&self, job_family_id: &str) -> crate::core::SDKResult<GetJobFam... |
| 70 | ✅ 获取单个序列信息 | GET | `/open-apis/contact/v3/job_families/:job_family_id` | `src/service/contact/v3/job_family.rs` | 66 | pub async fn get(&self, job_family_id: &str) -> crate::core::SDKResult<GetJobFam... |
| 71 | ✅ 获取租户序列列表 | GET | `/open-apis/contact/v3/job_families` | `src/service/contact/v3/group_member.rs` | 26 | pub async fn add( |
| 72 | ✅ 删除序列 | DELETE | `/open-apis/contact/v3/job_families/:job_family_id` | `src/service/contact/v3/job_family.rs` | 66 | pub async fn get(&self, job_family_id: &str) -> crate::core::SDKResult<GetJobFam... |
| 73 | ✅ 获取单个职务信息 | GET | `/open-apis/contact/v3/job_titles/:job_title_id` | `src/service/contact/v3/job_title.rs` | 28 | pub async fn get(&self, job_title_id: &str) -> crate::core::SDKResult<GetJobTitl... |
| 74 | ✅ 获取租户职务列表 | GET | `/open-apis/contact/v3/job_titles` | `src/service/contact/v3/group_member.rs` | 26 | pub async fn add( |
| 75 | ✅ 获取单个工作城市信息 | GET | `/open-apis/contact/v3/work_cities/:work_city_id` | `src/service/contact/v3/work_city.rs` | 28 | pub async fn get(&self, work_city_id: &str) -> crate::core::SDKResult<GetWorkCit... |
| 76 | ✅ 获取租户工作城市列表 | GET | `/open-apis/contact/v3/work_cities` | `src/service/contact/v3/group_member.rs` | 26 | pub async fn add( |
| 77 | ❌ 创建员工 | POST | `/open-apis/directory/v1/employees` | `未找到` | - | - |
| 78 | ❌ 更新员工信息 | PATCH | `/open-apis/directory/v1/employees/:employee_id` | `未找到` | - | - |
| 79 | ❌ 更新在职员工为待离职 | PATCH | `/open-apis/directory/v1/employees/:employee_id/to_be_resigned` | `未找到` | - | - |
| 80 | ❌ 更新待离职成员为在职 | PATCH | `/open-apis/directory/v1/employees/:employee_id/regular` | `未找到` | - | - |
| 81 | ❌ 批量获取员工信息 | POST | `/open-apis/directory/v1/employees/mget` | `未找到` | - | - |
| 82 | ❌ 批量获取员工列表 | POST | `/open-apis/directory/v1/employees/filter` | `未找到` | - | - |
| 83 | ❌ 搜索员工信息 | POST | `/open-apis/directory/v1/employees/search` | `未找到` | - | - |
| 84 | ❌ 创建部门 | POST | `/open-apis/directory/v1/departments` | `未找到` | - | - |
| 85 | ❌ 更新部门 | PATCH | `/open-apis/directory/v1/departments/:department_id` | `未找到` | - | - |
| 86 | ❌ 删除部门 | DELETE | `/open-apis/directory/v1/departments/:department_id` | `未找到` | - | - |
| 87 | ❌ 批量获取部门信息 | POST | `/open-apis/directory/v1/departments/mget` | `未找到` | - | - |
| 88 | ❌ 获取部门列表 | POST | `/open-apis/directory/v1/departments/filter` | `未找到` | - | - |
| 89 | ❌ 搜索部门 | POST | `/open-apis/directory/v1/departments/search` | `未找到` | - | - |
| 90 | ✅ 发送消息 | POST | `/open-apis/im/v1/messages` | `src/service/im/v1/pin/mod.rs` | 72 | pub async fn create( |
| 91 | ✅ 回复消息 | POST | `/open-apis/im/v1/messages/:message_id/reply` | `src/service/im/v1/pin/mod.rs` | 72 | pub async fn create( |
| 92 | ✅ 编辑消息 | PUT | `/open-apis/im/v1/messages/:message_id` | `src/service/im/v1/pin/mod.rs` | 72 | pub async fn create( |
| 93 | ✅ 转发消息 | POST | `/open-apis/im/v1/messages/:message_id/forward` | `src/service/im/v1/pin/mod.rs` | 72 | pub async fn create( |
| 94 | ✅ 合并转发消息 | POST | `/open-apis/im/v1/messages/merge_forward` | `src/service/im/v1/pin/mod.rs` | 72 | pub async fn create( |
| 95 | ✅ 转发话题 | POST | `/open-apis/im/v1/threads/:thread_id/forward` | `src/service/im/v1/pin/mod.rs` | 72 | pub async fn create( |
| 96 | ✅ 撤回消息 | DELETE | `/open-apis/im/v1/messages/:message_id` | `src/service/im/v1/pin/mod.rs` | 72 | pub async fn create( |
| 97 | ✅ 添加跟随气泡 | POST | `/open-apis/im/v1/messages/:message_id/push_follow_up` | `src/service/im/v1/pin/mod.rs` | 72 | pub async fn create( |
| 98 | ✅ 查询消息已读信息 | GET | `/open-apis/im/v1/messages/:message_id/read_users` | `src/service/im/v1/pin/mod.rs` | 72 | pub async fn create( |
| 99 | ✅ 获取会话历史消息 | GET | `/open-apis/im/v1/messages` | `src/service/im/v1/pin/mod.rs` | 72 | pub async fn create( |
| 100 | ✅ 获取消息中的资源文件 | GET | `/open-apis/im/v1/messages/:message_id/resources/:file_key` | `src/service/im/v1/pin/mod.rs` | 72 | pub async fn create( |
| 101 | ✅ 获取指定消息的内容 | GET | `/open-apis/im/v1/messages/:message_id` | `src/service/im/v1/pin/mod.rs` | 72 | pub async fn create( |
| 102 | ❌ 批量发送消息 | POST | `/open-apis/message/v4/batch_send/` | `未找到` | - | - |
| 103 | ✅ 批量撤回消息 | DELETE | `/open-apis/im/v1/batch_messages/:batch_message_id` | `src/service/im/v1/pin/mod.rs` | 72 | pub async fn create( |
| 104 | ✅ 查询批量消息推送和阅读人数 | GET | `/open-apis/im/v1/batch_messages/:batch_message_id/read_user` | `src/service/im/v1/batch_message/mod.rs` | 134 | pub async fn read_user( |
| 105 | ✅ 查询批量消息整体进度 | GET | `/open-apis/im/v1/batch_messages/:batch_message_id/get_progress` | `src/service/im/v1/batch_message/mod.rs` | 117 | pub async fn get_progress( |
| 106 | ✅ 上传图片 | POST | `/open-apis/im/v1/images` | `src/service/im/v1/pin/mod.rs` | 72 | pub async fn create( |
| 107 | ✅ 下载图片 | GET | `/open-apis/im/v1/images/:image_key` | `src/service/im/v1/pin/mod.rs` | 72 | pub async fn create( |
| 108 | ✅ 上传文件 | POST | `/open-apis/im/v1/files` | `src/service/im/v1/pin/mod.rs` | 72 | pub async fn create( |
| 109 | ✅ 下载文件 | GET | `/open-apis/im/v1/files/:file_key` | `src/service/im/v1/pin/mod.rs` | 72 | pub async fn create( |
| 110 | ✅ 发送应用内加急 | PATCH | `/open-apis/im/v1/messages/:message_id/urgent_app` | `src/service/im/v1/buzz_messages/mod.rs` | 52 | pub async fn urgent_app( |
| 111 | ✅ 发送短信加急 | PATCH | `/open-apis/im/v1/messages/:message_id/urgent_sms` | `src/service/im/v1/buzz_messages/mod.rs` | 73 | pub async fn urgent_sms( |
| 112 | ✅ 发送电话加急 | PATCH | `/open-apis/im/v1/messages/:message_id/urgent_phone` | `src/service/im/v1/buzz_messages/mod.rs` | 94 | pub async fn urgent_phone( |
| 113 | ✅ 添加消息表情回复 | POST | `/open-apis/im/v1/messages/:message_id/reactions` | `src/service/im/v1/pin/mod.rs` | 72 | pub async fn create( |
| 114 | ✅ 获取消息表情回复 | GET | `/open-apis/im/v1/messages/:message_id/reactions` | `src/service/im/v1/pin/mod.rs` | 72 | pub async fn create( |
| 115 | ✅ 删除消息表情回复 | DELETE | `/open-apis/im/v1/messages/:message_id/reactions/:reaction_id` | `src/service/im/v1/pin/mod.rs` | 72 | pub async fn create( |
| 116 | ✅ Pin 消息 | POST | `/open-apis/im/v1/pins` | `src/service/im/v1/pin/mod.rs` | 72 | pub async fn create( |
| 117 | ✅ 移除 Pin 消息 | DELETE | `/open-apis/im/v1/pins/:message_id` | `src/service/im/v1/pin/mod.rs` | 72 | pub async fn create( |
| 118 | ✅ 获取群内 Pin 消息 | GET | `/open-apis/im/v1/pins` | `src/service/im/v1/pin/mod.rs` | 72 | pub async fn create( |
| 119 | ✅ 更新已发送的消息卡片 | PATCH | `/open-apis/im/v1/messages/:message_id` | `src/service/im/v1/pin/mod.rs` | 72 | pub async fn create( |
| 120 | ❌ 延时更新消息卡片 | POST | `/open-apis/interactive/v1/card/update` | `未找到` | - | - |
| 121 | ❌ 删除仅特定人可见的消息卡片 | POST | `/open-apis/ephemeral/v1/delete` | `未找到` | - | - |
| 122 | ✅ 更新 URL 预览 | POST | `/open-apis/im/v2/url_previews/batch_update` | `src/service/im/v1/url_preview/mod.rs` | 31 | pub async fn batch_update( |
| 123 | ✅ 创建群 | POST | `/open-apis/im/v1/chats` | `src/service/im/v1/pin/mod.rs` | 72 | pub async fn create( |
| 124 | ✅ 解散群 | DELETE | `/open-apis/im/v1/chats/:chat_id` | `src/service/im/v1/pin/mod.rs` | 72 | pub async fn create( |
| 125 | ✅ 更新群信息 | PUT | `/open-apis/im/v1/chats/:chat_id` | `src/service/im/v1/pin/mod.rs` | 72 | pub async fn create( |
| 126 | ✅ 更新群发言权限 | PUT | `/open-apis/im/v1/chats/:chat_id/moderation` | `src/service/im/v1/pin/mod.rs` | 72 | pub async fn create( |
| 127 | ✅ 获取群信息 | GET | `/open-apis/im/v1/chats/:chat_id` | `src/service/im/v1/pin/mod.rs` | 72 | pub async fn create( |
| 128 | ✅ 更新群置顶 | POST | `/open-apis/im/v1/chats/:chat_id/top_notice/put_top_notice` | `src/service/im/v1/pin/mod.rs` | 72 | pub async fn create( |
| 129 | ✅ 撤销群置顶 | POST | `/open-apis/im/v1/chats/:chat_id/top_notice/delete_top_notice` | `src/service/im/v1/pin/mod.rs` | 72 | pub async fn create( |
| 130 | ✅ 获取用户或机器人所在的群列表 | GET | `/open-apis/im/v1/chats` | `src/service/im/v1/pin/mod.rs` | 72 | pub async fn create( |
| 131 | ✅ 搜索对用户或机器人可见的群列表 | GET | `/open-apis/im/v1/chats/search` | `src/service/im/v1/pin/mod.rs` | 72 | pub async fn create( |
| 132 | ✅ 获取群成员发言权限 | GET | `/open-apis/im/v1/chats/:chat_id/moderation` | `src/service/im/v1/pin/mod.rs` | 72 | pub async fn create( |
| 133 | ✅ 获取群分享链接 | POST | `/open-apis/im/v1/chats/:chat_id/link` | `src/service/im/v1/pin/mod.rs` | 72 | pub async fn create( |
| 134 | ✅ 指定群管理员 | POST | `/open-apis/im/v1/chats/:chat_id/managers/add_managers` | `src/service/im/v1/pin/mod.rs` | 72 | pub async fn create( |
| 135 | ✅ 删除群管理员 | POST | `/open-apis/im/v1/chats/:chat_id/managers/delete_managers` | `src/service/im/v1/pin/mod.rs` | 72 | pub async fn create( |
| 136 | ✅ 将用户或机器人拉入群聊 | POST | `/open-apis/im/v1/chats/:chat_id/members` | `src/service/im/v1/pin/mod.rs` | 72 | pub async fn create( |
| 137 | ✅ 用户或机器人主动加入群聊 | PATCH | `/open-apis/im/v1/chats/:chat_id/members/me_join` | `src/service/im/v1/pin/mod.rs` | 72 | pub async fn create( |
| 138 | ✅ 将用户或机器人移出群聊 | DELETE | `/open-apis/im/v1/chats/:chat_id/members` | `src/service/im/v1/pin/mod.rs` | 72 | pub async fn create( |
| 139 | ✅ 获取群成员列表 | GET | `/open-apis/im/v1/chats/:chat_id/members` | `src/service/im/v1/pin/mod.rs` | 72 | pub async fn create( |
| 140 | ✅ 判断用户或机器人是否在群里 | GET | `/open-apis/im/v1/chats/:chat_id/members/is_in_chat` | `src/service/im/v1/pin/mod.rs` | 72 | pub async fn create( |
| 141 | ❌ 获取群公告基本信息 | GET | `/open-apis/docx/v1/chats/:chat_id/announcement` | `未找到` | - | - |
| 142 | ❌ 获取群公告所有块 | GET | `/open-apis/docx/v1/chats/:chat_id/announcement/blocks` | `未找到` | - | - |
| 143 | ❌ 在群公告中创建块 | POST | `/open-apis/docx/v1/chats/:chat_id/announcement/blocks/:block_id/children` | `未找到` | - | - |
| 144 | ❌ 批量更新群公告块的内容 | PATCH | `/open-apis/docx/v1/chats/:chat_id/announcement/blocks/batch_update` | `未找到` | - | - |
| 145 | ❌ 获取群公告块的内容 | GET | `/open-apis/docx/v1/chats/:chat_id/announcement/blocks/:block_id` | `未找到` | - | - |
| 146 | ❌ 获取所有子块 | GET | `/open-apis/docx/v1/chats/:chat_id/announcement/blocks/:block_id/children` | `未找到` | - | - |
| 147 | ❌ 删除群公告中的块 | DELETE | `/open-apis/docx/v1/chats/:chat_id/announcement/blocks/:block_id/children/batch_delete` | `未找到` | - | - |
| 148 | ✅ 更新群公告信息 | PATCH | `/open-apis/im/v1/chats/:chat_id/announcement` | `src/service/im/v1/pin/mod.rs` | 72 | pub async fn create( |
| 149 | ✅ 获取群公告信息 | GET | `/open-apis/im/v1/chats/:chat_id/announcement` | `src/service/im/v1/pin/mod.rs` | 72 | pub async fn create( |
| 150 | ✅ 添加会话标签页 | POST | `/open-apis/im/v1/chats/:chat_id/chat_tabs` | `src/service/im/v1/pin/mod.rs` | 72 | pub async fn create( |
| 151 | ✅ 删除会话标签页 | DELETE | `/open-apis/im/v1/chats/:chat_id/chat_tabs/delete_tabs` | `src/service/im/v1/pin/mod.rs` | 72 | pub async fn create( |
| 152 | ✅ 更新会话标签页 | POST | `/open-apis/im/v1/chats/:chat_id/chat_tabs/update_tabs` | `src/service/im/v1/pin/mod.rs` | 72 | pub async fn create( |
| 153 | ✅ 会话标签页排序 | POST | `/open-apis/im/v1/chats/:chat_id/chat_tabs/sort_tabs` | `src/service/im/v1/pin/mod.rs` | 72 | pub async fn create( |
| 154 | ✅ 拉取会话标签页 | GET | `/open-apis/im/v1/chats/:chat_id/chat_tabs/list_tabs` | `src/service/im/v1/pin/mod.rs` | 72 | pub async fn create( |
| 155 | ✅ 添加群菜单 | POST | `/open-apis/im/v1/chats/:chat_id/menu_tree` | `src/service/im/v1/pin/mod.rs` | 72 | pub async fn create( |
| 156 | ✅ 删除群菜单 | DELETE | `/open-apis/im/v1/chats/:chat_id/menu_tree` | `src/service/im/v1/pin/mod.rs` | 72 | pub async fn create( |
| 157 | ✅ 修改群菜单元信息 | PATCH | `/open-apis/im/v1/chats/:chat_id/menu_items/:menu_item_id` | `src/service/im/v1/pin/mod.rs` | 72 | pub async fn create( |
| 158 | ✅ 排序群菜单 | POST | `/open-apis/im/v1/chats/:chat_id/menu_tree/sort` | `src/service/im/v1/pin/mod.rs` | 72 | pub async fn create( |
| 159 | ✅ 获取群菜单 | GET | `/open-apis/im/v1/chats/:chat_id/menu_tree` | `src/service/im/v1/pin/mod.rs` | 72 | pub async fn create( |
| 160 | ❌ 创建卡片实体 | POST | `/open-apis/cardkit/v1/cards` | `未找到` | - | - |
| 161 | ❌ 更新卡片实体配置 | PATCH | `/open-apis/cardkit/v1/cards/:card_id/settings` | `未找到` | - | - |
| 162 | ❌ 局部更新卡片实体 | POST | `/open-apis/cardkit/v1/cards/:card_id/batch_update` | `未找到` | - | - |
| 163 | ❌ 全量更新卡片实体 | PUT | `/open-apis/cardkit/v1/cards/:card_id` | `未找到` | - | - |
| 164 | ❌ 更新组件 | PUT | `/open-apis/cardkit/v1/cards/:card_id/elements/:element_id` | `未找到` | - | - |
| 165 | ❌ 更新组件属性 | PATCH | `/open-apis/cardkit/v1/cards/:card_id/elements/:element_id` | `未找到` | - | - |
| 166 | ❌ 流式更新文本 | PUT | `/open-apis/cardkit/v1/cards/:card_id/elements/:element_id/content` | `未找到` | - | - |
| 167 | ❌ 删除组件 | DELETE | `/open-apis/cardkit/v1/cards/:card_id/elements/:element_id` | `未找到` | - | - |
| 168 | ✅ 创建应用消息流卡片 | POST | `/open-apis/im/v2/app_feed_card` | `src/service/im/v1/pin/mod.rs` | 72 | pub async fn create( |
| 169 | ✅ 更新应用消息流卡片 | PUT | `/open-apis/im/v2/app_feed_card/batch` | `src/service/im/v1/url_preview/mod.rs` | 31 | pub async fn batch_update( |
| 170 | ✅ 删除应用消息流卡片 | DELETE | `/open-apis/im/v2/app_feed_card/batch` | `src/service/im/v1/url_preview/mod.rs` | 31 | pub async fn batch_update( |
| 171 | ✅ 机器人单聊即时提醒 | PATCH | `/open-apis/im/v2/feed_cards/bot_time_sentive` | `src/service/im/v1/pin/mod.rs` | 72 | pub async fn create( |
| 172 | ✅ 更新消息流卡片按钮 | PUT | `/open-apis/im/v2/chat_button` | `src/service/im/v1/pin/mod.rs` | 72 | pub async fn create( |
| 173 | ✅ 即时提醒 | PATCH | `/open-apis/im/v2/feed_cards/:feed_card_id` | `src/service/im/v1/pin/mod.rs` | 72 | pub async fn create( |
| 174 | ✅ 查询实体与标签的绑定关系 | GET | `/open-apis/im/v2/biz_entity_tag_relation` | `src/service/im/v1/pin/mod.rs` | 72 | pub async fn create( |
| 175 | ✅ 创建标签 | POST | `/open-apis/im/v2/tags` | `src/service/im/v1/pin/mod.rs` | 72 | pub async fn create( |
| 176 | ✅ 修改标签 | PATCH | `/open-apis/im/v2/tags/:tag_id` | `src/service/im/v1/pin/mod.rs` | 72 | pub async fn create( |
| 177 | ✅ 绑定标签到群 | POST | `/open-apis/im/v2/biz_entity_tag_relation` | `src/service/im/v1/pin/mod.rs` | 72 | pub async fn create( |
| 178 | ✅ 解绑标签与群 | PUT | `/open-apis/im/v2/biz_entity_tag_relation` | `src/service/im/v1/pin/mod.rs` | 72 | pub async fn create( |
| 179 | ❌ 获取我的空间（根文件夹）元数据 | GET | `/open-apis/drive/explorer/v2/root_folder/meta` | `未找到` | - | - |
| 180 | ❌ 获取文件夹中的文件清单 | GET | `/open-apis/drive/v1/files` | `未找到` | - | - |
| 181 | ❌ 获取文件夹元数据 | GET | `/open-apis/drive/explorer/v2/folder/:folderToken/meta` | `未找到` | - | - |
| 182 | ❌ 获取文件元数据 | POST | `/open-apis/drive/v1/metas/batch_query` | `未找到` | - | - |
| 183 | ❌ 获取文件统计信息 | GET | `/open-apis/drive/v1/files/:file_token/statistics` | `未找到` | - | - |
| 184 | ❌ 获取文件访问记录 | GET | `/open-apis/drive/v1/files/:file_token/view_records` | `未找到` | - | - |
| 185 | ❌ 复制文件 | POST | `/open-apis/drive/v1/files/:file_token/copy` | `未找到` | - | - |
| 186 | ❌ 移动文件或文件夹 | POST | `/open-apis/drive/v1/files/:file_token/move` | `未找到` | - | - |
| 187 | ❌ 删除文件或文件夹 | DELETE | `/open-apis/drive/v1/files/:file_token` | `未找到` | - | - |
| 188 | ❌ 创建文件快捷方式 | POST | `/open-apis/drive/v1/files/create_shortcut` | `未找到` | - | - |
| 189 | ❌ 搜索云文档 | POST | `/open-apis/suite/docs-api/search/object` | `未找到` | - | - |
| 190 | ❌ 下载文件 | GET | `/open-apis/drive/v1/files/:file_token/download` | `未找到` | - | - |
| 191 | ❌ 创建导入任务 | POST | `/open-apis/drive/v1/import_tasks` | `未找到` | - | - |
| 192 | ❌ 查询导入任务结果 | GET | `/open-apis/drive/v1/import_tasks/:ticket` | `未找到` | - | - |
| 193 | ❌ 创建导出任务 | POST | `/open-apis/drive/v1/export_tasks` | `未找到` | - | - |
| 194 | ❌ 下载导出文件 | GET | `/open-apis/drive/export_tasks/file/:file_token/download` | `未找到` | - | - |
| 195 | ❌ 下载素材 | GET | `/open-apis/drive/v1/medias/:file_token/download` | `未找到` | - | - |
| 196 | ❌ 获取素材临时下载链接 | GET | `/open-apis/drive/v1/medias/batch_get_tmp_download_url` | `未找到` | - | - |
| 197 | ❌ 创建文档版本 | POST | `/open-apis/drive/v1/files/:file_token/versions` | `未找到` | - | - |
| 198 | ❌ 获取文档版本列表 | GET | `/open-apis/drive/v1/files/:file_token/versions` | `未找到` | - | - |
| 199 | ❌ 获取文档版本信息 | GET | `/open-apis/drive/v1/files/:file_token/versions/:version_id` | `未找到` | - | - |
| 200 | ❌ 删除文档版本 | DELETE | `/open-apis/drive/v1/files/:file_token/versions/:version_id` | `未找到` | - | - |
| 201 | ❌ 获取云文档的点赞者列表 | GET | `/open-apis/drive/v2/files/:file_token/likes` | `未找到` | - | - |
| 202 | ❌ 订阅云文档事件 | POST | `/open-apis/drive/v1/files/:file_token/subscribe` | `未找到` | - | - |
| 203 | ❌ 查询云文档事件订阅状态 | GET | `/open-apis/drive/v1/files/:file_token/get_subscribe` | `未找到` | - | - |
| 204 | ❌ 取消云文档事件订阅 | DELETE | `/open-apis/drive/v1/files/:file_token/delete_subscribe` | `未找到` | - | - |
| 205 | ❌ 获取知识空间列表 | GET | `/open-apis/wiki/v2/spaces` | `未找到` | - | - |
| 206 | ❌ 获取知识空间信息 | GET | `/open-apis/wiki/v2/spaces/:space_id` | `未找到` | - | - |
| 207 | ❌ 创建知识空间 | POST | `/open-apis/wiki/v2/spaces` | `未找到` | - | - |
| 208 | ❌ 获取知识空间成员列表 | GET | `/open-apis/wiki/v2/spaces/:space_id/members` | `未找到` | - | - |
| 209 | ❌ 删除知识空间成员 | DELETE | `/open-apis/wiki/v2/spaces/:space_id/members/:member_id` | `未找到` | - | - |
| 210 | ❌ 更新知识空间设置 | PUT | `/open-apis/wiki/v2/spaces/:space_id/setting` | `未找到` | - | - |
| 211 | ❌ 创建知识空间节点 | POST | `/open-apis/wiki/v2/spaces/:space_id/nodes` | `未找到` | - | - |
| 212 | ❌ 获取知识空间节点信息 | GET | `/open-apis/wiki/v2/spaces/get_node` | `未找到` | - | - |
| 213 | ❌ 获取知识空间子节点列表 | GET | `/open-apis/wiki/v2/spaces/:space_id/nodes` | `未找到` | - | - |
| 214 | ❌ 移动知识空间节点 | POST | `/open-apis/wiki/v2/spaces/:space_id/nodes/:node_token/move` | `未找到` | - | - |
| 215 | ❌ 更新知识空间节点标题 | POST | `/open-apis/wiki/v2/spaces/:space_id/nodes/:node_token/update_title` | `未找到` | - | - |
| 216 | ❌ 创建知识空间节点副本 | POST | `/open-apis/wiki/v2/spaces/:space_id/nodes/:node_token/copy` | `未找到` | - | - |
| 217 | ❌ 获取任务结果 | GET | `/open-apis/wiki/v2/tasks/:task_id` | `未找到` | - | - |
| 218 | ❌ 搜索 Wiki | POST | `/open-apis/wiki/v1/nodes/search` | `未找到` | - | - |
| 219 | ❌ 创建文档 | POST | `/open-apis/docx/v1/documents` | `未找到` | - | - |
| 220 | ❌ 获取文档基本信息 | GET | `/open-apis/docx/v1/documents/:document_id` | `未找到` | - | - |
| 221 | ❌ 获取文档纯文本内容 | GET | `/open-apis/docx/v1/documents/:document_id/raw_content` | `未找到` | - | - |
| 222 | ❌ 获取文档所有块 | GET | `/open-apis/docx/v1/documents/:document_id/blocks` | `未找到` | - | - |
| 223 | ❌ 创建块 | POST | `/open-apis/docx/v1/documents/:document_id/blocks/:block_id/children` | `未找到` | - | - |
| 224 | ❌ 创建嵌套块 | POST | `/open-apis/docx/v1/documents/:document_id/blocks/:block_id/descendant` | `未找到` | - | - |
| 225 | ❌ 更新块的内容 | PATCH | `/open-apis/docx/v1/documents/:document_id/blocks/:block_id` | `未找到` | - | - |
| 226 | ❌ 获取块的内容 | GET | `/open-apis/docx/v1/documents/:document_id/blocks/:block_id` | `未找到` | - | - |
| 227 | ❌ 批量更新块的内容 | PATCH | `/open-apis/docx/v1/documents/:document_id/blocks/batch_update` | `未找到` | - | - |
| 228 | ❌ 获取所有子块 | GET | `/open-apis/docx/v1/documents/:document_id/blocks/:block_id/children` | `未找到` | - | - |
| 229 | ❌ 删除块 | DELETE | `/open-apis/docx/v1/documents/:document_id/blocks/:block_id/children/batch_delete` | `未找到` | - | - |
| 230 | ❌ 创建电子表格 | POST | `/open-apis/sheets/v3/spreadsheets` | `未找到` | - | - |
| 231 | ❌ 修改电子表格属性 | PATCH | `/open-apis/sheets/v3/spreadsheets/:spreadsheet_token` | `未找到` | - | - |
| 232 | ❌ 获取电子表格信息 | GET | `/open-apis/sheets/v3/spreadsheets/:spreadsheet_token` | `未找到` | - | - |
| 233 | ❌ 操作工作表 | POST | `/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/sheets_batch_update` | `未找到` | - | - |
| 234 | ❌ 更新工作表属性 | POST | `/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/sheets_batch_update` | `未找到` | - | - |
| 235 | ❌ 获取工作表 | GET | `/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/query` | `未找到` | - | - |
| 236 | ❌ 查询工作表 | GET | `/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id` | `未找到` | - | - |
| 237 | ❌ 增加行列 | POST | `/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/dimension_range` | `未找到` | - | - |
| 238 | ❌ 插入行列 | POST | `/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/insert_dimension_range` | `未找到` | - | - |
| 239 | ❌ 更新行列 | PUT | `/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/dimension_range` | `未找到` | - | - |
| 240 | ❌ 移动行列 | POST | `/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/move_dimension` | `未找到` | - | - |
| 241 | ❌ 删除行列 | DELETE | `/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/dimension_range` | `未找到` | - | - |
| 242 | ❌ 查找单元格 | POST | `/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/find` | `未找到` | - | - |
| 243 | ❌ 替换单元格 | POST | `/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/replace` | `未找到` | - | - |
| 244 | ❌ 批量设置单元格样式  | PUT | `/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/styles_batch_update` | `未找到` | - | - |
| 245 | ❌ 写入图片 | POST | `/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/values_image` | `未找到` | - | - |
| 246 | ❌ 读取多个范围 | GET | `/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/values_batch_get` | `未找到` | - | - |
| 247 | ❌ 向多个范围写入数据 | POST | `/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/values_batch_update` | `未找到` | - | - |
| 248 | ❌ 创建筛选 | POST | `/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter` | `未找到` | - | - |
| 249 | ❌ 更新筛选 | PUT | `/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter` | `未找到` | - | - |
| 250 | ❌ 获取筛选 | GET | `/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter` | `未找到` | - | - |
| 251 | ❌ 删除筛选 | DELETE | `/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter` | `未找到` | - | - |
| 252 | ❌ 创建筛选视图 | POST | `/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter_views` | `未找到` | - | - |
| 253 | ❌ 更新筛选视图 | PATCH | `/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter_views/:filter_view_id` | `未找到` | - | - |
| 254 | ❌ 查询筛选视图 | GET | `/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter_views/query` | `未找到` | - | - |
| 255 | ❌ 获取筛选视图 | GET | `/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter_views/:filter_view_id` | `未找到` | - | - |
| 256 | ❌ 删除筛选视图 | DELETE | `/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter_views/:filter_view_id` | `未找到` | - | - |
| 257 | ❌ 创建筛选条件 | POST | `/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter_views/:filter_view_id/conditions` | `未找到` | - | - |
| 258 | ❌ 更新筛选条件 | PUT | `/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter_views/:filter_view_id/conditions/:condition_id` | `未找到` | - | - |
| 259 | ❌ 查询筛选条件 | GET | `/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter_views/:filter_view_id/conditions/query` | `未找到` | - | - |
| 260 | ❌ 获取筛选条件 | GET | `/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter_views/:filter_view_id/conditions/:condition_id` | `未找到` | - | - |
| 261 | ❌ 删除筛选条件 | DELETE | `/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/filter_views/:filter_view_id/conditions/:condition_id` | `未找到` | - | - |
| 262 | ❌ 增加保护范围 | POST | `/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/protected_dimension` | `未找到` | - | - |
| 263 | ❌ 修改保护范围 | POST | `/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/protected_range_batch_update` | `未找到` | - | - |
| 264 | ❌ 获取保护范围 | GET | `/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/protected_range_batch_get` | `未找到` | - | - |
| 265 | ❌ 删除保护范围 | DELETE | `/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/protected_range_batch_del` | `未找到` | - | - |
| 266 | ❌ 更新下拉列表设置 | PUT | `/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/dataValidation/:sheetId/:dataValidationId` | `未找到` | - | - |
| 267 | ❌ 删除下拉列表设置 | DELETE | `/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/dataValidation` | `未找到` | - | - |
| 268 | ❌ 批量创建条件格式 | POST | `/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/condition_formats/batch_create` | `未找到` | - | - |
| 269 | ❌ 批量更新条件格式 | POST | `/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/condition_formats/batch_update` | `未找到` | - | - |
| 270 | ❌ 批量获取条件格式 | GET | `/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/condition_formats` | `未找到` | - | - |
| 271 | ❌ 批量删除条件格式 | DELETE | `/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/condition_formats/batch_delete` | `未找到` | - | - |
| 272 | ❌ 创建浮动图片 | POST | `/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/float_images` | `未找到` | - | - |
| 273 | ❌ 更新浮动图片 | PATCH | `/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/float_images/:float_image_id` | `未找到` | - | - |
| 274 | ❌ 获取浮动图片 | GET | `/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/float_images/:float_image_id` | `未找到` | - | - |
| 275 | ❌ 查询浮动图片 | GET | `/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/float_images/query` | `未找到` | - | - |
| 276 | ❌ 删除浮动图片 | DELETE | `/open-apis/sheets/v3/spreadsheets/:spreadsheet_token/sheets/:sheet_id/float_images/:float_image_id` | `未找到` | - | - |
| 277 | ❌ 创建多维表格 | POST | `/open-apis/bitable/v1/apps` | `未找到` | - | - |
| 278 | ❌ 复制多维表格 | POST | `/open-apis/bitable/v1/apps/:app_token/copy` | `未找到` | - | - |
| 279 | ❌ 获取多维表格元数据 | GET | `/open-apis/bitable/v1/apps/:app_token` | `未找到` | - | - |
| 280 | ❌ 更新多维表格元数据 | PUT | `/open-apis/bitable/v1/apps/:app_token` | `未找到` | - | - |
| 281 | ❌ 新增一个数据表 | POST | `/open-apis/bitable/v1/apps/:app_token/tables` | `未找到` | - | - |
| 282 | ❌ 新增多个数据表 | POST | `/open-apis/bitable/v1/apps/:app_token/tables/batch_create` | `未找到` | - | - |
| 283 | ❌ 更新数据表 | PATCH | `/open-apis/bitable/v1/apps/:app_token/tables/:table_id` | `未找到` | - | - |
| 284 | ❌ 列出数据表 | GET | `/open-apis/bitable/v1/apps/:app_token/tables` | `未找到` | - | - |
| 285 | ❌ 删除一个数据表 | DELETE | `/open-apis/bitable/v1/apps/:app_token/tables/:table_id` | `未找到` | - | - |
| 286 | ❌ 删除多个数据表 | POST | `/open-apis/bitable/v1/apps/:app_token/tables/batch_delete` | `未找到` | - | - |
| 287 | ❌ 新增视图 | POST | `/open-apis/bitable/v1/apps/:app_token/tables/:table_id/views` | `未找到` | - | - |
| 288 | ❌ 更新视图 | PATCH | `/open-apis/bitable/v1/apps/:app_token/tables/:table_id/views/:view_id` | `未找到` | - | - |
| 289 | ❌ 列出视图 | GET | `/open-apis/bitable/v1/apps/:app_token/tables/:table_id/views` | `未找到` | - | - |
| 290 | ❌ 获取视图 | GET | `/open-apis/bitable/v1/apps/:app_token/tables/:table_id/views/:view_id` | `未找到` | - | - |
| 291 | ❌ 删除视图 | DELETE | `/open-apis/bitable/v1/apps/:app_token/tables/:table_id/views/:view_id` | `未找到` | - | - |
| 292 | ❌ 新增记录 | POST | `/open-apis/bitable/v1/apps/:app_token/tables/:table_id/records` | `未找到` | - | - |
| 293 | ❌ 更新记录 | PUT | `/open-apis/bitable/v1/apps/:app_token/tables/:table_id/records/:record_id` | `未找到` | - | - |
| 294 | ❌ 查询记录 | POST | `/open-apis/bitable/v1/apps/:app_token/tables/:table_id/records/search` | `未找到` | - | - |
| 295 | ❌ 删除记录 | DELETE | `/open-apis/bitable/v1/apps/:app_token/tables/:table_id/records/:record_id` | `未找到` | - | - |
| 296 | ❌ 新增多条记录 | POST | `/open-apis/bitable/v1/apps/:app_token/tables/:table_id/records/batch_create` | `未找到` | - | - |
| 297 | ❌ 更新多条记录 | POST | `/open-apis/bitable/v1/apps/:app_token/tables/:table_id/records/batch_update` | `未找到` | - | - |
| 298 | ❌ 批量获取记录 | POST | `/open-apis/bitable/v1/apps/:app_token/tables/:table_id/records/batch_get` | `未找到` | - | - |
| 299 | ❌ 删除多条记录 | POST | `/open-apis/bitable/v1/apps/:app_token/tables/:table_id/records/batch_delete` | `未找到` | - | - |
| 300 | ❌ 新增字段 | POST | `/open-apis/bitable/v1/apps/:app_token/tables/:table_id/fields` | `未找到` | - | - |


## 分类统计

### 按服务分类的实现情况

- **unknown**: 144 个API (100.0%)

### 未实现的核心API (156个)

- 获取事件出口 IP (GET /open-apis/event/v1/outbound_ip)
- 获取用户信息 (GET /open-apis/authen/v1/user_info)
- 批量获取脱敏的用户登录信息 (POST /open-apis/passport/v1/sessions/query)
- 创建员工 (POST /open-apis/directory/v1/employees)
- 更新员工信息 (PATCH /open-apis/directory/v1/employees/:employee_id)
- 更新在职员工为待离职 (PATCH /open-apis/directory/v1/employees/:employee_id/to_be_resigned)
- 更新待离职成员为在职 (PATCH /open-apis/directory/v1/employees/:employee_id/regular)
- 批量获取员工信息 (POST /open-apis/directory/v1/employees/mget)
- 批量获取员工列表 (POST /open-apis/directory/v1/employees/filter)
- 搜索员工信息 (POST /open-apis/directory/v1/employees/search)
- ... 还有 146 个未实现的核心API
