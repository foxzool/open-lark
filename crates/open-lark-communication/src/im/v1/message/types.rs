use super::MessageService;
use open_lark_core::core::api_resp::ApiResponseTrait;
use serde::{Deserialize, Serialize};

/// 发送消息的通用trait
///
/// 所有可以作为消息内容发送的类型都应该实现此trait。
/// 它定义了获取消息类型和内容的标准接口。
///
/// # 实现
///
/// - `MessageText`: 文本消息
/// - `MessagePost`: 富文本消息  
/// - `MessageImage`: 图片消息
/// - `MessageCardTemplate`: 卡片模板消息
///
/// # 示例
///
/// ```rust
/// use open_lark::crate::im::v1::message::{MessageText, SendMessageTrait};
///
/// let text_msg = MessageText::new("Hello, World!");
/// assert_eq!(text_msg.msg_type(), "text");
/// assert_eq!(text_msg.content(), "{\"text\":\"Hello, World!\"}");
/// ```
pub trait SendMessageTrait {
    /// 获取消息类型
    ///
    /// 返回消息的类型标识，如 "text"、"post"、"image" 等
    fn msg_type(&self) -> String;

    /// 获取消息内容
    ///
    /// 返回序列化后的消息内容JSON字符串
    fn content(&self) -> String;
}

/// Response data for message creation
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateMessageResp {
    pub data: Message,
}

impl open_lark_core::core::api_resp::ApiResponseTrait for CreateMessageResp {
    fn data_format() -> open_lark_core::core::api_resp::ResponseFormat {
        open_lark_core::core::api_resp::ResponseFormat::Data
    }
}

/// 飞书消息
///
/// 表示一条完整的飞书消息，包含消息ID、类型、内容、发送者等所有信息。
///
/// # 字段说明
///
/// - `message_id`: 消息的唯一标识符
/// - `msg_type`: 消息类型（text、post、image等）
/// - `sender`: 消息发送者信息
/// - `body`: 消息具体内容
/// - `chat_id`: 所属会话ID
/// - `create_time`/`update_time`: 创建和更新时间戳
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Message {
    /// 消息id
    pub message_id: String,
    /// 根消息id，用于回复消息场景
    pub root_id: Option<String>,
    /// 父消息的id，用于回复消息场景
    pub parent_id: Option<String>,
    /// 消息所属的话题 ID（不返回说明该消息非话题消息）
    pub thread_id: Option<String>,
    /// 消息类型 包括：text、post、image、file、audio、media、sticker、interactive、share_chat、
    /// share_user等
    pub msg_type: String,
    /// 消息生成的时间戳（毫秒）
    pub create_time: String,
    /// 消息更新的时间戳（毫秒）
    pub update_time: String,
    /// 消息是否被撤回
    pub deleted: bool,
    /// 消息是否被更新
    pub updated: bool,
    /// 所属的群
    pub chat_id: String,
    /// 发送者，可以是用户或应用
    pub sender: Sender,
    /// 消息内容
    pub body: MessageBody,
    /// 被@的用户或机器人的id列表
    pub mentions: Option<Vec<Mention>>,
}

impl ApiResponseTrait for Message {
    fn data_format() -> open_lark_core::core::api_resp::ResponseFormat {
        open_lark_core::core::api_resp::ResponseFormat::Data
    }
}

/// 发送者，可以是用户或应用
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sender {
    /// 该字段标识发送者的id
    id: String,
    /// 该字段标识发送者的id类型
    ///
    /// 可选值有：
    /// - open_id
    /// - app_id
    id_type: String,
    /// 该字段标识发送者的类型
    ///
    /// 可选值有：
    /// - user: 用户
    /// - app: 应用
    /// - anonymous: 匿名
    /// - unknown: 未知
    sender_type: String,
    /// 为租户在飞书上的唯一标识，用来换取对应的tenant_access_token，
    /// 也可以用作租户在应用里面的唯一标识
    tenant_key: String,
}

/// 消息内容
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageBody {
    /// 消息内容，json结构序列化后的字符串。不同msg_type对应不同内容。
    ///
    /// 消息类型 包括：text、post、image、file、audio、media、sticker、interactive、share_chat、
    /// share_user等，
    pub content: String,
}

/// 被@的用户或机器人的id列表
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Mention {
    /// 被@的用户或机器人的序号。例如，第3个被@到的成员，值为"@_user_3"
    pub key: String,
    /// 被@的用户或者机器人的open_id
    pub id: String,
    /// 被@的用户或机器人 id 类型，目前仅支持 open_id
    pub id_type: String,
    /// 被@的用户或机器人的姓名
    pub name: String,
    /// 为租户在飞书上的唯一标识，用来换取对应的tenant_access_token，
    /// 也可以用作租户在应用里面的唯一标识
    pub tenant_key: String,
    /// 合并转发消息中，上一层级的消息id message_id
    pub upper_message_id: String,
}

/// Response data for message listing
#[derive(Debug, Serialize, Deserialize)]
pub struct ListMessageRespData {
    /// 是否还有更多项
    pub has_more: bool,
    /// 分页标记，当 has_more 为 true 时，会同时返回新的 page_token，否则不返回 page_token
    pub page_token: Option<String>,
    pub items: Vec<Message>,
}

impl ApiResponseTrait for ListMessageRespData {
    fn data_format() -> open_lark_core::core::api_resp::ResponseFormat {
        open_lark_core::core::api_resp::ResponseFormat::Data
    }
}

/// Message iterator for listing messages with pagination
#[allow(dead_code)]
pub struct ListMessageIterator<'a> {
    service: &'a super::MessageService,
    request: crate::im::v1::message::list::ListMessageRequest,
    page_token: Option<String>,
    has_more: bool,
}

impl<'a> ListMessageIterator<'a> {
    pub fn new(
        service: &'a MessageService,
        request: crate::im::v1::message::list::ListMessageRequest,
    ) -> Self {
        Self {
            service,
            request,
            page_token: None,
            has_more: true,
        }
    }

    // FUTURE: 实现异步迭代器或流式处理分页结果
    // 标准 Iterator trait 不支持异步，可考虑：
    // 1. 使用 futures::Stream trait
    // 2. 使用 async-stream crate
    // 3. 实现自定义的 AsyncIterator trait
}

impl<'a> std::fmt::Debug for ListMessageIterator<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ListMessageIterator")
            .field("has_more", &self.has_more)
            .field("page_token", &self.page_token)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    // 测试用的 SendMessageTrait 实现
    #[derive(Debug, Clone)]
    struct TestMessage {
        msg_type_value: String,
        content_value: String,
    }

    impl TestMessage {
        fn new(msg_type: impl ToString, content: impl ToString) -> Self {
            Self {
                msg_type_value: msg_type.to_string(),
                content_value: content.to_string(),
            }
        }
    }

    impl SendMessageTrait for TestMessage {
        fn msg_type(&self) -> String {
            self.msg_type_value.clone()
        }

        fn content(&self) -> String {
            self.content_value.clone()
        }
    }

    #[test]
    fn test_send_message_trait_basic() {
        let msg = TestMessage::new("text", "Hello World");
        assert_eq!(msg.msg_type(), "text");
        assert_eq!(msg.content(), "Hello World");
    }

    #[test]
    fn test_send_message_trait_empty_content() {
        let msg = TestMessage::new("text", "");
        assert_eq!(msg.msg_type(), "text");
        assert_eq!(msg.content(), "");
    }

    #[test]
    fn test_send_message_trait_unicode() {
        let msg = TestMessage::new("text", "你好，世界！🌍");
        assert_eq!(msg.msg_type(), "text");
        assert_eq!(msg.content(), "你好，世界！🌍");
    }

    #[test]
    fn test_send_message_trait_complex_content() {
        let complex_content =
            r#"{"text":{"type":"plain","content":"复杂消息内容\n换行符\"引号\""}}"#;
        let msg = TestMessage::new("post", complex_content);
        assert_eq!(msg.msg_type(), "post");
        assert_eq!(msg.content(), complex_content);
    }

    #[test]
    fn test_create_message_resp_serialization() {
        let message = create_test_message();
        let resp = CreateMessageResp {
            data: message.clone(),
        };

        let json = serde_json::to_string(&resp).unwrap();
        let parsed: CreateMessageResp = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.data.message_id, message.message_id);
        assert_eq!(parsed.data.msg_type, message.msg_type);
    }

    #[test]
    fn test_create_message_resp_deserialization() {
        let json = r#"{
            "data": {
                "message_id": "msg_test123",
                "msg_type": "text",
                "create_time": "1640995200000",
                "update_time": "1640995200000",
                "deleted": false,
                "updated": false,
                "chat_id": "chat_test456",
                "sender": {
                    "id": "user_test789",
                    "id_type": "open_id",
                    "sender_type": "user",
                    "tenant_key": "tenant_test"
                },
                "body": {
                    "content": "{\"text\":\"Hello World\"}"
                }
            }
        }"#;

        let resp: CreateMessageResp = serde_json::from_str(json).unwrap();
        assert_eq!(resp.data.message_id, "msg_test123");
        assert_eq!(resp.data.msg_type, "text");
        assert_eq!(resp.data.body.content, "{\"text\":\"Hello World\"}");
    }

    #[test]
    fn test_message_serialization_roundtrip() {
        let message = create_test_message();
        let json = serde_json::to_string(&message).unwrap();
        let parsed: Message = serde_json::from_str(&json).unwrap();

        assert_eq!(message.message_id, parsed.message_id);
        assert_eq!(message.msg_type, parsed.msg_type);
        assert_eq!(message.chat_id, parsed.chat_id);
    }

    #[test]
    fn test_message_with_optional_fields() {
        let json = r#"{
            "message_id": "msg_opt_test",
            "root_id": "root_123",
            "parent_id": "parent_456",
            "thread_id": "thread_789",
            "msg_type": "interactive",
            "create_time": "1640995200000",
            "update_time": "1640995200000",
            "deleted": false,
            "updated": true,
            "chat_id": "chat_interactive",
            "sender": {
                "id": "app_123",
                "id_type": "app_id",
                "sender_type": "app",
                "tenant_key": "tenant_workplace"
            },
            "body": {
                "content": "{\"type\":\"card\",\"elements\":[]}"
            },
            "mentions": [
                {
                    "key": "@_user_1",
                    "id": "user_mentioned",
                    "id_type": "open_id",
                    "name": "张三",
                    "tenant_key": "tenant_workplace",
                    "upper_message_id": "msg_upper_123"
                }
            ]
        }"#;

        let message: Message = serde_json::from_str(json).unwrap();

        assert_eq!(message.message_id, "msg_opt_test");
        assert_eq!(message.root_id, Some("root_123".to_string()));
        assert_eq!(message.parent_id, Some("parent_456".to_string()));
        assert_eq!(message.thread_id, Some("thread_789".to_string()));
        assert!(message.updated);
        assert!(message.mentions.is_some());

        let mentions = message.mentions.unwrap();
        assert_eq!(mentions.len(), 1);
        assert_eq!(mentions[0].key, "@_user_1");
        assert_eq!(mentions[0].name, "张三");
    }

    #[test]
    fn test_message_without_optional_fields() {
        let json = r#"{
            "message_id": "msg_minimal",
            "msg_type": "text",
            "create_time": "1640995200000",
            "update_time": "1640995200000",
            "deleted": false,
            "updated": false,
            "chat_id": "chat_minimal",
            "sender": {
                "id": "user_minimal",
                "id_type": "open_id",
                "sender_type": "user",
                "tenant_key": "tenant_minimal"
            },
            "body": {
                "content": "{\"text\":\"minimal\"}"
            }
        }"#;

        let message: Message = serde_json::from_str(json).unwrap();

        assert_eq!(message.message_id, "msg_minimal");
        assert_eq!(message.root_id, None);
        assert_eq!(message.parent_id, None);
        assert_eq!(message.thread_id, None);
        assert!(message.mentions.is_none());
    }

    #[test]
    fn test_sender_different_types() {
        // User sender
        let user_json = r#"{
            "id": "user_123",
            "id_type": "open_id",
            "sender_type": "user",
            "tenant_key": "tenant_user"
        }"#;
        let user_sender: Sender = serde_json::from_str(user_json).unwrap();
        assert_eq!(user_sender.sender_type, "user");

        // App sender
        let app_json = r#"{
            "id": "app_456",
            "id_type": "app_id",
            "sender_type": "app",
            "tenant_key": "tenant_app"
        }"#;
        let app_sender: Sender = serde_json::from_str(app_json).unwrap();
        assert_eq!(app_sender.sender_type, "app");

        // Anonymous sender
        let anon_json = r#"{
            "id": "anon_789",
            "id_type": "open_id",
            "sender_type": "anonymous",
            "tenant_key": "tenant_anon"
        }"#;
        let anon_sender: Sender = serde_json::from_str(anon_json).unwrap();
        assert_eq!(anon_sender.sender_type, "anonymous");
    }

    #[test]
    fn test_message_body_complex_content() {
        let complex_content = r#"{
            "content": "{\"type\":\"template\",\"data\":{\"template_id\":\"tpl_123\",\"template_variable\":{\"name\":\"张三\",\"title\":\"欢迎\"}}}"
        }"#;

        let body: MessageBody = serde_json::from_str(complex_content).unwrap();
        assert!(body.content.contains("张三"));
        assert!(body.content.contains("template_id"));
    }

    #[test]
    fn test_message_body_unicode_and_escaping() {
        let special_content = r#"{
            "content": "{\"text\":\"特殊字符：\\n\\t\\\"引号\\\"🎉表情符号\"}"
        }"#;

        let body: MessageBody = serde_json::from_str(special_content).unwrap();
        assert!(body.content.contains("特殊字符"));
        assert!(body.content.contains("🎉表情符号"));

        // Roundtrip test
        let json = serde_json::to_string(&body).unwrap();
        let parsed: MessageBody = serde_json::from_str(&json).unwrap();
        assert_eq!(body.content, parsed.content);
    }

    #[test]
    fn test_mention_complete_structure() {
        let json = r#"{
            "key": "@_user_2",
            "id": "mention_user_456",
            "id_type": "open_id",
            "name": "李四",
            "tenant_key": "tenant_mention",
            "upper_message_id": "msg_parent_789"
        }"#;

        let mention: Mention = serde_json::from_str(json).unwrap();

        assert_eq!(mention.key, "@_user_2");
        assert_eq!(mention.id, "mention_user_456");
        assert_eq!(mention.id_type, "open_id");
        assert_eq!(mention.name, "李四");
        assert_eq!(mention.tenant_key, "tenant_mention");
        assert_eq!(mention.upper_message_id, "msg_parent_789");

        // Test serialization roundtrip
        let serialized = serde_json::to_string(&mention).unwrap();
        let deserialized: Mention = serde_json::from_str(&serialized).unwrap();
        assert_eq!(mention.key, deserialized.key);
        assert_eq!(mention.name, deserialized.name);
    }

    #[test]
    fn test_list_message_resp_data_with_pagination() {
        let json = r#"{
            "has_more": true,
            "page_token": "page_token_next_123",
            "items": [
                {
                    "message_id": "msg_page_1",
                    "msg_type": "text",
                    "create_time": "1640995200000",
                    "update_time": "1640995200000",
                    "deleted": false,
                    "updated": false,
                    "chat_id": "chat_page",
                    "sender": {
                        "id": "user_page",
                        "id_type": "open_id",
                        "sender_type": "user",
                        "tenant_key": "tenant_page"
                    },
                    "body": {
                        "content": "{\"text\":\"Page 1\"}"
                    }
                }
            ]
        }"#;

        let resp: ListMessageRespData = serde_json::from_str(json).unwrap();

        assert!(resp.has_more);
        assert_eq!(resp.page_token, Some("page_token_next_123".to_string()));
        assert_eq!(resp.items.len(), 1);
        assert_eq!(resp.items[0].message_id, "msg_page_1");
    }

    #[test]
    fn test_list_message_resp_data_last_page() {
        let json = r#"{
            "has_more": false,
            "items": []
        }"#;

        let resp: ListMessageRespData = serde_json::from_str(json).unwrap();

        assert!(!resp.has_more);
        assert_eq!(resp.page_token, None);
        assert_eq!(resp.items.len(), 0);
    }

    #[test]
    fn test_list_message_iterator_creation() {
        let service = create_test_message_service();
        let request = create_test_list_request();

        let iterator = ListMessageIterator::new(&service, request);

        assert!(iterator.has_more);
        assert_eq!(iterator.page_token, None);
    }

    #[test]
    fn test_list_message_iterator_debug_format() {
        let service = create_test_message_service();
        let request = create_test_list_request();
        let iterator = ListMessageIterator::new(&service, request);

        let debug_str = format!("{:?}", iterator);
        assert!(debug_str.contains("ListMessageIterator"));
        assert!(debug_str.contains("has_more: true"));
    }

    #[test]
    fn test_api_response_trait_implementation() {
        // Test CreateMessageResp
        assert_eq!(
            CreateMessageResp::data_format(),
            open_lark_core::core::api_resp::ResponseFormat::Data
        );

        // Test Message
        assert_eq!(
            Message::data_format(),
            open_lark_core::core::api_resp::ResponseFormat::Data
        );

        // Test ListMessageRespData
        assert_eq!(
            ListMessageRespData::data_format(),
            open_lark_core::core::api_resp::ResponseFormat::Data
        );
    }

    #[test]
    fn test_message_edge_cases() {
        // Test very long message content
        let long_content = "A".repeat(10000);
        let mut message = create_test_message();
        message.body.content = format!(r#"{{"text":"{}"}}"#, long_content);

        let json = serde_json::to_string(&message).unwrap();
        let parsed: Message = serde_json::from_str(&json).unwrap();
        assert!(parsed.body.content.len() > 10000);
    }

    #[test]
    fn test_message_special_characters_in_content() {
        let special_chars = r#"{}[]<>:"/\|?*`~!@#$%^&()-_=+;'"#;
        let mut message = create_test_message();
        message.body.content = format!(r#"{{"text":"{}"}}"#, special_chars);

        let json = serde_json::to_string(&message).unwrap();
        let parsed: Message = serde_json::from_str(&json).unwrap();
        assert!(parsed.body.content.contains(special_chars));
    }

    #[test]
    fn test_send_message_trait_performance() {
        use std::time::Instant;

        let messages: Vec<TestMessage> = (0..1000)
            .map(|i| TestMessage::new("text", format!("Message {}", i)))
            .collect();

        let start = Instant::now();
        for msg in &messages {
            let _type = msg.msg_type();
            let _content = msg.content();
        }
        let duration = start.elapsed();

        // Performance assertion - should process 1000 messages quickly
        assert!(
            duration.as_millis() < 100,
            "Processing 1000 messages took too long: {:?}",
            duration
        );
    }

    #[test]
    fn test_serialization_performance() {
        use std::time::Instant;

        let messages: Vec<Message> = (0..100)
            .map(|i| {
                let mut msg = create_test_message();
                msg.message_id = format!("msg_perf_{}", i);
                msg.body.content = format!(r#"{{"text":"Performance test message {}"}}"#, i);
                msg
            })
            .collect();

        let start = Instant::now();
        let json_results: Vec<String> = messages
            .iter()
            .map(|msg| serde_json::to_string(msg).unwrap())
            .collect();
        let duration = start.elapsed();

        // Should serialize 100 messages quickly
        assert!(
            duration.as_millis() < 200,
            "Serializing 100 messages took too long: {:?}",
            duration
        );
        assert_eq!(json_results.len(), 100);
    }

    #[test]
    fn test_deserialization_edge_cases() {
        // Test with null values in optional fields
        let json_with_nulls = r#"{
            "message_id": "msg_null_test",
            "root_id": null,
            "parent_id": null,
            "thread_id": null,
            "mentions": null,
            "msg_type": "text",
            "create_time": "1640995200000",
            "update_time": "1640995200000",
            "deleted": false,
            "updated": false,
            "chat_id": "chat_null",
            "sender": {
                "id": "user_null",
                "id_type": "open_id",
                "sender_type": "user",
                "tenant_key": "tenant_null"
            },
            "body": {
                "content": "{\"text\":\"null test\"}"
            }
        }"#;

        let message: Message = serde_json::from_str(json_with_nulls).unwrap();
        assert_eq!(message.message_id, "msg_null_test");
        assert_eq!(message.root_id, None);
        assert_eq!(message.parent_id, None);
        assert_eq!(message.thread_id, None);
        assert_eq!(message.mentions, None);
    }

    #[test]
    fn test_message_id_validation() {
        let mut message = create_test_message();

        // Test valid message IDs
        let valid_ids = [
            "msg_123",
            "abc-def-ghi",
            "MSG_UPPERCASE",
            "msg_with_numbers_123",
        ];
        for valid_id in valid_ids {
            message.message_id = valid_id.to_string();
            let json = serde_json::to_string(&message).unwrap();
            let parsed: Message = serde_json::from_str(&json).unwrap();
            assert_eq!(parsed.message_id, valid_id);
        }
    }

    #[test]
    fn test_chat_id_validation() {
        let mut message = create_test_message();

        // Test valid chat IDs
        let valid_chat_ids = ["chat_123", "oc_123456789", "group_chat_id"];
        for valid_chat_id in valid_chat_ids {
            message.chat_id = valid_chat_id.to_string();
            let json = serde_json::to_string(&message).unwrap();
            let parsed: Message = serde_json::from_str(&json).unwrap();
            assert_eq!(parsed.chat_id, valid_chat_id);
        }
    }

    // Helper functions for testing
    fn create_test_message() -> Message {
        Message {
            message_id: "test_msg_123".to_string(),
            root_id: None,
            parent_id: None,
            thread_id: None,
            msg_type: "text".to_string(),
            create_time: "1640995200000".to_string(),
            update_time: "1640995200000".to_string(),
            deleted: false,
            updated: false,
            chat_id: "test_chat_456".to_string(),
            sender: Sender {
                id: "test_user_789".to_string(),
                id_type: "open_id".to_string(),
                sender_type: "user".to_string(),
                tenant_key: "test_tenant".to_string(),
            },
            body: MessageBody {
                content: r#"{"text":"Hello World"}"#.to_string(),
            },
            mentions: None,
        }
    }

    fn create_test_message_service() -> MessageService {
        use open_lark_core::core::config::Config;

        MessageService {
            config: Config::default(),
        }
    }

    fn create_test_list_request() -> crate::im::v1::message::list::ListMessageRequest {
        crate::im::v1::message::list::ListMessageRequest::builder()
            .container_id("test_chat_123")
            .container_id_type("chat")
            .page_size(20)
            .build()
    }
}
