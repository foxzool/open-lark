use crate::{
    event::context::EventContext,
    service::im::v1::{
        p2_im_message_read_v1::{P2ImMessageReadV1, P2ImMessageReadV1ProcessorImpl},
        p2_im_message_receive_v1::{P2ImMessageReceiveV1, P2ImMessageReceiveV1ProcessorImpl},
    },
};
use log::debug;
use std::collections::HashMap;

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
        if let Some(handler) = self.processor_map.get(event) {
            handler.handle(payload)
        } else {
            log::warn!("No event processor found for event: {}", event);
            Err(anyhow::anyhow!("event processor {} not found", event))
        }
    }

    pub fn do_without_validation(&self, payload: Vec<u8>) -> anyhow::Result<()> {
        let mut context = serde_json::from_slice::<EventContext>(&payload)?;
        debug!("{:?}", context);
        if context.schema.is_some() {
            // 解析 v2 事件
            context.schema = Some("p2".to_string());
            context
                .type_
                .clone_from(&context.header.as_ref().unwrap().event_type);
            context
                .token
                .clone_from(&context.header.as_ref().unwrap().token)
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

impl EventDispatcherHandlerBuilder {
    pub fn register_p2_im_message_receive_v1<F>(mut self, f: F) -> Result<Self, String>
    where
        F: Fn(P2ImMessageReceiveV1) + 'static + Sync + Send,
    {
        let key = "p2.im.message.receive_v1".to_string();
        if self.processor_map.contains_key(&key) {
            return Err(format!("processor already registered, type: {}", key));
        }
        let processor = P2ImMessageReceiveV1ProcessorImpl::new(f);
        self.processor_map.insert(key, Box::new(processor));
        Ok(self)
    }

    pub fn register_p2_im_message_read_v1<F>(mut self, f: F) -> Result<Self, String>
    where
        F: Fn(P2ImMessageReadV1) + 'static + Sync + Send,
    {
        let key = "p2.im.message.message_read_v1".to_string();
        if self.processor_map.contains_key(&key) {
            return Err(format!("processor already registered, type: {}", key));
        }
        let processor = P2ImMessageReadV1ProcessorImpl::new(f);
        self.processor_map.insert(key, Box::new(processor));
        Ok(self)
    }
}
