//! 批量新增用户
//!
//! docPath: https://open.feishu.cn/document/ukTMukTMukTM/uIDOwUjLygDM14iM4ATN

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    endpoints::CONTACT_V2_USER_BATCH_ADD,
};

/// 批量新增用户请求
pub struct BatchAddUsersRequest {
    config: Config,
}

impl BatchAddUsersRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/ukTMukTMukTM/uIDOwUjLygDM14iM4ATN
    pub async fn execute(self, params: serde_json::Value) -> SDKResult<serde_json::Value> {
        self.execute_with_options(params, openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        params: serde_json::Value,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<serde_json::Value> {
        // url: POST:/open-apis/contact/v2/user/batch_add
        let req: ApiRequest<serde_json::Value> = ApiRequest::post(CONTACT_V2_USER_BATCH_ADD)
            .body(serialize_params(&params, "批量新增用户")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;

        extract_response_data(resp, "批量新增用户")
    }
}
