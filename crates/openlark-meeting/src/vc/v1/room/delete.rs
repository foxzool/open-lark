//! 删除会议室
//!
//! docPath: https://open.feishu.cn/document/server-docs/vc-v1/room/delete

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use crate::common::api_utils::{extract_response_data, validate_required_field};
use serde::{Deserialize, Serialize};

/// 删除会议室请求

#[derive(Debug, Clone)]
pub struct DeleteRoomRequest {
    /// 配置信息
    config: Config,
    /// 会议室 ID（路径参数）
    room_id: String,
}

/// 删除会议室响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteRoomResponse {
    /// 删除状态
    pub success: bool,
}

impl ApiResponseTrait for DeleteRoomResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl DeleteRoomRequest {
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
    /// docPath: https://open.feishu.cn/document/server-docs/vc-v1/room/delete
    pub async fn execute(self) -> SDKResult<DeleteRoomResponse> {
        use crate::common::api_endpoints::VcApiV1;

        validate_required_field("room_id", Some(&self.room_id), "会议室 ID 不能为空")?;

        let api_endpoint = VcApiV1::RoomDelete(self.room_id.clone());
        let api_request: ApiRequest<DeleteRoomResponse> = ApiRequest::delete(api_endpoint.to_url());

        let response = Transport::request(api_request, &self.config, None).await?;
        extract_response_data(response, "删除会议室")
    }
}

/// 删除会议室请求构建器

#[derive(Debug, Clone)]
pub struct DeleteRoomRequestBuilder {
    request: DeleteRoomRequest,
}

impl DeleteRoomRequestBuilder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: DeleteRoomRequest::new(config),
        }
    }

    /// 设置会议室 ID
    pub fn room_id(mut self, room_id: impl Into<String>) -> Self {
        self.request = self.request.room_id(room_id);
        self
    }

    /// 构建请求
    pub fn build(self) -> DeleteRoomRequest {
        self.request
    }
}
