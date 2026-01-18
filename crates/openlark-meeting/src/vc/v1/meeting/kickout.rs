//! 移除参会人
//!
//! docPath: https://open.feishu.cn/document/server-docs/vc-v1/meeting/kickout

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::VcApiV1;
use crate::common::api_utils::{extract_response_data, serialize_params};

/// 移除参会人请求
#[derive(Debug, Clone)]
pub struct KickoutMeetingRequest {
    config: Config,
    meeting_id: String,
}

/// 移除参会人响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct KickoutMeetingResponse {
    /// 操作结果
    pub success: bool,
}

impl ApiResponseTrait for KickoutMeetingResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl KickoutMeetingRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            meeting_id: String::new(),
        }
    }

    /// 会议 ID（路径参数）
    pub fn meeting_id(mut self, meeting_id: impl Into<String>) -> Self {
        self.meeting_id = meeting_id.into();
        self
    }

    /// 执行请求
    ///
    /// 说明：该接口请求体字段较多，建议直接按文档构造 JSON 传入。
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/vc-v1/meeting/kickout
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<KickoutMeetingResponse> {
        validate_required!(self.meeting_id, "meeting_id 不能为空");

        let api_endpoint = VcApiV1::MeetingKickout(self.meeting_id);
        let req: ApiRequest<KickoutMeetingResponse> =
            ApiRequest::post(api_endpoint.to_url()).body(serialize_params(&body, "移除参会人")?);

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "移除参会人")
    }
}

/// 移除参会人请求构建器
#[derive(Debug, Clone)]
pub struct KickoutMeetingRequestBuilder {
    request: KickoutMeetingRequest,
}

impl KickoutMeetingRequestBuilder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: KickoutMeetingRequest::new(config),
        }
    }

    /// 设置会议ID
    pub fn meeting_id(mut self, meeting_id: impl Into<String>) -> Self {
        self.request = self.request.meeting_id(meeting_id);
        self
    }

    /// 构建请求
    pub fn build(self) -> KickoutMeetingRequest {
        self.request
    }
}
