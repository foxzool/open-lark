/// 创建订阅
///
/// docPath: /document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file-subscription/create
/// doc: https://open.feishu.cn/document/server-docs/docs/docs-assistant/file-subscription/create
use openlark_core::{
    api::ApiRequest,
    config::Config,
    error::validation_error,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

use super::models::Subscription;

/// 创建订阅请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFileSubscriptionRequest {
    /// 文件 token
    pub file_token: String,
    /// 订阅关系 ID（可选）
    pub subscription_id: Option<String>,
    /// 订阅类型（例如 comment_update）
    pub subscription_type: String,
    /// 是否订阅（可选）
    pub is_subcribe: Option<bool>,
    /// 文档类型（必填）
    pub file_type: String,
}

impl CreateFileSubscriptionRequest {
    pub fn new(
        file_token: impl Into<String>,
        subscription_type: impl Into<String>,
        file_type: impl Into<String>,
    ) -> Self {
        Self {
            file_token: file_token.into(),
            subscription_id: None,
            subscription_type: subscription_type.into(),
            is_subcribe: None,
            file_type: file_type.into(),
        }
    }

    pub fn subscription_id(mut self, subscription_id: impl Into<String>) -> Self {
        self.subscription_id = Some(subscription_id.into());
        self
    }

    pub fn is_subcribe(mut self, is_subcribe: bool) -> Self {
        self.is_subcribe = Some(is_subcribe);
        self
    }
}

#[derive(Debug, Serialize)]
struct CreateFileSubscriptionRequestBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    subscription_id: Option<String>,
    subscription_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_subcribe: Option<bool>,
    file_type: String,
}

pub type CreateFileSubscriptionResponse = Subscription;

/// 创建订阅
pub async fn create_file_subscription(
    request: CreateFileSubscriptionRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<CreateFileSubscriptionResponse> {
    if request.file_token.trim().is_empty() {
        return Err(validation_error("file_token", "file_token 不能为空"));
    }
    if request.subscription_type.trim().is_empty() {
        return Err(validation_error(
            "subscription_type",
            "subscription_type 不能为空",
        ));
    }
    if request.file_type.trim().is_empty() {
        return Err(validation_error("file_type", "file_type 不能为空"));
    }

    let api_endpoint = DriveApi::CreateFileSubscription(request.file_token.clone());

    let mut api_request: ApiRequest<CreateFileSubscriptionResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(serialize_params(
            &CreateFileSubscriptionRequestBody {
                subscription_id: request.subscription_id,
                subscription_type: request.subscription_type,
                is_subcribe: request.is_subcribe,
                file_type: request.file_type,
            },
            "创建订阅",
        )?);

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "创建订阅")
}
