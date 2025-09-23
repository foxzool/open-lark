//! 即时消息服务端点常量定义

// ==================== 即时消息服务端点 ====================

/// 发送消息
pub const IM_V1_SEND_MESSAGE: &str = "/open-apis/im/v1/messages";

/// 获取消息详情
pub const IM_V1_GET_MESSAGE: &str = "/open-apis/im/v1/messages/{message_id}";

/// 更新消息内容
pub const IM_V1_UPDATE_MESSAGE: &str = "/open-apis/im/v1/messages/{message_id}";

/// 删除消息
pub const IM_V1_DELETE_MESSAGE: &str = "/open-apis/im/v1/messages/{message_id}";

/// 获取消息已读用户
pub const IM_V1_READ_MESSAGE: &str = "/open-apis/im/v1/messages/{message_id}/read_users";

/// 获取消息列表
pub const IM_V1_LIST_MESSAGE: &str = "/open-apis/im/v1/messages";

/// 回复消息
pub const IM_V1_REPLY_MESSAGE: &str = "/open-apis/im/v1/messages/{message_id}/reply";

/// 消息表情回复
pub const IM_V1_MESSAGE_REACTIONS: &str = "/open-apis/im/v1/messages/{message_id}/reactions";

/// 删除消息表情回复
pub const IM_V1_DELETE_MESSAGE_REACTION: &str =
    "/open-apis/im/v1/messages/{message_id}/reactions/{reaction_id}";

/// 批量发送消息
pub const IM_V1_BATCH_MESSAGES: &str = "/open-apis/im/v1/batch_messages";

/// 删除批量消息
pub const IM_V1_DELETE_BATCH_MESSAGE: &str = "/open-apis/im/v1/batch_messages/{batch_message_id}";

/// 获取批量消息发送进度
pub const IM_V1_BATCH_MESSAGE_PROGRESS: &str =
    "/open-apis/im/v1/batch_messages/{batch_message_id}/get_progress";

/// 获取批量消息已读用户
pub const IM_V1_BATCH_MESSAGE_READ_USER: &str =
    "/open-apis/im/v1/batch_messages/{batch_message_id}/read_user";

/// 消息加急 - 应用内加急
pub const IM_V1_MESSAGE_URGENT_APP: &str = "/open-apis/im/v1/messages/{message_id}/urgent_app";

/// 消息加急 - 短信加急
pub const IM_V1_MESSAGE_URGENT_SMS: &str = "/open-apis/im/v1/messages/{message_id}/urgent_sms";

/// 消息加急 - 电话加急
pub const IM_V1_MESSAGE_URGENT_PHONE: &str = "/open-apis/im/v1/messages/{message_id}/urgent_phone";

/// 延迟更新消息
pub const IM_V1_MESSAGE_DELAY_UPDATE: &str = "/open-apis/im/v1/messages/{message_id}/delay_update";

/// 批量更新消息URL预览
pub const IM_V1_MESSAGE_URL_PREVIEW_BATCH_UPDATE: &str =
    "/open-apis/im/v1/messages/{message_id}/url_preview/batch_update";

/// 消息详情获取 (兼容性别名)
pub const IM_V1_MESSAGE_GET: &str = "/open-apis/im/v1/messages/{message_id}";

/// 消息更新 (兼容性别名)
pub const IM_V1_MESSAGE_PATCH: &str = "/open-apis/im/v1/messages/{message_id}";

/// 消息删除 (兼容性别名)
pub const IM_V1_MESSAGE_DELETE: &str = "/open-apis/im/v1/messages/{message_id}";

// ==================== 会话管理端点 ====================

/// 创建群聊
pub const IM_CHAT_CREATE: &str = "/open-apis/im/v1/chats";

/// 获取会话信息
pub const IM_CHAT_GET: &str = "/open-apis/im/v1/chats/{chat_id}";

/// 更新会话信息
pub const IM_CHAT_UPDATE: &str = "/open-apis/im/v1/chats/{chat_id}";

/// 解散群聊
pub const IM_CHAT_DELETE: &str = "/open-apis/im/v1/chats/{chat_id}";

/// 获取群成员列表
pub const IM_CHAT_MEMBERS: &str = "/open-apis/im/v1/chats/{chat_id}/members";

/// 添加群成员
pub const IM_CHAT_ADD_MEMBERS: &str = "/open-apis/im/v1/chats/{chat_id}/members";

/// 批量移除群成员
pub const IM_CHAT_REMOVE_MEMBERS: &str = "/open-apis/im/v1/chats/{chat_id}/members/batch_delete";

// ==================== Pin 消息管理端点 ====================

/// Pin 消息
pub const IM_V1_PINS: &str = "/open-apis/im/v1/pins";

/// 取消 Pin 消息
pub const IM_V1_DELETE_PIN: &str = "/open-apis/im/v1/pins/{pin_id}";

// ==================== 文件和图片管理端点 ====================

/// 上传文件
pub const IM_V1_FILES: &str = "/open-apis/im/v1/files";

/// 下载文件
pub const IM_V1_DOWNLOAD_FILE: &str = "/open-apis/im/v1/files/{file_key}";

/// 上传图片
pub const IM_V1_IMAGES: &str = "/open-apis/im/v1/images";

/// 下载图片
pub const IM_V1_DOWNLOAD_IMAGE: &str = "/open-apis/im/v1/images/{image_key}";

// ==================== IM V2 端点 ====================

/// 创建应用推送卡片
pub const IM_V2_APP_FEED_CARD: &str = "/open-apis/im/v2/app_feed_card";

/// 获取应用推送卡片
pub const IM_V2_GET_APP_FEED_CARD: &str = "/open-apis/im/v2/app_feed_card/{card_id}";

/// 删除应用推送卡片
pub const IM_V2_DELETE_APP_FEED_CARD: &str = "/open-apis/im/v2/app_feed_card/{card_id}";

/// 机器人时间敏感消息
pub const IM_V2_GROUPS_BOTS_TIME_SENSITIVE: &str = "/open-apis/im/v2/groups-bots/bot_time_sentive";

/// 更新机器人消息
pub const IM_V2_GROUPS_BOTS_UPDATE: &str = "/open-apis/im/v2/groups-bots/{message_id}/update";

/// 机器人消息补丁
pub const IM_V2_GROUPS_BOTS_PATCH: &str = "/open-apis/im/v2/groups-bots/patch";

// ==================== 兼容性别名 ====================

/// 发送消息 (兼容性别名)
pub const IM_V1_MESSAGES: &str = IM_V1_SEND_MESSAGE;

/// 回复消息 (兼容性别名)
pub const IM_V1_MESSAGE_REPLY: &str = IM_V1_REPLY_MESSAGE;

/// 群聊管理 (兼容性别名)
pub const IM_V1_CHATS: &str = IM_CHAT_CREATE;

/// 获取会话信息 (兼容性别名)
pub const IM_V1_CHATS_GET: &str = IM_CHAT_GET;

/// 更新会话信息 (兼容性别名)
pub const IM_V1_CHATS_UPDATE: &str = IM_CHAT_UPDATE;

/// 群成员管理 (兼容性别名)
pub const IM_V1_CHAT_MEMBERS: &str = IM_CHAT_MEMBERS;
