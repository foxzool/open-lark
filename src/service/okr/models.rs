use serde::{Deserialize, Serialize};

/// 分页响应基础结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageResponse<T> {
    /// 数据项列表
    pub items: Vec<T>,
    /// 分页标记，用于获取下一页数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

/// OKR 周期状态
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PeriodStatus {
    /// 草稿状态
    Draft,
    /// 进行中
    Active,
    /// 已结束
    Ended,
    /// 已暂停
    Paused,
}

/// OKR 状态
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OkrStatus {
    /// 正常
    Normal,
    /// 已删除
    Deleted,
    /// 草稿
    Draft,
}

/// 进展记录类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProgressRecordType {
    /// 简单更新
    Simple,
    /// 详细更新
    Detail,
    /// 图片更新
    Image,
}

/// Key Result 类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KeyResultType {
    /// 数值型
    Numeric,
    /// 百分比型
    Percentage,
    /// 里程碑型
    Milestone,
}

/// 多语言文本
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct I18nText {
    /// 中文文本
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zh_cn: Option<String>,
    /// 英文文本
    #[serde(skip_serializing_if = "Option::is_none")]
    pub en_us: Option<String>,
    /// 日文文本
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ja_jp: Option<String>,
}

/// 用户信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// 用户ID
    pub user_id: String,
    /// 用户名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<I18nText>,
    /// 用户头像
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
}

/// OKR 周期
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Period {
    /// 周期ID
    pub period_id: String,
    /// 周期名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<I18nText>,
    /// 周期状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<PeriodStatus>,
    /// 开始时间（毫秒时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// 结束时间（毫秒时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// 创建时间（毫秒时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    /// 更新时间（毫秒时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modify_time: Option<i64>,
}

/// OKR 周期规则
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeriodRule {
    /// 规则ID
    pub rule_id: String,
    /// 周期ID
    pub period_id: String,
    /// 规则类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_type: Option<String>,
    /// 规则配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<serde_json::Value>,
}

/// Key Result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyResult {
    /// Key Result ID
    pub kr_id: String,
    /// 内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<I18nText>,
    /// 类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kr_type: Option<KeyResultType>,
    /// 当前值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_value: Option<f64>,
    /// 目标值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_value: Option<f64>,
    /// 进度百分比
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_rate: Option<f64>,
    /// 完成状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed: Option<bool>,
}

/// Objective
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Objective {
    /// Objective ID
    pub objective_id: String,
    /// 内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<I18nText>,
    /// 进度百分比
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_rate: Option<f64>,
    /// Key Results 列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_results: Option<Vec<KeyResult>>,
}

/// OKR
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Okr {
    /// OKR ID
    pub okr_id: String,
    /// 用户ID
    pub user_id: String,
    /// 周期ID
    pub period_id: String,
    /// 状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<OkrStatus>,
    /// Objectives 列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub objectives: Option<Vec<Objective>>,
    /// 创建时间（毫秒时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    /// 更新时间（毫秒时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modify_time: Option<i64>,
}

/// 进展记录附件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressAttachment {
    /// 附件ID
    pub attachment_id: String,
    /// 附件名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 附件URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// 附件类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_type: Option<String>,
    /// 文件大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
}

/// 进展记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressRecord {
    /// 进展记录ID
    pub progress_id: String,
    /// OKR ID
    pub okr_id: String,
    /// 内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// 记录类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_type: Option<ProgressRecordType>,
    /// 进度百分比
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_rate: Option<f64>,
    /// 附件列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<ProgressAttachment>>,
    /// 创建者
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<User>,
    /// 创建时间（毫秒时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    /// 更新时间（毫秒时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modify_time: Option<i64>,
}

/// OKR 复盘信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Review {
    /// 复盘ID
    pub review_id: String,
    /// OKR ID
    pub okr_id: String,
    /// 周期ID
    pub period_id: String,
    /// 复盘内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// 复盘评分
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
    /// 复盘者
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reviewer: Option<User>,
    /// 创建时间（毫秒时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
}
