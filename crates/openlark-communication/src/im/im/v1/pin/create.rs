//! Pin 消息
//!
//! docPath: https://open.feishu.cn/document/server-docs/im-v1/pin/create

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    endpoints::IM_V1_PINS,
    im::im::v1::pin::models::{CreatePinBody, CreatePinResponse},
};

/// Pin 消息请求
pub struct CreatePinRequest {
    config: Config,
}

impl CreatePinRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/im-v1/pin/create
    pub async fn execute(self, body: CreatePinBody) -> SDKResult<CreatePinResponse> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        body: CreatePinBody,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreatePinResponse> {

        validate_required!(body.message_id, "message_id 不能为空");

        // url: POST:/open-apis/im/v1/pins
        let req: ApiRequest<CreatePinResponse> =
            ApiRequest::post(IM_V1_PINS).body(serialize_params(&body, "Pin 消息")?);

        
        let resp = Transport::request(req, &self.config, Some(option)).await?;

        extract_response_data(resp, "Pin 消息")
}
}
