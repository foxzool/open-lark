//! 即时消息服务端点

/// 即时消息服务端点
pub struct Messaging;

impl Messaging {
    /// / 获取消息详情
    pub const IM_V1_SEND_MESSAGE: &'static str = "/open-apis/im/v1/messages";
    /// / 更新消息
    pub const IM_V1_GET_MESSAGE: &'static str = "/open-apis/im/v1/messages/{message_id}";
    /// / 删除消息
    pub const IM_V1_UPDATE_MESSAGE: &'static str = "/open-apis/im/v1/messages/{message_id}";
    /// / 消息已读回执
    pub const IM_V1_DELETE_MESSAGE: &'static str = "/open-apis/im/v1/messages/{message_id}";
    /// / 获取消息列表
    pub const IM_V1_READ_MESSAGE: &'static str = "/open-apis/im/v1/messages/{message_id}/read_users";
    /// / 转发消息
    pub const IM_V1_LIST_MESSAGE: &'static str = "/open-apis/im/v1/messages";
    /// / 上传图片
    pub const IM_V1_FORWARD_MESSAGE: &'static str = "/open-apis/im/v1/messages/{message_id}/forward";
    /// 聊天管理
    pub const IM_V1_IMAGES_UPLOAD: &'static str = "/open-apis/im/v1/images";
    /// / 获取聊天信息
    pub const IM_CHAT_CREATE: &'static str = "/open-apis/im/v1/chats";
    /// / 更新聊天信息
    pub const IM_CHAT_GET: &'static str = "/open-apis/im/v1/chats/{chat_id}";
    /// / 解散聊天
    pub const IM_CHAT_UPDATE: &'static str = "/open-apis/im/v1/chats/{chat_id}";
    /// / 获取聊天成员列表
    pub const IM_CHAT_DELETE: &'static str = "/open-apis/im/v1/chats/{chat_id}";
    /// / 将用户或机器人拉入聊天
    pub const IM_CHAT_MEMBERS: &'static str = "/open-apis/im/v1/chats/{chat_id}/members";
    /// / 将用户或机器人移出聊天
    pub const IM_CHAT_ADD_MEMBERS: &'static str = "/open-apis/im/v1/chats/{chat_id}/members";
    /// 回复消息
    pub const IM_CHAT_REMOVE_MEMBERS: &'static str = "/open-apis/im/v1/chats/{chat_id}/members/batch_delete";
    /// 消息表情回应
    pub const IM_V1_REPLY_MESSAGE: &'static str = "/open-apis/im/v1/messages/{message_id}/reply";
    /// / 删除消息表情回应
    pub const IM_V1_MESSAGE_REACTIONS: &'static str = "/open-apis/im/v1/messages/{message_id}/reactions";
    /// 批量消息
    pub const IM_V1_DELETE_MESSAGE_REACTION: &'static str = "/open-apis/im/v1/messages/{message_id}/reactions/{reaction_id}";
    /// / 批量撤回消息
    pub const IM_V1_BATCH_MESSAGES: &'static str = "/open-apis/im/v1/batch_messages";
    /// / 查询批量发送消息进度
    pub const IM_V1_DELETE_BATCH_MESSAGE: &'static str = "/open-apis/im/v1/batch_messages/{batch_message_id}";
    /// / 查询批量发送消息已读状态
    pub const IM_V1_BATCH_MESSAGE_PROGRESS: &'static str = "/open-apis/im/v1/batch_messages/{batch_message_id}/get_progress";
    /// 紧急消息/消息加急
    pub const IM_V1_BATCH_MESSAGE_READ_USER: &'static str = "/open-apis/im/v1/batch_messages/{batch_message_id}/read_user";
    /// / 短信加急
    pub const IM_V1_MESSAGE_URGENT_APP: &'static str = "/open-apis/im/v1/messages/{message_id}/urgent_app";
    /// / 电话加急
    pub const IM_V1_MESSAGE_URGENT_SMS: &'static str = "/open-apis/im/v1/messages/{message_id}/urgent_sms";
    /// 延时更新卡片
    pub const IM_V1_MESSAGE_URGENT_PHONE: &'static str = "/open-apis/im/v1/messages/{message_id}/urgent_phone";
    /// Pin 消息
    pub const IM_V1_MESSAGE_DELAY_UPDATE: &'static str = "/open-apis/im/v1/messages/{message_id}/delay_update";
    /// / 删除Pin消息
    pub const IM_V1_PINS: &'static str = "/open-apis/im/v1/pins";
    /// 文件和图片
    pub const IM_V1_DELETE_PIN: &'static str = "/open-apis/im/v1/pins/{pin_id}";
    /// / 下载文件
    pub const IM_V1_FILES: &'static str = "/open-apis/im/v1/files";
    /// / 上传图片
    pub const IM_V1_DOWNLOAD_FILE: &'static str = "/open-apis/im/v1/files/{file_key}";
    /// / 下载图片
    pub const IM_V1_IMAGES: &'static str = "/open-apis/im/v1/images";
    /// URL预览
    pub const IM_V1_DOWNLOAD_IMAGE: &'static str = "/open-apis/im/v1/images/{image_key}";
    /// 基础消息操作
    pub const IM_V1_MESSAGE_URL_PREVIEW_BATCH_UPDATE: &'static str = "/open-apis/im/v1/messages/{message_id}/url_preview/batch_update";
    /// / 更新消息内容 (需要使用 EndpointBuilder::replace_param 替换 {message_id})
    pub const IM_V1_MESSAGE_GET: &'static str = "/open-apis/im/v1/messages/{message_id}";
    /// / 删除消息 (需要使用 EndpointBuilder::replace_param 替换 {message_id})
    pub const IM_V1_MESSAGE_PATCH: &'static str = "/open-apis/im/v1/messages/{message_id}";
    /// V2 API 端点
    pub const IM_V1_MESSAGE_DELETE: &'static str = "/open-apis/im/v1/messages/{message_id}";
    /// / 获取应用信息流卡片
    pub const IM_V2_APP_FEED_CARD: &'static str = "/open-apis/im/v2/app_feed_card";
    /// / 删除应用信息流卡片
    pub const IM_V2_GET_APP_FEED_CARD: &'static str = "/open-apis/im/v2/app_feed_card/{card_id}";
    /// Groups bots
    pub const IM_V2_DELETE_APP_FEED_CARD: &'static str = "/open-apis/im/v2/app_feed_card/{card_id}";
    /// / 更新群机器人消息
    pub const IM_V2_GROUPS_BOTS_TIME_SENSITIVE: &'static str = "/open-apis/im/v2/groups-bots/bot_time_sentive";
    /// / 批量更新群机器人设置
    pub const IM_V2_GROUPS_BOTS_UPDATE: &'static str = "/open-apis/im/v2/groups-bots/{message_id}/update";
    /// ==================== 云盘服务端点 ====================
    pub const IM_V2_GROUPS_BOTS_PATCH: &'static str = "/open-apis/im/v2/groups-bots/patch";
    /// / 批量发送消息 v4
    pub const APPROVAL_V4_MESSAGES: &'static str = "/open-apis/approval/v4/messages";
    /// / 更新消息 (需要使用 EndpointBuilder::replace_param 替换 {message_id})
    pub const MESSAGE_V4_BATCH_SEND: &'static str = "/open-apis/message/v4/batch_send/";
    /// ===== 搜索端点 =====
    pub const APPROVAL_V4_MESSAGE_PATCH: &'static str = "/open-apis/approval/v4/messages/{message_id}";
    /// ===== 函数管理端点 =====
    pub const APASS_V1_FLOW_USER_TASK_CHAT_GROUP: &'static str = "/open-apis/apaas/v1/application/{app_id}/flow/user_task/{task_id}/chat_group";
    /// / 工单消息创建
    pub const HELPDESK_V1_TICKET_MESSAGES: &'static str = "/open-apis/helpdesk/v1/tickets/{ticket_id}/messages";
    /// / 工单机器人消息
    pub const HELPDESK_V1_TICKET_MESSAGE_CREATE: &'static str = "/open-apis/helpdesk/v1/tickets/{ticket_id}/messages";
    /// FAQ管理
    pub const HELPDESK_V1_TICKET_BOT_MESSAGES: &'static str = "/open-apis/helpdesk/v1/tickets/{ticket_id}/bot_messages";
    /// / 应用部门使用概览
    pub const APPLICATION_V6_APP_USAGE_MESSAGE_PUSH_OVERVIEW: &'static str = "/open-apis/application/v6/app_usage/{app_id}/message_push_overview";
    /// Drive 云盘服务
    pub const DOCX_V1_CHAT_ANNOUNCEMENT_BLOCK: &'static str = "/open-apis/docx/v1/chats/{}/announcement/blocks/{}";
    /// / 用户邮箱消息详情 (需要使用 EndpointBuilder::replace_param 替换 {user_mailbox_id} 和 {message_id})
    pub const MAIL_V1_USER_MAILBOX_MESSAGES: &'static str = "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/messages";
    /// / 通过卡片获取邮件消息 (需要使用 EndpointBuilder::replace_param 替换 {user_mailbox_id})
    pub const MAIL_V1_USER_MAILBOX_MESSAGE: &'static str = "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/messages/{message_id}";
    /// 用户邮箱规则
    pub const MAIL_V1_USER_MAILBOX_MESSAGES_GET_BY_CARD: &'static str = "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/messages/get_by_card";
    /// ==================== 目录服务相关端点 ====================
    pub const MAIL_V1_USER_MAILBOX_MESSAGE_ATTACHMENT_DOWNLOAD_URL: &'static str = "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/messages/{message_id}/attachments/{attachment_id}/download_url";
    /// / 搜索应用
    pub const SEARCH_V2_MESSAGE: &'static str = "/open-apis/search/v2/message";
    /// / 消息获取
    pub const AILY_V1_MESSAGES: &'static str = "/open-apis/aily/v1/sessions/{session_id}/messages";
    /// / 技能启动
    pub const AILY_V1_MESSAGE_GET: &'static str = "/open-apis/aily/v1/sessions/{session_id}/messages/{message_id}";
}
