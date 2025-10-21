use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::{tenant_tag::*, EndpointBuilder},
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    service::tenant_tag::models::{Tag, TagStatus, TagType, UserIdType},
};

/// 标签管理服务
pub struct TagService {
    pub config: Config,
}

/// 创建标签请求
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTagRequest {
    /// 标签名称
    pub name: String,
    /// 标签描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 标签类型
    pub tag_type: TagType,
}

/// 创建标签响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTagResponse {
    /// 标签信息
    pub tag: Tag,
}

impl ApiResponseTrait for CreateTagResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 修改标签请求
#[derive(Debug, Serialize, Deserialize)]
pub struct PatchTagRequest {
    /// 标签名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 标签描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 标签状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TagStatus>,
}

/// 修改标签响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchTagResponse {
    /// 更新后的标签信息
    pub tag: Tag,
}

impl ApiResponseTrait for PatchTagResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 查询标签响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListTagResponse {
    /// 标签列表
    pub tags: Vec<Tag>,
    /// 是否还有更多数据
    pub has_more: bool,
    /// 分页标记
    pub page_token: Option<String>,
}

impl ApiResponseTrait for ListTagResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl TagService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建标签
    pub async fn create(
        &self,
        request: CreateTagRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CreateTagResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: TENANT_TAG_V1_TAGS.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 修改标签
    pub async fn patch(
        &self,
        tag_id: &str,
        request: PatchTagRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<PatchTagResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::PATCH,
            api_path: EndpointBuilder::replace_param(TENANT_TAG_V1_TAG_OPERATION, "tag_id", tag_id),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 查询标签
    pub async fn list(
        &self,
        tag_type: Option<TagType>,
        page_size: Option<i32>,
        page_token: Option<String>,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ListTagResponse>> {
        let mut query_params = HashMap::new();
        if let Some(tag_type) = tag_type {
            query_params.insert("tag_type", tag_type.as_str().to_string());
        }
        if let Some(page_size) = page_size {
            query_params.insert("page_size", page_size.to_string());
        }
        if let Some(page_token) = page_token {
            query_params.insert("page_token", page_token);
        }
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: TENANT_TAG_V1_TAGS.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::api_resp::ResponseFormat;
    use std::time::Duration;

    #[test]
    fn test_tag_service_creation() {
        let config = Config::default();
        let service = TagService::new(config.clone());

        assert_eq!(service.config.app_id, config.app_id);
        assert_eq!(service.config.app_secret, config.app_secret);
    }

    #[test]
    fn test_tag_service_with_custom_config() {
        let config = Config::builder()
            .app_id("tag_service_app")
            .app_secret("tag_service_secret")
            .req_timeout(Duration::from_secs(60))
            .build();

        let service = TagService::new(config.clone());

        assert_eq!(service.config.app_id, "tag_service_app");
        assert_eq!(service.config.app_secret, "tag_service_secret");
        assert_eq!(service.config.req_timeout, Some(Duration::from_secs(60)));
    }

    #[test]
    fn test_create_tag_request_creation() {
        let request = CreateTagRequest {
            name: "测试标签".to_string(),
            description: Some("用于测试的标签".to_string()),
            tag_type: TagType::Chat,
        };

        assert_eq!(request.name, "测试标签");
        assert_eq!(request.description, Some("用于测试的标签".to_string()));
        assert_eq!(request.tag_type, TagType::Chat);
    }

    #[test]
    fn test_create_tag_request_serialization() {
        let request = CreateTagRequest {
            name: "API Test Tag".to_string(),
            description: Some("Tag for API testing".to_string()),
            tag_type: TagType::Chat,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("API Test Tag"));
        assert!(json.contains("Tag for API testing"));
        assert!(json.contains("\"tag_type\":\"chat\""));

        let deserialized: CreateTagRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.name, request.name);
        assert_eq!(deserialized.description, request.description);
        assert_eq!(deserialized.tag_type, request.tag_type);
    }

    #[test]
    fn test_create_tag_request_without_description() {
        let request = CreateTagRequest {
            name: "Simple Tag".to_string(),
            description: None,
            tag_type: TagType::Chat,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("Simple Tag"));
        assert!(!json.contains("description"));

        let deserialized: CreateTagRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.description, None);
    }

    #[test]
    fn test_create_tag_request_debug_trait() {
        let request = CreateTagRequest {
            name: "Debug Test".to_string(),
            description: Some("Debug description".to_string()),
            tag_type: TagType::Chat,
        };

        let debug_output = format!("{:?}", request);
        assert!(debug_output.contains("CreateTagRequest"));
        assert!(debug_output.contains("Debug Test"));
        assert!(debug_output.contains("Debug description"));
    }

    #[test]
    fn test_create_tag_response_data_format() {
        assert_eq!(CreateTagResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_create_tag_response_creation() {
        let tag = Tag {
            tag_id: "created_tag_123".to_string(),
            name: "Created Tag".to_string(),
            description: Some("Tag created successfully".to_string()),
            tag_type: TagType::Chat,
            status: TagStatus::Active,
            create_time: Some("1642723200000".to_string()),
            update_time: Some("1642723200000".to_string()),
            creator_id: Some("creator_456".to_string()),
        };

        let response = CreateTagResponse { tag: tag.clone() };

        assert_eq!(response.tag.tag_id, "created_tag_123");
        assert_eq!(response.tag.name, "Created Tag");
        assert_eq!(response.tag.status, TagStatus::Active);
    }

    #[test]
    fn test_create_tag_response_serialization() {
        let tag = Tag {
            tag_id: "response_tag".to_string(),
            name: "Response Tag".to_string(),
            description: None,
            tag_type: TagType::Chat,
            status: TagStatus::Active,
            create_time: Some("1640995200000".to_string()),
            update_time: Some("1640995200000".to_string()),
            creator_id: Some("response_creator".to_string()),
        };

        let response = CreateTagResponse { tag };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("response_tag"));
        assert!(json.contains("Response Tag"));
        assert!(json.contains("response_creator"));

        let deserialized: CreateTagResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.tag.tag_id, "response_tag");
        assert_eq!(deserialized.tag.name, "Response Tag");
    }

    #[test]
    fn test_patch_tag_request_creation() {
        let request = PatchTagRequest {
            name: Some("更新后的标签".to_string()),
            description: Some("更新的描述".to_string()),
            status: Some(TagStatus::Inactive),
        };

        assert_eq!(request.name, Some("更新后的标签".to_string()));
        assert_eq!(request.description, Some("更新的描述".to_string()));
        assert_eq!(request.status, Some(TagStatus::Inactive));
    }

    #[test]
    fn test_patch_tag_request_partial_update() {
        let request = PatchTagRequest {
            name: Some("Updated Name".to_string()),
            description: None,
            status: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("Updated Name"));
        assert!(!json.contains("description"));
        assert!(!json.contains("status"));

        let deserialized: PatchTagRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.name, Some("Updated Name".to_string()));
        assert_eq!(deserialized.description, None);
        assert_eq!(deserialized.status, None);
    }

    #[test]
    fn test_patch_tag_request_empty_update() {
        let request = PatchTagRequest {
            name: None,
            description: None,
            status: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert_eq!(json, "{}");
    }

    #[test]
    fn test_patch_tag_response_data_format() {
        assert_eq!(PatchTagResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_patch_tag_response_creation() {
        let updated_tag = Tag {
            tag_id: "patched_tag_456".to_string(),
            name: "Patched Tag".to_string(),
            description: Some("Updated description".to_string()),
            tag_type: TagType::Chat,
            status: TagStatus::Inactive,
            create_time: Some("1640995200000".to_string()),
            update_time: Some("1672531200000".to_string()),
            creator_id: Some("original_creator".to_string()),
        };

        let response = PatchTagResponse { tag: updated_tag };

        assert_eq!(response.tag.tag_id, "patched_tag_456");
        assert_eq!(response.tag.name, "Patched Tag");
        assert_eq!(response.tag.status, TagStatus::Inactive);
    }

    #[test]
    fn test_list_tag_response_data_format() {
        assert_eq!(ListTagResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_list_tag_response_creation() {
        let tag1 = Tag {
            tag_id: "list_tag_1".to_string(),
            name: "Tag 1".to_string(),
            description: Some("First tag".to_string()),
            tag_type: TagType::Chat,
            status: TagStatus::Active,
            create_time: Some("1640995200000".to_string()),
            update_time: Some("1640995200000".to_string()),
            creator_id: Some("creator_1".to_string()),
        };

        let tag2 = Tag {
            tag_id: "list_tag_2".to_string(),
            name: "Tag 2".to_string(),
            description: Some("Second tag".to_string()),
            tag_type: TagType::Chat,
            status: TagStatus::Active,
            create_time: Some("1642723200000".to_string()),
            update_time: Some("1642723200000".to_string()),
            creator_id: Some("creator_2".to_string()),
        };

        let response = ListTagResponse {
            tags: vec![tag1, tag2],
            has_more: true,
            page_token: Some("next_page_token".to_string()),
        };

        assert_eq!(response.tags.len(), 2);
        assert!(response.has_more);
        assert_eq!(response.page_token, Some("next_page_token".to_string()));
        assert_eq!(response.tags[0].tag_id, "list_tag_1");
        assert_eq!(response.tags[1].tag_id, "list_tag_2");
    }

    #[test]
    fn test_list_tag_response_empty_list() {
        let response = ListTagResponse {
            tags: vec![],
            has_more: false,
            page_token: None,
        };

        assert_eq!(response.tags.len(), 0);
        assert!(!response.has_more);
        assert_eq!(response.page_token, None);
    }

    #[test]
    fn test_list_tag_response_serialization() {
        let tag = Tag {
            tag_id: "serialization_test".to_string(),
            name: "Serialization Test".to_string(),
            description: None,
            tag_type: TagType::Chat,
            status: TagStatus::Active,
            create_time: Some("1600000000000".to_string()),
            update_time: Some("1600000000000".to_string()),
            creator_id: Some("test_creator".to_string()),
        };

        let response = ListTagResponse {
            tags: vec![tag],
            has_more: false,
            page_token: None,
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("serialization_test"));
        assert!(json.contains("Serialization Test"));
        assert!(json.contains("test_creator"));
        assert!(json.contains("\"has_more\":false"));

        let deserialized: ListTagResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.tags.len(), 1);
        assert_eq!(deserialized.tags[0].tag_id, "serialization_test");
        assert!(!deserialized.has_more);
    }

    #[test]
    fn test_api_request_construction_for_create() {
        let request = CreateTagRequest {
            name: "API Test".to_string(),
            description: Some("Testing API construction".to_string()),
            tag_type: TagType::Chat,
        };

        let body = serde_json::to_vec(&request).unwrap();
        assert!(!body.is_empty());

        let json_body = String::from_utf8(body).unwrap();
        assert!(json_body.contains("API Test"));
        assert!(json_body.contains("Testing API construction"));
    }

    #[test]
    fn test_endpoint_parameter_replacement() {
        let tag_id = "test_tag_123";
        let endpoint =
            EndpointBuilder::replace_param(TENANT_TAG_V1_TAG_OPERATION, "tag_id", tag_id);

        assert!(endpoint.contains(tag_id));
        assert!(!endpoint.contains("{tag_id}"));
    }

    #[test]
    fn test_query_params_construction() {
        let mut query_params = HashMap::new();
        query_params.insert("user_id_type", "user_id".to_string());
        query_params.insert("page_size", "50".to_string());
        query_params.insert("tag_type", "chat".to_string());

        assert_eq!(
            query_params.get("user_id_type"),
            Some(&"user_id".to_string())
        );
        assert_eq!(query_params.get("page_size"), Some(&"50".to_string()));
        assert_eq!(query_params.get("tag_type"), Some(&"chat".to_string()));
        assert_eq!(query_params.len(), 3);
    }

    #[test]
    fn test_debug_traits_for_requests() {
        let create_request = CreateTagRequest {
            name: "Debug Create".to_string(),
            description: Some("Debug description".to_string()),
            tag_type: TagType::Chat,
        };

        let patch_request = PatchTagRequest {
            name: Some("Debug Patch".to_string()),
            description: None,
            status: Some(TagStatus::Active),
        };

        let create_debug = format!("{:?}", create_request);
        let patch_debug = format!("{:?}", patch_request);

        assert!(create_debug.contains("CreateTagRequest"));
        assert!(create_debug.contains("Debug Create"));
        assert!(patch_debug.contains("PatchTagRequest"));
        assert!(patch_debug.contains("Debug Patch"));
    }

    #[test]
    fn test_clone_traits_for_responses() {
        let tag = Tag {
            tag_id: "clone_test".to_string(),
            name: "Clone Test".to_string(),
            description: Some("Testing clone".to_string()),
            tag_type: TagType::Chat,
            status: TagStatus::Active,
            create_time: Some("1600000000000".to_string()),
            update_time: Some("1600000000000".to_string()),
            creator_id: Some("clone_creator".to_string()),
        };

        let create_response = CreateTagResponse { tag: tag.clone() };
        let patch_response = PatchTagResponse { tag: tag.clone() };
        let list_response = ListTagResponse {
            tags: vec![tag],
            has_more: false,
            page_token: None,
        };

        let cloned_create = create_response.clone();
        let cloned_patch = patch_response.clone();
        let cloned_list = list_response.clone();

        assert_eq!(cloned_create.tag.tag_id, create_response.tag.tag_id);
        assert_eq!(cloned_patch.tag.tag_id, patch_response.tag.tag_id);
        assert_eq!(cloned_list.tags.len(), list_response.tags.len());
    }

    #[test]
    fn test_tag_service_multiple_instances() {
        let config = Config::default();

        let service1 = TagService::new(config.clone());
        let service2 = TagService::new(config.clone());

        assert_eq!(service1.config.app_id, service2.config.app_id);
        assert_eq!(service1.config.app_secret, service2.config.app_secret);
    }
}
