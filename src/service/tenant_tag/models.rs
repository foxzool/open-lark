use serde::{Deserialize, Serialize};

/// 标签类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TagType {
    /// 群标签
    #[serde(rename = "chat")]
    Chat,
}

impl TagType {
    pub fn as_str(&self) -> &'static str {
        match self {
            TagType::Chat => "chat",
        }
    }
}

/// 标签状态
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TagStatus {
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

/// 标签信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    /// 标签ID
    pub tag_id: String,
    /// 标签名称
    pub name: String,
    /// 标签描述
    pub description: Option<String>,
    /// 标签类型
    pub tag_type: TagType,
    /// 标签状态
    pub status: TagStatus,
    /// 创建时间
    pub create_time: Option<String>,
    /// 更新时间
    pub update_time: Option<String>,
    /// 创建者ID
    pub creator_id: Option<String>,
}

/// 标签绑定关系
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagBinding {
    /// 标签ID
    pub tag_id: String,
    /// 实体ID（如群ID）
    pub entity_id: String,
    /// 实体类型
    pub entity_type: String,
    /// 绑定时间
    pub bind_time: Option<String>,
    /// 绑定者ID
    pub binder_id: Option<String>,
}