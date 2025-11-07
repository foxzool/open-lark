//! IM v1 Pin 消息服务
//!
//! 提供飞书开放平台 Pin 消息管理的 v1 版本 API 实现，包括：
//! - 创建 Pin 消息
//! - 删除 Pin 消息
//! - 获取群内 Pin 消息列表

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]

use crate::core::{
    api_resp::{ApiResponseTrait, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    ApiRequest, SDKResult,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Pin 消息服务
#[derive(Debug, Clone)]
pub struct PinService {
    config: Config,
}

impl PinService {
    /// 创建新的 Pin 服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建 Pin 消息
    ///
    /// 将指定消息 Pin 到群聊中
    ///
    /// # 参数
    /// * `message_id` - 要 Pin 的消息 ID
    /// * `user_id_type` - 用户 ID 类型，可选值包括 open_id、user_id、union_id
    ///
    /// # 返回值
    /// 返回创建的 Pin 消息信息
    pub async fn create(
        &self,
        message_id: &str,
        user_id_type: Option<UserIdType>,
    ) -> SDKResult<CreatePinResponse> {
        let mut query_params = HashMap::new();
        query_params.insert("message_id", message_id.to_string());

        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: crate::core::endpoints_original::Endpoints::IM_V1_PINS.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        let resp = Transport::<CreatePinResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 删除 Pin 消息
    ///
    /// 移除指定的 Pin 消息
    ///
    /// # 参数
    /// * `pin_id` - Pin 消息 ID
    /// * `user_id_type` - 用户 ID 类型，可选值包括 open_id、user_id、union_id
    ///
    /// # 返回值
    /// 返回删除操作结果
    pub async fn delete(
        &self,
        pin_id: &str,
        user_id_type: Option<UserIdType>,
    ) -> SDKResult<EmptyResponse> {
        let mut query_params = HashMap::new();

        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_path = crate::core::endpoints_original::Endpoints::IM_V1_DELETE_PIN
            .replace("{pin_id}", pin_id);

        let api_req = ApiRequest {
            http_method: reqwest::Method::DELETE,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        let resp = Transport::<EmptyResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 获取群内 Pin 消息列表
    ///
    /// 获取指定群聊中的所有 Pin 消息
    ///
    /// # 参数
    /// * `chat_id` - 群聊 ID
    /// * `user_id_type` - 用户 ID 类型，可选值包括 open_id、user_id、union_id
    /// * `page_size` - 分页大小，默认为 20
    /// * `page_token` - 分页标记，用于获取下一页数据
    ///
    /// # 返回值
    /// 返回群内 Pin 消息列表，支持分页
    pub async fn list(
        &self,
        chat_id: &str,
        user_id_type: Option<UserIdType>,
        page_size: Option<i32>,
        page_token: Option<String>,
    ) -> SDKResult<ListPinResponse> {
        let mut query_params = HashMap::new();
        query_params.insert("chat_id", chat_id.to_string());

        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        if let Some(page_size) = page_size {
            query_params.insert("page_size", page_size.to_string());
        }

        if let Some(page_token) = page_token {
            query_params.insert("page_token", page_token);
        }

        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path: crate::core::endpoints_original::Endpoints::IM_V1_PINS.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        let resp = Transport::<ListPinResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }
}

// ==================== 数据模型 ====================

/// 用户 ID 类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserIdType {
    #[serde(rename = "open_id")]
    OpenId,
    #[serde(rename = "user_id")]
    UserId,
    #[serde(rename = "union_id")]
    UnionId,
}

impl UserIdType {
    pub fn as_str(&self) -> &'static str {
        match self {
            UserIdType::OpenId => "open_id",
            UserIdType::UserId => "user_id",
            UserIdType::UnionId => "union_id",
        }
    }
}

/// Pin 消息信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PinInfo {
    /// Pin 消息 ID
    pub pin_id: String,
    /// 消息 ID
    pub message_id: String,
    /// 群聊 ID
    pub chat_id: String,
    /// Pin 操作者 ID
    pub operator_id: String,
    /// 创建时间
    pub create_time: String,
}

/// 创建 Pin 消息响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreatePinResponse {
    /// Pin 消息信息
    pub pin: PinInfo,
}

impl ApiResponseTrait for CreatePinResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 空响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EmptyResponse {}

impl ApiResponseTrait for EmptyResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取 Pin 消息列表响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListPinResponse {
    /// Pin 消息列表
    pub items: Vec<PinInfo>,
    /// 是否还有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for ListPinResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

// ==================== 构建器模式 ====================

/// 创建 Pin 消息构建器
#[derive(Debug, Clone)]
pub struct CreatePinBuilder {
    message_id: String,
    user_id_type: Option<UserIdType>,
}

impl CreatePinBuilder {
    /// 创建新的构建器
    pub fn new(message_id: impl Into<String>) -> Self {
        Self {
            message_id: message_id.into(),
            user_id_type: None,
        }
    }

    /// 设置用户 ID 类型
    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 执行创建操作
    pub async fn execute(self, service: &PinService) -> SDKResult<CreatePinResponse> {
        service.create(&self.message_id, self.user_id_type).await
    }
}

/// 删除 Pin 消息构建器
#[derive(Debug, Clone)]
pub struct DeletePinBuilder {
    pin_id: String,
    user_id_type: Option<UserIdType>,
}

impl DeletePinBuilder {
    /// 创建新的构建器
    pub fn new(pin_id: impl Into<String>) -> Self {
        Self {
            pin_id: pin_id.into(),
            user_id_type: None,
        }
    }

    /// 设置用户 ID 类型
    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 执行删除操作
    pub async fn execute(self, service: &PinService) -> SDKResult<EmptyResponse> {
        service.delete(&self.pin_id, self.user_id_type).await
    }
}

/// 获取 Pin 消息列表构建器
#[derive(Debug, Clone)]
pub struct ListPinBuilder {
    chat_id: String,
    user_id_type: Option<UserIdType>,
    page_size: Option<i32>,
    page_token: Option<String>,
}

impl ListPinBuilder {
    /// 创建新的构建器
    pub fn new(chat_id: impl Into<String>) -> Self {
        Self {
            chat_id: chat_id.into(),
            user_id_type: None,
            page_size: None,
            page_token: None,
        }
    }

    /// 设置用户 ID 类型
    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 设置分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 执行查询操作
    pub async fn execute(self, service: &PinService) -> SDKResult<ListPinResponse> {
        service.list(&self.chat_id, self.user_id_type, self.page_size, self.page_token).await
    }
}

impl PinService {
    /// 创建 Pin 消息构建器
    pub fn create_pin_builder(&self, message_id: impl Into<String>) -> CreatePinBuilder {
        CreatePinBuilder::new(message_id)
    }

    /// 删除 Pin 消息构建器
    pub fn delete_pin_builder(&self, pin_id: impl Into<String>) -> DeletePinBuilder {
        DeletePinBuilder::new(pin_id)
    }

    /// 获取 Pin 消息列表构建器
    pub fn list_pin_builder(&self, chat_id: impl Into<String>) -> ListPinBuilder {
        ListPinBuilder::new(chat_id)
    }
}

// ==================== 单元测试 ====================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pin_service_creation() {
        let config = Config::default();
        let service = PinService::new(config);
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_user_id_type() {
        assert_eq!(UserIdType::OpenId.as_str(), "open_id");
        assert_eq!(UserIdType::UserId.as_str(), "user_id");
        assert_eq!(UserIdType::UnionId.as_str(), "union_id");
    }

    #[test]
    fn test_pin_info_creation() {
        let pin_info = PinInfo {
            pin_id: "pin_123".to_string(),
            message_id: "msg_456".to_string(),
            chat_id: "chat_789".to_string(),
            operator_id: "user_abc".to_string(),
            create_time: "2023-01-01T00:00:00Z".to_string(),
        };

        assert_eq!(pin_info.pin_id, "pin_123");
        assert_eq!(pin_info.message_id, "msg_456");
        assert_eq!(pin_info.chat_id, "chat_789");
        assert_eq!(pin_info.operator_id, "user_abc");
        assert_eq!(pin_info.create_time, "2023-01-01T00:00:00Z");
    }

    #[test]
    fn test_create_pin_builder() {
        let builder = CreatePinBuilder::new("msg_123")
            .user_id_type(UserIdType::OpenId);

        assert_eq!(builder.message_id, "msg_123");
        assert_eq!(builder.user_id_type, Some(UserIdType::OpenId));
    }

    #[test]
    fn test_delete_pin_builder() {
        let builder = DeletePinBuilder::new("pin_456")
            .user_id_type(UserIdType::UserId);

        assert_eq!(builder.pin_id, "pin_456");
        assert_eq!(builder.user_id_type, Some(UserIdType::UserId));
    }

    #[test]
    fn test_list_pin_builder() {
        let builder = ListPinBuilder::new("chat_789")
            .user_id_type(UserIdType::UnionId)
            .page_size(20)
            .page_token("token_abc");

        assert_eq!(builder.chat_id, "chat_789");
        assert_eq!(builder.user_id_type, Some(UserIdType::UnionId));
        assert_eq!(builder.page_size, Some(20));
        assert_eq!(builder.page_token, Some("token_abc".to_string()));
    }

    #[test]
    fn test_response_default_creation() {
        let create_response = CreatePinResponse::default();
        assert_eq!(create_response.pin.pin_id, "");

        let empty_response = EmptyResponse::default();
        // Empty response has no fields to test

        let list_response = ListPinResponse::default();
        assert_eq!(list_response.items.len(), 0);
        assert_eq!(list_response.has_more, None);
        assert_eq!(list_response.page_token, None);
    }

    #[test]
    fn test_api_response_trait_implementation() {
        assert_eq!(CreatePinResponse::data_format(), ResponseFormat::Data);
        assert_eq!(EmptyResponse::data_format(), ResponseFormat::Data);
        assert_eq!(ListPinResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_endpoint_constants() {
        // Test that the endpoint constants are properly defined
        assert_eq!(
            crate::core::endpoints_original::Endpoints::IM_V1_PINS,
            "/open-apis/im/v1/pins"
        );
        assert_eq!(
            crate::core::endpoints_original::Endpoints::IM_V1_DELETE_PIN,
            "/open-apis/im/v1/pins/{pin_id}"
        );
    }

    #[test]
    fn test_query_parameters_construction() {
        let mut query_params = HashMap::new();
        query_params.insert("message_id", "msg_123".to_string());
        query_params.insert("user_id_type", "open_id".to_string());

        assert_eq!(query_params.len(), 2);
        assert_eq!(query_params.get("message_id"), Some(&"msg_123".to_string()));
        assert_eq!(query_params.get("user_id_type"), Some(&"open_id".to_string()));
    }

    #[test]
    fn test_comprehensive_pin_data() {
        // Test comprehensive pin data with all fields
        let comprehensive_pin = PinInfo {
            pin_id: "comprehensive_pin_001".to_string(),
            message_id: "msg_comprehensive_002".to_string(),
            chat_id: "chat_comprehensive_003".to_string(),
            operator_id: "operator_comprehensive_004".to_string(),
            create_time: "2023-12-31T23:59:59Z".to_string(),
        };

        assert_eq!(
            comprehensive_pin.pin_id,
            "comprehensive_pin_001"
        );
        assert_eq!(
            comprehensive_pin.message_id,
            "msg_comprehensive_002"
        );
        assert_eq!(
            comprehensive_pin.chat_id,
            "chat_comprehensive_003"
        );
        assert_eq!(
            comprehensive_pin.operator_id,
            "operator_comprehensive_004"
        );
        assert_eq!(
            comprehensive_pin.create_time,
            "2023-12-31T23:59:59Z"
        );
    }

    #[test]
    fn test_list_pin_response_with_data() {
        let mut list_response = ListPinResponse::default();
        list_response.items = vec![
            PinInfo {
                pin_id: "pin_list_001".to_string(),
                message_id: "msg_list_001".to_string(),
                chat_id: "chat_list".to_string(),
                operator_id: "operator_list".to_string(),
                create_time: "2023-01-01T00:00:00Z".to_string(),
            },
            PinInfo {
                pin_id: "pin_list_002".to_string(),
                message_id: "msg_list_002".to_string(),
                chat_id: "chat_list".to_string(),
                operator_id: "operator_list".to_string(),
                create_time: "2023-01-02T00:00:00Z".to_string(),
            },
        ];
        list_response.has_more = Some(true);
        list_response.page_token = Some("next_page_token".to_string());

        assert_eq!(list_response.items.len(), 2);
        assert_eq!(list_response.items[0].pin_id, "pin_list_001");
        assert_eq!(list_response.items[1].pin_id, "pin_list_002");
        assert_eq!(list_response.has_more, Some(true));
        assert_eq!(list_response.page_token, Some("next_page_token".to_string()));
    }
}