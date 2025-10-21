use serde::{Deserialize, Serialize};

use crate::event::{context::EventHeader, dispatcher::EventHandler};

#[derive(Debug, Serialize, Deserialize)]
pub struct P2ImMessageRecalledV1 {
    pub schema: String,
    pub header: EventHeader,
    pub event: P2ImMessageRecalledV1Data,
}

pub(crate) struct P2ImMessageRecalledV1ProcessorImpl<F>
where
    F: Fn(P2ImMessageRecalledV1) + 'static,
{
    f: F,
}

impl<F> EventHandler for P2ImMessageRecalledV1ProcessorImpl<F>
where
    F: Fn(P2ImMessageRecalledV1) + 'static + Sync + Send,
{
    fn handle(&self, payload: &[u8]) -> anyhow::Result<()> {
        let message: P2ImMessageRecalledV1 = serde_json::from_slice(payload)?;
        (self.f)(message);
        Ok(())
    }
}

impl<F> P2ImMessageRecalledV1ProcessorImpl<F>
where
    F: Fn(P2ImMessageRecalledV1) + 'static,
{
    pub(crate) fn new(f: F) -> Self {
        P2ImMessageRecalledV1ProcessorImpl { f }
    }
}

/// 消息撤回事件数据
#[derive(Debug, Serialize, Deserialize)]
pub struct P2ImMessageRecalledV1Data {
    /// 撤回的消息ID
    pub message_id: String,
    /// 撤回者信息
    pub operator: EventOperator,
    /// 聊天信息
    pub chat_info: EventChatInfo,
    /// 撤回时间 (Unix时间戳，单位：秒)
    pub recall_time: String,
}

/// 事件操作者信息
#[derive(Debug, Serialize, Deserialize)]
pub struct EventOperator {
    /// 操作者用户 ID
    pub operator_id: UserId,
    /// 操作者类型
    pub operator_type: String,
}

/// 聊天信息
#[derive(Debug, Serialize, Deserialize)]
pub struct EventChatInfo {
    /// 聊天 ID
    pub chat_id: String,
    /// 聊天类型 (p2p, group)
    pub chat_type: String,
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
