//! 查询会议室层级详情
//!
//! docPath: https://open.feishu.cn/document/server-docs/vc-v1/room_level/get

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::validation_error,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::VcApiV1;

/// 查询会议室层级详情请求

#[derive(Debug, Clone)]
pub struct GetRoomLevelRequest {
    /// 配置信息
    config: Config,
    /// 会议室层级 ID（路径参数）
    room_level_id: String,
    /// 查询参数
    query_params: Vec<(String, String)>,
}

/// 查询会议室层级详情响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetRoomLevelResponse {
    /// 会议室层级 ID
    pub room_level_id: String,
    /// 层级名称
    pub name: String,
    /// 容量范围
    pub capacity_min: Option<i32>,
    pub capacity_max: Option<i32>,
}

impl ApiResponseTrait for GetRoomLevelResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl GetRoomLevelRequest {
    /// 创建新的请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            room_level_id: String::new(),
            query_params: Vec::new(),
        }
    }

    /// 设置会议室层级 ID（路径参数）
    pub fn room_level_id(mut self, room_level_id: impl Into<String>) -> Self {
        self.room_level_id = room_level_id.into();
        self
    }

    /// 追加查询参数
    pub fn query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query_params.push((key.into(), value.into()));
        self
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/vc-v1/room_level/get
    pub async fn execute(self) -> SDKResult<GetRoomLevelResponse> {
        // 参数验证
        if self.room_level_id.trim().is_empty() {
            return Err(validation_error("room_level_id", "会议室层级 ID 不能为空"));
        }

        // 🚀 使用新的枚举+builder系统生成API端点
        let api_endpoint = VcApiV1::RoomLevelGet(self.room_level_id.clone());

        // 创建API请求 - 使用类型安全的URL生成
        let mut api_request: ApiRequest<GetRoomLevelResponse> =
            ApiRequest::get(&api_endpoint.to_url());

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

/// 查询会议室层级详情请求构建器

#[derive(Debug, Clone)]
pub struct GetRoomLevelRequestBuilder {
    request: GetRoomLevelRequest,
}

impl GetRoomLevelRequestBuilder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: GetRoomLevelRequest::new(config),
        }
    }

    /// 设置会议室层级 ID
    pub fn room_level_id(mut self, room_level_id: impl Into<String>) -> Self {
        self.request = self.request.room_level_id(room_level_id);
        self
    }

    /// 追加查询参数
    pub fn query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.request = self.request.query_param(key, value);
        self
    }

    /// 构建请求
    pub fn build(self) -> GetRoomLevelRequest {
        self.request
    }
}
