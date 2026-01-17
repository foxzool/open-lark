//! 预约会议
//!
//! docPath: https://open.feishu.cn/document/server-docs/vc-v1/reserve/apply

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::extract_response_data;

/// 预约会议请求

#[derive(Debug, Clone)]
pub struct ApplyReserveRequest {
    /// 配置信息
    config: Config,
}

/// 预约会议响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApplyReserveResponse {
    /// 会议 ID
    pub meeting_id: String,
    /// 预约 ID
    pub reserve_id: String,
}

impl ApiResponseTrait for ApplyReserveResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApplyReserveRequest {
    /// 创建新的请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// 说明：该接口请求体字段较多，建议直接按文档构造 JSON 传入。
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/vc-v1/reserve/apply
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<ApplyReserveResponse> {
        use crate::common::api_endpoints::VcApiV1;

        let api_endpoint = VcApiV1::ReserveCreate;
        let api_request: ApiRequest<ApplyReserveResponse> =
            ApiRequest::post(api_endpoint.to_url()).body(serde_json::to_vec(&body)?);

        let response = Transport::request(api_request, &self.config, None).await?;
        extract_response_data(response, "预约会议")
    }
}

/// 预约会议请求构建器

#[derive(Debug, Clone)]
pub struct ApplyReserveRequestBuilder {
    request: ApplyReserveRequest,
}

impl ApplyReserveRequestBuilder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: ApplyReserveRequest::new(config),
        }
    }

    /// 构建请求
    pub fn build(self) -> ApplyReserveRequest {
        self.request
    }
}
