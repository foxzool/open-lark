//! IM服务端点定义
//!
//! 此模块提供IM相关的端点常量定义

// IM相关的端点常量
pub const IM_V1_MESSAGES: &str = "/open-apis/im/v1/messages";
pub const IM_V1_CHATS: &str = "/open-apis/im/v1/chats";
pub const IM_V1_BATCH_MESSAGES: &str = "/open-apis/im/v1/batch_messages";
pub const IM_V2_MESSAGES: &str = "/open-apis/im/v2/messages";

// 为了向后兼容性，提供端点常量别名
pub const SEND_MESSAGE: &str = IM_V1_MESSAGES;
pub const CHAT_CREATE: &str = IM_V1_CHATS;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_im_endpoints_compatibility() {
        // 验证IM端点的兼容性
        assert_eq!(IM_V1_MESSAGES, SEND_MESSAGE);
        assert_eq!(IM_V1_CHATS, CHAT_CREATE);
        assert_eq!(IM_V1_MESSAGES, "/open-apis/im/v1/messages");
        assert_eq!(IM_V1_CHATS, "/open-apis/im/v1/chats");
    }
}
