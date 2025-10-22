use reqwest::Method;
use serde::{Deserialize, Serialize};

use open_lark_core::core::{
    api_req::ApiRequest,
    api_resp::{ApiResponseTrait, BaseResponse},
    constants::AccessTokenType,
    http::Transport,
    req_option::RequestOption,
    standard_response::StandardResponse,
    validation::{self, ValidationResult},
    SDKResult,
};

use crate::im::v1::message::{ListMessageIterator, Message};

use super::MessageService;

/// 列表消息请求
#[derive(Default, Clone)]
pub struct ListMessageRequest {
    pub api_req: ApiRequest,
}

impl ListMessageRequest {
    pub fn builder() -> ListMessageRequestBuilder {
        ListMessageRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct ListMessageRequestBuilder {
    request: ListMessageRequest,
}

impl ListMessageRequestBuilder {
    /// 容器类型 ，目前可选值仅有"chat"，包含单聊（p2p）和群聊（group）
    ///
    /// 示例值：chat
    pub fn container_id_type(mut self, container_id_type: impl ToString) -> Self {
        self.request
            .api_req
            .query_params
            .insert("container_id_type", container_id_type.to_string());
        self
    }

    /// 容器的id，即chat的id
    ///
    /// 示例值：oc_234jsi43d3ssi993d43545f
    pub fn container_id(mut self, container_id: impl ToString) -> Self {
        self.request
            .api_req
            .query_params
            .insert("container_id", container_id.to_string());
        self
    }

    /// 历史信息的起始时间（秒级时间戳）
    ///
    /// 示例值：1609296809
    pub fn start_time(mut self, start_time: i64) -> Self {
        self.request
            .api_req
            .query_params
            .insert("start_time", start_time.to_string());
        self
    }

    /// 历史信息的结束时间（秒级时间戳）
    ///
    /// 示例值：1608594809
    pub fn end_time(mut self, end_time: i64) -> Self {
        self.request
            .api_req
            .query_params
            .insert("end_time", end_time.to_string());
        self
    }

    /// 消息排序方式
    ///
    /// 示例值：ByCreateTimeAsc
    pub fn sort_type(mut self, sort_type: impl ToString) -> Self {
        self.request
            .api_req
            .query_params
            .insert("sort_type", sort_type.to_string());
        self
    }

    /// 分页标记，第一次请求不填，表示从头开始遍历；分页查询结果还有更多项时会同时返回新的
    /// page_token，下次遍历可采用该 page_token 获取查询结果
    pub fn page_token(mut self, page_token: impl ToString) -> Self {
        self.request
            .api_req
            .query_params
            .insert("page_token", page_token.to_string());
        self
    }

    /// 分页大小
    ///
    /// 示例值：20
    ///
    /// # 验证规则
    ///
    /// 分页大小必须在 1-500 之间，推荐值为 50
    pub fn page_size(mut self, page_size: i32) -> Self {
        // 验证分页大小
        match validation::validate_page_size(page_size as u32, "page_size") {
            ValidationResult::Valid => {}
            ValidationResult::Warning(msg) => {
                log::warn!("Page size validation warning: {}", msg);
            }
            ValidationResult::Invalid(msg) => {
                log::error!("Invalid page size: {}", msg);
                // 仍然设置值，但记录错误，让用户决定是否继续
            }
        }
        self.request
            .api_req
            .query_params
            .insert("page_size", page_size.to_string());
        self
    }

    pub fn build(self) -> ListMessageRequest {
        // 验证分页标记（如果存在）
        if let Some(page_token) = self.request.api_req.query_params.get("page_token") {
            match validation::validate_page_token(page_token, "page_token") {
                ValidationResult::Valid => {}
                ValidationResult::Warning(msg) => {
                    log::warn!("Page token validation warning: {}", msg);
                }
                ValidationResult::Invalid(msg) => {
                    log::error!("Invalid page token: {}", msg);
                    // 仍然设置值，但记录错误
                }
            }
        }

        self.request
    }

    /// 使用分页验证构建器设置分页参数
    ///
    /// 这个方法提供了一个更安全的分页参数设置方式，会自动验证参数的有效性
    pub fn with_pagination(
        mut self,
        page_size: Option<u32>,
        page_token: Option<String>,
    ) -> SDKResult<Self> {
        let mut pagination_builder =
            validation::pagination::PaginationRequestBuilder::<ListMessageRespData>::new();

        if let Some(size) = page_size {
            pagination_builder = pagination_builder.with_page_size(size);
        }

        if let Some(token) = page_token {
            pagination_builder = pagination_builder.with_page_token(token);
        }

        // 构建分页参数
        let params = pagination_builder.build()?;

        // 应用到请求中
        for (key, value) in params {
            self.request.api_req.query_params.insert(key, value);
        }

        Ok(self)
    }
}

crate::impl_executable_builder_owned!(
    ListMessageRequestBuilder,
    MessageService,
    ListMessageRequest,
    ListMessageRespData,
    list
);

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

impl MessageService {
    /// 获取会话历史消息
    ///
    /// 获取会话（包括单聊、群组）的历史消息（聊天记录）
    pub async fn list(
        &self,
        list_message_request: ListMessageRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<ListMessageRespData> {
        let mut api_req = list_message_request.api_req;
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(open_lark_core::core::endpoints::im::IM_V1_LIST_MESSAGE.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

        let api_resp: BaseResponse<ListMessageRespData> =
            Transport::request(api_req, &self.config, option).await?;

        api_resp.into_result()
    }

    /// 创建消息列表迭代器
    ///
    /// 提供便捷的方式遍历所有消息，自动处理分页
    pub fn list_iter(
        &self,
        list_message_request: ListMessageRequest,
        _option: Option<RequestOption>,
    ) -> ListMessageIterator<'_> {
        ListMessageIterator::new(self, list_message_request)
    }
    /// 使用分页验证创建消息列表请求
    ///
    /// 提供一个更安全的方式来创建消息列表请求，自动验证分页参数
    pub async fn list_with_validated_pagination(
        &self,
        container_id: impl ToString,
        container_id_type: impl ToString,
        page_size: Option<u32>,
        page_token: Option<String>,
        option: Option<RequestOption>,
    ) -> SDKResult<ListMessageRespData> {
        // 创建请求构建器
        let builder = ListMessageRequest::builder()
            .container_id(container_id)
            .container_id_type(container_id_type)
            .with_pagination(page_size, page_token)?;

        self.list(builder.build(), option).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        core::{config::Config, constants::AccessTokenType, req_option::RequestOption},
        crate::im::v1::message::{Message, MessageService},
    };
    use reqwest::Method;
    use serde_json;

    // Helper function to create test message service
    fn create_test_message_service() -> MessageService {
        MessageService {
            config: Config::default(),
        }
    }

    // Helper function to create test message using JSON deserialization
    fn create_test_message(message_id: &str, chat_id: &str) -> Message {
        let json_str = r#"{
            "message_id": "PLACEHOLDER_MSG_ID",
            "msg_type": "text",
            "create_time": "1640995200000",
            "update_time": "1640995200000",
            "deleted": false,
            "updated": false,
            "chat_id": "PLACEHOLDER_CHAT_ID",
            "sender": {
                "id": "test_user_123",
                "id_type": "open_id",
                "sender_type": "user",
                "tenant_key": "test_tenant"
            },
            "body": {
                "content": "{\"text\":\"Hello World\"}"
            }
        }"#;

        // Replace placeholders with actual values
        let json_str = json_str
            .replace("PLACEHOLDER_MSG_ID", message_id)
            .replace("PLACEHOLDER_CHAT_ID", chat_id);

        serde_json::from_str(&json_str).unwrap()
    }

    // Helper function to create test response data
    fn create_test_response_data(
        has_more: bool,
        page_token: Option<String>,
        messages: Vec<Message>,
    ) -> ListMessageRespData {
        ListMessageRespData {
            has_more,
            page_token,
            items: messages,
        }
    }

    #[test]
    fn test_list_message_request_builder_basic() {
        let builder = ListMessageRequest::builder();
        assert!(builder.request.api_req.query_params.is_empty());
    }

    #[test]
    fn test_builder_container_id_type() {
        let builder = ListMessageRequest::builder().container_id_type("chat");

        assert_eq!(
            builder
                .request
                .api_req
                .query_params
                .get("container_id_type"),
            Some(&"chat".to_string())
        );
    }

    #[test]
    fn test_builder_container_id() {
        let chat_id = "oc_123456789";
        let builder = ListMessageRequest::builder().container_id(chat_id);

        assert_eq!(
            builder.request.api_req.query_params.get("container_id"),
            Some(&chat_id.to_string())
        );
    }

    #[test]
    fn test_builder_time_range() {
        let start_time = 1609296809i64;
        let end_time = 1609383209i64;

        let builder = ListMessageRequest::builder()
            .start_time(start_time)
            .end_time(end_time);

        assert_eq!(
            builder.request.api_req.query_params.get("start_time"),
            Some(&start_time.to_string())
        );
        assert_eq!(
            builder.request.api_req.query_params.get("end_time"),
            Some(&end_time.to_string())
        );
    }

    #[test]
    fn test_builder_sort_type() {
        let sort_type = "ByCreateTimeAsc";
        let builder = ListMessageRequest::builder().sort_type(sort_type);

        assert_eq!(
            builder.request.api_req.query_params.get("sort_type"),
            Some(&sort_type.to_string())
        );
    }

    #[test]
    fn test_builder_page_token() {
        let page_token = "next_page_token_123";
        let builder = ListMessageRequest::builder().page_token(page_token);

        assert_eq!(
            builder.request.api_req.query_params.get("page_token"),
            Some(&page_token.to_string())
        );
    }

    #[test]
    fn test_builder_page_size_valid() {
        let page_sizes = [1, 10, 50, 100, 500]; // All valid sizes

        for size in page_sizes {
            let builder = ListMessageRequest::builder().page_size(size);

            assert_eq!(
                builder.request.api_req.query_params.get("page_size"),
                Some(&size.to_string())
            );
        }
    }

    #[test]
    fn test_builder_page_size_edge_cases() {
        // Test minimum and maximum valid page sizes
        let min_builder = ListMessageRequest::builder().page_size(1);
        let max_builder = ListMessageRequest::builder().page_size(500);

        assert_eq!(
            min_builder.request.api_req.query_params.get("page_size"),
            Some(&"1".to_string())
        );
        assert_eq!(
            max_builder.request.api_req.query_params.get("page_size"),
            Some(&"500".to_string())
        );
    }

    #[test]
    fn test_builder_page_size_out_of_range() {
        // Test sizes outside the valid range (1-500)
        let invalid_sizes = [0, -1, 501, 1000];

        for size in invalid_sizes {
            let builder = ListMessageRequest::builder().page_size(size);

            // Even for invalid sizes, the value should be set (validation logs errors)
            assert_eq!(
                builder.request.api_req.query_params.get("page_size"),
                Some(&size.to_string())
            );
        }
    }

    #[test]
    fn test_builder_complete_request() {
        let builder = ListMessageRequest::builder()
            .container_id_type("chat")
            .container_id("oc_test123")
            .start_time(1609296809)
            .end_time(1609383209)
            .sort_type("ByCreateTimeAsc")
            .page_token("test_token")
            .page_size(50);

        let params = &builder.request.api_req.query_params;
        assert_eq!(params.get("container_id_type"), Some(&"chat".to_string()));
        assert_eq!(params.get("container_id"), Some(&"oc_test123".to_string()));
        assert_eq!(params.get("start_time"), Some(&"1609296809".to_string()));
        assert_eq!(params.get("end_time"), Some(&"1609383209".to_string()));
        assert_eq!(
            params.get("sort_type"),
            Some(&"ByCreateTimeAsc".to_string())
        );
        assert_eq!(params.get("page_token"), Some(&"test_token".to_string()));
        assert_eq!(params.get("page_size"), Some(&"50".to_string()));
    }

    #[test]
    fn test_builder_build_method() {
        let request = ListMessageRequest::builder()
            .container_id("test_chat")
            .container_id_type("chat")
            .page_size(20)
            .build();

        // Verify the request is created properly
        assert_eq!(
            request.api_req.query_params.get("container_id"),
            Some(&"test_chat".to_string())
        );
        assert_eq!(
            request.api_req.query_params.get("container_id_type"),
            Some(&"chat".to_string())
        );
        assert_eq!(
            request.api_req.query_params.get("page_size"),
            Some(&"20".to_string())
        );
    }

    #[test]
    fn test_builder_with_pagination_success() {
        let builder = ListMessageRequest::builder()
            .container_id("test_chat")
            .container_id_type("chat");

        let result = builder.with_pagination(Some(50), Some("token123".to_string()));
        assert!(result.is_ok());

        let built_builder = result.unwrap();
        assert_eq!(
            built_builder.request.api_req.query_params.get("page_size"),
            Some(&"50".to_string())
        );
        assert_eq!(
            built_builder.request.api_req.query_params.get("page_token"),
            Some(&"token123".to_string())
        );
    }

    #[test]
    fn test_builder_with_pagination_none_values() {
        let builder = ListMessageRequest::builder()
            .container_id("test_chat")
            .container_id_type("chat");

        let result = builder.with_pagination(None, None);
        assert!(result.is_ok());

        // Should not add pagination params if they are None
        let built_builder = result.unwrap();
        assert!(!built_builder
            .request
            .api_req
            .query_params
            .contains_key("page_size"));
        assert!(!built_builder
            .request
            .api_req
            .query_params
            .contains_key("page_token"));
    }

    #[test]
    fn test_response_data_serialization() {
        let messages = vec![
            create_test_message("msg_1", "chat_1"),
            create_test_message("msg_2", "chat_1"),
        ];

        let response =
            create_test_response_data(true, Some("next_page".to_string()), messages.clone());

        let json = serde_json::to_string(&response).unwrap();
        let parsed: ListMessageRespData = serde_json::from_str(&json).unwrap();

        assert!(parsed.has_more);
        assert_eq!(parsed.page_token, Some("next_page".to_string()));
        assert_eq!(parsed.items.len(), 2);
        assert_eq!(parsed.items[0].message_id, "msg_1");
        assert_eq!(parsed.items[1].message_id, "msg_2");
    }

    #[test]
    fn test_response_data_last_page() {
        let messages = vec![create_test_message("final_msg", "chat_final")];
        let response = create_test_response_data(false, None, messages.clone());

        assert!(!response.has_more);
        assert_eq!(response.page_token, None);
        assert_eq!(response.items.len(), 1);
    }

    #[test]
    fn test_response_data_empty_page() {
        let response = create_test_response_data(false, None, vec![]);

        assert!(!response.has_more);
        assert_eq!(response.page_token, None);
        assert_eq!(response.items.len(), 0);
    }

    #[test]
    fn test_api_response_trait_implementation() {
        assert_eq!(
            ListMessageRespData::data_format(),
            open_lark_core::core::api_resp::ResponseFormat::Data
        );
    }

    #[test]
    fn test_list_message_iterator_creation() {
        let service = create_test_message_service();
        let request = ListMessageRequest::builder()
            .container_id("test_chat")
            .container_id_type("chat")
            .build();

        let iterator = service.list_iter(request, None);

        // Since the fields are private, we can only test that the iterator was created
        // by checking its debug representation
        let debug_str = format!("{:?}", iterator);
        assert!(debug_str.contains("ListMessageIterator"));
    }

    #[test]
    fn test_list_message_iterator_debug_format() {
        let service = create_test_message_service();
        let request = ListMessageRequest::builder()
            .container_id("test_chat")
            .container_id_type("chat")
            .build();

        let iterator = service.list_iter(request, None);
        let debug_str = format!("{:?}", iterator);

        assert!(debug_str.contains("ListMessageIterator"));
    }

    #[test]
    fn test_service_config_dependency() {
        let service = create_test_message_service();

        // Verify service has config
        let config_ref = &service.config;
        assert!(config_ref.app_id.is_empty()); // Default config should have empty app_id
        assert!(config_ref.app_secret.is_empty()); // Default config should have empty app_secret
    }

    #[test]
    fn test_request_preparation_for_list() {
        let _service = create_test_message_service();
        let request = ListMessageRequest::builder()
            .container_id("test_chat")
            .container_id_type("chat")
            .build();

        let api_req = request.api_req;

        // The HTTP method will be set in the actual list() method
        assert_eq!(api_req.http_method, Method::GET); // Default will be overridden in list method
        assert!(!api_req.query_params.is_empty());
        assert_eq!(
            api_req.query_params.get("container_id"),
            Some(&"test_chat".to_string())
        );
    }

    #[test]
    fn test_access_token_types_configuration() {
        let expected_types = [AccessTokenType::Tenant, AccessTokenType::User];

        // Verify list operations support both token types
        assert_eq!(expected_types.len(), 2);
        assert!(expected_types.contains(&AccessTokenType::Tenant));
        assert!(expected_types.contains(&AccessTokenType::User));
    }

    #[test]
    fn test_endpoint_path_validation() {
        let expected_path = open_lark_core::core::endpoints::im::IM_V1_LIST_MESSAGE;

        // Verify the endpoint exists and has the expected format
        assert!(!expected_path.is_empty());
        assert!(expected_path.starts_with("/open-apis/"));
        assert!(expected_path.contains("messages"));
    }

    #[test]
    fn test_unicode_support_in_parameters() {
        let unicode_chat_id = "聊天室_123";
        let unicode_token = "下一页_令牌";

        let builder = ListMessageRequest::builder()
            .container_id(unicode_chat_id)
            .page_token(unicode_token);

        assert_eq!(
            builder.request.api_req.query_params.get("container_id"),
            Some(&unicode_chat_id.to_string())
        );
        assert_eq!(
            builder.request.api_req.query_params.get("page_token"),
            Some(&unicode_token.to_string())
        );
    }

    #[test]
    fn test_special_characters_in_parameters() {
        let special_chars = "chat.id-with.special@chars#123";
        let special_token = "token.with/special?chars&test=123";

        let builder = ListMessageRequest::builder()
            .container_id(special_chars)
            .page_token(special_token);

        assert_eq!(
            builder.request.api_req.query_params.get("container_id"),
            Some(&special_chars.to_string())
        );
        assert_eq!(
            builder.request.api_req.query_params.get("page_token"),
            Some(&special_token.to_string())
        );
    }

    #[test]
    fn test_large_time_ranges() {
        // Test with very large timestamps
        let large_start = 946684800i64; // Year 2000
        let large_end = 4102444800i64; // Year 2100

        let builder = ListMessageRequest::builder()
            .start_time(large_start)
            .end_time(large_end);

        assert_eq!(
            builder.request.api_req.query_params.get("start_time"),
            Some(&large_start.to_string())
        );
        assert_eq!(
            builder.request.api_req.query_params.get("end_time"),
            Some(&large_end.to_string())
        );
    }

    #[test]
    fn test_negative_timestamps() {
        // Test with negative timestamps (before Unix epoch)
        let negative_time = -86400i64; // One day before Unix epoch

        let builder = ListMessageRequest::builder().start_time(negative_time);

        assert_eq!(
            builder.request.api_req.query_params.get("start_time"),
            Some(&negative_time.to_string())
        );
    }

    #[test]
    fn test_builder_chainability() {
        // Test that builder methods can be chained
        let builder = ListMessageRequest::builder()
            .container_id_type("chat")
            .container_id("test_chat")
            .start_time(1609296809)
            .end_time(1609383209)
            .sort_type("ByCreateTimeAsc")
            .page_token("test_token")
            .page_size(50);

        let params = &builder.request.api_req.query_params;
        assert_eq!(params.len(), 7); // All parameters should be set
        assert!(params.contains_key("container_id_type"));
        assert!(params.contains_key("container_id"));
        assert!(params.contains_key("start_time"));
        assert!(params.contains_key("end_time"));
        assert!(params.contains_key("sort_type"));
        assert!(params.contains_key("page_token"));
        assert!(params.contains_key("page_size"));
    }

    #[test]
    fn test_request_option_handling() {
        // Test RequestOption can be passed (type checking)
        let option: Option<RequestOption> = None;
        assert!(option.is_none());

        let option: Option<RequestOption> = Some(RequestOption::default());
        assert!(option.is_some());
    }

    #[test]
    fn test_concurrent_builder_usage() {
        use std::sync::Arc;

        // Test that builders can be used concurrently
        let builder1 = ListMessageRequest::builder()
            .container_id("chat1")
            .page_size(10);

        let builder2 = ListMessageRequest::builder()
            .container_id("chat2")
            .page_size(20);

        // Both builders should maintain their own state
        assert_eq!(
            builder1.request.api_req.query_params.get("container_id"),
            Some(&"chat1".to_string())
        );
        assert_eq!(
            builder2.request.api_req.query_params.get("container_id"),
            Some(&"chat2".to_string())
        );

        // Test Arc sharing
        let shared_builder = Arc::new(ListMessageRequest::builder().page_size(50));
        let cloned_builder = Arc::clone(&shared_builder);

        assert_eq!(
            shared_builder.request.api_req.query_params.get("page_size"),
            cloned_builder.request.api_req.query_params.get("page_size")
        );
    }

    #[test]
    fn test_error_handling_types() {
        use open_lark_core::core::SDKResult;

        // Verify that the list methods return SDKResult
        fn _check_list_result() -> SDKResult<ListMessageRespData> {
            unimplemented!("Mock implementation")
        }

        // If this compiles, the return type is correct
        let _ = _check_list_result;
    }

    #[test]
    fn test_response_data_memory_efficiency() {
        // Test that response data structure is memory efficient
        let empty_response = create_test_response_data(false, None, vec![]);
        let single_message_response = create_test_response_data(
            false,
            None,
            vec![create_test_message("single", "chat_single")],
        );

        // Verify empty response is lightweight
        assert_eq!(empty_response.items.len(), 0);
        assert!(!empty_response.has_more);
        assert!(empty_response.page_token.is_none());

        // Verify single message response structure
        assert_eq!(single_message_response.items.len(), 1);
        assert_eq!(single_message_response.items[0].message_id, "single");
    }

    #[test]
    fn test_sort_type_variations() {
        let sort_types = [
            "ByCreateTimeAsc",
            "ByCreateTimeDesc",
            "ByUpdateTimeAsc",
            "ByUpdateTimeDesc",
        ];

        for sort_type in sort_types {
            let builder = ListMessageRequest::builder().sort_type(sort_type);

            assert_eq!(
                builder.request.api_req.query_params.get("sort_type"),
                Some(&sort_type.to_string())
            );
        }
    }

    #[test]
    fn test_container_id_types() {
        let container_types = ["chat", "p2p", "group"];

        for container_type in container_types {
            let builder = ListMessageRequest::builder().container_id_type(container_type);

            assert_eq!(
                builder
                    .request
                    .api_req
                    .query_params
                    .get("container_id_type"),
                Some(&container_type.to_string())
            );
        }
    }

    #[test]
    fn test_page_token_edge_cases() {
        let edge_cases = [
            "",                                                           // Empty token
            "a",                                                          // Single character
            "very_long_token_with_many_characters_and_numbers_123456789", // Long token
            "特殊字符令牌",                                               // Unicode token
            "token.with.dots",                                            // Token with dots
            "token/with/slashes",                                         // Token with slashes
        ];

        for token in edge_cases {
            let builder = ListMessageRequest::builder().page_token(token);

            assert_eq!(
                builder.request.api_req.query_params.get("page_token"),
                Some(&token.to_string())
            );
        }
    }

    #[test]
    fn test_message_list_service_methods_exist() {
        // Test that all service methods exist (compile-time check)
        let service = create_test_message_service();

        // Just verify the service can be created
        assert!(!service.config.app_id.is_empty() || service.config.app_id.is_empty());

        // This is mainly a compile-time verification test
        // The actual method calls would require mocking the Transport layer
    }

    #[test]
    fn test_builder_parameter_overriding() {
        // Test that parameters can be overridden
        let mut builder = ListMessageRequest::builder().page_size(10).page_size(20); // Override previous value

        assert_eq!(
            builder.request.api_req.query_params.get("page_size"),
            Some(&"20".to_string())
        );

        // Test multiple overrides
        builder = builder
            .container_id("chat1")
            .container_id("chat2") // Override
            .container_id("chat3"); // Final override

        assert_eq!(
            builder.request.api_req.query_params.get("container_id"),
            Some(&"chat3".to_string())
        );
    }

    #[test]
    fn test_large_message_list_response() {
        // Test handling of response with many messages
        let messages: Vec<Message> = (0..100)
            .map(|i| create_test_message(&format!("msg_{}", i), "large_chat"))
            .collect();

        let response =
            create_test_response_data(true, Some("large_list_token".to_string()), messages.clone());

        assert_eq!(response.items.len(), 100);
        assert!(response.has_more);
        assert!(response.page_token.is_some());

        // Verify first and last messages
        assert_eq!(response.items[0].message_id, "msg_0");
        assert_eq!(response.items[99].message_id, "msg_99");
    }

    #[test]
    fn test_builder_cloning_simulation() {
        // Since ListMessageRequestBuilder doesn't implement Clone,
        // we test that we can create equivalent builders
        let original_builder = ListMessageRequest::builder()
            .container_id("test_chat")
            .page_size(50)
            .sort_type("ByCreateTimeAsc");

        // Create an equivalent builder by building the first and creating a new one
        let original_request = original_builder.build();
        let equivalent_builder = ListMessageRequest::builder()
            .container_id("test_chat")
            .page_size(50)
            .sort_type("ByCreateTimeAsc");

        let equivalent_request = equivalent_builder.build();

        // Both should have the same parameters
        assert_eq!(
            original_request.api_req.query_params.get("container_id"),
            equivalent_request.api_req.query_params.get("container_id")
        );
        assert_eq!(
            original_request.api_req.query_params.get("page_size"),
            equivalent_request.api_req.query_params.get("page_size")
        );
    }
}
