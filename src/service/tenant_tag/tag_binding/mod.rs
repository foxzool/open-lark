use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::tenant_tag::*,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    service::tenant_tag::models::{TagBinding, UserIdType},
};

/// 标签绑定服务
pub struct TagBindingService {
    pub config: Config,
}

/// 查询绑定关系请求参数
#[derive(Debug, Default)]
pub struct GetTagBindingRequest {
    /// 实体ID
    pub entity_id: String,
    /// 实体类型
    pub entity_type: String,
    /// 标签ID（可选）
    pub tag_id: Option<String>,
    /// 分页大小
    pub page_size: Option<i32>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 用户ID类型
    pub user_id_type: Option<UserIdType>,
}

/// 查询实体与标签的绑定关系响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTagBindingResponse {
    /// 绑定关系列表
    pub bindings: Vec<TagBinding>,
    /// 是否还有更多数据
    pub has_more: bool,
    /// 分页标记
    pub page_token: Option<String>,
}

impl ApiResponseTrait for GetTagBindingResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 绑定标签到群请求
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTagBindingRequest {
    /// 标签ID列表
    pub tag_ids: Vec<String>,
    /// 群ID
    pub chat_id: String,
}

/// 绑定标签到群响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTagBindingResponse {
    /// 成功绑定的标签ID列表
    pub success_tag_ids: Vec<String>,
    /// 失败的标签ID列表
    pub failed_tag_ids: Vec<String>,
}

impl ApiResponseTrait for CreateTagBindingResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 解绑标签与群请求
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTagBindingRequest {
    /// 要解绑的标签ID列表
    pub tag_ids: Vec<String>,
    /// 群ID
    pub chat_id: String,
}

/// 解绑标签与群响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTagBindingResponse {
    /// 成功解绑的标签ID列表
    pub success_tag_ids: Vec<String>,
    /// 失败的标签ID列表
    pub failed_tag_ids: Vec<String>,
}

impl ApiResponseTrait for UpdateTagBindingResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl TagBindingService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 查询实体与标签的绑定关系
    pub async fn get(
        &self,
        request: GetTagBindingRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetTagBindingResponse>> {
        let mut query_params = HashMap::new();
        query_params.insert("entity_id", request.entity_id);
        query_params.insert("entity_type", request.entity_type);

        if let Some(tag_id) = request.tag_id {
            query_params.insert("tag_id", tag_id);
        }
        if let Some(page_size) = request.page_size {
            query_params.insert("page_size", page_size.to_string());
        }
        if let Some(page_token) = request.page_token {
            query_params.insert("page_token", page_token);
        }
        if let Some(user_id_type) = request.user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: TENANT_TAG_V1_TAG_BINDINGS.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 绑定标签到群
    pub async fn create(
        &self,
        request: CreateTagBindingRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CreateTagBindingResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: TENANT_TAG_V1_TAG_BINDINGS.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 解绑标签与群
    pub async fn update(
        &self,
        request: UpdateTagBindingRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UpdateTagBindingResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::PUT,
            api_path: TENANT_TAG_V1_TAG_BINDINGS.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}
