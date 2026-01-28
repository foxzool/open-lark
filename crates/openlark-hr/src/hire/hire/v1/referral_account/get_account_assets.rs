//! 查询内推账户
//!
//! docPath: https://open.feishu.cn/document/server-docs/hire-v1/referral_account/get_account_assets

use openlark_core::{
    api::{ApiResponseTrait, ResponseFormat},
    config::Config,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 查询内推账户请求
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct GetAccountAssetsRequest {
    /// 配置信息
    config: Config,
    // TODO: 添加请求字段
}

impl GetAccountAssetsRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            // TODO: 初始化字段
        }
    }

    // TODO: 添加字段 setter 方法

    /// 执行请求
    pub async fn execute(self) -> SDKResult<GetAccountAssetsResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        _option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetAccountAssetsResponse> {
        // TODO: 实现 API 调用逻辑
        todo!("实现 查询内推账户 API 调用")
    }
}

/// 查询内推账户响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetAccountAssetsResponse {
    /// 响应数据
    ///
    /// TODO: 根据官方文档添加具体字段
    pub data: Value,
}

impl ApiResponseTrait for GetAccountAssetsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
