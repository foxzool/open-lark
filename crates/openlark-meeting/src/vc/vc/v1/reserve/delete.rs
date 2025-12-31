//! 删除预约
//!
//! docPath: https://open.feishu.cn/document/server-docs/vc-v1/reserve/delete

use openlark_core::{api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult};

use crate::{common::api_utils::extract_response_data, endpoints::VC_V1_RESERVES};

/// 删除预约请求
pub struct DeleteReserveRequest {
    config: Config,
    reserve_id: String,
}

impl DeleteReserveRequest {
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
    /// docPath: https://open.feishu.cn/document/server-docs/vc-v1/reserve/delete
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        validate_required!(self.reserve_id, "reserve_id 不能为空");

        // url: DELETE:/open-apis/vc/v1/reserves/:reserve_id
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::delete(format!("{}/{}", VC_V1_RESERVES, self.reserve_id));

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "删除预约")
    }
}

