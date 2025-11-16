//! OpenLark Communication 服务端点定义
//!
//! 此模块包含通讯和协作相关的所有API端点常量，从 openlark-core 迁移而来。
//! 包含即时通讯、邮件服务、视频会议等完整协作功能。
//!
//! # 服务模块包含
//!
//! - **im**: 即时通讯（消息发送、聊天管理、批量消息）
//! - **mail**: 邮件服务（邮件组、邮箱管理、规则设置）
//! - **vc**: 视频会议（会议室、会议管理、参会控制）
//! - **event**: 事件系统（事件订阅、处理、分发）
//! - **moments**: 动态分享（朋友圈、内容分享、社交互动）
//!
//! # 使用示例
//!
//! ```rust
//! use openlark_communication::endpoints::*;
//!
//! // 即时通讯
//! let messages_endpoint = IM_V1_MESSAGES;
//! let chats_endpoint = IM_V1_CHATS;
//! let batch_messages_endpoint = IM_V1_BATCH_MESSAGES;
//!
//! // 视频会议
//! let rooms_endpoint = VC_V1_ROOMS;
//! let meetings_endpoint = VC_V1_MEETINGS;
//! let room_create_endpoint = VC_V1_ROOM_CREATE;
//!
//! // 邮件服务
//! let mailgroups_endpoint = MAIL_V1_MAILGROUPS;
//! let user_mailbox_endpoint = MAIL_V1_USER_MAILBOXES;
//! ```

// 导入核心端点（auth, application等基础端点）
pub use openlark_core::endpoints::{auth, application, apass, platform_integration};

// ==================== IM (即时通讯) v1/v2 ====================
// 即时通讯系统 - 消息发送、聊天管理、批量消息处理

/// IM消息管理 v1
/// 基础消息发送和管理
pub const IM_V1_MESSAGES: &str = "/open-apis/im/v1/messages";

/// IM聊天管理 v1
/// 聊天会话的创建和管理
pub const IM_V1_CHATS: &str = "/open-apis/im/v1/chats";

/// IM批量消息 v1
/// 批量消息发送和处理
pub const IM_V1_BATCH_MESSAGES: &str = "/open-apis/im/v1/batch_messages";

/// IM消息管理 v2
/// 增强版消息功能
pub const IM_V2_MESSAGES: &str = "/open-apis/im/v2/messages";

/// IM聊天管理 v2
/// 增强版聊天功能
pub const IM_V2_CHATS: &str = "/open-apis/im/v2/chats";

// ==================== Mail (邮件服务) v1 ====================
// 邮件系统 - 邮件组管理、邮箱事件、文件夹和消息

/// Mail邮件组管理
/// 企业邮件组的创建和管理
pub const MAIL_V1_MAILGROUPS: &str = "/open-apis/mail/v1/mailgroups";
pub const MAIL_V1_MAILGROUP: &str = "/open-apis/mail/v1/mailgroups/{mailgroup_id}";

/// Mail邮件组管理员
/// 邮件组管理员权限管理
pub const MAIL_V1_MAILGROUP_MANAGERS_BATCH_CREATE: &str = "/open-apis/mail/v1/mailgroups/{mailgroup_id}/managers/batch_create";
pub const MAIL_V1_MAILGROUP_MANAGERS_BATCH_DELETE: &str = "/open-apis/mail/v1/mailgroups/{mailgroup_id}/managers/batch_delete";
pub const MAIL_V1_MAILGROUP_MANAGERS: &str = "/open-apis/mail/v1/mailgroups/{mailgroup_id}/managers";

/// Mail用户邮箱
/// 用户邮箱的基础管理
pub const MAIL_V1_USER_MAILBOXES: &str = "/open-apis/mail/v1/user_mailboxes";

/// Mail用户邮箱事件
/// 邮箱事件的订阅和管理
pub const MAIL_V1_USER_MAILBOX_EVENTS_SUBSCRIBE: &str = "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/events/subscribe";
pub const MAIL_V1_USER_MAILBOX_EVENTS_SUBSCRIPTION: &str = "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/events/subscription";
pub const MAIL_V1_USER_MAILBOX_EVENTS_UNSUBSCRIBE: &str = "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/events/unsubscribe";

/// Mail用户邮箱文件夹
/// 邮件文件夹组织管理
pub const MAIL_V1_USER_MAILBOX_FOLDERS: &str = "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/folders";
pub const MAIL_V1_USER_MAILBOX_FOLDER: &str = "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/folders/{folder_id}";

/// Mail用户邮箱消息
/// 邮件消息的处理和管理
pub const MAIL_V1_USER_MAILBOX_MESSAGES: &str = "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/messages";
pub const MAIL_V1_USER_MAILBOX_MESSAGE: &str = "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/messages/{message_id}";
pub const MAIL_V1_USER_MAILBOX_MESSAGES_GET_BY_CARD: &str = "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/messages/get_by_card";

/// Mail用户邮箱规则
/// 邮件处理规则和过滤设置
pub const MAIL_V1_USER_MAILBOX_RULES: &str = "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/rules";
pub const MAIL_V1_USER_MAILBOX_RULE: &str = "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/rules/{rule_id}";
pub const MAIL_V1_USER_MAILBOX_RULES_REORDER: &str = "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/rules/reorder";

/// Mail用户邮箱联系人
/// 邮件联系人管理
pub const MAIL_V1_USER_MAILBOX_MAIL_CONTACTS: &str = "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/mail_contacts";
pub const MAIL_V1_USER_MAILBOX_MAIL_CONTACT: &str = "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/mail_contacts/{contact_id}";

/// Mail用户邮箱附件
/// 邮件附件下载和管理
pub const MAIL_V1_USER_MAILBOX_MESSAGE_ATTACHMENT_DOWNLOAD_URL: &str = "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/messages/{message_id}/attachments/{attachment_id}/download_url";

// ==================== VC (视频会议) v1 ====================
// 视频会议系统 - 会议室管理、会议创建、参会控制

/// VC会议室管理
/// 会议室的创建、查询、管理
pub const VC_V1_ROOMS: &str = "/open-apis/vc/v1/rooms";
pub const VC_V1_ROOM_GET: &str = "/open-apis/vc/v1/rooms/{room_id}";
pub const VC_V1_ROOM_CREATE: &str = "/open-apis/vc/v1/rooms";
pub const VC_V1_ROOM_UPDATE: &str = "/open-apis/vc/v1/rooms/{room_id}";
pub const VC_V1_ROOM_DELETE: &str = "/open-apis/vc/v1/rooms/{room_id}";
pub const VC_V1_ROOM_SEARCH: &str = "/open-apis/vc/v1/rooms/search";

/// VC会议管理
/// 会议的创建、管理、控制
pub const VC_V1_MEETINGS: &str = "/open-apis/vc/v1/meetings";
pub const VC_V1_MEETING_GET: &str = "/open-apis/vc/v1/meetings/{meeting_id}";
pub const VC_V1_MEETING_CREATE: &str = "/open-apis/vc/v1/meetings";
pub const VC_V1_MEETING_UPDATE: &str = "/open-apis/vc/v1/meetings/{meeting_id}";
pub const VC_V1_MEETING_END: &str = "/open-apis/vc/v1/meetings/{meeting_id}/end";
pub const VC_V1_MEETING_INVITE: &str = "/open-apis/vc/v1/meetings/{meeting_id}/invite";
pub const VC_V1_MEETING_KICKOUT: &str = "/open-apis/vc/v1/meetings/{meeting_id}/kickout";
pub const VC_V1_MEETING_LIST_BY_NO: &str = "/open-apis/vc/v1/meetings/list_by_no";
pub const VC_V1_MEETING_SET_HOST: &str = "/open-apis/vc/v1/meetings/{meeting_id}/set_host";

// ==================== Event (事件系统) v1 ====================
// 事件系统 - 事件订阅、处理、分发机制

/// Event事件订阅
/// 系统事件的订阅和管理
pub const EVENT_V1_SUBSCRIPTIONS: &str = "/open-apis/event/v1/subscriptions";
pub const EVENT_V1_SUBSCRIPTION_CREATE: &str = "/open-apis/event/v1/subscriptions/create";
pub const EVENT_V1_SUBSCRIPTION_GET: &str = "/open-apis/event/v1/subscriptions/{subscription_id}";
pub const EVENT_V1_SUBSCRIPTION_DELETE: &str = "/open-apis/event/v1/subscriptions/{subscription_id}";
pub const EVENT_V1_SUBSCRIPTION_UPDATE: &str = "/open-apis/event/v1/subscriptions/{subscription_id}";

/// Event事件历史
/// 历史事件查询和重放
pub const EVENT_V1_HISTORY: &str = "/open-apis/event/v1/history";
pub const EVENT_V1_HISTORY_REPLAY: &str = "/open-apis/event/v1/history/replay";

/// Event事件分发
/// 事件的分发和路由
pub const EVENT_V1_DISPATCHER: &str = "/open-apis/event/v1/dispatcher";
pub const EVENT_V1_DISPATCHER_STATUS: &str = "/open-apis/event/v1/dispatcher/status";

// ==================== Moments (动态分享) v1 ====================
// 动态分享系统 - 朋友圈动态、内容分享

/// Moments动态管理
/// 用户动态的创建和管理
pub const MOMENTS_V1_POSTS: &str = "/open-apis/moments/v1/posts";
pub const MOMENTS_V1_POST_CREATE: &str = "/open-apis/moments/v1/posts/create";
pub const MOMENTS_V1_POST_GET: &str = "/open-apis/moments/v1/posts/{post_id}";
pub const MOMENTS_V1_POST_UPDATE: &str = "/open-apis/moments/v1/posts/{post_id}";
pub const MOMENTS_V1_POST_DELETE: &str = "/open-apis/moments/v1/posts/{post_id}";

/// Moments内容分享
/// 内容的分享和传播
pub const MOMENTS_V1_SHARES: &str = "/open-apis/moments/v1/shares";
pub const MOMENTS_V1_SHARE_CREATE: &str = "/open-apis/moments/v1/shares/create";
pub const MOMENTS_V1_SHARE_GET: &str = "/open-apis/moments/v1/shares/{share_id}";
pub const MOMENTS_V1_SHARE_DELETE: &str = "/open-apis/moments/v1/shares/{share_id}";

/// Moments社交互动
/// 点赞、评论、转发等社交功能
pub const MOMENTS_V1_INTERACTIONS: &str = "/open-apis/moments/v1/interactions";
pub const MOMENTS_V1_INTERACTION_LIKE: &str = "/open-apis/moments/v1/interactions/like";
pub const MOMENTS_V1_INTERACTION_COMMENT: &str = "/open-apis/moments/v1/interactions/comment";
pub const MOMENTS_V1_INTERACTION_SHARE: &str = "/open-apis/moments/v1/interactions/share";

// ==================== 兼容性别名 ====================

/// 为保持向后兼容性，提供一些简短的别名
/// IM别名
pub const SEND_MESSAGE: &str = IM_V1_MESSAGES;
pub const CHAT_CREATE: &str = IM_V1_CHATS;

/// VC别名
pub const ROOM_LIST: &str = VC_V1_ROOMS;
pub const MEETING_LIST: &str = VC_V1_MEETINGS;

/// Mail别名
pub const MAILGROUP_LIST: &str = MAIL_V1_MAILGROUPS;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_im_endpoints() {
        // 验证IM端点
        assert!(IM_V1_MESSAGES.starts_with("/open-apis/im/v1/"));
        assert!(IM_V1_MESSAGES.contains("messages"));
        assert!(IM_V1_CHATS.contains("chats"));
        assert!(IM_V1_BATCH_MESSAGES.contains("batch_messages"));
        assert!(IM_V2_MESSAGES.starts_with("/open-apis/im/v2/"));
    }

    #[test]
    fn test_mail_endpoints() {
        // 验证Mail端点
        assert!(MAIL_V1_MAILGROUPS.starts_with("/open-apis/mail/v1/"));
        assert!(MAIL_V1_MAILGROUPS.contains("mailgroups"));
        assert!(MAIL_V1_USER_MAILBOX_EVENTS_SUBSCRIBE.contains("user_mailboxes"));
        assert!(MAIL_V1_USER_MAILBOX_FOLDERS.contains("folders"));
        assert!(MAIL_V1_USER_MAILBOX_MESSAGES.contains("messages"));
        assert!(MAIL_V1_USER_MAILBOX_RULES.contains("rules"));
        assert!(MAIL_V1_USER_MAILBOX_MAIL_CONTACTS.contains("mail_contacts"));
    }

    #[test]
    fn test_vc_endpoints() {
        // 验证VC端点
        assert!(VC_V1_ROOMS.starts_with("/open-apis/vc/v1/"));
        assert!(VC_V1_ROOMS.contains("rooms"));
        assert!(VC_V1_MEETINGS.contains("meetings"));
        assert!(VC_V1_ROOM_CREATE.contains("rooms"));
        assert!(VC_V1_MEETING_CREATE.contains("meetings"));
        assert!(VC_V1_MEETING_INVITE.contains("invite"));
        assert!(VC_V1_MEETING_KICKOUT.contains("kickout"));
        assert!(VC_V1_MEETING_SET_HOST.contains("set_host"));
    }

    #[test]
    fn test_event_endpoints() {
        // 验证Event端点
        assert!(EVENT_V1_SUBSCRIPTIONS.starts_with("/open-apis/event/v1/"));
        assert!(EVENT_V1_SUBSCRIPTIONS.contains("subscriptions"));
        assert!(EVENT_V1_HISTORY.contains("history"));
        assert!(EVENT_V1_DISPATCHER.contains("dispatcher"));
        assert!(EVENT_V1_SUBSCRIPTION_CREATE.contains("create"));
        assert!(EVENT_V1_SUBSCRIPTION_DELETE.contains("{subscription_id}"));
    }

    #[test]
    fn test_moments_endpoints() {
        // 验证Moments端点
        assert!(MOMENTS_V1_POSTS.starts_with("/open-apis/moments/v1/"));
        assert!(MOMENTS_V1_POSTS.contains("posts"));
        assert!(MOMENTS_V1_SHARES.contains("shares"));
        assert!(MOMENTS_V1_INTERACTIONS.contains("interactions"));
        assert!(MOMENTS_V1_POST_CREATE.contains("create"));
        assert!(MOMENTS_V1_INTERACTION_LIKE.contains("like"));
        assert!(MOMENTS_V1_INTERACTION_COMMENT.contains("comment"));
    }

    #[test]
    fn test_backward_compatibility() {
        // 验证兼容性别名
        assert_eq!(SEND_MESSAGE, IM_V1_MESSAGES);
        assert_eq!(CHAT_CREATE, IM_V1_CHATS);
        assert_eq!(ROOM_LIST, VC_V1_ROOMS);
        assert_eq!(MEETING_LIST, VC_V1_MEETINGS);
        assert_eq!(MAILGROUP_LIST, MAIL_V1_MAILGROUPS);
    }

    #[test]
    fn test_endpoint_parameter_placeholders() {
        // 测试关键端点是否包含正确的参数占位符
        assert!(MAIL_V1_MAILGROUP.contains("{mailgroup_id}"));
        assert!(MAIL_V1_MAILGROUP_MANAGERS_BATCH_CREATE.contains("{mailgroup_id}"));
        assert!(MAIL_V1_USER_MAILBOX_EVENTS_SUBSCRIBE.contains("{user_mailbox_id}"));
        assert!(MAIL_V1_USER_MAILBOX_FOLDERS.contains("{user_mailbox_id}"));
        assert!(MAIL_V1_USER_MAILBOX_FOLDER.contains("{user_mailbox_id}"));
        assert!(MAIL_V1_USER_MAILBOX_FOLDER.contains("{folder_id}"));
        assert!(MAIL_V1_USER_MAILBOX_MESSAGE.contains("{user_mailbox_id}"));
        assert!(MAIL_V1_USER_MAILBOX_MESSAGE.contains("{message_id}"));
        assert!(VC_V1_ROOM_GET.contains("{room_id}"));
        assert!(VC_V1_ROOM_UPDATE.contains("{room_id}"));
        assert!(VC_V1_ROOM_DELETE.contains("{room_id}"));
        assert!(VC_V1_MEETING_GET.contains("{meeting_id}"));
        assert!(VC_V1_MEETING_UPDATE.contains("{meeting_id}"));
        assert!(VC_V1_MEETING_END.contains("{meeting_id}"));
        assert!(VC_V1_MEETING_INVITE.contains("{meeting_id}"));
        assert!(VC_V1_MEETING_KICKOUT.contains("{meeting_id}"));
        assert!(VC_V1_MEETING_SET_HOST.contains("{meeting_id}"));
        assert!(EVENT_V1_SUBSCRIPTION_GET.contains("{subscription_id}"));
        assert!(EVENT_V1_SUBSCRIPTION_DELETE.contains("{subscription_id}"));
        assert!(EVENT_V1_SUBSCRIPTION_UPDATE.contains("{subscription_id}"));
        assert!(MOMENTS_V1_POST_GET.contains("{post_id}"));
        assert!(MOMENTS_V1_POST_UPDATE.contains("{post_id}"));
        assert!(MOMENTS_V1_POST_DELETE.contains("{post_id}"));
        assert!(MOMENTS_V1_SHARE_GET.contains("{share_id}"));
        assert!(MOMENTS_V1_SHARE_DELETE.contains("{share_id}"));
    }

    #[test]
    fn test_service_grouping() {
        // 测试服务分组的正确性
        let im_endpoints = [IM_V1_MESSAGES, IM_V1_CHATS, IM_V2_MESSAGES];
        for endpoint in im_endpoints {
            assert!(endpoint.contains("/im/"), "{} 应该包含 /im/", endpoint);
        }

        let mail_endpoints = [MAIL_V1_MAILGROUPS, MAIL_V1_USER_MAILBOX_EVENTS_SUBSCRIBE];
        for endpoint in mail_endpoints {
            assert!(endpoint.contains("/mail/"), "{} 应该包含 /mail/", endpoint);
        }

        let vc_endpoints = [VC_V1_ROOMS, VC_V1_MEETINGS, VC_V1_ROOM_CREATE];
        for endpoint in vc_endpoints {
            assert!(endpoint.contains("/vc/"), "{} 应该包含 /vc/", endpoint);
        }

        let event_endpoints = [EVENT_V1_SUBSCRIPTIONS, EVENT_V1_HISTORY, EVENT_V1_DISPATCHER];
        for endpoint in event_endpoints {
            assert!(endpoint.contains("/event/"), "{} 应该包含 /event/", endpoint);
        }

        let moments_endpoints = [MOMENTS_V1_POSTS, MOMENTS_V1_SHARES, MOMENTS_V1_INTERACTIONS];
        for endpoint in moments_endpoints {
            assert!(endpoint.contains("/moments/"), "{} 应该包含 /moments/", endpoint);
        }
    }
}// Endpoints and EndpointBuilder are now available directly from openlark_core::endpoints
