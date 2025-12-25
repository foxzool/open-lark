use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

/// 查询云文档事件订阅状态
///
/// 查询文件的订阅状态。
/// docPath: /document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/get_subscribe
/// doc: https://open.feishu.cn/document/docs/drive-v1/event/get_subscribe

/// 获取文件的订阅状态请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSubscribeRequest {
    #[serde(skip)]
    config: Config,
    /// 文件token
    pub file_token: String,
    /// 订阅类型
    pub event_type: Option<String>,
}

impl GetSubscribeRequest {
    /// 创建获取文件订阅状态请求
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

    pub async fn execute(self) -> SDKResult<GetSubscribeResponse> {
        if self.file_token.is_empty() {
            return Err(openlark_core::error::validation_error(
                "file_token",
                "file_token 不能为空",
            ));
        }

        let api_endpoint = DriveApi::GetFileSubscribe(self.file_token.clone());
        let mut request = ApiRequest::<GetSubscribeResponse>::get(&api_endpoint.to_url());

        if let Some(et) = &self.event_type {
            request = request.query("event_type", et);
        }

        let response = Transport::request(request, &self.config, None).await?;
        extract_response_data(response, "查询云文档事件订阅状态")
    }
}

/// 获取文件订阅状态响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSubscribeResponse {
    /// 是否已订阅
    pub is_subscribe: bool,
}

impl ApiResponseTrait for GetSubscribeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_subscribe_request_builder() {
        let config = Config::default();
        let request = GetSubscribeRequest::new(config, "file_token");

        assert_eq!(request.file_token, "file_token");
        assert!(request.event_type.is_none());
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(GetSubscribeResponse::data_format(), ResponseFormat::Data);
    }
}
