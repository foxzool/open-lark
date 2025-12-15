/// 获取订阅状态
///
/// 根据订阅ID获取该订阅的状态
/// docPath: https://open.feishu.cn/document/server-docs/docs/docs-assistant/file-subscription/get
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

/// 获取订阅状态请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSubscriptionRequest {
    /// 文件token
    pub file_token: String,
    /// 订阅ID
    pub subscription_id: String,
}

impl GetSubscriptionRequest {
    /// 创建获取订阅状态请求
    ///
    /// # 参数
    /// * `file_token` - 文件token
    /// * `subscription_id` - 订阅ID
    pub fn new(
        file_token: impl Into<String>,
        subscription_id: impl Into<String>,
    ) -> Self {
        Self {
            file_token: file_token.into(),
            subscription_id: subscription_id.into(),
        }
    }
}

/// 获取订阅状态响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSubscriptionResponse {
    /// 订阅信息
    pub subscription: SubscriptionInfo,
}

/// 订阅信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionInfo {
    /// 订阅ID
    pub subscription_id: String,
    /// 订阅类型
    pub subscription_type: String,
    /// 订阅状态
    pub status: String,
    /// 订阅者信息
    pub subscriber: SubscriberInfo,
    /// 文件token
    pub file_token: String,
    /// 创建时间
    pub create_time: String,
    /// 更新时间
    pub update_time: String,
}

/// 订阅者信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriberInfo {
    /// 用户ID
    pub user_id: String,
    /// 用户名称
    pub name: String,
    /// 用户类型
    pub user_type: String,
}

impl ApiResponseTrait for GetSubscriptionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取订阅状态
///
/// 根据订阅ID获取该订阅的状态
/// docPath: https://open.feishu.cn/document/server-docs/docs/docs-assistant/file-subscription/get
pub async fn get_subscription(
    request: GetSubscriptionRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<GetSubscriptionResponse>> {
    // 创建API请求
    let mut api_request: ApiRequest<GetSubscriptionResponse> =
        ApiRequest::get(&format!("/open-apis/drive/v1/files/{}/subscriptions/{}",
            request.file_token, request.subscription_id));

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
    fn test_get_subscription_request_builder() {
        let request = GetSubscriptionRequest::new("file_token", "subscription_123");

        assert_eq!(request.file_token, "file_token");
        assert_eq!(request.subscription_id, "subscription_123");
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(GetSubscriptionResponse::data_format(), ResponseFormat::Data);
    }
}