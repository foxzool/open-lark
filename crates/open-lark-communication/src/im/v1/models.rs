use serde::{Deserialize, Serialize },
use serde_repr::{Deserialize_repr, Serialize_repr },

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
pub enum UserIdType {
    UserId,
    UnionId,
    OpenId,
impl UserIdType {
    pub fn as_str(&self) -> &'static str {
        match self {
            UserIdType::UserId => "user_id",
            UserIdType::UnionId => "union_id",
            UserIdType::OpenId => "open_id",
        }
    }
/// 接收ID类型
pub enum ReceiveIdType {
    Email,
    ChatId,
impl ReceiveIdType {
            ReceiveIdType::OpenId => "open_id",
            ReceiveIdType::UserId => "user_id",
            ReceiveIdType::UnionId => "union_id",
            ReceiveIdType::Email => "email",
            ReceiveIdType::ChatId => "chat_id",
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
/// 表情回复类型
pub struct EmojiType {
    /// 表情类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji_type: Option<String>,
impl EmojiType {
    pub fn new() -> Self {
        Self { emoji_type: None }
    pub fn with_emoji_type(mut self, emoji_type: impl Into<String>) -> Self {
        self.emoji_type = Some(emoji_type.into());
        self
impl Default for EmojiType {
    fn default() -> Self {
        Self::new()
/// 表情回复信息
pub struct MessageReaction {
    /// 消息ID
    pub message_id: Option<String>,
    pub emoji_type: Option<EmojiType>,
    /// 回复次数
    pub reaction_count: Option<i32>,
    /// 是否包含当前用户
    pub has_reacted: Option<bool>,
    /// 回复用户列表
    pub reaction_users: Option<Vec<ReactionUser>>,
/// 表情回复用户信息
pub struct ReactionUser {
    /// 用户ID
    pub user_id: Option<String>,
    /// 用户名称
    pub name: Option<String>,
    /// 用户头像
    pub avatar: Option<String>,
    /// 回复时间
    pub reaction_time: Option<String>,
/// Pin消息信息
pub struct Pin {
    /// Pin ID
    pub pin_id: Option<String>,
    /// 聊天ID
    pub chat_id: Option<String>,
    /// Pin创建者
    pub operator_id: Option<String>,
    /// Pin类型
    pub pin_type: Option<String>,
    /// 创建时间
    pub create_time: Option<String>,
/// 批量消息信息
pub struct BatchMessage {
    /// 批量消息ID
    pub batch_message_id: Option<String>,
    /// 状态
    pub status: Option<BatchMessageStatus>,
    /// 已发送人数
    pub sent_count: Option<i32>,
    /// 已读人数
    pub read_count: Option<i32>,
    /// 总人数
    pub total_count: Option<i32>,
    /// 发送进度
    pub progress: Option<f64>,
/// 图片信息
pub struct ImageInfo {
    /// 图片key
    pub image_key: Option<String>,
    /// 图片类型
    pub image_type: Option<String>,
    /// 图片大小
    pub image_size: Option<i64>,
    /// 图片宽度
    pub width: Option<i32>,
    /// 图片高度
    pub height: Option<i32>,
/// 文件信息
pub struct FileInfo {
    /// 文件key
    pub file_key: Option<String>,
    /// 文件名
    pub file_name: Option<String>,
    /// 文件类型
    pub file_type: Option<String>,
    /// 文件大小
    pub file_size: Option<i64>,
    /// 文件token
    pub file_token: Option<String>,
/// 消息卡片信息
pub struct MessageCard {
    /// 卡片ID
    pub card_id: Option<String>,
    /// 卡片内容
    pub card: Option<serde_json::Value>,
    /// 是否仅特定人可见
    pub update_multi: Option<bool>,
    /// 特定用户列表
    pub user_id_list: Option<Vec<String>>,
/// 加急类型
pub enum UrgentType {
    /// 应用内加急
    App,
    /// 短信加急
    Sms,
    /// 电话加急
    Phone,
/// 加急信息
pub struct UrgentInfo {
    /// 加急类型
    pub urgent_type: Option<UrgentType>,
    /// 加急用户列表
    /// 自定义消息内容
    pub message: Option<String>,
/// URL预览信息
pub struct UrlPreview {
    /// URL
    pub url: Option<String>,
    /// 标题
    pub title: Option<String>,
    /// 描述
    pub description: Option<String>,
    /// 图片URL
    pub image_url: Option<String>,
    /// 预览类型
    pub preview_type: Option<String>,
/// 分页信息
pub struct PageInfo {
    /// 分页标记
    pub page_token: Option<String>,
    /// 是否还有更多页
    pub has_more: Option<bool>,
/// 消息已读信息
pub struct MessageReadInfo {
    /// 已读用户列表
    pub read_users: Option<Vec<ReadUser>>,
    /// 是否还有更多
/// 已读用户信息
pub struct ReadUser {
    /// 已读时间
    pub read_time: Option<String>,
    /// 租户key
    pub tenant_key: Option<String>,
#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
    #[test]
    fn test_message_type_serialization() {
        let types = vec![
            MessageType::Text,
            MessageType::Post,
            MessageType::Image,
            MessageType::File,
            MessageType::Audio,
            MessageType::Media,
            MessageType::Sticker,
            MessageType::Interactive,
            MessageType::ShareChat,
            MessageType::ShareUser,
            MessageType::System,
        ];
        for msg_type in types {
            let serialized = serde_json::to_string(&msg_type).unwrap();
            let deserialized: MessageType = serde_json::from_str(&serialized).unwrap();
            assert_eq!(msg_type, deserialized);
    fn test_user_id_type_serialization() {
        let types = vec![UserIdType::UserId, UserIdType::UnionId, UserIdType::OpenId];
        for user_type in types {
            let serialized = serde_json::to_string(&user_type).unwrap();
            let deserialized: UserIdType = serde_json::from_str(&serialized).unwrap();
            assert_eq!(user_type, deserialized);
    fn test_user_id_type_as_str() {
        assert_eq!(UserIdType::UserId.as_str(), "user_id");
        assert_eq!(UserIdType::UnionId.as_str(), "union_id");
        assert_eq!(UserIdType::OpenId.as_str(), "open_id");
    fn test_receive_id_type_serialization() {
            ReceiveIdType::OpenId,
            ReceiveIdType::UserId,
            ReceiveIdType::UnionId,
            ReceiveIdType::Email,
            ReceiveIdType::ChatId,
        for receive_type in types {
            let serialized = serde_json::to_string(&receive_type).unwrap();
            let deserialized: ReceiveIdType = serde_json::from_str(&serialized).unwrap();
            assert_eq!(receive_type, deserialized);
    fn test_receive_id_type_as_str() {
        assert_eq!(ReceiveIdType::OpenId.as_str(), "open_id");
        assert_eq!(ReceiveIdType::UserId.as_str(), "user_id");
        assert_eq!(ReceiveIdType::UnionId.as_str(), "union_id");
        assert_eq!(ReceiveIdType::Email.as_str(), "email");
        assert_eq!(ReceiveIdType::ChatId.as_str(), "chat_id");
    fn test_batch_message_status_serialization() {
        let statuses = vec![
            BatchMessageStatus::Processing,
            BatchMessageStatus::PartialSuccess,
            BatchMessageStatus::Success,
            BatchMessageStatus::Failed,
        for status in statuses {
            let serialized = serde_json::to_string(&status).unwrap();
            let deserialized: BatchMessageStatus = serde_json::from_str(&serialized).unwrap();
            assert_eq!(status, deserialized);
    fn test_batch_message_status_values() {
        assert_eq!(BatchMessageStatus::Processing as u8, 0);
        assert_eq!(BatchMessageStatus::PartialSuccess as u8, 1);
        assert_eq!(BatchMessageStatus::Success as u8, 2);
        assert_eq!(BatchMessageStatus::Failed as u8, 3);
    fn test_emoji_type_creation() {
        let emoji = EmojiType::new();
        assert_eq!(emoji.emoji_type, None);
        let emoji_with_type = EmojiType::new().with_emoji_type("smile");
        assert_eq!(emoji_with_type.emoji_type, Some("smile".to_string()));
    fn test_emoji_type_default() {
        let emoji: EmojiType = Default::default();
    fn test_emoji_type_serialization() {
        let emoji = EmojiType {
            emoji_type: Some("thumbs_up".to_string()),
        },
        let serialized = serde_json::to_string(&emoji).unwrap();
        let deserialized: EmojiType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(emoji.emoji_type, deserialized.emoji_type);
    fn test_message_reaction_serialization() {
        let reaction = MessageReaction {
            message_id: Some("msg_123".to_string()),
            emoji_type: Some(EmojiType {
                emoji_type: Some("heart".to_string()),
            }),
            reaction_count: Some(5),
            has_reacted: Some(true),
            reaction_users: Some(vec![ReactionUser {
                user_id: Some("user_456".to_string()),
                name: Some("张三".to_string()),
                avatar: Some("avatar_url".to_string()),
                reaction_time: Some("2024-01-01T10:00:00Z".to_string()),
            }]),
        let serialized = serde_json::to_string(&reaction).unwrap();
        let deserialized: MessageReaction = serde_json::from_str(&serialized).unwrap();
        assert_eq!(reaction.message_id, deserialized.message_id);
        assert_eq!(reaction.reaction_count, deserialized.reaction_count);
        assert_eq!(reaction.has_reacted, deserialized.has_reacted);
        assert_eq!(
            reaction.reaction_users.as_ref().unwrap().len(),
            deserialized.reaction_users.as_ref().unwrap().len()
        );
    fn test_reaction_user_serialization() {
        let user = ReactionUser {
            user_id: Some("user_789".to_string()),
            name: Some("李四".to_string()),
            avatar: Some("https://avatar.example.com/789".to_string()),
            reaction_time: Some("2024-01-01T11:30:00Z".to_string()),
        let serialized = serde_json::to_string(&user).unwrap();
        let deserialized: ReactionUser = serde_json::from_str(&serialized).unwrap();
        assert_eq!(user.user_id, deserialized.user_id);
        assert_eq!(user.name, deserialized.name);
        assert_eq!(user.avatar, deserialized.avatar);
        assert_eq!(user.reaction_time, deserialized.reaction_time);
    fn test_pin_serialization() {
        let pin = Pin {
            pin_id: Some("pin_123".to_string()),
            message_id: Some("msg_456".to_string()),
            chat_id: Some("chat_789".to_string()),
            operator_id: Some("operator_101".to_string()),
            pin_type: Some("manual".to_string()),
            create_time: Some("2024-01-01T09:00:00Z".to_string()),
        let serialized = serde_json::to_string(&pin).unwrap();
        let deserialized: Pin = serde_json::from_str(&serialized).unwrap();
        assert_eq!(pin.pin_id, deserialized.pin_id);
        assert_eq!(pin.message_id, deserialized.message_id);
        assert_eq!(pin.chat_id, deserialized.chat_id);
        assert_eq!(pin.operator_id, deserialized.operator_id);
        assert_eq!(pin.pin_type, deserialized.pin_type);
        assert_eq!(pin.create_time, deserialized.create_time);
    fn test_batch_message_serialization() {
        let batch_msg = BatchMessage {
            batch_message_id: Some("batch_123".to_string()),
            status: Some(BatchMessageStatus::PartialSuccess),
            sent_count: Some(80),
            read_count: Some(65),
            total_count: Some(100),
            progress: Some(0.8),
        let serialized = serde_json::to_string(&batch_msg).unwrap();
        let deserialized: BatchMessage = serde_json::from_str(&serialized).unwrap();
        assert_eq!(batch_msg.batch_message_id, deserialized.batch_message_id);
        assert_eq!(batch_msg.message_id, deserialized.message_id);
        assert_eq!(batch_msg.status, deserialized.status);
        assert_eq!(batch_msg.sent_count, deserialized.sent_count);
        assert_eq!(batch_msg.read_count, deserialized.read_count);
        assert_eq!(batch_msg.total_count, deserialized.total_count);
        assert_eq!(batch_msg.progress, deserialized.progress);
    fn test_image_info_serialization() {
        let image = ImageInfo {
            image_key: Some("img_key_123".to_string()),
            image_type: Some("png".to_string()),
            image_size: Some(1024000),
            width: Some(1920),
            height: Some(1080),
        let serialized = serde_json::to_string(&image).unwrap();
        let deserialized: ImageInfo = serde_json::from_str(&serialized).unwrap();
        assert_eq!(image.image_key, deserialized.image_key);
        assert_eq!(image.image_type, deserialized.image_type);
        assert_eq!(image.image_size, deserialized.image_size);
        assert_eq!(image.width, deserialized.width);
        assert_eq!(image.height, deserialized.height);
    fn test_file_info_serialization() {
        let file = FileInfo {
            file_key: Some("file_key_456".to_string()),
            file_name: Some("document.pdf".to_string()),
            file_type: Some("pdf".to_string()),
            file_size: Some(2048000),
            file_token: Some("token_789".to_string()),
        let serialized = serde_json::to_string(&file).unwrap();
        let deserialized: FileInfo = serde_json::from_str(&serialized).unwrap();
        assert_eq!(file.file_key, deserialized.file_key);
        assert_eq!(file.file_name, deserialized.file_name);
        assert_eq!(file.file_type, deserialized.file_type);
        assert_eq!(file.file_size, deserialized.file_size);
        assert_eq!(file.file_token, deserialized.file_token);
    fn test_message_card_serialization() {
        let card_content = serde_json::json!({
            "type": "template",
            "data": {
                "template_id": "ctp_123",
                "template_variable": {
                    "title": "测试卡片",
                    "content": "这是一个测试消息卡片"
                }
            }
        });
