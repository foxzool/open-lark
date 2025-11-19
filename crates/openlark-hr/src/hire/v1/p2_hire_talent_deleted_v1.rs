use serde::{Deserialize, Serialize};
/// 人才删除事件数据
///
/// 当人才被删除时触发此事件，包含被删除人才的基本信息。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TalentDeletedData {
    /// 人才ID
    pub talent_id: String,
    /// 人才姓名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 人才邮箱
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 人才电话
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// 删除时间（毫秒时间戳）
    pub delete_time: i64,
    /// 删除操作者ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator_id: Option<String>,
    /// 删除原因
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// 人才来源
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// 创建时间（毫秒时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
}
