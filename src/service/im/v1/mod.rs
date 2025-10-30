//! IM API v1版本

/// 简单的占位符模块，用于解决导入问题
#[derive(Debug, Clone)]
pub struct MessageService;

pub mod message {

    #[derive(Debug)]
    pub struct CreateMessageRequest;

    #[derive(Debug)]
    pub struct CreateMessageRequestBody;

    #[derive(Debug)]
    pub struct MessageCardTemplate;

    pub trait SendMessageTrait {
        fn send(&self) -> bool;
        fn msg_type(&self) -> String;
        fn content(&self) -> String;
    }

    impl SendMessageTrait for MessageCardTemplate {
        fn send(&self) -> bool {
            // 默认实现，实际使用时应该通过客户端发送
            true
        }

        fn msg_type(&self) -> String {
            "interactive".to_string()
        }

        fn content(&self) -> String {
            // 默认返回空JSON对象，实际使用时应该序列化卡片内容
            "{}".to_string()
        }
    }
}

pub mod p2_im_message_read_v1 {
    use serde::{Deserialize, Serialize};
    use crate::core::api_resp::{ApiResponseTrait, ResponseFormat};

    #[derive(Debug, Deserialize, Serialize)]
    pub struct P2ImMessageReadV1;

    impl ApiResponseTrait for P2ImMessageReadV1 {
        fn data_format() -> ResponseFormat {
            ResponseFormat::Data
        }
    }
}