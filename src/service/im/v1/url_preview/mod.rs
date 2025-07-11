use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::core::{
    api_req::ApiRequest,
    api_resp::{BaseResponse, EmptyResponse},
    config::Config,
    constants::AccessTokenType,
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
    ) -> SDKResult<BaseResponse<EmptyResponse>> {
        let api_req = ApiRequest {
            http_method: Method::PATCH,
            api_path: format!("/open-apis/im/v1/messages/{message_id}/url_preview/batch_update"),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}
