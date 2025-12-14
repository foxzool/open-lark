use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
/// 创建订阅
///
/// 为文件创建订阅，用于接收文件更新通知
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/file/subscription/create
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

/// 创建订阅请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFileSubscriptionRequest {
    /// 文件token
    pub file_token: String,
    /// 订阅类型
    pub subscription_type: String,
    /// 回调URL
    pub url: String,
}

impl CreateFileSubscriptionRequest {
    /// 创建订阅请求
    ///
    /// # 参数
    /// * `file_token` - 文件token
    /// * `subscription_type` - 订阅类型
    /// * `url` - 回调URL
    pub fn new(
        file_token: impl Into<String>,
        subscription_type: impl Into<String>,
        url: impl Into<String>,
    ) -> Self {
        Self {
            file_token: file_token.into(),
            subscription_type: subscription_type.into(),
            url: url.into(),
        }
    }
}

/// 订阅信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionInfo {
    /// 订阅ID
    pub subscription_id: String,
    /// 订阅类型
    pub subscription_type: String,
    /// 回调URL
    pub url: String,
    /// 创建时间
    pub create_time: i64,
}

/// 创建订阅响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFileSubscriptionResponse {
    /// 订阅信息
    pub data: Option<SubscriptionInfo>,
}

impl ApiResponseTrait for CreateFileSubscriptionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建订阅
///
/// 为文件创建订阅，用于接收文件更新通知
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/file/subscription/create
pub async fn create_file_subscription(
    request: CreateFileSubscriptionRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<CreateFileSubscriptionResponse>> {
    // 使用DriveApi枚举生成API端点
    let api_endpoint = DriveApi::CreateFileSubscription(request.file_token.clone());

    // 创建API请求
    let mut api_request: ApiRequest<CreateFileSubscriptionResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(serde_json::json!({
            "subscription_type": request.subscription_type,
            "url": request.url
        }));

    // 如果有请求选项，应用它们
    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    // 发送请求
    Transport::request(api_request, config, None).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_file_subscription_request_builder() {
        let request = CreateFileSubscriptionRequest::new(
            "file_token",
            "file_updated",
            "https://example.com/webhook",
        );

        assert_eq!(request.file_token, "file_token");
        assert_eq!(request.subscription_type, "file_updated");
        assert_eq!(request.url, "https://example.com/webhook");
    }

    #[test]
    fn test_subscription_info_structure() {
        let subscription = SubscriptionInfo {
            subscription_id: "subscription_id".to_string(),
            subscription_type: "file_updated".to_string(),
            url: "https://example.com/webhook".to_string(),
            create_time: 1640995200,
        };

        assert_eq!(subscription.subscription_id, "subscription_id");
        assert_eq!(subscription.subscription_type, "file_updated");
        assert_eq!(subscription.url, "https://example.com/webhook");
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(
            CreateFileSubscriptionResponse::data_format(),
            ResponseFormat::Data
        );
    }
}