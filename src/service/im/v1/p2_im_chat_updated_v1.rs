use serde::{Deserialize, Serialize};

use crate::event::{context::EventHeader, dispatcher::EventHandler};

#[derive(Debug, Serialize, Deserialize)]
pub struct P2ImChatUpdatedV1 {
    pub schema: String,
    pub header: EventHeader,
    pub event: P2ImChatUpdatedV1Data,
}

pub(crate) struct P2ImChatUpdatedV1ProcessorImpl<F>
where
    F: Fn(P2ImChatUpdatedV1) + 'static,
{
    f: F,
}

impl<F> EventHandler for P2ImChatUpdatedV1ProcessorImpl<F>
where
    F: Fn(P2ImChatUpdatedV1) + 'static + Sync + Send,
{
    fn handle(&self, payload: &[u8]) -> anyhow::Result<()> {
        let message: P2ImChatUpdatedV1 = serde_json::from_slice(payload)?;
        (self.f)(message);
        Ok(())
    }
}

impl<F> P2ImChatUpdatedV1ProcessorImpl<F>
where
    F: Fn(P2ImChatUpdatedV1) + 'static,
{
    pub(crate) fn new(f: F) -> Self {
        P2ImChatUpdatedV1ProcessorImpl { f }
    }
}

/// 聊天更新事件数据
#[derive(Debug, Serialize, Deserialize)]
pub struct P2ImChatUpdatedV1Data {
    /// 聊天 ID
    pub chat_id: String,
    /// 聊天类型 (p2p, group)
    pub chat_type: String,
    /// 操作者信息
    pub operator: EventOperator,
    /// 更新前的聊天信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_change: Option<ChatUpdateInfo>,
    /// 更新后的聊天信息
    pub after_change: ChatUpdateInfo,
    /// 更新时间 (Unix时间戳，单位：秒)
    pub update_time: String,
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

/// 聊天更新信息
#[derive(Debug, Serialize, Deserialize)]
pub struct ChatUpdateInfo {
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
    /// 聊天所有者
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<UserId>,
    /// 管理员列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_ids: Option<Vec<UserId>>,
}

/// 聊天设置
#[derive(Debug, Serialize, Deserialize)]
pub struct ChatSettings {
    /// 是否允许@所有人
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at_all_permission: Option<String>,
    /// 加群审批设置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_member_permission: Option<String>,
    /// 群可发现性
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_owner_add: Option<bool>,
    /// 允许分享群
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_allowed: Option<bool>,
    /// 群消息模式
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_mode: Option<String>,
    /// 是否开启消息历史
    #[serde(skip_serializing_if = "Option::is_none")]
    pub history_enabled: Option<bool>,
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
