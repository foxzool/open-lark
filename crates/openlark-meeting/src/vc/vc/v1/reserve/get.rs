//! 获取预约
//!
//! docPath: https://open.feishu.cn/document/server-docs/vc-v1/reserve/get

use openlark_core::{api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult};

use crate::{common::api_utils::extract_response_data, endpoints::VC_V1_RESERVES};

/// 获取预约请求
pub struct GetReserveRequest {
    config: Config,
    reserve_id: String,
    query_params: Vec<(String, String)>,
}

impl GetReserveRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            reserve_id: String::new(),
            query_params: Vec::new(),
        }
    }

    /// 预约 ID（路径参数）
    pub fn reserve_id(mut self, reserve_id: impl Into<String>) -> Self {
        self.reserve_id = reserve_id.into();
        self
    }

    /// 追加查询参数
    pub fn query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query_params.push((key.into(), value.into()));
        self
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/vc-v1/reserve/get
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        validate_required!(self.reserve_id, "reserve_id 不能为空");

        // url: GET:/open-apis/vc/v1/reserves/:reserve_id
        let mut req: ApiRequest<serde_json::Value> =
            ApiRequest::get(format!("{}/{}", VC_V1_RESERVES, self.reserve_id));
        for (k, v) in self.query_params {
            req = req.query(k, v);
        }

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "获取预约")
    }
}

