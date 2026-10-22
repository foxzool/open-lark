// WebSocket 客户端模块
//
// 公开入口仅暴露会话级类型（LarkWsClient / EventHandler 等）。
// 会话协议在单一 `session` loop 中实现；frame / package 为内部细节。

mod client;
mod dispatcher;
mod frame_handler;
mod headers;
mod package;
mod session;
pub mod typed;

/// 会话级公开 API。
pub use client::LarkWsClient;
pub use dispatcher::{CallbackEventHandler, EventDispatcherHandler, EventHandler};
pub use session::{InvalidStateKind, WsClientError, WsClientResult, WsCloseReason};
pub use typed::{
    CardAction, CardActionCard, CardActionTrigger, CardActionTriggerEvent,
    CardActionTriggerHandler, CardActionTriggerResponse, CardContext, CardOperator, CardToast,
    EventHeader, ImMention, ImMessageReceiveV1, ImMessageReceiveV1Event, ImMessageReceiveV1Handler,
    ImMessageSender, ImReceivedMessage, ImSenderId,
};

// 按域发布的 typed 事件模块（envelope + handler + register_*）。
// 类型在域模块内发布（各域嵌套类型同名，如 `UserId`，按模块隔离）。
pub use typed::{
    approval, calendar, contact, corehr_v1, corehr_v2, drive, hire, im, mail, task, vc,
};

#[cfg(test)]
mod tests;

#[cfg(test)]
mod full_session_tests;

#[cfg(test)]
mod session_behavior_tests;
