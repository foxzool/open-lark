//! im 域 typed 事件（`im.*`）。
//!
//! 字段权威：Go SDK v3.12.2 `service/im/v1/model.go` 对应 `P2*` 结构的 json tag；
//! 事件清单对齐 `service/im/v1/dispatcher.go`。Go 指针字段映射为 `Option<T>`，
//! serde 采用 `#[serde(default)]` 宽松风格（`im.message.receive_v1` 家族同样如此）。

use serde::{Deserialize, Serialize};

use super::EventHeader;

/// `im.chat.disbanded_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ImChatDisbandedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: ImChatDisbandedV1Event,
}

/// `im.chat.disbanded_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ImChatDisbandedV1Event {
    /// 群组 ID。
    #[serde(default)]
    pub chat_id: Option<String>,
    /// 解散群的操作者的 ID 信息。
    #[serde(default)]
    pub operator_id: Option<UserId>,
    /// 被解散的群是否是外部群。
    #[serde(default)]
    pub external: Option<bool>,
    /// 操作者的租户 Key。
    #[serde(default)]
    pub operator_tenant_key: Option<String>,
    /// 群名称。
    #[serde(default)]
    pub name: Option<String>,
    /// 群国际化名称。
    #[serde(default)]
    pub i18n_names: Option<I18nNames>,
}

/// 事件中的用户 ID 集合（Go `UserId`）。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct UserId {
    /// user_id。
    #[serde(default)]
    pub user_id: Option<String>,
    /// open_id。
    #[serde(default)]
    pub open_id: Option<String>,
    /// union_id。
    #[serde(default)]
    pub union_id: Option<String>,
}

/// 群国际化名称（Go `I18nNames`）。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct I18nNames {
    /// 中文名。
    #[serde(default)]
    pub zh_cn: Option<String>,
    /// 英文名。
    #[serde(default)]
    pub en_us: Option<String>,
    /// 日文名。
    #[serde(default)]
    pub ja_jp: Option<String>,
}

/// `im.chat.updated_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ImChatUpdatedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: ImChatUpdatedV1Event,
}

/// `im.chat.updated_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ImChatUpdatedV1Event {
    /// 群组 ID。
    #[serde(default)]
    pub chat_id: Option<String>,
    /// 修改群配置的操作者的 ID 信息。
    #[serde(default)]
    pub operator_id: Option<UserId>,
    /// 是否是外部群。
    #[serde(default)]
    pub external: Option<bool>,
    /// 操作者的租户 Key。
    #[serde(default)]
    pub operator_tenant_key: Option<String>,
    /// 更新后的群信息。
    #[serde(default)]
    pub after_change: Option<ChatChange>,
    /// 更新前的群信息。
    #[serde(default)]
    pub before_change: Option<ChatChange>,
    /// 群可发言成员名单的变更信息。
    #[serde(default)]
    pub moderator_list: Option<ModeratorList>,
}

/// 群信息变更快照（Go `ChatChange`）。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChatChange {
    /// 群头像。
    #[serde(default)]
    pub avatar: Option<String>,
    /// 群名称。
    #[serde(default)]
    pub name: Option<String>,
    /// 群描述。
    #[serde(default)]
    pub description: Option<String>,
    /// 群国际化名称。
    #[serde(default)]
    pub i18n_names: Option<I18nNames>,
    /// 谁可以邀请成员（用户或机器人）入群。
    #[serde(default)]
    pub add_member_permission: Option<String>,
    /// 群分享权限。
    #[serde(default)]
    pub share_card_permission: Option<String>,
    /// 谁可以 at 所有人。
    #[serde(default)]
    pub at_all_permission: Option<String>,
    /// 谁可以编辑群。
    #[serde(default)]
    pub edit_permission: Option<String>,
    /// 加群是否需要审批。
    #[serde(default)]
    pub membership_approval: Option<String>,
    /// 入群提示消息的可见性。
    #[serde(default)]
    pub join_message_visibility: Option<String>,
    /// 出群消息可见性。
    #[serde(default)]
    pub leave_message_visibility: Option<String>,
    /// 发言权限。
    #[serde(default)]
    pub moderation_permission: Option<String>,
    /// 群主 ID。
    #[serde(default)]
    pub owner_id: Option<UserId>,
    /// 群标签。
    #[serde(default)]
    pub labels: Vec<String>,
    /// 防泄密模式设置。
    #[serde(default)]
    pub restricted_mode_setting: Option<RestrictedModeSetting>,
    /// 群消息形式（`chat` / `thread`）。
    #[serde(default)]
    pub group_message_type: Option<String>,
}

/// 防泄密模式设置（Go `RestrictedModeSetting`）。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct RestrictedModeSetting {
    /// 防泄密模式是否开启。
    #[serde(default)]
    pub status: Option<bool>,
    /// 允许截屏录屏。
    #[serde(default)]
    pub screenshot_has_permission_setting: Option<String>,
    /// 允许下载消息中图片、视频和文件。
    #[serde(default)]
    pub download_has_permission_setting: Option<String>,
    /// 允许复制和转发消息。
    #[serde(default)]
    pub message_has_permission_setting: Option<String>,
}

/// 群可发言成员名单变更（Go `ModeratorList`）。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ModeratorList {
    /// 被添加进可发言名单的用户列表（该列表中一定有群主）。
    #[serde(default)]
    pub added_member_list: Vec<ListEventModerator>,
    /// 被移除出可发言名单的用户列表。
    #[serde(default)]
    pub removed_member_list: Vec<ListEventModerator>,
}

/// 可发言名单成员（Go `ListEventModerator`）。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListEventModerator {
    /// 租户 Key。
    #[serde(default)]
    pub tenant_key: Option<String>,
    /// 用户 ID。
    #[serde(default)]
    pub user_id: Option<UserId>,
}

/// `im.chat.access_event.bot_p2p_chat_entered_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ImChatAccessEventBotP2pChatEnteredV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: ImChatAccessEventBotP2pChatEnteredV1Event,
}

/// `im.chat.access_event.bot_p2p_chat_entered_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ImChatAccessEventBotP2pChatEnteredV1Event {
    /// 会话 ID。
    #[serde(default)]
    pub chat_id: Option<String>,
    /// 操作者的 ID。
    #[serde(default)]
    pub operator_id: Option<UserId>,
    /// 会话内用户可见的最新消息 ID。
    #[serde(default)]
    pub last_message_id: Option<String>,
    /// 会话内用户可见的最新消息发送时间（毫秒时间戳）。
    #[serde(default, deserialize_with = "super::flex_time::de_opt_string")]
    pub last_message_create_time: Option<String>,
}

/// `im.chat.member.bot.added_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ImChatMemberBotAddedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: ImChatMemberBotAddedV1Event,
}

/// `im.chat.member.bot.added_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ImChatMemberBotAddedV1Event {
    /// 群组 ID。
    #[serde(default)]
    pub chat_id: Option<String>,
    /// 操作者的 ID。
    #[serde(default)]
    pub operator_id: Option<UserId>,
    /// 是否是外部群。
    #[serde(default)]
    pub external: Option<bool>,
    /// 操作者的租户 Key。
    #[serde(default)]
    pub operator_tenant_key: Option<String>,
    /// 群名称。
    #[serde(default)]
    pub name: Option<String>,
    /// 群国际化名称。
    #[serde(default)]
    pub i18n_names: Option<I18nNames>,
}

/// `im.chat.member.bot.deleted_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ImChatMemberBotDeletedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: ImChatMemberBotDeletedV1Event,
}

/// `im.chat.member.bot.deleted_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ImChatMemberBotDeletedV1Event {
    /// 群组 ID。
    #[serde(default)]
    pub chat_id: Option<String>,
    /// 操作者的 ID。
    #[serde(default)]
    pub operator_id: Option<UserId>,
    /// 是否是外部群。
    #[serde(default)]
    pub external: Option<bool>,
    /// 操作者的租户 Key。
    #[serde(default)]
    pub operator_tenant_key: Option<String>,
    /// 群名称。
    #[serde(default)]
    pub name: Option<String>,
    /// 群国际化名称。
    #[serde(default)]
    pub i18n_names: Option<I18nNames>,
}

/// `im.chat.member.user.added_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ImChatMemberUserAddedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: ImChatMemberUserAddedV1Event,
}

/// `im.chat.member.user.added_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ImChatMemberUserAddedV1Event {
    /// 群组 ID。
    #[serde(default)]
    pub chat_id: Option<String>,
    /// 操作者的 ID。
    #[serde(default)]
    pub operator_id: Option<UserId>,
    /// 是否是外部群。
    #[serde(default)]
    pub external: Option<bool>,
    /// 操作者的租户 Key。
    #[serde(default)]
    pub operator_tenant_key: Option<String>,
    /// 被添加的用户列表。
    #[serde(default)]
    pub users: Vec<ChatMemberUser>,
    /// 群名称。
    #[serde(default)]
    pub name: Option<String>,
    /// 群国际化名称。
    #[serde(default)]
    pub i18n_names: Option<I18nNames>,
}

/// `im.chat.member.user.deleted_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ImChatMemberUserDeletedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: ImChatMemberUserDeletedV1Event,
}

/// `im.chat.member.user.deleted_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ImChatMemberUserDeletedV1Event {
    /// 群组 ID。
    #[serde(default)]
    pub chat_id: Option<String>,
    /// 操作者的 ID。
    #[serde(default)]
    pub operator_id: Option<UserId>,
    /// 是否是外部群。
    #[serde(default)]
    pub external: Option<bool>,
    /// 操作者的租户 Key。
    #[serde(default)]
    pub operator_tenant_key: Option<String>,
    /// 被移除用户列表。
    #[serde(default)]
    pub users: Vec<ChatMemberUser>,
    /// 群名称。
    #[serde(default)]
    pub name: Option<String>,
    /// 群国际化名称。
    #[serde(default)]
    pub i18n_names: Option<I18nNames>,
}

/// `im.chat.member.user.withdrawn_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ImChatMemberUserWithdrawnV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: ImChatMemberUserWithdrawnV1Event,
}

/// `im.chat.member.user.withdrawn_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ImChatMemberUserWithdrawnV1Event {
    /// 群组 ID。
    #[serde(default)]
    pub chat_id: Option<String>,
    /// 操作者的 ID。
    #[serde(default)]
    pub operator_id: Option<UserId>,
    /// 是否是外部群。
    #[serde(default)]
    pub external: Option<bool>,
    /// 操作者的租户 Key。
    #[serde(default)]
    pub operator_tenant_key: Option<String>,
    /// 被撤销加群的用户列表。
    #[serde(default)]
    pub users: Vec<ChatMemberUser>,
    /// 群名称。
    #[serde(default)]
    pub name: Option<String>,
    /// 群国际化名称。
    #[serde(default)]
    pub i18n_names: Option<I18nNames>,
}

/// 群成员变更中的用户（Go `ChatMemberUser`）。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChatMemberUser {
    /// 用户名字。
    #[serde(default)]
    pub name: Option<String>,
    /// 租户 Key。
    #[serde(default)]
    pub tenant_key: Option<String>,
    /// 用户 ID。
    #[serde(default)]
    pub user_id: Option<UserId>,
}

/// `im.message.message_read_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ImMessageMessageReadV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: ImMessageMessageReadV1Event,
}

/// `im.message.message_read_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ImMessageMessageReadV1Event {
    /// 已读的加急用户。
    #[serde(default)]
    pub reader: Option<EventMessageReader>,
    /// 消息 ID 列表。
    #[serde(default)]
    pub message_id_list: Vec<String>,
}

/// 已读回执中的读者（Go `EventMessageReader`）。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EventMessageReader {
    /// 用户 ID。
    #[serde(default)]
    pub reader_id: Option<UserId>,
    /// 用户已读的时间，毫秒级时间戳。
    #[serde(default, deserialize_with = "super::flex_time::de_opt_string")]
    pub read_time: Option<String>,
    /// 租户 Key。
    #[serde(default)]
    pub tenant_key: Option<String>,
}

/// `im.message.recalled_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ImMessageRecalledV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: ImMessageRecalledV1Event,
}

/// `im.message.recalled_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ImMessageRecalledV1Event {
    /// 被撤回的消息 ID。
    #[serde(default)]
    pub message_id: Option<String>,
    /// 消息所在的群组 ID。
    #[serde(default)]
    pub chat_id: Option<String>,
    /// 撤回的时间，毫秒级时间戳。
    #[serde(default, deserialize_with = "super::flex_time::de_opt_string")]
    pub recall_time: Option<String>,
    /// 撤回类型。
    #[serde(default)]
    pub recall_type: Option<String>,
}

/// `im.message.reaction.created_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ImMessageReactionCreatedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: ImMessageReactionCreatedV1Event,
}

/// `im.message.reaction.created_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ImMessageReactionCreatedV1Event {
    /// 消息 ID。
    #[serde(default)]
    pub message_id: Option<String>,
    /// 表情回复的资源类型。
    #[serde(default)]
    pub reaction_type: Option<Emoji>,
    /// 操作人类型（`user` / `app`）。
    #[serde(default)]
    pub operator_type: Option<String>,
    /// 用户 ID（`operator_type` 为 `user` 时有返回值）。
    #[serde(default)]
    pub user_id: Option<UserId>,
    /// 应用 ID（`operator_type` 为 `app` 时有返回值）。
    #[serde(default)]
    pub app_id: Option<String>,
    /// 添加表情回复的时间戳，单位：ms。
    #[serde(default, deserialize_with = "super::flex_time::de_opt_string")]
    pub action_time: Option<String>,
}

/// `im.message.reaction.deleted_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ImMessageReactionDeletedV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: ImMessageReactionDeletedV1Event,
}

/// `im.message.reaction.deleted_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ImMessageReactionDeletedV1Event {
    /// 消息 ID。
    #[serde(default)]
    pub message_id: Option<String>,
    /// 表情回复的资源类型。
    #[serde(default)]
    pub reaction_type: Option<Emoji>,
    /// 操作人类型（`user` / `app`）。
    #[serde(default)]
    pub operator_type: Option<String>,
    /// 用户 ID（`operator_type` 为 `user` 时有返回值）。
    #[serde(default)]
    pub user_id: Option<UserId>,
    /// 应用 ID（`operator_type` 为 `app` 时有返回值）。
    #[serde(default)]
    pub app_id: Option<String>,
    /// 删除表情回复的时间戳，单位：ms。
    #[serde(default, deserialize_with = "super::flex_time::de_opt_string")]
    pub action_time: Option<String>,
}

/// 表情回复类型（Go `Emoji`）。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct Emoji {
    /// emoji 类型。
    #[serde(default)]
    pub emoji_type: Option<String>,
}

/// `im.message.receive_v1` 完整 envelope。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ImMessageReceiveV1 {
    /// schema，通常为 `2.0`。
    #[serde(default)]
    pub schema: String,
    /// 事件头。
    pub header: EventHeader,
    /// 事件体。
    pub event: ImMessageReceiveV1Event,
}

/// `im.message.receive_v1` 的 `event` 对象。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ImMessageReceiveV1Event {
    /// 发送者。
    #[serde(default)]
    pub sender: Option<ImMessageSender>,
    /// 消息。
    pub message: ImReceivedMessage,
}

/// 消息发送者。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct ImMessageSender {
    /// sender_id 结构。
    #[serde(default)]
    pub sender_id: Option<ImSenderId>,
    /// 发送者类型，例如 `user`。
    #[serde(default)]
    pub sender_type: String,
    /// 租户 key。
    #[serde(default)]
    pub tenant_key: String,
}

/// 发送者 ID 集合。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct ImSenderId {
    /// union_id。
    #[serde(default)]
    pub union_id: String,
    /// user_id。
    #[serde(default)]
    pub user_id: String,
    /// open_id。
    #[serde(default)]
    pub open_id: String,
}

/// 收到的 IM 消息。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct ImReceivedMessage {
    /// 消息 ID。
    #[serde(default)]
    pub message_id: String,
    /// 根消息 ID。
    #[serde(default)]
    pub root_id: String,
    /// 父消息 ID。
    #[serde(default)]
    pub parent_id: String,
    /// 创建时间（毫秒）。JSON 字符串与整数都接受，存成十进制字符串。
    #[serde(default, deserialize_with = "super::flex_time::de_string")]
    pub create_time: String,
    /// 更新时间（毫秒）。JSON 字符串与整数都接受，存成十进制字符串。
    #[serde(default, deserialize_with = "super::flex_time::de_string")]
    pub update_time: String,
    /// 聊天 ID。
    #[serde(default)]
    pub chat_id: String,
    /// 话题 ID。无话题时为空。
    #[serde(default)]
    pub thread_id: String,
    /// 聊天类型。
    #[serde(default)]
    pub chat_type: String,
    /// 消息类型。
    #[serde(default)]
    pub message_type: String,
    /// 内容 JSON 字符串。
    #[serde(default)]
    pub content: String,
    /// 被 @ 的用户。
    #[serde(default)]
    pub mentions: Vec<ImMention>,
    /// 发送端 `user_agent`。缺字段时为 `None`。
    #[serde(default)]
    pub user_agent: Option<String>,
}

/// 消息中的 @ 提及。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct ImMention {
    /// 序号，例如 `@_user_1`。
    #[serde(default)]
    pub key: String,
    /// 被提及用户 ID。
    #[serde(default)]
    pub id: Option<ImSenderId>,
    /// `user` 或 `bot`。
    #[serde(default)]
    pub mentioned_type: String,
    /// 姓名。
    #[serde(default)]
    pub name: String,
    /// 租户 key。
    #[serde(default)]
    pub tenant_key: String,
}

typed_events! {
    /// `im.message.receive_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "im.message.receive_v1" | IM_MESSAGE_RECEIVE_V1 | ImMessageReceiveV1 | ImMessageReceiveV1Handler | register_im_message_receive_v1;

    /// `im.chat.disbanded_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "im.chat.disbanded_v1" | IM_CHAT_DISBANDED_V1 | ImChatDisbandedV1 | ImChatDisbandedV1Handler | register_im_chat_disbanded_v1;

    /// `im.chat.updated_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "im.chat.updated_v1" | IM_CHAT_UPDATED_V1 | ImChatUpdatedV1 | ImChatUpdatedV1Handler | register_im_chat_updated_v1;

    /// `im.chat.access_event.bot_p2p_chat_entered_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "im.chat.access_event.bot_p2p_chat_entered_v1" | IM_CHAT_ACCESS_EVENT_BOT_P2P_CHAT_ENTERED_V1 | ImChatAccessEventBotP2pChatEnteredV1 | ImChatAccessEventBotP2pChatEnteredV1Handler | register_im_chat_access_event_bot_p2p_chat_entered_v1;

    /// `im.chat.member.bot.added_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "im.chat.member.bot.added_v1" | IM_CHAT_MEMBER_BOT_ADDED_V1 | ImChatMemberBotAddedV1 | ImChatMemberBotAddedV1Handler | register_im_chat_member_bot_added_v1;

    /// `im.chat.member.bot.deleted_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "im.chat.member.bot.deleted_v1" | IM_CHAT_MEMBER_BOT_DELETED_V1 | ImChatMemberBotDeletedV1 | ImChatMemberBotDeletedV1Handler | register_im_chat_member_bot_deleted_v1;

    /// `im.chat.member.user.added_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "im.chat.member.user.added_v1" | IM_CHAT_MEMBER_USER_ADDED_V1 | ImChatMemberUserAddedV1 | ImChatMemberUserAddedV1Handler | register_im_chat_member_user_added_v1;

    /// `im.chat.member.user.deleted_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "im.chat.member.user.deleted_v1" | IM_CHAT_MEMBER_USER_DELETED_V1 | ImChatMemberUserDeletedV1 | ImChatMemberUserDeletedV1Handler | register_im_chat_member_user_deleted_v1;

    /// `im.chat.member.user.withdrawn_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "im.chat.member.user.withdrawn_v1" | IM_CHAT_MEMBER_USER_WITHDRAWN_V1 | ImChatMemberUserWithdrawnV1 | ImChatMemberUserWithdrawnV1Handler | register_im_chat_member_user_withdrawn_v1;

    /// `im.message.message_read_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "im.message.message_read_v1" | IM_MESSAGE_MESSAGE_READ_V1 | ImMessageMessageReadV1 | ImMessageMessageReadV1Handler | register_im_message_message_read_v1;

    /// `im.message.recalled_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "im.message.recalled_v1" | IM_MESSAGE_RECALLED_V1 | ImMessageRecalledV1 | ImMessageRecalledV1Handler | register_im_message_recalled_v1;

    /// `im.message.reaction.created_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "im.message.reaction.created_v1" | IM_MESSAGE_REACTION_CREATED_V1 | ImMessageReactionCreatedV1 | ImMessageReactionCreatedV1Handler | register_im_message_reaction_created_v1;

    /// `im.message.reaction.deleted_v1` typed 处理器。
    ///
    /// 闭包只要签名匹配也会自动实现本 trait。
    "im.message.reaction.deleted_v1" | IM_MESSAGE_REACTION_DELETED_V1 | ImMessageReactionDeletedV1 | ImMessageReactionDeletedV1Handler | register_im_message_reaction_deleted_v1;
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use std::time::Instant;

    use super::*;
    use crate::ws_client::{EventDispatcherHandler, EventHandler};

    const CHAT_DISBANDED_FIXTURE: &str = r#"{
      "schema":"2.0",
      "header":{"event_id":"e1","event_type":"im.chat.disbanded_v1","create_time":"1700000000000","token":"t","app_id":"cli_x","tenant_key":"tk"},
      "event":{
        "chat_id":"oc_disbanded",
        "operator_id":{"union_id":"ou_op","user_id":"u_op","open_id":"on_op"},
        "external":false,
        "operator_tenant_key":"tk2",
        "name":"旧群",
        "i18n_names":{"zh_cn":"旧群","en_us":"Old Group","ja_jp":"旧グループ"}
      }
    }"#;

    const CHAT_UPDATED_FIXTURE: &str = r#"{
      "schema":"2.0",
      "header":{"event_id":"e2","event_type":"im.chat.updated_v1","token":"t","app_id":"cli_x","tenant_key":"tk"},
      "event":{
        "chat_id":"oc_upd",
        "operator_id":{"open_id":"on_op"},
        "external":false,
        "operator_tenant_key":"tk",
        "after_change":{"name":"新名","owner_id":{"open_id":"on_owner"},"labels":["l1","l2"],"restricted_mode_setting":{"status":true},"group_message_type":"thread"},
        "before_change":{"name":"旧名"},
        "moderator_list":{"added_member_list":[{"tenant_key":"tk","user_id":{"open_id":"on_m1"}}],"removed_member_list":[]}
      }
    }"#;

    const BOT_P2P_ENTERED_FIXTURE: &str = r#"{
      "schema":"2.0",
      "header":{"event_id":"e3","event_type":"im.chat.access_event.bot_p2p_chat_entered_v1","token":"t","app_id":"cli_x","tenant_key":"tk"},
      "event":{
        "chat_id":"oc_p2p",
        "operator_id":{"open_id":"on_u"},
        "last_message_id":"om_last",
        "last_message_create_time":"1700000000000"
      }
    }"#;

    const MEMBER_BOT_ADDED_FIXTURE: &str = r#"{
      "schema":"2.0",
      "header":{"event_id":"e4","event_type":"im.chat.member.bot.added_v1","token":"t","app_id":"cli_x","tenant_key":"tk"},
      "event":{"chat_id":"oc_g","operator_id":{"open_id":"on_op"},"external":false,"operator_tenant_key":"tk","name":"机器人群","i18n_names":{"zh_cn":"机器人群"}}
    }"#;

    const MEMBER_BOT_DELETED_FIXTURE: &str = r#"{
      "schema":"2.0",
      "header":{"event_id":"e5","event_type":"im.chat.member.bot.deleted_v1","token":"t","app_id":"cli_x","tenant_key":"tk"},
      "event":{"chat_id":"oc_g","operator_id":{"open_id":"on_op"},"external":false,"operator_tenant_key":"tk","name":"机器人群","i18n_names":{"zh_cn":"机器人群"}}
    }"#;

    const MEMBER_USER_ADDED_FIXTURE: &str = r#"{
      "schema":"2.0",
      "header":{"event_id":"e6","event_type":"im.chat.member.user.added_v1","token":"t","app_id":"cli_x","tenant_key":"tk"},
      "event":{
        "chat_id":"oc_g",
        "operator_id":{"open_id":"on_op"},
        "external":false,
        "operator_tenant_key":"tk",
        "users":[{"name":"张三","tenant_key":"tk","user_id":{"open_id":"on_u1","user_id":"u1"}}],
        "name":"群"
      }
    }"#;

    const MEMBER_USER_DELETED_FIXTURE: &str = r#"{
      "schema":"2.0",
      "header":{"event_id":"e7","event_type":"im.chat.member.user.deleted_v1","token":"t","app_id":"cli_x","tenant_key":"tk"},
      "event":{
        "chat_id":"oc_g",
        "operator_id":{"open_id":"on_op"},
        "external":false,
        "operator_tenant_key":"tk",
        "users":[{"name":"李四","tenant_key":"tk","user_id":{"open_id":"on_u2"}}],
        "name":"群"
      }
    }"#;

    const MEMBER_USER_WITHDRAWN_FIXTURE: &str = r#"{
      "schema":"2.0",
      "header":{"event_id":"e8","event_type":"im.chat.member.user.withdrawn_v1","token":"t","app_id":"cli_x","tenant_key":"tk"},
      "event":{
        "chat_id":"oc_g",
        "operator_id":{"open_id":"on_u3"},
        "external":false,
        "operator_tenant_key":"tk",
        "users":[{"name":"王五","tenant_key":"tk","user_id":{"open_id":"on_u3"}}],
        "name":"群"
      }
    }"#;

    const MESSAGE_READ_FIXTURE: &str = r#"{
      "schema":"2.0",
      "header":{"event_id":"e9","event_type":"im.message.message_read_v1","token":"t","app_id":"cli_x","tenant_key":"tk"},
      "event":{
        "reader":{"reader_id":{"open_id":"on_reader"},"read_time":"1700000000000","tenant_key":"tk"},
        "message_id_list":["om_1","om_2"]
      }
    }"#;

    const MESSAGE_RECALLED_FIXTURE: &str = r#"{
      "schema":"2.0",
      "header":{"event_id":"e10","event_type":"im.message.recalled_v1","token":"t","app_id":"cli_x","tenant_key":"tk"},
      "event":{"message_id":"om_recalled","chat_id":"oc_g","recall_time":"1700000000000","recall_type":"user_recall"}
    }"#;

    const REACTION_CREATED_FIXTURE: &str = r#"{
      "schema":"2.0",
      "header":{"event_id":"e11","event_type":"im.message.reaction.created_v1","token":"t","app_id":"cli_x","tenant_key":"tk"},
      "event":{"message_id":"om_r","reaction_type":{"emoji_type":"SMILE"},"operator_type":"user","user_id":{"open_id":"on_u"},"app_id":"","action_time":"1700000000000"}
    }"#;

    const REACTION_DELETED_FIXTURE: &str = r#"{
      "schema":"2.0",
      "header":{"event_id":"e12","event_type":"im.message.reaction.deleted_v1","token":"t","app_id":"cli_x","tenant_key":"tk"},
      "event":{"message_id":"om_r","reaction_type":{"emoji_type":"THUMBSUP"},"operator_type":"app","user_id":null,"app_id":"cli_bot","action_time":"1700000000001"}
    }"#;

    #[test]
    fn typed_im_chat_disbanded_deserializes_chat_id() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_im_chat_disbanded_v1(move |event: ImChatDisbandedV1| {
                assert_eq!(event.header.event_type, "im.chat.disbanded_v1");
                assert_eq!(
                    event
                        .event
                        .operator_id
                        .as_ref()
                        .and_then(|id| id.open_id.as_deref()),
                    Some("on_op")
                );
                assert_eq!(
                    event
                        .event
                        .i18n_names
                        .as_ref()
                        .and_then(|n| n.zh_cn.as_deref()),
                    Some("旧群")
                );
                *seen_h.lock().expect("lock") = event.event.chat_id.clone();
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(CHAT_DISBANDED_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("oc_disbanded"));
    }

    #[test]
    fn typed_im_chat_updated_deserializes_after_change() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_im_chat_updated_v1(move |event: ImChatUpdatedV1| {
                let after = event.event.after_change.expect("after_change");
                assert_eq!(after.name.as_deref(), Some("新名"));
                assert_eq!(
                    after.owner_id.as_ref().and_then(|id| id.open_id.as_deref()),
                    Some("on_owner")
                );
                assert_eq!(after.labels, vec!["l1".to_string(), "l2".to_string()]);
                assert_eq!(
                    after
                        .restricted_mode_setting
                        .as_ref()
                        .and_then(|s| s.status),
                    Some(true)
                );
                assert_eq!(
                    event
                        .event
                        .moderator_list
                        .as_ref()
                        .and_then(|m| m.added_member_list.first())
                        .and_then(|m| m.user_id.as_ref())
                        .and_then(|id| id.open_id.as_deref()),
                    Some("on_m1")
                );
                *seen_h.lock().expect("lock") = event.event.chat_id.clone();
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(CHAT_UPDATED_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("oc_upd"));
    }

    #[test]
    fn typed_im_bot_p2p_entered_deserializes_last_message() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_im_chat_access_event_bot_p2p_chat_entered_v1(
                move |event: ImChatAccessEventBotP2pChatEnteredV1| {
                    assert_eq!(
                        event.header.event_type,
                        "im.chat.access_event.bot_p2p_chat_entered_v1"
                    );
                    assert_eq!(
                        event
                            .event
                            .operator_id
                            .as_ref()
                            .and_then(|id| id.open_id.as_deref()),
                        Some("on_u")
                    );
                    assert_eq!(
                        event.event.last_message_create_time.as_deref(),
                        Some("1700000000000")
                    );
                    *seen_h.lock().expect("lock") = event.event.last_message_id.clone();
                    Ok(())
                },
            )
            .expect("register")
            .build();
        handler
            .do_without_validation(BOT_P2P_ENTERED_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("om_last"));
    }

    #[test]
    fn typed_im_member_bot_added_deserializes_chat_id() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_im_chat_member_bot_added_v1(move |event: ImChatMemberBotAddedV1| {
                assert_eq!(event.event.external, Some(false));
                assert_eq!(event.event.name.as_deref(), Some("机器人群"));
                *seen_h.lock().expect("lock") = event.event.chat_id.clone();
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(MEMBER_BOT_ADDED_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("oc_g"));
    }

    #[test]
    fn typed_im_member_bot_deleted_deserializes_chat_id() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_im_chat_member_bot_deleted_v1(move |event: ImChatMemberBotDeletedV1| {
                assert_eq!(
                    event
                        .event
                        .i18n_names
                        .as_ref()
                        .and_then(|n| n.zh_cn.as_deref()),
                    Some("机器人群")
                );
                *seen_h.lock().expect("lock") = event.event.chat_id.clone();
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(MEMBER_BOT_DELETED_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("oc_g"));
    }

    #[test]
    fn typed_im_member_user_added_deserializes_users() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_im_chat_member_user_added_v1(move |event: ImChatMemberUserAddedV1| {
                let user = event.event.users.first().expect("user");
                assert_eq!(user.name.as_deref(), Some("张三"));
                assert_eq!(
                    user.user_id.as_ref().and_then(|id| id.user_id.as_deref()),
                    Some("u1")
                );
                *seen_h.lock().expect("lock") = event.event.chat_id.clone();
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(MEMBER_USER_ADDED_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("oc_g"));
    }

    #[test]
    fn typed_im_member_user_deleted_deserializes_users() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_im_chat_member_user_deleted_v1(move |event: ImChatMemberUserDeletedV1| {
                let user = event.event.users.first().expect("user");
                assert_eq!(user.name.as_deref(), Some("李四"));
                assert_eq!(
                    user.user_id.as_ref().and_then(|id| id.open_id.as_deref()),
                    Some("on_u2")
                );
                *seen_h.lock().expect("lock") = event.event.chat_id.clone();
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(MEMBER_USER_DELETED_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("oc_g"));
    }

    #[test]
    fn typed_im_member_user_withdrawn_deserializes_users() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_im_chat_member_user_withdrawn_v1(move |event: ImChatMemberUserWithdrawnV1| {
                let user = event.event.users.first().expect("user");
                assert_eq!(user.name.as_deref(), Some("王五"));
                assert_eq!(
                    event
                        .event
                        .operator_id
                        .as_ref()
                        .and_then(|id| id.open_id.as_deref()),
                    Some("on_u3")
                );
                *seen_h.lock().expect("lock") = event.event.chat_id.clone();
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(MEMBER_USER_WITHDRAWN_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("oc_g"));
    }

    #[test]
    fn typed_im_message_read_deserializes_reader_and_ids() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_im_message_message_read_v1(move |event: ImMessageMessageReadV1| {
                let reader = event.event.reader.as_ref().expect("reader");
                assert_eq!(
                    reader
                        .reader_id
                        .as_ref()
                        .and_then(|id| id.open_id.as_deref()),
                    Some("on_reader")
                );
                assert_eq!(reader.read_time.as_deref(), Some("1700000000000"));
                assert_eq!(event.event.message_id_list, vec!["om_1", "om_2"]);
                *seen_h.lock().expect("lock") = Some(event.event.message_id_list.len());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(MESSAGE_READ_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(*seen.lock().expect("lock"), Some(2));
    }

    #[test]
    fn typed_im_message_recalled_deserializes_recall_fields() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_im_message_recalled_v1(move |event: ImMessageRecalledV1| {
                assert_eq!(event.event.chat_id.as_deref(), Some("oc_g"));
                assert_eq!(event.event.recall_time.as_deref(), Some("1700000000000"));
                assert_eq!(event.event.recall_type.as_deref(), Some("user_recall"));
                *seen_h.lock().expect("lock") = event.event.message_id.clone();
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(MESSAGE_RECALLED_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("om_recalled"));
    }

    #[test]
    fn typed_im_reaction_created_deserializes_emoji_type() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_im_message_reaction_created_v1(move |event: ImMessageReactionCreatedV1| {
                assert_eq!(
                    event
                        .event
                        .reaction_type
                        .as_ref()
                        .and_then(|e| e.emoji_type.as_deref()),
                    Some("SMILE")
                );
                assert_eq!(event.event.operator_type.as_deref(), Some("user"));
                assert_eq!(
                    event
                        .event
                        .user_id
                        .as_ref()
                        .and_then(|id| id.open_id.as_deref()),
                    Some("on_u")
                );
                *seen_h.lock().expect("lock") = event.event.message_id.clone();
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(REACTION_CREATED_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("om_r"));
    }

    #[test]
    fn typed_im_reaction_deleted_deserializes_app_operator() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_im_message_reaction_deleted_v1(move |event: ImMessageReactionDeletedV1| {
                assert_eq!(
                    event
                        .event
                        .reaction_type
                        .as_ref()
                        .and_then(|e| e.emoji_type.as_deref()),
                    Some("THUMBSUP")
                );
                assert_eq!(event.event.operator_type.as_deref(), Some("app"));
                assert_eq!(event.event.app_id.as_deref(), Some("cli_bot"));
                assert_eq!(event.event.user_id, None);
                *seen_h.lock().expect("lock") = event.event.message_id.clone();
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(REACTION_DELETED_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("om_r"));
    }

    /// 事件串常量与票面清单逐字一致。
    #[test]
    fn typed_im_event_constants_match_go_dispatcher() {
        assert_eq!(
            EventDispatcherHandler::IM_CHAT_DISBANDED_V1,
            "im.chat.disbanded_v1"
        );
        assert_eq!(
            EventDispatcherHandler::IM_CHAT_UPDATED_V1,
            "im.chat.updated_v1"
        );
        assert_eq!(
            EventDispatcherHandler::IM_CHAT_ACCESS_EVENT_BOT_P2P_CHAT_ENTERED_V1,
            "im.chat.access_event.bot_p2p_chat_entered_v1"
        );
        assert_eq!(
            EventDispatcherHandler::IM_CHAT_MEMBER_BOT_ADDED_V1,
            "im.chat.member.bot.added_v1"
        );
        assert_eq!(
            EventDispatcherHandler::IM_CHAT_MEMBER_BOT_DELETED_V1,
            "im.chat.member.bot.deleted_v1"
        );
        assert_eq!(
            EventDispatcherHandler::IM_CHAT_MEMBER_USER_ADDED_V1,
            "im.chat.member.user.added_v1"
        );
        assert_eq!(
            EventDispatcherHandler::IM_CHAT_MEMBER_USER_DELETED_V1,
            "im.chat.member.user.deleted_v1"
        );
        assert_eq!(
            EventDispatcherHandler::IM_CHAT_MEMBER_USER_WITHDRAWN_V1,
            "im.chat.member.user.withdrawn_v1"
        );
        assert_eq!(
            EventDispatcherHandler::IM_MESSAGE_MESSAGE_READ_V1,
            "im.message.message_read_v1"
        );
        assert_eq!(
            EventDispatcherHandler::IM_MESSAGE_RECALLED_V1,
            "im.message.recalled_v1"
        );
        assert_eq!(
            EventDispatcherHandler::IM_MESSAGE_REACTION_CREATED_V1,
            "im.message.reaction.created_v1"
        );
        assert_eq!(
            EventDispatcherHandler::IM_MESSAGE_REACTION_DELETED_V1,
            "im.message.reaction.deleted_v1"
        );
    }

    /// typed 注册后再 `register_raw` 同 key 返回 Err；反方向同理。
    #[test]
    fn typed_im_duplicate_register_fails() {
        struct Noop;
        impl crate::ws_client::EventHandler for Noop {
            fn handle(
                &self,
                _payload: &[u8],
            ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
                Ok(())
            }
        }
        let err = EventDispatcherHandler::builder()
            .register_im_chat_disbanded_v1(|_event: ImChatDisbandedV1| Ok(()))
            .expect("first")
            .register_raw("im.chat.disbanded_v1", Noop)
            .expect_err("dup typed-then-raw");
        assert!(err.contains("already registered"));
        let err = EventDispatcherHandler::builder()
            .register_raw("im.message.recalled_v1", Noop)
            .expect("raw first")
            .register_im_message_recalled_v1(|_event: ImMessageRecalledV1| Ok(()))
            .expect_err("dup raw-then-typed");
        assert!(err.contains("already registered"));
    }

    #[test]
    #[allow(deprecated)]
    fn flat_reexports_still_work_while_deprecated() {
        // #709：flat 再导出已标 deprecated，但按 DEPRECATED_API_SUPPORT_POLICY
        // 至少保留一个公开发布周期；且与模块路径是同一类型，可互换。
        use crate::ws_client::ImMessageReceiveV1 as FlatIm;
        use crate::ws_client::im::ImMessageReceiveV1 as ModIm;

        fn module_typed(event: ModIm) -> ModIm {
            event
        }
        let flat: FlatIm = serde_json::from_str(IM_FIXTURE).expect("parse flat");
        let via_module = module_typed(flat);
        assert_eq!(via_module.event.message.message_id, "om_hello");
    }

    #[test]
    fn im_message_receive_v1_available_at_module_path_and_dispatches() {
        // #709：`im.message.receive_v1` 家族从 typed.rs 手写链收敛进本模块，
        // `ws_client::im::*` 成为正典入口（flat 再导出随后标 deprecated）。
        use crate::ws_client::im::{ImMessageReceiveV1, ImMessageReceiveV1Event};

        let fixture = br#"{
          "schema":"2.0",
          "header":{"event_id":"e1","event_type":"im.message.receive_v1","token":"t","app_id":"cli_x","tenant_key":"tk"},
          "event":{
            "sender":{"sender_id":{"open_id":"ou_1"},"sender_type":"user","tenant_key":"tk"},
            "message":{
              "message_id":"om_modpath",
              "chat_id":"oc_1",
              "chat_type":"p2p",
              "message_type":"text",
              "content":"{\"text\":\"hi\"}",
              "mentions":[{"key":"@_user_1","id":{"open_id":"ou_mentioned"},"name":"Ada","mentioned_type":"user"}]
            }
          }
        }"#;

        let seen = std::sync::Arc::new(std::sync::Mutex::new(None));
        let seen_h = std::sync::Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_im_message_receive_v1(move |event: ImMessageReceiveV1| {
                *seen_h.lock().expect("lock") = Some(event.event.message.message_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler.do_without_validation(fixture).expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("om_modpath"));

        // 模块路径上可独立反序列化 envelope。
        let envelope: ImMessageReceiveV1 = serde_json::from_slice(fixture).expect("deserialize");
        let ImMessageReceiveV1Event { sender, .. } = envelope.event;
        assert_eq!(
            sender.expect("sender").sender_id.expect("id").open_id,
            "ou_1"
        );
    }
    // ---- 以下测试自 typed.rs 迁入（#709：im.message.receive_v1 收敛进本模块） ----

    const IM_FIXTURE: &str = r#"{
      "schema":"2.0",
      "header":{"event_id":"e1","event_type":"im.message.receive_v1","token":"t","app_id":"cli_x","tenant_key":"tk"},
      "event":{
        "sender":{"sender_id":{"open_id":"ou_1"},"sender_type":"user","tenant_key":"tk"},
        "message":{
          "message_id":"om_hello",
          "chat_id":"oc_1",
          "chat_type":"p2p",
          "message_type":"text",
          "content":"{\"text\":\"hi\"}",
          "mentions":[{"key":"@_user_1","id":{"open_id":"ou_mentioned"},"name":"Ada","mentioned_type":"user"}]
        }
      }
    }"#;

    struct CaptureIm(Arc<Mutex<Option<String>>>);
    impl ImMessageReceiveV1Handler for CaptureIm {
        fn handle(
            &self,
            event: ImMessageReceiveV1,
        ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            *self.0.lock().expect("lock") = Some(event.event.message.message_id);
            Ok(())
        }
    }

    #[test]
    fn typed_im_message_receive_deserializes_message_id() {
        let seen = Arc::new(Mutex::new(None));
        let handler = EventDispatcherHandler::builder()
            .register_im_message_receive_v1(CaptureIm(Arc::clone(&seen)))
            .expect("register")
            .build();
        handler
            .do_without_validation(IM_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("om_hello"));
    }

    #[test]
    fn typed_im_message_receive_deserializes_mention_open_id() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_im_message_receive_v1(move |event: ImMessageReceiveV1| {
                let mention = &event.event.message.mentions[0];
                *seen_h.lock().expect("lock") = mention.id.as_ref().map(|id| id.open_id.clone());
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(IM_FIXTURE.as_bytes())
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("ou_mentioned"));
    }

    #[test]
    fn register_raw_still_works_for_im_message() {
        struct Raw(Arc<Mutex<bool>>);
        impl EventHandler for Raw {
            fn handle(
                &self,
                payload: &[u8],
            ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
                assert!(payload.starts_with(b"{"));
                *self.0.lock().expect("lock") = true;
                Ok(())
            }
        }
        let seen = Arc::new(Mutex::new(false));
        let handler = EventDispatcherHandler::builder()
            .register_raw("im.message.receive_v1", Raw(Arc::clone(&seen)))
            .expect("register")
            .build();
        handler
            .do_without_validation(IM_FIXTURE.as_bytes())
            .expect("dispatch");
        assert!(*seen.lock().expect("lock"));
    }

    #[test]
    fn typed_im_bad_json_errors() {
        let handler = EventDispatcherHandler::builder()
            .register_im_message_receive_v1(|_event: ImMessageReceiveV1| Ok(()))
            .expect("register")
            .build();
        let err = handler
            .do_without_validation(br#"{"header":{"event_type":"im.message.receive_v1"}}"#)
            .expect_err("bad json");
        assert!(
            err.contains("im.message.receive_v1"),
            "error should name the event type, got {err}"
        );
        assert!(
            err.contains("反序列化") || err.contains("missing field"),
            "error should mention deserialize failure, got {err}"
        );
    }

    #[test]
    fn typed_im_receive_duplicate_register_fails() {
        let err = EventDispatcherHandler::builder()
            .register_im_message_receive_v1(|_event: ImMessageReceiveV1| Ok(()))
            .expect("first")
            .register_raw("im.message.receive_v1", {
                struct Noop;
                impl EventHandler for Noop {
                    fn handle(
                        &self,
                        _payload: &[u8],
                    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
                        Ok(())
                    }
                }
                Noop
            })
            .expect_err("dup");
        assert!(err.contains("already registered"));
    }

    #[test]
    fn typed_im_dispatch_1000_under_budget() {
        let fixture = IM_FIXTURE.as_bytes();
        let raw = EventDispatcherHandler::builder()
            .register_raw("im.message.receive_v1", {
                // raw 使用者手动反序列化相同事件，确保比较的是相同处理工作。
                struct DeserializeImMessage;
                impl EventHandler for DeserializeImMessage {
                    fn handle(
                        &self,
                        payload: &[u8],
                    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
                        let event: ImMessageReceiveV1 = serde_json::from_slice(payload)?;
                        assert_eq!(event.event.message.message_id, "om_hello");
                        Ok(())
                    }
                }
                DeserializeImMessage
            })
            .expect("raw")
            .build();
        let typed = EventDispatcherHandler::builder()
            .register_im_message_receive_v1(|event: ImMessageReceiveV1| {
                assert_eq!(event.event.message.message_id, "om_hello");
                Ok(())
            })
            .expect("register")
            .build();

        for _ in 0..100 {
            raw.do_without_validation(fixture).expect("warm raw");
            typed.do_without_validation(fixture).expect("warm typed");
        }

        let raw_started = Instant::now();
        for _ in 0..1000 {
            raw.do_without_validation(fixture).expect("raw");
        }
        let raw_elapsed = raw_started.elapsed();

        let typed_started = Instant::now();
        for _ in 0..1000 {
            typed.do_without_validation(fixture).expect("typed");
        }
        let typed_elapsed = typed_started.elapsed();

        eprintln!(
            "perf668 raw={raw_elapsed:?} typed={typed_elapsed:?} ratio={:.3}",
            typed_elapsed.as_secs_f64() / raw_elapsed.as_secs_f64().max(1e-12)
        );
        assert!(
            typed_elapsed.as_millis() < 500,
            "1000 typed dispatches took {typed_elapsed:?}"
        );
        // 两条路径都在命中后解析相同类型，typed 注册糖仍应满足原来的 2x 预算。
        assert!(
            typed_elapsed.as_secs_f64() <= raw_elapsed.as_secs_f64() * 2.0,
            "typed {typed_elapsed:?} exceeds 2x raw {raw_elapsed:?}"
        );
    }

    #[test]
    fn message_user_agent_reaches_typed_handler() {
        let seen = Arc::new(Mutex::new(None));
        let seen_h = Arc::clone(&seen);
        let handler = EventDispatcherHandler::builder()
            .register_im_message_receive_v1(move |event: ImMessageReceiveV1| {
                *seen_h.lock().expect("lock") = event.event.message.user_agent.clone();
                Ok(())
            })
            .expect("register")
            .build();
        handler
            .do_without_validation(
                br#"{
                  "header":{"event_type":"im.message.receive_v1"},
                  "event":{"message":{"message_type":"text","user_agent":"Mozilla"}}
                }"#,
            )
            .expect("dispatch");
        assert_eq!(seen.lock().expect("lock").as_deref(), Some("Mozilla"));

        let missing: ImReceivedMessage = serde_json::from_str("{}").expect("defaults");
        assert_eq!(missing.user_agent, None);
    }
}
