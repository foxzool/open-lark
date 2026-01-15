//! 查询会议室列表
//!
//! docPath: https://open.feishu.cn/document/server-docs/vc-v1/room/list

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::validation_error,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 查询会议室列表请求

#[derive(Debug, Clone)]
pub struct ListRoomRequest {
    /// 配置信息
    config: Config,
    /// 查询参数
    query_params: Vec<(String, String)>,
}

/// 查询会议室列表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListRoomResponse {
    /// 会议室列表
    pub rooms: Vec<RoomItem>,
    /// 页码
    pub page_token: Option<String>,
    /// 是否有下一页
    pub has_more: Option<bool>,
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

impl ApiResponseTrait for ListRoomResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ListRoomRequest {
    /// 创建新的请求
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
    /// docPath: https://open.feishu.cn/document/server-docs/vc-v1/room/list
    pub async fn execute(self) -> SDKResult<ListRoomResponse> {
        // 🚀 使用新的枚举+builder系统生成API端点
        use crate::common::api_endpoints::VcApiV1;
        let api_endpoint = VcApiV1::RoomList;

        // 创建API请求 - 使用类型安全的URL生成
        let mut api_request: ApiRequest<ListRoomResponse> = ApiRequest::get(api_endpoint.to_url());

        // 添加查询参数
        for (key, value) in self.query_params {
            api_request = api_request.query(key, value);
        }

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response
            .data
            .ok_or_else(|| validation_error("响应数据为空", "服务器没有返回有效的数据"))
    }
}

/// 查询会议室列表请求构建器

#[derive(Debug, Clone)]
pub struct ListRoomRequestBuilder {
    request: ListRoomRequest,
}

impl ListRoomRequestBuilder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: ListRoomRequest::new(config),
        }
    }

    /// 追加查询参数
    pub fn query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.request = self.request.query_param(key, value);
        self
    }

    /// 构建请求
    pub fn build(self) -> ListRoomRequest {
        self.request
    }
}
