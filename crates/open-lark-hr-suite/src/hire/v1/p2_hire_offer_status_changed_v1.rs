use serde::{Deserialize, Serialize};
/// Offer状态变更事件数据
///
/// 当Offer状态发生变更时触发此事件，包含状态变更的详细信息。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfferStatusChangedData {
    /// Offer ID
    pub offer_id: String,
    /// 投递ID
    pub application_id: String,
    /// 职位ID
    pub job_id: String,
    /// 人才ID
    pub talent_id: String,
    /// 原状态
    pub old_status: OfferStatus,
    /// 新状态
    pub new_status: OfferStatus,
    /// 状态变更时间（毫秒时间戳）
    pub change_time: i64,
    /// 操作者ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator_id: Option<String>,
    /// 变更原因
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Offer详情
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offer_info: Option<OfferInfo>,
}

/// Offer状态枚举
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OfferStatus {
    /// 草稿
    Draft,
    /// 待审批
    PendingApproval,
    /// 已通过
    Approved,
    /// 已拒绝
    Rejected,
    /// 已发出
    Sent,
    /// 候选人已接受
    Accepted,
    /// 候选人已拒绝
    Declined,
    /// 已撤回
    Withdrawn,
    /// 已过期
    Expired,
}

/// Offer基本信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfferInfo {
    /// Offer标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 职位名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_title: Option<String>,
    /// 部门
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department: Option<String>,
    /// 薪资范围
    #[serde(skip_serializing_if = "Option::is_none")]
    pub salary_range: Option<String>,
    /// 入职日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// Offer有效期至
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_date: Option<String>,
}
