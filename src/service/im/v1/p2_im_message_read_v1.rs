use serde::{Deserialize, Serialize};

use crate::{
    event::{context::EventHeader, dispatcher::EventHandler},
    service::im::v1::p2_im_message_receive_v1::{EventMessage, EventSender},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct P2ImMessageReadV1 {
    pub schema: String,
    pub header: EventHeader,
    pub event: P2ImMessageMessageReadV1Data,
}

/// 事件
#[derive(Debug, Serialize, Deserialize)]
pub struct P2ImMessageMessageReadV1Data {
    pub sender: EventSender,
    pub message: EventMessage,
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
