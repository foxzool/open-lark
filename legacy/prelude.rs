//! 预定义模块，导出最常用的类型和trait
//!
//! 通过 `use open_lark::prelude::*;` 可以一次性导入所有常用的类型和trait，
//! 包括客户端、配置、错误类型、消息类型和事件类型等。
//!
//! # 示例
//!
//! ```rust
//! use open_lark::prelude::*;
//!
//! // 创建客户端
//! let client = LarkClient::builder("app_id", "app_secret")
//!     .with_app_type(AppType::SelfBuild)
//!     .build();
//!
//! // 创建消息
//! let message = CreateMessageRequestBody::builder()
//!     .receive_id("user_id")
//!     .msg_type("text")
//!     .content("{\"text\":\"Hello, World!\"}")
//!     .build();
//! ```

pub use crate::{
    client::{LarkClient, LarkClientBuilder},
    core::{
        constants::AppType, error::LarkAPIError, req_option::RequestOption,
        token_manager::PreheatingConfig, SDKResult,
    },
    custom_bot::CustomBot,
    event::dispatcher::EventDispatcherHandler,
};

// 导出常用的消息相关类型 - 需要 im feature
#[cfg(feature = "im")]
pub use crate::service::im::v1::message::{
    CreateMessageRequest, CreateMessageRequestBody, SendMessageTrait,
};

// 导出常用的卡片类型
pub use crate::card::{FeishuCard, FeishuCardConfig, FeishuCardLanguage};

// 导出常用的事件类型 - 需要 im feature
#[cfg(feature = "im")]
pub use crate::service::im::v1::{
    p2_im_message_read_v1::P2ImMessageReadV1, p2_im_message_receive_v1::P2ImMessageReceiveV1,
};
