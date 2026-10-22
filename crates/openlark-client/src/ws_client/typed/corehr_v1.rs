//! corehr 域 v1 typed 事件（v1 事件串，与 v2 两套独立体系，见 [`super::corehr_v2`]）。
//!
//! 字段权威：Go SDK v3.12.2 `service/corehr/v1/model.go` 对应 `P2*` 结构的 json tag；
//! 事件清单对齐 `service/corehr/v1/dispatcher.go`。Go 指针字段映射为 `Option<T>`，
//! serde 采用 `#[serde(default)]` 宽松风格，与 [`super::ImMessageReceiveV1`] 一致。

use serde::{Deserialize, Serialize};

use super::EventHeader;

/// `corehr.common_data.id.user_mapping_changed_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CommonDataIdUserMappingChangedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: CommonDataIdUserMappingChangedV1Event,
}

/// `corehr.common_data.id.user_mapping_changed_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CommonDataIdUserMappingChangedV1Event {
    /// 变更类型。insert：创建用户ID映射，delete：删除用户ID映射。
    #[serde(default)]
    pub change_type: Option<String>,
    /// ID转换类型。1:飞书人事<->飞书，2:飞书人事<->PeopleAdmin，3:PeopleAdmin<->飞书。
    #[serde(default)]
    pub id_transform_type: Option<i64>,
    /// 飞书人事ID，仅id_transform_type等于1、2时有值。
    #[serde(default)]
    pub corehr_id: Option<String>,
    /// PeopleAdmin ID，仅id_transform_type等于2、3时有值。
    #[serde(default)]
    pub people_admin_id: Option<String>,
    /// 飞书用户ID，仅id_transform_type等于1、3时有值。
    #[serde(default)]
    pub feishu_id: Option<UserId>,
}

/// `corehr.common_data.meta_data.updated_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CommonDataMetaDataUpdatedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: CommonDataMetaDataUpdatedV1Event,
}

/// `corehr.common_data.meta_data.updated_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CommonDataMetaDataUpdatedV1Event {
    /// 发生变更的对象 API Name，可通过[获取字段详情](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/custom_field/get_by_param)查询字段详情
    #[serde(default)]
    pub api_name: Option<String>,
    /// 变更的字段
    #[serde(default)]
    pub field_changes: Vec<String>,
    /// 变更的元数据类型（包括对象和枚举）
    #[serde(default)]
    pub metadata_type: Option<String>,
    /// 变更的枚举
    #[serde(default)]
    pub enum_value_changes: Vec<String>,
}

/// `corehr.contract.created_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ContractCreatedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: ContractCreatedV1Event,
}

/// `corehr.contract.created_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ContractCreatedV1Event {
    /// 合同ID，该合同详细信息可通过[【合同信息查询接口】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/contract/get)获取
    #[serde(default)]
    pub contract_id: Option<String>,
}

/// `corehr.contract.deleted_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ContractDeletedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: ContractDeletedV1Event,
}

/// `corehr.contract.deleted_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ContractDeletedV1Event {
    /// 合同ID，删除后无法通过接口查询到数据
    #[serde(default)]
    pub contract_id: Option<String>,
}

/// `corehr.contract.updated_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ContractUpdatedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: ContractUpdatedV1Event,
}

/// `corehr.contract.updated_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ContractUpdatedV1Event {
    /// 合同ID，该合同详细信息可通过[【合同信息查询接口】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/contract/get)获取
    #[serde(default)]
    pub contract_id: Option<String>,
}

/// `corehr.department.created_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DepartmentCreatedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: DepartmentCreatedV1Event,
}

/// `corehr.department.created_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DepartmentCreatedV1Event {
    /// 新建部门的 ID
    #[serde(default)]
    pub department_id: Option<String>,
}

/// `corehr.department.deleted_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DepartmentDeletedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: DepartmentDeletedV1Event,
}

/// `corehr.department.deleted_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DepartmentDeletedV1Event {
    /// 被删除部门的 ID
    #[serde(default)]
    pub department_id: Option<String>,
    /// 部门编码
    #[serde(default)]
    pub code: Option<String>,
}

/// `corehr.department.updated_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DepartmentUpdatedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: DepartmentUpdatedV1Event,
}

/// `corehr.department.updated_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DepartmentUpdatedV1Event {
    /// 被更新部门的 ID
    #[serde(default)]
    pub department_id: Option<String>,
    /// 发生变更的字段
    #[serde(default)]
    pub field_changes: Vec<String>,
}

/// `corehr.employment.converted_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmploymentConvertedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: EmploymentConvertedV1Event,
}

/// `corehr.employment.converted_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmploymentConvertedV1Event {
    /// 雇佣ID，可通过[【搜索员工信息】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/employee/search)接口获取
    #[serde(default)]
    pub employment_id: Option<String>,
}

/// `corehr.employment.created_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmploymentCreatedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: EmploymentCreatedV1Event,
}

/// `corehr.employment.created_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmploymentCreatedV1Event {
    /// 被创建的雇佣信息的 ID
    #[serde(default)]
    pub employment_id: Option<String>,
    /// 员工的飞书用户 ID
    #[serde(default)]
    pub target_user_id: Option<UserId>,
}

/// `corehr.employment.deleted_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmploymentDeletedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: EmploymentDeletedV1Event,
}

/// `corehr.employment.deleted_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmploymentDeletedV1Event {
    /// 被删除的雇佣信息的 ID
    #[serde(default)]
    pub employment_id: Option<String>,
    /// 员工的飞书用户 ID
    #[serde(default)]
    pub target_user_id: Option<UserId>,
}

/// `corehr.employment.resigned_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmploymentResignedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: EmploymentResignedV1Event,
}

/// `corehr.employment.resigned_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmploymentResignedV1Event {
    /// 离职员工雇佣ID，可通过[批量查询员工信息;](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/employee/batch_get)获取员工信息。
    #[serde(default)]
    pub employment_id: Option<String>,
}

/// `corehr.employment.updated_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmploymentUpdatedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: EmploymentUpdatedV1Event,
}

/// `corehr.employment.updated_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmploymentUpdatedV1Event {
    /// 被更新的雇佣信息 ID
    #[serde(default)]
    pub employment_id: Option<String>,
    /// 员工的飞书用户 ID
    #[serde(default)]
    pub target_user_id: Option<UserId>,
    /// 发生变更的字段
    #[serde(default)]
    pub field_changes: Vec<String>,
}

/// `corehr.job.created_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JobCreatedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: JobCreatedV1Event,
}

/// `corehr.job.created_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JobCreatedV1Event {
    /// 职务 ID，可通过[【查询单个职务】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/job/get)获取详细信息
    #[serde(default)]
    pub job_id: Option<String>,
}

/// `corehr.job.deleted_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JobDeletedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: JobDeletedV1Event,
}

/// `corehr.job.deleted_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JobDeletedV1Event {
    /// 职务 ID
    #[serde(default)]
    pub job_id: Option<String>,
}

/// `corehr.job.updated_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JobUpdatedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: JobUpdatedV1Event,
}

/// `corehr.job.updated_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JobUpdatedV1Event {
    /// 职务 ID，可通过[【查询单个职务】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/job/get)获取详细信息
    #[serde(default)]
    pub job_id: Option<String>,
}

/// `corehr.job_change.updated_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JobChangeUpdatedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: JobChangeUpdatedV1Event,
}

/// `corehr.job_change.updated_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JobChangeUpdatedV1Event {
    /// 雇员ID，可通过[【搜索员工信息】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/employee/search)接口获取
    #[serde(default)]
    pub employment_id: Option<String>,
    /// 用户 ID，飞书相关ID，;可通过[【通讯录接口】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/user/field-overview)接口获取
    #[serde(default)]
    pub target_user_id: Option<UserId>,
    /// 异动记录 id，可通过接口;[搜索异动信息;](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/job_change/search)获取详细信息
    #[serde(default)]
    pub job_change_id: Option<String>,
    /// 异动属性/方式
    #[serde(default)]
    pub transfer_mode: Option<i64>,
    /// 异动类型唯一标识，可通过接口;[获取异动类型列表;](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/transfer_type/query)获取
    #[serde(default)]
    pub transfer_type_unique_identifier: Option<String>,
    /// 异动原因唯一标识，可通过接口;[获取异动原因列表;](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/transfer_reason/query)获取详细信息
    #[serde(default)]
    pub transfer_reason_unique_identifier: Option<String>,
    /// 异动发起后的审批流程 id，如果是直接异动，则无需要审批流程id
    #[serde(default)]
    pub process_id: Option<String>,
    /// 异动生效日期，格式："YYYY-MM-DD";- 示例：2022-04-03
    #[serde(default)]
    pub effective_date: Option<String>,
    /// 异动状态
    #[serde(default)]
    pub status: Option<i64>,
    /// 异动记录标识符
    #[serde(default)]
    pub transfer_key: Option<String>,
}

/// `corehr.job_data.changed_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JobDataChangedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: JobDataChangedV1Event,
}

/// `corehr.job_data.changed_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JobDataChangedV1Event {
    /// 任职信息 ID，可通过[【搜索员工信息】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/employee/search)接口获取
    #[serde(default)]
    pub job_data_id: Option<String>,
    /// 雇佣信息 ID，可通过[【搜索员工信息】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/employee/search)接口获取
    #[serde(default)]
    pub employment_id: Option<String>,
    /// 用户 ID，飞书相关ID，;可通过[【通讯录接口】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/user/field-overview)接口获取
    #[serde(default)]
    pub target_user_id: Option<UserId>,
    /// 员工异动 ID，可通过[【搜索异动信息】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/job_change/search)接口获取
    #[serde(default)]
    pub job_change_id: Option<String>,
}

/// `corehr.job_data.created_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JobDataCreatedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: JobDataCreatedV1Event,
}

/// `corehr.job_data.created_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JobDataCreatedV1Event {
    /// ID
    #[serde(default)]
    pub job_data_id: Option<String>,
}

/// `corehr.job_data.deleted_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JobDataDeletedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: JobDataDeletedV1Event,
}

/// `corehr.job_data.deleted_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JobDataDeletedV1Event {
    /// ID
    #[serde(default)]
    pub job_data_id: Option<String>,
}

/// `corehr.job_data.employed_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JobDataEmployedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: JobDataEmployedV1Event,
}

/// `corehr.job_data.employed_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JobDataEmployedV1Event {
    /// 任职记录 ID，详细信息可通过[【获取任职记录】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/job_data/get)接口查询
    #[serde(default)]
    pub job_data_id: Option<String>,
    /// 雇佣记录 ID，详细信息可通过[【查询员工信息】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/employee/batch_get)接口查询
    #[serde(default)]
    pub employment_id: Option<String>,
    /// 用户 ID（备注：入职后开通飞书账号成功，会生成飞书通讯录相关ID，但由于开通过程是异步，本事件不保证一定返回，如未返回，说明飞书账号开通失败，业务需兼容处理）
    #[serde(default)]
    pub target_user_id: Option<UserId>,
}

/// `corehr.job_data.updated_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JobDataUpdatedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: JobDataUpdatedV1Event,
}

/// `corehr.job_data.updated_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JobDataUpdatedV1Event {
    /// ID
    #[serde(default)]
    pub job_data_id: Option<String>,
}

/// `corehr.offboarding.updated_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OffboardingUpdatedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: OffboardingUpdatedV1Event,
}

/// `corehr.offboarding.updated_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OffboardingUpdatedV1Event {
    /// 离职员工雇佣ID，可通过[批量查询员工信息;](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/employee/batch_get)获取员工信息。
    #[serde(default)]
    pub employment_id: Option<String>,
    /// 离职员工的用户 ID
    #[serde(default)]
    pub target_user_id: Option<UserId>,
    /// 离职记录ID，每条离职记录的唯一标识。
    #[serde(default)]
    pub offboarding_id: Option<String>,
    /// 离职申请流程的实例 ID，可用于[查询流程相关信息](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/process/list)，例如：作为[获取单个流程详情](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/process/list)的process_id查询流程详情。
    #[serde(default)]
    pub process_id: Option<String>,
    /// 离职状态
    #[serde(default)]
    pub status: Option<i64>,
}

/// `corehr.org_role_authorization.updated_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OrgRoleAuthorizationUpdatedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: OrgRoleAuthorizationUpdatedV1Event,
}

/// `corehr.org_role_authorization.updated_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OrgRoleAuthorizationUpdatedV1Event {
    /// 角色id
    #[serde(default)]
    pub role_id: Option<String>,
    /// 管理范围信息
    #[serde(default)]
    pub management_scope_list: Vec<ManagementScope>,
    /// 员工雇佣信息ID list
    #[serde(default)]
    pub employment_id_list: Vec<String>,
}

/// `corehr.person.created_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PersonCreatedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: PersonCreatedV1Event,
}

/// `corehr.person.created_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PersonCreatedV1Event {
    /// 人员ID
    #[serde(default)]
    pub person_id: Option<String>,
}

/// `corehr.person.deleted_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PersonDeletedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: PersonDeletedV1Event,
}

/// `corehr.person.deleted_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PersonDeletedV1Event {
    /// 人员ID
    #[serde(default)]
    pub person_id: Option<String>,
}

/// `corehr.person.updated_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PersonUpdatedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: PersonUpdatedV1Event,
}

/// `corehr.person.updated_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PersonUpdatedV1Event {
    /// 被更新个人信息的 ID
    #[serde(default)]
    pub person_id: Option<String>,
    /// 发生变更的字段，若部分字段变更无需接受事件消息，可根据该字段进行事件消息过滤
    #[serde(default)]
    pub field_changes: Vec<String>,
}

/// `corehr.pre_hire.updated_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PreHireUpdatedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: PreHireUpdatedV1Event,
}

/// `corehr.pre_hire.updated_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PreHireUpdatedV1Event {
    /// 待入职 ID，可通过[【搜索待入职人员】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/pre_hire/search)获取更详细信息
    #[serde(default)]
    pub pre_hire_id: Option<String>,
    /// 变更的字段。;由于历史原因，部分字段（例如个人信息）变更会发送为‘wk_updated_at’字段，不会显示真实变更字段，需要通过[【搜索待入职人员】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/pre_hire/search)获取更详细信息。
    #[serde(default)]
    pub field_changes: Vec<String>,
}

/// 事件嵌套类型（Go `UserId`）。
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UserId {
    /// 对应 Go json tag `user_id`。
    #[serde(default)]
    pub user_id: Option<String>,
    /// 对应 Go json tag `open_id`。
    #[serde(default)]
    pub open_id: Option<String>,
    /// 对应 Go json tag `union_id`。
    #[serde(default)]
    pub union_id: Option<String>,
}

/// 事件嵌套类型（Go `ManagementScope`）。
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ManagementScope {
    /// 管理维度;;可选值有：;;- department：部门;- work_location：工作地点;- company：公司;- social_security_location：社保城市
    #[serde(default)]
    pub management_dimension: Option<String>,
    /// 被授权部门/工作地点/公司/社保城市 ID
    #[serde(default)]
    pub obj_id: Option<String>,
}

typed_events! {
    /// `corehr.common_data.id.user_mapping_changed_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.common_data.id.user_mapping_changed_v1" | COREHR_COMMON_DATA_ID_USER_MAPPING_CHANGED_V1 | CommonDataIdUserMappingChangedV1 | CommonDataIdUserMappingChangedV1Handler | register_corehr_common_data_id_user_mapping_changed_v1;
    /// `corehr.common_data.meta_data.updated_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.common_data.meta_data.updated_v1" | COREHR_COMMON_DATA_META_DATA_UPDATED_V1 | CommonDataMetaDataUpdatedV1 | CommonDataMetaDataUpdatedV1Handler | register_corehr_common_data_meta_data_updated_v1;
    /// `corehr.contract.created_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.contract.created_v1" | COREHR_CONTRACT_CREATED_V1 | ContractCreatedV1 | ContractCreatedV1Handler | register_corehr_contract_created_v1;
    /// `corehr.contract.deleted_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.contract.deleted_v1" | COREHR_CONTRACT_DELETED_V1 | ContractDeletedV1 | ContractDeletedV1Handler | register_corehr_contract_deleted_v1;
    /// `corehr.contract.updated_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.contract.updated_v1" | COREHR_CONTRACT_UPDATED_V1 | ContractUpdatedV1 | ContractUpdatedV1Handler | register_corehr_contract_updated_v1;
    /// `corehr.department.created_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.department.created_v1" | COREHR_DEPARTMENT_CREATED_V1 | DepartmentCreatedV1 | DepartmentCreatedV1Handler | register_corehr_department_created_v1;
    /// `corehr.department.deleted_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.department.deleted_v1" | COREHR_DEPARTMENT_DELETED_V1 | DepartmentDeletedV1 | DepartmentDeletedV1Handler | register_corehr_department_deleted_v1;
    /// `corehr.department.updated_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.department.updated_v1" | COREHR_DEPARTMENT_UPDATED_V1 | DepartmentUpdatedV1 | DepartmentUpdatedV1Handler | register_corehr_department_updated_v1;
    /// `corehr.employment.converted_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.employment.converted_v1" | COREHR_EMPLOYMENT_CONVERTED_V1 | EmploymentConvertedV1 | EmploymentConvertedV1Handler | register_corehr_employment_converted_v1;
    /// `corehr.employment.created_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.employment.created_v1" | COREHR_EMPLOYMENT_CREATED_V1 | EmploymentCreatedV1 | EmploymentCreatedV1Handler | register_corehr_employment_created_v1;
    /// `corehr.employment.deleted_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.employment.deleted_v1" | COREHR_EMPLOYMENT_DELETED_V1 | EmploymentDeletedV1 | EmploymentDeletedV1Handler | register_corehr_employment_deleted_v1;
    /// `corehr.employment.resigned_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.employment.resigned_v1" | COREHR_EMPLOYMENT_RESIGNED_V1 | EmploymentResignedV1 | EmploymentResignedV1Handler | register_corehr_employment_resigned_v1;
    /// `corehr.employment.updated_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.employment.updated_v1" | COREHR_EMPLOYMENT_UPDATED_V1 | EmploymentUpdatedV1 | EmploymentUpdatedV1Handler | register_corehr_employment_updated_v1;
    /// `corehr.job.created_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.job.created_v1" | COREHR_JOB_CREATED_V1 | JobCreatedV1 | JobCreatedV1Handler | register_corehr_job_created_v1;
    /// `corehr.job.deleted_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.job.deleted_v1" | COREHR_JOB_DELETED_V1 | JobDeletedV1 | JobDeletedV1Handler | register_corehr_job_deleted_v1;
    /// `corehr.job.updated_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.job.updated_v1" | COREHR_JOB_UPDATED_V1 | JobUpdatedV1 | JobUpdatedV1Handler | register_corehr_job_updated_v1;
    /// `corehr.job_change.updated_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.job_change.updated_v1" | COREHR_JOB_CHANGE_UPDATED_V1 | JobChangeUpdatedV1 | JobChangeUpdatedV1Handler | register_corehr_job_change_updated_v1;
    /// `corehr.job_data.changed_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.job_data.changed_v1" | COREHR_JOB_DATA_CHANGED_V1 | JobDataChangedV1 | JobDataChangedV1Handler | register_corehr_job_data_changed_v1;
    /// `corehr.job_data.created_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.job_data.created_v1" | COREHR_JOB_DATA_CREATED_V1 | JobDataCreatedV1 | JobDataCreatedV1Handler | register_corehr_job_data_created_v1;
    /// `corehr.job_data.deleted_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.job_data.deleted_v1" | COREHR_JOB_DATA_DELETED_V1 | JobDataDeletedV1 | JobDataDeletedV1Handler | register_corehr_job_data_deleted_v1;
    /// `corehr.job_data.employed_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.job_data.employed_v1" | COREHR_JOB_DATA_EMPLOYED_V1 | JobDataEmployedV1 | JobDataEmployedV1Handler | register_corehr_job_data_employed_v1;
    /// `corehr.job_data.updated_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.job_data.updated_v1" | COREHR_JOB_DATA_UPDATED_V1 | JobDataUpdatedV1 | JobDataUpdatedV1Handler | register_corehr_job_data_updated_v1;
    /// `corehr.offboarding.updated_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.offboarding.updated_v1" | COREHR_OFFBOARDING_UPDATED_V1 | OffboardingUpdatedV1 | OffboardingUpdatedV1Handler | register_corehr_offboarding_updated_v1;
    /// `corehr.org_role_authorization.updated_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.org_role_authorization.updated_v1" | COREHR_ORG_ROLE_AUTHORIZATION_UPDATED_V1 | OrgRoleAuthorizationUpdatedV1 | OrgRoleAuthorizationUpdatedV1Handler | register_corehr_org_role_authorization_updated_v1;
    /// `corehr.person.created_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.person.created_v1" | COREHR_PERSON_CREATED_V1 | PersonCreatedV1 | PersonCreatedV1Handler | register_corehr_person_created_v1;
    /// `corehr.person.deleted_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.person.deleted_v1" | COREHR_PERSON_DELETED_V1 | PersonDeletedV1 | PersonDeletedV1Handler | register_corehr_person_deleted_v1;
    /// `corehr.person.updated_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.person.updated_v1" | COREHR_PERSON_UPDATED_V1 | PersonUpdatedV1 | PersonUpdatedV1Handler | register_corehr_person_updated_v1;
    /// `corehr.pre_hire.updated_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.pre_hire.updated_v1" | COREHR_PRE_HIRE_UPDATED_V1 | PreHireUpdatedV1 | PreHireUpdatedV1Handler | register_corehr_pre_hire_updated_v1;
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use super::*;
    use crate::ws_client::EventDispatcherHandler;

    const COREHR_COMMON_DATA_ID_USER_MAPPING_CHANGED_V1_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "corehr.common_data.id.user_mapping_changed_v1",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "change_type": "change_type_str",
    "id_transform_type": 42,
    "corehr_id": "corehr_id_str",
    "people_admin_id": "people_admin_id_str",
    "feishu_id": {
      "user_id": "user_id_str",
      "open_id": "open_id_str",
      "union_id": "union_id_str"
    }
  }
}"#;

    const COREHR_COMMON_DATA_META_DATA_UPDATED_V1_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "corehr.common_data.meta_data.updated_v1",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "api_name": "api_name_str",
    "field_changes": [
      "field_changes_str"
    ],
    "metadata_type": "metadata_type_str",
    "enum_value_changes": [
      "enum_value_changes_str"
    ]
  }
}"#;

    const COREHR_CONTRACT_CREATED_V1_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "corehr.contract.created_v1",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "contract_id": "contract_id_str"
  }
}"#;

    const COREHR_CONTRACT_DELETED_V1_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "corehr.contract.deleted_v1",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "contract_id": "contract_id_str"
  }
}"#;

    const COREHR_CONTRACT_UPDATED_V1_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "corehr.contract.updated_v1",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "contract_id": "contract_id_str"
  }
}"#;

    const COREHR_DEPARTMENT_CREATED_V1_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "corehr.department.created_v1",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "department_id": "department_id_str"
  }
}"#;

    const COREHR_DEPARTMENT_DELETED_V1_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "corehr.department.deleted_v1",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "department_id": "department_id_str",
    "code": "code_str"
  }
}"#;

    const COREHR_DEPARTMENT_UPDATED_V1_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "corehr.department.updated_v1",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "department_id": "department_id_str",
    "field_changes": [
      "field_changes_str"
    ]
  }
}"#;

    const COREHR_EMPLOYMENT_CONVERTED_V1_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "corehr.employment.converted_v1",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "employment_id": "employment_id_str"
  }
}"#;

    const COREHR_EMPLOYMENT_CREATED_V1_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "corehr.employment.created_v1",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "employment_id": "employment_id_str",
    "target_user_id": {
      "user_id": "user_id_str",
      "open_id": "open_id_str",
      "union_id": "union_id_str"
    }
  }
}"#;

    const COREHR_EMPLOYMENT_DELETED_V1_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "corehr.employment.deleted_v1",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "employment_id": "employment_id_str",
    "target_user_id": {
      "user_id": "user_id_str",
      "open_id": "open_id_str",
      "union_id": "union_id_str"
    }
  }
}"#;

    const COREHR_EMPLOYMENT_RESIGNED_V1_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "corehr.employment.resigned_v1",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "employment_id": "employment_id_str"
  }
}"#;

    const COREHR_EMPLOYMENT_UPDATED_V1_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "corehr.employment.updated_v1",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "employment_id": "employment_id_str",
    "target_user_id": {
      "user_id": "user_id_str",
      "open_id": "open_id_str",
      "union_id": "union_id_str"
    },
    "field_changes": [
      "field_changes_str"
    ]
  }
}"#;

    const COREHR_JOB_CREATED_V1_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "corehr.job.created_v1",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "job_id": "job_id_str"
  }
}"#;

    const COREHR_JOB_DELETED_V1_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "corehr.job.deleted_v1",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "job_id": "job_id_str"
  }
}"#;

    const COREHR_JOB_UPDATED_V1_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "corehr.job.updated_v1",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "job_id": "job_id_str"
  }
}"#;

    const COREHR_JOB_CHANGE_UPDATED_V1_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "corehr.job_change.updated_v1",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "employment_id": "employment_id_str",
    "target_user_id": {
      "user_id": "user_id_str",
      "open_id": "open_id_str",
      "union_id": "union_id_str"
    },
    "job_change_id": "job_change_id_str",
    "transfer_mode": 42,
    "transfer_type_unique_identifier": "transfer_type_unique_identifier_str",
    "transfer_reason_unique_identifier": "transfer_reason_unique_identifier_str",
    "process_id": "process_id_str",
    "effective_date": "effective_date_str",
    "status": 42,
    "transfer_key": "transfer_key_str"
  }
}"#;

    const COREHR_JOB_DATA_CHANGED_V1_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "corehr.job_data.changed_v1",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "job_data_id": "job_data_id_str",
    "employment_id": "employment_id_str",
    "target_user_id": {
      "user_id": "user_id_str",
      "open_id": "open_id_str",
      "union_id": "union_id_str"
    },
    "job_change_id": "job_change_id_str"
  }
}"#;

    const COREHR_JOB_DATA_CREATED_V1_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "corehr.job_data.created_v1",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "job_data_id": "job_data_id_str"
  }
}"#;

    const COREHR_JOB_DATA_DELETED_V1_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "corehr.job_data.deleted_v1",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "job_data_id": "job_data_id_str"
  }
}"#;

    const COREHR_JOB_DATA_EMPLOYED_V1_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "corehr.job_data.employed_v1",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "job_data_id": "job_data_id_str",
    "employment_id": "employment_id_str",
    "target_user_id": {
      "user_id": "user_id_str",
      "open_id": "open_id_str",
      "union_id": "union_id_str"
    }
  }
}"#;

    const COREHR_JOB_DATA_UPDATED_V1_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "corehr.job_data.updated_v1",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "job_data_id": "job_data_id_str"
  }
}"#;

    const COREHR_OFFBOARDING_UPDATED_V1_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "corehr.offboarding.updated_v1",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "employment_id": "employment_id_str",
    "target_user_id": {
      "user_id": "user_id_str",
      "open_id": "open_id_str",
      "union_id": "union_id_str"
    },
    "offboarding_id": "offboarding_id_str",
    "process_id": "process_id_str",
    "status": 42
  }
}"#;

    const COREHR_ORG_ROLE_AUTHORIZATION_UPDATED_V1_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "corehr.org_role_authorization.updated_v1",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "role_id": "role_id_str",
    "management_scope_list": [
      {
        "management_dimension": "management_dimension_str",
        "obj_id": "obj_id_str"
      }
    ],
    "employment_id_list": [
      "employment_id_list_str"
    ]
  }
}"#;

    const COREHR_PERSON_CREATED_V1_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "corehr.person.created_v1",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "person_id": "person_id_str"
  }
}"#;

    const COREHR_PERSON_DELETED_V1_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "corehr.person.deleted_v1",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "person_id": "person_id_str"
  }
}"#;

    const COREHR_PERSON_UPDATED_V1_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "corehr.person.updated_v1",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "person_id": "person_id_str",
    "field_changes": [
      "field_changes_str"
    ]
  }
}"#;

    const COREHR_PRE_HIRE_UPDATED_V1_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "corehr.pre_hire.updated_v1",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "pre_hire_id": "pre_hire_id_str",
    "field_changes": [
      "field_changes_str"
    ]
  }
}"#;

    #[test]
    fn typed_corehr_common_data_id_user_mapping_changed_v1_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_corehr_common_data_id_user_mapping_changed_v1(
                move |event: CommonDataIdUserMappingChangedV1| {
                    assert_eq!(
                        event.header.event_type,
                        "corehr.common_data.id.user_mapping_changed_v1"
                    );
                    assert_eq!(event.event.change_type.as_deref(), Some("change_type_str"));
                    assert_eq!(event.event.id_transform_type, Some(42));
                    assert_eq!(event.event.corehr_id.as_deref(), Some("corehr_id_str"));
                    let feishu_id = event.event.feishu_id.as_ref().expect("feishu_id");
                    assert_eq!(feishu_id.user_id.as_deref(), Some("user_id_str"));
                    *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                    Ok(())
                },
            )
            .expect("register")
            .build();
        handler
            .do_without_validation(COREHR_COMMON_DATA_ID_USER_MAPPING_CHANGED_V1_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_corehr_common_data_meta_data_updated_v1_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_corehr_common_data_meta_data_updated_v1(
                move |event: CommonDataMetaDataUpdatedV1| {
                    assert_eq!(
                        event.header.event_type,
                        "corehr.common_data.meta_data.updated_v1"
                    );
                    assert_eq!(event.event.api_name.as_deref(), Some("api_name_str"));
                    assert_eq!(event.event.field_changes[0].as_str(), "field_changes_str");
                    assert_eq!(
                        event.event.metadata_type.as_deref(),
                        Some("metadata_type_str")
                    );
                    *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                    Ok(())
                },
            )
            .expect("register")
            .build();
        handler
            .do_without_validation(COREHR_COMMON_DATA_META_DATA_UPDATED_V1_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_corehr_contract_created_v1_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_corehr_contract_created_v1(move |event: ContractCreatedV1| {
                assert_eq!(event.header.event_type, "corehr.contract.created_v1");
                assert_eq!(event.event.contract_id.as_deref(), Some("contract_id_str"));
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(COREHR_CONTRACT_CREATED_V1_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_corehr_contract_deleted_v1_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_corehr_contract_deleted_v1(move |event: ContractDeletedV1| {
                assert_eq!(event.header.event_type, "corehr.contract.deleted_v1");
                assert_eq!(event.event.contract_id.as_deref(), Some("contract_id_str"));
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(COREHR_CONTRACT_DELETED_V1_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_corehr_contract_updated_v1_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_corehr_contract_updated_v1(move |event: ContractUpdatedV1| {
                assert_eq!(event.header.event_type, "corehr.contract.updated_v1");
                assert_eq!(event.event.contract_id.as_deref(), Some("contract_id_str"));
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(COREHR_CONTRACT_UPDATED_V1_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_corehr_department_created_v1_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_corehr_department_created_v1(move |event: DepartmentCreatedV1| {
                assert_eq!(event.header.event_type, "corehr.department.created_v1");
                assert_eq!(
                    event.event.department_id.as_deref(),
                    Some("department_id_str")
                );
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(COREHR_DEPARTMENT_CREATED_V1_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_corehr_department_deleted_v1_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_corehr_department_deleted_v1(move |event: DepartmentDeletedV1| {
                assert_eq!(event.header.event_type, "corehr.department.deleted_v1");
                assert_eq!(
                    event.event.department_id.as_deref(),
                    Some("department_id_str")
                );
                assert_eq!(event.event.code.as_deref(), Some("code_str"));
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(COREHR_DEPARTMENT_DELETED_V1_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_corehr_department_updated_v1_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_corehr_department_updated_v1(move |event: DepartmentUpdatedV1| {
                assert_eq!(event.header.event_type, "corehr.department.updated_v1");
                assert_eq!(
                    event.event.department_id.as_deref(),
                    Some("department_id_str")
                );
                assert_eq!(event.event.field_changes[0].as_str(), "field_changes_str");
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(COREHR_DEPARTMENT_UPDATED_V1_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_corehr_employment_converted_v1_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_corehr_employment_converted_v1(move |event: EmploymentConvertedV1| {
                assert_eq!(event.header.event_type, "corehr.employment.converted_v1");
                assert_eq!(
                    event.event.employment_id.as_deref(),
                    Some("employment_id_str")
                );
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(COREHR_EMPLOYMENT_CONVERTED_V1_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_corehr_employment_created_v1_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_corehr_employment_created_v1(move |event: EmploymentCreatedV1| {
                assert_eq!(event.header.event_type, "corehr.employment.created_v1");
                assert_eq!(
                    event.event.employment_id.as_deref(),
                    Some("employment_id_str")
                );
                let target_user_id = event.event.target_user_id.as_ref().expect("target_user_id");
                assert_eq!(target_user_id.user_id.as_deref(), Some("user_id_str"));
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(COREHR_EMPLOYMENT_CREATED_V1_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_corehr_employment_deleted_v1_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_corehr_employment_deleted_v1(move |event: EmploymentDeletedV1| {
                assert_eq!(event.header.event_type, "corehr.employment.deleted_v1");
                assert_eq!(
                    event.event.employment_id.as_deref(),
                    Some("employment_id_str")
                );
                let target_user_id = event.event.target_user_id.as_ref().expect("target_user_id");
                assert_eq!(target_user_id.user_id.as_deref(), Some("user_id_str"));
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(COREHR_EMPLOYMENT_DELETED_V1_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_corehr_employment_resigned_v1_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_corehr_employment_resigned_v1(move |event: EmploymentResignedV1| {
                assert_eq!(event.header.event_type, "corehr.employment.resigned_v1");
                assert_eq!(
                    event.event.employment_id.as_deref(),
                    Some("employment_id_str")
                );
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(COREHR_EMPLOYMENT_RESIGNED_V1_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_corehr_employment_updated_v1_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_corehr_employment_updated_v1(move |event: EmploymentUpdatedV1| {
                assert_eq!(event.header.event_type, "corehr.employment.updated_v1");
                assert_eq!(
                    event.event.employment_id.as_deref(),
                    Some("employment_id_str")
                );
                let target_user_id = event.event.target_user_id.as_ref().expect("target_user_id");
                assert_eq!(target_user_id.user_id.as_deref(), Some("user_id_str"));
                assert_eq!(event.event.field_changes[0].as_str(), "field_changes_str");
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(COREHR_EMPLOYMENT_UPDATED_V1_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_corehr_job_created_v1_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_corehr_job_created_v1(move |event: JobCreatedV1| {
                assert_eq!(event.header.event_type, "corehr.job.created_v1");
                assert_eq!(event.event.job_id.as_deref(), Some("job_id_str"));
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(COREHR_JOB_CREATED_V1_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_corehr_job_deleted_v1_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_corehr_job_deleted_v1(move |event: JobDeletedV1| {
                assert_eq!(event.header.event_type, "corehr.job.deleted_v1");
                assert_eq!(event.event.job_id.as_deref(), Some("job_id_str"));
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(COREHR_JOB_DELETED_V1_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_corehr_job_updated_v1_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_corehr_job_updated_v1(move |event: JobUpdatedV1| {
                assert_eq!(event.header.event_type, "corehr.job.updated_v1");
                assert_eq!(event.event.job_id.as_deref(), Some("job_id_str"));
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(COREHR_JOB_UPDATED_V1_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_corehr_job_change_updated_v1_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_corehr_job_change_updated_v1(move |event: JobChangeUpdatedV1| {
                assert_eq!(event.header.event_type, "corehr.job_change.updated_v1");
                assert_eq!(
                    event.event.employment_id.as_deref(),
                    Some("employment_id_str")
                );
                let target_user_id = event.event.target_user_id.as_ref().expect("target_user_id");
                assert_eq!(target_user_id.user_id.as_deref(), Some("user_id_str"));
                assert_eq!(
                    event.event.job_change_id.as_deref(),
                    Some("job_change_id_str")
                );
                assert_eq!(event.event.transfer_mode, Some(42));
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(COREHR_JOB_CHANGE_UPDATED_V1_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_corehr_job_data_changed_v1_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_corehr_job_data_changed_v1(move |event: JobDataChangedV1| {
                assert_eq!(event.header.event_type, "corehr.job_data.changed_v1");
                assert_eq!(event.event.job_data_id.as_deref(), Some("job_data_id_str"));
                assert_eq!(
                    event.event.employment_id.as_deref(),
                    Some("employment_id_str")
                );
                let target_user_id = event.event.target_user_id.as_ref().expect("target_user_id");
                assert_eq!(target_user_id.user_id.as_deref(), Some("user_id_str"));
                assert_eq!(
                    event.event.job_change_id.as_deref(),
                    Some("job_change_id_str")
                );
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(COREHR_JOB_DATA_CHANGED_V1_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_corehr_job_data_created_v1_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_corehr_job_data_created_v1(move |event: JobDataCreatedV1| {
                assert_eq!(event.header.event_type, "corehr.job_data.created_v1");
                assert_eq!(event.event.job_data_id.as_deref(), Some("job_data_id_str"));
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(COREHR_JOB_DATA_CREATED_V1_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_corehr_job_data_deleted_v1_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_corehr_job_data_deleted_v1(move |event: JobDataDeletedV1| {
                assert_eq!(event.header.event_type, "corehr.job_data.deleted_v1");
                assert_eq!(event.event.job_data_id.as_deref(), Some("job_data_id_str"));
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(COREHR_JOB_DATA_DELETED_V1_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_corehr_job_data_employed_v1_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_corehr_job_data_employed_v1(move |event: JobDataEmployedV1| {
                assert_eq!(event.header.event_type, "corehr.job_data.employed_v1");
                assert_eq!(event.event.job_data_id.as_deref(), Some("job_data_id_str"));
                assert_eq!(
                    event.event.employment_id.as_deref(),
                    Some("employment_id_str")
                );
                let target_user_id = event.event.target_user_id.as_ref().expect("target_user_id");
                assert_eq!(target_user_id.user_id.as_deref(), Some("user_id_str"));
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(COREHR_JOB_DATA_EMPLOYED_V1_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_corehr_job_data_updated_v1_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_corehr_job_data_updated_v1(move |event: JobDataUpdatedV1| {
                assert_eq!(event.header.event_type, "corehr.job_data.updated_v1");
                assert_eq!(event.event.job_data_id.as_deref(), Some("job_data_id_str"));
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(COREHR_JOB_DATA_UPDATED_V1_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_corehr_offboarding_updated_v1_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_corehr_offboarding_updated_v1(move |event: OffboardingUpdatedV1| {
                assert_eq!(event.header.event_type, "corehr.offboarding.updated_v1");
                assert_eq!(
                    event.event.employment_id.as_deref(),
                    Some("employment_id_str")
                );
                let target_user_id = event.event.target_user_id.as_ref().expect("target_user_id");
                assert_eq!(target_user_id.user_id.as_deref(), Some("user_id_str"));
                assert_eq!(
                    event.event.offboarding_id.as_deref(),
                    Some("offboarding_id_str")
                );
                assert_eq!(event.event.process_id.as_deref(), Some("process_id_str"));
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(COREHR_OFFBOARDING_UPDATED_V1_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_corehr_org_role_authorization_updated_v1_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_corehr_org_role_authorization_updated_v1(
                move |event: OrgRoleAuthorizationUpdatedV1| {
                    assert_eq!(
                        event.header.event_type,
                        "corehr.org_role_authorization.updated_v1"
                    );
                    assert_eq!(event.event.role_id.as_deref(), Some("role_id_str"));
                    assert_eq!(event.event.management_scope_list.len(), 1);
                    assert_eq!(
                        event.event.management_scope_list[0]
                            .management_dimension
                            .as_deref(),
                        Some("management_dimension_str")
                    );
                    assert_eq!(
                        event.event.employment_id_list[0].as_str(),
                        "employment_id_list_str"
                    );
                    *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                    Ok(())
                },
            )
            .expect("register")
            .build();
        handler
            .do_without_validation(COREHR_ORG_ROLE_AUTHORIZATION_UPDATED_V1_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_corehr_person_created_v1_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_corehr_person_created_v1(move |event: PersonCreatedV1| {
                assert_eq!(event.header.event_type, "corehr.person.created_v1");
                assert_eq!(event.event.person_id.as_deref(), Some("person_id_str"));
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(COREHR_PERSON_CREATED_V1_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_corehr_person_deleted_v1_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_corehr_person_deleted_v1(move |event: PersonDeletedV1| {
                assert_eq!(event.header.event_type, "corehr.person.deleted_v1");
                assert_eq!(event.event.person_id.as_deref(), Some("person_id_str"));
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(COREHR_PERSON_DELETED_V1_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_corehr_person_updated_v1_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_corehr_person_updated_v1(move |event: PersonUpdatedV1| {
                assert_eq!(event.header.event_type, "corehr.person.updated_v1");
                assert_eq!(event.event.person_id.as_deref(), Some("person_id_str"));
                assert_eq!(event.event.field_changes[0].as_str(), "field_changes_str");
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(COREHR_PERSON_UPDATED_V1_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_corehr_pre_hire_updated_v1_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_corehr_pre_hire_updated_v1(move |event: PreHireUpdatedV1| {
                assert_eq!(event.header.event_type, "corehr.pre_hire.updated_v1");
                assert_eq!(event.event.pre_hire_id.as_deref(), Some("pre_hire_id_str"));
                assert_eq!(event.event.field_changes[0].as_str(), "field_changes_str");
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(COREHR_PRE_HIRE_UPDATED_V1_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    /// 事件串常量与 Go v3.12.2 dispatcher 赋值行逐字一致。
    #[test]
    fn typed_corehr_v1_event_constants_match_go_dispatcher() {
        assert_eq!(
            EventDispatcherHandler::COREHR_COMMON_DATA_ID_USER_MAPPING_CHANGED_V1,
            "corehr.common_data.id.user_mapping_changed_v1"
        );
        assert_eq!(
            EventDispatcherHandler::COREHR_COMMON_DATA_META_DATA_UPDATED_V1,
            "corehr.common_data.meta_data.updated_v1"
        );
        assert_eq!(
            EventDispatcherHandler::COREHR_CONTRACT_CREATED_V1,
            "corehr.contract.created_v1"
        );
        assert_eq!(
            EventDispatcherHandler::COREHR_CONTRACT_DELETED_V1,
            "corehr.contract.deleted_v1"
        );
        assert_eq!(
            EventDispatcherHandler::COREHR_CONTRACT_UPDATED_V1,
            "corehr.contract.updated_v1"
        );
        assert_eq!(
            EventDispatcherHandler::COREHR_DEPARTMENT_CREATED_V1,
            "corehr.department.created_v1"
        );
        assert_eq!(
            EventDispatcherHandler::COREHR_DEPARTMENT_DELETED_V1,
            "corehr.department.deleted_v1"
        );
        assert_eq!(
            EventDispatcherHandler::COREHR_DEPARTMENT_UPDATED_V1,
            "corehr.department.updated_v1"
        );
        assert_eq!(
            EventDispatcherHandler::COREHR_EMPLOYMENT_CONVERTED_V1,
            "corehr.employment.converted_v1"
        );
        assert_eq!(
            EventDispatcherHandler::COREHR_EMPLOYMENT_CREATED_V1,
            "corehr.employment.created_v1"
        );
        assert_eq!(
            EventDispatcherHandler::COREHR_EMPLOYMENT_DELETED_V1,
            "corehr.employment.deleted_v1"
        );
        assert_eq!(
            EventDispatcherHandler::COREHR_EMPLOYMENT_RESIGNED_V1,
            "corehr.employment.resigned_v1"
        );
        assert_eq!(
            EventDispatcherHandler::COREHR_EMPLOYMENT_UPDATED_V1,
            "corehr.employment.updated_v1"
        );
        assert_eq!(
            EventDispatcherHandler::COREHR_JOB_CREATED_V1,
            "corehr.job.created_v1"
        );
        assert_eq!(
            EventDispatcherHandler::COREHR_JOB_DELETED_V1,
            "corehr.job.deleted_v1"
        );
        assert_eq!(
            EventDispatcherHandler::COREHR_JOB_UPDATED_V1,
            "corehr.job.updated_v1"
        );
        assert_eq!(
            EventDispatcherHandler::COREHR_JOB_CHANGE_UPDATED_V1,
            "corehr.job_change.updated_v1"
        );
        assert_eq!(
            EventDispatcherHandler::COREHR_JOB_DATA_CHANGED_V1,
            "corehr.job_data.changed_v1"
        );
        assert_eq!(
            EventDispatcherHandler::COREHR_JOB_DATA_CREATED_V1,
            "corehr.job_data.created_v1"
        );
        assert_eq!(
            EventDispatcherHandler::COREHR_JOB_DATA_DELETED_V1,
            "corehr.job_data.deleted_v1"
        );
        assert_eq!(
            EventDispatcherHandler::COREHR_JOB_DATA_EMPLOYED_V1,
            "corehr.job_data.employed_v1"
        );
        assert_eq!(
            EventDispatcherHandler::COREHR_JOB_DATA_UPDATED_V1,
            "corehr.job_data.updated_v1"
        );
        assert_eq!(
            EventDispatcherHandler::COREHR_OFFBOARDING_UPDATED_V1,
            "corehr.offboarding.updated_v1"
        );
        assert_eq!(
            EventDispatcherHandler::COREHR_ORG_ROLE_AUTHORIZATION_UPDATED_V1,
            "corehr.org_role_authorization.updated_v1"
        );
        assert_eq!(
            EventDispatcherHandler::COREHR_PERSON_CREATED_V1,
            "corehr.person.created_v1"
        );
        assert_eq!(
            EventDispatcherHandler::COREHR_PERSON_DELETED_V1,
            "corehr.person.deleted_v1"
        );
        assert_eq!(
            EventDispatcherHandler::COREHR_PERSON_UPDATED_V1,
            "corehr.person.updated_v1"
        );
        assert_eq!(
            EventDispatcherHandler::COREHR_PRE_HIRE_UPDATED_V1,
            "corehr.pre_hire.updated_v1"
        );
    }

    /// typed 与 `register_raw` 同 key 重复注册返回 Err（双向）。
    #[test]
    fn typed_corehr_v1_duplicate_register_fails() {
        struct Noop;
        impl crate::ws_client::EventHandler for Noop {
            fn handle(
                &self,
                _payload: &[u8],
            ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
                Ok(())
            }
        }
        let err = EventDispatcherHandler::builder()
            .register_corehr_common_data_id_user_mapping_changed_v1(
                |_event: CommonDataIdUserMappingChangedV1| Ok(()),
            )
            .expect("first")
            .register_raw("corehr.common_data.id.user_mapping_changed_v1", Noop)
            .expect_err("dup typed-then-raw");
        assert!(err.contains("already registered"));
        let err = EventDispatcherHandler::builder()
            .register_raw("corehr.common_data.id.user_mapping_changed_v1", Noop)
            .expect("raw first")
            .register_corehr_common_data_id_user_mapping_changed_v1(
                |_event: CommonDataIdUserMappingChangedV1| Ok(()),
            )
            .expect_err("dup raw-then-typed");
        assert!(err.contains("already registered"));
    }
}
