use serde::{Deserialize, Serialize};

/// 绩效结果开通事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct P2PerformanceResultOpenedV1 {
    /// 事件ID
    pub event_id: String,
    /// 事件类型
    pub event_type: String,
    /// 事件创建时间戳
    pub created_time: String,
    /// 事件内容
    pub event: PerformanceResultOpenedEvent,
}

/// 绩效结果开通事件内容
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceResultOpenedEvent {
    /// 周期ID
    pub semester_id: String,
    /// 项目ID
    pub activity_id: String,
    /// 被评估人ID列表
    pub reviewee_ids: Vec<String>,
    /// 开通时间戳
    pub opened_at: i64,
}

/// 绩效详情变更事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct P2PerformanceDetailChangedV1 {
    /// 事件ID
    pub event_id: String,
    /// 事件类型
    pub event_type: String,
    /// 事件创建时间戳
    pub created_time: String,
    /// 事件内容
    pub event: PerformanceDetailChangedEvent,
}

/// 绩效详情变更事件内容
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceDetailChangedEvent {
    /// 项目ID
    pub activity_id: String,
    /// 被评估人ID
    pub reviewee_id: String,
    /// 评估人ID
    pub reviewer_id: String,
    /// 变更的评估项ID列表
    pub changed_item_ids: Vec<String>,
    /// 变更类型 (created, updated, deleted)
    pub change_type: String,
    /// 变更时间戳
    pub changed_at: i64,
}
