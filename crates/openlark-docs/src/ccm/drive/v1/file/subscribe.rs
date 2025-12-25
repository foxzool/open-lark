use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
/// 订阅文件更新
///
/// 订阅文件的更新通知
/// docPath: /document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/subscribe
/// doc: https://open.feishu.cn/document/server-docs/docs/drive-v1/event/subscribe
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

/// 订阅文件请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscribeFileRequest {
    #[serde(skip)]
    config: Config,
    /// 文件token
    pub file_token: String,
    /// 订阅类型
    pub event_type: Option<String>,
}

impl SubscribeFileRequest {
    /// 创建订阅文件请求
    ///
    /// # 参数
    /// * `config` - 配置
    /// * `file_token` - 文件token
    pub fn new(config: Config, file_token: impl Into<String>) -> Self {
        Self {
            config,
            file_token: file_token.into(),
            event_type: None,
        }
    }

    /// 设置订阅类型
    pub fn event_type(mut self, event_type: impl Into<String>) -> Self {
        self.event_type = Some(event_type.into());
        self
    }

    pub async fn execute(self) -> SDKResult<SubscribeFileResponse> {
        if self.file_token.is_empty() {
            return Err(openlark_core::error::validation_error(
                "file_token",
                "file_token 不能为空",
            ));
        }

        let api_endpoint = DriveApi::SubscribeFile(self.file_token.clone());
        let mut request = ApiRequest::<SubscribeFileResponse>::post(&api_endpoint.to_url());

        if let Some(et) = &self.event_type {
            request = request.query("event_type", et);
        }

        let response = Transport::request(request, &self.config, None).await?;
        extract_response_data(response, "订阅云文档事件")
    }
}

/// 订阅文件响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscribeFileResponse {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subscribe_file_request_builder() {
        let config = Config::default();
        let request = SubscribeFileRequest::new(config, "file_token");

        assert_eq!(request.file_token, "file_token");
    }

    #[test]
    fn test_subscribe_data_structure() {
        let subscribe_data = SubscribeFileResponse {
            subscribed: true,
            file_token: "file_token".to_string(),
        };

        assert!(subscribe_data.subscribed);
        assert_eq!(subscribe_data.file_token, "file_token");
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(SubscribeFileResponse::data_format(), ResponseFormat::Data);
    }
}
