//! 全额提取内推账户余额
//!
//! docPath: https://open.feishu.cn/document/server-docs/hire-v1/referral_account/withdraw

use openlark_core::{
    api::{ApiResponseTrait, ResponseFormat},
    config::Config, SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 全额提取内推账户余额请求
#[derive(Debug, Clone)]
pub struct WithdrawRequest {
    /// 配置信息
    config: Config,
    // TODO: 添加请求字段
}

impl WithdrawRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            // TODO: 初始化字段
        }
    }

    // TODO: 添加字段 setter 方法

    /// 执行请求
    pub async fn execute(self) -> SDKResult<WithdrawResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        _option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<WithdrawResponse> {
        // TODO: 实现 API 调用逻辑
        todo!("实现 全额提取内推账户余额 API 调用")
    }
}

/// 全额提取内推账户余额响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WithdrawResponse {
    /// 响应数据
    ///
    /// TODO: 根据官方文档添加具体字段
    pub data: Value,
}

impl ApiResponseTrait for WithdrawResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
