//! 删除建筑物
//!
//! docPath: https://open.feishu.cn/document/server-docs/historic-version/meeting_room-v1/api-reference/delete-building

use openlark_core::{api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult};

use crate::{common::api_utils::extract_response_data, endpoints::MEETING_ROOM};

/// 删除建筑物请求
pub struct DeleteBuildingRequest {
    config: Config,
    building_id: String,
}

impl DeleteBuildingRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            building_id: String::new(),
        }
    }

    /// 建筑物 ID
    pub fn building_id(mut self, building_id: impl Into<String>) -> Self {
        self.building_id = building_id.into();
        self
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/historic-version/meeting_room-v1/api-reference/delete-building
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        validate_required!(self.building_id, "building_id 不能为空");

        // url: POST:/open-apis/meeting_room/building/delete
        let req: ApiRequest<serde_json::Value> = ApiRequest::post(format!(
            "{}/building/delete",
            MEETING_ROOM
        ))
        .body(serde_json::json!({
            "building_id": self.building_id
        }));

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "删除建筑物")
    }
}
