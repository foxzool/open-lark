use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use openlark_core::standard_response::StandardResponse;
use openlark_core::{
    api_req::ApiRequest,
    api_resp::{BaseResponse, EmptyResponse},
    config::Config,
    constants::AccessTokenType,
    endpoints::EndpointBuilder,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};

/// URL预览服务
pub struct UrlPreviewService {
    pub config: Config,
}

/// 批量更新URL预览请求
#[derive(Debug, Serialize, Deserialize)]
pub struct BatchUpdateUrlPreviewRequest {
    /// URL预览列表
    pub previews: Vec<UrlPreviewInfo>,
}

// 接入统一 Service 抽象（IM v1 - UrlPreviewService）
// impl_full_service!(UrlPreviewService, "im.url_preview", "v1");

/// URL预览信息
#[derive(Debug, Serialize, Deserialize)]
pub struct UrlPreviewInfo {
    /// URL
    pub url: String,
    /// 预览标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 预览描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 预览图片URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    /// 其他自定义字段
    #[serde(flatten)]
    pub extra: Option<Value>,
}

impl UrlPreviewService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 批量更新URL预览
    pub async fn batch_update(
        &self,
        message_id: &str,
        request: BatchUpdateUrlPreviewRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<EmptyResponse> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::PATCH);
        api_req.set_api_path(EndpointBuilder::replace_param(
            openlark_core::endpoints::im::IM_V1_MESSAGE_URL_PREVIEW_BATCH_UPDATE,
            "message_id",
            message_id,
        ));
        api_req
            .set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);
        api_req.body = serde_json::to_vec(&request)?;

        let api_resp: BaseResponse<EmptyResponse> =
            Transport::request(api_req, &self.config, option).await?;
        api_resp.into_result()
    }
}
