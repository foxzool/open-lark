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
    /// `zh_cn` 字段。
    pub zh_cn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `en_us` 字段。
    pub en_us: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 可兼容普通字符串或多语言对象的文本字段。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum FlexibleText {
    /// `Plain` 变体。
    Plain(String),
    /// `I18n` 变体。
    I18n(I18nText),
}

impl FlexibleText {
    /// 提供 `zh_cn_or_plain` 能力。
    pub fn zh_cn_or_plain(&self) -> Option<&str> {
        match self {
            Self::Plain(value) => Some(value.as_str()),
            Self::I18n(value) => value.zh_cn.as_deref().or(value.en_us.as_deref()),
        }
    }
}

/// 常见的 ID + 名称引用对象。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct IdNameObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 标识。
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 名称。
    pub name: Option<I18nText>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 带 code + name 的区域对象。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CodeNameObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `code` 字段。
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 名称。
    pub name: Option<I18nText>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 通用分页响应壳。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct PaginatedResponse<T> {
    #[serde(default)]
    /// 结果项列表。
    pub items: Vec<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 下一页分页标记。
    pub page_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 是否还有更多结果。
    pub has_more: Option<bool>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 末尾长尾接口常见的目录/模板类对象。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CatalogItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 标识。
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `code` 字段。
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 名称。
    pub name: Option<FlexibleText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 标题。
    pub title: Option<FlexibleText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `i18n_name` 字段。
    pub i18n_name: Option<I18nText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `description` 字段。
    pub description: Option<FlexibleText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `status` 字段。
    pub status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `active_status` 字段。
    pub active_status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `version` 字段。
    pub version: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `parent_id` 字段。
    pub parent_id: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 面试任务摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct InterviewTaskSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `interview_id` 字段。
    pub interview_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `application_id` 字段。
    pub application_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `interview_round_id` 字段。
    pub interview_round_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `interview_round_name` 字段。
    pub interview_round_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `status` 字段。
    pub status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `interviewer` 字段。
    pub interviewer: Option<IdNameObject>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 候选人操作日志摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TalentOperationLogEntry {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 标识。
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 候选人 ID。
    pub talent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `application_id` 字段。
    pub application_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `operator` 字段。
    pub operator: Option<IdNameObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `operation_type` 字段。
    pub operation_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `operation_time` 字段。
    pub operation_time: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 多元化附加信息摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct DiversityInclusionRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `application_id` 字段。
    pub application_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 候选人 ID。
    pub talent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 性别。
    pub gender: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 职位广告发布记录摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct JobPublishRecordSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 标识。
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `job_id` 字段。
    pub job_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `channel_id` 字段。
    pub channel_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `publish_status` 字段。
    pub publish_status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `status` 字段。
    pub status: Option<i32>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 长尾接口常见的简单操作结果。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct GenericOperationResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 标识。
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `task_id` 字段。
    pub task_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `job_id` 字段。
    pub job_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 候选人 ID。
    pub talent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `application_id` 字段。
    pub application_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `publish_id` 字段。
    pub publish_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `exam_id` 字段。
    pub exam_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `status` 字段。
    pub status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `result` 字段。
    pub result: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `success` 字段。
    pub success: Option<bool>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 奖励金额。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct BonusAmount {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `point_bonus` 字段。
    pub point_bonus: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `cash_bonus` 字段。
    pub cash_bonus: Option<Vec<CashAmount>>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 现金奖励金额。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CashAmount {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `currency_type` 字段。
    pub currency_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `amount` 字段。
    pub amount: Option<f64>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 附件元数据。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct AttachmentMeta {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `file_id` 字段。
    pub file_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `file_name` 字段。
    pub file_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `content_type` 字段。
    pub content_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `file_size` 字段。
    pub file_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `create_time` 字段。
    pub create_time: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 分数信息。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ScoreInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 分数。
    pub score: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `total_score` 字段。
    pub total_score: Option<f64>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 招聘备注。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct NoteRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 标识。
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 候选人 ID。
    pub talent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `application_id` 字段。
    pub application_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `is_private` 字段。
    pub is_private: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `create_time` 字段。
    pub create_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `modify_time` 字段。
    pub modify_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `creator_id` 字段。
    pub creator_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 内容。
    pub content: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 招聘附件/文件句柄。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct HireAttachment {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 标识。
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `url` 字段。
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 名称。
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `mime` 字段。
    pub mime: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `create_time` 字段。
    pub create_time: Option<i64>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// `zh_name` / `en_name` 形式的双语文本。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct LocalizedLabel {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `zh_name` 字段。
    pub zh_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `en_name` 字段。
    pub en_name: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 投递所关联的职位摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ApplicationJobInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `job_id` 字段。
    pub job_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `job_name` 字段。
    pub job_name: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 投递所关联的候选人摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ApplicationTalentInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 候选人 ID。
    pub talent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `talent_name` 字段。
    pub talent_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `mobile` 字段。
    pub mobile: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 邮箱地址。
    pub email: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 投递关联的 offer 摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ApplicationOfferInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `offer_id` 字段。
    pub offer_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `offer_status` 字段。
    pub offer_status: Option<i32>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 投递摘要信息。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ApplicationSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `application_id` 字段。
    pub application_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 候选人 ID。
    pub talent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `job_id` 字段。
    pub job_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `application_status` 字段。
    pub application_status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `stage_id` 字段。
    pub stage_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `stage_name` 字段。
    pub stage_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `job_info` 字段。
    pub job_info: Option<ApplicationJobInfo>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 投递关联的面试记录摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ApplicationInterviewRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `interview_id` 字段。
    pub interview_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `interview_round_id` 字段。
    pub interview_round_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `interview_round_name` 字段。
    pub interview_round_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `status` 字段。
    pub status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `interviewer` 字段。
    pub interviewer: Option<IdNameObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 分数。
    pub score: Option<ScoreInfo>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 职位上的招聘相关人员。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct JobRecruiterRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `recruiter_id` 字段。
    pub recruiter_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `manager_id` 字段。
    pub manager_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `user_id` 字段。
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 名称。
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `role` 字段。
    pub role: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `role_type` 字段。
    pub role_type: Option<i32>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 职位摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct JobSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `job_id` 字段。
    pub job_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `job_name` 字段。
    pub job_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `active_status` 字段。
    pub active_status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `process_id` 字段。
    pub process_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `process_name` 字段。
    pub process_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `department_id` 字段。
    pub department_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `job_description` 字段。
    pub job_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `recruiters` 字段。
    pub recruiters: Option<Vec<JobRecruiterRecord>>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 职位设置摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct JobConfigInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `job_id` 字段。
    pub job_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `process_id` 字段。
    pub process_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `process_name` 字段。
    pub process_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `job_requirement_schema_id` 字段。
    pub job_requirement_schema_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `interview_registration_schema_id` 字段。
    pub interview_registration_schema_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `offer_application_form_id` 字段。
    pub offer_application_form_id: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 外部投递摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ExternalApplicationSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `external_application_id` 字段。
    pub external_application_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `application_id` 字段。
    pub application_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 候选人 ID。
    pub talent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `job_id` 字段。
    pub job_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `status` 字段。
    pub status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `source_name` 字段。
    pub source_name: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 外部 Offer 摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ExternalOfferSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `external_offer_id` 字段。
    pub external_offer_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `offer_id` 字段。
    pub offer_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `application_id` 字段。
    pub application_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 候选人 ID。
    pub talent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `status` 字段。
    pub status: Option<i32>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 招聘官网推广渠道摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct WebsiteChannelSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `channel_id` 字段。
    pub channel_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `website_id` 字段。
    pub website_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 名称。
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `code` 字段。
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `active_status` 字段。
    pub active_status: Option<i32>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 招聘官网投递任务结果。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct WebsiteDeliveryTaskResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `delivery_task_id` 字段。
    pub delivery_task_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `application_id` 字段。
    pub application_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 候选人 ID。
    pub talent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `status` 字段。
    pub status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `error_message` 字段。
    pub error_message: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 招聘官网职位广告摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct WebsiteJobPostSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `job_post_id` 字段。
    pub job_post_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `website_id` 字段。
    pub website_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `job_id` 字段。
    pub job_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 标题。
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `active_status` 字段。
    pub active_status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `job_channel_id` 字段。
    pub job_channel_id: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 招聘官网站点用户摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct WebsiteSiteUserSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `site_user_id` 字段。
    pub site_user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `website_id` 字段。
    pub website_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `user_id` 字段。
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 邮箱地址。
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `mobile` 字段。
    pub mobile: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 猎头供应商摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct AgencySummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `agency_id` 字段。
    pub agency_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 名称。
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `code` 字段。
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `active_status` 字段。
    pub active_status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `status` 字段。
    pub status: Option<i32>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 猎头供应商下的猎头账号摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct AgencyAccountSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `agency_account_id` 字段。
    pub agency_account_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `agency_id` 字段。
    pub agency_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `user_id` 字段。
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 名称。
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `status` 字段。
    pub status: Option<i32>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 猎头保护期摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct AgencyProtectionSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `protection_id` 字段。
    pub protection_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `agency_id` 字段。
    pub agency_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 候选人 ID。
    pub talent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `expiration_time` 字段。
    pub expiration_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `status` 字段。
    pub status: Option<i32>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 人才外部信息摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TalentExternalInfoRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `external_info_id` 字段。
    pub external_info_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 候选人 ID。
    pub talent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `source_name` 字段。
    pub source_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `external_id` 字段。
    pub external_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `status` 字段。
    pub status: Option<i32>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 招聘需求摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct JobRequirementSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `job_requirement_id` 字段。
    pub job_requirement_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `job_id` 字段。
    pub job_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 标题。
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 名称。
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `status` 字段。
    pub status: Option<i32>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 投递操作结果摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ApplicationOperationResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `application_id` 字段。
    pub application_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `stage_id` 字段。
    pub stage_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `stage_name` 字段。
    pub stage_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `offer_id` 字段。
    pub offer_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `employee_id` 字段。
    pub employee_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `onboard_status` 字段。
    pub onboard_status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `result` 字段。
    pub result: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `success` 字段。
    pub success: Option<bool>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 外部背调摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ExternalBackgroundCheckSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `external_background_check_id` 字段。
    pub external_background_check_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `application_id` 字段。
    pub application_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 候选人 ID。
    pub talent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `status` 字段。
    pub status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `vendor_name` 字段。
    pub vendor_name: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 外部面试摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ExternalInterviewSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `external_interview_id` 字段。
    pub external_interview_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `application_id` 字段。
    pub application_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 候选人 ID。
    pub talent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `status` 字段。
    pub status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `interview_round_name` 字段。
    pub interview_round_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 开始时间。
    pub start_time: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 三方协议摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TripartiteAgreementSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `agreement_id` 字段。
    pub agreement_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `application_id` 字段。
    pub application_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 候选人 ID。
    pub talent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `status` 字段。
    pub status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `sign_status` 字段。
    pub sign_status: Option<i32>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 生态自定义字段摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct EcoCustomFieldSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `custom_field_id` 字段。
    pub custom_field_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 名称。
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `code` 字段。
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `status` 字段。
    pub status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `active_status` 字段。
    pub active_status: Option<i32>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 生态背调套餐摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct EcoBackgroundCheckPackageSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `package_id` 字段。
    pub package_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 名称。
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `status` 字段。
    pub status: Option<i32>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 生态考试试卷摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct EcoExamPaperSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `paper_id` 字段。
    pub paper_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 名称。
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `status` 字段。
    pub status: Option<i32>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 生态流程操作结果。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct EcoOperationResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `custom_field_id` 字段。
    pub custom_field_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `package_id` 字段。
    pub package_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `paper_id` 字段。
    pub paper_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `background_check_id` 字段。
    pub background_check_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `order_id` 字段。
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `status` 字段。
    pub status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `progress` 字段。
    pub progress: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `result` 字段。
    pub result: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `success` 字段。
    pub success: Option<bool>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 入职员工摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct EmployeeSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `employee_id` 字段。
    pub employee_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `application_id` 字段。
    pub application_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 候选人 ID。
    pub talent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `status` 字段。
    pub status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `onboard_status` 字段。
    pub onboard_status: Option<i32>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 内推账户操作结果。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ReferralAccountOperationResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `account_id` 字段。
    pub account_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `status` 字段。
    pub status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `result` 字段。
    pub result: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `success` 字段。
    pub success: Option<bool>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 外部面评操作结果。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ExternalInterviewAssessmentResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `external_interview_assessment_id` 字段。
    pub external_interview_assessment_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `external_interview_id` 字段。
    pub external_interview_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `status` 字段。
    pub status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `result` 字段。
    pub result: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `success` 字段。
    pub success: Option<bool>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 面试摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct InterviewSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `interview_id` 字段。
    pub interview_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `application_id` 字段。
    pub application_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 候选人 ID。
    pub talent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `status` 字段。
    pub status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `interview_round_name` 字段。
    pub interview_round_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 开始时间。
    pub start_time: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 背调订单摘要。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct BackgroundCheckOrderSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `order_id` 字段。
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `application_id` 字段。
    pub application_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 候选人 ID。
    pub talent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `status` 字段。
    pub status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `vendor_name` 字段。
    pub vendor_name: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 外部内推奖励操作结果。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ExternalReferralRewardResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `external_referral_reward_id` 字段。
    pub external_referral_reward_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `account_id` 字段。
    pub account_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `amount` 字段。
    pub amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `result` 字段。
    pub result: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `success` 字段。
    pub success: Option<bool>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 人才库操作结果。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TalentPoolOperationResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `talent_pool_id` 字段。
    pub talent_pool_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 候选人 ID。
    pub talent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `result` 字段。
    pub result: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `success` 字段。
    pub success: Option<bool>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 生态考试操作结果。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct EcoExamOperationResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `exam_id` 字段。
    pub exam_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `status` 字段。
    pub status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `result` 字段。
    pub result: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `success` 字段。
    pub success: Option<bool>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// Offer 自定义字段操作结果。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct OfferCustomFieldOperationResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `offer_custom_field_id` 字段。
    pub offer_custom_field_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `result` 字段。
    pub result: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `success` 字段。
    pub success: Option<bool>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 面试官更新结果。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct InterviewerOperationResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `interviewer_id` 字段。
    pub interviewer_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `user_id` 字段。
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `verify_status` 字段。
    pub verify_status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `result` 字段。
    pub result: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `success` 字段。
    pub success: Option<bool>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 职位 manager 操作结果。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct JobManagerOperationResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `job_id` 字段。
    pub job_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `manager_id` 字段。
    pub manager_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `result` 字段。
    pub result: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `success` 字段。
    pub success: Option<bool>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}
