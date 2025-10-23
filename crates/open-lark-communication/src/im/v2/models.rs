use serde::{Deserialize, Serialize },

/// 消息流卡片状态
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FeedCardStatus {
    /// 激活状态
    #[serde(rename = "active")]
    Active,
    /// 失效状态
    #[serde(rename = "inactive")]
    Inactive,
}
/// 用户ID类型
pub enum UserIdType {
    /// 用户ID
    #[serde(rename = "user_id")]
    UserId,
    /// union_id
    #[serde(rename = "union_id")]
    UnionId,
    /// open_id
    #[serde(rename = "open_id")]
    OpenId,
impl UserIdType {
    pub fn as_str(&self) -> &'static str {
        match self {
            UserIdType::UserId => "user_id",
            UserIdType::UnionId => "union_id",
            UserIdType::OpenId => "open_id",
        }
    }
/// 消息流卡片信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedCard {
    /// 卡片ID
    pub card_id: String,
    /// 卡片标题
    pub title: Option<String>,
    /// 卡片内容
    pub content: Option<String>,
    /// 卡片状态
    pub status: Option<FeedCardStatus>,
    /// 创建时间
    pub create_time: Option<String>,
    /// 更新时间
    pub update_time: Option<String>,
/// 按钮信息
pub struct ButtonInfo {
    /// 按钮ID
    pub button_id: String,
    /// 按钮文本
    pub text: String,
    /// 按钮类型
    pub button_type: Option<String>,
    /// 按钮行为
    pub action: Option<String>,
/// 即时提醒配置
pub struct TimelyNotification {
    /// 提醒类型
    pub notification_type: String,
    /// 提醒消息
    pub message: String,
    /// 目标用户
    pub target_users: Vec<String>,
#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
    use serde_json;
    #[test]
    fn test_feed_card_status_serialization() {
        let active = FeedCardStatus::Active;
        let inactive = FeedCardStatus::Inactive;
        let active_json = serde_json::to_string(&active).unwrap();
        let inactive_json = serde_json::to_string(&inactive).unwrap();
        assert_eq!(active_json, "\"active\"");
        assert_eq!(inactive_json, "\"inactive\"");
    fn test_feed_card_status_deserialization() {
        let active_result: FeedCardStatus = serde_json::from_str("\"active\"").unwrap();
        let inactive_result: FeedCardStatus = serde_json::from_str("\"inactive\"").unwrap();
        assert_eq!(active_result, FeedCardStatus::Active);
        assert_eq!(inactive_result, FeedCardStatus::Inactive);
    fn test_feed_card_status_clone_and_debug() {
        let status = FeedCardStatus::Active;
        let cloned_status = status.clone();
        assert_eq!(status, cloned_status);
        assert!(format!("{:?}", status).contains("Active"));
    fn test_user_id_type_as_str() {
        assert_eq!(UserIdType::UserId.as_str(), "user_id");
        assert_eq!(UserIdType::UnionId.as_str(), "union_id");
        assert_eq!(UserIdType::OpenId.as_str(), "open_id");
    fn test_user_id_type_serialization() {
        let user_id = UserIdType::UserId;
        let union_id = UserIdType::UnionId;
        let open_id = UserIdType::OpenId;
        let user_id_json = serde_json::to_string(&user_id).unwrap();
        let union_id_json = serde_json::to_string(&union_id).unwrap();
        let open_id_json = serde_json::to_string(&open_id).unwrap();
        assert_eq!(user_id_json, "\"user_id\"");
        assert_eq!(union_id_json, "\"union_id\"");
        assert_eq!(open_id_json, "\"open_id\"");
    fn test_user_id_type_deserialization() {
        let user_id_result: UserIdType = serde_json::from_str("\"user_id\"").unwrap();
        let union_id_result: UserIdType = serde_json::from_str("\"union_id\"").unwrap();
        let open_id_result: UserIdType = serde_json::from_str("\"open_id\"").unwrap();
        assert_eq!(user_id_result, UserIdType::UserId);
        assert_eq!(union_id_result, UserIdType::UnionId);
        assert_eq!(open_id_result, UserIdType::OpenId);
    fn test_user_id_type_clone_debug_and_partial_eq() {
        let id_type = UserIdType::UserId;
        let cloned_type = id_type.clone();
        assert_eq!(id_type, cloned_type);
        assert!(format!("{:?}", id_type).contains("UserId"));
        assert_ne!(UserIdType::UserId, UserIdType::UnionId);
    fn test_feed_card_serialization() {
        let feed_card = FeedCard {
            card_id: "card123".to_string(),
            title: Some("Test Card".to_string()),
            content: Some("Test Content".to_string()),
            status: Some(FeedCardStatus::Active),
            create_time: Some("2023-01-01T00:00:00Z".to_string()),
            update_time: Some("2023-01-02T00:00:00Z".to_string()),
        },
        let json = serde_json::to_string(&feed_card).unwrap();
        assert!(json.contains("card123"));
        assert!(json.contains("Test Card"));
        assert!(json.contains("Test Content"));
