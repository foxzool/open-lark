use serde::{Deserialize, Serialize};

/// 卡片实体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    /// 卡片ID
    pub card_id: Option<String>,
    /// 卡片标题
    pub title: Option<String>,
    /// 卡片描述
    pub description: Option<String>,
    /// 卡片JSON内容
    pub card_json: Option<serde_json::Value>,
    /// 卡片状态
    pub status: Option<CardStatus>,
    /// 创建时间
    pub create_time: Option<String>,
    /// 更新时间
    pub update_time: Option<String>,
}

/// 卡片状态
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CardStatus {
    /// 草稿
    Draft,
    /// 已发布
    Published,
    /// 已删除
    Deleted,
}

/// 卡片组件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardElement {
    /// 组件ID
    pub element_id: Option<String>,
    /// 组件类型
    pub element_type: Option<String>,
    /// 组件内容
    pub content: Option<serde_json::Value>,
    /// 组件属性
    pub properties: Option<serde_json::Value>,
    /// 父组件ID
    pub parent_id: Option<String>,
    /// 创建时间
    pub create_time: Option<String>,
    /// 更新时间
    pub update_time: Option<String>,
}

/// 用户ID类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UserIdType {
    /// Open ID
    OpenId,
    /// Union ID
    UnionId,
    /// User ID
    UserId,
}

impl std::fmt::Display for UserIdType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserIdType::OpenId => write!(f, "open_id"),
            UserIdType::UnionId => write!(f, "union_id"),
            UserIdType::UserId => write!(f, "user_id"),
        }
    }
}

/// 卡片配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardSettings {
    /// 是否启用交互
    pub enable_interaction: Option<bool>,
    /// 卡片主题
    pub theme: Option<String>,
    /// 自定义配置
    pub custom_config: Option<serde_json::Value>,
}

/// 批量更新操作
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchUpdateOperation {
    /// 操作类型
    pub operation: String,
    /// 目标路径
    pub path: String,
    /// 操作值
    pub value: Option<serde_json::Value>,
}
