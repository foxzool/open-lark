//! 批量查询会议室详情
//!
//! docPath: https://open.feishu.cn/document/server-docs/vc-v1/room/mget

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::validation_error,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 批量查询会议室详情请求
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MgetRoomRequest {
    /// 配置信息
    config: Config,
}

/// 批量查询会议室详情响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MgetRoomResponse {
    /// 会议室列表
    pub rooms: Vec<RoomItem>,
}

/// 会议室信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RoomItem {
    /// 会议室 ID
    pub room_id: String,
    /// 会议室名称
    pub name: String,
    /// 会议室容量
    pub capacity: i32,
}

impl ApiResponseTrait for MgetRoomResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl MgetRoomRequest {
    /// 创建新的请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/vc-v1/room/mget
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<MgetRoomResponse> {
        // 🚀 使用新的枚举+builder系统生成API端点
        use crate::common::api_endpoints::VcApiV1;
        let api_endpoint = VcApiV1::RoomBatchGet;

        // 创建API请求 - 使用类型安全的URL生成
        let api_request: ApiRequest<MgetRoomResponse> =
            ApiRequest::post(api_endpoint.to_url()).body(serde_json::to_vec(&body)?);

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response
            .data
            .ok_or_else(|| validation_error("响应数据为空", "服务器没有返回有效的数据"))
    }
}

/// 批量查询会议室详情请求构建器
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MgetRoomRequestBuilder {
    request: MgetRoomRequest,
}

impl MgetRoomRequestBuilder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: MgetRoomRequest::new(config),
        }
    }

    /// 构建请求
    pub fn build(self) -> MgetRoomRequest {
        self.request
    }
}
