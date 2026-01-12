//! 获取预约
//!
//! docPath: https://open.feishu.cn/document/server-docs/vc-v1/reserve/get

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::validation_error,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::VcApiV1;

/// 获取预约请求
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GetReserveRequest {
    /// 配置信息
    config: Config,
    /// 预约 ID（路径参数）
    reserve_id: String,
    /// 查询参数
    query_params: Vec<(String, String)>,
}

/// 获取预约响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetReserveResponse {
    /// 预约 ID
    pub reserve_id: String,
    /// 会议 ID
    pub meeting_id: String,
    /// 预约主题
    pub topic: String,
    /// 开始时间
    pub start_time: String,
    /// 结束时间
    pub end_time: String,
}

impl ApiResponseTrait for GetReserveResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl GetReserveRequest {
    /// 创建新的请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            reserve_id: String::new(),
            query_params: Vec::new(),
        }
    }

    /// 设置预约 ID（路径参数）
    pub fn reserve_id(mut self, reserve_id: impl Into<String>) -> Self {
        self.reserve_id = reserve_id.into();
        self
    }

    /// 追加查询参数
    pub fn query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query_params.push((key.into(), value.into()));
        self
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/vc-v1/reserve/get
    pub async fn execute(self) -> SDKResult<GetReserveResponse> {
        // 参数验证
        if self.reserve_id.trim().is_empty() {
            return Err(validation_error("reserve_id", "预约 ID 不能为空"));
        }

        // 🚀 使用新的枚举+builder系统生成API端点
        let api_endpoint = VcApiV1::ReserveGet(self.reserve_id.clone());

        // 创建API请求 - 使用类型安全的URL生成
        let mut api_request: ApiRequest<GetReserveResponse> =
            ApiRequest::get(api_endpoint.to_url());

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

/// 获取预约请求构建器
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GetReserveRequestBuilder {
    request: GetReserveRequest,
}

impl GetReserveRequestBuilder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: GetReserveRequest::new(config),
        }
    }

    /// 设置预约 ID
    pub fn reserve_id(mut self, reserve_id: impl Into<String>) -> Self {
        self.request = self.request.reserve_id(reserve_id);
        self
    }

    /// 追加查询参数
    pub fn query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.request = self.request.query_param(key, value);
        self
    }

    /// 构建请求
    pub fn build(self) -> GetReserveRequest {
        self.request
    }
}
