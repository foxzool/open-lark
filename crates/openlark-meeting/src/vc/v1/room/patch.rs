//! 更新会议室
//!
//! docPath: https://open.feishu.cn/document/server-docs/vc-v1/room/patch

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::validation_error,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 更新会议室请求
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PatchRoomRequest {
    /// 配置信息
    config: Config,
    /// 会议室 ID（路径参数）
    room_id: String,
}

/// 更新会议室响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PatchRoomResponse {
    /// 更新状态
    pub success: bool,
}

impl ApiResponseTrait for PatchRoomResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl PatchRoomRequest {
    /// 创建新的请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            room_id: String::new(),
        }
    }

    /// 设置会议室 ID（路径参数）
    pub fn room_id(mut self, room_id: impl Into<String>) -> Self {
        self.room_id = room_id.into();
        self
    }

    /// 执行请求
    ///
    /// 说明：该接口请求体字段较多，建议直接按文档构造 JSON 传入。
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/vc-v1/room/patch
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<PatchRoomResponse> {
        // 参数验证
        if self.room_id.trim().is_empty() {
            return Err(validation_error("room_id", "会议室 ID 不能为空"));
        }

        // 🚀 使用新的枚举+builder系统生成API端点
        use crate::common::api_endpoints::VcApiV1;
        let api_endpoint = VcApiV1::RoomPatch(self.room_id.clone());

        // 创建API请求 - 使用类型安全的URL生成
        let api_request: ApiRequest<PatchRoomResponse> =
            ApiRequest::patch(api_endpoint.to_url()).body(serde_json::to_vec(&body)?);

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response
            .data
            .ok_or_else(|| validation_error("响应数据为空", "服务器没有返回有效的数据"))
    }
}

/// 更新会议室请求构建器
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PatchRoomRequestBuilder {
    request: PatchRoomRequest,
}

impl PatchRoomRequestBuilder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: PatchRoomRequest::new(config),
        }
    }

    /// 设置会议室 ID
    pub fn room_id(mut self, room_id: impl Into<String>) -> Self {
        self.request = self.request.room_id(room_id);
        self
    }

    /// 构建请求
    pub fn build(self) -> PatchRoomRequest {
        self.request
    }
}
