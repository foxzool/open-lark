//! CardKit v1 更新组件属性API
//!
//! 提供部分更新卡片组件属性的功能，支持只更新特定字段而不影响其他内容

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

/// 更新组件属性请求
#[derive(Debug, Clone)]
pub struct PatchCardElementRequest {
    /// 卡片ID
    pub card_id: String,
    /// 组件ID
    pub element_id: String,
    /// 要更新的组件字段
    pub element_type: Option<String>,
    /// 要更新的组件内容
    pub content: Option<serde_json::Value>,
    /// 要更新的组件属性
    pub properties: Option<serde_json::Value>,
    /// 要更新的父组件ID
    pub parent_id: Option<String>,
    /// 用户ID类型
    pub user_id_type: Option<UserIdType>,
}

impl PatchCardElementRequest {
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

    /// 设置要更新的组件类型
    pub fn element_type(mut self, element_type: impl Into<String>) -> Self {
        self.element_type = Some(element_type.into());
        self
    }

    /// 设置要更新的组件内容
    pub fn content(mut self, content: serde_json::Value) -> Self {
        self.content = Some(content);
        self
    }

    /// 设置要更新的组件属性
    pub fn properties(mut self, properties: serde_json::Value) -> Self {
        self.properties = Some(properties);
        self
    }

    /// 设置要更新的父组件ID
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

        // 验证至少有一个要更新的字段
        if self.element_type.is_none() && self.content.is_none()
            && self.properties.is_none() && self.parent_id.is_none() {
            return Err("至少需要提供一个要更新的字段".to_string());
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

    /// 构建部分更新请求体
    pub fn build_patch_body(&self) -> serde_json::Value {
        let mut body = json!({});

        // 只包含非None字段
        if let Some(ref element_type) = self.element_type {
            body["element_type"] = json!(element_type);
        }
        if let Some(ref content) = self.content {
            body["content"] = content.clone();
        }
        if let Some(ref properties) = self.properties {
            body["properties"] = properties.clone();
        }
        if let Some(ref parent_id) = self.parent_id {
            body["parent_id"] = json!(parent_id);
        }

        body
    }
}

/// 更新组件属性响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchCardElementResponseData {
    /// 更新后的组件信息
    pub element: CardElement,
    /// 更新的字段列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_fields: Option<Vec<String>>,
}

/// 更新组件属性响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PatchCardElementResponse {
    /// 响应数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<PatchCardElementResponseData>,
    /// 是否成功
    pub success: bool,
    /// 错误消息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// 错误代码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
}

impl ApiResponseTrait for PatchCardElementResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl CardElementService {
    /// 更新组件属性
    ///
    /// 部分更新指定卡片组件的属性，只更新提供的字段
    ///
    /// # 参数
    /// * `req` - 更新组件属性请求
    ///
    /// # 返回值
    /// 返回更新后的组件信息
    ///
    /// # 示例
    /// ```rust
    /// use open_lark::service::cardkit::v1::card_element::patch::{PatchCardElementRequest, UserIdType};
    /// use serde_json::json;
    ///
    /// let request = PatchCardElementRequest::new("card_123", "element_456")
    ///     .properties(json!({
    ///         "style": "primary",
    ///         "disabled": false
    ///     }))
    ///     .user_id_type(UserIdType::OpenId);
    /// let response = service.patch_card_element(&request).await?;
    /// ```
    pub async fn patch_card_element(&self, req: &PatchCardElementRequest) -> SDKResult<PatchCardElementResponse> {
        req.validate()?;
        log::debug!("开始更新组件属性: card_id={}, element_id={}", req.card_id, req.element_id);

        // 构建查询参数
        let mut query_params: HashMap<&str, String> = HashMap::new();
        if let Some(user_id_type) = &req.user_id_type {
            query_params.insert("user_id_type", user_id_type.to_string());
        }

        // 构建部分更新请求体
        let body = req.build_patch_body();

        // 构建API路径，替换card_id和element_id占位符
        let api_path = crate::core::endpoints_original::Endpoints::CARDKIT_V1_CARD_ELEMENTS_PATCH
            .replace("{card_id}", &req.card_id)
            .replace("{element_id}", &req.element_id);

        let api_req = ApiRequest {
            http_method: reqwest::Method::PATCH,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: serde_json::to_vec(&body).unwrap_or_default(),
            ..Default::default()
        };

        let resp = Transport::<PatchCardElementResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        if response.success {
            log::info!("组件属性更新成功: card_id={}, element_id={}", req.card_id, req.element_id);
        } else {
            log::warn!("组件属性更新失败: card_id={}, element_id={}, error={:?}",
                req.card_id, req.element_id, response.error_message);
        }

        Ok(response)
    }
}

// ==================== 构建器模式 ====================

/// 更新组件属性构建器
#[derive(Debug, Clone)]
pub struct PatchCardElementBuilder {
    request: PatchCardElementRequest,
}

impl PatchCardElementBuilder {
    /// 创建新的构建器
    pub fn new(card_id: impl Into<String>, element_id: impl Into<String>) -> Self {
        Self {
            request: PatchCardElementRequest::new(card_id, element_id),
        }
    }

    /// 设置要更新的组件类型
    pub fn element_type(mut self, element_type: impl Into<String>) -> Self {
        self.request = self.request.element_type(element_type);
        self
    }

    /// 设置要更新的组件内容
    pub fn content(mut self, content: serde_json::Value) -> Self {
        self.request = self.request.content(content);
        self
    }

    /// 设置要更新的组件属性
    pub fn properties(mut self, properties: serde_json::Value) -> Self {
        self.request = self.request.properties(properties);
        self
    }

    /// 设置要更新的父组件ID
    pub fn parent_id(mut self, parent_id: impl Into<String>) -> Self {
        self.request = self.request.parent_id(parent_id);
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.request = self.request.user_id_type(user_id_type);
        self
    }

    /// 执行更新组件属性操作
    pub async fn execute(self, service: &CardElementService) -> SDKResult<PatchCardElementResponse> {
        service.patch_card_element(&self.request).await
    }
}

impl CardElementService {
    /// 更新组件属性构建器
    pub fn patch_card_element_builder(&self, card_id: impl Into<String>, element_id: impl Into<String>) -> PatchCardElementBuilder {
        PatchCardElementBuilder::new(card_id, element_id)
    }
}

// ==================== 单元测试 ====================

#[cfg(test)]
mod tests {
    use super::*;
    use config::Config;

    #[test]
    fn test_patch_card_element_request_creation() {
        let request = PatchCardElementRequest::new("card_123", "element_456");
        assert_eq!(request.card_id, "card_123");
        assert_eq!(request.element_id, "element_456");
        assert_eq!(request.element_type, None);
        assert_eq!(request.content, None);
        assert_eq!(request.properties, None);
        assert_eq!(request.parent_id, None);
        assert_eq!(request.user_id_type, None);
    }

    #[test]
    fn test_patch_card_element_request_with_fields() {
        let content = json!({
            "type": "plain_text",
            "content": "更新后的文本"
        });
        let properties = json!({
            "style": "secondary",
            "size": "small"
        });

        let request = PatchCardElementRequest::new("card_123", "element_456")
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
    fn test_patch_card_element_request_validation() {
        // 测试正常情况
        let valid_request = PatchCardElementRequest::new("card_123", "element_456")
            .properties(json!({"style": "primary"}));
        assert!(valid_request.validate().is_ok());

        // 测试空card_id
        let empty_card_request = PatchCardElementRequest::new("", "element_456")
            .properties(json!({"style": "primary"}));
        assert!(empty_card_request.validate().is_err());

        // 测试空element_id
        let empty_element_request = PatchCardElementRequest::new("card_123", "")
            .properties(json!({"style": "primary"}));
        assert!(empty_element_request.validate().is_err());

        // 测试没有更新字段
        let no_fields_request = PatchCardElementRequest::new("card_123", "element_456");
        assert!(no_fields_request.validate().is_err());

        // 测试不支持的组件类型
        let unsupported_request = PatchCardElementRequest::new("card_123", "element_456")
            .element_type("unsupported_type");
        assert!(unsupported_request.validate().is_err());

        // 测试null content
        let null_content_request = PatchCardElementRequest::new("card_123", "element_456")
            .content(json!(null));
        assert!(null_content_request.validate().is_err());

        // 测试null properties
        let null_properties_request = PatchCardElementRequest::new("card_123", "element_456")
            .properties(json!(null));
        assert!(null_properties_request.validate().is_err());
    }

    #[test]
    fn test_patch_card_element_build_patch_body() {
        // 测试只更新properties
        let properties_only = PatchCardElementRequest::new("card_123", "element_456")
            .properties(json!({"style": "primary"}));

        let body = properties_only.build_patch_body();
        assert_eq!(body.get("element_type"), None);
        assert_eq!(body.get("content"), None);
        assert_eq!(body["properties"], json!({"style": "primary"}));
        assert_eq!(body.get("parent_id"), None);

        // 测试更新多个字段
        let multi_field = PatchCardElementRequest::new("card_123", "element_456")
            .element_type("button")
            .content(json!({"text": "点击"}))
            .properties(json!({"disabled": false}))
            .parent_id("parent_789");

        let body = multi_field.build_patch_body();
        assert_eq!(body["element_type"], "button");
        assert_eq!(body["content"], json!({"text": "点击"}));
        assert_eq!(body["properties"], json!({"disabled": false}));
        assert_eq!(body["parent_id"], "parent_789");
    }

    #[test]
    fn test_patch_card_element_response_creation() {
        let element = CardElement {
            element_id: Some("element_456".to_string()),
            element_type: Some("text".to_string()),
            content: Some(json!({
                "type": "plain_text",
                "content": "更新后的文本"
            })),
            ..Default::default()
        };

        let response_data = PatchCardElementResponseData {
            element,
            updated_fields: Some(vec!["content".to_string(), "properties".to_string()]),
        };

        let response = PatchCardElementResponse {
            data: Some(response_data),
            success: true,
            ..Default::default()
        };

        assert!(response.success);
        assert!(response.data.is_some());
        assert_eq!(response.data.as_ref().unwrap().element.element_id, Some("element_456".to_string()));
        assert_eq!(response.data.as_ref().unwrap().updated_fields.as_ref().unwrap(),
                   vec!["content".to_string(), "properties".to_string()]);
    }

    #[test]
    fn test_patch_card_element_builder() {
        let content = json!({
            "type": "plain_text",
            "content": "构建器测试"
        });
        let properties = json!({
            "color": "#FF0000",
            "bold": true
        });

        let builder = PatchCardElementBuilder::new("card_123", "element_456")
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
    fn test_patch_card_element_builder_validation() {
        // 测试有效构建器
        let valid_builder = PatchCardElementBuilder::new("card_123", "element_456")
            .properties(json!({"style": "primary"}));
        assert!(valid_builder.request.validate().is_ok());

        // 测试无效构建器
        let invalid_builder = PatchCardElementBuilder::new("", "element_456")
            .properties(json!({"style": "primary"}));
        assert!(invalid_builder.request.validate().is_err());

        // 测试没有更新字段的构建器
        let no_fields_builder = PatchCardElementBuilder::new("card_123", "element_456");
        assert!(no_fields_builder.request.validate().is_err());

        // 测试不支持的组件类型
        let unsupported_builder = PatchCardElementBuilder::new("card_123", "element_456")
            .element_type("invalid_type");
        assert!(unsupported_builder.request.validate().is_err());
    }

    #[test]
    fn test_patch_card_element_service_method() {
        let config = Config::default();
        let service = CardElementService::new(config);

        // 验证服务包含所需的方法
        let service_str = format!("{:?}", service);
        assert!(!service_str.is_empty());

        // 验证构建器方法存在
        let builder = service.patch_card_element_builder("card_123", "element_456");
        assert_eq!(builder.request.card_id, "card_123");
        assert_eq!(builder.request.element_id, "element_456");
    }

    #[test]
    fn test_patch_card_element_endpoint_construction() {
        // 验证端点常量存在
        assert_eq!(
            crate::core::endpoints_original::Endpoints::CARDKIT_V1_CARD_ELEMENTS_PATCH,
            "/open-apis/cardkit/v1/cards/{card_id}/elements/{element_id}"
        );

        // 验证路径替换逻辑
        let template = crate::core::endpoints_original::Endpoints::CARDKIT_V1_CARD_ELEMENTS_PATCH;
        let final_path = template
            .replace("{card_id}", "card_123")
            .replace("{element_id}", "element_456");
        assert_eq!(final_path, "/open-apis/cardkit/v1/cards/card_123/elements/element_456");
    }

    #[test]
    fn test_patch_card_element_json_serialization() {
        let request = PatchCardElementRequest::new("card_123", "element_456")
            .element_type("text")
            .content(json!({"type": "plain_text", "content": "序列化测试"}))
            .properties(json!({"color": "blue", "size": 16}))
            .user_id_type(UserIdType::OpenId);

        // 测试请求体构建
        let body = request.build_patch_body();

        assert_eq!(body["element_type"], "text");
        assert_eq!(body["content"]["type"], "plain_text");
        assert_eq!(body["content"]["content"], "序列化测试");
        assert_eq!(body["properties"]["color"], "blue");
        assert_eq!(body["properties"]["size"], 16);
        assert_eq!(body.get("parent_id"), None); // 没有设置的字段不应该出现
    }

    #[test]
    fn test_patch_card_element_response_trait() {
        assert_eq!(PatchCardElementResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_patch_card_element_comprehensive_scenario() {
        // 测试完整的业务场景 - 部分更新按钮组件的特定属性
        let request = PatchCardElementRequest::new("card_comprehensive_001", "btn_submit")
            .properties(json!({
                "style": "primary",
                "size": "large",
                "disabled": false,
                "loading": false
            }))
            .user_id_type(UserIdType::UnionId);

        assert!(request.validate().is_ok());
        assert_eq!(request.card_id, "card_comprehensive_001");
        assert_eq!(request.element_id, "btn_submit");
        assert!(request.properties.is_some());
        assert_eq!(request.user_id_type, Some(UserIdType::UnionId));

        // 验证请求体只包含要更新的字段
        let body = request.build_patch_body();
        assert!(body.get("element_type").is_none());
        assert!(body.get("content").is_none());
        assert!(body.get("properties").is_some());
        assert!(body.get("parent_id").is_none());
    }

    #[test]
    fn test_patch_card_element_partial_updates() {
        // 测试各种部分更新组合

        // 1. 只更新类型
        let type_only = PatchCardElementRequest::new("card_001", "element_001")
            .element_type("markdown");
        assert!(type_only.validate().is_ok());

        // 2. 只更新内容
        let content_only = PatchCardElementRequest::new("card_001", "element_001")
            .content(json!({"text": "新内容"}));
        assert!(content_only.validate().is_ok());

        // 3. 只更新属性
        let properties_only = PatchCardElementRequest::new("card_001", "element_001")
            .properties(json!({"style": "secondary"}));
        assert!(properties_only.validate().is_ok());

        // 4. 只更新父组件
        let parent_only = PatchCardElementRequest::new("card_001", "element_001")
            .parent_id("new_parent_001");
        assert!(parent_only.validate().is_ok());

        // 5. 更新所有字段
        let all_fields = PatchCardElementRequest::new("card_001", "element_001")
            .element_type("button")
            .content(json!({"text": "完整按钮"}))
            .properties(json!({"style": "danger"}))
            .parent_id("container_001");
        assert!(all_fields.validate().is_ok());
    }

    #[test]
    fn test_patch_card_element_edge_cases() {
        // 测试特殊字符和边界情况
        let special_content = json!({
            "text": "特殊字符测试 🎉✨",
            "unicode": "测试中文字符",
            "symbols": "@#$%^&*()_+-=[]{}|;':\",./<>?"
        });

        let special_request = PatchCardElementRequest::new("card_special_001", "element_special_001")
            .content(special_content)
            .properties(json!({
                "css_classes": ["btn", "btn-primary", "hover-effect"],
                "attributes": {
                    "data-testid": "submit-button",
                    "aria-label": "提交表单"
                }
            }));

        assert!(special_request.validate().is_ok());
        assert!(special_request.content.is_some());
        assert!(special_request.properties.is_some());

        // 测试空对象和空数组
        let empty_structures = PatchCardElementRequest::new("card_empty_001", "element_empty_001")
            .properties(json!({
                "empty_object": {},
                "empty_array": [],
                "null_values": [null, null, null]
            }));

        assert!(empty_structures.validate().is_ok());
        let body = empty_structures.build_patch_body();
        assert_eq!(body["properties"]["empty_object"], json!({}));
        assert_eq!(body["properties"]["empty_array"], json!([]));
        assert_eq!(body["properties"]["null_values"], json!([null, null, null]));
    }

    #[test]
    fn test_patch_card_element_builder_pattern() {
        // 测试构建器模式的流畅性
        let builder = PatchCardElementBuilder::new("test_card", "test_element")
            .element_type("div")
            .properties(json!({
                "theme": "dark",
                "scrollable": true
            }))
            .user_id_type(UserIdType::OpenId);

        // 验证构建器状态
        assert_eq!(builder.request.card_id, "test_card");
        assert_eq!(builder.request.element_id, "test_element");
        assert_eq!(builder.request.element_type, Some("div".to_string()));
        assert!(builder.request.properties.is_some());
        assert_eq!(builder.request.user_id_type, Some(UserIdType::OpenId));

        // 验证请求验证通过
        assert!(builder.request.validate().is_ok());

        // 测试链式调用
        let chained_builder = builder
            .content(json!({"type": "markdown", "content": "重新设置内容"}))
            .request;
        assert_eq!(chained_builder.content["content"], "重新设置内容");

        // 验证之前的字段仍然存在
        assert_eq!(chained_builder.element_type, Some("div".to_string()));
        assert!(chained_builder.properties.is_some());
    }

    #[test]
    fn test_patch_card_element_different_element_types() {
        // 测试不同类型的组件部分更新
        let element_types = ["text", "image", "button", "div", "input", "select"];

        for element_type in &element_types {
            let request = PatchCardElementRequest::new("card_test_001", "element_test_001")
                .element_type(*element_type)
                .properties(json!({"updated": true}));

            assert!(request.validate().is_ok(), "Element type {} should be valid", element_type);
            assert_eq!(request.element_type, Some(element_type.to_string()));
        }
    }

    #[test]
    fn test_patch_card_element_vs_full_update() {
        // 对比部分更新和全量更新的区别
        let card_id = "card_comparison_001";
        let element_id = "element_comparison_001";

        // 部分更新 - 只更新特定字段
        let patch_request = PatchCardElementRequest::new(card_id, element_id)
            .properties(json!({"style": "primary"}));

        let patch_body = patch_request.build_patch_body();
        assert_eq!(patch_body.keys().count(), 1); // 只有properties字段
        assert!(patch_body.get("element_type").is_none());
        assert!(patch_body.get("content").is_none());
        assert!(patch_body.get("parent_id").is_none());

        // 验证请求验证
        assert!(patch_request.validate().is_ok());

        // 验证路径构建
        let api_path = crate::core::endpoints_original::Endpoints::CARDKIT_V1_CARD_ELEMENTS_PATCH
            .replace("{card_id}", card_id)
            .replace("{element_id}", element_id);
        assert_eq!(api_path, "/open-apis/cardkit/v1/cards/card_comparison_001/elements/element_comparison_001");
    }

    #[test]
    fn test_patch_card_element_validation_edge_cases() {
        // 测试验证逻辑的边界情况

        // 1. 空白字符的card_id和element_id
        let whitespace_request = PatchCardElementRequest::new("   ", "\t\n")
            .properties(json!({"test": true}));
        assert!(whitespace_request.validate().is_err());

        // 2. 空白字符的element_type
        let empty_type_request = PatchCardElementRequest::new("card_001", "element_001")
            .element_type("   \t\n");
        assert!(empty_type_request.validate().is_err());

        // 3. 混合有效和无效字段
        let mixed_request = PatchCardElementRequest::new("card_001", "element_001")
            .element_type("valid_type")
            .properties(json!(null)); // 无效的properties
        assert!(mixed_request.validate().is_err());

        // 4. 最小有效请求
        let minimal_request = PatchCardElementRequest::new("c", "e")
            .element_type("text");
        assert!(minimal_request.validate().is_ok());

        // 5. 最大有效请求（所有字段）
        let maximal_request = PatchCardElementRequest::new("card_max_001", "element_max_001")
            .element_type("div")
            .content(json!({"type": "markdown", "content": "完整内容"}))
            .properties(json!({"style": "primary", "size": "large", "disabled": false}))
            .parent_id("parent_max_001")
            .user_id_type(UserIdType::UnionId);
        assert!(maximal_request.validate().is_ok());
    }
}