//! 取消候选人入职
//!
//! docPath: https://open.feishu.cn/document/server-docs/hire-v1/application/cancel_onboard

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 取消候选人入职请求
#[derive(Debug, Clone)]
pub struct CancelOnboardRequest {
    /// 配置信息
    config: Config,
    // TODO: 添加请求字段
}

impl CancelOnboardRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            // TODO: 初始化字段
        }
    }

    // TODO: 添加字段 setter 方法

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CancelOnboardResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CancelOnboardResponse> {
        // TODO: 实现 API 调用逻辑
        todo!("实现 取消候选人入职 API 调用")
    }
}

/// 取消候选人入职响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CancelOnboardResponse {
    /// 响应数据
    ///
    /// TODO: 根据官方文档添加具体字段
    pub data: Value,
}

impl ApiResponseTrait for CancelOnboardResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
