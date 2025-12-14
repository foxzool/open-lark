use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
/// 订阅文件更新
///
/// 订阅文件的更新通知
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/event/subscribe
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

/// 订阅文件请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscribeFileRequest {
    /// 文件token
    pub file_token: String,
}

impl SubscribeFileRequest {
    /// 创建订阅文件请求
    ///
    /// # 参数
    /// * `file_token` - 文件token
    pub fn new(file_token: impl Into<String>) -> Self {
        Self {
            file_token: file_token.into(),
        }
    }
}

/// 订阅文件响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscribeFileResponse {
    /// 订阅结果
    pub data: Option<SubscribeData>,
}

/// 订阅数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscribeData {
    /// 是否订阅成功
    pub subscribed: bool,
    /// 文件token
    pub file_token: String,
}

impl ApiResponseTrait for SubscribeFileResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 订阅文件更新
///
/// 订阅文件的更新通知
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/event/subscribe
pub async fn subscribe_file(
    request: SubscribeFileRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<SubscribeFileResponse>> {
    // 使用DriveApi枚举生成API端点
    let api_endpoint = DriveApi::SubscribeFile(request.file_token.clone());

    // 创建API请求
    let mut api_request: ApiRequest<SubscribeFileResponse> =
        ApiRequest::post(&api_endpoint.to_url());

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
    fn test_subscribe_file_request_builder() {
        let request = SubscribeFileRequest::new("file_token");

        assert_eq!(request.file_token, "file_token");
    }

    #[test]
    fn test_subscribe_data_structure() {
        let subscribe_data = SubscribeData {
            subscribed: true,
            file_token: "file_token".to_string(),
        };

        assert!(subscribe_data.subscribed);
        assert_eq!(subscribe_data.file_token, "file_token");
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(
            SubscribeFileResponse::data_format(),
            ResponseFormat::Data
        );
    }
}