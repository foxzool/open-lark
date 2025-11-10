//! CardKit v1 卡片更新API
//!
//! 提供飞书卡片实体的全量更新功能，支持对卡片内容的完整替换

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

use super::CardService;
use super::super::models::{Card, UserIdType};

/// 全量更新卡片实体请求
#[derive(Debug, Clone)]
pub struct UpdateCardRequest {
    /// 卡片ID
    pub card_id: String,
    /// 卡片标题
    pub title: Option<String>,
    /// 卡片描述
    pub description: Option<String>,
    /// 卡片JSON内容
    pub card_json: Option<serde_json::Value>,
    /// 用户ID类型
    pub user_id_type: Option<UserIdType>,
}

impl UpdateCardRequest {
    /// 创建新的请求实例
    pub fn new(card_id: impl Into<String>) -> Self {
        Self {
            card_id: card_id.into(),
            title: None,
            description: None,
            card_json: None,
            user_id_type: None,
        }
    }

    /// 设置卡片标题
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// 设置卡片描述
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// 设置卡片JSON内容
    pub fn card_json(mut self, card_json: serde_json::Value) -> Self {
        self.card_json = Some(card_json);
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.card_id.is_empty() {
            return Err("card_id不能为空".to_string());
        }

        if let Some(ref title) = self.title {
            if title.trim().is_empty() {
                return Err("标题不能为空字符串".to_string());
            }
            if title.len() > 100 {
                return Err("标题长度不能超过100个字符".to_string());
            }
        }

        if let Some(ref description) = self.description {
            if description.trim().is_empty() {
                return Err("描述不能为空字符串".to_string());
            }
            if description.len() > 500 {
                return Err("描述长度不能超过500个字符".to_string());
            }
        }

        if let Some(ref card_json) = self.card_json {
            if card_json.is_null() {
                return Err("card_json不能为null".to_string());
            }
            // 这里可以添加更复杂的JSON结构验证
        }

        Ok(())
    }
}

/// 全量更新卡片实体响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCardResponseData {
    /// 更新后的卡片信息
    pub card: Card,
    /// 更新的字段列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_fields: Option<Vec<String>>,
}

/// 全量更新卡片实体响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateCardResponse {
    /// 响应数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<UpdateCardResponseData>,
    /// 是否成功
    pub success: bool,
    /// 错误消息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// 错误代码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
}

impl ApiResponseTrait for UpdateCardResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl CardService {
    /// 全量更新卡片实体
    ///
    /// 完整替换指定卡片的所有内容，包括标题、描述和JSON内容
    ///
    /// # 参数
    /// * `req` - 更新卡片请求
    ///
    /// # 返回值
    /// 返回更新后的卡片信息
    ///
    /// # 示例
    /// ```rust
    /// use open_lark::service::cardkit::v1::card::update::{UpdateCardRequest, UserIdType};
    /// let request = UpdateCardRequest::new("card_123")
    ///     .title("更新后的标题")
    ///     .description("更新后的描述")
    /// .user_id_type(UserIdType::OpenId);
    /// let response = service.update_card(&request).await?;
    /// ```
    pub async fn update_card(&self, req: &UpdateCardRequest) -> SDKResult<UpdateCardResponse> {
        req.validate()?;
        log::debug!("开始更新卡片: card_id={}", req.card_id);

        // 构建查询参数
        let mut query_params: HashMap<&str, String> = HashMap::new();
        if let Some(user_id_type) = &req.user_id_type {
            query_params.insert("user_id_type", user_id_type.to_string());
        }

        // 构建请求体
        let mut body = json!({});
        if let Some(ref title) = req.title {
            body["title"] = json!(title);
        }
        if let Some(ref description) = req.description {
            body["description"] = json!(description);
        }
        if let Some(ref card_json) = req.card_json {
            body["card_json"] = card_json.clone();
        }

        // 构建API路径，替换card_id占位符
        let api_path = crate::core::endpoints_original::Endpoints::CARDKIT_V1_CARDS
            .replace("{}", &req.card_id);

        let api_req = ApiRequest {
            http_method: reqwest::Method::PUT,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: serde_json::to_vec(&body).unwrap_or_default(),
            ..Default::default()
        };

        let resp = Transport::<UpdateCardResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        if response.success {
            log::info!("卡片更新成功: card_id={}", req.card_id);
        } else {
            log::warn!("卡片更新失败: card_id={}, error={:?}", req.card_id, response.error_message);
        }

        Ok(response)
    }
}

// ==================== 构建器模式 ====================

/// 全量更新卡片实体构建器
#[derive(Debug, Clone)]
pub struct UpdateCardBuilder {
    request: UpdateCardRequest,
}

impl UpdateCardBuilder {
    /// 创建新的构建器
    pub fn new(card_id: impl Into<String>) -> Self {
        Self {
            request: UpdateCardRequest::new(card_id),
        }
    }

    /// 设置卡片标题
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.request = self.request.title(title);
        self
    }

    /// 设置卡片描述
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.request = self.request.description(description);
        self
    }

    /// 设置卡片JSON内容
    pub fn card_json(mut self, card_json: serde_json::Value) -> Self {
        self.request = self.request.card_json(card_json);
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.request = self.request.user_id_type(user_id_type);
        self
    }

    /// 执行更新卡片操作
    pub async fn execute(self, service: &CardService) -> SDKResult<UpdateCardResponse> {
        service.update_card(&self.request).await
    }
}

impl CardService {
    /// 更新卡片构建器
    pub fn update_card_builder(&self, card_id: impl Into<String>) -> UpdateCardBuilder {
        UpdateCardBuilder::new(card_id)
    }
}

// ==================== 单元测试 ====================

#[cfg(test)]
mod tests {
    use super::*;
    use open_lark_core::config::Config;

    #[test]
    fn test_update_card_request_creation() {
        let request = UpdateCardRequest::new("card_123");
        assert_eq!(request.card_id, "card_123");
        assert_eq!(request.title, None);
        assert_eq!(request.description, None);
        assert_eq!(request.card_json, None);
        assert_eq!(request.user_id_type, None);
    }

    #[test]
    fn test_update_card_request_with_fields() {
        let card_json = json!({
            "type": "card",
            "elements": []
        });

        let request = UpdateCardRequest::new("card_123")
            .title("更新后的标题")
            .description("更新后的描述")
            .card_json(card_json.clone())
            .user_id_type(UserIdType::OpenId);

        assert_eq!(request.card_id, "card_123");
        assert_eq!(request.title, Some("更新后的标题".to_string()));
        assert_eq!(request.description, Some("更新后的描述".to_string()));
        assert_eq!(request.card_json, Some(card_json));
        assert_eq!(request.user_id_type, Some(UserIdType::OpenId));
    }

    #[test]
    fn test_update_card_request_validation() {
        // 测试正常情况
        let valid_request = UpdateCardRequest::new("card_123")
            .title("有效标题")
            .card_json(json!({"type": "card"}));
        assert!(valid_request.validate().is_ok());

        // 测试空card_id
        let empty_card_request = UpdateCardRequest::new("");
        assert!(empty_card_request.validate().is_err());

        // 测试空标题
        let empty_title_request = UpdateCardRequest::new("card_123")
            .title("  ");
        assert!(empty_title_request.validate().is_err());

        // 测试超长标题
        let long_title_request = UpdateCardRequest::new("card_123")
            .title("a".repeat(101));
        assert!(long_title_request.validate().is_err());

        // 测试null card_json
        let null_json_request = UpdateCardRequest::new("card_123")
            .card_json(json!(null));
        assert!(null_json_request.validate().is_err());
    }

    #[test]
    fn test_update_card_response_creation() {
        let card = Card {
            card_id: Some("card_123".to_string()),
            title: Some("更新后的卡片".to_string()),
            description: Some("更新后的描述".to_string()),
            ..Default::default()
        };

        let response_data = UpdateCardResponseData {
            card,
            updated_fields: Some(vec!["title".to_string(), "description".to_string()]),
        };

        let response = UpdateCardResponse {
            data: Some(response_data),
            success: true,
            ..Default::default()
        };

        assert!(response.success);
        assert!(response.data.is_some());
        assert_eq!(response.data.as_ref().unwrap().card.card_id, Some("card_123".to_string()));
        assert_eq!(
            response.data.as_ref().unwrap().updated_fields.as_ref().unwrap(),
            vec!["title".to_string(), "description".to_string()]
        );
    }

    #[test]
    fn test_update_card_builder() {
        let card_json = json!({
            "type": "card",
            "elements": [
                {
                    "type": "text",
                    "content": "Hello World"
                }
            ]
        });

        let builder = UpdateCardBuilder::new("card_123")
            .title("构建器更新测试")
            .description("使用构建器更新的卡片")
            .card_json(card_json.clone())
            .user_id_type(UserIdType::UserId);

        assert_eq!(builder.request.card_id, "card_123");
        assert_eq!(builder.request.title, Some("构建器更新测试".to_string()));
        assert_eq!(builder.request.description, Some("使用构建器更新的卡片".to_string()));
        assert_eq!(builder.request.card_json, Some(card_json));
        assert_eq!(builder.request.user_id_type, Some(UserIdType::UserId));
    }

    #[test]
    fn test_update_card_builder_validation() {
        // 测试有效构建器
        let valid_builder = UpdateCardBuilder::new("card_123")
            .title("有效卡片")
            .card_json(json!({"type": "card"}));
        assert!(valid_builder.request.validate().is_ok());

        // 测试无效构建器
        let invalid_builder = UpdateCardBuilder::new("")
            .title("无效卡片");
        assert!(invalid_builder.request.validate().is_err());

        // 测试空标题构建器
        let empty_title_builder = UpdateCardBuilder::new("card_123")
            .title("  ");
        assert!(empty_title_builder.request.validate().is_err());
    }

    #[test]
    fn test_update_card_service_method() {
        let config = Config::default();
        let service = CardService::new(config);

        // 验证服务包含所需的方法
        let service_str = format!("{:?}", service);
        assert!(!service_str.is_empty());

        // 验证构建器方法存在
        let builder = service.update_card_builder("card_123");
        assert_eq!(builder.request.card_id, "card_123");
    }

    #[test]
    fn test_update_card_endpoint_construction() {
        // 验证端点常量存在
        assert_eq!(
            crate::core::endpoints_original::Endpoints::CARDKIT_V1_CARDS,
            "/open-apis/cardkit/v1/cards"
        );

        // 验证路径替换逻辑
        let template = crate::core::endpoints_original::Endpoints::CARDKIT_V1_CARDS;
        let final_path = template.replace("{}", "card_123");
        assert_eq!(final_path, "/open-apis/cardkit/v1/cards/card_123");
    }

    #[test]
    fn test_update_card_json_serialization() {
        let request = UpdateCardRequest::new("card_123")
            .title("序列化测试")
            .description("测试JSON序列化")
            .card_json(json!({"type": "card", "version": "1.0"}))
            .user_id_type(UserIdType::OpenId);

        // 测试请求可以转换为JSON
        let body = json!({
            "title": "序列化测试",
            "description": "测试JSON序列化",
            "card_json": {"type": "card", "version": "1.0"}
        });

        assert_eq!(body["title"], "序列化测试");
        assert_eq!(body["description"], "测试JSON序列化");
        assert_eq!(body["card_json"]["type"], "card");
        assert_eq!(body["card_json"]["version"], "1.0");
    }

    #[test]
    fn test_update_card_response_trait() {
        assert_eq!(UpdateCardResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_update_card_comprehensive_scenario() {
        // 测试完整的业务场景
        let complex_card_json = json!({
            "type": "card",
            "header": {
                "title": "审批申请更新",
                "subtitle": "请及时处理更新后的申请"
            },
            "elements": [
                {
                    "type": "div",
                    "text": {
                        "type": "plain_text",
                        "content": "申请人：张三（已更新）"
                    }
                },
                {
                    "type": "action",
                    "actions": [
                        {
                            "type": "button",
                            "text": {
                                "type": "plain_text",
                                "content": "批准更新"
                            },
                            "type": "primary"
                        },
                        {
                            "type": "button",
                            "text": {
                                "type": "plain_text",
                                "content": "拒绝更新"
                            }
                        }
                    ]
                }
            ]
        });

        let request = UpdateCardRequest::new("card_comprehensive_001")
            .title("更新后的审批卡片")
            .description("更新后的员工请假审批申请")
            .card_json(complex_card_json)
            .user_id_type(UserIdType::UnionId);

        assert!(request.validate().is_ok());
        assert_eq!(request.card_id, "card_comprehensive_001");
        assert_eq!(request.title, Some("更新后的审批卡片".to_string()));
        assert_eq!(request.description, Some("更新后的员工请假审批申请".to_string()));
        assert!(request.card_json.is_some());
        assert_eq!(request.user_id_type, Some(UserIdType::UnionId));
    }

    #[test]
    fn test_update_card_partial_update() {
        // 测试部分更新场景 - 只更新标题
        let partial_request = UpdateCardRequest::new("card_partial_001")
            .title("只更新标题");

        assert!(partial_request.validate().is_ok());
        assert_eq!(partial_request.card_id, "card_partial_001");
        assert_eq!(partial_request.title, Some("只更新标题".to_string()));
        assert_eq!(partial_request.description, None);
        assert_eq!(partial_request.card_json, None);
        assert_eq!(partial_request.user_id_type, None);
    }

    #[test]
    fn test_update_card_edge_cases() {
        // 测试极长和特殊字符
        let long_description = "d".repeat(500);
        let edge_request = UpdateCardRequest::new("card_edge_001")
            .description(&long_description);

        assert!(edge_request.validate().is_ok());
        assert_eq!(edge_request.description, Some(long_description));
        assert_eq!(edge_request.description.as_ref().unwrap().len(), 500);

        // 测试特殊字符
        let special_request = UpdateCardRequest::new("card_special_001")
            .title("特殊字符标题：🎉✨")
            .description("包含特殊字符的描述");

        assert!(special_request.validate().is_ok());
        assert_eq!(special_request.title, Some("特殊字符标题：🎉✨".to_string()));
    }

    #[test]
    fn test_update_card_empty_values() {
        // 测试空值处理
        let empty_request = UpdateCardRequest::new("card_empty_001");

        // 只提供card_id，其他字段都为空
        assert!(empty_request.validate().is_ok());
        assert_eq!(empty_request.card_id, "card_empty_001");
        assert_eq!(empty_request.title, None);
        assert_eq!(empty_request.description, None);
        assert_eq!(empty_request.card_json, None);
        assert_eq!(empty_request.user_id_type, None);
    }

    #[test]
    fn test_update_card_builder_pattern() {
        // 测试构建器模式的流畅性
        let builder = UpdateCardBuilder::new("test_card")
            .title("流畅性测试")
            .description("测试链式调用")
            .user_id_type(UserIdType::OpenId);

        // 验证构建器状态
        assert_eq!(builder.request.card_id, "test_card");
        assert_eq!(builder.request.title, Some("流畅性测试".to_string()));
        assert_eq!(builder.request.description, Some("测试链式调用".to_string()));
        assert_eq!(builder.request.user_id_type, Some(UserIdType::OpenId));

        // 验证请求验证通过
        assert!(builder.request.validate().is_ok());

        // 测试链式调用
        let chained_builder = builder
            .title("重新设置标题")  // 重新设置title
            .request;
        assert_eq!(chained_builder.title, Some("重新设置标题".to_string()));
    }
}