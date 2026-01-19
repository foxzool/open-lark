//! 创建标签
//!
//! docPath: https://open.feishu.cn/document/tenant-tag/create

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    endpoints::IM_V2_TAGS,
};

/// 创建标签请求
pub struct CreateTagRequest {
    config: Config,
}

impl CreateTagRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// 说明：该接口请求体字段较多，建议直接按文档构造 JSON 传入。
    ///
    /// docPath: https://open.feishu.cn/document/tenant-tag/create
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<serde_json::Value> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        body: serde_json::Value,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<serde_json::Value> {

        // url: POST:/open-apis/im/v2/tags
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::post(IM_V2_TAGS).body(serialize_params(&body, "创建标签")?);

        
        let resp = Transport::request(req, &self.config, Some(option)).await?;

        extract_response_data(resp, "创建标签")
}
}
