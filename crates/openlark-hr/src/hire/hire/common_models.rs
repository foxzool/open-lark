//! Hire 业务域通用响应模型。
//!
//! 用于承接第一批 typed response 收敛中反复出现的 i18n、分页、引用对象、
//! 金额与附件元数据等结构，避免在每个 API 文件里重复定义。

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 国际化文本。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct I18nText {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zh_cn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub en_us: Option<String>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

/// 常见的 ID + 名称引用对象。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct IdNameObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<I18nText>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

/// 带 code + name 的区域对象。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CodeNameObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<I18nText>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

/// 通用分页响应壳。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct PaginatedResponse<T> {
    #[serde(default)]
    pub items: Vec<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

/// 奖励金额。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct BonusAmount {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub point_bonus: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cash_bonus: Option<Vec<CashAmount>>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

/// 现金奖励金额。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CashAmount {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

/// 附件元数据。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct AttachmentMeta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

/// 分数信息。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ScoreInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_score: Option<f64>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

/// 招聘备注。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct NoteRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub talent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_private: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modify_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

/// 招聘附件/文件句柄。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct HireAttachment {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

/// `zh_name` / `en_name` 形式的双语文本。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct LocalizedLabel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zh_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub en_name: Option<String>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

/// 投递所关联的职位摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ApplicationJobInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

/// 投递所关联的候选人摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ApplicationTalentInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub talent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub talent_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

/// 投递关联的 offer 摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ApplicationOfferInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offer_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offer_status: Option<i32>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

/// 投递摘要信息。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ApplicationSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub talent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_info: Option<ApplicationJobInfo>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

/// 投递关联的面试记录摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ApplicationInterviewRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interview_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interview_round_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interview_round_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interviewer: Option<IdNameObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<ScoreInfo>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

/// 职位上的招聘相关人员。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct JobRecruiterRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recruiter_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manager_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_type: Option<i32>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

/// 职位摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct JobSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recruiters: Option<Vec<JobRecruiterRecord>>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

/// 职位设置摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct JobConfigInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_requirement_schema_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interview_registration_schema_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offer_application_form_id: Option<String>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

/// 外部投递摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ExternalApplicationSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_application_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub talent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_name: Option<String>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

/// 外部 Offer 摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ExternalOfferSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_offer_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offer_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub talent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

/// 招聘官网推广渠道摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct WebsiteChannelSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_status: Option<i32>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

/// 招聘官网投递任务结果。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct WebsiteDeliveryTaskResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_task_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub talent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

/// 招聘官网职位广告摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct WebsiteJobPostSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_post_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_channel_id: Option<String>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

/// 招聘官网站点用户摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct WebsiteSiteUserSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

/// 猎头供应商摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct AgencySummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agency_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

/// 猎头供应商下的猎头账号摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct AgencyAccountSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agency_account_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agency_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

/// 猎头保护期摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct AgencyProtectionSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protection_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agency_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub talent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

/// 人才外部信息摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TalentExternalInfoRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_info_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub talent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

/// 招聘需求摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct JobRequirementSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_requirement_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

/// 投递操作结果摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ApplicationOperationResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offer_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub onboard_status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

/// 外部背调摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ExternalBackgroundCheckSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_background_check_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub talent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_name: Option<String>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

/// 外部面试摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ExternalInterviewSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_interview_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub talent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interview_round_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

/// 三方协议摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TripartiteAgreementSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agreement_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub talent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sign_status: Option<i32>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

/// 生态自定义字段摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct EcoCustomFieldSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_field_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_status: Option<i32>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

/// 生态背调套餐摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct EcoBackgroundCheckPackageSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

/// 生态考试试卷摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct EcoExamPaperSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paper_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

/// 生态流程操作结果。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct EcoOperationResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_field_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paper_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_check_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

/// 入职员工摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct EmployeeSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub talent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub onboard_status: Option<i32>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

/// 内推账户操作结果。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ReferralAccountOperationResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

/// 外部面评操作结果。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ExternalInterviewAssessmentResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_interview_assessment_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_interview_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

/// 面试摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct InterviewSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interview_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub talent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interview_round_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

/// 背调订单摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct BackgroundCheckOrderSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub talent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_name: Option<String>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

/// 外部内推奖励操作结果。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ExternalReferralRewardResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_referral_reward_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

/// 人才库操作结果。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TalentPoolOperationResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub talent_pool_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub talent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

/// 生态考试操作结果。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct EcoExamOperationResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exam_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}
