//! 查询会议室详情
//!
//! docPath: https://open.feishu.cn/document/server-docs/vc-v1/room/get

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::validation_error,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::VcApiV1;

/// 查询会议室详情请求
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GetRoomRequest {
    /// 配置信息
    config: Config,
    /// 会议室 ID（路径参数）
    room_id: String,
    /// 查询参数
    query_params: Vec<(String, String)>,
}

/// 查询会议室详情响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetRoomResponse {
    /// 会议室 ID
    pub room_id: String,
    /// 会议室名称
    pub name: String,
    /// 会议室层级 ID
    pub room_level_id: String,
    /// 会议室容量
    pub capacity: i32,
    /// 建筑物 ID
    pub building_id: String,
    /// 楼层
    pub floor: Option<String>,
    /// 邮箱
    pub email: Option<String>,
    /// 会议室状态
    pub status: String,
    /// 是否启用
    pub active: bool,
    /// 是否需要审批
    pub approval_required: bool,
}

impl ApiResponseTrait for GetRoomResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl GetRoomRequest {
    /// 创建新的查询请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            room_id: String::new(),
            query_params: Vec::new(),
        }
    }

    /// 设置会议室 ID（路径参数）
    pub fn room_id(mut self, room_id: impl Into<String>) -> Self {
        self.room_id = room_id.into();
        self
    }

    /// 追加查询参数
    pub fn query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query_params.push((key.into(), value.into()));
        self
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/vc-v1/room/get
    pub async fn execute(self) -> SDKResult<GetRoomResponse> {
        // 参数验证
        if self.room_id.trim().is_empty() {
            return Err(validation_error("room_id", "会议室 ID 不能为空"));
        }

        // 🚀 使用新的枚举+builder系统生成API端点
        // 替代传统的字符串拼接方式，提供类型安全和IDE自动补全
        let api_endpoint = VcApiV1::RoomGet(self.room_id.clone());

        // 创建API请求 - 使用类型安全的URL生成
        let mut api_request: ApiRequest<GetRoomResponse> = ApiRequest::get(&api_endpoint.to_url());

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

/// 查询会议室详情请求构建器
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GetRoomRequestBuilder {
    request: GetRoomRequest,
}

impl GetRoomRequestBuilder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: GetRoomRequest::new(config),
        }
    }

    /// 设置会议室 ID
    pub fn room_id(mut self, room_id: impl Into<String>) -> Self {
        self.request = self.request.room_id(room_id);
        self
    }

    /// 追加查询参数
    pub fn query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.request = self.request.query_param(key, value);
        self
    }

    /// 构建请求
    pub fn build(self) -> GetRoomRequest {
        self.request
    }
}
