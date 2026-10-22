//! corehr 域 v1 typed 事件（v1 事件串，与 v2 两套独立体系，见 [`super::corehr_v2`]）。
//!
//! 字段权威：Go SDK v3.12.2 `service/corehr/v1/model.go` 对应 `P2*` 结构的 json tag；
//! 事件清单对齐 `service/corehr/v1/dispatcher.go`。Go 指针字段映射为 `Option<T>`，
//! serde 采用 `#[serde(default)]` 宽松风格，与 [`super::ImMessageReceiveV1`] 一致。

use serde::{Deserialize, Serialize};

use super::EventHeader;

/// `corehr.common_data.id.user_mapping_changed_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrCommonDataIdUserMappingChangedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: CorehrCommonDataIdUserMappingChangedV1Event,
}

/// `corehr.common_data.id.user_mapping_changed_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrCommonDataIdUserMappingChangedV1Event {
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
pub struct CorehrCommonDataMetaDataUpdatedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: CorehrCommonDataMetaDataUpdatedV1Event,
}

/// `corehr.common_data.meta_data.updated_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrCommonDataMetaDataUpdatedV1Event {
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
pub struct CorehrContractCreatedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: CorehrContractCreatedV1Event,
}

/// `corehr.contract.created_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrContractCreatedV1Event {
    /// 合同ID，该合同详细信息可通过[【合同信息查询接口】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/contract/get)获取
    #[serde(default)]
    pub contract_id: Option<String>,
}

/// `corehr.contract.deleted_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrContractDeletedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: CorehrContractDeletedV1Event,
}

/// `corehr.contract.deleted_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrContractDeletedV1Event {
    /// 合同ID，删除后无法通过接口查询到数据
    #[serde(default)]
    pub contract_id: Option<String>,
}

/// `corehr.contract.updated_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrContractUpdatedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: CorehrContractUpdatedV1Event,
}

/// `corehr.contract.updated_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrContractUpdatedV1Event {
    /// 合同ID，该合同详细信息可通过[【合同信息查询接口】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/contract/get)获取
    #[serde(default)]
    pub contract_id: Option<String>,
}

/// `corehr.department.created_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrDepartmentCreatedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: CorehrDepartmentCreatedV1Event,
}

/// `corehr.department.created_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrDepartmentCreatedV1Event {
    /// 新建部门的 ID
    #[serde(default)]
    pub department_id: Option<String>,
}

/// `corehr.department.deleted_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrDepartmentDeletedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: CorehrDepartmentDeletedV1Event,
}

/// `corehr.department.deleted_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrDepartmentDeletedV1Event {
    /// 被删除部门的 ID
    #[serde(default)]
    pub department_id: Option<String>,
    /// 部门编码
    #[serde(default)]
    pub code: Option<String>,
}

/// `corehr.department.updated_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrDepartmentUpdatedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: CorehrDepartmentUpdatedV1Event,
}

/// `corehr.department.updated_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrDepartmentUpdatedV1Event {
    /// 被更新部门的 ID
    #[serde(default)]
    pub department_id: Option<String>,
    /// 发生变更的字段
    #[serde(default)]
    pub field_changes: Vec<String>,
}

/// `corehr.employment.converted_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrEmploymentConvertedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: CorehrEmploymentConvertedV1Event,
}

/// `corehr.employment.converted_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrEmploymentConvertedV1Event {
    /// 雇佣ID，可通过[【搜索员工信息】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/employee/search)接口获取
    #[serde(default)]
    pub employment_id: Option<String>,
}

/// `corehr.employment.created_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrEmploymentCreatedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: CorehrEmploymentCreatedV1Event,
}

/// `corehr.employment.created_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrEmploymentCreatedV1Event {
    /// 被创建的雇佣信息的 ID
    #[serde(default)]
    pub employment_id: Option<String>,
    /// 员工的飞书用户 ID
    #[serde(default)]
    pub target_user_id: Option<UserId>,
}

/// `corehr.employment.deleted_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrEmploymentDeletedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: CorehrEmploymentDeletedV1Event,
}

/// `corehr.employment.deleted_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrEmploymentDeletedV1Event {
    /// 被删除的雇佣信息的 ID
    #[serde(default)]
    pub employment_id: Option<String>,
    /// 员工的飞书用户 ID
    #[serde(default)]
    pub target_user_id: Option<UserId>,
}

/// `corehr.employment.resigned_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrEmploymentResignedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: CorehrEmploymentResignedV1Event,
}

/// `corehr.employment.resigned_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrEmploymentResignedV1Event {
    /// 离职员工雇佣ID，可通过[批量查询员工信息;](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/employee/batch_get)获取员工信息。
    #[serde(default)]
    pub employment_id: Option<String>,
}

/// `corehr.employment.updated_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrEmploymentUpdatedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: CorehrEmploymentUpdatedV1Event,
}

/// `corehr.employment.updated_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrEmploymentUpdatedV1Event {
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
pub struct CorehrJobCreatedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: CorehrJobCreatedV1Event,
}

/// `corehr.job.created_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrJobCreatedV1Event {
    /// 职务 ID，可通过[【查询单个职务】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/job/get)获取详细信息
    #[serde(default)]
    pub job_id: Option<String>,
}

/// `corehr.job.deleted_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrJobDeletedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: CorehrJobDeletedV1Event,
}

/// `corehr.job.deleted_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrJobDeletedV1Event {
    /// 职务 ID
    #[serde(default)]
    pub job_id: Option<String>,
}

/// `corehr.job.updated_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrJobUpdatedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: CorehrJobUpdatedV1Event,
}

/// `corehr.job.updated_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrJobUpdatedV1Event {
    /// 职务 ID，可通过[【查询单个职务】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/job/get)获取详细信息
    #[serde(default)]
    pub job_id: Option<String>,
}

/// `corehr.job_change.updated_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrJobChangeUpdatedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: CorehrJobChangeUpdatedV1Event,
}

/// `corehr.job_change.updated_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrJobChangeUpdatedV1Event {
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
pub struct CorehrJobDataChangedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: CorehrJobDataChangedV1Event,
}

/// `corehr.job_data.changed_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrJobDataChangedV1Event {
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
pub struct CorehrJobDataCreatedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: CorehrJobDataCreatedV1Event,
}

/// `corehr.job_data.created_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrJobDataCreatedV1Event {
    /// ID
    #[serde(default)]
    pub job_data_id: Option<String>,
}

/// `corehr.job_data.deleted_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrJobDataDeletedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: CorehrJobDataDeletedV1Event,
}

/// `corehr.job_data.deleted_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrJobDataDeletedV1Event {
    /// ID
    #[serde(default)]
    pub job_data_id: Option<String>,
}

/// `corehr.job_data.employed_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrJobDataEmployedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: CorehrJobDataEmployedV1Event,
}

/// `corehr.job_data.employed_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrJobDataEmployedV1Event {
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
pub struct CorehrJobDataUpdatedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: CorehrJobDataUpdatedV1Event,
}

/// `corehr.job_data.updated_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrJobDataUpdatedV1Event {
    /// ID
    #[serde(default)]
    pub job_data_id: Option<String>,
}

/// `corehr.offboarding.updated_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrOffboardingUpdatedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: CorehrOffboardingUpdatedV1Event,
}

/// `corehr.offboarding.updated_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrOffboardingUpdatedV1Event {
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
pub struct CorehrOrgRoleAuthorizationUpdatedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: CorehrOrgRoleAuthorizationUpdatedV1Event,
}

/// `corehr.org_role_authorization.updated_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrOrgRoleAuthorizationUpdatedV1Event {
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
pub struct CorehrPersonCreatedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: CorehrPersonCreatedV1Event,
}

/// `corehr.person.created_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrPersonCreatedV1Event {
    /// 人员ID
    #[serde(default)]
    pub person_id: Option<String>,
}

/// `corehr.person.deleted_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrPersonDeletedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: CorehrPersonDeletedV1Event,
}

/// `corehr.person.deleted_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrPersonDeletedV1Event {
    /// 人员ID
    #[serde(default)]
    pub person_id: Option<String>,
}

/// `corehr.person.updated_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrPersonUpdatedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: CorehrPersonUpdatedV1Event,
}

/// `corehr.person.updated_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrPersonUpdatedV1Event {
    /// 被更新个人信息的 ID
    #[serde(default)]
    pub person_id: Option<String>,
    /// 发生变更的字段，若部分字段变更无需接受事件消息，可根据该字段进行事件消息过滤
    #[serde(default)]
    pub field_changes: Vec<String>,
}

/// `corehr.pre_hire.updated_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrPreHireUpdatedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: CorehrPreHireUpdatedV1Event,
}

/// `corehr.pre_hire.updated_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrPreHireUpdatedV1Event {
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
    "corehr.common_data.id.user_mapping_changed_v1" | COREHR_COMMON_DATA_ID_USER_MAPPING_CHANGED_V1 | CorehrCommonDataIdUserMappingChangedV1 | CorehrCommonDataIdUserMappingChangedV1Handler | register_corehr_common_data_id_user_mapping_changed_v1;
    /// `corehr.common_data.meta_data.updated_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.common_data.meta_data.updated_v1" | COREHR_COMMON_DATA_META_DATA_UPDATED_V1 | CorehrCommonDataMetaDataUpdatedV1 | CorehrCommonDataMetaDataUpdatedV1Handler | register_corehr_common_data_meta_data_updated_v1;
    /// `corehr.contract.created_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.contract.created_v1" | COREHR_CONTRACT_CREATED_V1 | CorehrContractCreatedV1 | CorehrContractCreatedV1Handler | register_corehr_contract_created_v1;
    /// `corehr.contract.deleted_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.contract.deleted_v1" | COREHR_CONTRACT_DELETED_V1 | CorehrContractDeletedV1 | CorehrContractDeletedV1Handler | register_corehr_contract_deleted_v1;
    /// `corehr.contract.updated_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.contract.updated_v1" | COREHR_CONTRACT_UPDATED_V1 | CorehrContractUpdatedV1 | CorehrContractUpdatedV1Handler | register_corehr_contract_updated_v1;
    /// `corehr.department.created_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.department.created_v1" | COREHR_DEPARTMENT_CREATED_V1 | CorehrDepartmentCreatedV1 | CorehrDepartmentCreatedV1Handler | register_corehr_department_created_v1;
    /// `corehr.department.deleted_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.department.deleted_v1" | COREHR_DEPARTMENT_DELETED_V1 | CorehrDepartmentDeletedV1 | CorehrDepartmentDeletedV1Handler | register_corehr_department_deleted_v1;
    /// `corehr.department.updated_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.department.updated_v1" | COREHR_DEPARTMENT_UPDATED_V1 | CorehrDepartmentUpdatedV1 | CorehrDepartmentUpdatedV1Handler | register_corehr_department_updated_v1;
    /// `corehr.employment.converted_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.employment.converted_v1" | COREHR_EMPLOYMENT_CONVERTED_V1 | CorehrEmploymentConvertedV1 | CorehrEmploymentConvertedV1Handler | register_corehr_employment_converted_v1;
    /// `corehr.employment.created_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.employment.created_v1" | COREHR_EMPLOYMENT_CREATED_V1 | CorehrEmploymentCreatedV1 | CorehrEmploymentCreatedV1Handler | register_corehr_employment_created_v1;
    /// `corehr.employment.deleted_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.employment.deleted_v1" | COREHR_EMPLOYMENT_DELETED_V1 | CorehrEmploymentDeletedV1 | CorehrEmploymentDeletedV1Handler | register_corehr_employment_deleted_v1;
    /// `corehr.employment.resigned_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.employment.resigned_v1" | COREHR_EMPLOYMENT_RESIGNED_V1 | CorehrEmploymentResignedV1 | CorehrEmploymentResignedV1Handler | register_corehr_employment_resigned_v1;
    /// `corehr.employment.updated_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.employment.updated_v1" | COREHR_EMPLOYMENT_UPDATED_V1 | CorehrEmploymentUpdatedV1 | CorehrEmploymentUpdatedV1Handler | register_corehr_employment_updated_v1;
    /// `corehr.job.created_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.job.created_v1" | COREHR_JOB_CREATED_V1 | CorehrJobCreatedV1 | CorehrJobCreatedV1Handler | register_corehr_job_created_v1;
    /// `corehr.job.deleted_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.job.deleted_v1" | COREHR_JOB_DELETED_V1 | CorehrJobDeletedV1 | CorehrJobDeletedV1Handler | register_corehr_job_deleted_v1;
    /// `corehr.job.updated_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.job.updated_v1" | COREHR_JOB_UPDATED_V1 | CorehrJobUpdatedV1 | CorehrJobUpdatedV1Handler | register_corehr_job_updated_v1;
    /// `corehr.job_change.updated_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.job_change.updated_v1" | COREHR_JOB_CHANGE_UPDATED_V1 | CorehrJobChangeUpdatedV1 | CorehrJobChangeUpdatedV1Handler | register_corehr_job_change_updated_v1;
    /// `corehr.job_data.changed_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.job_data.changed_v1" | COREHR_JOB_DATA_CHANGED_V1 | CorehrJobDataChangedV1 | CorehrJobDataChangedV1Handler | register_corehr_job_data_changed_v1;
    /// `corehr.job_data.created_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.job_data.created_v1" | COREHR_JOB_DATA_CREATED_V1 | CorehrJobDataCreatedV1 | CorehrJobDataCreatedV1Handler | register_corehr_job_data_created_v1;
    /// `corehr.job_data.deleted_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.job_data.deleted_v1" | COREHR_JOB_DATA_DELETED_V1 | CorehrJobDataDeletedV1 | CorehrJobDataDeletedV1Handler | register_corehr_job_data_deleted_v1;
    /// `corehr.job_data.employed_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.job_data.employed_v1" | COREHR_JOB_DATA_EMPLOYED_V1 | CorehrJobDataEmployedV1 | CorehrJobDataEmployedV1Handler | register_corehr_job_data_employed_v1;
    /// `corehr.job_data.updated_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.job_data.updated_v1" | COREHR_JOB_DATA_UPDATED_V1 | CorehrJobDataUpdatedV1 | CorehrJobDataUpdatedV1Handler | register_corehr_job_data_updated_v1;
    /// `corehr.offboarding.updated_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.offboarding.updated_v1" | COREHR_OFFBOARDING_UPDATED_V1 | CorehrOffboardingUpdatedV1 | CorehrOffboardingUpdatedV1Handler | register_corehr_offboarding_updated_v1;
    /// `corehr.org_role_authorization.updated_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.org_role_authorization.updated_v1" | COREHR_ORG_ROLE_AUTHORIZATION_UPDATED_V1 | CorehrOrgRoleAuthorizationUpdatedV1 | CorehrOrgRoleAuthorizationUpdatedV1Handler | register_corehr_org_role_authorization_updated_v1;
    /// `corehr.person.created_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.person.created_v1" | COREHR_PERSON_CREATED_V1 | CorehrPersonCreatedV1 | CorehrPersonCreatedV1Handler | register_corehr_person_created_v1;
    /// `corehr.person.deleted_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.person.deleted_v1" | COREHR_PERSON_DELETED_V1 | CorehrPersonDeletedV1 | CorehrPersonDeletedV1Handler | register_corehr_person_deleted_v1;
    /// `corehr.person.updated_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.person.updated_v1" | COREHR_PERSON_UPDATED_V1 | CorehrPersonUpdatedV1 | CorehrPersonUpdatedV1Handler | register_corehr_person_updated_v1;
    /// `corehr.pre_hire.updated_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.pre_hire.updated_v1" | COREHR_PRE_HIRE_UPDATED_V1 | CorehrPreHireUpdatedV1 | CorehrPreHireUpdatedV1Handler | register_corehr_pre_hire_updated_v1;
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
                move |event: CorehrCommonDataIdUserMappingChangedV1| {
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
                move |event: CorehrCommonDataMetaDataUpdatedV1| {
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
            .register_corehr_contract_created_v1(move |event: CorehrContractCreatedV1| {
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
            .register_corehr_contract_deleted_v1(move |event: CorehrContractDeletedV1| {
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
            .register_corehr_contract_updated_v1(move |event: CorehrContractUpdatedV1| {
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
            .register_corehr_department_created_v1(move |event: CorehrDepartmentCreatedV1| {
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
            .register_corehr_department_deleted_v1(move |event: CorehrDepartmentDeletedV1| {
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
            .register_corehr_department_updated_v1(move |event: CorehrDepartmentUpdatedV1| {
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
            .register_corehr_employment_converted_v1(move |event: CorehrEmploymentConvertedV1| {
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
            .register_corehr_employment_created_v1(move |event: CorehrEmploymentCreatedV1| {
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
            .register_corehr_employment_deleted_v1(move |event: CorehrEmploymentDeletedV1| {
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
            .register_corehr_employment_resigned_v1(move |event: CorehrEmploymentResignedV1| {
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
            .register_corehr_employment_updated_v1(move |event: CorehrEmploymentUpdatedV1| {
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
            .register_corehr_job_created_v1(move |event: CorehrJobCreatedV1| {
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
            .register_corehr_job_deleted_v1(move |event: CorehrJobDeletedV1| {
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
            .register_corehr_job_updated_v1(move |event: CorehrJobUpdatedV1| {
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
            .register_corehr_job_change_updated_v1(move |event: CorehrJobChangeUpdatedV1| {
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
            .register_corehr_job_data_changed_v1(move |event: CorehrJobDataChangedV1| {
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
            .register_corehr_job_data_created_v1(move |event: CorehrJobDataCreatedV1| {
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
            .register_corehr_job_data_deleted_v1(move |event: CorehrJobDataDeletedV1| {
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
            .register_corehr_job_data_employed_v1(move |event: CorehrJobDataEmployedV1| {
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
            .register_corehr_job_data_updated_v1(move |event: CorehrJobDataUpdatedV1| {
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
            .register_corehr_offboarding_updated_v1(move |event: CorehrOffboardingUpdatedV1| {
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
                move |event: CorehrOrgRoleAuthorizationUpdatedV1| {
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
            .register_corehr_person_created_v1(move |event: CorehrPersonCreatedV1| {
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
            .register_corehr_person_deleted_v1(move |event: CorehrPersonDeletedV1| {
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
            .register_corehr_person_updated_v1(move |event: CorehrPersonUpdatedV1| {
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
            .register_corehr_pre_hire_updated_v1(move |event: CorehrPreHireUpdatedV1| {
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
                |_event: CorehrCommonDataIdUserMappingChangedV1| Ok(()),
            )
            .expect("first")
            .register_raw("corehr.common_data.id.user_mapping_changed_v1", Noop)
            .expect_err("dup typed-then-raw");
        assert!(err.contains("already registered"));
        let err = EventDispatcherHandler::builder()
            .register_raw("corehr.common_data.id.user_mapping_changed_v1", Noop)
            .expect("raw first")
            .register_corehr_common_data_id_user_mapping_changed_v1(
                |_event: CorehrCommonDataIdUserMappingChangedV1| Ok(()),
            )
            .expect_err("dup raw-then-typed");
        assert!(err.contains("already registered"));
    }
}
