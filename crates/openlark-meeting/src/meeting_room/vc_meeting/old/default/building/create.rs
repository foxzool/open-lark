//! 创建建筑物
//!
//! docPath: https://open.feishu.cn/document/server-docs/historic-version/meeting_room-v1/api-reference/create-building

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

use crate::common::api_utils::{extract_response_data, serialize_params};
use crate::endpoints::MEETING_ROOM;

/// 创建建筑物请求
pub struct CreateBuildingRequest {
    config: Config,
}

impl CreateBuildingRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/historic-version/meeting_room-v1/api-reference/create-building
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<serde_json::Value> {
        // url: POST:/open-apis/meeting_room/building/create
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::post(format!("{}/building/create", MEETING_ROOM))
                .body(serialize_params(&body, "创建建筑物")?);

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "创建建筑物")
    }
}

