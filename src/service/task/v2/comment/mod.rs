use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, EmptyResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::{EndpointBuilder, Endpoints},
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    service::task::models::{Comment, UserIdType},
};

/// 评论服务
pub struct CommentService {
    pub config: Config,
}

/// 创建评论请求
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCommentRequest {
    /// 评论内容
    pub content: String,
    /// 父评论ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
}

/// 创建评论响应
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCommentResponse {
    /// 创建的评论
    pub comment: Comment,
}

impl ApiResponseTrait for CreateCommentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新评论请求
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCommentRequest {
    /// 评论内容
    pub content: String,
}

/// 更新评论响应
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCommentResponse {
    /// 更新后的评论
    pub comment: Comment,
}

impl ApiResponseTrait for UpdateCommentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取评论响应
#[derive(Debug, Serialize, Deserialize)]
pub struct GetCommentResponse {
    /// 评论详情
    pub comment: Comment,
}

impl ApiResponseTrait for GetCommentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 评论列表响应
#[derive(Debug, Serialize, Deserialize)]
pub struct ListCommentsResponse {
    /// 评论列表
    pub items: Vec<Comment>,
    /// 下一页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl ApiResponseTrait for ListCommentsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl CommentService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建评论
    pub async fn create(
        &self,
        task_guid: &str,
        request: CreateCommentRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CreateCommentResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: EndpointBuilder::replace_param(
                Endpoints::TASK_V2_TASK_COMMENTS,
                "task_guid",
                task_guid,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取评论详情
    pub async fn get(
        &self,
        task_guid: &str,
        comment_id: &str,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetCommentResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let temp_path = EndpointBuilder::replace_param(
            Endpoints::TASK_V2_TASK_COMMENT_GET,
            "task_guid",
            task_guid,
        );
        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(&temp_path, "comment_id", comment_id),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 更新评论
    pub async fn patch(
        &self,
        task_guid: &str,
        comment_id: &str,
        request: UpdateCommentRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UpdateCommentResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let temp_path2 = EndpointBuilder::replace_param(
            Endpoints::TASK_V2_TASK_COMMENT_GET,
            "task_guid",
            task_guid,
        );
        let api_req = ApiRequest {
            http_method: Method::PATCH,
            api_path: EndpointBuilder::replace_param(&temp_path2, "comment_id", comment_id),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 删除评论
    pub async fn delete(
        &self,
        task_guid: &str,
        comment_id: &str,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EmptyResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let temp_path3 = EndpointBuilder::replace_param(
            Endpoints::TASK_V2_TASK_COMMENT_GET,
            "task_guid",
            task_guid,
        );
        let api_req = ApiRequest {
            http_method: Method::DELETE,
            api_path: EndpointBuilder::replace_param(&temp_path3, "comment_id", comment_id),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取评论列表
    pub async fn list(
        &self,
        task_guid: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ListCommentsResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }
        if let Some(page_size) = page_size {
            query_params.insert("page_size", page_size.to_string());
        }
        if let Some(page_token) = page_token {
            query_params.insert("page_token", page_token.to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(
                Endpoints::TASK_V2_TASK_COMMENTS,
                "task_guid",
                task_guid,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}
