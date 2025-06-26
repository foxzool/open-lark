use serde::{Deserialize, Serialize};

/// 消息流卡片状态
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FeedCardStatus {
    /// 激活状态
    #[serde(rename = "active")]
    Active,
    /// 失效状态
    #[serde(rename = "inactive")]
    Inactive,
}

/// 用户ID类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UserIdType {
    /// 用户ID
    #[serde(rename = "user_id")]
    UserId,
    /// union_id
    #[serde(rename = "union_id")]
    UnionId,
    /// open_id
    #[serde(rename = "open_id")]
    OpenId,
}

impl UserIdType {
    pub fn as_str(&self) -> &'static str {
        match self {
            UserIdType::UserId => "user_id",
            UserIdType::UnionId => "union_id",
            UserIdType::OpenId => "open_id",
        }
    }
}

/// 消息流卡片信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedCard {
    /// 卡片ID
    pub card_id: String,
    /// 卡片标题
    pub title: Option<String>,
    /// 卡片内容
    pub content: Option<String>,
    /// 卡片状态
    pub status: Option<FeedCardStatus>,
    /// 创建时间
    pub create_time: Option<String>,
    /// 更新时间
    pub update_time: Option<String>,
}

/// 按钮信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ButtonInfo {
    /// 按钮ID
    pub button_id: String,
    /// 按钮文本
    pub text: String,
    /// 按钮类型
    pub button_type: Option<String>,
    /// 按钮行为
    pub action: Option<String>,
}

/// 即时提醒配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelyNotification {
    /// 提醒类型
    pub notification_type: String,
    /// 提醒消息
    pub message: String,
    /// 目标用户
    pub target_users: Vec<String>,
}
