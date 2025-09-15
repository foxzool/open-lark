use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::{EndpointBuilder, Endpoints},
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    service::mail::models::UserIdType,
};

/// 邮件附件服务
pub struct AttachmentService {
    pub config: Config,
}

/// 获取附件下载链接响应
#[derive(Debug, Serialize, Deserialize)]
pub struct GetAttachmentDownloadUrlResponse {
    /// 下载链接
    pub download_url: String,
    /// 链接过期时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<i64>,
}

impl ApiResponseTrait for GetAttachmentDownloadUrlResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl AttachmentService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取附件下载链接
    ///
    /// 该接口用于获取邮件附件的下载链接。
    ///
    /// # 参数
    ///
    /// - `user_mailbox_id`: 用户邮箱ID
    /// - `message_id`: 邮件ID
    /// - `attachment_id`: 附件ID
    /// - `user_id_type`: 用户ID类型
    /// - `option`: 请求选项
    ///
    /// # 错误
    ///
    /// - 参数无效
    /// - 权限不足
    /// - 附件不存在
    pub async fn download_url(
        &self,
        user_mailbox_id: &str,
        message_id: &str,
        attachment_id: &str,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetAttachmentDownloadUrlResponse>> {
        let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
        }

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(
                &EndpointBuilder::replace_param(
                    &EndpointBuilder::replace_param(
                        Endpoints::MAIL_V1_USER_MAILBOX_MESSAGE_ATTACHMENT_DOWNLOAD_URL,
                        "user_mailbox_id",
                        user_mailbox_id,
                    ),
                    "message_id",
                    message_id,
                ),
                "attachment_id",
                attachment_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}
