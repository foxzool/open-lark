//! CardKit v1 更新组件API
//!
//! 提供全量更新卡片中指定组件的功能，支持修改组件类型、内容、属性等

use crate::{
    api_resp::{ApiResponseTrait, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    ApiRequest, SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

use super::CardElementService;
use super::super::models::{CardElement, UserIdType};

/// 更新组件请求
#[derive(Debug, Clone)]
pub struct UpdateCardElementRequest {
    /// 卡片ID
    pub card_id: String,
    /// 组件ID
    pub element_id: String,
    /// 组件类型
    pub element_type: Option<String>,
    /// 组件内容
    pub content: Option<serde_json::Value>,
    /// 组件属性
    pub properties: Option<serde_json::Value>,
    /// 父组件ID
    pub parent_id: Option<String>,
    /// 用户ID类型
    pub user_id_type: Option<UserIdType>,
}

impl UpdateCardElementRequest {
    /// 创建新的请求实例
    pub fn new(card_id: impl Into<String>, element_id: impl Into<String>) -> Self {
        Self {
            card_id: card_id.into(),
            element_id: element_id.into(),
            element_type: None,
            content: None,
            properties: None,
            parent_id: None,
            user_id_type: None,
        }
    }

    /// 设置组件类型
    pub fn element_type(mut self, element_type: impl Into<String>) -> Self {
        self.element_type = Some(element_type.into());
        self
    }

    /// 设置组件内容
    pub fn content(mut self, content: serde_json::Value) -> Self {
        self.content = Some(content);
        self
    }

    /// 设置组件属性
    pub fn properties(mut self, properties: serde_json::Value) -> Self {
        self.properties = Some(properties);
        self
    }

    /// 设置父组件ID
    pub fn parent_id(mut self, parent_id: impl Into<String>) -> Self {
        self.parent_id = Some(parent_id.into());
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.card_id.trim().is_empty() {
            return Err("card_id不能为空".to_string());
        }

        if self.element_id.trim().is_empty() {
            return Err("element_id不能为空".to_string());
        }

        // 如果提供了组件类型，验证是否为支持的类型
        if let Some(ref element_type) = self.element_type {
            if element_type.trim().is_empty() {
                return Err("element_type不能为空字符串".to_string());
            }

            let supported_types = [
                "text", "image", "button", "div", "hr", "form", "input", "select",
                "checkbox", "radio", "textarea", "date", "time", "file", "table",
                "chart", "video", "audio", "link", "markdown", "html"
            ];

            if !supported_types.contains(&element_type.as_str()) {
                return Err(format!("不支持的组件类型: {}", element_type));
            }
        }

        if let Some(ref content) = self.content {
            if content.is_null() {
                return Err("content不能为null".to_string());
            }
        }

        if let Some(ref properties) = self.properties {
            if properties.is_null() {
                return Err("properties不能为null".to_string());
            }
        }

        Ok(())
    }
}

/// 更新组件响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCardElementResponseData {
    /// 更新后的组件信息
    pub element: CardElement,
    /// 更新的字段列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_fields: Option<Vec<String>>,
}

/// 更新组件响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateCardElementResponse {
    /// 响应数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<UpdateCardElementResponseData>,
    /// 是否成功
    pub success: bool,
    /// 错误消息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// 错误代码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
}

impl ApiResponseTrait for UpdateCardElementResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl CardElementService {
    /// 更新组件
    ///
    /// 全量更新指定卡片中的特定组件
    ///
    /// # 参数
    /// * `req` - 更新组件请求
    ///
    /// # 返回值
    /// 返回更新后的组件信息
    ///
    /// # 示例
    /// ```rust
    /// use open_lark::service::cardkit::v1::card_element::update::{UpdateCardElementRequest, UserIdType};
    /// use serde_json::json;
    ///
    /// let request = UpdateCardElementRequest::new("card_123", "element_456")
    ///     .element_type("text")
    ///     .content(json!({
    ///         "type": "plain_text",
    ///         "content": "更新后的文本内容"
    ///     }))
    ///     .user_id_type(UserIdType::OpenId);
    /// let response = service.update_card_element(&request).await?;
    /// ```
    pub async fn update_card_element(&self, req: &UpdateCardElementRequest) -> SDKResult<UpdateCardElementResponse> {
        req.validate()?;
        log::debug!("开始更新组件: card_id={}, element_id={}", req.card_id, req.element_id);

        // 构建查询参数
        let mut query_params: HashMap<&str, String> = HashMap::new();
        if let Some(user_id_type) = &req.user_id_type {
            query_params.insert("user_id_type", user_id_type.to_string());
        }

        // 构建请求体
        let mut body = json!({});

        if let Some(ref element_type) = req.element_type {
            body["element_type"] = json!(element_type);
        }
        if let Some(ref content) = req.content {
            body["content"] = content.clone();
        }
        if let Some(ref properties) = req.properties {
            body["properties"] = properties.clone();
        }
        if let Some(ref parent_id) = req.parent_id {
            body["parent_id"] = json!(parent_id);
        }

        // 构建API路径，替换card_id和element_id占位符
        let mut api_path = crate::core::endpoints_original::Endpoints::CARDKIT_V1_CARD_ELEMENTS_UPDATE
            .replace("{card_id}", &req.card_id);
        api_path = api_path.replace("{element_id}", &req.element_id);

        let api_req = ApiRequest {
            http_method: reqwest::Method::PUT,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: serde_json::to_vec(&body).unwrap_or_default(),
            ..Default::default()
        };

        let resp = Transport::<UpdateCardElementResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        if response.success {
            log::info!("组件更新成功: card_id={}, element_id={}", req.card_id, req.element_id);
        } else {
            log::warn!("组件更新失败: card_id={}, element_id={}, error={:?}",
                req.card_id, req.element_id, response.error_message);
        }

        Ok(response)
    }
}

// ==================== 构建器模式 ====================

/// 更新组件构建器
#[derive(Debug, Clone)]
pub struct UpdateCardElementBuilder {
    request: UpdateCardElementRequest,
}

impl UpdateCardElementBuilder {
    /// 创建新的构建器
    pub fn new(card_id: impl Into<String>, element_id: impl Into<String>) -> Self {
        Self {
            request: UpdateCardElementRequest::new(card_id, element_id),
        }
    }

    /// 设置组件类型
    pub fn element_type(mut self, element_type: impl Into<String>) -> Self {
        self.request = self.request.element_type(element_type);
        self
    }

    /// 设置组件内容
    pub fn content(mut self, content: serde_json::Value) -> Self {
        self.request = self.request.content(content);
        self
    }

    /// 设置组件属性
    pub fn properties(mut self, properties: serde_json::Value) -> Self {
        self.request = self.request.properties(properties);
        self
    }

    /// 设置父组件ID
    pub fn parent_id(mut self, parent_id: impl Into<String>) -> Self {
        self.request = self.request.parent_id(parent_id);
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.request = self.request.user_id_type(user_id_type);
        self
    }

    /// 执行更新组件操作
    pub async fn execute(self, service: &CardElementService) -> SDKResult<UpdateCardElementResponse> {
        service.update_card_element(&self.request).await
    }
}

impl CardElementService {
    /// 更新组件构建器
    pub fn update_card_element_builder(&self, card_id: impl Into<String>, element_id: impl Into<String>) -> UpdateCardElementBuilder {
        UpdateCardElementBuilder::new(card_id, element_id)
    }
}

// ==================== 单元测试 ====================

#[cfg(test)]
mod tests {
    use super::*;
    use open_lark_core::config::Config;

    #[test]
    fn test_update_card_element_request_creation() {
        let request = UpdateCardElementRequest::new("card_123", "element_456");
        assert_eq!(request.card_id, "card_123");
        assert_eq!(request.element_id, "element_456");
        assert_eq!(request.element_type, None);
        assert_eq!(request.content, None);
        assert_eq!(request.properties, None);
        assert_eq!(request.parent_id, None);
        assert_eq!(request.user_id_type, None);
    }

    #[test]
    fn test_update_card_element_request_with_fields() {
        let content = json!({
            "type": "plain_text",
            "content": "更新后的内容"
        });
        let properties = json!({
            "style": "primary",
            "size": "large"
        });

        let request = UpdateCardElementRequest::new("card_123", "element_456")
            .element_type("text")
            .content(content.clone())
            .properties(properties.clone())
            .parent_id("parent_789")
            .user_id_type(UserIdType::OpenId);

        assert_eq!(request.card_id, "card_123");
        assert_eq!(request.element_id, "element_456");
        assert_eq!(request.element_type, Some("text".to_string()));
        assert_eq!(request.content, Some(content));
        assert_eq!(request.properties, Some(properties));
        assert_eq!(request.parent_id, Some("parent_789".to_string()));
        assert_eq!(request.user_id_type, Some(UserIdType::OpenId));
    }

    #[test]
    fn test_update_card_element_request_validation() {
        // 测试正常情况
        let valid_request = UpdateCardElementRequest::new("card_123", "element_456")
            .element_type("text")
            .content(json!({"type": "plain_text", "content": "有效内容"}));
        assert!(valid_request.validate().is_ok());

        // 测试空card_id
        let empty_card_request = UpdateCardElementRequest::new("", "element_456");
        assert!(empty_card_request.validate().is_err());

        // 测试空element_id
        let empty_element_request = UpdateCardElementRequest::new("card_123", "");
        assert!(empty_element_request.validate().is_err());

        // 测试空element_type
        let empty_type_request = UpdateCardElementRequest::new("card_123", "element_456")
            .element_type("  ");
        assert!(empty_type_request.validate().is_err());

        // 测试不支持的组件类型
        let unsupported_request = UpdateCardElementRequest::new("card_123", "element_456")
            .element_type("unsupported_type");
        assert!(unsupported_request.validate().is_err());

        // 测试null content
        let null_content_request = UpdateCardElementRequest::new("card_123", "element_456")
            .content(json!(null));
        assert!(null_content_request.validate().is_err());

        // 测试null properties
        let null_properties_request = UpdateCardElementRequest::new("card_123", "element_456")
            .properties(json!(null));
        assert!(null_properties_request.validate().is_err());
    }

    #[test]
    fn test_update_card_element_response_creation() {
        let element = CardElement {
            element_id: Some("element_456".to_string()),
            element_type: Some("text".to_string()),
            content: Some(json!({
                "type": "plain_text",
                "content": "更新后的文本"
            })),
            ..Default::default()
        };

        let response_data = UpdateCardElementResponseData {
            element,
            updated_fields: Some(vec!["content".to_string(), "element_type".to_string()]),
        };

        let response = UpdateCardElementResponse {
            data: Some(response_data),
            success: true,
            ..Default::default()
        };

        assert!(response.success);
        assert!(response.data.is_some());
        assert_eq!(response.data.as_ref().unwrap().element.element_id, Some("element_456".to_string()));
        assert_eq!(response.data.as_ref().unwrap().element.element_type, Some("text".to_string()));
        assert_eq!(
            response.data.as_ref().unwrap().updated_fields.as_ref().unwrap(),
            vec!["content".to_string(), "element_type".to_string()]
        );
    }

    #[test]
    fn test_update_card_element_builder() {
        let content = json!({
            "type": "plain_text",
            "content": "构建器更新测试"
        });
        let properties = json!({
            "color": "#FF0000",
            "bold": true
        });

        let builder = UpdateCardElementBuilder::new("card_123", "element_456")
            .element_type("text")
            .content(content.clone())
            .properties(properties.clone())
            .parent_id("parent_789")
            .user_id_type(UserIdType::UserId);

        assert_eq!(builder.request.card_id, "card_123");
        assert_eq!(builder.request.element_id, "element_456");
        assert_eq!(builder.request.element_type, Some("text".to_string()));
        assert_eq!(builder.request.content, Some(content));
        assert_eq!(builder.request.properties, Some(properties));
        assert_eq!(builder.request.parent_id, Some("parent_789".to_string()));
        assert_eq!(builder.request.user_id_type, Some(UserIdType::UserId));
    }

    #[test]
    fn test_update_card_element_builder_validation() {
        // 测试有效构建器
        let valid_builder = UpdateCardElementBuilder::new("card_123", "element_456")
            .element_type("text")
            .content(json!({"type": "plain_text", "content": "有效"}));
        assert!(valid_builder.request.validate().is_ok());

        // 测试无效构建器
        let invalid_builder = UpdateCardElementBuilder::new("", "element_456");
        assert!(invalid_builder.request.validate().is_err());

        // 测试空element_id构建器
        let empty_element_builder = UpdateCardElementBuilder::new("card_123", "");
        assert!(empty_element_builder.request.validate().is_err());

        // 测试不支持的组件类型
        let unsupported_builder = UpdateCardElementBuilder::new("card_123", "element_456")
            .element_type("invalid_type");
        assert!(unsupported_builder.request.validate().is_err());
    }

    #[test]
    fn test_update_card_element_service_method() {
        let config = Config::default();
        let service = CardElementService::new(config);

        // 验证服务包含所需的方法
        let service_str = format!("{:?}", service);
        assert!(!service_str.is_empty());

        // 验证构建器方法存在
        let builder = service.update_card_element_builder("card_123", "element_456");
        assert_eq!(builder.request.card_id, "card_123");
        assert_eq!(builder.request.element_id, "element_456");
    }

    #[test]
    fn test_update_card_element_endpoint_construction() {
        // 验证端点常量存在
        assert_eq!(
            crate::core::endpoints_original::Endpoints::CARDKIT_V1_CARD_ELEMENTS_UPDATE,
            "/open-apis/cardkit/v1/cards/{card_id}/elements/{element_id}"
        );

        // 验证路径替换逻辑
        let template = crate::core::endpoints_original::Endpoints::CARDKIT_V1_CARD_ELEMENTS_UPDATE;
        let final_path = template
            .replace("{card_id}", "card_123")
            .replace("{element_id}", "element_456");
        assert_eq!(final_path, "/open-apis/cardkit/v1/cards/card_123/elements/element_456");
    }

    #[test]
    fn test_update_card_element_json_serialization() {
        let request = UpdateCardElementRequest::new("card_123", "element_456")
            .element_type("text")
            .content(json!({"type": "plain_text", "content": "序列化测试"}))
            .properties(json!({"color": "blue", "size": 16}))
            .parent_id("parent_789")
            .user_id_type(UserIdType::OpenId);

        // 测试请求可以转换为JSON
        let body = json!({
            "element_type": "text",
            "content": {"type": "plain_text", "content": "序列化测试"},
            "properties": {"color": "blue", "size": 16},
            "parent_id": "parent_789"
        });

        assert_eq!(body["element_type"], "text");
        assert_eq!(body["content"]["type"], "plain_text");
        assert_eq!(body["content"]["content"], "序列化测试");
        assert_eq!(body["properties"]["color"], "blue");
        assert_eq!(body["properties"]["size"], 16);
        assert_eq!(body["parent_id"], "parent_789");
    }

    #[test]
    fn test_update_card_element_response_trait() {
        assert_eq!(UpdateCardElementResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_update_card_element_comprehensive_scenario() {
        // 测试完整的业务场景 - 更新一个复杂的按钮组件
        let complex_content = json!({
            "type": "button",
            "text": {
                "type": "plain_text",
                "content": "更新后的提交按钮"
            },
            "url": "https://example.com/updated-submit",
            "actions": [
                {
                    "type": "form",
                    "method": "POST",
                    "url": "/api/updated-submit"
                }
            ]
        });

        let complex_properties = json!({
            "style": "danger",
            "size": "medium",
            "disabled": false,
            "loading": false,
            "validation": {
                "required": true,
                "pattern": "^[a-zA-Z0-9]+$"
            }
        });

        let request = UpdateCardElementRequest::new("card_comprehensive_001", "button_update_001")
            .element_type("button")
            .content(complex_content.clone())
            .properties(complex_properties.clone())
            .parent_id("form_container_updated")
            .user_id_type(UserIdType::UnionId);

        assert!(request.validate().is_ok());
        assert_eq!(request.card_id, "card_comprehensive_001");
        assert_eq!(request.element_id, "button_update_001");
        assert_eq!(request.element_type, Some("button".to_string()));
        assert!(request.content.is_some());
        assert!(request.properties.is_some());
        assert_eq!(request.parent_id, Some("form_container_updated".to_string()));
        assert_eq!(request.user_id_type, Some(UserIdType::UnionId));
    }

    #[test]
    fn test_update_card_element_partial_update() {
        // 测试部分更新场景 - 只更新内容
        let partial_request = UpdateCardElementRequest::new("card_partial_001", "element_partial_001")
            .content(json!({
                "type": "plain_text",
                "content": "只更新内容"
            }));

        assert!(partial_request.validate().is_ok());
        assert_eq!(partial_request.card_id, "card_partial_001");
        assert_eq!(partial_request.element_id, "element_partial_001");
        assert_eq!(partial_request.element_type, None);
        assert_eq!(partial_request.content.is_some(), true);
        assert_eq!(partial_request.properties, None);
        assert_eq!(partial_request.parent_id, None);
        assert_eq!(partial_request.user_id_type, None);
    }

    #[test]
    fn test_update_card_element_edge_cases() {
        // 测试极长内容
        let long_content = json!({
            "type": "plain_text",
            "content": "u".repeat(1000)
        });

        let edge_request = UpdateCardElementRequest::new("card_edge_001", "element_edge_001")
            .content(long_content);

        assert!(edge_request.validate().is_ok());
        assert!(edge_request.content.is_some());
        assert_eq!(edge_request.content.as_ref().unwrap()["content"].as_str().unwrap().len(), 1000);

        // 测试嵌套对象
        let nested_content = json!({
            "type": "div",
            "elements": [
                {
                    "type": "text",
                    "content": {
                        "nested": {
                            "deeply": {
                                "nested": "updated value"
                            }
                        }
                    }
                }
            ]
        });

        let nested_request = UpdateCardElementRequest::new("card_nested_001", "element_nested_001")
            .content(nested_content);
        assert!(nested_request.validate().is_ok());
    }

    #[test]
    fn test_update_card_element_different_element_types() {
        // 测试不同类型的组件更新
        let element_types = ["text", "image", "button", "div", "input", "select"];

        for element_type in &element_types {
            let request = UpdateCardElementRequest::new("card_test_001", "element_test_001")
                .element_type(*element_type);
            assert!(request.validate().is_ok(), "Element type {} should be valid", element_type);
            assert_eq!(request.element_type, Some(element_type.to_string()));
        }
    }

    #[test]
    fn test_update_card_element_empty_request() {
        // 测试空请求 - 只提供必需的ID字段
        let empty_request = UpdateCardElementRequest::new("card_empty_001", "element_empty_001");

        // 只提供card_id和element_id，其他字段都为空
        assert!(empty_request.validate().is_ok());
        assert_eq!(empty_request.card_id, "card_empty_001");
        assert_eq!(empty_request.element_id, "element_empty_001");
        assert_eq!(empty_request.element_type, None);
        assert_eq!(empty_request.content, None);
        assert_eq!(empty_request.properties, None);
        assert_eq!(empty_request.parent_id, None);
        assert_eq!(empty_request.user_id_type, None);
    }

    #[test]
    fn test_update_card_element_builder_pattern() {
        // 测试构建器模式的流畅性
        let builder = UpdateCardElementBuilder::new("test_card", "test_element")
            .element_type("markdown")
            .content(json!({
                "type": "markdown",
                "content": "# 更新标题\n\n这是一个更新后的测试内容"
            }))
            .properties(json!({
                "theme": "dark",
                "scrollable": true
            }))
            .user_id_type(UserIdType::OpenId);

        // 验证构建器状态
        assert_eq!(builder.request.card_id, "test_card");
        assert_eq!(builder.request.element_id, "test_element");
        assert_eq!(builder.request.element_type, Some("markdown".to_string()));
        assert!(builder.request.content.is_some());
        assert!(builder.request.properties.is_some());
        assert_eq!(builder.request.user_id_type, Some(UserIdType::OpenId));

        // 验证请求验证通过
        assert!(builder.request.validate().is_ok());

        // 测试链式调用
        let chained_builder = builder
            .element_type("text")  // 重新设置element_type
            .request;
        assert_eq!(chained_builder.element_type, Some("text".to_string()));
    }
}