//! 查询会议室层级详情
//!
//! docPath: https://open.feishu.cn/document/server-docs/vc-v1/room_level/get

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};

use crate::common::api_utils::extract_response_data;
use crate::endpoints::VC_V1_ROOM_LEVELS;

/// 查询会议室层级详情请求
pub struct GetRoomLevelRequest {
    config: Config,
    room_level_id: String,
    query_params: Vec<(String, String)>,
}

impl GetRoomLevelRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            room_level_id: String::new(),
            query_params: Vec::new(),
        }
    }

    /// 会议室层级 ID（路径参数）
    pub fn room_level_id(mut self, room_level_id: impl Into<String>) -> Self {
        self.room_level_id = room_level_id.into();
        self
    }

    /// 追加查询参数
    pub fn query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query_params.push((key.into(), value.into()));
        self
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/vc-v1/room_level/get
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        validate_required!(self.room_level_id, "room_level_id 不能为空");

        // url: GET:/open-apis/vc/v1/room_levels/:room_level_id
        let mut req: ApiRequest<serde_json::Value> =
            ApiRequest::get(format!("{}/{}", VC_V1_ROOM_LEVELS, self.room_level_id));
        for (k, v) in self.query_params {
            req = req.query(k, v);
        }

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "查询会议室层级详情")
    }
}
