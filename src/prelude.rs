// 预定义模块，导出最常用的类型和 trait，方便用户通过 `use open_lark::prelude::*;` 一次性导入

pub use crate::client::{LarkClient, LarkClientBuilder};
pub use crate::core::constants::AppType;
pub use crate::core::error::LarkAPIError;
pub use crate::core::req_option::RequestOption;
pub use crate::core::SDKResult;
pub use crate::custom_bot::CustomBot;
pub use crate::event::dispatcher::EventDispatcherHandler;

// 导出常用的消息相关类型
pub use crate::service::im::v1::message::{
    CreateMessageRequest, CreateMessageRequestBody, SendMessageTrait,
};

// 导出常用的卡片类型
pub use crate::card::{FeishuCard, FeishuCardConfig, FeishuCardLanguage};

// 导出常用的事件类型
pub use crate::service::im::v1::{
    p2_im_message_read_v1::P2ImMessageReadV1, p2_im_message_receive_v1::P2ImMessageReceiveV1,
};