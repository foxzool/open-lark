//! 删除岗位
//!
//! docPath: https://open.feishu.cn/document/server-docs/corehr-v2/position/del_position

use openlark_core::{
    api::{ApiResponseTrait, ResponseFormat},
    config::Config, SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 删除岗位请求
#[derive(Debug, Clone)]
pub struct DelPositionRequest {
    /// 配置信息
    config: Config,
    // TODO: 添加请求字段
}

impl DelPositionRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            // TODO: 初始化字段
        }
    }

    // TODO: 添加字段 setter 方法

    /// 执行请求
    pub async fn execute(self) -> SDKResult<DelPositionResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        _option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<DelPositionResponse> {
        // TODO: 实现 API 调用逻辑
        todo!("实现 删除岗位 API 调用")
    }
}

/// 删除岗位响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DelPositionResponse {
    /// 响应数据
    ///
    /// TODO: 根据官方文档添加具体字段
    pub data: Value,
}

impl ApiResponseTrait for DelPositionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
