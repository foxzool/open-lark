//! 创建会议室
//!
//! docPath: https://open.feishu.cn/document/server-docs/vc-v1/room/create

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

use crate::common::api_utils::{extract_response_data, serialize_params};
use crate::endpoints::VC_V1_ROOMS;

/// 创建会议室请求
pub struct CreateRoomRequest {
    config: Config,
}

impl CreateRoomRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// 说明：该接口请求体字段较多，建议直接按文档构造 JSON 传入。
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/vc-v1/room/create
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<serde_json::Value> {
        // url: POST:/open-apis/vc/v1/rooms
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::post(VC_V1_ROOMS).body(serialize_params(&body, "创建会议室")?);

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "创建会议室")
    }
}

