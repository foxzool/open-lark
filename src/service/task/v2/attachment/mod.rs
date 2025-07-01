use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, EmptyResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    service::task::models::{Attachment, UserIdType},
};

/// 附件服务
pub struct AttachmentService {
    pub config: Config,
}

/// 上传附件请求
#[derive(Debug, Serialize, Deserialize)]
pub struct UploadAttachmentRequest {
    /// 文件名
    pub name: String,
    /// 文件大小
    pub size: i64,
    /// 父类型 (task)
    pub parent_type: String,
    /// 父ID (任务GUID)
    pub parent_id: String,
}

/// 上传附件响应
#[derive(Debug, Serialize, Deserialize)]
pub struct UploadAttachmentResponse {
    /// 附件
    pub attachment: Attachment,
}

impl ApiResponseTrait for UploadAttachmentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 附件列表响应
#[derive(Debug, Serialize, Deserialize)]
pub struct ListAttachmentsResponse {
    /// 附件列表
    pub items: Vec<Attachment>,
    /// 下一页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl ApiResponseTrait for ListAttachmentsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取附件响应
#[derive(Debug, Serialize, Deserialize)]
pub struct GetAttachmentResponse {
    /// 附件详情
    pub attachment: Attachment,
}

impl ApiResponseTrait for GetAttachmentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl AttachmentService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 上传附件
    pub async fn upload(
        &self,
        request: UploadAttachmentRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UploadAttachmentResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert(
                "user_id_type".to_string(),
                user_id_type.as_str().to_string(),
            );
        }

        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: "/open-apis/task/v2/attachments/upload".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 列取附件
    pub async fn list(
        &self,
        parent_type: &str,
        parent_id: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ListAttachmentsResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert(
                "user_id_type".to_string(),
                user_id_type.as_str().to_string(),
            );
        }
        query_params.insert("parent_type".to_string(), parent_type.to_string());
        query_params.insert("parent_id".to_string(), parent_id.to_string());
        if let Some(page_size) = page_size {
            query_params.insert("page_size".to_string(), page_size.to_string());
        }
        if let Some(page_token) = page_token {
            query_params.insert("page_token".to_string(), page_token.to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: "/open-apis/task/v2/attachments".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取附件
    pub async fn get(
        &self,
        attachment_guid: &str,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetAttachmentResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert(
                "user_id_type".to_string(),
                user_id_type.as_str().to_string(),
            );
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: format!("/open-apis/task/v2/attachments/{attachment_guid}"),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 删除附件
    pub async fn delete(
        &self,
        attachment_guid: &str,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EmptyResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert(
                "user_id_type".to_string(),
                user_id_type.as_str().to_string(),
            );
        }

        let api_req = ApiRequest {
            http_method: Method::DELETE,
            api_path: format!("/open-apis/task/v2/attachments/{attachment_guid}"),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}
