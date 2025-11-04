use serde::{Deserialize, Serialize};
/// 投递阶段变更事件数据
///
/// 当候选人投递状态发生变更时触发此事件，包含投递的阶段转移信息。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationStageChangedData {
    /// 投递ID
    pub application_id: String,
    /// 职位ID
    pub job_id: String,
    /// 人才ID
    pub talent_id: String,
    /// 原阶段ID
    pub old_stage_id: String,
    /// 新阶段ID
    pub new_stage_id: String,
    /// 变更时间（毫秒时间戳）
    pub change_time: i64,
    /// 变更操作者ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator_id: Option<String>,
    /// 变更原因
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// 备注信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}
