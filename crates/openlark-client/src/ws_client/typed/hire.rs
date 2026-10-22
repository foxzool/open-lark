//! hire 域 typed 事件（`hire.*`）。
//!
//! 字段权威：Go SDK v3.12.2 `service/hire/v1/model.go` 对应 `P2*` 结构的 json tag；
//! 事件清单对齐 `service/hire/v1/dispatcher.go`。Go 指针字段映射为 `Option<T>`，
//! serde 采用 `#[serde(default)]` 宽松风格，与 [`super::ImMessageReceiveV1`] 一致。

use serde::{Deserialize, Serialize};

use super::EventHeader;

/// `hire.application.deleted_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HireApplicationDeletedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: HireApplicationDeletedV1Event,
}

/// `hire.application.deleted_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HireApplicationDeletedV1Event {
    /// 投递 ID 列表。
    #[serde(default)]
    pub application_ids: Vec<String>,
}

/// `hire.application.stage_changed_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HireApplicationStageChangedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: HireApplicationStageChangedV1Event,
}

/// `hire.application.stage_changed_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HireApplicationStageChangedV1Event {
    /// 投递 ID。
    #[serde(default)]
    pub application_id: Option<String>,
    /// 投递阶段转移前的阶段 ID。
    #[serde(default)]
    pub origin_stage_id: Option<String>,
    /// 投递阶段转移后的阶段 ID。
    #[serde(default)]
    pub target_stage_id: Option<String>,
    /// 投递阶段变更时间戳（单位：毫秒）。
    #[serde(default, deserialize_with = "super::flex_time::de_opt_i64")]
    pub update_time: Option<i64>,
}

/// `hire.eco_account.created_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HireEcoAccountCreatedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: HireEcoAccountCreatedV1Event,
}

/// `hire.eco_account.created_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HireEcoAccountCreatedV1Event {
    /// 账号类型。
    #[serde(default)]
    pub scope: Option<i64>,
    /// 账号 ID，招聘系统内唯一。
    #[serde(default)]
    pub account_id: Option<String>,
    /// 账号名称。
    #[serde(default)]
    pub account_name: Option<String>,
    /// 账号适用范围（1 社招、2 校招）。
    #[serde(default)]
    pub usage_list: Vec<i64>,
    /// 自定义字段键值对。
    #[serde(default)]
    pub custom_field_list: Vec<EcoAccountCustomFieldEventData>,
}

/// `hire.eco_background_check.canceled_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HireEcoBackgroundCheckCanceledV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: HireEcoBackgroundCheckCanceledV1Event,
}

/// `hire.eco_background_check.canceled_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HireEcoBackgroundCheckCanceledV1Event {
    /// 背调 ID，招聘系统内唯一。
    #[serde(default)]
    pub background_check_id: Option<String>,
    /// 终止原因。
    #[serde(default)]
    pub termination_reason: Option<String>,
}

/// `hire.eco_background_check.created_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HireEcoBackgroundCheckCreatedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: HireEcoBackgroundCheckCreatedV1Event,
}

/// `hire.eco_background_check.created_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HireEcoBackgroundCheckCreatedV1Event {
    /// 背调 ID，招聘系统内唯一。
    #[serde(default)]
    pub background_check_id: Option<String>,
    /// 账号 ID，招聘系统内唯一。
    #[serde(default)]
    pub account_id: Option<String>,
    /// 套餐 ID。
    #[serde(default)]
    pub package_id: Option<String>,
    /// 附加调查项 ID 列表。
    #[serde(default)]
    pub additional_item_id_list: Vec<String>,
    /// 备注。
    #[serde(default)]
    pub comment: Option<String>,
    /// 候选人信息。
    #[serde(default)]
    pub candidate_info: Option<EcoBackgroundCheckCreateEventCandidateInfo>,
    /// 联系人（委托人）信息。
    #[serde(default)]
    pub client_contact_info: Option<EcoBackgroundCheckCreateEventContactInfo>,
    /// 自定义字段键值对。
    #[serde(default)]
    pub custom_field_list: Vec<EcoBackgroundCheckCreateEventCustomKv>,
}

/// `hire.eco_exam.created_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HireEcoExamCreatedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: HireEcoExamCreatedV1Event,
}

/// `hire.eco_exam.created_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HireEcoExamCreatedV1Event {
    /// 笔试 ID，招聘系统内唯一。
    #[serde(default)]
    pub exam_id: Option<String>,
    /// 账号 ID，招聘系统内唯一。
    #[serde(default)]
    pub account_id: Option<String>,
    /// 试卷 ID。
    #[serde(default)]
    pub paper_id: Option<String>,
    /// 候选人信息。
    #[serde(default)]
    pub candidate_info: Option<EcoExamCreateEventCandidateInfo>,
    /// 人才 ID。灰度字段。
    #[serde(default)]
    pub talent_id: Option<String>,
    /// 投递 ID。灰度字段。
    #[serde(default)]
    pub application_id: Option<String>,
    /// 职位信息。灰度字段。
    #[serde(default)]
    pub job_info: Option<EcoExamCreateEventJobInfo>,
}

/// `hire.ehr_import_task.imported_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HireEhrImportTaskImportedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: HireEhrImportTaskImportedV1Event,
}

/// `hire.ehr_import_task.imported_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HireEhrImportTaskImportedV1Event {
    /// 导入任务 ID。
    #[serde(default)]
    pub task_id: Option<String>,
    /// 投递 ID。
    #[serde(default)]
    pub application_id: Option<String>,
    /// 导入部门 ID。已不推荐使用，请使用 ehr_department 字段。
    #[serde(default)]
    pub ehr_department_id: Option<String>,
    /// 招聘需求 ID。
    #[serde(default)]
    pub ehr_requirement_id: Option<String>,
    /// 操作人的飞书招聘 user_id。已不推荐使用，请使用 operator_user_id 字段。
    #[serde(default)]
    pub operator_id: Option<String>,
    /// 操作用户。
    #[serde(default)]
    pub operator_user_id: Option<UserId>,
    /// 导入部门。
    #[serde(default)]
    pub ehr_department: Option<DepartmentId>,
}

/// `hire.ehr_import_task_for_internship_offer.imported_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HireEhrImportTaskForInternshipOfferImportedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: HireEhrImportTaskForInternshipOfferImportedV1Event,
}

/// `hire.ehr_import_task_for_internship_offer.imported_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HireEhrImportTaskForInternshipOfferImportedV1Event {
    /// 导入任务 ID。
    #[serde(default)]
    pub task_id: Option<String>,
    /// 投递 ID。
    #[serde(default)]
    pub application_id: Option<String>,
    /// Offer ID。
    #[serde(default)]
    pub offer_id: Option<String>,
    /// 入职 ID（实习）。
    #[serde(default)]
    pub pre_onboard_id: Option<String>,
    /// 导入部门 ID，等同于 ehr_department.department_id。
    #[serde(default)]
    pub ehr_department_id: Option<String>,
    /// 操作人的用户 user_id，等同于 operator_user_id.user_id。
    #[serde(default)]
    pub operator_id: Option<String>,
    /// 操作人用户 ID。
    #[serde(default)]
    pub operator_user_id: Option<UserId>,
    /// 导入部门 ID。
    #[serde(default)]
    pub ehr_department: Option<DepartmentId>,
}

/// `hire.offer.status_changed_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HireOfferStatusChangedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: HireOfferStatusChangedV1Event,
}

/// `hire.offer.status_changed_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HireOfferStatusChangedV1Event {
    /// 发生状态变更的 Offer ID。
    #[serde(default)]
    pub offer_id: Option<String>,
    /// Offer 状态。
    #[serde(default)]
    pub offer_status: Option<i64>,
}

/// `hire.referral_account.assets_update_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HireReferralAccountAssetsUpdateV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: HireReferralAccountAssetsUpdateV1Event,
}

/// `hire.referral_account.assets_update_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HireReferralAccountAssetsUpdateV1Event {
    /// 账户 ID，同一用户在同一租户同一应用中账户唯一。
    #[serde(default)]
    pub account_id: Option<String>,
    /// 账户余额信息。
    #[serde(default)]
    pub assets: Option<Assets>,
    /// 变更时间戳（单位：毫秒）。
    #[serde(default, deserialize_with = "super::flex_time::de_opt_string")]
    pub modify_time: Option<String>,
}

/// `hire.talent.deleted_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HireTalentDeletedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: HireTalentDeletedV1Event,
}

/// `hire.talent.deleted_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HireTalentDeletedV1Event {
    /// 人才 ID。
    #[serde(default)]
    pub talent_id: Option<String>,
}

/// `hire.talent.tag_subscription_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HireTalentTagSubscriptionV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: HireTalentTagSubscriptionV1Event,
}

/// `hire.talent.tag_subscription_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HireTalentTagSubscriptionV1Event {
    /// 人才 ID。
    #[serde(default)]
    pub talent_id: Option<String>,
    /// 投递 ID。
    #[serde(default)]
    pub application_id: Option<String>,
    /// 变更类型。
    #[serde(default)]
    pub r#type: Option<i64>,
    /// 人才标签。
    #[serde(default)]
    pub tag: Option<TalentTag>,
    /// 锁定状态。
    #[serde(default)]
    pub lock_status: Option<i64>,
    /// 投递阶段，如果变更类型为阶段变更，则为变更后的投递阶段。
    #[serde(default)]
    pub application_stage: Option<ApplicationStageInfo>,
}

/// 事件中的用户 ID 集合（Go `UserId`）。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct UserId {
    /// user_id。
    #[serde(default)]
    pub user_id: Option<String>,
    /// open_id。
    #[serde(default)]
    pub open_id: Option<String>,
    /// union_id。
    #[serde(default)]
    pub union_id: Option<String>,
}

/// 部门 ID（Go `DepartmentId`）。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct DepartmentId {
    /// department_id。
    #[serde(default)]
    pub department_id: Option<String>,
    /// open_department_id。
    #[serde(default)]
    pub open_department_id: Option<String>,
}

/// 账号自定义字段（Go `EcoAccountCustomFieldEventData`）。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct EcoAccountCustomFieldEventData {
    /// 自定义字段的标识。
    #[serde(default)]
    pub key: Option<String>,
    /// 客户填写的自定义字段的值。
    #[serde(default)]
    pub value: Option<String>,
}

/// 背调候选人信息（Go `EcoBackgroundCheckCreateEventCandidateInfo`）。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct EcoBackgroundCheckCreateEventCandidateInfo {
    /// 候选人姓名。
    #[serde(default)]
    pub name: Option<String>,
    /// 候选人手机号。
    #[serde(default)]
    pub mobile: Option<EcoBackgroundCheckCreateEventMobile>,
    /// 候选人邮箱。
    #[serde(default)]
    pub email: Option<String>,
    /// 名字。
    #[serde(default)]
    pub first_name: Option<String>,
    /// 姓氏。
    #[serde(default)]
    pub last_name: Option<String>,
}

/// 背调联系人信息（Go `EcoBackgroundCheckCreateEventContactInfo`）。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct EcoBackgroundCheckCreateEventContactInfo {
    /// 联系人姓名。
    #[serde(default)]
    pub name: Option<String>,
    /// 联系人手机号。
    #[serde(default)]
    pub mobile: Option<EcoBackgroundCheckCreateEventMobile>,
    /// 联系人邮箱。
    #[serde(default)]
    pub email: Option<String>,
}

/// 背调自定义字段（Go `EcoBackgroundCheckCreateEventCustomKv`）。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct EcoBackgroundCheckCreateEventCustomKv {
    /// 自定义字段标识。
    #[serde(default)]
    pub key: Option<String>,
    /// 自定义字段值。
    #[serde(default)]
    pub value: Option<String>,
}

/// 背调手机号（Go `EcoBackgroundCheckCreateEventMobile`）。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct EcoBackgroundCheckCreateEventMobile {
    /// 电话国家区号。
    #[serde(default)]
    pub code: Option<String>,
    /// 手机号码。
    #[serde(default)]
    pub number: Option<String>,
}

/// 笔试候选人信息（Go `EcoExamCreateEventCandidateInfo`）。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct EcoExamCreateEventCandidateInfo {
    /// 候选人姓名。
    #[serde(default)]
    pub name: Option<String>,
    /// 候选人手机号。
    #[serde(default)]
    pub mobile: Option<EcoExamCreateEventMobile>,
    /// 候选人邮箱。
    #[serde(default)]
    pub email: Option<String>,
}

/// 笔试手机号（Go `EcoExamCreateEventMobile`）。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct EcoExamCreateEventMobile {
    /// 电话国家区号。
    #[serde(default)]
    pub code: Option<String>,
    /// 手机号码。
    #[serde(default)]
    pub number: Option<String>,
}

/// 笔试职位信息（Go `EcoExamCreateEventJobInfo`）。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EcoExamCreateEventJobInfo {
    /// 职位部门。ID 为招聘系统 ATS/SaaS 部门 ID。
    #[serde(default)]
    pub department: Option<EcoExamCreateEventJobDepartment>,
    /// 职位地址列表，顺序与职位配置一致。
    #[serde(default)]
    pub addresses: Vec<CommonAddress>,
}

/// 笔试职位部门（Go `EcoExamCreateEventJobDepartment`）。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EcoExamCreateEventJobDepartment {
    /// 招聘系统 ATS/SaaS 部门 ID。
    #[serde(default)]
    pub id: Option<String>,
    /// 部门名称。
    #[serde(default)]
    pub name: Option<I18n>,
}

/// 通用地址（Go `CommonAddress`）。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CommonAddress {
    /// ID。
    #[serde(default)]
    pub id: Option<String>,
    /// 名称。
    #[serde(default)]
    pub name: Option<I18n>,
    /// 区县。
    #[serde(default)]
    pub district: Option<CodeNameObject>,
    /// 城市。
    #[serde(default)]
    pub city: Option<CodeNameObject>,
    /// 省。
    #[serde(default)]
    pub state: Option<CodeNameObject>,
    /// 国家。
    #[serde(default)]
    pub country: Option<CodeNameObject>,
}

/// 编码 + 多语名称（Go `CodeNameObject`）。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CodeNameObject {
    /// 编码。
    #[serde(default)]
    pub code: Option<String>,
    /// 名称。
    #[serde(default)]
    pub name: Option<I18n>,
}

/// 中英双语名称（Go `I18n`）。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct I18n {
    /// 中文名称。
    #[serde(default)]
    pub zh_cn: Option<String>,
    /// 英文名称。
    #[serde(default)]
    pub en_us: Option<String>,
}

/// 内推账户余额（Go `Assets`）。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Assets {
    /// 已确认的奖励。
    #[serde(default)]
    pub confirmed_bonus: Option<BonusAmount>,
    /// 已发放的奖励。
    #[serde(default)]
    pub paid_bonus: Option<BonusAmount>,
}

/// 奖励金额（Go `BonusAmount`）。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BonusAmount {
    /// 奖励类型。
    #[serde(default)]
    pub bonus_type: Option<i64>,
    /// 积分奖励。
    #[serde(default)]
    pub point_bonus: Option<i64>,
    /// 现金奖励。
    #[serde(default)]
    pub cash: Option<Cash>,
    /// 现金奖励列表。
    #[serde(default)]
    pub cash_bonus: Vec<Cash>,
}

/// 现金奖励（Go `Cash`）。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Cash {
    /// 币种。
    #[serde(default)]
    pub currency_type: Option<String>,
    /// 数额，保留到小数点后两位。
    #[serde(default)]
    pub amount: Option<f64>,
}

/// 人才标签（Go `TalentTag`）。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TalentTag {
    /// 标签 ID。
    #[serde(default)]
    pub id: Option<String>,
    /// 标签名称。
    #[serde(default)]
    pub name: Option<I18n>,
    /// 描述名称。
    #[serde(default)]
    pub description: Option<I18n>,
    /// 标签类型。
    #[serde(default)]
    pub r#type: Option<i64>,
    /// 启停用状态。
    #[serde(default)]
    pub active_status: Option<i64>,
}

/// 投递阶段信息（Go `ApplicationStageInfo`）。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ApplicationStageInfo {
    /// 阶段 ID。
    #[serde(default)]
    pub id: Option<String>,
    /// 中文名称。
    #[serde(default)]
    pub zh_name: Option<String>,
    /// 英文名称。
    #[serde(default)]
    pub en_name: Option<String>,
    /// 阶段类型。
    #[serde(default)]
    pub r#type: Option<i64>,
}

typed_events! {
    /// `hire.application.deleted_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "hire.application.deleted_v1" | HIRE_APPLICATION_DELETED_V1 | HireApplicationDeletedV1 | HireApplicationDeletedV1Handler | register_hire_application_deleted_v1;

    /// `hire.application.stage_changed_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "hire.application.stage_changed_v1" | HIRE_APPLICATION_STAGE_CHANGED_V1 | HireApplicationStageChangedV1 | HireApplicationStageChangedV1Handler | register_hire_application_stage_changed_v1;

    /// `hire.eco_account.created_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "hire.eco_account.created_v1" | HIRE_ECO_ACCOUNT_CREATED_V1 | HireEcoAccountCreatedV1 | HireEcoAccountCreatedV1Handler | register_hire_eco_account_created_v1;

    /// `hire.eco_background_check.canceled_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "hire.eco_background_check.canceled_v1" | HIRE_ECO_BACKGROUND_CHECK_CANCELED_V1 | HireEcoBackgroundCheckCanceledV1 | HireEcoBackgroundCheckCanceledV1Handler | register_hire_eco_background_check_canceled_v1;

    /// `hire.eco_background_check.created_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "hire.eco_background_check.created_v1" | HIRE_ECO_BACKGROUND_CHECK_CREATED_V1 | HireEcoBackgroundCheckCreatedV1 | HireEcoBackgroundCheckCreatedV1Handler | register_hire_eco_background_check_created_v1;

    /// `hire.eco_exam.created_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "hire.eco_exam.created_v1" | HIRE_ECO_EXAM_CREATED_V1 | HireEcoExamCreatedV1 | HireEcoExamCreatedV1Handler | register_hire_eco_exam_created_v1;

    /// `hire.ehr_import_task.imported_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "hire.ehr_import_task.imported_v1" | HIRE_EHR_IMPORT_TASK_IMPORTED_V1 | HireEhrImportTaskImportedV1 | HireEhrImportTaskImportedV1Handler | register_hire_ehr_import_task_imported_v1;

    /// `hire.ehr_import_task_for_internship_offer.imported_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "hire.ehr_import_task_for_internship_offer.imported_v1" | HIRE_EHR_IMPORT_TASK_FOR_INTERNSHIP_OFFER_IMPORTED_V1 | HireEhrImportTaskForInternshipOfferImportedV1 | HireEhrImportTaskForInternshipOfferImportedV1Handler | register_hire_ehr_import_task_for_internship_offer_imported_v1;

    /// `hire.offer.status_changed_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "hire.offer.status_changed_v1" | HIRE_OFFER_STATUS_CHANGED_V1 | HireOfferStatusChangedV1 | HireOfferStatusChangedV1Handler | register_hire_offer_status_changed_v1;

    /// `hire.referral_account.assets_update_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "hire.referral_account.assets_update_v1" | HIRE_REFERRAL_ACCOUNT_ASSETS_UPDATE_V1 | HireReferralAccountAssetsUpdateV1 | HireReferralAccountAssetsUpdateV1Handler | register_hire_referral_account_assets_update_v1;

    /// `hire.talent.deleted_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "hire.talent.deleted_v1" | HIRE_TALENT_DELETED_V1 | HireTalentDeletedV1 | HireTalentDeletedV1Handler | register_hire_talent_deleted_v1;

    /// `hire.talent.tag_subscription_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "hire.talent.tag_subscription_v1" | HIRE_TALENT_TAG_SUBSCRIPTION_V1 | HireTalentTagSubscriptionV1 | HireTalentTagSubscriptionV1Handler | register_hire_talent_tag_subscription_v1;
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use super::*;
    use crate::ws_client::EventDispatcherHandler;

    const APPLICATION_DELETED_FIXTURE: &str = r#"{
      "schema":"2.0",
      "header":{"event_id":"h1","event_type":"hire.application.deleted_v1","token":"t","app_id":"cli_x","tenant_key":"tk"},
      "event":{"application_ids":["app_1","app_2"]}
    }"#;

    const APPLICATION_STAGE_CHANGED_FIXTURE: &str = r#"{
      "schema":"2.0",
      "header":{"event_id":"h2","event_type":"hire.application.stage_changed_v1","token":"t","app_id":"cli_x","tenant_key":"tk"},
      "event":{"application_id":"app_1","origin_stage_id":"stg_1","target_stage_id":"stg_2","update_time":1700000000000}
    }"#;

    const ECO_ACCOUNT_CREATED_FIXTURE: &str = r#"{
      "schema":"2.0",
      "header":{"event_id":"h3","event_type":"hire.eco_account.created_v1","token":"t","app_id":"cli_x","tenant_key":"tk"},
      "event":{
        "scope":1,
        "account_id":"acc_1",
        "account_name":"服务商账号",
        "usage_list":[1,2],
        "custom_field_list":[{"key":"cf_1","value":"值"}]
      }
    }"#;

    const ECO_BACKGROUND_CHECK_CANCELED_FIXTURE: &str = r#"{
      "schema":"2.0",
      "header":{"event_id":"h4","event_type":"hire.eco_background_check.canceled_v1","token":"t","app_id":"cli_x","tenant_key":"tk"},
      "event":{"background_check_id":"bc_1","termination_reason":"候选人取消"}
    }"#;

    const ECO_BACKGROUND_CHECK_CREATED_FIXTURE: &str = r#"{
      "schema":"2.0",
      "header":{"event_id":"h5","event_type":"hire.eco_background_check.created_v1","token":"t","app_id":"cli_x","tenant_key":"tk"},
      "event":{
        "background_check_id":"bc_2",
        "account_id":"acc_1",
        "package_id":"pkg_1",
        "additional_item_id_list":["item_1","item_2"],
        "comment":"背调备注",
        "candidate_info":{"name":"张三","mobile":{"code":"86","number":"13800000000"},"email":"zhang@example.com","first_name":"三","last_name":"张"},
        "client_contact_info":{"name":"李四","mobile":{"code":"86","number":"13900000000"},"email":"li@example.com"},
        "custom_field_list":[{"key":"bcf_1","value":"v"}]
      }
    }"#;

    const ECO_EXAM_CREATED_FIXTURE: &str = r#"{
      "schema":"2.0",
      "header":{"event_id":"h6","event_type":"hire.eco_exam.created_v1","token":"t","app_id":"cli_x","tenant_key":"tk"},
      "event":{
        "exam_id":"exam_1",
        "account_id":"acc_1",
        "paper_id":"paper_1",
        "candidate_info":{"name":"张三","mobile":{"code":"86","number":"13800000000"},"email":"zhang@example.com"},
        "talent_id":"tal_1",
        "application_id":"app_1",
        "job_info":{"department":{"id":"dep_1","name":{"zh_cn":"研发部","en_us":"R&D"}},"addresses":[{"id":"addr_1","name":{"zh_cn":"北京"},"district":{"code":"110105","name":{"zh_cn":"朝阳区"}}}]}
      }
    }"#;

    const EHR_IMPORT_TASK_IMPORTED_FIXTURE: &str = r#"{
      "schema":"2.0",
      "header":{"event_id":"h7","event_type":"hire.ehr_import_task.imported_v1","token":"t","app_id":"cli_x","tenant_key":"tk"},
      "event":{
        "task_id":"eit_1",
        "application_id":"app_1",
        "ehr_department_id":"dep_1",
        "ehr_requirement_id":"req_1",
        "operator_id":"op_1",
        "operator_user_id":{"open_id":"on_op"},
        "ehr_department":{"department_id":"dep_1","open_department_id":"od_1"}
      }
    }"#;

    const EHR_IMPORT_TASK_FOR_INTERNSHIP_OFFER_IMPORTED_FIXTURE: &str = r#"{
      "schema":"2.0",
      "header":{"event_id":"h8","event_type":"hire.ehr_import_task_for_internship_offer.imported_v1","token":"t","app_id":"cli_x","tenant_key":"tk"},
      "event":{
        "task_id":"eit_2",
        "application_id":"app_2",
        "offer_id":"offer_1",
        "pre_onboard_id":"pre_1",
        "ehr_department_id":"dep_2",
        "operator_id":"op_2",
        "operator_user_id":{"open_id":"on_op2"},
        "ehr_department":{"department_id":"dep_2","open_department_id":"od_2"}
      }
    }"#;

    const OFFER_STATUS_CHANGED_FIXTURE: &str = r#"{
      "schema":"2.0",
      "header":{"event_id":"h9","event_type":"hire.offer.status_changed_v1","token":"t","app_id":"cli_x","tenant_key":"tk"},
      "event":{"offer_id":"offer_1","offer_status":5}
    }"#;

    const REFERRAL_ACCOUNT_ASSETS_UPDATE_FIXTURE: &str = r#"{
      "schema":"2.0",
      "header":{"event_id":"h10","event_type":"hire.referral_account.assets_update_v1","token":"t","app_id":"cli_x","tenant_key":"tk"},
      "event":{
        "account_id":"ref_1",
        "assets":{
          "confirmed_bonus":{"bonus_type":1,"point_bonus":100,"cash":{"currency_type":"CNY","amount":10.5},"cash_bonus":[{"currency_type":"CNY","amount":1.25}]},
          "paid_bonus":{"bonus_type":1,"point_bonus":50}
        },
        "modify_time":"1700000000000"
      }
    }"#;

    const TALENT_DELETED_FIXTURE: &str = r#"{
      "schema":"2.0",
      "header":{"event_id":"h11","event_type":"hire.talent.deleted_v1","token":"t","app_id":"cli_x","tenant_key":"tk"},
      "event":{"talent_id":"tal_1"}
    }"#;

    const TALENT_TAG_SUBSCRIPTION_FIXTURE: &str = r#"{
      "schema":"2.0",
      "header":{"event_id":"h12","event_type":"hire.talent.tag_subscription_v1","token":"t","app_id":"cli_x","tenant_key":"tk"},
      "event":{
        "talent_id":"tal_1",
        "application_id":"app_1",
        "type":1,
        "tag":{"id":"tag_1","name":{"zh_cn":"高潜","en_us":"HighPot"},"description":{"zh_cn":"描述"},"type":2,"active_status":1},
        "lock_status":0,
        "application_stage":{"id":"stg_3","zh_name":"面试","en_name":"Interview","type":2}
      }
    }"#;

    #[test]
    fn typed_hire_application_deleted_deserializes_ids() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_hire_application_deleted_v1(move |event: HireApplicationDeletedV1| {
                assert_eq!(event.header.event_type, "hire.application.deleted_v1");
                assert_eq!(
                    event.event.application_ids,
                    vec!["app_1".to_string(), "app_2".to_string()]
                );
                *seen_h.lock().expect("lock") = event.event.application_ids.first().cloned();
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(APPLICATION_DELETED_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("app_1"));
    }

    #[test]
    fn typed_hire_application_stage_changed_deserializes_stages() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_hire_application_stage_changed_v1(
                move |event: HireApplicationStageChangedV1| {
                    assert_eq!(event.event.origin_stage_id.as_deref(), Some("stg_1"));
                    assert_eq!(event.event.target_stage_id.as_deref(), Some("stg_2"));
                    assert_eq!(event.event.update_time, Some(1700000000000));
                    *seen_h.lock().expect("lock") = event.event.application_id.clone();
                    Ok(())
                },
            )
            .expect("register")
            .build();
        handler
            .do_without_validation(APPLICATION_STAGE_CHANGED_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("app_1"));
    }

    #[test]
    fn typed_hire_eco_account_created_deserializes_custom_fields() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_hire_eco_account_created_v1(move |event: HireEcoAccountCreatedV1| {
                assert_eq!(event.event.scope, Some(1));
                assert_eq!(event.event.usage_list, vec![1, 2]);
                assert_eq!(event.event.account_name.as_deref(), Some("服务商账号"));
                let field = event.event.custom_field_list.first().expect("custom");
                assert_eq!(field.key.as_deref(), Some("cf_1"));
                assert_eq!(field.value.as_deref(), Some("值"));
                *seen_h.lock().expect("lock") = event.event.account_id.clone();
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(ECO_ACCOUNT_CREATED_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("acc_1"));
    }

    #[test]
    fn typed_hire_eco_background_check_canceled_deserializes_reason() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_hire_eco_background_check_canceled_v1(
                move |event: HireEcoBackgroundCheckCanceledV1| {
                    assert_eq!(
                        event.event.termination_reason.as_deref(),
                        Some("候选人取消")
                    );
                    *seen_h.lock().expect("lock") = event.event.background_check_id.clone();
                    Ok(())
                },
            )
            .expect("register")
            .build();
        handler
            .do_without_validation(ECO_BACKGROUND_CHECK_CANCELED_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("bc_1"));
    }

    #[test]
    fn typed_hire_eco_background_check_created_deserializes_candidate() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_hire_eco_background_check_created_v1(
                move |event: HireEcoBackgroundCheckCreatedV1| {
                    assert_eq!(event.event.package_id.as_deref(), Some("pkg_1"));
                    assert_eq!(
                        event.event.additional_item_id_list,
                        vec!["item_1".to_string(), "item_2".to_string()]
                    );
                    let candidate = event.event.candidate_info.as_ref().expect("candidate");
                    assert_eq!(candidate.name.as_deref(), Some("张三"));
                    assert_eq!(
                        candidate.mobile.as_ref().and_then(|m| m.number.as_deref()),
                        Some("13800000000")
                    );
                    let contact = event.event.client_contact_info.as_ref().expect("contact");
                    assert_eq!(contact.name.as_deref(), Some("李四"));
                    *seen_h.lock().expect("lock") = event.event.background_check_id.clone();
                    Ok(())
                },
            )
            .expect("register")
            .build();
        handler
            .do_without_validation(ECO_BACKGROUND_CHECK_CREATED_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("bc_2"));
    }

    #[test]
    fn typed_hire_eco_exam_created_deserializes_job_info() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_hire_eco_exam_created_v1(move |event: HireEcoExamCreatedV1| {
                assert_eq!(event.event.paper_id.as_deref(), Some("paper_1"));
                assert_eq!(event.event.talent_id.as_deref(), Some("tal_1"));
                let candidate = event.event.candidate_info.as_ref().expect("candidate");
                assert_eq!(
                    candidate.mobile.as_ref().and_then(|m| m.code.as_deref()),
                    Some("86")
                );
                let job = event.event.job_info.as_ref().expect("job_info");
                let department = job.department.as_ref().expect("department");
                assert_eq!(
                    department.name.as_ref().and_then(|n| n.zh_cn.as_deref()),
                    Some("研发部")
                );
                let addr = job.addresses.first().expect("addresses");
                assert_eq!(
                    addr.district.as_ref().and_then(|d| d.code.as_deref()),
                    Some("110105")
                );
                *seen_h.lock().expect("lock") = event.event.exam_id.clone();
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(ECO_EXAM_CREATED_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("exam_1"));
    }

    #[test]
    fn typed_hire_ehr_import_task_imported_deserializes_department() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_hire_ehr_import_task_imported_v1(move |event: HireEhrImportTaskImportedV1| {
                assert_eq!(event.event.ehr_requirement_id.as_deref(), Some("req_1"));
                assert_eq!(event.event.operator_id.as_deref(), Some("op_1"));
                assert_eq!(
                    event
                        .event
                        .operator_user_id
                        .as_ref()
                        .and_then(|u| u.open_id.as_deref()),
                    Some("on_op")
                );
                let dep = event.event.ehr_department.as_ref().expect("department");
                assert_eq!(dep.open_department_id.as_deref(), Some("od_1"));
                *seen_h.lock().expect("lock") = event.event.task_id.clone();
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(EHR_IMPORT_TASK_IMPORTED_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("eit_1"));
    }

    #[test]
    fn typed_hire_ehr_import_task_for_internship_offer_deserializes_offer() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_hire_ehr_import_task_for_internship_offer_imported_v1(
                move |event: HireEhrImportTaskForInternshipOfferImportedV1| {
                    assert_eq!(event.event.offer_id.as_deref(), Some("offer_1"));
                    assert_eq!(event.event.pre_onboard_id.as_deref(), Some("pre_1"));
                    let dep = event.event.ehr_department.as_ref().expect("department");
                    assert_eq!(dep.department_id.as_deref(), Some("dep_2"));
                    *seen_h.lock().expect("lock") = event.event.task_id.clone();
                    Ok(())
                },
            )
            .expect("register")
            .build();
        handler
            .do_without_validation(EHR_IMPORT_TASK_FOR_INTERNSHIP_OFFER_IMPORTED_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("eit_2"));
    }

    #[test]
    fn typed_hire_offer_status_changed_deserializes_status() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_hire_offer_status_changed_v1(move |event: HireOfferStatusChangedV1| {
                assert_eq!(event.event.offer_status, Some(5));
                *seen_h.lock().expect("lock") = event.event.offer_id.clone();
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(OFFER_STATUS_CHANGED_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("offer_1"));
    }

    #[test]
    fn typed_hire_referral_account_assets_update_deserializes_assets() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_hire_referral_account_assets_update_v1(
                move |event: HireReferralAccountAssetsUpdateV1| {
                    let assets = event.event.assets.as_ref().expect("assets");
                    let confirmed = assets.confirmed_bonus.as_ref().expect("confirmed");
                    assert_eq!(confirmed.bonus_type, Some(1));
                    assert_eq!(confirmed.point_bonus, Some(100));
                    let cash = confirmed.cash.as_ref().expect("cash");
                    assert_eq!(cash.currency_type.as_deref(), Some("CNY"));
                    assert_eq!(cash.amount, Some(10.5));
                    assert_eq!(
                        confirmed.cash_bonus.first().and_then(|c| c.amount),
                        Some(1.25)
                    );
                    assert_eq!(
                        assets.paid_bonus.as_ref().and_then(|b| b.point_bonus),
                        Some(50)
                    );
                    assert_eq!(event.event.modify_time.as_deref(), Some("1700000000000"));
                    *seen_h.lock().expect("lock") = event.event.account_id.clone();
                    Ok(())
                },
            )
            .expect("register")
            .build();
        handler
            .do_without_validation(REFERRAL_ACCOUNT_ASSETS_UPDATE_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("ref_1"));
    }

    #[test]
    fn typed_hire_talent_deleted_deserializes_talent_id() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_hire_talent_deleted_v1(move |event: HireTalentDeletedV1| {
                assert_eq!(event.header.event_type, "hire.talent.deleted_v1");
                *seen_h.lock().expect("lock") = event.event.talent_id.clone();
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(TALENT_DELETED_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("tal_1"));
    }

    #[test]
    fn typed_hire_talent_tag_subscription_deserializes_tag() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_hire_talent_tag_subscription_v1(move |event: HireTalentTagSubscriptionV1| {
                assert_eq!(event.event.r#type, Some(1));
                assert_eq!(event.event.lock_status, Some(0));
                let tag = event.event.tag.as_ref().expect("tag");
                assert_eq!(tag.id.as_deref(), Some("tag_1"));
                assert_eq!(
                    tag.name.as_ref().and_then(|n| n.zh_cn.as_deref()),
                    Some("高潜")
                );
                assert_eq!(
                    tag.description.as_ref().and_then(|d| d.zh_cn.as_deref()),
                    Some("描述")
                );
                assert_eq!(tag.active_status, Some(1));
                let stage = event.event.application_stage.as_ref().expect("stage");
                assert_eq!(stage.en_name.as_deref(), Some("Interview"));
                *seen_h.lock().expect("lock") = event.event.application_id.clone();
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(TALENT_TAG_SUBSCRIPTION_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("app_1"));
    }

    /// 事件串常量与票面清单逐字一致。
    #[test]
    fn typed_hire_event_constants_match_go_dispatcher() {
        assert_eq!(
            EventDispatcherHandler::HIRE_APPLICATION_DELETED_V1,
            "hire.application.deleted_v1"
        );
        assert_eq!(
            EventDispatcherHandler::HIRE_APPLICATION_STAGE_CHANGED_V1,
            "hire.application.stage_changed_v1"
        );
        assert_eq!(
            EventDispatcherHandler::HIRE_ECO_ACCOUNT_CREATED_V1,
            "hire.eco_account.created_v1"
        );
        assert_eq!(
            EventDispatcherHandler::HIRE_ECO_BACKGROUND_CHECK_CANCELED_V1,
            "hire.eco_background_check.canceled_v1"
        );
        assert_eq!(
            EventDispatcherHandler::HIRE_ECO_BACKGROUND_CHECK_CREATED_V1,
            "hire.eco_background_check.created_v1"
        );
        assert_eq!(
            EventDispatcherHandler::HIRE_ECO_EXAM_CREATED_V1,
            "hire.eco_exam.created_v1"
        );
        assert_eq!(
            EventDispatcherHandler::HIRE_EHR_IMPORT_TASK_IMPORTED_V1,
            "hire.ehr_import_task.imported_v1"
        );
        assert_eq!(
            EventDispatcherHandler::HIRE_EHR_IMPORT_TASK_FOR_INTERNSHIP_OFFER_IMPORTED_V1,
            "hire.ehr_import_task_for_internship_offer.imported_v1"
        );
        assert_eq!(
            EventDispatcherHandler::HIRE_OFFER_STATUS_CHANGED_V1,
            "hire.offer.status_changed_v1"
        );
        assert_eq!(
            EventDispatcherHandler::HIRE_REFERRAL_ACCOUNT_ASSETS_UPDATE_V1,
            "hire.referral_account.assets_update_v1"
        );
        assert_eq!(
            EventDispatcherHandler::HIRE_TALENT_DELETED_V1,
            "hire.talent.deleted_v1"
        );
        assert_eq!(
            EventDispatcherHandler::HIRE_TALENT_TAG_SUBSCRIPTION_V1,
            "hire.talent.tag_subscription_v1"
        );
    }

    /// typed 注册后再 `register_raw` 同 key 返回 Err；反方向同理。
    #[test]
    fn typed_hire_duplicate_register_fails() {
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
            .register_hire_talent_deleted_v1(|_event: HireTalentDeletedV1| Ok(()))
            .expect("first")
            .register_raw("hire.talent.deleted_v1", Noop)
            .expect_err("dup typed-then-raw");
        assert!(err.contains("already registered"));
        let err = EventDispatcherHandler::builder()
            .register_raw("hire.offer.status_changed_v1", Noop)
            .expect("raw first")
            .register_hire_offer_status_changed_v1(|_event: HireOfferStatusChangedV1| Ok(()))
            .expect_err("dup raw-then-typed");
        assert!(err.contains("already registered"));
    }
}
