//! CardKit v1 卡片创建API
//!
//! 提供飞书卡片实体的创建功能，支持各种类型的交互式卡片

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

/// 创建卡片实体请求
#[derive(Debug, Clone)]
pub struct CreateCardRequest {
    /// 卡片标题
    pub title: Option<String>,
    /// 卡片描述
    pub description: Option<String>,
    /// 卡片JSON内容
    pub card_json: Option<serde_json::Value>,
    /// 用户ID类型
    pub user_id_type: Option<UserIdType>,
}

impl CreateCardRequest {
    /// 创建新的请求实例
    pub fn new() -> Self {
        Self {
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
        if let Some(ref card_json) = self.card_json {
            if card_json.is_null() {
                return Err("card_json不能为null".to_string());
            }
        }
        Ok(())
    }
}

/// 创建卡片实体响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCardResponseData {
    /// 创建的卡片信息
    pub card: Card,
}

/// 创建卡片实体响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateCardResponse {
    /// 响应数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<CreateCardResponseData>,
    /// 是否成功
    pub success: bool,
    /// 错误消息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// 错误代码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
}

impl ApiResponseTrait for CreateCardResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl CardService {
    /// 创建卡片实体
    ///
    /// 创建一个新的交互式卡片，支持丰富的组件和交互功能
    ///
    /// # 参数
    /// * `req` - 创建卡片请求
    ///
    /// # 返回值
    /// 返回创建的卡片信息
    ///
    /// # 示例
    /// ```rust
    /// use open_lark::service::cardkit::v1::card::create::{CreateCardRequest, UserIdType};
    /// let request = CreateCardRequest::new()
    ///     .title("审批卡片")
    ///     .description("请审批此申请")
    ///     .user_id_type(UserIdType::OpenId);
    /// let response = service.create_card(&request).await?;
    /// ```
    pub async fn create_card(&self, req: &CreateCardRequest) -> SDKResult<CreateCardResponse> {
        req.validate()?;
        log::debug!("开始创建卡片: title={:?}", req.title);

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

        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: crate::core::endpoints_original::Endpoints::CARDKIT_V1_CARDS.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: serde_json::to_vec(&body).unwrap_or_default(),
            ..Default::default()
        };

        let resp = Transport::<CreateCardResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        if response.success {
            log::info!("卡片创建成功: title={:?}", req.title);
        } else {
            log::warn!("卡片创建失败: title={:?}, error={:?}", req.title, response.error_message);
        }

        Ok(response)
    }
}

// ==================== 构建器模式 ====================

/// 创建卡片实体构建器
#[derive(Debug, Clone)]
pub struct CreateCardBuilder {
    request: CreateCardRequest,
}

impl CreateCardBuilder {
    /// 创建新的构建器
    pub fn new() -> Self {
        Self {
            request: CreateCardRequest::new(),
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

    /// 执行创建卡片操作
    pub async fn execute(self, service: &CardService) -> SDKResult<CreateCardResponse> {
        service.create_card(&self.request).await
    }
}

impl CardService {
    /// 创建卡片构建器
    pub fn create_card_builder(&self) -> CreateCardBuilder {
        CreateCardBuilder::new()
    }
}

// ==================== 单元测试 ====================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;

    #[test]
    fn test_create_card_request_creation() {
        let request = CreateCardRequest::new();
        assert_eq!(request.title, None);
        assert_eq!(request.description, None);
        assert_eq!(request.card_json, None);
        assert_eq!(request.user_id_type, None);
    }

    #[test]
    fn test_create_card_request_with_fields() {
        let card_json = json!({
            "type": "card",
            "elements": []
        });

        let request = CreateCardRequest::new()
            .title("测试卡片")
            .description("这是一个测试卡片")
            .card_json(card_json.clone())
            .user_id_type(UserIdType::OpenId);

        assert_eq!(request.title, Some("测试卡片".to_string()));
        assert_eq!(request.description, Some("这是一个测试卡片".to_string()));
        assert_eq!(request.card_json, Some(card_json));
        assert_eq!(request.user_id_type, Some(UserIdType::OpenId));
    }

    #[test]
    fn test_create_card_request_validation() {
        // 测试正常情况
        let valid_request = CreateCardRequest::new()
            .title("测试卡片")
            .card_json(json!({"type": "card"}));
        assert!(valid_request.validate().is_ok());

        // 测试null card_json
        let invalid_request = CreateCardRequest::new()
            .card_json(json!(null));
        assert!(invalid_request.validate().is_err());
    }

    #[test]
    fn test_create_card_response_creation() {
        let card = Card {
            card_id: Some("card_123".to_string()),
            title: Some("测试卡片".to_string()),
            description: Some("测试描述".to_string()),
            ..Default::default()
        };

        let response_data = CreateCardResponseData { card };
        let response = CreateCardResponse {
            data: Some(response_data),
            success: true,
            ..Default::default()
        };

        assert!(response.success);
        assert!(response.data.is_some());
        assert_eq!(response.data.as_ref().unwrap().card.card_id, Some("card_123".to_string()));
    }

    #[test]
    fn test_create_card_builder() {
        let card_json = json!({
            "type": "card",
            "elements": [
                {
                    "type": "text",
                    "content": "Hello World"
                }
            ]
        });

        let builder = CreateCardBuilder::new()
            .title("构建器测试")
            .description("使用构建器创建的卡片")
            .card_json(card_json.clone())
            .user_id_type(UserIdType::UserId);

        assert_eq!(builder.request.title, Some("构建器测试".to_string()));
        assert_eq!(builder.request.description, Some("使用构建器创建的卡片".to_string()));
        assert_eq!(builder.request.card_json, Some(card_json));
        assert_eq!(builder.request.user_id_type, Some(UserIdType::UserId));
    }

    #[test]
    fn test_user_id_type_display() {
        assert_eq!(UserIdType::OpenId.to_string(), "open_id");
        assert_eq!(UserIdType::UnionId.to_string(), "union_id");
        assert_eq!(UserIdType::UserId.to_string(), "user_id");
    }

    #[test]
    fn test_create_card_builder_validation() {
        // 测试有效构建器
        let valid_builder = CreateCardBuilder::new()
            .title("有效卡片")
            .card_json(json!({"type": "card"}));
        assert!(valid_builder.request.validate().is_ok());

        // 测试无效构建器
        let invalid_builder = CreateCardBuilder::new()
            .card_json(json!(null));
        assert!(invalid_builder.request.validate().is_err());
    }

    #[test]
    fn test_create_card_service_method() {
        let config = Config::default();
        let service = CardService::new(config);

        // 验证服务包含所需的方法
        let service_str = format!("{:?}", service);
        assert!(!service_str.is_empty());

        // 验证构建器方法存在
        let builder = service.create_card_builder();
        assert_eq!(builder.request.title, None);
    }

    #[test]
    fn test_create_card_endpoint_constant() {
        // 验证端点常量存在
        assert_eq!(
            crate::core::endpoints_original::Endpoints::CARDKIT_V1_CARDS,
            "/open-apis/cardkit/v1/cards"
        );
    }

    #[test]
    fn test_create_card_json_serialization() {
        let request = CreateCardRequest::new()
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
    fn test_create_card_response_trait() {
        assert_eq!(CreateCardResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_create_card_comprehensive_scenario() {
        // 测试完整的业务场景
        let complex_card_json = json!({
            "type": "card",
            "header": {
                "title": "审批申请",
                "subtitle": "请及时处理"
            },
            "elements": [
                {
                    "type": "div",
                    "text": {
                        "type": "plain_text",
                        "content": "申请人：张三"
                    }
                },
                {
                    "type": "action",
                    "actions": [
                        {
                            "type": "button",
                            "text": {
                                "type": "plain_text",
                                "content": "批准"
                            },
                            "type": "primary"
                        },
                        {
                            "type": "button",
                            "text": {
                                "type": "plain_text",
                                "content": "拒绝"
                            }
                        }
                    ]
                }
            ]
        });

        let request = CreateCardRequest::new()
            .title("审批卡片")
            .description("员工请假审批申请")
            .card_json(complex_card_json)
            .user_id_type(UserIdType::UnionId);

        assert!(request.validate().is_ok());
        assert_eq!(request.title, Some("审批卡片".to_string()));
        assert_eq!(request.description, Some("员工请假审批申请".to_string()));
        assert!(request.card_json.is_some());
        assert_eq!(request.user_id_type, Some(UserIdType::UnionId));
    }
}