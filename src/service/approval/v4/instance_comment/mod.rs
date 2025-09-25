use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, EmptyResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::{approval::*, EndpointBuilder},
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    service::approval::models::{ApprovalComment, UserIdType},
};

/// 审批评论服务
pub struct InstanceCommentService {
    pub config: Config,
}

/// 创建评论请求
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCommentRequest {
    /// 评论内容
    pub content: String,
    /// 附件
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<serde_json::Value>>,
}

/// 创建评论响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCommentResponse {
    /// 评论ID
    pub comment_id: String,
}

impl ApiResponseTrait for CreateCommentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取评论列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListCommentsResponse {
    /// 评论列表
    pub comments: Vec<ApprovalComment>,
    /// 是否还有更多数据
    pub has_more: bool,
    /// 分页标记
    pub page_token: Option<String>,
}

impl ApiResponseTrait for ListCommentsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl InstanceCommentService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建评论
    pub async fn create(
        &self,
        instance_code: &str,
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
                APPROVAL_V4_INSTANCE_COMMENTS_CREATE,
                "instance_code",
                instance_code,
            ),
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
        instance_code: &str,
        comment_id: &str,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EmptyResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::DELETE,
            api_path: {
                let path = EndpointBuilder::replace_param(
                    APPROVAL_V4_INSTANCE_COMMENT_DELETE,
                    "instance_code",
                    instance_code,
                );
                EndpointBuilder::replace_param(&path, "comment_id", comment_id)
            },
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 清空评论
    pub async fn remove_all(
        &self,
        instance_code: &str,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EmptyResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::DELETE,
            api_path: EndpointBuilder::replace_param(
                APPROVAL_V4_INSTANCE_COMMENTS_REPLY,
                "instance_code",
                instance_code,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取评论
    pub async fn list(
        &self,
        instance_code: &str,
        page_size: Option<i32>,
        page_token: Option<String>,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ListCommentsResponse>> {
        let mut query_params = HashMap::new();
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
            api_path: EndpointBuilder::replace_param(
                APPROVAL_V4_INSTANCE_COMMENTS_LIST,
                "instance_code",
                instance_code,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}
