//! OKR服务数据模型
//!
//! 定义OKR目标管理相关的数据结构，包括：
//! - OKR周期管理
//! - 目标(Objective)和关键结果(Key Result)
//! - 进展记录和复盘
//! - 用户和组织权限管理

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
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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

// ==================== 请求和响应模型 ====================

/// 创建周期请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePeriodRequest {
    /// 周期名称
    pub name: I18nText,
    /// 开始时间（毫秒时间戳）
    pub start_time: i64,
    /// 结束时间（毫秒时间戳）
    pub end_time: i64,
    /// 描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<I18nText>,
}

/// 更新周期请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePeriodRequest {
    /// 周期ID
    pub period_id: String,
    /// 周期名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<I18nText>,
    /// 开始时间（毫秒时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// 结束时间（毫秒时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// 状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<PeriodStatus>,
}

/// 获取周期请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPeriodRequest {
    /// 周期ID
    pub period_id: String,
}

/// 列出周期请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListPeriodsRequest {
    /// 状态筛选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<PeriodStatus>,
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 页面标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 创建OKR请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOkrRequest {
    /// 用户ID
    pub user_id: String,
    /// 周期ID
    pub period_id: String,
    /// 目标列表
    pub objectives: Vec<CreateObjectiveRequest>,
}

/// 创建目标请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateObjectiveRequest {
    /// 目标内容
    pub content: I18nText,
    /// 关键结果列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_results: Option<Vec<CreateKeyResultRequest>>,
}

/// 创建关键结果请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateKeyResultRequest {
    /// 关键结果内容
    pub content: I18nText,
    /// 类型
    pub kr_type: KeyResultType,
    /// 目标值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_value: Option<f64>,
}

/// 更新OKR请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateOkrRequest {
    /// OKR ID
    pub okr_id: String,
    /// 目标列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub objectives: Option<Vec<UpdateObjectiveRequest>>,
}

/// 更新目标请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateObjectiveRequest {
    /// 目标ID
    pub objective_id: String,
    /// 目标内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<I18nText>,
    /// 关键结果列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_results: Option<Vec<UpdateKeyResultRequest>>,
}

/// 更新关键结果请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateKeyResultRequest {
    /// 关键结果ID
    pub kr_id: String,
    /// 关键结果内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<I18nText>,
    /// 当前值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_value: Option<f64>,
    /// 目标值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_value: Option<f64>,
    /// 完成状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed: Option<bool>,
}

/// 获取OKR请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetOkrRequest {
    /// OKR ID
    pub okr_id: String,
}

/// 用户OKR列表请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserOkrListRequest {
    /// 用户ID筛选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 周期ID筛选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_id: Option<String>,
    /// 状态筛选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<OkrStatus>,
    /// 页面标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 每页数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

/// 批量获取OKR请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchOkrRequest {
    /// OKR ID 列表
    pub okr_ids: Vec<String>,
    /// 是否包含详细信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_details: Option<bool>,
}

/// 删除OKR请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteOkrRequest {
    /// OKR ID
    pub okr_id: String,
}

/// 创建进展记录请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProgressRecordRequest {
    /// OKR ID
    pub okr_id: String,
    /// 内容
    pub content: String,
    /// 记录类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_type: Option<ProgressRecordType>,
    /// 进度百分比
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_rate: Option<f64>,
    /// 附件列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<ProgressAttachment>>,
}

/// 获取进展记录列表请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListProgressRecordsRequest {
    /// OKR ID
    pub okr_id: String,
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 页面标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 创建复盘请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateReviewRequest {
    /// OKR ID
    pub okr_id: String,
    /// 复盘内容
    pub content: String,
    /// 复盘评分
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
}

/// 获取复盘请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetReviewRequest {
    /// 复盘ID
    pub review_id: String,
}

// ==================== 响应模型 ====================

/// 周期响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeriodResponse {
    pub code: i32,
    pub msg: String,
    pub data: Option<Period>,
}

/// 周期列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeriodsResponse {
    pub code: i32,
    pub msg: String,
    pub data: Option<PageResponse<Period>>,
}

/// OKR响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OkrResponse {
    pub code: i32,
    pub msg: String,
    pub data: Option<Okr>,
}

/// 用户OKR列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserOkrListResponse {
    pub code: i32,
    pub msg: String,
    pub data: Option<PageResponse<Okr>>,
}

/// 批量OKR响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchOkrResponse {
    pub code: i32,
    pub msg: String,
    pub data: Option<Vec<Okr>>,
}

/// 进展记录响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressRecordResponse {
    pub code: i32,
    pub msg: String,
    pub data: Option<ProgressRecord>,
}

/// 进展记录列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressRecordsResponse {
    pub code: i32,
    pub msg: String,
    pub data: Option<PageResponse<ProgressRecord>>,
}

/// 复盘响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewResponse {
    pub code: i32,
    pub msg: String,
    pub data: Option<Review>,
}

/// 空响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmptyResponse {
    pub code: i32,
    pub msg: String,
}