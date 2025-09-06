use serde::{Deserialize, Serialize};

use crate::{
    event::{context::EventHeader, dispatcher::EventHandler},
    service::im::v1::p2_im_message_receive_v1::UserId,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct P2ImMessageReadV1 {
    pub schema: String,
    pub header: EventHeader,
    pub event: P2ImMessageMessageReadV1Data,
}

/// 消息已读事件数据
#[derive(Debug, Serialize, Deserialize)]
pub struct P2ImMessageMessageReadV1Data {
    /// 消息阅读者信息
    pub reader: EventReader,
    /// 已读消息ID列表
    pub message_id_list: Vec<String>,
}

/// 消息阅读者信息
#[derive(Debug, Serialize, Deserialize)]
pub struct EventReader {
    /// 阅读时间戳（毫秒）
    pub read_time: String,
    /// 阅读者ID信息
    pub reader_id: UserId,
    /// tenant key，为租户在飞书上的唯一标识
    pub tenant_key: String,
}

pub struct P2ImMessageReadV1ProcessorImpl<F>
where
    F: Fn(P2ImMessageReadV1) + 'static,
{
    f: F,
}

impl<F> P2ImMessageReadV1ProcessorImpl<F>
where
    F: Fn(P2ImMessageReadV1) + 'static,
{
    pub fn new(f: F) -> Self {
        P2ImMessageReadV1ProcessorImpl { f }
    }
}

impl<F> EventHandler for P2ImMessageReadV1ProcessorImpl<F>
where
    F: Fn(P2ImMessageReadV1) + 'static + Sync + Send,
{
    fn handle(&self, payload: &[u8]) -> anyhow::Result<()> {
        let message: P2ImMessageReadV1 = serde_json::from_slice(payload)?;
        (self.f)(message);
        Ok(())
    }
}
