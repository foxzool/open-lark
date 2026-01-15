//! 更新预约
//!
//! docPath: https://open.feishu.cn/document/server-docs/vc-v1/reserve/update

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::validation_error,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::VcApiV1;

/// 更新预约请求

#[derive(Debug, Clone)]
pub struct UpdateReserveRequest {
    /// 配置信息
    config: Config,
    /// 预约 ID（路径参数）
    reserve_id: String,
}

/// 更新预约响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateReserveResponse {
    /// 更新状态
    pub success: bool,
}

impl ApiResponseTrait for UpdateReserveResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl UpdateReserveRequest {
    /// 创建新的请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            reserve_id: String::new(),
        }
    }

    /// 设置预约 ID（路径参数）
    pub fn reserve_id(mut self, reserve_id: impl Into<String>) -> Self {
        self.reserve_id = reserve_id.into();
        self
    }

    /// 执行请求
    ///
    /// 说明：该接口请求体字段较多，建议直接按文档构造 JSON 传入。
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/vc-v1/reserve/update
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<UpdateReserveResponse> {
        // 参数验证
        if self.reserve_id.trim().is_empty() {
            return Err(validation_error("reserve_id", "预约 ID 不能为空"));
        }

        // 🚀 使用新的枚举+builder系统生成API端点
        let api_endpoint = VcApiV1::ReservePatch(self.reserve_id.clone());

        // 创建API请求 - 使用类型安全的URL生成
        let api_request: ApiRequest<UpdateReserveResponse> =
            ApiRequest::patch(api_endpoint.to_url()).body(serde_json::to_vec(&body)?);

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response
            .data
            .ok_or_else(|| validation_error("响应数据为空", "服务器没有返回有效的数据"))
    }
}

/// 更新预约请求构建器

#[derive(Debug, Clone)]
pub struct UpdateReserveRequestBuilder {
    request: UpdateReserveRequest,
}

impl UpdateReserveRequestBuilder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: UpdateReserveRequest::new(config),
        }
    }

    /// 设置预约 ID
    pub fn reserve_id(mut self, reserve_id: impl Into<String>) -> Self {
        self.request = self.request.reserve_id(reserve_id);
        self
    }

    /// 构建请求
    pub fn build(self) -> UpdateReserveRequest {
        self.request
    }
}
