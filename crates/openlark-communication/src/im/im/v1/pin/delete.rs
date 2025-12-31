//! 移除 Pin 消息
//!
//! docPath: https://open.feishu.cn/document/server-docs/im-v1/pin/delete

use openlark_core::{api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult};

use crate::{
    common::{api_utils::extract_response_data, models::EmptyData},
    endpoints::IM_V1_PINS,
};

/// 移除 Pin 消息请求
pub struct DeletePinRequest {
    config: Config,
    message_id: String,
}

impl DeletePinRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            message_id: String::new(),
        }
    }

    /// 消息 ID（路径参数）
    pub fn message_id(mut self, message_id: impl Into<String>) -> Self {
        self.message_id = message_id.into();
        self
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/im-v1/pin/delete
    pub async fn execute(self) -> SDKResult<EmptyData> {
        validate_required!(self.message_id, "message_id 不能为空");

        // url: DELETE:/open-apis/im/v1/pins/:message_id
        let req: ApiRequest<EmptyData> = ApiRequest::delete(format!("{}/{}", IM_V1_PINS, self.message_id));

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "移除 Pin 消息")
    }
}

