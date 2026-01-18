//! 查询会议明细
//!
//! docPath: https://open.feishu.cn/document/server-docs/vc-v1/meeting-room-data/get

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::VcApiV1;
use crate::common::api_utils::extract_response_data;

/// 查询会议明细请求
#[derive(Debug, Clone)]
pub struct GetMeetingListRequest {
    config: Config,
    query_params: Vec<(String, String)>,
}

/// 查询会议明细响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetMeetingListResponse {
    /// 会议列表数据
    pub data: serde_json::Value,
}

impl ApiResponseTrait for GetMeetingListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl GetMeetingListRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            query_params: Vec::new(),
        }
    }

    /// 追加查询参数
    pub fn query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query_params.push((key.into(), value.into()));
        self
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/vc-v1/meeting-room-data/get
    pub async fn execute(self) -> SDKResult<GetMeetingListResponse> {
        let api_endpoint = VcApiV1::MeetingListList;
        let mut req: ApiRequest<GetMeetingListResponse> = ApiRequest::get(api_endpoint.to_url());
        for (k, v) in self.query_params {
            req = req.query(k, v);
        }

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "查询会议明细")
    }
}

/// 查询会议明细请求构建器
#[derive(Debug, Clone)]
pub struct GetMeetingListRequestBuilder {
    request: GetMeetingListRequest,
}

impl GetMeetingListRequestBuilder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: GetMeetingListRequest::new(config),
        }
    }

    /// 添加查询参数
    pub fn query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.request = self.request.query_param(key, value);
        self
    }

    /// 构建请求
    pub fn build(self) -> GetMeetingListRequest {
        self.request
    }
}
