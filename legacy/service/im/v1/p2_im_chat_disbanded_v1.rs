use serde::{Deserialize, Serialize};

use crate::event::{context::EventHeader, dispatcher::EventHandler};

#[derive(Debug, Serialize, Deserialize)]
pub struct P2ImChatDisbandedV1 {
    pub schema: String,
    pub header: EventHeader,
    pub event: P2ImChatDisbandedV1Data,
}

pub(crate) struct P2ImChatDisbandedV1ProcessorImpl<F>
where
    F: Fn(P2ImChatDisbandedV1) + 'static,
{
    f: F,
}

impl<F> EventHandler for P2ImChatDisbandedV1ProcessorImpl<F>
where
    F: Fn(P2ImChatDisbandedV1) + 'static + Sync + Send,
{
    fn handle(&self, payload: &[u8]) -> anyhow::Result<()> {
        let message: P2ImChatDisbandedV1 = serde_json::from_slice(payload)?;
        (self.f)(message);
        Ok(())
    }
}

impl<F> P2ImChatDisbandedV1ProcessorImpl<F>
where
    F: Fn(P2ImChatDisbandedV1) + 'static,
{
    pub(crate) fn new(f: F) -> Self {
        P2ImChatDisbandedV1ProcessorImpl { f }
    }
}

/// 聊天解散事件数据
#[derive(Debug, Serialize, Deserialize)]
pub struct P2ImChatDisbandedV1Data {
    /// 聊天 ID
    pub chat_id: String,
    /// 聊天类型 (group)
    pub chat_type: String,
    /// 操作者信息
    pub operator: EventOperator,
    /// 聊天名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 解散时间 (Unix时间戳，单位：秒)
    pub disband_time: String,
    /// 解散原因
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// 事件操作者信息
#[derive(Debug, Serialize, Deserialize)]
pub struct EventOperator {
    /// 操作者用户 ID
    pub operator_id: UserId,
    /// 操作者类型 (user, bot, app)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator_type: Option<String>,
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
