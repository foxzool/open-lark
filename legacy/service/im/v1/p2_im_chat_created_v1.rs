use serde::{Deserialize, Serialize};

use crate::event::{context::EventHeader, dispatcher::EventHandler};

#[derive(Debug, Serialize, Deserialize)]
pub struct P2ImChatCreatedV1 {
    pub schema: String,
    pub header: EventHeader,
    pub event: P2ImChatCreatedV1Data,
}

pub(crate) struct P2ImChatCreatedV1ProcessorImpl<F>
where
    F: Fn(P2ImChatCreatedV1) + 'static,
{
    f: F,
}

impl<F> EventHandler for P2ImChatCreatedV1ProcessorImpl<F>
where
    F: Fn(P2ImChatCreatedV1) + 'static + Sync + Send,
{
    fn handle(&self, payload: &[u8]) -> anyhow::Result<()> {
        let message: P2ImChatCreatedV1 = serde_json::from_slice(payload)?;
        (self.f)(message);
        Ok(())
    }
}

impl<F> P2ImChatCreatedV1ProcessorImpl<F>
where
    F: Fn(P2ImChatCreatedV1) + 'static,
{
    pub(crate) fn new(f: F) -> Self {
        P2ImChatCreatedV1ProcessorImpl { f }
    }
}

/// 聊天创建事件数据
#[derive(Debug, Serialize, Deserialize)]
pub struct P2ImChatCreatedV1Data {
    /// 聊天 ID
    pub chat_id: String,
    /// 聊天类型 (p2p, group)
    pub chat_type: String,
    /// 创建者信息
    pub creator: EventUser,
    /// 聊天名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 聊天描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 聊天头像
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    /// 聊天设置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_settings: Option<ChatSettings>,
    /// 聊天成员列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<ChatMember>>,
    /// 创建时间 (Unix时间戳，单位：秒)
    pub create_time: String,
}

/// 事件中的用户信息
#[derive(Debug, Serialize, Deserialize)]
pub struct EventUser {
    /// 用户 ID
    pub user_id: UserId,
    /// 用户名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 用户类型 (user, bot, app)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_type: Option<String>,
}

/// 聊天设置
#[derive(Debug, Serialize, Deserialize)]
pub struct ChatSettings {
    /// 是否允许@所有人
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at_all_permission: Option<String>,
    /// 加群审批
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_member_permission: Option<String>,
    /// 群可发现性
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_owner_add: Option<bool>,
    /// 允许分享群
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_allowed: Option<bool>,
}

/// 聊天成员
#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMember {
    /// 成员用户 ID
    pub user_id: UserId,
    /// 成员类型 (user, bot, app)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_type: Option<String>,
    /// 成员角色 (owner, admin, member)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// 加入时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_time: Option<String>,
}

/// 用户 ID 信息
#[derive(Debug, Serialize, Deserialize)]
pub struct UserId {
    /// 用户的 union id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub union_id: Option<String>,
    /// 用户的 user id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 用户的 open id
    pub open_id: String,
}
