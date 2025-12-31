//! 更新订阅状态
//!
//! docPath: https://open.feishu.cn/document/server-docs/docs/docs-assistant/file-subscription/patch

use openlark_core::{
    api::ApiRequest, config::Config, error::validation_error, http::Transport, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

use super::models::Subscription;

/// 更新订阅状态请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchSubscriptionRequest {
    /// 文件 token
    pub file_token: String,
    /// 订阅 ID
    pub subscription_id: String,
    /// 是否订阅
    pub is_subscribe: bool,
    /// 文档类型（必填）
    pub file_type: String,
}

impl PatchSubscriptionRequest {
    pub fn new(
        file_token: impl Into<String>,
        subscription_id: impl Into<String>,
        is_subscribe: bool,
        file_type: impl Into<String>,
    ) -> Self {
        Self {
            file_token: file_token.into(),
            subscription_id: subscription_id.into(),
            is_subscribe,
            file_type: file_type.into(),
        }
    }
}

#[derive(Debug, Serialize)]
struct PatchSubscriptionRequestBody {
    is_subscribe: bool,
    file_type: String,
}

pub type PatchSubscriptionResponse = Subscription;

/// 更新订阅状态
pub async fn patch_subscription(
    request: PatchSubscriptionRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<PatchSubscriptionResponse> {
    if request.file_token.trim().is_empty() {
        return Err(validation_error("file_token", "file_token 不能为空"));
    }
    if request.subscription_id.trim().is_empty() {
        return Err(validation_error(
            "subscription_id",
            "subscription_id 不能为空",
        ));
    }
    if request.file_type.trim().is_empty() {
        return Err(validation_error("file_type", "file_type 不能为空"));
    }
    match request.file_type.as_str() {
        "doc" | "docx" | "wiki" => {}
        _ => {
            return Err(validation_error(
                "file_type",
                "file_type 仅支持 doc/docx/wiki",
            ));
        }
    }

    let api_endpoint = DriveApi::UpdateFileSubscription(
        request.file_token.clone(),
        request.subscription_id.clone(),
    );

    let mut api_request: ApiRequest<PatchSubscriptionResponse> =
        ApiRequest::patch(&api_endpoint.to_url()).body(serialize_params(
            &PatchSubscriptionRequestBody {
                is_subscribe: request.is_subscribe,
                file_type: request.file_type,
            },
            "更新订阅状态",
        )?);

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "更新订阅状态")
}
