//! corehr 域 v2 typed 事件（v2 事件串，与 v1 两套独立体系，见 [`super::corehr_v1`]）。
//!
//! 字段权威：Go SDK v3.12.2 `service/corehr/v2/model.go` 对应 `P2*` 结构的 json tag；
//! 事件清单对齐 `service/corehr/v2/dispatcher.go`。Go 指针字段映射为 `Option<T>`，
//! serde 采用 `#[serde(default)]` 宽松风格，与 [`super::im::ImMessageReceiveV1`] 一致。

use serde::{Deserialize, Serialize};

use super::EventHeader;

/// `corehr.approval_groups.updated_v2` 完整 envelope。
///
/// 官方没有替代事件。不推荐的是字段 `approval_group_status`，推荐同事件字段
/// `approval_group_status_v2`。本类型与 `register_corehr_approval_groups_updated_v2` 保留。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrApprovalGroupsUpdatedV2 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: CorehrApprovalGroupsUpdatedV2Event,
}

/// `corehr.approval_groups.updated_v2` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrApprovalGroupsUpdatedV2Event {
    /// 组织架构调整审批组 ID
    #[serde(default)]
    pub approval_group_id: Option<String>,
    /// 组织架构调整流程 ID， 用户通过『飞书人事-我的团队-组织架构』或『飞书 人事-人员管理-组织架构』 发起一个组织架构调整，并提交审批后，系统会根据管理员在审批流程中配置的规则，生成 一个或多个审批单据。
    #[serde(default)]
    pub process_id: Option<String>,
    /// 组织架构调整流程状态，枚举类型， 描述该审批单据的状态。;<md-alert type="tip" icon="none">;【不推荐使用，无法区分审批通过、执行失败和等待执行状态，推荐使用approval_group_status_v2】;</md-alert>
    #[serde(default)]
    pub approval_group_status: Option<i64>,
    /// 调整主题， 用户在『飞书人事-我的团队/人员管理 -组织架构-发起调整』填写的调整变更主题
    #[serde(default)]
    pub topic: Option<String>,
    /// 调整原因，用户在『飞书人事-我的团队/人员管理 -组织架构-发起调整』填写的调整变更详细原因， 便于审批人批阅。
    #[serde(default)]
    pub adjust_reason: Option<String>,
    /// 生效日期;- 日期格式：YYYY-MM-DD;- 最小值： 1900-01-01;- 最大值： 9999-12-31
    #[serde(default)]
    pub effective_date: Option<String>,
    /// 发起人，在『飞书人事-我的团队/人员管理 -组织架构-发起调整』填写的调整变更发起人。
    #[serde(default)]
    pub created_by: Option<String>,
    /// 组织架构调整 ID，用户在『飞书人事-我的团队/人员管理 -组织架构-发起调整』 时生成的唯一 ID。
    #[serde(default)]
    pub draft_id: Option<String>,
    /// 整个组织架构调整状态，枚举类型，一个组织架构调整可能涉及多个审批流程。 该状态描述整个调整的审核状态
    #[serde(default)]
    pub draft_status: Option<i64>,
    /// 组织架构调整流程状态，枚举类型，描述该审批单据的状态。
    #[serde(default)]
    pub approval_group_status_v2: Option<i64>,
}

/// `corehr.company.created_v2` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrCompanyCreatedV2 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: CorehrCompanyCreatedV2Event,
}

/// `corehr.company.created_v2` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrCompanyCreatedV2Event {
    /// 公司ID。;- 调用[【查询单个公司】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/company/get)、[【通过 ID 批量查询公司信息】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/company/batch_get)接口返回公司详细信息
    #[serde(default)]
    pub company_id: Option<String>,
}

/// `corehr.company.deleted_v2` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrCompanyDeletedV2 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: CorehrCompanyDeletedV2Event,
}

/// `corehr.company.deleted_v2` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrCompanyDeletedV2Event {
    /// 被删除公司的 ID。
    #[serde(default)]
    pub company_id: Option<String>,
}

/// `corehr.company.updated_v2` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrCompanyUpdatedV2 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: CorehrCompanyUpdatedV2Event,
}

/// `corehr.company.updated_v2` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrCompanyUpdatedV2Event {
    /// 公司ID。;- 调用[【查询单个公司】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/company/get)、[【通过 ID 批量查询公司信息】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/company/batch_get)接口返回公司详细信息
    #[serde(default)]
    pub company_id: Option<String>,
    /// 发生变更的字段
    #[serde(default)]
    pub field_changes: Vec<String>,
    /// 子变更事件
    #[serde(default)]
    pub sub_events: Vec<OrganizationDomainEventData>,
}

/// `corehr.cost_center.created_v2` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrCostCenterCreatedV2 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: CorehrCostCenterCreatedV2Event,
}

/// `corehr.cost_center.created_v2` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrCostCenterCreatedV2Event {
    /// 成本中心ID。;- 调用[【搜索成本中心信息】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/cost_center/search)接口返回成本中心详细信息
    #[serde(default)]
    pub cost_center_id: Option<String>,
}

/// `corehr.cost_center.deleted_v2` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrCostCenterDeletedV2 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: CorehrCostCenterDeletedV2Event,
}

/// `corehr.cost_center.deleted_v2` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrCostCenterDeletedV2Event {
    /// 被删除成本中心的 ID。
    #[serde(default)]
    pub cost_center_id: Option<String>,
}

/// `corehr.cost_center.updated_v2` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrCostCenterUpdatedV2 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: CorehrCostCenterUpdatedV2Event,
}

/// `corehr.cost_center.updated_v2` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrCostCenterUpdatedV2Event {
    /// 成本中心ID。;- 调用[【搜索成本中心信息】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/cost_center/search)接口返回成本中心详细信息
    #[serde(default)]
    pub cost_center_id: Option<String>,
    /// 发生变更的字段
    #[serde(default)]
    pub field_changes: Vec<String>,
}

/// `corehr.custom_org.created_v2` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrCustomOrgCreatedV2 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: CorehrCustomOrgCreatedV2Event,
}

/// `corehr.custom_org.created_v2` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrCustomOrgCreatedV2Event {
    /// 自定义组织ID。;- 调用[【批量查询当前版本的自定义组织信息】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/custom_org/query)接口返回自定义组织详细信息
    #[serde(default)]
    pub org_id: Option<String>,
    /// 组织类型编码，可在「飞书人事-设置-组织设置」中相应的自定义组织目录下查看
    #[serde(default)]
    pub object_api_name: Option<String>,
}

/// `corehr.custom_org.deleted_v2` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrCustomOrgDeletedV2 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: CorehrCustomOrgDeletedV2Event,
}

/// `corehr.custom_org.deleted_v2` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrCustomOrgDeletedV2Event {
    /// 被删除自定义组织的 ID
    #[serde(default)]
    pub org_id: Option<String>,
    /// 组织类型编码，可在「飞书人事-设置-组织设置」中相应的自定义组织目录下查看
    #[serde(default)]
    pub object_api_name: Option<String>,
}

/// `corehr.custom_org.updated_v2` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrCustomOrgUpdatedV2 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: CorehrCustomOrgUpdatedV2Event,
}

/// `corehr.custom_org.updated_v2` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrCustomOrgUpdatedV2Event {
    /// 自定义组织ID。;- 调用[【批量查询当前版本的自定义组织信息】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/custom_org/query)接口返回自定义组织详细信息
    #[serde(default)]
    pub org_id: Option<String>,
    /// 发生变更的字段
    #[serde(default)]
    pub field_changes: Vec<String>,
    /// 组织类型编码，可在「飞书人事-设置-组织设置」中相应的自定义组织目录下查看
    #[serde(default)]
    pub object_api_name: Option<String>,
}

/// `corehr.department.created_v2` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrDepartmentCreatedV2 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: CorehrDepartmentCreatedV2Event,
}

/// `corehr.department.created_v2` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrDepartmentCreatedV2Event {
    /// 部门id
    #[serde(default)]
    pub department_id: Option<String>,
}

/// `corehr.department.updated_v2` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrDepartmentUpdatedV2 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: CorehrDepartmentUpdatedV2Event,
}

/// `corehr.department.updated_v2` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrDepartmentUpdatedV2Event {
    /// 部门id
    #[serde(default)]
    pub department_id: Option<String>,
    /// 发生变更的字段
    #[serde(default)]
    pub field_changes: Vec<String>,
}

/// `corehr.employee.domain_event_v2` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrEmployeeDomainEventV2 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: CorehrEmployeeDomainEventV2Event,
}

/// `corehr.employee.domain_event_v2` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrEmployeeDomainEventV2Event {
    /// 事件类型
    #[serde(default)]
    pub event_type: Option<i64>,
    /// 子事件类型
    #[serde(default)]
    pub sub_event_type: Option<i64>,
    /// 操作用户 ID
    #[serde(default)]
    pub operator_user_id: Option<String>,
    /// 业务场景;- 标识触发此事件操作场景：系统页面操作、开发平台接口操作等
    #[serde(default)]
    pub opt_scene: Option<String>,
    /// 操作描述
    #[serde(default)]
    pub opt_desc: Option<String>,
    /// 操作时间戳，单位：秒
    #[serde(default, deserialize_with = "super::flex_time::de_opt_string")]
    pub opt_time: Option<String>,
    /// 操作 ID
    #[serde(default)]
    pub opt_id: Option<String>,
    /// 被操作人的雇佣 ID
    #[serde(default)]
    pub employment_id: Option<String>,
    /// 变更数据
    #[serde(default)]
    pub data: Vec<EmployeeDomainEventData>,
}

/// `corehr.job_change.status_updated_v2` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrJobChangeStatusUpdatedV2 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: CorehrJobChangeStatusUpdatedV2Event,
}

/// `corehr.job_change.status_updated_v2` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrJobChangeStatusUpdatedV2Event {
    /// 雇员ID
    #[serde(default)]
    pub employment_id: Option<String>,
    /// 员工的飞书用户 ID
    #[serde(default)]
    pub target_user_id: Option<UserId>,
    /// 异动记录 id
    #[serde(default)]
    pub job_change_id: Option<String>,
    /// 异动属性/方式
    #[serde(default)]
    pub transfer_mode: Option<i64>,
    /// 异动类型唯一标识
    #[serde(default)]
    pub transfer_type_unique_identifier: Option<String>,
    /// 异动原因唯一标识
    #[serde(default)]
    pub transfer_reason_unique_identifier: Option<String>,
    /// 异动关联流程 id，如果是直接异动，则不需要关联流程
    #[serde(default)]
    pub process_id: Option<String>,
    /// 异动生效日期
    #[serde(default)]
    pub effective_date: Option<String>,
    /// 异动状态
    #[serde(default)]
    pub status: Option<i64>,
    /// 原异动状态
    #[serde(default)]
    pub original_status: Option<i64>,
    /// 异动记录标识符
    #[serde(default)]
    pub transfer_key: Option<String>,
    /// 异动变更内容
    #[serde(default)]
    pub details_of_job_status_change: Vec<String>,
}

/// `corehr.job_change.updated_v2` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrJobChangeUpdatedV2 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: CorehrJobChangeUpdatedV2Event,
}

/// `corehr.job_change.updated_v2` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrJobChangeUpdatedV2Event {
    /// 异动员工雇员ID，可通过[【搜索员工信息】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/employee/search)接口获取
    #[serde(default)]
    pub employment_id: Option<String>,
    /// Saas租户ID
    #[serde(default)]
    pub tenant_id: Option<String>,
    /// 流程ID，可通过【流程-获取单个流程详情】接口获取
    #[serde(default)]
    pub process_id: Option<String>,
    /// 发起人雇员ID，可通过[【搜索员工信息】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/employee/search)接口获取
    #[serde(default)]
    pub initiator: Option<String>,
    /// 操作人雇员ID，可通过[【搜索员工信息】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/employee/search)接口获取
    #[serde(default)]
    pub operator: Option<String>,
    /// 更新时间毫秒时间戳;- 示例：1714985958000
    #[serde(default, deserialize_with = "super::flex_time::de_opt_string")]
    pub updated_time: Option<String>,
    /// 异动记录ID，可通过接口;[搜索异动信息;](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/job_change/search)获取详细信息
    #[serde(default)]
    pub job_change_id: Option<String>,
    /// 变更后异动状态
    #[serde(default)]
    pub status: Option<i64>,
    /// 操作原因
    #[serde(default)]
    pub operate_reason: Option<String>,
    /// 变更类型
    #[serde(default)]
    pub transfer_type: Option<i64>,
    /// 变更字段
    #[serde(default)]
    pub updated_fields: Vec<String>,
    /// 异动类型，可通过接口;[获取异动类型列表;](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/transfer_type/query)获取
    #[serde(default)]
    pub transform_type: Option<String>,
    /// 异动原因，可通过接口;[获取异动原因列表;](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/transfer_reason/query)获取详细信息
    #[serde(default)]
    pub transform_reason: Option<String>,
}

/// `corehr.job_family.created_v2` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrJobFamilyCreatedV2 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: CorehrJobFamilyCreatedV2Event,
}

/// `corehr.job_family.created_v2` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrJobFamilyCreatedV2Event {
    /// 序列ID。;- 调用[【查询单个序列】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/job_family/get)、[【通过序列 ID 批量查询序列信息】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/job_family/batch_get)接口返回序列详细信息
    #[serde(default)]
    pub job_family_id: Option<String>,
}

/// `corehr.job_family.deleted_v2` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrJobFamilyDeletedV2 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: CorehrJobFamilyDeletedV2Event,
}

/// `corehr.job_family.deleted_v2` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrJobFamilyDeletedV2Event {
    /// 被删除序列的 ID。
    #[serde(default)]
    pub job_family_id: Option<String>,
}

/// `corehr.job_family.updated_v2` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrJobFamilyUpdatedV2 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: CorehrJobFamilyUpdatedV2Event,
}

/// `corehr.job_family.updated_v2` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrJobFamilyUpdatedV2Event {
    /// 序列ID。;- 调用[【查询单个序列】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/job_family/get)、[【通过序列 ID 批量查询序列信息】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/job_family/batch_get)接口返回序列详细信息
    #[serde(default)]
    pub job_family_id: Option<String>,
    /// 发生变更的字段
    #[serde(default)]
    pub field_changes: Vec<String>,
}

/// `corehr.job_grade.created_v2` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrJobGradeCreatedV2 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: CorehrJobGradeCreatedV2Event,
}

/// `corehr.job_grade.created_v2` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrJobGradeCreatedV2Event {
    /// 职等ID。;- 调用[【批量查询职等信息】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/job_grade/query)接口返回职等详细信息
    #[serde(default)]
    pub job_grade_id: Option<String>,
}

/// `corehr.job_grade.deleted_v2` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrJobGradeDeletedV2 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: CorehrJobGradeDeletedV2Event,
}

/// `corehr.job_grade.deleted_v2` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrJobGradeDeletedV2Event {
    /// 被删除职等的 ID
    #[serde(default)]
    pub job_grade_id: Option<String>,
}

/// `corehr.job_grade.updated_v2` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrJobGradeUpdatedV2 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: CorehrJobGradeUpdatedV2Event,
}

/// `corehr.job_grade.updated_v2` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrJobGradeUpdatedV2Event {
    /// 职等ID。;- 调用[【批量查询职等信息】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/job_grade/query)接口返回职等详细信息
    #[serde(default)]
    pub job_grade_id: Option<String>,
    /// 变更字段
    #[serde(default)]
    pub field_changes: Vec<String>,
}

/// `corehr.job_level.created_v2` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrJobLevelCreatedV2 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: CorehrJobLevelCreatedV2Event,
}

/// `corehr.job_level.created_v2` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrJobLevelCreatedV2Event {
    /// 职级ID。;- 调用[【查询单个职级】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/job_level/get)、[【通过职级 ID 批量获取职级信息】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/job_level/batch_get)接口返回职级详细信息
    #[serde(default)]
    pub job_level_id: Option<String>,
}

/// `corehr.job_level.deleted_v2` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrJobLevelDeletedV2 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: CorehrJobLevelDeletedV2Event,
}

/// `corehr.job_level.deleted_v2` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrJobLevelDeletedV2Event {
    /// 被删除职级的 ID。
    #[serde(default)]
    pub job_level_id: Option<String>,
}

/// `corehr.job_level.updated_v2` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrJobLevelUpdatedV2 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: CorehrJobLevelUpdatedV2Event,
}

/// `corehr.job_level.updated_v2` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrJobLevelUpdatedV2Event {
    /// 职级ID。;- 调用[【查询单个职级】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/job_level/get)、[【通过职级 ID 批量获取职级信息】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/job_level/batch_get)接口返回职级详细信息
    #[serde(default)]
    pub job_level_id: Option<String>,
    /// 发生变更的字段
    #[serde(default)]
    pub field_changes: Vec<String>,
}

/// `corehr.location.created_v2` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrLocationCreatedV2 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: CorehrLocationCreatedV2Event,
}

/// `corehr.location.created_v2` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrLocationCreatedV2Event {
    /// 地点ID。;- 调用[【查询单个地点】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/location/get)、[【通过地点 ID 批量获取地点信息】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/location/batch_get)接口返回地点详细信息
    #[serde(default)]
    pub location_id: Option<String>,
}

/// `corehr.location.deleted_v2` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrLocationDeletedV2 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: CorehrLocationDeletedV2Event,
}

/// `corehr.location.deleted_v2` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrLocationDeletedV2Event {
    /// 被删除地点的 ID。
    #[serde(default)]
    pub location_id: Option<String>,
}

/// `corehr.location.updated_v2` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrLocationUpdatedV2 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: CorehrLocationUpdatedV2Event,
}

/// `corehr.location.updated_v2` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrLocationUpdatedV2Event {
    /// 地点ID。;- 调用[【查询单个地点】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/location/get)、[【通过地点 ID 批量获取地点信息】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/location/batch_get)接口返回地点详细信息
    #[serde(default)]
    pub location_id: Option<String>,
    /// 发生变更的字段
    #[serde(default)]
    pub field_changes: Vec<String>,
    /// 子变更事件
    #[serde(default)]
    pub sub_events: Vec<OrganizationDomainEventData>,
}

/// `corehr.offboarding.checklist_updated_v2` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrOffboardingChecklistUpdatedV2 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: CorehrOffboardingChecklistUpdatedV2Event,
}

/// `corehr.offboarding.checklist_updated_v2` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrOffboardingChecklistUpdatedV2Event {
    /// 离职员工雇佣ID，可通过[批量查询员工信息;](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/employee/batch_get)获取员工信息。
    #[serde(default)]
    pub employment_id: Option<String>,
    /// 离职员工的用户ID
    #[serde(default)]
    pub target_user_id: Option<UserId>,
    /// 离职记录ID，每条离职记录的唯一标识。
    #[serde(default)]
    pub offboarding_id: Option<String>,
    /// 离职流转流程的实例 ID，可用于[查询流程相关信息](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/process/list)，例如：作为[获取单个流程详情](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/process/list)的process_id查询流程详情。
    #[serde(default)]
    pub checklist_process_id: Option<String>,
    /// 离职流转状态
    #[serde(default)]
    pub checklist_status: Option<i64>,
}

/// `corehr.offboarding.status_updated_v2` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrOffboardingStatusUpdatedV2 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: CorehrOffboardingStatusUpdatedV2Event,
}

/// `corehr.offboarding.status_updated_v2` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrOffboardingStatusUpdatedV2Event {
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

/// `corehr.offboarding.updated_v2` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrOffboardingUpdatedV2 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: CorehrOffboardingUpdatedV2Event,
}

/// `corehr.offboarding.updated_v2` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrOffboardingUpdatedV2Event {
    /// 飞书人事的租户ID
    #[serde(default)]
    pub tenant_id: Option<String>,
    /// 离职记录ID，每条离职记录的唯一标识。
    #[serde(default)]
    pub offboarding_info_id: Option<String>,
    /// 离职申请流程的实例 ID，可用于[查询流程相关信息](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/process/list)，例如：作为[获取单个流程详情](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/process/list)的process_id查询流程详情。
    #[serde(default)]
    pub process_id: Option<String>,
    /// 离职流转流程的实例ID，可用于[查询流程相关信息](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/process/list)，例如：作为[获取单个流程详情](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/process/list)的process_id查询流程详情。
    #[serde(default)]
    pub checklist_process_id: Option<String>,
    /// 离职员工雇佣ID，可通过[批量查询员工信息;](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/employee/batch_get)获取员工信息。
    #[serde(default)]
    pub employment_id: Option<String>,
    /// 操作人雇佣ID，可通过[批量查询员工信息;](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/employee/batch_get)获取员工信息。
    #[serde(default)]
    pub operator: Option<String>,
    /// 变更后的离职状态
    #[serde(default)]
    pub status: Option<i64>,
    /// 变更后的离职手续办理状态
    #[serde(default)]
    pub checklist_status: Option<i64>,
    /// 更新时间毫秒时间戳
    #[serde(default, deserialize_with = "super::flex_time::de_opt_string")]
    pub updated_time: Option<String>,
    /// 变更离职信息字段的唯一标识列表
    #[serde(default)]
    pub updated_fields: Vec<String>,
    /// 离职员工的用户 ID
    #[serde(default)]
    pub target_user_id: Option<UserId>,
}

/// `corehr.pathway.created_v2` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrPathwayCreatedV2 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: CorehrPathwayCreatedV2Event,
}

/// `corehr.pathway.created_v2` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrPathwayCreatedV2Event {
    /// 通道ID
    #[serde(default)]
    pub pathway_id: Option<String>,
}

/// `corehr.pathway.deleted_v2` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrPathwayDeletedV2 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: CorehrPathwayDeletedV2Event,
}

/// `corehr.pathway.deleted_v2` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrPathwayDeletedV2Event {
    /// 通道ID
    #[serde(default)]
    pub pathway_id: Option<String>,
}

/// `corehr.pathway.updated_v2` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrPathwayUpdatedV2 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: CorehrPathwayUpdatedV2Event,
}

/// `corehr.pathway.updated_v2` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrPathwayUpdatedV2Event {
    /// 通道 ID
    #[serde(default)]
    pub pathway_id: Option<String>,
    /// 发生变更的字段
    #[serde(default)]
    pub field_changes: Vec<String>,
}

/// `corehr.position.created_v2` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrPositionCreatedV2 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: CorehrPositionCreatedV2Event,
}

/// `corehr.position.created_v2` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrPositionCreatedV2Event {
    /// 岗位ID，可通过[查询岗位](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/position/query)接口去获取岗位详细信息
    #[serde(default)]
    pub position_id: Option<String>,
}

/// `corehr.position.deleted_v2` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrPositionDeletedV2 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: CorehrPositionDeletedV2Event,
}

/// `corehr.position.deleted_v2` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrPositionDeletedV2Event {
    /// 岗位ID
    #[serde(default)]
    pub position_id: Option<String>,
}

/// `corehr.position.updated_v2` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrPositionUpdatedV2 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: CorehrPositionUpdatedV2Event,
}

/// `corehr.position.updated_v2` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrPositionUpdatedV2Event {
    /// 岗位ID，可通过[查询岗位](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/position/query)接口去获取岗位详细信息
    #[serde(default)]
    pub position_id: Option<String>,
    /// 发生变更的字段，包括以下预置字段及自定义字段：;- "name"：名称;- "code"：编码;- "active"：状态;- "department"：所属部门;- "cost_center"：岗位默认成本中心;- "job"：职务;- "job_family"：序列;- "job_level"：职级;- "job_grade"：职等;- "work_location"：工作地点;- "employee_type"：人员类型;- "working_hours_type"：工时制度;- "direct_leader"：直属上级;- "dotted_line_leader"：虚线上级;- "is_key_position"：是否关键岗位;- "description"：描述;- "effective_time"：版本生效日期;- "expiration_time"：版本过期时间;;自定义字段详细见[获取自定义字段列表](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/custom_field/query)
    #[serde(default)]
    pub field_changes: Vec<String>,
}

/// `corehr.pre_hire.onboarding_task_changed_v2` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrPreHireOnboardingTaskChangedV2 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: CorehrPreHireOnboardingTaskChangedV2Event,
}

/// `corehr.pre_hire.onboarding_task_changed_v2` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrPreHireOnboardingTaskChangedV2Event {
    /// 飞书人事租户ID
    #[serde(default)]
    pub tenant_id: Option<String>,
    /// 待入职ID
    #[serde(default)]
    pub pre_hire_id: Option<String>,
    /// 入职任务状态变更
    #[serde(default)]
    pub onboarding_task_changes: Vec<OnboardingTaskChange>,
    /// 入职流程状态变更;- 当流程状态无变更，仅有任务状态变更时，此字段的会返回空
    #[serde(default)]
    pub onboarding_flow_change: Option<OnboardingFlowChange>,
    /// 待入职流程ID
    #[serde(default)]
    pub onboarding_flow_id: Option<String>,
    /// 流程信息
    #[serde(default)]
    pub flow_info: Option<OnboardingFlow>,
}

/// `corehr.probation.updated_v2` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrProbationUpdatedV2 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: CorehrProbationUpdatedV2Event,
}

/// `corehr.probation.updated_v2` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrProbationUpdatedV2Event {
    /// 雇佣 ID，可通过[【搜索员工信息】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/employee/search)接口获取
    #[serde(default)]
    pub employment_id: Option<String>,
    /// 试用期状态
    #[serde(default)]
    pub probation_status: Option<String>,
    /// 试用期实际结束日期，格式："YYYY-MM-DD"
    #[serde(default)]
    pub actual_probation_end_date: Option<String>,
}

/// `corehr.process.updated_v2` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrProcessUpdatedV2 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: CorehrProcessUpdatedV2Event,
}

/// `corehr.process.updated_v2` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrProcessUpdatedV2Event {
    /// 流程运行实例 id，详细信息可通过[获取单个流程详情](https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/process/get)获取
    #[serde(default)]
    pub process_id: Option<String>,
    /// 变更后状态
    #[serde(default)]
    pub status: Option<i64>,
    /// 业务类型， ;详情请查看[接入指南](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/process-form_variable_data/access-guide)
    #[serde(default)]
    pub biz_type: Option<String>,
    /// 流程定义id。流程定义是流程的模板，由流程定义可以创建流程实例。;;**示例值**：;;"people_7023711013443944467_7382148112896872236"
    #[serde(default)]
    pub flow_definition_id: Option<String>,
    /// 流程属性
    #[serde(default)]
    pub properties: Option<i64>,
}

/// `corehr.process.approver.updated_v2` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrProcessApproverUpdatedV2 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: CorehrProcessApproverUpdatedV2Event,
}

/// `corehr.process.approver.updated_v2` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrProcessApproverUpdatedV2Event {
    /// 流程运行实例 id，详细信息可通过[获取单个流程详情](https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/process/get)获取
    #[serde(default)]
    pub process_id: Option<String>,
    /// 单据ID
    #[serde(default)]
    pub approver_id: Option<String>,
    /// 单据类型
    #[serde(default)]
    pub r#type: Option<i64>,
    /// 单据状态
    #[serde(default)]
    pub status: Option<i64>,
    /// 业务类型，详情请查看[接入指南](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/process-form_variable_data/access-guide)
    #[serde(default)]
    pub biz_type: Option<String>,
    /// 流程定义id
    #[serde(default)]
    pub flow_definition_id: Option<String>,
    /// 节点定义id
    #[serde(default)]
    pub node_definition_id: Option<String>,
    /// 节点id（已废弃，请使用node_id_str字段)
    #[serde(default)]
    pub node_id: Option<String>,
    /// 节点id
    #[serde(default)]
    pub node_id_str: Option<String>,
}

/// `corehr.process.cc.updated_v2` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrProcessCcUpdatedV2 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: CorehrProcessCcUpdatedV2Event,
}

/// `corehr.process.cc.updated_v2` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrProcessCcUpdatedV2Event {
    /// 流程运行实例 id，详细信息可通过[获取单个流程详情](https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/process/get)获取
    #[serde(default)]
    pub process_id: Option<String>,
    /// 抄送单据ID
    #[serde(default)]
    pub approver_id: Option<String>,
    /// 单据状态
    #[serde(default)]
    pub status: Option<i64>,
    /// 业务类型，详情请查看[接入指南](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/process-form_variable_data/access-guide)
    #[serde(default)]
    pub biz_type: Option<String>,
}

/// `corehr.process.node.updated_v2` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrProcessNodeUpdatedV2 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: CorehrProcessNodeUpdatedV2Event,
}

/// `corehr.process.node.updated_v2` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrProcessNodeUpdatedV2Event {
    /// 流程定义 id
    #[serde(default)]
    pub flow_definition_id: Option<String>,
    /// 节点定义 id
    #[serde(default)]
    pub node_definition_id: Option<String>,
    /// 流程运行实例 id，详细信息可通过[获取单个流程详情](https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/process/get)获取
    #[serde(default)]
    pub process_id: Option<String>,
    /// 节点运行实例 id
    #[serde(default)]
    pub process_node_id: Option<String>,
    /// 节点类型
    #[serde(default)]
    pub node_type: Option<i64>,
    /// 节点状态
    #[serde(default)]
    pub node_status: Option<i64>,
    /// 业务类型，详情请查看[接入指南](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/process-form_variable_data/access-guide);
    #[serde(default)]
    pub biz_type: Option<String>,
}

/// `corehr.process.status.update_v2` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrProcessStatusUpdateV2 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: CorehrProcessStatusUpdateV2Event,
}

/// `corehr.process.status.update_v2` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrProcessStatusUpdateV2Event {
    /// 流程运行实例 id，详细信息可通过[获取单个流程详情](https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/process/get)获取
    #[serde(default)]
    pub process_id: Option<String>,
    /// 变更后状态
    #[serde(default)]
    pub status: Option<i64>,
    /// 业务类型， ;详情请查看[接入指南](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/process-form_variable_data/access-guide)
    #[serde(default)]
    pub biz_type: Option<String>,
    /// 流程定义id。流程定义是流程的模板，由流程定义可以创建流程实例。;;**示例值**：;;"people_7023711013443944467_7382148112896872236"
    #[serde(default)]
    pub flow_definition_id: Option<String>,
    /// 流程属性
    #[serde(default)]
    pub properties: Option<i64>,
}

/// `corehr.process_comment_info.updated_v2` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrProcessCommentInfoUpdatedV2 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: CorehrProcessCommentInfoUpdatedV2Event,
}

/// `corehr.process_comment_info.updated_v2` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrProcessCommentInfoUpdatedV2Event {
    /// 流程运行实例id
    #[serde(default)]
    pub process_id: Option<String>,
    /// 流程下评论的唯一id
    #[serde(default)]
    pub comment_id: Option<String>,
    /// 评论事件的类型
    #[serde(default)]
    pub r#type: Option<i64>,
}

/// `corehr.signature_file.status_updated_v2` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrSignatureFileStatusUpdatedV2 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: CorehrSignatureFileStatusUpdatedV2Event,
}

/// `corehr.signature_file.status_updated_v2` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorehrSignatureFileStatusUpdatedV2Event {
    /// 发生状态变更的电子签文件id，[【查询电子签文件详情】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/signature_file/query)
    #[serde(default)]
    pub signature_file_id: Option<String>,
    /// 变更前的电子签文件状态，枚举值可通过文档[【飞书人事枚举常量】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/feishu-people-enum-constant)电子签文件状态（signature_file_state）枚举定义部分获得
    #[serde(default)]
    pub before_status: Option<String>,
    /// 变更后的电子签文件状态，枚举值可通过文档[【飞书人事枚举常量】](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/feishu-people-enum-constant)电子签文件状态（signature_file_state）枚举定义部分获得
    #[serde(default)]
    pub after_status: Option<String>,
    /// 业务id, 由接入业务自行维护,可以通过该流程id获取该流程下的签署文件信息list
    #[serde(default)]
    pub biz_process_id: Option<String>,
}

/// 事件嵌套类型（Go `OrganizationDomainEventData`）。
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OrganizationDomainEventData {
    /// 变更实体的ID
    #[serde(default)]
    pub id: Option<String>,
    /// 变更实体标识，ObjAPIName
    #[serde(default)]
    pub entity: Option<String>,
    /// 聚合实体标识，父对象 ObjAPIName
    #[serde(default)]
    pub agg_entity: Option<String>,
    /// 聚合实体ID, 父对象ID
    #[serde(default)]
    pub agg_entity_id: Option<String>,
    /// 所属聚合实体字段， FieldAPIName
    #[serde(default)]
    pub agg_entity_field: Option<String>,
    /// 操作类型
    #[serde(default)]
    pub opt_type: Option<i64>,
    /// 变更字段列表
    #[serde(default)]
    pub field_changes: Vec<String>,
}

/// 事件嵌套类型（Go `EmployeeDomainEventData`）。
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EmployeeDomainEventData {
    /// 变更实体的 ID;- 雇佣ID，个人信息ID;- 特殊：; - 任职记录版本ID：请使用[批量查询员工任职](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/employees-job_data/batch_get)获取任职记录，对照 version_id 字段查找对象
    #[serde(default)]
    pub id: Option<String>,
    /// 变更实体标识，如：;- job_data: 任职信息;- person：个人信息;- employment：雇佣信息;- email: 邮箱;- person_name：姓名
    #[serde(default)]
    pub entity: Option<String>,
    /// 聚合实体标识，如：;- job_data: 任职信息;- person：个人信息;- employment：雇佣信息
    #[serde(default)]
    pub agg_entity: Option<String>,
    /// 聚合实体 ID，父对象 ID，如：;- person：个人信息 ID;- employment：雇佣 ID
    #[serde(default)]
    pub agg_entity_id: Option<String>,
    /// 操作类型
    #[serde(default)]
    pub opt_type: Option<i64>,
    /// 字段标识，ApiName
    #[serde(default)]
    pub fields: Vec<String>,
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

/// 事件嵌套类型（Go `OnboardingTaskChange`）。
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OnboardingTaskChange {
    /// 变更后任务状态
    #[serde(default)]
    pub after_status: Option<String>,
    /// 任务标识码;;如果是系统内置的任务，标识码与名称对应关系如下：;;其中『创建账户SSO』为隐藏的任务节点，在『个人信息』前自动执行;;| 名称 | task code |;| ---- | --- |;| 职位信息 | 1 |;| 个人信息 | 2 |;|创建账户SSO|3|;|签到| 4|;|签署入职文件|9|;;;如果标识码不在上面，说明是自定义任务节点，如：3095697a-065f-4627-a47c-46fe958a6754，名称的获取方式如下：;- 通过pre_hire_id调用[搜索待入职人员信息](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/pre_hire/search)接口或[查询待入职](https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/corehr-v2/pre_hire/query);- 查询字段fields中添加onboarding_info.onboarding_task_list;- 查询后返回的onboarding_task_list结构体中则包含code和名字的对应关系，示例如下;```;{; "code": 0,; "data": {; "has_more": false,; "items": \[; {; "onboarding_info": {; "onboarding_task_list": \[; {; "task_code": "2",; "task_name": "填写个人信息",; "task_status": "in_progress"; },; {; "task_code": "3095697a-065f-4627-a47c-46fe958a6754",; "task_name": "修改入职日期",; "task_status": "uninitialized"; },; {; "task_code": "d37b9d7c-232d-4a55-98fa-541318234ede",; "task_name": "工签补充任务",; "task_status": "uninitialized"; }; \]; },; "pre_hire_id": "7186491930107561516"; }; \]; },; "msg": "success";};```;
    #[serde(default)]
    pub task_code: Option<String>,
}

/// 事件嵌套类型（Go `OnboardingFlowChange`）。
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OnboardingFlowChange {
    /// 入职流程状态变更
    #[serde(default)]
    pub after_status: Option<String>,
}

/// 事件嵌套类型（Go `OnboardingFlow`）。
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OnboardingFlow {
    /// 流程id
    #[serde(default)]
    pub id: Option<String>,
    /// 流程名称
    #[serde(default)]
    pub name: Option<I18nV2>,
}

/// 事件嵌套类型（Go `I18nV2`）。
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct I18nV2 {
    /// zh-CN
    #[serde(default)]
    pub zh_cn: Option<String>,
    /// en-US
    #[serde(default)]
    pub en_us: Option<String>,
}

typed_events! {
    /// `corehr.approval_groups.updated_v2` typed 处理器。
    ///
    /// 官方没有替代事件。字段 `approval_group_status` 不推荐，请读 `approval_group_status_v2`。
    /// 本处理器保留。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.approval_groups.updated_v2" | COREHR_APPROVAL_GROUPS_UPDATED_V2 | CorehrApprovalGroupsUpdatedV2 | CorehrApprovalGroupsUpdatedV2Handler | register_corehr_approval_groups_updated_v2;
    /// `corehr.company.created_v2` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.company.created_v2" | COREHR_COMPANY_CREATED_V2 | CorehrCompanyCreatedV2 | CorehrCompanyCreatedV2Handler | register_corehr_company_created_v2;
    /// `corehr.company.deleted_v2` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.company.deleted_v2" | COREHR_COMPANY_DELETED_V2 | CorehrCompanyDeletedV2 | CorehrCompanyDeletedV2Handler | register_corehr_company_deleted_v2;
    /// `corehr.company.updated_v2` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.company.updated_v2" | COREHR_COMPANY_UPDATED_V2 | CorehrCompanyUpdatedV2 | CorehrCompanyUpdatedV2Handler | register_corehr_company_updated_v2;
    /// `corehr.cost_center.created_v2` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.cost_center.created_v2" | COREHR_COST_CENTER_CREATED_V2 | CorehrCostCenterCreatedV2 | CorehrCostCenterCreatedV2Handler | register_corehr_cost_center_created_v2;
    /// `corehr.cost_center.deleted_v2` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.cost_center.deleted_v2" | COREHR_COST_CENTER_DELETED_V2 | CorehrCostCenterDeletedV2 | CorehrCostCenterDeletedV2Handler | register_corehr_cost_center_deleted_v2;
    /// `corehr.cost_center.updated_v2` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.cost_center.updated_v2" | COREHR_COST_CENTER_UPDATED_V2 | CorehrCostCenterUpdatedV2 | CorehrCostCenterUpdatedV2Handler | register_corehr_cost_center_updated_v2;
    /// `corehr.custom_org.created_v2` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.custom_org.created_v2" | COREHR_CUSTOM_ORG_CREATED_V2 | CorehrCustomOrgCreatedV2 | CorehrCustomOrgCreatedV2Handler | register_corehr_custom_org_created_v2;
    /// `corehr.custom_org.deleted_v2` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.custom_org.deleted_v2" | COREHR_CUSTOM_ORG_DELETED_V2 | CorehrCustomOrgDeletedV2 | CorehrCustomOrgDeletedV2Handler | register_corehr_custom_org_deleted_v2;
    /// `corehr.custom_org.updated_v2` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.custom_org.updated_v2" | COREHR_CUSTOM_ORG_UPDATED_V2 | CorehrCustomOrgUpdatedV2 | CorehrCustomOrgUpdatedV2Handler | register_corehr_custom_org_updated_v2;
    /// `corehr.department.created_v2` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.department.created_v2" | COREHR_DEPARTMENT_CREATED_V2 | CorehrDepartmentCreatedV2 | CorehrDepartmentCreatedV2Handler | register_corehr_department_created_v2;
    /// `corehr.department.updated_v2` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.department.updated_v2" | COREHR_DEPARTMENT_UPDATED_V2 | CorehrDepartmentUpdatedV2 | CorehrDepartmentUpdatedV2Handler | register_corehr_department_updated_v2;
    /// `corehr.employee.domain_event_v2` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.employee.domain_event_v2" | COREHR_EMPLOYEE_DOMAIN_EVENT_V2 | CorehrEmployeeDomainEventV2 | CorehrEmployeeDomainEventV2Handler | register_corehr_employee_domain_event_v2;
    /// `corehr.job_change.status_updated_v2` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.job_change.status_updated_v2" | COREHR_JOB_CHANGE_STATUS_UPDATED_V2 | CorehrJobChangeStatusUpdatedV2 | CorehrJobChangeStatusUpdatedV2Handler | register_corehr_job_change_status_updated_v2;
    /// `corehr.job_change.updated_v2` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.job_change.updated_v2" | COREHR_JOB_CHANGE_UPDATED_V2 | CorehrJobChangeUpdatedV2 | CorehrJobChangeUpdatedV2Handler | register_corehr_job_change_updated_v2;
    /// `corehr.job_family.created_v2` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.job_family.created_v2" | COREHR_JOB_FAMILY_CREATED_V2 | CorehrJobFamilyCreatedV2 | CorehrJobFamilyCreatedV2Handler | register_corehr_job_family_created_v2;
    /// `corehr.job_family.deleted_v2` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.job_family.deleted_v2" | COREHR_JOB_FAMILY_DELETED_V2 | CorehrJobFamilyDeletedV2 | CorehrJobFamilyDeletedV2Handler | register_corehr_job_family_deleted_v2;
    /// `corehr.job_family.updated_v2` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.job_family.updated_v2" | COREHR_JOB_FAMILY_UPDATED_V2 | CorehrJobFamilyUpdatedV2 | CorehrJobFamilyUpdatedV2Handler | register_corehr_job_family_updated_v2;
    /// `corehr.job_grade.created_v2` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.job_grade.created_v2" | COREHR_JOB_GRADE_CREATED_V2 | CorehrJobGradeCreatedV2 | CorehrJobGradeCreatedV2Handler | register_corehr_job_grade_created_v2;
    /// `corehr.job_grade.deleted_v2` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.job_grade.deleted_v2" | COREHR_JOB_GRADE_DELETED_V2 | CorehrJobGradeDeletedV2 | CorehrJobGradeDeletedV2Handler | register_corehr_job_grade_deleted_v2;
    /// `corehr.job_grade.updated_v2` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.job_grade.updated_v2" | COREHR_JOB_GRADE_UPDATED_V2 | CorehrJobGradeUpdatedV2 | CorehrJobGradeUpdatedV2Handler | register_corehr_job_grade_updated_v2;
    /// `corehr.job_level.created_v2` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.job_level.created_v2" | COREHR_JOB_LEVEL_CREATED_V2 | CorehrJobLevelCreatedV2 | CorehrJobLevelCreatedV2Handler | register_corehr_job_level_created_v2;
    /// `corehr.job_level.deleted_v2` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.job_level.deleted_v2" | COREHR_JOB_LEVEL_DELETED_V2 | CorehrJobLevelDeletedV2 | CorehrJobLevelDeletedV2Handler | register_corehr_job_level_deleted_v2;
    /// `corehr.job_level.updated_v2` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.job_level.updated_v2" | COREHR_JOB_LEVEL_UPDATED_V2 | CorehrJobLevelUpdatedV2 | CorehrJobLevelUpdatedV2Handler | register_corehr_job_level_updated_v2;
    /// `corehr.location.created_v2` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.location.created_v2" | COREHR_LOCATION_CREATED_V2 | CorehrLocationCreatedV2 | CorehrLocationCreatedV2Handler | register_corehr_location_created_v2;
    /// `corehr.location.deleted_v2` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.location.deleted_v2" | COREHR_LOCATION_DELETED_V2 | CorehrLocationDeletedV2 | CorehrLocationDeletedV2Handler | register_corehr_location_deleted_v2;
    /// `corehr.location.updated_v2` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.location.updated_v2" | COREHR_LOCATION_UPDATED_V2 | CorehrLocationUpdatedV2 | CorehrLocationUpdatedV2Handler | register_corehr_location_updated_v2;
    /// `corehr.offboarding.checklist_updated_v2` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.offboarding.checklist_updated_v2" | COREHR_OFFBOARDING_CHECKLIST_UPDATED_V2 | CorehrOffboardingChecklistUpdatedV2 | CorehrOffboardingChecklistUpdatedV2Handler | register_corehr_offboarding_checklist_updated_v2;
    /// `corehr.offboarding.status_updated_v2` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.offboarding.status_updated_v2" | COREHR_OFFBOARDING_STATUS_UPDATED_V2 | CorehrOffboardingStatusUpdatedV2 | CorehrOffboardingStatusUpdatedV2Handler | register_corehr_offboarding_status_updated_v2;
    /// `corehr.offboarding.updated_v2` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.offboarding.updated_v2" | COREHR_OFFBOARDING_UPDATED_V2 | CorehrOffboardingUpdatedV2 | CorehrOffboardingUpdatedV2Handler | register_corehr_offboarding_updated_v2;
    /// `corehr.pathway.created_v2` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.pathway.created_v2" | COREHR_PATHWAY_CREATED_V2 | CorehrPathwayCreatedV2 | CorehrPathwayCreatedV2Handler | register_corehr_pathway_created_v2;
    /// `corehr.pathway.deleted_v2` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.pathway.deleted_v2" | COREHR_PATHWAY_DELETED_V2 | CorehrPathwayDeletedV2 | CorehrPathwayDeletedV2Handler | register_corehr_pathway_deleted_v2;
    /// `corehr.pathway.updated_v2` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.pathway.updated_v2" | COREHR_PATHWAY_UPDATED_V2 | CorehrPathwayUpdatedV2 | CorehrPathwayUpdatedV2Handler | register_corehr_pathway_updated_v2;
    /// `corehr.position.created_v2` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.position.created_v2" | COREHR_POSITION_CREATED_V2 | CorehrPositionCreatedV2 | CorehrPositionCreatedV2Handler | register_corehr_position_created_v2;
    /// `corehr.position.deleted_v2` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.position.deleted_v2" | COREHR_POSITION_DELETED_V2 | CorehrPositionDeletedV2 | CorehrPositionDeletedV2Handler | register_corehr_position_deleted_v2;
    /// `corehr.position.updated_v2` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.position.updated_v2" | COREHR_POSITION_UPDATED_V2 | CorehrPositionUpdatedV2 | CorehrPositionUpdatedV2Handler | register_corehr_position_updated_v2;
    /// `corehr.pre_hire.onboarding_task_changed_v2` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.pre_hire.onboarding_task_changed_v2" | COREHR_PRE_HIRE_ONBOARDING_TASK_CHANGED_V2 | CorehrPreHireOnboardingTaskChangedV2 | CorehrPreHireOnboardingTaskChangedV2Handler | register_corehr_pre_hire_onboarding_task_changed_v2;
    /// `corehr.probation.updated_v2` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.probation.updated_v2" | COREHR_PROBATION_UPDATED_V2 | CorehrProbationUpdatedV2 | CorehrProbationUpdatedV2Handler | register_corehr_probation_updated_v2;
    /// `corehr.process.updated_v2` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.process.updated_v2" | COREHR_PROCESS_UPDATED_V2 | CorehrProcessUpdatedV2 | CorehrProcessUpdatedV2Handler | register_corehr_process_updated_v2;
    /// `corehr.process.approver.updated_v2` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.process.approver.updated_v2" | COREHR_PROCESS_APPROVER_UPDATED_V2 | CorehrProcessApproverUpdatedV2 | CorehrProcessApproverUpdatedV2Handler | register_corehr_process_approver_updated_v2;
    /// `corehr.process.cc.updated_v2` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.process.cc.updated_v2" | COREHR_PROCESS_CC_UPDATED_V2 | CorehrProcessCcUpdatedV2 | CorehrProcessCcUpdatedV2Handler | register_corehr_process_cc_updated_v2;
    /// `corehr.process.node.updated_v2` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.process.node.updated_v2" | COREHR_PROCESS_NODE_UPDATED_V2 | CorehrProcessNodeUpdatedV2 | CorehrProcessNodeUpdatedV2Handler | register_corehr_process_node_updated_v2;
    /// `corehr.process.status.update_v2` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.process.status.update_v2" | COREHR_PROCESS_STATUS_UPDATE_V2 | CorehrProcessStatusUpdateV2 | CorehrProcessStatusUpdateV2Handler | register_corehr_process_status_update_v2;
    /// `corehr.process_comment_info.updated_v2` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.process_comment_info.updated_v2" | COREHR_PROCESS_COMMENT_INFO_UPDATED_V2 | CorehrProcessCommentInfoUpdatedV2 | CorehrProcessCommentInfoUpdatedV2Handler | register_corehr_process_comment_info_updated_v2;
    /// `corehr.signature_file.status_updated_v2` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "corehr.signature_file.status_updated_v2" | COREHR_SIGNATURE_FILE_STATUS_UPDATED_V2 | CorehrSignatureFileStatusUpdatedV2 | CorehrSignatureFileStatusUpdatedV2Handler | register_corehr_signature_file_status_updated_v2;
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use super::*;
    use crate::ws_client::EventDispatcherHandler;

    const COREHR_APPROVAL_GROUPS_UPDATED_V2_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "corehr.approval_groups.updated_v2",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "approval_group_id": "approval_group_id_str",
    "process_id": "process_id_str",
    "approval_group_status": 42,
    "topic": "topic_str",
    "adjust_reason": "adjust_reason_str",
    "effective_date": "effective_date_str",
    "created_by": "created_by_str",
    "draft_id": "draft_id_str",
    "draft_status": 42,
    "approval_group_status_v2": 42
  }
}"#;

    const COREHR_COMPANY_CREATED_V2_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "corehr.company.created_v2",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "company_id": "company_id_str"
  }
}"#;

    const COREHR_COMPANY_DELETED_V2_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "corehr.company.deleted_v2",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "company_id": "company_id_str"
  }
}"#;

    const COREHR_COMPANY_UPDATED_V2_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "corehr.company.updated_v2",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "company_id": "company_id_str",
    "field_changes": [
      "field_changes_str"
    ],
    "sub_events": [
      {
        "id": "id_str",
        "entity": "entity_str",
        "agg_entity": "agg_entity_str",
        "agg_entity_id": "agg_entity_id_str",
        "agg_entity_field": "agg_entity_field_str",
        "opt_type": 42,
        "field_changes": [
          "field_changes_str"
        ]
      }
    ]
  }
}"#;

    const COREHR_COST_CENTER_CREATED_V2_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "corehr.cost_center.created_v2",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "cost_center_id": "cost_center_id_str"
  }
}"#;

    const COREHR_COST_CENTER_DELETED_V2_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "corehr.cost_center.deleted_v2",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "cost_center_id": "cost_center_id_str"
  }
}"#;

    const COREHR_COST_CENTER_UPDATED_V2_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "corehr.cost_center.updated_v2",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "cost_center_id": "cost_center_id_str",
    "field_changes": [
      "field_changes_str"
    ]
  }
}"#;

    const COREHR_CUSTOM_ORG_CREATED_V2_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "corehr.custom_org.created_v2",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "org_id": "org_id_str",
    "object_api_name": "object_api_name_str"
  }
}"#;

    const COREHR_CUSTOM_ORG_DELETED_V2_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "corehr.custom_org.deleted_v2",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "org_id": "org_id_str",
    "object_api_name": "object_api_name_str"
  }
}"#;

    const COREHR_CUSTOM_ORG_UPDATED_V2_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "corehr.custom_org.updated_v2",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "org_id": "org_id_str",
    "field_changes": [
      "field_changes_str"
    ],
    "object_api_name": "object_api_name_str"
  }
}"#;

    const COREHR_DEPARTMENT_CREATED_V2_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "corehr.department.created_v2",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "department_id": "department_id_str"
  }
}"#;

    const COREHR_DEPARTMENT_UPDATED_V2_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "corehr.department.updated_v2",
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

    const COREHR_EMPLOYEE_DOMAIN_EVENT_V2_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "corehr.employee.domain_event_v2",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "event_type": 42,
    "sub_event_type": 42,
    "operator_user_id": "operator_user_id_str",
    "opt_scene": "opt_scene_str",
    "opt_desc": "opt_desc_str",
    "opt_time": "opt_time_str",
    "opt_id": "opt_id_str",
    "employment_id": "employment_id_str",
    "data": [
      {
        "id": "id_str",
        "entity": "entity_str",
        "agg_entity": "agg_entity_str",
        "agg_entity_id": "agg_entity_id_str",
        "opt_type": 42,
        "fields": [
          "fields_str"
        ]
      }
    ]
  }
}"#;

    const COREHR_JOB_CHANGE_STATUS_UPDATED_V2_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "corehr.job_change.status_updated_v2",
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
    "original_status": 42,
    "transfer_key": "transfer_key_str",
    "details_of_job_status_change": [
      "details_of_job_status_change_str"
    ]
  }
}"#;

    const COREHR_JOB_CHANGE_UPDATED_V2_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "corehr.job_change.updated_v2",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "employment_id": "employment_id_str",
    "tenant_id": "tenant_id_str",
    "process_id": "process_id_str",
    "initiator": "initiator_str",
    "operator": "operator_str",
    "updated_time": "updated_time_str",
    "job_change_id": "job_change_id_str",
    "status": 42,
    "operate_reason": "operate_reason_str",
    "transfer_type": 42,
    "updated_fields": [
      "updated_fields_str"
    ],
    "transform_type": "transform_type_str",
    "transform_reason": "transform_reason_str"
  }
}"#;

    const COREHR_JOB_FAMILY_CREATED_V2_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "corehr.job_family.created_v2",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "job_family_id": "job_family_id_str"
  }
}"#;

    const COREHR_JOB_FAMILY_DELETED_V2_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "corehr.job_family.deleted_v2",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "job_family_id": "job_family_id_str"
  }
}"#;

    const COREHR_JOB_FAMILY_UPDATED_V2_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "corehr.job_family.updated_v2",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "job_family_id": "job_family_id_str",
    "field_changes": [
      "field_changes_str"
    ]
  }
}"#;

    const COREHR_JOB_GRADE_CREATED_V2_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "corehr.job_grade.created_v2",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "job_grade_id": "job_grade_id_str"
  }
}"#;

    const COREHR_JOB_GRADE_DELETED_V2_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "corehr.job_grade.deleted_v2",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "job_grade_id": "job_grade_id_str"
  }
}"#;

    const COREHR_JOB_GRADE_UPDATED_V2_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "corehr.job_grade.updated_v2",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "job_grade_id": "job_grade_id_str",
    "field_changes": [
      "field_changes_str"
    ]
  }
}"#;

    const COREHR_JOB_LEVEL_CREATED_V2_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "corehr.job_level.created_v2",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "job_level_id": "job_level_id_str"
  }
}"#;

    const COREHR_JOB_LEVEL_DELETED_V2_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "corehr.job_level.deleted_v2",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "job_level_id": "job_level_id_str"
  }
}"#;

    const COREHR_JOB_LEVEL_UPDATED_V2_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "corehr.job_level.updated_v2",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "job_level_id": "job_level_id_str",
    "field_changes": [
      "field_changes_str"
    ]
  }
}"#;

    const COREHR_LOCATION_CREATED_V2_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "corehr.location.created_v2",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "location_id": "location_id_str"
  }
}"#;

    const COREHR_LOCATION_DELETED_V2_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "corehr.location.deleted_v2",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "location_id": "location_id_str"
  }
}"#;

    const COREHR_LOCATION_UPDATED_V2_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "corehr.location.updated_v2",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "location_id": "location_id_str",
    "field_changes": [
      "field_changes_str"
    ],
    "sub_events": [
      {
        "id": "id_str",
        "entity": "entity_str",
        "agg_entity": "agg_entity_str",
        "agg_entity_id": "agg_entity_id_str",
        "agg_entity_field": "agg_entity_field_str",
        "opt_type": 42,
        "field_changes": [
          "field_changes_str"
        ]
      }
    ]
  }
}"#;

    const COREHR_OFFBOARDING_CHECKLIST_UPDATED_V2_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "corehr.offboarding.checklist_updated_v2",
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
    "checklist_process_id": "checklist_process_id_str",
    "checklist_status": 42
  }
}"#;

    const COREHR_OFFBOARDING_STATUS_UPDATED_V2_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "corehr.offboarding.status_updated_v2",
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

    const COREHR_OFFBOARDING_UPDATED_V2_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "corehr.offboarding.updated_v2",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "tenant_id": "tenant_id_str",
    "offboarding_info_id": "offboarding_info_id_str",
    "process_id": "process_id_str",
    "checklist_process_id": "checklist_process_id_str",
    "employment_id": "employment_id_str",
    "operator": "operator_str",
    "status": 42,
    "checklist_status": 42,
    "updated_time": "updated_time_str",
    "updated_fields": [
      "updated_fields_str"
    ],
    "target_user_id": {
      "user_id": "user_id_str",
      "open_id": "open_id_str",
      "union_id": "union_id_str"
    }
  }
}"#;

    const COREHR_PATHWAY_CREATED_V2_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "corehr.pathway.created_v2",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "pathway_id": "pathway_id_str"
  }
}"#;

    const COREHR_PATHWAY_DELETED_V2_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "corehr.pathway.deleted_v2",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "pathway_id": "pathway_id_str"
  }
}"#;

    const COREHR_PATHWAY_UPDATED_V2_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "corehr.pathway.updated_v2",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "pathway_id": "pathway_id_str",
    "field_changes": [
      "field_changes_str"
    ]
  }
}"#;

    const COREHR_POSITION_CREATED_V2_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "corehr.position.created_v2",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "position_id": "position_id_str"
  }
}"#;

    const COREHR_POSITION_DELETED_V2_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "corehr.position.deleted_v2",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "position_id": "position_id_str"
  }
}"#;

    const COREHR_POSITION_UPDATED_V2_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "corehr.position.updated_v2",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "position_id": "position_id_str",
    "field_changes": [
      "field_changes_str"
    ]
  }
}"#;

    const COREHR_PRE_HIRE_ONBOARDING_TASK_CHANGED_V2_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "corehr.pre_hire.onboarding_task_changed_v2",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "tenant_id": "tenant_id_str",
    "pre_hire_id": "pre_hire_id_str",
    "onboarding_task_changes": [
      {
        "after_status": "after_status_str",
        "task_code": "task_code_str"
      }
    ],
    "onboarding_flow_change": {
      "after_status": "after_status_str"
    },
    "onboarding_flow_id": "onboarding_flow_id_str",
    "flow_info": {
      "id": "id_str",
      "name": {
        "zh_cn": "zh_cn_str",
        "en_us": "en_us_str"
      }
    }
  }
}"#;

    const COREHR_PROBATION_UPDATED_V2_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "corehr.probation.updated_v2",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "employment_id": "employment_id_str",
    "probation_status": "probation_status_str",
    "actual_probation_end_date": "actual_probation_end_date_str"
  }
}"#;

    const COREHR_PROCESS_UPDATED_V2_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "corehr.process.updated_v2",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "process_id": "process_id_str",
    "status": 42,
    "biz_type": "biz_type_str",
    "flow_definition_id": "flow_definition_id_str",
    "properties": 42
  }
}"#;

    const COREHR_PROCESS_APPROVER_UPDATED_V2_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "corehr.process.approver.updated_v2",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "process_id": "process_id_str",
    "approver_id": "approver_id_str",
    "type": 42,
    "status": 42,
    "biz_type": "biz_type_str",
    "flow_definition_id": "flow_definition_id_str",
    "node_definition_id": "node_definition_id_str",
    "node_id": "node_id_str",
    "node_id_str": "node_id_str_str"
  }
}"#;

    const COREHR_PROCESS_CC_UPDATED_V2_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "corehr.process.cc.updated_v2",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "process_id": "process_id_str",
    "approver_id": "approver_id_str",
    "status": 42,
    "biz_type": "biz_type_str"
  }
}"#;

    const COREHR_PROCESS_NODE_UPDATED_V2_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "corehr.process.node.updated_v2",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "flow_definition_id": "flow_definition_id_str",
    "node_definition_id": "node_definition_id_str",
    "process_id": "process_id_str",
    "process_node_id": "process_node_id_str",
    "node_type": 42,
    "node_status": 42,
    "biz_type": "biz_type_str"
  }
}"#;

    const COREHR_PROCESS_STATUS_UPDATE_V2_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "corehr.process.status.update_v2",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "process_id": "process_id_str",
    "status": 42,
    "biz_type": "biz_type_str",
    "flow_definition_id": "flow_definition_id_str",
    "properties": 42
  }
}"#;

    const COREHR_PROCESS_COMMENT_INFO_UPDATED_V2_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "corehr.process_comment_info.updated_v2",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "process_id": "process_id_str",
    "comment_id": "comment_id_str",
    "type": 42
  }
}"#;

    const COREHR_SIGNATURE_FILE_STATUS_UPDATED_V2_FIXTURE: &str = r#"{
  "schema": "2.0",
  "header": {
    "event_id": "evt_1",
    "event_type": "corehr.signature_file.status_updated_v2",
    "create_time": "1700000000000",
    "token": "t",
    "app_id": "cli_x",
    "tenant_key": "tk"
  },
  "event": {
    "signature_file_id": "signature_file_id_str",
    "before_status": "before_status_str",
    "after_status": "after_status_str",
    "biz_process_id": "biz_process_id_str"
  }
}"#;

    #[test]
    fn typed_corehr_approval_groups_updated_v2_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_corehr_approval_groups_updated_v2(
                move |event: CorehrApprovalGroupsUpdatedV2| {
                    assert_eq!(event.header.event_type, "corehr.approval_groups.updated_v2");
                    assert_eq!(
                        event.event.approval_group_id.as_deref(),
                        Some("approval_group_id_str")
                    );
                    assert_eq!(event.event.process_id.as_deref(), Some("process_id_str"));
                    assert_eq!(event.event.approval_group_status, Some(42));
                    *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                    Ok(())
                },
            )
            .expect("register")
            .build();
        handler
            .do_without_validation(COREHR_APPROVAL_GROUPS_UPDATED_V2_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_corehr_company_created_v2_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_corehr_company_created_v2(move |event: CorehrCompanyCreatedV2| {
                assert_eq!(event.header.event_type, "corehr.company.created_v2");
                assert_eq!(event.event.company_id.as_deref(), Some("company_id_str"));
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(COREHR_COMPANY_CREATED_V2_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_corehr_company_deleted_v2_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_corehr_company_deleted_v2(move |event: CorehrCompanyDeletedV2| {
                assert_eq!(event.header.event_type, "corehr.company.deleted_v2");
                assert_eq!(event.event.company_id.as_deref(), Some("company_id_str"));
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(COREHR_COMPANY_DELETED_V2_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_corehr_company_updated_v2_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_corehr_company_updated_v2(move |event: CorehrCompanyUpdatedV2| {
                assert_eq!(event.header.event_type, "corehr.company.updated_v2");
                assert_eq!(event.event.company_id.as_deref(), Some("company_id_str"));
                assert_eq!(event.event.field_changes[0].as_str(), "field_changes_str");
                assert_eq!(event.event.sub_events.len(), 1);
                assert_eq!(event.event.sub_events[0].id.as_deref(), Some("id_str"));
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(COREHR_COMPANY_UPDATED_V2_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_corehr_cost_center_created_v2_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_corehr_cost_center_created_v2(move |event: CorehrCostCenterCreatedV2| {
                assert_eq!(event.header.event_type, "corehr.cost_center.created_v2");
                assert_eq!(
                    event.event.cost_center_id.as_deref(),
                    Some("cost_center_id_str")
                );
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(COREHR_COST_CENTER_CREATED_V2_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_corehr_cost_center_deleted_v2_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_corehr_cost_center_deleted_v2(move |event: CorehrCostCenterDeletedV2| {
                assert_eq!(event.header.event_type, "corehr.cost_center.deleted_v2");
                assert_eq!(
                    event.event.cost_center_id.as_deref(),
                    Some("cost_center_id_str")
                );
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(COREHR_COST_CENTER_DELETED_V2_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_corehr_cost_center_updated_v2_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_corehr_cost_center_updated_v2(move |event: CorehrCostCenterUpdatedV2| {
                assert_eq!(event.header.event_type, "corehr.cost_center.updated_v2");
                assert_eq!(
                    event.event.cost_center_id.as_deref(),
                    Some("cost_center_id_str")
                );
                assert_eq!(event.event.field_changes[0].as_str(), "field_changes_str");
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(COREHR_COST_CENTER_UPDATED_V2_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_corehr_custom_org_created_v2_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_corehr_custom_org_created_v2(move |event: CorehrCustomOrgCreatedV2| {
                assert_eq!(event.header.event_type, "corehr.custom_org.created_v2");
                assert_eq!(event.event.org_id.as_deref(), Some("org_id_str"));
                assert_eq!(
                    event.event.object_api_name.as_deref(),
                    Some("object_api_name_str")
                );
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(COREHR_CUSTOM_ORG_CREATED_V2_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_corehr_custom_org_deleted_v2_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_corehr_custom_org_deleted_v2(move |event: CorehrCustomOrgDeletedV2| {
                assert_eq!(event.header.event_type, "corehr.custom_org.deleted_v2");
                assert_eq!(event.event.org_id.as_deref(), Some("org_id_str"));
                assert_eq!(
                    event.event.object_api_name.as_deref(),
                    Some("object_api_name_str")
                );
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(COREHR_CUSTOM_ORG_DELETED_V2_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_corehr_custom_org_updated_v2_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_corehr_custom_org_updated_v2(move |event: CorehrCustomOrgUpdatedV2| {
                assert_eq!(event.header.event_type, "corehr.custom_org.updated_v2");
                assert_eq!(event.event.org_id.as_deref(), Some("org_id_str"));
                assert_eq!(event.event.field_changes[0].as_str(), "field_changes_str");
                assert_eq!(
                    event.event.object_api_name.as_deref(),
                    Some("object_api_name_str")
                );
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(COREHR_CUSTOM_ORG_UPDATED_V2_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_corehr_department_created_v2_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_corehr_department_created_v2(move |event: CorehrDepartmentCreatedV2| {
                assert_eq!(event.header.event_type, "corehr.department.created_v2");
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
            .do_without_validation(COREHR_DEPARTMENT_CREATED_V2_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_corehr_department_updated_v2_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_corehr_department_updated_v2(move |event: CorehrDepartmentUpdatedV2| {
                assert_eq!(event.header.event_type, "corehr.department.updated_v2");
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
            .do_without_validation(COREHR_DEPARTMENT_UPDATED_V2_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_corehr_employee_domain_event_v2_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_corehr_employee_domain_event_v2(move |event: CorehrEmployeeDomainEventV2| {
                assert_eq!(event.header.event_type, "corehr.employee.domain_event_v2");
                assert_eq!(event.event.event_type, Some(42));
                assert_eq!(event.event.sub_event_type, Some(42));
                assert_eq!(
                    event.event.operator_user_id.as_deref(),
                    Some("operator_user_id_str")
                );
                assert_eq!(event.event.data.len(), 1);
                assert_eq!(event.event.data[0].id.as_deref(), Some("id_str"));
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(COREHR_EMPLOYEE_DOMAIN_EVENT_V2_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_corehr_job_change_status_updated_v2_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_corehr_job_change_status_updated_v2(
                move |event: CorehrJobChangeStatusUpdatedV2| {
                    assert_eq!(
                        event.header.event_type,
                        "corehr.job_change.status_updated_v2"
                    );
                    assert_eq!(
                        event.event.employment_id.as_deref(),
                        Some("employment_id_str")
                    );
                    let target_user_id =
                        event.event.target_user_id.as_ref().expect("target_user_id");
                    assert_eq!(target_user_id.user_id.as_deref(), Some("user_id_str"));
                    assert_eq!(
                        event.event.job_change_id.as_deref(),
                        Some("job_change_id_str")
                    );
                    assert_eq!(event.event.transfer_mode, Some(42));
                    *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                    Ok(())
                },
            )
            .expect("register")
            .build();
        handler
            .do_without_validation(COREHR_JOB_CHANGE_STATUS_UPDATED_V2_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_corehr_job_change_updated_v2_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_corehr_job_change_updated_v2(move |event: CorehrJobChangeUpdatedV2| {
                assert_eq!(event.header.event_type, "corehr.job_change.updated_v2");
                assert_eq!(
                    event.event.employment_id.as_deref(),
                    Some("employment_id_str")
                );
                assert_eq!(event.event.tenant_id.as_deref(), Some("tenant_id_str"));
                assert_eq!(event.event.process_id.as_deref(), Some("process_id_str"));
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(COREHR_JOB_CHANGE_UPDATED_V2_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_corehr_job_family_created_v2_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_corehr_job_family_created_v2(move |event: CorehrJobFamilyCreatedV2| {
                assert_eq!(event.header.event_type, "corehr.job_family.created_v2");
                assert_eq!(
                    event.event.job_family_id.as_deref(),
                    Some("job_family_id_str")
                );
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(COREHR_JOB_FAMILY_CREATED_V2_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_corehr_job_family_deleted_v2_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_corehr_job_family_deleted_v2(move |event: CorehrJobFamilyDeletedV2| {
                assert_eq!(event.header.event_type, "corehr.job_family.deleted_v2");
                assert_eq!(
                    event.event.job_family_id.as_deref(),
                    Some("job_family_id_str")
                );
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(COREHR_JOB_FAMILY_DELETED_V2_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_corehr_job_family_updated_v2_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_corehr_job_family_updated_v2(move |event: CorehrJobFamilyUpdatedV2| {
                assert_eq!(event.header.event_type, "corehr.job_family.updated_v2");
                assert_eq!(
                    event.event.job_family_id.as_deref(),
                    Some("job_family_id_str")
                );
                assert_eq!(event.event.field_changes[0].as_str(), "field_changes_str");
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(COREHR_JOB_FAMILY_UPDATED_V2_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_corehr_job_grade_created_v2_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_corehr_job_grade_created_v2(move |event: CorehrJobGradeCreatedV2| {
                assert_eq!(event.header.event_type, "corehr.job_grade.created_v2");
                assert_eq!(
                    event.event.job_grade_id.as_deref(),
                    Some("job_grade_id_str")
                );
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(COREHR_JOB_GRADE_CREATED_V2_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_corehr_job_grade_deleted_v2_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_corehr_job_grade_deleted_v2(move |event: CorehrJobGradeDeletedV2| {
                assert_eq!(event.header.event_type, "corehr.job_grade.deleted_v2");
                assert_eq!(
                    event.event.job_grade_id.as_deref(),
                    Some("job_grade_id_str")
                );
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(COREHR_JOB_GRADE_DELETED_V2_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_corehr_job_grade_updated_v2_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_corehr_job_grade_updated_v2(move |event: CorehrJobGradeUpdatedV2| {
                assert_eq!(event.header.event_type, "corehr.job_grade.updated_v2");
                assert_eq!(
                    event.event.job_grade_id.as_deref(),
                    Some("job_grade_id_str")
                );
                assert_eq!(event.event.field_changes[0].as_str(), "field_changes_str");
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(COREHR_JOB_GRADE_UPDATED_V2_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_corehr_job_level_created_v2_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_corehr_job_level_created_v2(move |event: CorehrJobLevelCreatedV2| {
                assert_eq!(event.header.event_type, "corehr.job_level.created_v2");
                assert_eq!(
                    event.event.job_level_id.as_deref(),
                    Some("job_level_id_str")
                );
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(COREHR_JOB_LEVEL_CREATED_V2_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_corehr_job_level_deleted_v2_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_corehr_job_level_deleted_v2(move |event: CorehrJobLevelDeletedV2| {
                assert_eq!(event.header.event_type, "corehr.job_level.deleted_v2");
                assert_eq!(
                    event.event.job_level_id.as_deref(),
                    Some("job_level_id_str")
                );
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(COREHR_JOB_LEVEL_DELETED_V2_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_corehr_job_level_updated_v2_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_corehr_job_level_updated_v2(move |event: CorehrJobLevelUpdatedV2| {
                assert_eq!(event.header.event_type, "corehr.job_level.updated_v2");
                assert_eq!(
                    event.event.job_level_id.as_deref(),
                    Some("job_level_id_str")
                );
                assert_eq!(event.event.field_changes[0].as_str(), "field_changes_str");
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(COREHR_JOB_LEVEL_UPDATED_V2_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_corehr_location_created_v2_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_corehr_location_created_v2(move |event: CorehrLocationCreatedV2| {
                assert_eq!(event.header.event_type, "corehr.location.created_v2");
                assert_eq!(event.event.location_id.as_deref(), Some("location_id_str"));
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(COREHR_LOCATION_CREATED_V2_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_corehr_location_deleted_v2_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_corehr_location_deleted_v2(move |event: CorehrLocationDeletedV2| {
                assert_eq!(event.header.event_type, "corehr.location.deleted_v2");
                assert_eq!(event.event.location_id.as_deref(), Some("location_id_str"));
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(COREHR_LOCATION_DELETED_V2_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_corehr_location_updated_v2_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_corehr_location_updated_v2(move |event: CorehrLocationUpdatedV2| {
                assert_eq!(event.header.event_type, "corehr.location.updated_v2");
                assert_eq!(event.event.location_id.as_deref(), Some("location_id_str"));
                assert_eq!(event.event.field_changes[0].as_str(), "field_changes_str");
                assert_eq!(event.event.sub_events.len(), 1);
                assert_eq!(event.event.sub_events[0].id.as_deref(), Some("id_str"));
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(COREHR_LOCATION_UPDATED_V2_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_corehr_offboarding_checklist_updated_v2_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_corehr_offboarding_checklist_updated_v2(
                move |event: CorehrOffboardingChecklistUpdatedV2| {
                    assert_eq!(
                        event.header.event_type,
                        "corehr.offboarding.checklist_updated_v2"
                    );
                    assert_eq!(
                        event.event.employment_id.as_deref(),
                        Some("employment_id_str")
                    );
                    let target_user_id =
                        event.event.target_user_id.as_ref().expect("target_user_id");
                    assert_eq!(target_user_id.user_id.as_deref(), Some("user_id_str"));
                    assert_eq!(
                        event.event.offboarding_id.as_deref(),
                        Some("offboarding_id_str")
                    );
                    assert_eq!(
                        event.event.checklist_process_id.as_deref(),
                        Some("checklist_process_id_str")
                    );
                    *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                    Ok(())
                },
            )
            .expect("register")
            .build();
        handler
            .do_without_validation(COREHR_OFFBOARDING_CHECKLIST_UPDATED_V2_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_corehr_offboarding_status_updated_v2_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_corehr_offboarding_status_updated_v2(
                move |event: CorehrOffboardingStatusUpdatedV2| {
                    assert_eq!(
                        event.header.event_type,
                        "corehr.offboarding.status_updated_v2"
                    );
                    assert_eq!(
                        event.event.employment_id.as_deref(),
                        Some("employment_id_str")
                    );
                    let target_user_id =
                        event.event.target_user_id.as_ref().expect("target_user_id");
                    assert_eq!(target_user_id.user_id.as_deref(), Some("user_id_str"));
                    assert_eq!(
                        event.event.offboarding_id.as_deref(),
                        Some("offboarding_id_str")
                    );
                    assert_eq!(event.event.process_id.as_deref(), Some("process_id_str"));
                    *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                    Ok(())
                },
            )
            .expect("register")
            .build();
        handler
            .do_without_validation(COREHR_OFFBOARDING_STATUS_UPDATED_V2_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_corehr_offboarding_updated_v2_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_corehr_offboarding_updated_v2(move |event: CorehrOffboardingUpdatedV2| {
                assert_eq!(event.header.event_type, "corehr.offboarding.updated_v2");
                assert_eq!(event.event.tenant_id.as_deref(), Some("tenant_id_str"));
                assert_eq!(
                    event.event.offboarding_info_id.as_deref(),
                    Some("offboarding_info_id_str")
                );
                assert_eq!(event.event.process_id.as_deref(), Some("process_id_str"));
                let target_user_id = event.event.target_user_id.as_ref().expect("target_user_id");
                assert_eq!(target_user_id.user_id.as_deref(), Some("user_id_str"));
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(COREHR_OFFBOARDING_UPDATED_V2_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_corehr_pathway_created_v2_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_corehr_pathway_created_v2(move |event: CorehrPathwayCreatedV2| {
                assert_eq!(event.header.event_type, "corehr.pathway.created_v2");
                assert_eq!(event.event.pathway_id.as_deref(), Some("pathway_id_str"));
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(COREHR_PATHWAY_CREATED_V2_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_corehr_pathway_deleted_v2_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_corehr_pathway_deleted_v2(move |event: CorehrPathwayDeletedV2| {
                assert_eq!(event.header.event_type, "corehr.pathway.deleted_v2");
                assert_eq!(event.event.pathway_id.as_deref(), Some("pathway_id_str"));
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(COREHR_PATHWAY_DELETED_V2_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_corehr_pathway_updated_v2_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_corehr_pathway_updated_v2(move |event: CorehrPathwayUpdatedV2| {
                assert_eq!(event.header.event_type, "corehr.pathway.updated_v2");
                assert_eq!(event.event.pathway_id.as_deref(), Some("pathway_id_str"));
                assert_eq!(event.event.field_changes[0].as_str(), "field_changes_str");
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(COREHR_PATHWAY_UPDATED_V2_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_corehr_position_created_v2_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_corehr_position_created_v2(move |event: CorehrPositionCreatedV2| {
                assert_eq!(event.header.event_type, "corehr.position.created_v2");
                assert_eq!(event.event.position_id.as_deref(), Some("position_id_str"));
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(COREHR_POSITION_CREATED_V2_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_corehr_position_deleted_v2_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_corehr_position_deleted_v2(move |event: CorehrPositionDeletedV2| {
                assert_eq!(event.header.event_type, "corehr.position.deleted_v2");
                assert_eq!(event.event.position_id.as_deref(), Some("position_id_str"));
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(COREHR_POSITION_DELETED_V2_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_corehr_position_updated_v2_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_corehr_position_updated_v2(move |event: CorehrPositionUpdatedV2| {
                assert_eq!(event.header.event_type, "corehr.position.updated_v2");
                assert_eq!(event.event.position_id.as_deref(), Some("position_id_str"));
                assert_eq!(event.event.field_changes[0].as_str(), "field_changes_str");
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(COREHR_POSITION_UPDATED_V2_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_corehr_pre_hire_onboarding_task_changed_v2_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_corehr_pre_hire_onboarding_task_changed_v2(
                move |event: CorehrPreHireOnboardingTaskChangedV2| {
                    assert_eq!(
                        event.header.event_type,
                        "corehr.pre_hire.onboarding_task_changed_v2"
                    );
                    assert_eq!(event.event.tenant_id.as_deref(), Some("tenant_id_str"));
                    assert_eq!(event.event.pre_hire_id.as_deref(), Some("pre_hire_id_str"));
                    assert_eq!(event.event.onboarding_task_changes.len(), 1);
                    assert_eq!(
                        event.event.onboarding_task_changes[0]
                            .after_status
                            .as_deref(),
                        Some("after_status_str")
                    );
                    assert_eq!(
                        event.event.onboarding_flow_id.as_deref(),
                        Some("onboarding_flow_id_str")
                    );
                    *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                    Ok(())
                },
            )
            .expect("register")
            .build();
        handler
            .do_without_validation(COREHR_PRE_HIRE_ONBOARDING_TASK_CHANGED_V2_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_corehr_probation_updated_v2_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_corehr_probation_updated_v2(move |event: CorehrProbationUpdatedV2| {
                assert_eq!(event.header.event_type, "corehr.probation.updated_v2");
                assert_eq!(
                    event.event.employment_id.as_deref(),
                    Some("employment_id_str")
                );
                assert_eq!(
                    event.event.probation_status.as_deref(),
                    Some("probation_status_str")
                );
                assert_eq!(
                    event.event.actual_probation_end_date.as_deref(),
                    Some("actual_probation_end_date_str")
                );
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(COREHR_PROBATION_UPDATED_V2_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_corehr_process_updated_v2_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_corehr_process_updated_v2(move |event: CorehrProcessUpdatedV2| {
                assert_eq!(event.header.event_type, "corehr.process.updated_v2");
                assert_eq!(event.event.process_id.as_deref(), Some("process_id_str"));
                assert_eq!(event.event.status, Some(42));
                assert_eq!(event.event.biz_type.as_deref(), Some("biz_type_str"));
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(COREHR_PROCESS_UPDATED_V2_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_corehr_process_approver_updated_v2_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_corehr_process_approver_updated_v2(
                move |event: CorehrProcessApproverUpdatedV2| {
                    assert_eq!(
                        event.header.event_type,
                        "corehr.process.approver.updated_v2"
                    );
                    assert_eq!(event.event.process_id.as_deref(), Some("process_id_str"));
                    assert_eq!(event.event.approver_id.as_deref(), Some("approver_id_str"));
                    assert_eq!(event.event.r#type, Some(42));
                    *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                    Ok(())
                },
            )
            .expect("register")
            .build();
        handler
            .do_without_validation(COREHR_PROCESS_APPROVER_UPDATED_V2_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_corehr_process_cc_updated_v2_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_corehr_process_cc_updated_v2(move |event: CorehrProcessCcUpdatedV2| {
                assert_eq!(event.header.event_type, "corehr.process.cc.updated_v2");
                assert_eq!(event.event.process_id.as_deref(), Some("process_id_str"));
                assert_eq!(event.event.approver_id.as_deref(), Some("approver_id_str"));
                assert_eq!(event.event.status, Some(42));
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(COREHR_PROCESS_CC_UPDATED_V2_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_corehr_process_node_updated_v2_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_corehr_process_node_updated_v2(move |event: CorehrProcessNodeUpdatedV2| {
                assert_eq!(event.header.event_type, "corehr.process.node.updated_v2");
                assert_eq!(
                    event.event.flow_definition_id.as_deref(),
                    Some("flow_definition_id_str")
                );
                assert_eq!(
                    event.event.node_definition_id.as_deref(),
                    Some("node_definition_id_str")
                );
                assert_eq!(event.event.process_id.as_deref(), Some("process_id_str"));
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(COREHR_PROCESS_NODE_UPDATED_V2_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_corehr_process_status_update_v2_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_corehr_process_status_update_v2(move |event: CorehrProcessStatusUpdateV2| {
                assert_eq!(event.header.event_type, "corehr.process.status.update_v2");
                assert_eq!(event.event.process_id.as_deref(), Some("process_id_str"));
                assert_eq!(event.event.status, Some(42));
                assert_eq!(event.event.biz_type.as_deref(), Some("biz_type_str"));
                *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(COREHR_PROCESS_STATUS_UPDATE_V2_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_corehr_process_comment_info_updated_v2_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_corehr_process_comment_info_updated_v2(
                move |event: CorehrProcessCommentInfoUpdatedV2| {
                    assert_eq!(
                        event.header.event_type,
                        "corehr.process_comment_info.updated_v2"
                    );
                    assert_eq!(event.event.process_id.as_deref(), Some("process_id_str"));
                    assert_eq!(event.event.comment_id.as_deref(), Some("comment_id_str"));
                    assert_eq!(event.event.r#type, Some(42));
                    *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                    Ok(())
                },
            )
            .expect("register")
            .build();
        handler
            .do_without_validation(COREHR_PROCESS_COMMENT_INFO_UPDATED_V2_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    #[test]
    fn typed_corehr_signature_file_status_updated_v2_deserializes() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_corehr_signature_file_status_updated_v2(
                move |event: CorehrSignatureFileStatusUpdatedV2| {
                    assert_eq!(
                        event.header.event_type,
                        "corehr.signature_file.status_updated_v2"
                    );
                    assert_eq!(
                        event.event.signature_file_id.as_deref(),
                        Some("signature_file_id_str")
                    );
                    assert_eq!(
                        event.event.before_status.as_deref(),
                        Some("before_status_str")
                    );
                    assert_eq!(
                        event.event.after_status.as_deref(),
                        Some("after_status_str")
                    );
                    *seen_h.lock().expect("lock") = Some(event.header.event_id.clone());
                    Ok(())
                },
            )
            .expect("register")
            .build();
        handler
            .do_without_validation(COREHR_SIGNATURE_FILE_STATUS_UPDATED_V2_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("evt_1"));
    }

    /// 事件串常量与 Go v3.12.2 dispatcher 赋值行逐字一致。
    #[test]
    fn typed_corehr_v2_event_constants_match_go_dispatcher() {
        assert_eq!(
            EventDispatcherHandler::COREHR_APPROVAL_GROUPS_UPDATED_V2,
            "corehr.approval_groups.updated_v2"
        );
        assert_eq!(
            EventDispatcherHandler::COREHR_COMPANY_CREATED_V2,
            "corehr.company.created_v2"
        );
        assert_eq!(
            EventDispatcherHandler::COREHR_COMPANY_DELETED_V2,
            "corehr.company.deleted_v2"
        );
        assert_eq!(
            EventDispatcherHandler::COREHR_COMPANY_UPDATED_V2,
            "corehr.company.updated_v2"
        );
        assert_eq!(
            EventDispatcherHandler::COREHR_COST_CENTER_CREATED_V2,
            "corehr.cost_center.created_v2"
        );
        assert_eq!(
            EventDispatcherHandler::COREHR_COST_CENTER_DELETED_V2,
            "corehr.cost_center.deleted_v2"
        );
        assert_eq!(
            EventDispatcherHandler::COREHR_COST_CENTER_UPDATED_V2,
            "corehr.cost_center.updated_v2"
        );
        assert_eq!(
            EventDispatcherHandler::COREHR_CUSTOM_ORG_CREATED_V2,
            "corehr.custom_org.created_v2"
        );
        assert_eq!(
            EventDispatcherHandler::COREHR_CUSTOM_ORG_DELETED_V2,
            "corehr.custom_org.deleted_v2"
        );
        assert_eq!(
            EventDispatcherHandler::COREHR_CUSTOM_ORG_UPDATED_V2,
            "corehr.custom_org.updated_v2"
        );
        assert_eq!(
            EventDispatcherHandler::COREHR_DEPARTMENT_CREATED_V2,
            "corehr.department.created_v2"
        );
        assert_eq!(
            EventDispatcherHandler::COREHR_DEPARTMENT_UPDATED_V2,
            "corehr.department.updated_v2"
        );
        assert_eq!(
            EventDispatcherHandler::COREHR_EMPLOYEE_DOMAIN_EVENT_V2,
            "corehr.employee.domain_event_v2"
        );
        assert_eq!(
            EventDispatcherHandler::COREHR_JOB_CHANGE_STATUS_UPDATED_V2,
            "corehr.job_change.status_updated_v2"
        );
        assert_eq!(
            EventDispatcherHandler::COREHR_JOB_CHANGE_UPDATED_V2,
            "corehr.job_change.updated_v2"
        );
        assert_eq!(
            EventDispatcherHandler::COREHR_JOB_FAMILY_CREATED_V2,
            "corehr.job_family.created_v2"
        );
        assert_eq!(
            EventDispatcherHandler::COREHR_JOB_FAMILY_DELETED_V2,
            "corehr.job_family.deleted_v2"
        );
        assert_eq!(
            EventDispatcherHandler::COREHR_JOB_FAMILY_UPDATED_V2,
            "corehr.job_family.updated_v2"
        );
        assert_eq!(
            EventDispatcherHandler::COREHR_JOB_GRADE_CREATED_V2,
            "corehr.job_grade.created_v2"
        );
        assert_eq!(
            EventDispatcherHandler::COREHR_JOB_GRADE_DELETED_V2,
            "corehr.job_grade.deleted_v2"
        );
        assert_eq!(
            EventDispatcherHandler::COREHR_JOB_GRADE_UPDATED_V2,
            "corehr.job_grade.updated_v2"
        );
        assert_eq!(
            EventDispatcherHandler::COREHR_JOB_LEVEL_CREATED_V2,
            "corehr.job_level.created_v2"
        );
        assert_eq!(
            EventDispatcherHandler::COREHR_JOB_LEVEL_DELETED_V2,
            "corehr.job_level.deleted_v2"
        );
        assert_eq!(
            EventDispatcherHandler::COREHR_JOB_LEVEL_UPDATED_V2,
            "corehr.job_level.updated_v2"
        );
        assert_eq!(
            EventDispatcherHandler::COREHR_LOCATION_CREATED_V2,
            "corehr.location.created_v2"
        );
        assert_eq!(
            EventDispatcherHandler::COREHR_LOCATION_DELETED_V2,
            "corehr.location.deleted_v2"
        );
        assert_eq!(
            EventDispatcherHandler::COREHR_LOCATION_UPDATED_V2,
            "corehr.location.updated_v2"
        );
        assert_eq!(
            EventDispatcherHandler::COREHR_OFFBOARDING_CHECKLIST_UPDATED_V2,
            "corehr.offboarding.checklist_updated_v2"
        );
        assert_eq!(
            EventDispatcherHandler::COREHR_OFFBOARDING_STATUS_UPDATED_V2,
            "corehr.offboarding.status_updated_v2"
        );
        assert_eq!(
            EventDispatcherHandler::COREHR_OFFBOARDING_UPDATED_V2,
            "corehr.offboarding.updated_v2"
        );
        assert_eq!(
            EventDispatcherHandler::COREHR_PATHWAY_CREATED_V2,
            "corehr.pathway.created_v2"
        );
        assert_eq!(
            EventDispatcherHandler::COREHR_PATHWAY_DELETED_V2,
            "corehr.pathway.deleted_v2"
        );
        assert_eq!(
            EventDispatcherHandler::COREHR_PATHWAY_UPDATED_V2,
            "corehr.pathway.updated_v2"
        );
        assert_eq!(
            EventDispatcherHandler::COREHR_POSITION_CREATED_V2,
            "corehr.position.created_v2"
        );
        assert_eq!(
            EventDispatcherHandler::COREHR_POSITION_DELETED_V2,
            "corehr.position.deleted_v2"
        );
        assert_eq!(
            EventDispatcherHandler::COREHR_POSITION_UPDATED_V2,
            "corehr.position.updated_v2"
        );
        assert_eq!(
            EventDispatcherHandler::COREHR_PRE_HIRE_ONBOARDING_TASK_CHANGED_V2,
            "corehr.pre_hire.onboarding_task_changed_v2"
        );
        assert_eq!(
            EventDispatcherHandler::COREHR_PROBATION_UPDATED_V2,
            "corehr.probation.updated_v2"
        );
        assert_eq!(
            EventDispatcherHandler::COREHR_PROCESS_UPDATED_V2,
            "corehr.process.updated_v2"
        );
        assert_eq!(
            EventDispatcherHandler::COREHR_PROCESS_APPROVER_UPDATED_V2,
            "corehr.process.approver.updated_v2"
        );
        assert_eq!(
            EventDispatcherHandler::COREHR_PROCESS_CC_UPDATED_V2,
            "corehr.process.cc.updated_v2"
        );
        assert_eq!(
            EventDispatcherHandler::COREHR_PROCESS_NODE_UPDATED_V2,
            "corehr.process.node.updated_v2"
        );
        assert_eq!(
            EventDispatcherHandler::COREHR_PROCESS_STATUS_UPDATE_V2,
            "corehr.process.status.update_v2"
        );
        assert_eq!(
            EventDispatcherHandler::COREHR_PROCESS_COMMENT_INFO_UPDATED_V2,
            "corehr.process_comment_info.updated_v2"
        );
        assert_eq!(
            EventDispatcherHandler::COREHR_SIGNATURE_FILE_STATUS_UPDATED_V2,
            "corehr.signature_file.status_updated_v2"
        );
    }

    /// typed 与 `register_raw` 同 key 重复注册返回 Err（双向）。
    #[test]
    fn typed_corehr_v2_duplicate_register_fails() {
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
            .register_corehr_approval_groups_updated_v2(|_event: CorehrApprovalGroupsUpdatedV2| {
                Ok(())
            })
            .expect("first")
            .register_raw("corehr.approval_groups.updated_v2", Noop)
            .expect_err("dup typed-then-raw");
        assert!(err.contains("already registered"));
        let err = EventDispatcherHandler::builder()
            .register_raw("corehr.approval_groups.updated_v2", Noop)
            .expect("raw first")
            .register_corehr_approval_groups_updated_v2(|_event: CorehrApprovalGroupsUpdatedV2| {
                Ok(())
            })
            .expect_err("dup raw-then-typed");
        assert!(err.contains("already registered"));
    }
}
