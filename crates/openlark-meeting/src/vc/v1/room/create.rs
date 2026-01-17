//! 创建会议室
//!
//! docPath: https://open.feishu.cn/document/server-docs/vc-v1/room/create

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::extract_response_data;

/// 创建会议室请求
#[derive(Debug, Clone)]
pub struct CreateRoomRequest {
    /// 配置信息
    config: Config,
}

/// 创建会议室响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateRoomResponse {
    /// 会议室 ID
    pub room_id: String,
}

impl ApiResponseTrait for CreateRoomResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl CreateRoomRequest {
    /// 创建新的请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// 说明：该接口请求体字段较多，建议直接按文档构造 JSON 传入。
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/vc-v1/room/create
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<CreateRoomResponse> {
        use crate::common::api_endpoints::VcApiV1;

        let api_endpoint = VcApiV1::RoomCreate;
        let api_request: ApiRequest<CreateRoomResponse> =
            ApiRequest::post(api_endpoint.to_url()).body(serde_json::to_vec(&body)?);

        let response = Transport::request(api_request, &self.config, None).await?;
        extract_response_data(response, "创建会议室")
    }
}

/// 创建会议室请求构建器
#[derive(Debug, Clone)]
pub struct CreateRoomRequestBuilder {
    request: CreateRoomRequest,
}

impl CreateRoomRequestBuilder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: CreateRoomRequest::new(config),
        }
    }

    /// 构建请求
    pub fn build(self) -> CreateRoomRequest {
        self.request
    }
}
