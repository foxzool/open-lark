//! 停用内推账户
//!
//! docPath: https://open.feishu.cn/document/server-docs/hire-v1/referral_account/deactivate

use openlark_core::{
    api::{ApiResponseTrait, ResponseFormat},
    config::Config, SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 停用内推账户请求
#[derive(Debug, Clone)]
pub struct DeactivateRequest {
    /// 配置信息
    config: Config,
    // TODO: 添加请求字段
}

impl DeactivateRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            // TODO: 初始化字段
        }
    }

    // TODO: 添加字段 setter 方法

    /// 执行请求
    pub async fn execute(self) -> SDKResult<DeactivateResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        _option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<DeactivateResponse> {
        // TODO: 实现 API 调用逻辑
        todo!("实现 停用内推账户 API 调用")
    }
}

/// 停用内推账户响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeactivateResponse {
    /// 响应数据
    ///
    /// TODO: 根据官方文档添加具体字段
    pub data: Value,
}

impl ApiResponseTrait for DeactivateResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
