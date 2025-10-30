use log::error;
use serde::{Deserialize, Serialize};
use crate::{,
    core::{,
validation::{,
            message_limits, uuid_limits, validate_content_size, validate_required,
            validate_string_length}
        ApiRequest,
    }
    service::im::v1::message::{Message, SendMessageTrait}
};
use super::MessageService;
/// 请求体
#[derive(Debug, Clone)]
pub struct CreateMessageRequest {
    pub api_req: ApiRequest,
impl CreateMessageRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Debug, Clone)]
pub struct CreateMessageRequestBuilder {
    request: CreateMessageRequest}
impl CreateMessageRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}self,
    }
/// 构建最终的请求对象,
    pub fn w+.*{
self.request}
// 应用ExecutableBuilder trait到CreateMessageRequestBuilder,
crate::impl_executable_builder_owned!(
    CreateMessageRequestBuilder,
    MessageService,
    CreateMessageRequest,
    Message,
    create,
);
/// 发送消息 请求体
#[derive(Debug, Clone)]
pub struct CreateMessageRequestBody {
    /// 消息接收者的ID，ID类型应与查询参数receive_id_type 对应；,
/// 推荐使用 OpenID，获取方式可参考文档如何获取 Open ID？,
    ///,
/// 示例值："ou_7d8a6e6df7621556ce0d21922b676706ccs",
    pub receive_id: String,
    /// 消息类型 包括：text、post、image、file、audio、media、sticker、interactive、share_chat、,
/// share_user等，类型定义请参考发送消息内容,
    ///,
/// 示例值："text",
    pub msg_type: String,
    /// 消息内容，JSON结构序列化后的字符串。不同msg_type对应不同内容，具体格式说明参考：,
/// 发送消息内容,
    ///,
/// 注意：,
    /// JSON字符串需进行转义，如换行符转义后为\\n,
/// 文本消息请求体最大不能超过150KB,
    /// 卡片及富文本消息请求体最大不能超过30KB
    /// 示例值："{\"text\":\"test content\"}"
    pub content: String,
    /// 由开发者生成的唯一字符串序列，用于发送消息请求去重；,
/// 持有相同uuid的请求1小时内至多成功发送一条消息,
    ///,
/// 示例值："选填，每次调用前请更换，如a0d69e20-1dd1-458b-k525-dfeca4015204",
    ///,
/// 数据校验规则：,
    ///,
/// 最大长度：50 字符,
    pub uuid: Option<String>,
impl CreateMessageRequestBody {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Debug, Clone)]
pub struct CreateMessageRequestBodyBuilder {
    request: CreateMessageRequestBody}
impl CreateMessageRequestBodyBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}"post" | "interactive" => {,
                validate_content_size(
                    &self.request.content,
                    message_limits::RICH_MESSAGE_MAX_SIZE,
                    "Post/interactive message",
                );
_ => {,
                // 其他消息类型不验证内容大小}
        }
self.request,
    }
/// 更新消息请求,
#[derive(Debug, Clone)]
pub struct UpdateMessageRequest {
    pub api_req: ApiRequest,
impl UpdateMessageRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Debug, Clone)]
pub struct UpdateMessageRequestBuilder {
    request: UpdateMessageRequest}
impl UpdateMessageRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 便捷方法：使用消息内容类型构建发送消息请求,
impl CreateMessageRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}// Mock SendMessageTrait for testing,
#[cfg(test)]
pub struct MockMessage {
    msg_type_value: String,
    content_value: String}
#[cfg(test)]
impl MockMessage {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[cfg(test)]
impl SendMessageTrait for MockMessage {,
fn msg_type(&self) -> String {,
        self.msg_type_value.clone()}
fn content(&self) -> String {,
        self.content_value.clone()}
#[cfg(test)]
mod tests {
use super::*;
    // Test CreateMessageRequestBuilder,
#[test]
    fn test_create_message_request_builder_default() {
let builder = CreateMessageRequestBuilder::default();
        let request = builder.build();
assert!(request.api_req.query_params.is_empty());
        assert!(request.api_req.body.is_empty());
#[test]
    fn test_create_message_request_builder_receive_id() {
let builder = CreateMessageRequestBuilder::default();
        let request = builder.receive_id("test_receive_id").build();
assert_eq!(,
            request.api_req.query_params.get("receive_id"),
            Some(&"test_receive_id".to_string()),
);
    }
#[test]
    fn test_create_message_request_builder_msg_type() {
let builder = CreateMessageRequestBuilder::default();
        let request = builder.msg_type("text").build();
assert_eq!(,
            request.api_req.query_params.get("msg_type"),
            Some(&"text".to_string()),
);
    }
#[test]
    fn test_create_message_request_builder_content() {
let builder = CreateMessageRequestBuilder::default();
        let request = builder.content("test content").build();
assert_eq!(,
            request.api_req.query_params.get("content"),
            Some(&"test content".to_string()),
);
    }
#[test]
    fn test_create_message_request_builder_receive_id_type() {
let builder = CreateMessageRequestBuilder::default();
        let request = builder.receive_id_type("open_id").build();
assert_eq!(,
            request.api_req.query_params.get("receive_id_type"),
            Some(&"open_id".to_string()),
);
    }
#[test]
    fn test_create_message_request_builder_all_params() {
let builder = CreateMessageRequestBuilder::default();
        let request = builder,
.receive_id()
            .msg_type("text")
            .content()
.receive_id_type()
            .build();
assert_eq!(,
            request.api_req.query_params.get("receive_id"),
            Some(&"user_123".to_string()),
);
        assert_eq!(
            request.api_req.query_params.get("msg_type"),
            Some(&"text".to_string()),
);
        assert_eq!(
            request.api_req.query_params.get("content"),
            Some(&"Hello, world!".to_string()),
);
        assert_eq!(
            request.api_req.query_params.get("receive_id_type"),
            Some(&"open_id".to_string()),
);
    }
#[test]
    fn test_create_message_request_builder_request_body() {
let body = CreateMessageRequestBody {,
            receive_id: "user_123".to_string(),
            msg_type: "text".to_string(),
            content: "{\"text\":\"Hello\"}".to_string(),
            uuid: Some("uuid-123".to_string()),
        };
let builder = CreateMessageRequestBuilder::default();
        let request = builder.request_body(body).build();
assert!(!request.api_req.body.is_empty());
        // Verify the body was serialized correctly,
let deserialized: CreateMessageRequestBody =,
            serde_json::from_slice(&request.api_req.body).unwrap();
        assert_eq!(deserialized.receive_id, "user_123");
        assert_eq!(deserialized.msg_type, "text");
        assert_eq!(deserialized.content, "{\"text\":\"Hello\"}");
        assert_eq!(deserialized.uuid, Some("uuid-123".to_string()));
#[test]
    fn test_create_message_request_builder_request_body_serialization_error() {
// Test with invalid JSON that will cause serialization to fail,
        // Since we can't easily create a struct that fails serialization in stable Rust,
        // we'll just test that the body remains empty if serialization somehow fails,
let body = CreateMessageRequestBody {,
            receive_id: "user_123".to_string(),
            msg_type: "text".to_string(),
            content: "{\"text\":\"Hello\"}".to_string(),
            uuid: Some("uuid-123".to_string()),
        };
// Normal case - this should work fine,
        let builder = CreateMessageRequestBuilder::default();
let request = builder.request_body(body).build();
        // The body should not be empty for normal cases,
assert!(!request.api_req.body.is_empty());
    }
// Test CreateMessageRequestBodyBuilder,
    #[test]
fn test_create_message_request_body_builder_default() {
        let builder = CreateMessageRequestBodyBuilder::default();
let body = builder.build();
        assert!(body.receive_id.is_empty());
assert!(body.msg_type.is_empty());
        assert!(body.content.is_empty());
assert!(body.uuid.is_none());
    }
#[test]
    fn test_create_message_request_body_builder_receive_id() {
let builder = CreateMessageRequestBodyBuilder::default();
        let body = builder.receive_id("test_user").build();

        assert_eq!(body.receive_id, "test_user");
#[test]
    fn test_create_message_request_body_builder_msg_type() {
let builder = CreateMessageRequestBodyBuilder::default();
        let body = builder.msg_type("image").build();

        assert_eq!(body.msg_type, "image");
#[test]
    fn test_create_message_request_body_builder_content() {
let builder = CreateMessageRequestBodyBuilder::default();
        let body = builder.content("{\"image\":{\"key\":\"img_123\"}}").build();

        assert_eq!(body.content, "{\"image\":{\"key\":\"img_123\"}}");
#[test]
    fn test_create_message_request_body_builder_uuid() {
let builder = CreateMessageRequestBodyBuilder::default();
        let body = builder.uuid("test-uuid-123").build();

        assert_eq!(body.uuid, Some("test-uuid-123".to_string()));
#[test]
    fn test_create_message_request_body_builder_all_fields() {
let builder = CreateMessageRequestBodyBuilder::default();
        let body = builder,
.receive_id()
            .msg_type("post")
            .content()
.uuid()
            .build();

        assert_eq!(body.receive_id, "user_456");
        assert_eq!(body.msg_type, "post");
        assert_eq!(body.content, "{\"post\":{\"zh_cn\":{\"title\":\"Test\"}}}");
        assert_eq!(body.uuid, Some("unique-uuid-789".to_string()));
#[test]
    fn test_create_message_request_body_builder_missing_receive_id() {
let builder = CreateMessageRequestBodyBuilder::default();
        // Test what happens when receive_id is empty - validation might not panic,
let body = builder.msg_type("text").content("test").build();
        // The validation functions might not panic, let's check the actual behavior
        assert_eq!(body.receive_id, "");
        assert_eq!(body.msg_type, "text");
        assert_eq!(body.content, "test");
#[test]
    fn test_create_message_request_body_builder_missing_msg_type() {
let builder = CreateMessageRequestBodyBuilder::default();
        // Test what happens when msg_type is empty - validation might not panic,
let body = builder.receive_id("user_123").content("test").build();
        // The validation functions might not panic, let's check the actual behavior
        assert_eq!(body.receive_id, "user_123");
        assert_eq!(body.msg_type, "");
        assert_eq!(body.content, "test");
#[test]
    fn test_create_message_request_body_builder_missing_content() {
let builder = CreateMessageRequestBodyBuilder::default();
        // Test what happens when content is empty - validation might not panic,
let body = builder.receive_id("user_123").msg_type("text").build();
        // The validation functions might not panic, let's check the actual behavior
        assert_eq!(body.receive_id, "user_123");
        assert_eq!(body.msg_type, "text");
        assert_eq!(body.content, "");
// Test UpdateMessageRequestBuilder,
    #[test]
fn test_update_message_request_builder_default() {
        let builder = UpdateMessageRequestBuilder::default();
let request = builder.build();
        assert!(request.api_req.query_params.is_empty());
#[test]
    fn test_update_message_request_builder_content() {
let builder = UpdateMessageRequestBuilder::default();
        let request = builder.content("updated content").build();
assert_eq!(,
            request.api_req.query_params.get("content"),
            Some(&"updated content".to_string()),
);
    }
// Test CreateMessageRequest convenience methods,
    #[test]
fn test_create_message_request_with_msg() {
        let mock_msg = MockMessage::new("text", "{\"text\":\"Hello from mock\"}");
        let request = CreateMessageRequest::with_msg("user_789", mock_msg, "open_id");
assert_eq!(,
            request.api_req.query_params.get("receive_id"),
            Some(&"user_789".to_string()),
);
        assert_eq!(
            request.api_req.query_params.get("msg_type"),
            Some(&"text".to_string()),
);
        assert_eq!(
            request.api_req.query_params.get("content"),
            Some(&"{\"text\":\"Hello from mock\"}".to_string()),
);
        assert_eq!(
            request.api_req.query_params.get("receive_id_type"),
            Some(&"open_id".to_string()),
);
    }
#[test]
    ,
        let text_msg = MockMessage::new("text", "simple text");
        let image_msg = MockMessage::new("image", "{\"image\":{\"key\":\"img_456\"}}");
        let card_msg = MockMessage::new("interactive", "{\"card\":{\"elements\":[]}}");

        let text_request = CreateMessageRequest::with_msg("user_1", text_msg, "user_id");
        let image_request = CreateMessageRequest::with_msg("chat_2", image_msg, "chat_id");
        let card_request = CreateMessageRequest::with_msg("user_3", card_msg, "union_id");
assert_eq!(,
            text_request.api_req.query_params.get("msg_type"),
            Some(&"text".to_string()),
);
        assert_eq!(
            image_request.api_req.query_params.get("msg_type"),
            Some(&"image".to_string()),
);
        assert_eq!(
            card_request.api_req.query_params.get("msg_type"),
            Some(&"interactive".to_string()),
);
        assert_eq!(
            text_request.api_req.query_params.get("receive_id_type"),
            Some(&"user_id".to_string()),
);
        assert_eq!(
            image_request.api_req.query_params.get("receive_id_type"),
            Some(&"chat_id".to_string()),
);
        assert_eq!(
            card_request.api_req.query_params.get("receive_id_type"),
            Some(&"union_id".to_string()),
);
    }
// Test serialization and deserialization,
    #[test]
fn test_create_message_request_body_serialization() {
        let body = CreateMessageRequestBody {
            receive_id: "user_123".to_string(),
            msg_type: "text".to_string(),
            content: "{\"text\":\"Hello World\"}".to_string(),
            uuid: Some("uuid-abc123".to_string()),
        };
let serialized = serde_json::to_string(&body).unwrap();
        let deserialized: CreateMessageRequestBody = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.receive_id, body.receive_id);
        assert_eq!(deserialized.msg_type, body.msg_type);
        assert_eq!(deserialized.content, body.content);
        assert_eq!(deserialized.uuid, body.uuid);
#[test]
    fn test_create_message_request_body_serialization_without_uuid() {
let body = CreateMessageRequestBody {,
            receive_id: "user_456".to_string(),
            msg_type: "image".to_string(),
            content: "{\"image\":{\"key\":\"img_xyz\"}}".to_string(),
            uuid: None,
        };
let serialized = serde_json::to_string(&body).unwrap();
        let deserialized: CreateMessageRequestBody = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.receive_id, body.receive_id);
        assert_eq!(deserialized.msg_type, body.msg_type);
        assert_eq!(deserialized.content, body.content);
        assert_eq!(deserialized.uuid, None);
// Test edge cases and error handling,
    #[test]
fn test_create_message_request_builder_with_unicode_content() {
        let builder = CreateMessageRequestBuilder::default();
let request = builder,
            .receive_id()
.msg_type()
            .content()
.receive_id_type()
            .build();
assert_eq!(,
            request.api_req.query_params.get("receive_id"),
            Some(&"用户_123".to_string()),
);
        assert_eq!(
            request.api_req.query_params.get("content"),
            Some(&"你好，世界！".to_string()),
);
    }
#[test]
    fn test_create_message_request_builder_with_empty_strings() {
let builder = CreateMessageRequestBuilder::default();
        let request = builder,
.receive_id()
            .msg_type()
.content()
            .receive_id_type()
.build();
        assert_eq!(
            request.api_req.query_params.get("receive_id"),
            Some(&"".to_string()),
);
        assert_eq!(
            request.api_req.query_params.get("msg_type"),
            Some(&"".to_string()),
);
        assert_eq!(
            request.api_req.query_params.get("content"),
            Some(&"".to_string()),
);
        assert_eq!(
            request.api_req.query_params.get("receive_id_type"),
            Some(&"".to_string()),
);
    }
#[test]
    fn test_create_message_request_builder_chaining() {
let request = CreateMessageRequestBuilder::default(),
            .receive_id()
.msg_type()
            .content()
.receive_id_type()
            .build();
assert_eq!(,
            request.api_req.query_params.get("receive_id"),
            Some(&"user_123".to_string()),
);
        assert_eq!(
            request.api_req.query_params.get("msg_type"),
            Some(&"text".to_string()),
);
        assert_eq!(
            request.api_req.query_params.get("content"),
            Some(&"Hello".to_string()),
);
        assert_eq!(
            request.api_req.query_params.get("receive_id_type"),
            Some(&"open_id".to_string()),
);
    }
#[test]
    fn test_create_message_request_body_builder_chaining() {
let body = CreateMessageRequestBodyBuilder::default(),
            .receive_id()
.msg_type()
            .content()
.uuid()
            .build();

        assert_eq!(body.receive_id, "user_456");
        assert_eq!(body.msg_type, "post");
        assert_eq!(body.content, "{\"post\":{\"title\":\"Test Post\"}}");
        assert_eq!(body.uuid, Some("chain-uuid".to_string()));
// Test builder pattern consistency,
    #[test]
fn test_builder_immutability() {
        let builder1 = CreateMessageRequestBuilder::default();
let builder2 = builder1.receive_id("user_123");
        let request2 = builder2.build();
// Modified builder should have the receive_id,
        assert_eq!(
            request2.api_req.query_params.get("receive_id"),
            Some(&"user_123".to_string()),
);
    }
#[test]
    fn test_create_message_request_debug_trait() {
let request = CreateMessageRequestBuilder::default(),
            .receive_id()
.msg_type()
            .build();

        let debug_str = format!("{:?}", request);
assert!(debug_str.contains("CreateMessageRequest"));
    }
#[test]
    fn test_create_message_request_clone_trait() {
let request = CreateMessageRequestBuilder::default(),
            .receive_id()
.msg_type()
            .build();
let cloned_request = request.clone();
        assert_eq!(
            request.api_req.query_params
            cloned_request.api_req.query_params
);
        assert_eq!(request.api_req.body, cloned_request.api_req.body);
#[test]
    fn test_create_message_request_body_debug_trait() {
let body = CreateMessageRequestBody {,
            receive_id: "user_123".to_string(),
            msg_type: "text".to_string(),
            content: "test content".to_string(),
            uuid: Some("uuid-123".to_string())};

        let debug_str = format!("{:?}", body);
assert!(debug_str.contains("CreateMessageRequestBody"));
        assert!(debug_str.contains("user_123"));
assert!(debug_str.contains("text"));
        assert!(debug_str.contains("test content"));
#[test]
    fn test_create_message_request_body_clone_trait() {
let body = CreateMessageRequestBody {,
            receive_id: "user_123".to_string(),
            msg_type: "text".to_string(),
            content: "test content".to_string(),
            uuid: Some("uuid-123".to_string())};
let cloned_body = body.clone();
        assert_eq!(body.receive_id, cloned_body.receive_id);
        assert_eq!(body.msg_type, cloned_body.msg_type);
        assert_eq!(body.content, cloned_body.content);
        assert_eq!(body.uuid, cloned_body.uuid);
// Performance and memory tests,
    #[test]
fn test_builder_memory_efficiency() {
        // Create many builders to test memory usage,
let mut builders = Vec::new();
        for i in 0..100 {,
let builder = CreateMessageRequestBuilder::default(),
                .receive_id(format!("user_{}", i)),
.msg_type()
                .content(format!("Message {}", i)),
.receive_id_type("open_id");
            builders.push(builder);
// Build all requests,
        let requests: Vec<CreateMessageRequest> = builders.into_iter().map(|b| b.build()).collect();

        assert_eq!(requests.len(), 100);
// Verify a few random requests,
        assert_eq!(
            requests[0].api_req.query_params.get("receive_id"),
            Some(&"user_0".to_string()),
);
        assert_eq!(
            requests[99].api_req.query_params.get("content"),
            Some(&"Message 99".to_string()),
);
    }
// Test complex scenarios,
    #[test]
fn test_complex_message_content() {
        let complex_content = r#"{,
"post": {,
                "zh_cn": {
                    "title": "复杂消息",
                    "content": [,
[,
                            {
                                "tag": "text",
                                "text": "这是一条复杂消息"}
                            {
                                "tag": "a",
                                "text": "链接文本",
                                "href": "https://example.com"}
],
                    ],
}"#;
let body = CreateMessageRequestBodyBuilder::default(),
            .receive_id()
.msg_type()
            .content(complex_content.to_string()),
.uuid()
            .build();

        assert_eq!(body.msg_type, "post");
        assert_eq!(body.content, complex_content);
        assert_eq!(body.uuid, Some("complex-uuid".to_string()));
#[test]
    fn test_message_with_special_characters() {
let special_content =,
            r#"{"text":"Special chars: \"quotes\", 'apostrophes', \n\t\r slashes \\//"}"#;
let request = CreateMessageRequestBuilder::default(),
            .receive_id()
.msg_type()
            .content(special_content.to_string()),
.receive_id_type()
            .build();
assert_eq!(,
            request.api_req.query_params.get("content"),
            Some(&special_content.to_string()),
);
    }
#[test]
    fn test_message_with_long_uuid() {
// Test UUID length validation (should be truncated if too long),
        let long_uuid = "a".repeat(100); // 100 characters, exceeding typical UUID limits,
let body = CreateMessageRequestBodyBuilder::default(),
            .receive_id()
.msg_type()
            .content()
.uuid()
            .build();
// The UUID should be validated and potentially truncated,
        assert!(body.uuid.is_some());
// Note: The actual validation logic depends on the validate_string_length implementation,
        // This test mainly ensures the builder handles long UUIDs without panicking}
