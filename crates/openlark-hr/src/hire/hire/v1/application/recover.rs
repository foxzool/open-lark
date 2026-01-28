//! 恢复投递
//!
//! docPath: https://open.feishu.cn/document/server-docs/hire-v1/application/recover

use openlark_core::{
    api::{ApiResponseTrait, ResponseFormat},
    config::Config,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 恢复投递请求
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct RecoverRequest {
    /// 配置信息
    config: Config,
    // TODO: 添加请求字段
}

impl RecoverRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            // TODO: 初始化字段
        }
    }

    // TODO: 添加字段 setter 方法

    /// 执行请求
    pub async fn execute(self) -> SDKResult<RecoverResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        _option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<RecoverResponse> {
        // TODO: 实现 API 调用逻辑
        todo!("实现 恢复投递 API 调用")
    }
}

/// 恢复投递响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RecoverResponse {
    /// 响应数据
    ///
    /// TODO: 根据官方文档添加具体字段
    pub data: Value,
}

impl ApiResponseTrait for RecoverResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
