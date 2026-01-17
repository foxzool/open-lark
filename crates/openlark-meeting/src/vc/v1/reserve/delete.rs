//! 删除预约
//!
//! docPath: https://open.feishu.cn/document/server-docs/vc-v1/reserve/delete

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::VcApiV1;
use crate::common::api_utils::{extract_response_data, validate_required_field};

/// 删除预约请求

#[derive(Debug, Clone)]
pub struct DeleteReserveRequest {
    /// 配置信息
    config: Config,
    /// 预约 ID（路径参数）
    reserve_id: String,
}

/// 删除预约响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteReserveResponse {
    /// 删除状态
    pub success: bool,
}

impl ApiResponseTrait for DeleteReserveResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl DeleteReserveRequest {
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
    /// docPath: https://open.feishu.cn/document/server-docs/vc-v1/reserve/delete
    pub async fn execute(self) -> SDKResult<DeleteReserveResponse> {
        validate_required_field("reserve_id", Some(&self.reserve_id), "预约 ID 不能为空")?;

        let api_endpoint = VcApiV1::ReserveDelete(self.reserve_id.clone());
        let api_request: ApiRequest<DeleteReserveResponse> =
            ApiRequest::delete(api_endpoint.to_url());

        let response = Transport::request(api_request, &self.config, None).await?;
        extract_response_data(response, "删除预约")
    }
}

/// 删除预约请求构建器

#[derive(Debug, Clone)]
pub struct DeleteReserveRequestBuilder {
    request: DeleteReserveRequest,
}

impl DeleteReserveRequestBuilder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: DeleteReserveRequest::new(config),
        }
    }

    /// 设置预约 ID
    pub fn reserve_id(mut self, reserve_id: impl Into<String>) -> Self {
        self.request = self.request.reserve_id(reserve_id);
        self
    }

    /// 构建请求
    pub fn build(self) -> DeleteReserveRequest {
        self.request
    }
}
