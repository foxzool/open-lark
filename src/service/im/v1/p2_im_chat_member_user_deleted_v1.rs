use serde::{Deserialize, Serialize};

use crate::event::{context::EventHeader, dispatcher::EventHandler};

#[derive(Debug, Serialize, Deserialize)]
pub struct P2ImChatMemberUserDeletedV1 {
    pub schema: String,
    pub header: EventHeader,
    pub event: P2ImChatMemberUserDeletedV1Data,
}

pub(crate) struct P2ImChatMemberUserDeletedV1ProcessorImpl<F>
where
    F: Fn(P2ImChatMemberUserDeletedV1) + 'static,
{
    f: F,
}

impl<F> EventHandler for P2ImChatMemberUserDeletedV1ProcessorImpl<F>
where
    F: Fn(P2ImChatMemberUserDeletedV1) + 'static + Sync + Send,
{
    fn handle(&self, payload: &[u8]) -> anyhow::Result<()> {
        let message: P2ImChatMemberUserDeletedV1 = serde_json::from_slice(payload)?;
        (self.f)(message);
        Ok(())
    }
}

impl<F> P2ImChatMemberUserDeletedV1ProcessorImpl<F>
where
    F: Fn(P2ImChatMemberUserDeletedV1) + 'static,
{
    pub(crate) fn new(f: F) -> Self {
        P2ImChatMemberUserDeletedV1ProcessorImpl { f }
    }
}

/// 用户移出聊天事件数据
#[derive(Debug, Serialize, Deserialize)]
pub struct P2ImChatMemberUserDeletedV1Data {
    /// 聊天 ID
    pub chat_id: String,
    /// 聊天类型 (group)
    pub chat_type: String,
    /// 操作者信息 (移除用户的人)
    pub operator: EventOperator,
    /// 被移除的用户列表
    pub users: Vec<DeletedUser>,
    /// 移除时间 (Unix时间戳，单位：秒)
    pub delete_time: String,
    /// 移除原因
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<DeleteReason>,
}

/// 事件操作者信息
#[derive(Debug, Serialize, Deserialize)]
pub struct EventOperator {
    /// 操作者用户 ID
    pub operator_id: UserId,
    /// 操作者类型 (user, bot, app, system)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator_type: Option<String>,
}

/// 被移除的用户信息
#[derive(Debug, Serialize, Deserialize)]
pub struct DeletedUser {
    /// 用户 ID
    pub user_id: UserId,
    /// 用户名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 用户类型 (user)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_type: Option<String>,
    /// 之前的成员角色 (member, admin)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_role: Option<String>,
    /// 加入时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_time: Option<String>,
}

/// 移除原因
#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteReason {
    /// 原因类型 (kicked, left, disbanded, expired)
    pub reason_type: String,
    /// 详细描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
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
