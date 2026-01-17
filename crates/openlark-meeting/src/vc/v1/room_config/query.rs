//! 查询会议室配置
//!
//! docPath: https://open.feishu.cn/document/server-docs/historic-version/meeting_room-v1/room_config/query

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::VcApiV1;
use crate::common::api_utils::extract_response_data;

/// 查询会议室配置请求

#[derive(Debug, Clone)]
pub struct QueryRoomConfigRequest {
    /// 配置信息
    config: Config,
}

/// 查询会议室配置响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryRoomConfigResponse {
    /// 会议室配置列表
    pub configs: Vec<RoomConfigItem>,
}

/// 会议室配置项
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RoomConfigItem {
    /// 配置 ID
    pub config_id: String,
    /// 配置名称
    pub name: String,
}

impl ApiResponseTrait for QueryRoomConfigResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl QueryRoomConfigRequest {
    /// 创建新的请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// 说明：该接口请求体字段较多，建议直接按文档构造 JSON 传入。
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/historic-version/meeting_room-v1/room_config/query
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<QueryRoomConfigResponse> {
        let api_endpoint = VcApiV1::RoomConfigList;
        let api_request: ApiRequest<QueryRoomConfigResponse> =
            ApiRequest::get(api_endpoint.to_url()).body(serde_json::to_vec(&body)?);

        let response = Transport::request(api_request, &self.config, None).await?;
        extract_response_data(response, "查询会议室配置")
    }
}

/// 查询会议室配置请求构建器

#[derive(Debug, Clone)]
pub struct QueryRoomConfigRequestBuilder {
    request: QueryRoomConfigRequest,
}

impl QueryRoomConfigRequestBuilder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: QueryRoomConfigRequest::new(config),
        }
    }

    /// 构建请求
    pub fn build(self) -> QueryRoomConfigRequest {
        self.request
    }
}
