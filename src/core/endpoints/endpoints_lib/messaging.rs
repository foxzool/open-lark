//! 即时消息服务端点
//!
//! 包含即时消息、聊天、文件上传等相关的API端点。

/// 即时消息相关端点
pub struct Messaging;

impl Messaging {
    // ==================== 消息管理 ====================

    /// 发送消息
    pub const SEND_MESSAGE: &'static str = "/open-apis/im/v1/messages";

    /// 获取消息详情
    pub const GET_MESSAGE: &'static str = "/open-apis/im/v1/messages/{message_id}";

    /// 更新消息内容
    pub const UPDATE_MESSAGE: &'static str = "/open-apis/im/v1/messages/{message_id}";

    /// 删除消息
    pub const DELETE_MESSAGE: &'static str = "/open-apis/im/v1/messages/{message_id}";

    /// 获取消息已读用户
    pub const READ_MESSAGE: &'static str = "/open-apis/im/v1/messages/{message_id}/read_users";

    /// 获取消息列表
    pub const LIST_MESSAGE: &'static str = "/open-apis/im/v1/messages";

    /// 回复消息
    pub const REPLY_MESSAGE: &'static str = "/open-apis/im/v1/messages/{message_id}/reply";

    /// 转发消息
    pub const FORWARD_MESSAGE: &'static str = "/open-apis/im/v1/messages/{message_id}/forward";

    // ==================== 消息表情 ====================

    /// 消息表情回复
    pub const MESSAGE_REACTIONS: &'static str = "/open-apis/im/v1/messages/{message_id}/reactions";

    /// 删除消息表情回复
    pub const DELETE_MESSAGE_REACTION: &'static str =
        "/open-apis/im/v1/messages/{message_id}/reactions/{reaction_id}";

    // ==================== 批量消息 ====================

    /// 批量发送消息
    pub const BATCH_MESSAGES: &'static str = "/open-apis/im/v1/batch_messages";

    /// 删除批量消息
    pub const DELETE_BATCH_MESSAGE: &'static str =
        "/open-apis/im/v1/batch_messages/{batch_message_id}";

    /// 获取批量消息发送进度
    pub const BATCH_MESSAGE_PROGRESS: &'static str =
        "/open-apis/im/v1/batch_messages/{batch_message_id}/get_progress";

    /// 获取批量消息已读用户
    pub const BATCH_MESSAGE_READ_USER: &'static str =
        "/open-apis/im/v1/batch_messages/{batch_message_id}/read_user";

    // ==================== 消息加急 ====================

    /// 消息加急 - 应用内加急
    pub const MESSAGE_URGENT_APP: &'static str =
        "/open-apis/im/v1/messages/{message_id}/urgent_app";

    /// 消息加急 - 短信加急
    pub const MESSAGE_URGENT_SMS: &'static str =
        "/open-apis/im/v1/messages/{message_id}/urgent_sms";

    /// 消息加急 - 电话加急
    pub const MESSAGE_URGENT_PHONE: &'static str =
        "/open-apis/im/v1/messages/{message_id}/urgent_phone";

    // ==================== 消息更新 ====================

    /// 延迟更新消息
    pub const MESSAGE_DELAY_UPDATE: &'static str =
        "/open-apis/im/v1/messages/{message_id}/delay_update";

    /// 批量更新消息URL预览
    pub const MESSAGE_URL_PREVIEW_BATCH_UPDATE: &'static str =
        "/open-apis/im/v1/messages/{message_id}/url_preview/batch_update";

    // ==================== 会话管理 ====================

    /// 创建群聊
    pub const CHAT_CREATE: &'static str = "/open-apis/im/v1/chats";

    /// 获取会话信息
    pub const CHAT_GET: &'static str = "/open-apis/im/v1/chats/{chat_id}";

    /// 更新会话信息
    pub const CHAT_UPDATE: &'static str = "/open-apis/im/v1/chats/{chat_id}";

    /// 解散群聊
    pub const CHAT_DELETE: &'static str = "/open-apis/im/v1/chats/{chat_id}";

    /// 获取群成员列表
    pub const CHAT_MEMBERS: &'static str = "/open-apis/im/v1/chats/{chat_id}/members";

    /// 添加群成员
    pub const CHAT_ADD_MEMBERS: &'static str = "/open-apis/im/v1/chats/{chat_id}/members";

    /// 批量移除群成员
    pub const CHAT_REMOVE_MEMBERS: &'static str =
        "/open-apis/im/v1/chats/{chat_id}/members/batch_delete";

    // ==================== Pin消息管理 ====================

    /// Pin消息
    pub const PINS: &'static str = "/open-apis/im/v1/pins";

    /// 取消Pin消息
    pub const DELETE_PIN: &'static str = "/open-apis/im/v1/pins/{pin_id}";

    // ==================== 文件和图片管理 ====================

    /// 上传文件
    pub const FILES: &'static str = "/open-apis/im/v1/files";

    /// 下载文件
    pub const DOWNLOAD_FILE: &'static str = "/open-apis/im/v1/files/{file_key}";

    /// 上传图片
    pub const IMAGES: &'static str = "/open-apis/im/v1/images";

    /// 下载图片
    pub const DOWNLOAD_IMAGE: &'static str = "/open-apis/im/v1/images/{image_key}";

    // ==================== V2端点 ====================

    /// 创建应用推送卡片
    pub const V2_APP_FEED_CARD: &'static str = "/open-apis/im/v2/app_feed_card";

    /// 获取应用推送卡片
    pub const V2_GET_APP_FEED_CARD: &'static str =
        "/open-apis/im/v2/app_feed_card/{card_id}";

    /// 删除应用推送卡片
    pub const V2_DELETE_APP_FEED_CARD: &'static str =
        "/open-apis/im/v2/app_feed_card/{card_id}";

    /// 机器人时间敏感消息
    pub const V2_GROUPS_BOTS_TIME_SENSITIVE: &'static str =
        "/open-apis/im/v2/groups-bots/bot_time_sentive";

    /// 更新机器人消息
    pub const V2_GROUPS_BOTS_UPDATE: &'static str =
        "/open-apis/im/v2/groups-bots/{message_id}/update";

    /// 机器人消息补丁
    pub const V2_GROUPS_BOTS_PATCH: &'static str =
        "/open-apis/im/v2/groups-bots/patch";
}

// 向后兼容性别名 - 这些将从原有的Endpoints结构体重新导出
#[allow(dead_code)]
pub mod legacy {
    pub const IM_V1_SEND_MESSAGE: &str = Messaging::SEND_MESSAGE;
    pub const IM_V1_GET_MESSAGE: &str = Messaging::GET_MESSAGE;
    pub const IM_V1_UPDATE_MESSAGE: &str = Messaging::UPDATE_MESSAGE;
    pub const IM_V1_DELETE_MESSAGE: &str = Messaging::DELETE_MESSAGE;
    pub const IM_V1_READ_MESSAGE: &str = Messaging::READ_MESSAGE;
    pub const IM_V1_LIST_MESSAGE: &str = Messaging::LIST_MESSAGE;
    pub const IM_V1_REPLY_MESSAGE: &str = Messaging::REPLY_MESSAGE;
    pub const IM_V1_MESSAGE_REACTIONS: &str = Messaging::MESSAGE_REACTIONS;
    pub const IM_V1_DELETE_MESSAGE_REACTION: &str = Messaging::DELETE_MESSAGE_REACTION;
    pub const IM_V1_BATCH_MESSAGES: &str = Messaging::BATCH_MESSAGES;
    pub const IM_V1_DELETE_BATCH_MESSAGE: &str = Messaging::DELETE_BATCH_MESSAGE;
    pub const IM_V1_BATCH_MESSAGE_PROGRESS: &str = Messaging::BATCH_MESSAGE_PROGRESS;
    pub const IM_V1_BATCH_MESSAGE_READ_USER: &str = Messaging::BATCH_MESSAGE_READ_USER;
    pub const IM_V1_MESSAGE_URGENT_APP: &str = Messaging::MESSAGE_URGENT_APP;
    pub const IM_V1_MESSAGE_URGENT_SMS: &str = Messaging::MESSAGE_URGENT_SMS;
    pub const IM_V1_MESSAGE_URGENT_PHONE: &str = Messaging::MESSAGE_URGENT_PHONE;
    pub const IM_V1_MESSAGE_DELAY_UPDATE: &str = Messaging::MESSAGE_DELAY_UPDATE;
    pub const IM_V1_MESSAGE_URL_PREVIEW_BATCH_UPDATE: &str = Messaging::MESSAGE_URL_PREVIEW_BATCH_UPDATE;
    pub const IM_CHAT_CREATE: &str = Messaging::CHAT_CREATE;
    pub const IM_CHAT_GET: &str = Messaging::CHAT_GET;
    pub const IM_CHAT_UPDATE: &str = Messaging::CHAT_UPDATE;
    pub const IM_CHAT_DELETE: &str = Messaging::CHAT_DELETE;
    pub const IM_CHAT_MEMBERS: &str = Messaging::CHAT_MEMBERS;
    pub const IM_CHAT_ADD_MEMBERS: &str = Messaging::CHAT_ADD_MEMBERS;
    pub const IM_CHAT_REMOVE_MEMBERS: &str = Messaging::CHAT_REMOVE_MEMBERS;
    pub const IM_V1_PINS: &str = Messaging::PINS;
    pub const IM_V1_DELETE_PIN: &str = Messaging::DELETE_PIN;
    pub const IM_V1_FILES: &str = Messaging::FILES;
    pub const IM_V1_DOWNLOAD_FILE: &str = Messaging::DOWNLOAD_FILE;
    pub const IM_V1_IMAGES: &str = Messaging::IMAGES;
    pub const IM_V1_DOWNLOAD_IMAGE: &str = Messaging::DOWNLOAD_IMAGE;
    pub const IM_V2_APP_FEED_CARD: &str = Messaging::V2_APP_FEED_CARD;
    pub const IM_V2_GET_APP_FEED_CARD: &str = Messaging::V2_GET_APP_FEED_CARD;
    pub const IM_V2_DELETE_APP_FEED_CARD: &str = Messaging::V2_DELETE_APP_FEED_CARD;
}