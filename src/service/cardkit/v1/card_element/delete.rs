//! CardKit v1 删除组件API
//!
//! 提供删除卡片中指定组件的功能，支持完整的组件生命周期管理

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

/// 删除组件请求
#[derive(Debug, Clone)]
pub struct DeleteCardElementRequest {
    /// 卡片ID
    pub card_id: String,
    /// 组件ID
    pub element_id: String,
    /// 用户ID类型
    pub user_id_type: Option<UserIdType>,
}

impl DeleteCardElementRequest {
    /// 创建新的请求实例
    pub fn new(card_id: impl Into<String>, element_id: impl Into<String>) -> Self {
        Self {
            card_id: card_id.into(),
            element_id: element_id.into(),
            user_id_type: None,
        }
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

        // 验证ID格式的合理性
        if self.card_id.len() > 100 {
            return Err("card_id长度不能超过100个字符".to_string());
        }

        if self.element_id.len() > 100 {
            return Err("element_id长度不能超过100个字符".to_string());
        }

        // 验证字符安全性
        let allowed_chars = |s: &str| {
            s.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-' || c == '.')
        };

        if !allowed_chars(&self.card_id) {
            return Err("card_id包含不支持的字符，只允许字母、数字、下划线、连字符和点".to_string());
        }

        if !allowed_chars(&self.element_id) {
            return Err("element_id包含不支持的字符，只允许字母、数字、下划线、连字符和点".to_string());
        }

        Ok(())
    }
}

/// 删除组件响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteCardElementResponseData {
    /// 是否成功删除
    pub deleted: bool,
    /// 被删除的组件ID
    pub element_id: String,
    /// 卡片ID
    pub card_id: String,
    /// 删除时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_time: Option<i64>,
}

/// 删除组件响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeleteCardElementResponse {
    /// 响应数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<DeleteCardElementResponseData>,
    /// 是否成功
    pub success: bool,
    /// 错误消息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// 错误代码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
}

impl ApiResponseTrait for DeleteCardElementResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl CardElementService {
    /// 删除组件
    ///
    /// 删除指定卡片中的组件，支持完整的组件生命周期管理
    ///
    /// # 参数
    /// * `req` - 删除组件请求
    ///
    /// # 返回值
    /// 返回删除操作的结果信息
    ///
    /// # 示例
    /// ```rust
    /// use open_lark::service::cardkit::v1::card_element::delete::{DeleteCardElementRequest, UserIdType};
    ///
    /// let request = DeleteCardElementRequest::new("card_123", "element_456")
    ///     .user_id_type(UserIdType::OpenId);
    /// let response = service.delete_card_element(&request).await?;
    /// ```
    pub async fn delete_card_element(&self, req: &DeleteCardElementRequest) -> SDKResult<DeleteCardElementResponse> {
        req.validate()?;
        log::debug!("开始删除组件: card_id={}, element_id={}", req.card_id, req.element_id);

        // 构建查询参数
        let mut query_params: HashMap<&str, String> = HashMap::new();
        if let Some(user_id_type) = &req.user_id_type {
            query_params.insert("user_id_type", user_id_type.to_string());
        }

        // 构建API路径，替换card_id和element_id占位符
        let api_path = crate::core::endpoints_original::Endpoints::CARDKIT_V1_CARD_ELEMENTS_DELETE
            .replace("{card_id}", &req.card_id)
            .replace("{element_id}", &req.element_id);

        let api_req = ApiRequest {
            http_method: reqwest::Method::DELETE,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: Vec::new(), // DELETE请求通常不需要请求体
            ..Default::default()
        };

        let resp = Transport::<DeleteCardElementResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        if response.success {
            log::info!("组件删除成功: card_id={}, element_id={}", req.card_id, req.element_id);
        } else {
            log::warn!("组件删除失败: card_id={}, element_id={}, error={:?}",
                req.card_id, req.element_id, response.error_message);
        }

        Ok(response)
    }
}

// ==================== 构建器模式 ====================

/// 删除组件构建器
#[derive(Debug, Clone)]
pub struct DeleteCardElementBuilder {
    request: DeleteCardElementRequest,
}

impl DeleteCardElementBuilder {
    /// 创建新的构建器
    pub fn new(card_id: impl Into<String>, element_id: impl Into<String>) -> Self {
        Self {
            request: DeleteCardElementRequest::new(card_id, element_id),
        }
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.request = self.request.user_id_type(user_id_type);
        self
    }

    /// 执行删除组件操作
    pub async fn execute(self, service: &CardElementService) -> SDKResult<DeleteCardElementResponse> {
        service.delete_card_element(&self.request).await
    }
}

impl CardElementService {
    /// 删除组件构建器
    pub fn delete_card_element_builder(&self, card_id: impl Into<String>, element_id: impl Into<String>) -> DeleteCardElementBuilder {
        DeleteCardElementBuilder::new(card_id, element_id)
    }
}

// ==================== 单元测试 ====================

#[cfg(test)]
mod tests {
    use super::*;
    use config::Config;

    #[test]
    fn test_delete_card_element_request_creation() {
        let request = DeleteCardElementRequest::new("card_123", "element_456");
        assert_eq!(request.card_id, "card_123");
        assert_eq!(request.element_id, "element_456");
        assert_eq!(request.user_id_type, None);
    }

    #[test]
    fn test_delete_card_element_request_with_fields() {
        let request = DeleteCardElementRequest::new("card_123", "element_456")
            .user_id_type(UserIdType::OpenId);

        assert_eq!(request.card_id, "card_123");
        assert_eq!(request.element_id, "element_456");
        assert_eq!(request.user_id_type, Some(UserIdType::OpenId));
    }

    #[test]
    fn test_delete_card_element_request_validation() {
        // 测试正常情况
        let valid_request = DeleteCardElementRequest::new("card_123", "element_456");
        assert!(valid_request.validate().is_ok());

        // 测试空card_id
        let empty_card_request = DeleteCardElementRequest::new("", "element_456");
        assert!(empty_card_request.validate().is_err());

        // 测试空element_id
        let empty_element_request = DeleteCardElementRequest::new("card_123", "");
        assert!(empty_element_request.validate().is_err());

        // 测试空白字符card_id
        let whitespace_card_request = DeleteCardElementRequest::new("   ", "element_456");
        assert!(whitespace_card_request.validate().is_err());

        // 测试空白字符element_id
        let whitespace_element_request = DeleteCardElementRequest::new("card_123", "\t\n");
        assert!(whitespace_element_request.validate().is_err());

        // 测试超长card_id
        let long_card_request = DeleteCardElementRequest::new(&"a".repeat(101), "element_456");
        assert!(long_card_request.validate().is_err());

        // 测试超长element_id
        let long_element_request = DeleteCardElementRequest::new("card_123", &"b".repeat(101));
        assert!(long_element_request.validate().is_err());

        // 测试无效字符card_id
        let invalid_char_request = DeleteCardElementRequest::new("card@123", "element_456");
        assert!(invalid_char_request.validate().is_err());

        // 测试无效字符element_id
        let invalid_element_request = DeleteCardElementRequest::new("card_123", "element#456");
        assert!(invalid_element_request.validate().is_err());

        // 测试有效特殊字符
        let valid_special_request = DeleteCardElementRequest::new("card-123_456.test", "element-456_789.test");
        assert!(valid_special_request.validate().is_ok());
    }

    #[test]
    fn test_delete_card_element_response_creation() {
        let response_data = DeleteCardElementResponseData {
            deleted: true,
            element_id: "element_456".to_string(),
            card_id: "card_123".to_string(),
            delete_time: Some(1699123456),
        };

        let response = DeleteCardElementResponse {
            data: Some(response_data),
            success: true,
            ..Default::default()
        };

        assert!(response.success);
        assert!(response.data.is_some());
        assert_eq!(response.data.as_ref().unwrap().element_id, "element_456");
        assert_eq!(response.data.as_ref().unwrap().card_id, "card_123");
        assert_eq!(response.data.as_ref().unwrap().deleted, true);
        assert_eq!(response.data.as_ref().unwrap().delete_time, Some(1699123456));
    }

    #[test]
    fn test_delete_card_element_builder() {
        let builder = DeleteCardElementBuilder::new("card_123", "element_456")
            .user_id_type(UserIdType::UserId);

        assert_eq!(builder.request.card_id, "card_123");
        assert_eq!(builder.request.element_id, "element_456");
        assert_eq!(builder.request.user_id_type, Some(UserIdType::UserId));
    }

    #[test]
    fn test_delete_card_element_builder_validation() {
        // 测试有效构建器
        let valid_builder = DeleteCardElementBuilder::new("card_123", "element_456");
        assert!(valid_builder.request.validate().is_ok());

        // 测试无效构建器
        let invalid_builder = DeleteCardElementBuilder::new("", "element_456");
        assert!(invalid_builder.request.validate().is_err());

        // 测试无效element_id构建器
        let invalid_element_builder = DeleteCardElementBuilder::new("card_123", "");
        assert!(invalid_element_builder.request.validate().is_err());
    }

    #[test]
    fn test_delete_card_element_service_method() {
        let config = Config::default();
        let service = CardElementService::new(config);

        // 验证服务包含所需的方法
        let service_str = format!("{:?}", service);
        assert!(!service_str.is_empty());

        // 验证构建器方法存在
        let builder = service.delete_card_element_builder("card_123", "element_456");
        assert_eq!(builder.request.card_id, "card_123");
        assert_eq!(builder.request.element_id, "element_456");
    }

    #[test]
    fn test_delete_card_element_endpoint_construction() {
        // 验证端点常量存在
        assert_eq!(
            crate::core::endpoints_original::Endpoints::CARDKIT_V1_CARD_ELEMENTS_DELETE,
            "/open-apis/cardkit/v1/cards/{card_id}/elements/{element_id}"
        );

        // 验证路径替换逻辑
        let template = crate::core::endpoints_original::Endpoints::CARDKIT_V1_CARD_ELEMENTS_DELETE;
        let final_path = template
            .replace("{card_id}", "card_123")
            .replace("{element_id}", "element_456");
        assert_eq!(final_path, "/open-apis/cardkit/v1/cards/card_123/elements/element_456");
    }

    #[test]
    fn test_delete_card_element_response_trait() {
        assert_eq!(DeleteCardElementResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_delete_card_element_comprehensive_scenario() {
        // 测试完整的业务场景 - 删除一个组件
        let request = DeleteCardElementRequest::new("card_comprehensive_001", "btn_submit")
            .user_id_type(UserIdType::UnionId);

        assert!(request.validate().is_ok());
        assert_eq!(request.card_id, "card_comprehensive_001");
        assert_eq!(request.element_id, "btn_submit");
        assert_eq!(request.user_id_type, Some(UserIdType::UnionId));

        // 验证请求参数合理性
        assert!(request.card_id.len() < 100);
        assert!(request.element_id.len() < 100);
    }

    #[test]
    fn test_delete_card_element_different_id_types() {
        // 测试不同类型的ID
        let test_cases = vec![
            ("card_1", "element_1"),
            ("card-with-dashes", "element-with-dashes"),
            ("card_with_underscores", "element_with_underscores"),
            ("card.with.dots", "element.with.dots"),
            ("c1", "e1"),
            ("card123", "element456"),
        ];

        for (card_id, element_id) in test_cases {
            let request = DeleteCardElementRequest::new(card_id, element_id);
            assert!(request.validate().is_ok(),
                "Valid IDs should pass validation: card_id={}, element_id={}", card_id, element_id);
            assert_eq!(request.card_id, card_id);
            assert_eq!(request.element_id, element_id);
        }
    }

    #[test]
    fn test_delete_card_element_edge_cases() {
        // 测试边界情况

        // 1. 最小有效ID
        let minimal_request = DeleteCardElementRequest::new("c", "e");
        assert!(minimal_request.validate().is_ok());

        // 2. 最大有效ID长度
        let max_card_id = "c".repeat(100);
        let max_element_id = "e".repeat(100);
        let max_request = DeleteCardElementRequest::new(&max_card_id, &max_element_id);
        assert!(max_request.validate().is_ok());

        // 3. 混合字符ID
        let mixed_request = DeleteCardElementRequest::new("Card-123_456.Test", "Element-789_012.Test");
        assert!(mixed_request.validate().is_ok());

        // 4. 数字ID
        let numeric_request = DeleteCardElementRequest::new("123456", "789012");
        assert!(numeric_request.validate().is_ok());

        // 5. 单字符ID
        let single_char_request = DeleteCardElementRequest::new("a", "b");
        assert!(single_char_request.validate().is_ok());
    }

    #[test]
    fn test_delete_card_element_builder_pattern() {
        // 测试构建器模式的流畅性
        let builder = DeleteCardElementBuilder::new("test_card", "test_element")
            .user_id_type(UserIdType::OpenId);

        // 验证构建器状态
        assert_eq!(builder.request.card_id, "test_card");
        assert_eq!(builder.request.element_id, "test_element");
        assert_eq!(builder.request.user_id_type, Some(UserIdType::OpenId));

        // 验证请求验证通过
        assert!(builder.request.validate().is_ok());

        // 测试链式调用
        let chained_builder = builder
            .user_id_type(UserIdType::UserId)
            .request;
        assert_eq!(chained_builder.user_id_type, Some(UserIdType::UserId));
    }

    #[test]
    fn test_delete_card_element_security_validation() {
        // 测试安全性验证

        // 1. SQL注入测试
        let sql_injection_request = DeleteCardElementRequest::new("card'; DROP TABLE cards; --", "element_456");
        assert!(sql_injection_request.validate().is_err());

        // 2. XSS测试
        let xss_request = DeleteCardElementRequest::new("card<script>alert('xss')</script>", "element_456");
        assert!(xss_request.validate().is_err());

        // 3. 路径遍历测试
        let path_traversal_request = DeleteCardElementRequest::new("../../../etc/passwd", "element_456");
        assert!(path_traversal_request.validate().is_err());

        // 4. 命令注入测试
        let command_injection_request = DeleteCardElementRequest::new("card; rm -rf /", "element_456");
        assert!(command_injection_request.validate().is_err());

        // 5. 空字节注入测试
        let null_byte_request = DeleteCardElementRequest::new("card\x00malicious", "element_456");
        // 注意：Rust字符串处理会自动处理空字节，这里主要测试字符安全性
        assert!(null_byte_request.validate().is_err());
    }

    #[test]
    fn test_delete_card_element_user_id_type_variations() {
        // 测试不同用户ID类型
        let user_id_types = vec![
            UserIdType::OpenId,
            UserIdType::UserId,
            UserIdType::UnionId,
        ];

        for user_id_type in user_id_types {
            let request = DeleteCardElementRequest::new("card_123", "element_456")
                .user_id_type(user_id_type);

            assert!(request.validate().is_ok());
            assert_eq!(request.user_id_type, Some(user_id_type));
        }
    }

    #[test]
    fn test_delete_card_element_response_with_error() {
        // 测试错误响应
        let error_response = DeleteCardElementResponse {
            data: None,
            success: false,
            error_message: Some("组件不存在".to_string()),
            error_code: Some("ELEMENT_NOT_FOUND".to_string()),
        };

        assert!(!error_response.success);
        assert!(error_response.data.is_none());
        assert_eq!(error_response.error_message, Some("组件不存在".to_string()));
        assert_eq!(error_response.error_code, Some("ELEMENT_NOT_FOUND".to_string()));
    }

    #[test]
    fn test_delete_card_element_api_consistency() {
        // 测试API一致性 - 确保与其他CardKit API保持一致的模式

        // 1. 请求结构一致性
        let request = DeleteCardElementRequest::new("card_123", "element_456")
            .user_id_type(UserIdType::OpenId);

        // 验证基本字段
        assert!(!request.card_id.is_empty());
        assert!(!request.element_id.is_empty());

        // 2. 响应结构一致性
        let response = DeleteCardElementResponse {
            data: Some(DeleteCardElementResponseData {
                deleted: true,
                element_id: "element_456".to_string(),
                card_id: "card_123".to_string(),
                delete_time: None,
            }),
            success: true,
            error_message: None,
            error_code: None,
        };

        assert!(response.success);
        assert!(response.data.is_some());

        // 3. 验证与现有API模式一致
        assert!(request.validate().is_ok()); // 所有CardKit API都应该有验证
        assert_eq!(DeleteCardElementResponse::data_format(), ResponseFormat::Data); // 统一的数据格式
    }

    #[test]
    fn test_delete_card_element_performance_considerations() {
        // 测试性能考虑的边界情况

        // 1. 大量并发删除的ID格式（模拟实际使用场景）
        let batch_requests: Vec<_> = (0..1000)
            .map(|i| DeleteCardElementRequest::new(
                format!("card_batch_{}", i),
                format!("element_batch_{}", i)
            ))
            .collect();

        // 验证所有请求都能正确验证
        for (i, request) in batch_requests.iter().enumerate() {
            assert!(request.validate().is_ok(),
                "Batch request {} should be valid", i);
        }

        // 2. 长ID处理性能
        let long_id_request = DeleteCardElementRequest::new(
            &"very_long_card_id_for_performance_testing_purposes_123456789".repeat(2),
            "element_456"
        );
        // 应该被长度验证拒绝
        assert!(long_id_request.validate().is_err());

        // 3. 特殊字符处理性能
        let special_chars = "!@#$%^&*()[]{}|\\:;\"'<>?,/~`";
        for char in special_chars.chars() {
            let invalid_request = DeleteCardElementRequest::new(
                format!("card{}invalid", char),
                "element_456"
            );
            assert!(invalid_request.validate().is_err(),
                "Invalid char '{}' should be rejected", char);
        }
    }

    #[test]
    fn test_delete_card_element_real_world_scenarios() {
        // 测试真实世界的使用场景

        // 1. 用户界面组件删除
        let ui_request = DeleteCardElementRequest::new("user_interface_card_001", "submit_button")
            .user_id_type(UserIdType::OpenId);
        assert!(ui_request.validate().is_ok());

        // 2. 表单字段删除
        let form_request = DeleteCardElementRequest::new("survey_form_2024", "question_5")
            .user_id_type(UserIdType::UserId);
        assert!(form_request.validate().is_ok());

        // 3. 媒体组件删除
        let media_request = DeleteCardElementRequest::new("media_gallery_001", "image_42")
            .user_id_type(UserIdType::UnionId);
        assert!(media_request.validate().is_ok());

        // 4. 容器组件删除
        let container_request = DeleteCardElementRequest::new("layout_container", "section_header")
            .user_id_type(UserIdType::OpenId);
        assert!(container_request.validate().is_ok());

        // 5. 临时组件删除
        let temp_request = DeleteCardElementRequest::new("temp_card_draft", "placeholder_element")
            .user_id_type(UserIdType::UserId);
        assert!(temp_request.validate().is_ok());
    }

    #[test]
    fn test_delete_card_element_complete_lifecycle() {
        // 测试完整的组件生命周期管理

        // 创建场景：先创建，然后删除
        let card_id = "lifecycle_test_card";
        let element_ids = vec!["element_1", "element_2", "element_3"];

        // 模拟删除所有组件
        for element_id in element_ids {
            let delete_request = DeleteCardElementRequest::new(card_id, element_id)
                .user_id_type(UserIdType::OpenId);

            assert!(delete_request.validate().is_ok());
            assert_eq!(delete_request.card_id, card_id);
            assert_eq!(delete_request.element_id, element_id);

            // 验证删除请求的合理性
            assert!(delete_request.card_id.len() < 100);
            assert!(delete_request.element_id.len() < 100);
        }

        // 验证最后一个组件删除
        let final_request = DeleteCardElementRequest::new(card_id, "final_element")
            .user_id_type(UserIdType::OpenId);
        assert!(final_request.validate().is_ok());
    }

    #[test]
    fn test_delete_card_element_integration_readiness() {
        // 测试集成就绪性

        // 1. 端点集成
        let api_path = crate::core::endpoints_original::Endpoints::CARDKIT_V1_CARD_ELEMENTS_DELETE
            .replace("{card_id}", "test_card")
            .replace("{element_id}", "test_element");
        assert_eq!(api_path, "/open-apis/cardkit/v1/cards/test_card/elements/test_element");

        // 2. 服务集成
        let config = Config::default();
        let service = CardElementService::new(config);
        let builder = service.delete_card_element_builder("test_card", "test_element");
        assert!(builder.request.validate().is_ok());

        // 3. 构建器集成
        let chained_builder = builder.user_id_type(UserIdType::OpenId);
        assert!(chained_builder.request.validate().is_ok());

        // 4. 响应集成
        let response = DeleteCardElementResponse {
            data: Some(DeleteCardElementResponseData {
                deleted: true,
                element_id: "test_element".to_string(),
                card_id: "test_card".to_string(),
                delete_time: Some(1699123456),
            }),
            success: true,
            ..Default::default()
        };
        assert!(response.success);
        assert!(response.data.is_some());
    }
}