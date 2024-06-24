use std::collections::HashMap;

pub use im_v1_event::*;

use crate::event::context::EventContext;

mod im_v1_event;

/// 事件分发处理器
pub struct EventDispatcherHandler {
    /// 事件map,key为事件类型，value为事件处理器
    processor_map: HashMap<String, Box<dyn EventHandler>>,
    // 事件回调签名token，消息解密key
    verification_token: Option<String>,
    event_encrypt_key: Option<String>,
}

impl EventDispatcherHandler {
    pub fn builder() -> EventDispatcherHandlerBuilder {
        EventDispatcherHandlerBuilder {
            processor_map: HashMap::new(),
            verification_token: None,
            event_encrypt_key: None,
        }
    }

    pub fn set_verification_token(&mut self, verification_token: String) {
        self.verification_token = Some(verification_token);
    }

    pub fn set_event_encrypt_key(&mut self, event_encrypt_key: String) {
        self.event_encrypt_key = Some(event_encrypt_key);
    }

    fn emit(&self, event: &str, payload: &[u8]) -> anyhow::Result<()> {
        let handler = self
            .processor_map
            .get(event)
            .unwrap_or_else(|| panic!("event processor {event} not found"));
        handler.handle(payload)
    }

    pub fn do_without_validation(&self, payload: Vec<u8>) -> anyhow::Result<()> {
        let mut context = serde_json::from_slice::<EventContext>(&payload).unwrap();
        if context.schema.is_some() {
            // 解析 v2 事件
            context.schema = Some("p2".to_string());
            context.type_ = context.header.as_ref().unwrap().event_type.clone();
            context.token = context.header.as_ref().unwrap().token.clone();
        } else if context.uuid.is_some() {
            // 解析 v1 事件
            context.schema = Some("p1".to_string());
            context.type_ = context.event.get("type").map(|v| v.to_string());
        }

        let handler_name = format!("{}.{}", context.schema.unwrap(), context.type_.unwrap());
        self.emit(&handler_name, &payload)
    }
}

pub trait EventHandler {
    fn handle(&self, payload: &[u8]) -> anyhow::Result<()>;
}

pub struct EventDispatcherHandlerBuilder {
    /// 事件map,key为事件类型，value为事件处理器
    processor_map: HashMap<String, Box<dyn EventHandler>>,
    // 事件回调签名token，消息解密key
    verification_token: Option<String>,
    event_encrypt_key: Option<String>,
}

impl EventDispatcherHandlerBuilder {
    pub fn build(self) -> EventDispatcherHandler {
        EventDispatcherHandler {
            processor_map: self.processor_map,
            verification_token: self.verification_token,
            event_encrypt_key: self.event_encrypt_key,
        }
    }
}
