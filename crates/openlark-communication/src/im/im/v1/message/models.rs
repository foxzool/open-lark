//! 消息相关模型（不算 API）

use serde::{Deserialize, Serialize};

/// 发送消息时的接收者 ID 类型（receive_id_type）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReceiveIdType {
    OpenId,
    UnionId,
    UserId,
    Email,
    ChatId,
}

impl ReceiveIdType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::OpenId => "open_id",
            Self::UnionId => "union_id",
            Self::UserId => "user_id",
            Self::Email => "email",
            Self::ChatId => "chat_id",
        }
    }
}

/// 查询消息内容时的用户 ID 类型（user_id_type）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UserIdType {
    OpenId,
    UnionId,
    UserId,
}

impl UserIdType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::OpenId => "open_id",
            Self::UnionId => "union_id",
            Self::UserId => "user_id",
        }
    }
}

/// 获取会话历史消息时的容器类型（container_id_type）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContainerIdType {
    Chat,
    Thread,
}

impl ContainerIdType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Chat => "chat",
            Self::Thread => "thread",
        }
    }
}

/// 消息排序方式（sort_type）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SortType {
    ByCreateTimeAsc,
    ByCreateTimeDesc,
}

impl SortType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ByCreateTimeAsc => "ByCreateTimeAsc",
            Self::ByCreateTimeDesc => "ByCreateTimeDesc",
        }
    }
}

