# API实现映射表

**生成时间**: 2025-11-05  
**总API数（示例）**: 100  
**找到实现**: 67  
**实现率**: 67.0%  

| 序号 | API名称 | 请求方式 | API地址 | 文件路径 | 行号 | 实现预览 |
|------|---------|----------|---------|----------|------|----------|
| 1 | ❌ 获取事件出口 IP | GET | `/open-apis/event/v1/outbound_ip` | `未找到` | - | - |
| 2 | ✅ 获取用户信息 | GET | `/open-apis/authen/v1/user_info` | `src/service/auth/v1/mod.rs` | 27 | pub async fn user_info(&self) -> SDKResult<UserInfoResponse> { |
| 3 | ✅ 批量获取脱敏的用户登录信息 | POST | `/open-apis/passport/v1/sessions/query` | `src/service/attendance/v1/user_task.rs` | 86 | pub async fn query_result( |
| 4 | ❌ 退出登录 | POST | `/open-apis/passport/v1/sessions/logout` | `未找到` | - | - |
| 5 | ✅ 自建应用获取 tenant_access_token | POST | `/open-apis/auth/v3/tenant_access_token/internal` | `src/service/auth/v3/mod.rs` | 27 | pub async fn tenant_access_token_internal( |
| 6 | ✅ 自建应用获取 app_access_token | POST | `/open-apis/auth/v3/app_access_token/internal` | `src/service/auth/v3/mod.rs` | 27 | pub async fn tenant_access_token_internal( |
| 7 | ✅ 重新获取 app_ticket | POST | `/open-apis/auth/v3/app_ticket/resend` | `src/service/auth/v3/mod.rs` | 61 | pub async fn app_ticket_resend( |
| 8 | ✅ 商店应用获取 app_access_token | POST | `/open-apis/auth/v3/app_access_token` | `src/service/auth/v3/mod.rs` | 44 | pub async fn app_access_token_internal( |
| 9 | ✅ 商店应用获取 tenant_access_token | POST | `/open-apis/auth/v3/tenant_access_token` | `src/service/auth/v3/mod.rs` | 27 | pub async fn tenant_access_token_internal( |
| 10 | ✅ 获取通讯录授权范围 | GET | `/open-apis/contact/v3/scopes` | `src/service/contact/v3/functional_role_member.rs` | 63 | pub async fn scopes( |
| 11 | ✅ 创建用户 | POST | `/open-apis/contact/v3/users` | `src/service/calendar/v4/mod.rs` | 302 | pub async fn get_users_free_busy( |
| 12 | ❌ 修改用户部分信息 | PATCH | `/open-apis/contact/v3/users/:user_id` | `未找到` | - | - |
| 13 | ❌ 更新用户 ID | PATCH | `/open-apis/contact/v3/users/:user_id/update_user_id` | `未找到` | - | - |
| 14 | ❌ 获取单个用户信息 | GET | `/open-apis/contact/v3/users/:user_id` | `未找到` | - | - |
| 15 | ✅ 批量获取用户信息 | GET | `/open-apis/contact/v3/users/batch` | `src/service/contact/v3/group_member.rs` | 44 | pub async fn batch_add( |
| 16 | ❌ 获取部门直属用户列表 | GET | `/open-apis/contact/v3/users/find_by_department` | `未找到` | - | - |
| 17 | ❌ 通过手机号或邮箱获取用户 ID | POST | `/open-apis/contact/v3/users/batch_get_id` | `未找到` | - | - |
| 18 | ✅ 搜索用户 | GET | `/open-apis/search/v1/user` | `src/service/search/v1/user.rs` | 282 | pub async fn search_user_with_validated_pagination( |
| 19 | ❌ 删除用户 | DELETE | `/open-apis/contact/v3/users/:user_id` | `未找到` | - | - |
| 20 | ❌ 恢复已删除用户 | POST | `/open-apis/contact/v3/users/:user_id/resurrect` | `未找到` | - | - |
| 21 | ✅ 创建用户组 | POST | `/open-apis/contact/v3/group` | `src/service/contact/v3/group.rs` | 206 | pub async fn get_user_groups( |
| 22 | ✅ 更新用户组 | PATCH | `/open-apis/contact/v3/group/:group_id` | `src/service/payroll/v1/paygroup.rs` | 53 | pub async fn get_paygroup(&self, paygroup_id: &str) -> SDKResult<GetPaygroupResp... |
| 23 | ✅ 查询指定用户组 | GET | `/open-apis/contact/v3/group/:group_id` | `src/service/payroll/v1/paygroup.rs` | 53 | pub async fn get_paygroup(&self, paygroup_id: &str) -> SDKResult<GetPaygroupResp... |
| 24 | ✅ 查询用户组列表 | GET | `/open-apis/contact/v3/group/simplelist` | `src/service/contact/v3/group_member.rs` | 62 | pub async fn simplelist( |
| 25 | ❌ 查询用户所属用户组 | GET | `/open-apis/contact/v3/group/member_belong` | `未找到` | - | - |
| 26 | ✅ 删除用户组 | DELETE | `/open-apis/contact/v3/group/:group_id` | `src/service/payroll/v1/paygroup.rs` | 53 | pub async fn get_paygroup(&self, paygroup_id: &str) -> SDKResult<GetPaygroupResp... |
| 27 | ❌ 获取企业自定义用户字段 | GET | `/open-apis/contact/v3/custom_attrs` | `未找到` | - | - |
| 28 | ❌ 新增人员类型 | POST | `/open-apis/contact/v3/employee_type_enums` | `未找到` | - | - |
| 29 | ❌ 更新人员类型 | PUT | `/open-apis/contact/v3/employee_type_enums/:enum_id` | `未找到` | - | - |
| 30 | ❌ 查询人员类型 | GET | `/open-apis/contact/v3/employee_type_enums` | `未找到` | - | - |
| 31 | ❌ 删除人员类型 | DELETE | `/open-apis/contact/v3/employee_type_enums/:enum_id` | `未找到` | - | - |
| 32 | ✅ 创建部门 | POST | `/open-apis/contact/v3/departments` | `src/service/corehr/organization/mod.rs` | 161 | pub async fn batch_get_departments( |
| 33 | ✅ 修改部门部分信息 | PATCH | `/open-apis/contact/v3/departments/:department_id` | `src/service/contact/v3/department.rs` | 156 | pub async fn update_department_id( |
| 34 | ✅ 更新部门所有信息 | PUT | `/open-apis/contact/v3/departments/:department_id` | `src/service/contact/v3/department.rs` | 156 | pub async fn update_department_id( |
| 35 | ✅ 更新部门 ID | PATCH | `/open-apis/contact/v3/departments/:department_id/update_department_id` | `src/service/contact/v3/department.rs` | 156 | pub async fn update_department_id( |
| 36 | ❌ 部门群转为普通群 | POST | `/open-apis/contact/v3/departments/unbind_department_chat` | `未找到` | - | - |
| 37 | ✅ 获取单个部门信息 | GET | `/open-apis/contact/v3/departments/:department_id` | `src/service/contact/v3/department.rs` | 156 | pub async fn update_department_id( |
| 38 | ✅ 批量获取部门信息 | GET | `/open-apis/contact/v3/departments/batch` | `src/service/contact/v3/group_member.rs` | 44 | pub async fn batch_add( |
| 39 | ✅ 获取子部门列表 | GET | `/open-apis/contact/v3/departments/:department_id/children` | `src/service/contact/v3/department.rs` | 230 | pub async fn get_children( |
| 40 | ✅ 获取父部门信息 | GET | `/open-apis/contact/v3/departments/parent` | `src/service/contact/v3/department.rs` | 276 | pub async fn get_parent( |
| 41 | ✅ 搜索部门 | POST | `/open-apis/contact/v3/departments/search` | `src/service/contact/v3/user.rs` | 485 | pub async fn search( |
| 42 | ✅ 删除部门 | DELETE | `/open-apis/contact/v3/departments/:department_id` | `src/service/contact/v3/department.rs` | 156 | pub async fn update_department_id( |
| 43 | ✅ 创建单位 | POST | `/open-apis/contact/v3/unit` | `src/service/contact/v3/unit.rs` | 130 | pub async fn get(&self, unit_id: &str) -> crate::core::SDKResult<GetUnitResponse... |
| 44 | ✅ 修改单位信息 | PATCH | `/open-apis/contact/v3/unit/:unit_id` | `src/service/contact/v3/unit.rs` | 130 | pub async fn get(&self, unit_id: &str) -> crate::core::SDKResult<GetUnitResponse... |
| 45 | ✅ 建立部门与单位的绑定关系 | POST | `/open-apis/contact/v3/unit/bind_department` | `src/service/contact/v3/unit.rs` | 62 | pub async fn bind_department( |
| 46 | ✅ 解除部门与单位的绑定关系 | POST | `/open-apis/contact/v3/unit/unbind_department` | `src/service/contact/v3/unit.rs` | 83 | pub async fn unbind_department( |
| 47 | ✅ 获取单位绑定的部门列表 | GET | `/open-apis/contact/v3/unit/list_department` | `src/service/contact/v3/unit.rs` | 104 | pub async fn list_department( |
| 48 | ✅ 获取单位信息 | GET | `/open-apis/contact/v3/unit/:unit_id` | `src/service/contact/v3/unit.rs` | 130 | pub async fn get(&self, unit_id: &str) -> crate::core::SDKResult<GetUnitResponse... |
| 49 | ✅ 获取单位列表 | GET | `/open-apis/contact/v3/unit` | `src/service/contact/v3/unit.rs` | 130 | pub async fn get(&self, unit_id: &str) -> crate::core::SDKResult<GetUnitResponse... |
| 50 | ✅ 删除单位 | DELETE | `/open-apis/contact/v3/unit/:unit_id` | `src/service/contact/v3/unit.rs` | 130 | pub async fn get(&self, unit_id: &str) -> crate::core::SDKResult<GetUnitResponse... |
| 51 | ✅ 添加用户组成员 | POST | `/open-apis/contact/v3/group/:group_id/member/add` | `src/service/contact/v3/group_member.rs` | 26 | pub async fn add( |
| 52 | ✅ 批量添加用户组成员 | POST | `/open-apis/contact/v3/group/:group_id/member/batch_add` | `src/service/contact/v3/group_member.rs` | 44 | pub async fn batch_add( |
| 53 | ✅ 查询用户组成员列表 | GET | `/open-apis/contact/v3/group/:group_id/member/simplelist` | `src/service/contact/v3/group_member.rs` | 62 | pub async fn simplelist( |
| 54 | ✅ 移除用户组成员 | POST | `/open-apis/contact/v3/group/:group_id/member/remove` | `src/service/contact/v3/group_member.rs` | 84 | pub async fn remove( |
| 55 | ✅ 批量移除用户组成员 | POST | `/open-apis/contact/v3/group/:group_id/member/batch_remove` | `src/service/contact/v3/group_member.rs` | 105 | pub async fn batch_remove( |
| 56 | ❌ 创建角色 | POST | `/open-apis/contact/v3/functional_roles` | `未找到` | - | - |
| 57 | ✅ 修改角色名称 | PUT | `/open-apis/contact/v3/functional_roles/:role_id` | `src/service/contact/v3/functional_role.rs` | 62 | pub async fn get(&self, role_id: &str) -> crate::core::SDKResult<GetFunctionalRo... |
| 58 | ✅ 删除角色 | DELETE | `/open-apis/contact/v3/functional_roles/:role_id` | `src/service/contact/v3/functional_role.rs` | 62 | pub async fn get(&self, role_id: &str) -> crate::core::SDKResult<GetFunctionalRo... |
| 59 | ✅ 批量添加角色成员 | POST | `/open-apis/contact/v3/functional_roles/:role_id/members/batch_create` | `src/service/contact/v3/functional_role_member.rs` | 42 | pub async fn batch_create( |
| 60 | ✅ 批量设置角色成员管理范围 | PATCH | `/open-apis/contact/v3/functional_roles/:role_id/members/scopes` | `src/service/contact/v3/functional_role_member.rs` | 63 | pub async fn scopes( |
| 61 | ❌ 查询角色下某个成员的管理范围 | GET | `/open-apis/contact/v3/functional_roles/:role_id/members/:member_id` | `未找到` | - | - |
| 62 | ✅ 查询角色下的所有成员信息 | GET | `/open-apis/contact/v3/functional_roles/:role_id/members` | `src/service/task/v2/tasklist/mod.rs` | 208 | pub async fn add_members( |
| 63 | ✅ 删除角色下的成员 | PATCH | `/open-apis/contact/v3/functional_roles/:role_id/members/batch_delete` | `src/service/contact/v3/functional_role_member.rs` | 131 | pub async fn batch_delete( |
| 64 | ✅ 创建职级 | POST | `/open-apis/contact/v3/job_levels` | `src/service/corehr/job_management/mod.rs` | 286 | pub async fn list_job_levels( |
| 65 | ✅ 更新职级 | PUT | `/open-apis/contact/v3/job_levels/:job_level_id` | `src/service/contact/v3/job_level.rs` | 65 | pub async fn get(&self, job_level_id: &str) -> crate::core::SDKResult<GetJobLeve... |
| 66 | ✅ 获取单个职级信息 | GET | `/open-apis/contact/v3/job_levels/:job_level_id` | `src/service/contact/v3/job_level.rs` | 65 | pub async fn get(&self, job_level_id: &str) -> crate::core::SDKResult<GetJobLeve... |
| 67 | ✅ 获取租户职级列表 | GET | `/open-apis/contact/v3/job_levels` | `src/service/corehr/job_management/mod.rs` | 286 | pub async fn list_job_levels( |
| 68 | ✅ 删除职级 | DELETE | `/open-apis/contact/v3/job_levels/:job_level_id` | `src/service/contact/v3/job_level.rs` | 65 | pub async fn get(&self, job_level_id: &str) -> crate::core::SDKResult<GetJobLeve... |
| 69 | ✅ 创建序列 | POST | `/open-apis/contact/v3/job_families` | `src/service/corehr/job_management/mod.rs` | 190 | pub async fn list_job_families( |
| 70 | ✅ 更新序列 | PUT | `/open-apis/contact/v3/job_families/:job_family_id` | `src/service/contact/v3/job_family.rs` | 66 | pub async fn get(&self, job_family_id: &str) -> crate::core::SDKResult<GetJobFam... |
| 71 | ✅ 获取单个序列信息 | GET | `/open-apis/contact/v3/job_families/:job_family_id` | `src/service/contact/v3/job_family.rs` | 66 | pub async fn get(&self, job_family_id: &str) -> crate::core::SDKResult<GetJobFam... |
| 72 | ✅ 获取租户序列列表 | GET | `/open-apis/contact/v3/job_families` | `src/service/corehr/job_management/mod.rs` | 190 | pub async fn list_job_families( |
| 73 | ✅ 删除序列 | DELETE | `/open-apis/contact/v3/job_families/:job_family_id` | `src/service/contact/v3/job_family.rs` | 66 | pub async fn get(&self, job_family_id: &str) -> crate::core::SDKResult<GetJobFam... |
| 74 | ✅ 获取单个职务信息 | GET | `/open-apis/contact/v3/job_titles/:job_title_id` | `src/service/contact/v3/job_title.rs` | 28 | pub async fn get(&self, job_title_id: &str) -> crate::core::SDKResult<GetJobTitl... |
| 75 | ❌ 获取租户职务列表 | GET | `/open-apis/contact/v3/job_titles` | `未找到` | - | - |
| 76 | ✅ 获取单个工作城市信息 | GET | `/open-apis/contact/v3/work_cities/:work_city_id` | `src/service/contact/v3/work_city.rs` | 28 | pub async fn get(&self, work_city_id: &str) -> crate::core::SDKResult<GetWorkCit... |
| 77 | ❌ 获取租户工作城市列表 | GET | `/open-apis/contact/v3/work_cities` | `未找到` | - | - |
| 78 | ✅ 创建员工 | POST | `/open-apis/directory/v1/employees` | `src/service/ehr/v1/mod.rs` | 263 | pub async fn query_employees( |
| 79 | ✅ 更新员工信息 | PATCH | `/open-apis/directory/v1/employees/:employee_id` | `src/service/ehr/v1/mod.rs` | 165 | pub async fn get_employee(&self, _employee_id: &str) -> SDKResult<EmployeeRespon... |
| 80 | ✅ 离职员工 | DELETE | `/open-apis/directory/v1/employees/:employee_id` | `src/service/ehr/v1/mod.rs` | 165 | pub async fn get_employee(&self, _employee_id: &str) -> SDKResult<EmployeeRespon... |
| 81 | ❌ 恢复离职员工 | POST | `/open-apis/directory/v1/employees/:employee_id/resurrect` | `未找到` | - | - |
| 82 | ❌ 更新在职员工为待离职 | PATCH | `/open-apis/directory/v1/employees/:employee_id/to_be_resigned` | `未找到` | - | - |
| 83 | ❌ 更新待离职成员为在职 | PATCH | `/open-apis/directory/v1/employees/:employee_id/regular` | `未找到` | - | - |
| 84 | ❌ 批量获取员工信息 | POST | `/open-apis/directory/v1/employees/mget` | `未找到` | - | - |
| 85 | ❌ 批量获取员工列表 | POST | `/open-apis/directory/v1/employees/filter` | `未找到` | - | - |
| 86 | ✅ 搜索员工信息 | POST | `/open-apis/directory/v1/employees/search` | `src/service/attendance/v1/group.rs` | 154 | pub async fn search( |
| 87 | ✅ 创建部门 | POST | `/open-apis/directory/v1/departments` | `src/service/corehr/organization/mod.rs` | 161 | pub async fn batch_get_departments( |
| 88 | ✅ 更新部门 | PATCH | `/open-apis/directory/v1/departments/:department_id` | `src/service/contact/v3/department.rs` | 156 | pub async fn update_department_id( |
| 89 | ✅ 删除部门 | DELETE | `/open-apis/directory/v1/departments/:department_id` | `src/service/contact/v3/department.rs` | 156 | pub async fn update_department_id( |
| 90 | ❌ 批量获取部门信息 | POST | `/open-apis/directory/v1/departments/mget` | `未找到` | - | - |
| 91 | ❌ 获取部门列表 | POST | `/open-apis/directory/v1/departments/filter` | `未找到` | - | - |
| 92 | ✅ 搜索部门 | POST | `/open-apis/directory/v1/departments/search` | `src/service/attendance/v1/group.rs` | 154 | pub async fn search( |
| 93 | ✅ 发送消息 | POST | `/open-apis/im/v1/messages` | `src/service/aily/message/mod.rs` | 127 | pub async fn list_messages(, |
| 94 | ✅ 回复消息 | POST | `/open-apis/im/v1/messages/:message_id/reply` | `src/service/cloud_docs/comments/mod.rs` | 171 | pub async fn update_reply( |
| 95 | ❌ 编辑消息 | PUT | `/open-apis/im/v1/messages/:message_id` | `未找到` | - | - |
| 96 | ❌ 转发消息 | POST | `/open-apis/im/v1/messages/:message_id/forward` | `未找到` | - | - |
| 97 | ❌ 合并转发消息 | POST | `/open-apis/im/v1/messages/merge_forward` | `未找到` | - | - |
| 98 | ❌ 转发话题 | POST | `/open-apis/im/v1/threads/:thread_id/forward` | `未找到` | - | - |
| 99 | ❌ 撤回消息 | DELETE | `/open-apis/im/v1/messages/:message_id` | `未找到` | - | - |
| 100 | ❌ 添加跟随气泡 | POST | `/open-apis/im/v1/messages/:message_id/push_follow_up` | `未找到` | - | - |


## 统计信息

### 按服务分类的实现数量

- **unknown**: 67 个API

### 未实现的API (33个)

- 获取事件出口 IP (GET /open-apis/event/v1/outbound_ip)
- 退出登录 (POST /open-apis/passport/v1/sessions/logout)
- 修改用户部分信息 (PATCH /open-apis/contact/v3/users/:user_id)
- 更新用户 ID (PATCH /open-apis/contact/v3/users/:user_id/update_user_id)
- 获取单个用户信息 (GET /open-apis/contact/v3/users/:user_id)
- 获取部门直属用户列表 (GET /open-apis/contact/v3/users/find_by_department)
- 通过手机号或邮箱获取用户 ID (POST /open-apis/contact/v3/users/batch_get_id)
- 删除用户 (DELETE /open-apis/contact/v3/users/:user_id)
- 恢复已删除用户 (POST /open-apis/contact/v3/users/:user_id/resurrect)
- 查询用户所属用户组 (GET /open-apis/contact/v3/group/member_belong)
- ... 还有 23 个未实现的API
