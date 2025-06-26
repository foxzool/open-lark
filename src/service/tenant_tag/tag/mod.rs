use std::collections::HashMap;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::core::{
    api_req::ApiRequest,
    api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use crate::service::tenant_tag::models::{Tag, TagType, TagStatus, UserIdType};

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
            query_params.insert("user_id_type".to_string(), user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: "/open-apis/tenant-tag/v1/tags".to_string(),
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
            query_params.insert("user_id_type".to_string(), user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::PATCH,
            api_path: format!("/open-apis/tenant-tag/v1/tags/{}", tag_id),
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
            query_params.insert("tag_type".to_string(), tag_type.as_str().to_string());
        }
        if let Some(page_size) = page_size {
            query_params.insert("page_size".to_string(), page_size.to_string());
        }
        if let Some(page_token) = page_token {
            query_params.insert("page_token".to_string(), page_token);
        }
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type".to_string(), user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: "/open-apis/tenant-tag/v1/tags".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}