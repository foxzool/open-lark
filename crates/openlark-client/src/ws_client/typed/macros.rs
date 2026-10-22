//! typed 事件通用 adapter 与样板宏。
//!
//! 各域事件文件通过 [`typed_events!`] 宏生成 handler trait、闭包 blanket impl
//! 与 `register_*` 方法；反序列化统一走本模块的 [`TypedEventAdapter`]，
//! 行为对齐手写的 `ImMessageReceiveV1Adapter`。

use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::error::serialization_error;

use super::super::EventHandler;

/// typed 事件通用 adapter：按事件串反序列化后回调 handler。
pub(crate) struct TypedEventAdapter<H, E> {
    handler: H,
    event_type: &'static str,
    _marker: std::marker::PhantomData<fn(E)>,
}

impl<H, E> TypedEventAdapter<H, E> {
    /// 创建 adapter。
    pub(crate) fn new(handler: H, event_type: &'static str) -> Self {
        Self {
            handler,
            event_type,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<H, E> EventHandler for TypedEventAdapter<H, E>
where
    H: Fn(E) -> Result<(), Box<dyn std::error::Error + Send + Sync>> + Send + Sync + 'static,
    E: DeserializeOwned + Send + Sync + 'static,
{
    fn handle(&self, payload: &[u8]) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let event: E = serde_json::from_slice(payload)
            .map_err(|e| serialization_error(format!("反序列化 {} 失败: {e}", self.event_type)))?;
        (self.handler)(event)
    }

    fn handle_from_value(
        &self,
        value: &Value,
        _payload: &[u8],
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let event: E = E::deserialize(value)
            .map_err(|e| serialization_error(format!("反序列化 {} 失败: {e}", self.event_type)))?;
        (self.handler)(event)
    }
}

/// 为 typed 事件批量生成 handler trait、闭包 blanket impl 与注册方法。
///
/// 每个事件一行：`"事件串" | CONST 名 | envelope 名 | handler trait 名 | register 方法名;`，
/// 可选前置 `#[doc]` 成为 handler trait 的 doc。生成的注册方法写入 raw map，
/// 与 [`super::super::EventDispatcherHandler::register_raw`] 同 key 冲突返回 Err。
macro_rules! typed_events {
    ($(
        $(#[$handler_doc:meta])*
        $event_type:literal | $const_name:ident | $envelope:ident | $handler_trait:ident | $register:ident;
    )+) => {
        $(
            $(#[$handler_doc])*
            pub trait $handler_trait: Send + Sync + 'static {
                /// 处理反序列化后的事件。
                fn handle(
                    &self,
                    event: $envelope,
                ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
            }

            impl<F> $handler_trait for F
            where
                F: Fn($envelope) -> Result<(), Box<dyn std::error::Error + Send + Sync>>
                    + Send
                    + Sync
                    + 'static,
            {
                fn handle(
                    &self,
                    event: $envelope,
                ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
                    self(event)
                }
            }
        )+

        impl $crate::ws_client::EventDispatcherHandler {
            $(
                #[doc = concat!("事件类型 `", $event_type, "`.")]
                pub const $const_name: &'static str = $event_type;

                #[doc = concat!("注册 `", $event_type, "` typed 处理器。")]
                ///
                /// 写入 raw map。与 [`Self::register_raw`] 使用同一 `event_type` key，重复注册会失败。
                pub fn $register<H>(self, handler: H) -> Result<Self, String>
                where
                    H: $handler_trait,
                {
                    self.register_raw(
                        Self::$const_name,
                        super::macros::TypedEventAdapter::new(
                            move |event: $envelope| handler.handle(event),
                            Self::$const_name,
                        ),
                    )
                }
            )+
        }
    };
}
