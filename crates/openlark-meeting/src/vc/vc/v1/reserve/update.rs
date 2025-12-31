//! 更新预约
//!
//! docPath: https://open.feishu.cn/document/server-docs/vc-v1/reserve/update

use openlark_core::{api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    endpoints::VC_V1_RESERVES,
};

/// 更新预约请求
pub struct UpdateReserveRequest {
    config: Config,
    reserve_id: String,
}

impl UpdateReserveRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            reserve_id: String::new(),
        }
    }

    /// 预约 ID（路径参数）
    pub fn reserve_id(mut self, reserve_id: impl Into<String>) -> Self {
        self.reserve_id = reserve_id.into();
        self
    }

    /// 执行请求
    ///
    /// 说明：该接口请求体字段较多，建议直接按文档构造 JSON 传入。
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/vc-v1/reserve/update
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<serde_json::Value> {
        validate_required!(self.reserve_id, "reserve_id 不能为空");

        // url: PUT:/open-apis/vc/v1/reserves/:reserve_id
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::put(format!("{}/{}", VC_V1_RESERVES, self.reserve_id))
                .body(serialize_params(&body, "更新预约")?);

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "更新预约")
    }
}

