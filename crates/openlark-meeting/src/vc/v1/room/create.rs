//! 创建会议室
//!
//! docPath: https://open.feishu.cn/document/server-docs/vc-v1/room/create

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::validation_error,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 创建会议室请求
#[allow(dead_code)]
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
        // 🚀 使用新的枚举+builder系统生成API端点
        // 替代传统的字符串拼接方式，提供类型安全和IDE自动补全
        use crate::common::api_endpoints::VcApiV1;
        let api_endpoint = VcApiV1::RoomCreate;

        // 创建API请求 - 使用类型安全的URL生成
        let api_request: ApiRequest<CreateRoomResponse> =
            ApiRequest::post(api_endpoint.to_url()).body(serde_json::to_vec(&body)?);

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response
            .data
            .ok_or_else(|| validation_error("响应数据为空", "服务器没有返回有效的数据"))
    }
}

/// 创建会议室请求构建器
#[allow(dead_code)]
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
