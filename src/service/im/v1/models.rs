use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// 消息类型枚举
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MessageType {
    /// 文本消息
    Text,
    /// 富文本消息
    Post,
    /// 图片消息
    Image,
    /// 文件消息
    File,
    /// 音频消息
    Audio,
    /// 视频消息
    Media,
    /// 表情包消息
    Sticker,
    /// 交互式消息卡片
    Interactive,
    /// 分享群名片
    ShareChat,
    /// 分享用户名片
    ShareUser,
    /// 系统消息
    System,
}

/// 用户ID类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UserIdType {
    UserId,
    UnionId,
    OpenId,
}

impl UserIdType {
    pub fn as_str(&self) -> &'static str {
        match self {
            UserIdType::UserId => "user_id",
            UserIdType::UnionId => "union_id",
            UserIdType::OpenId => "open_id",
        }
    }
}

/// 接收ID类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReceiveIdType {
    OpenId,
    UserId,
    UnionId,
    Email,
    ChatId,
}

impl ReceiveIdType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ReceiveIdType::OpenId => "open_id",
            ReceiveIdType::UserId => "user_id",
            ReceiveIdType::UnionId => "union_id",
            ReceiveIdType::Email => "email",
            ReceiveIdType::ChatId => "chat_id",
        }
    }
}

/// 批量消息状态
#[derive(Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum BatchMessageStatus {
    /// 处理中
    Processing = 0,
    /// 部分成功
    PartialSuccess = 1,
    /// 全部成功
    Success = 2,
    /// 全部失败
    Failed = 3,
}

/// 表情回复类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmojiType {
    /// 表情类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji_type: Option<String>,
}

impl EmojiType {
    pub fn new() -> Self {
        Self { emoji_type: None }
    }

    pub fn with_emoji_type(mut self, emoji_type: impl Into<String>) -> Self {
        self.emoji_type = Some(emoji_type.into());
        self
    }
}

impl Default for EmojiType {
    fn default() -> Self {
        Self::new()
    }
}

/// 表情回复信息
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageReaction {
    /// 消息ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    /// 表情类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji_type: Option<EmojiType>,
    /// 回复次数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reaction_count: Option<i32>,
    /// 是否包含当前用户
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_reacted: Option<bool>,
    /// 回复用户列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reaction_users: Option<Vec<ReactionUser>>,
}

/// 表情回复用户信息
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReactionUser {
    /// 用户ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 用户名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 用户头像
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    /// 回复时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reaction_time: Option<String>,
}

/// Pin消息信息
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pin {
    /// Pin ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pin_id: Option<String>,
    /// 消息ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    /// 聊天ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<String>,
    /// Pin创建者
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator_id: Option<String>,
    /// Pin类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pin_type: Option<String>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
}

/// 批量消息信息
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BatchMessage {
    /// 批量消息ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_message_id: Option<String>,
    /// 消息ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    /// 状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<BatchMessageStatus>,
    /// 已发送人数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sent_count: Option<i32>,
    /// 已读人数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_count: Option<i32>,
    /// 总人数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    /// 发送进度
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<f64>,
}

/// 图片信息
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImageInfo {
    /// 图片key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_key: Option<String>,
    /// 图片类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_type: Option<String>,
    /// 图片大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_size: Option<i64>,
    /// 图片宽度
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
    /// 图片高度
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
}

/// 文件信息
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileInfo {
    /// 文件key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_key: Option<String>,
    /// 文件名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    /// 文件类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_type: Option<String>,
    /// 文件大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
    /// 文件token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_token: Option<String>,
}

/// 消息卡片信息
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageCard {
    /// 卡片ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_id: Option<String>,
    /// 卡片内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<serde_json::Value>,
    /// 是否仅特定人可见
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_multi: Option<bool>,
    /// 特定用户列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_list: Option<Vec<String>>,
}

/// 加急类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UrgentType {
    /// 应用内加急
    App,
    /// 短信加急
    Sms,
    /// 电话加急
    Phone,
}

/// 加急信息
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UrgentInfo {
    /// 加急类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urgent_type: Option<UrgentType>,
    /// 加急用户列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_list: Option<Vec<String>>,
    /// 自定义消息内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// URL预览信息
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UrlPreview {
    /// URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// 标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 图片URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    /// 预览类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_type: Option<String>,
}

/// 分页信息
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PageInfo {
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多页
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

/// 消息已读信息
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageReadInfo {
    /// 已读用户列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_users: Option<Vec<ReadUser>>,
    /// 是否还有更多
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 已读用户信息
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReadUser {
    /// 用户ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 已读时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_time: Option<String>,
    /// 租户key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_key: Option<String>,
}
