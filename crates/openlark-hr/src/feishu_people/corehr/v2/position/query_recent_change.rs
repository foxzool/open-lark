//! 查询指定时范围内当前版本信息发生变更的岗位
//!
//! docPath: https://open.feishu.cn/document/server-docs/corehr-v2/position/query_recent_change

use openlark_core::{
    api::{ApiResponseTrait, ResponseFormat},
    config::Config,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 查询指定时范围内当前版本信息发生变更的岗位请求
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct QueryRecentChangeRequest {
    /// 配置信息
    config: Config,
    // TODO: 添加请求字段
}

impl QueryRecentChangeRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            // TODO: 初始化字段
        }
    }

    // TODO: 添加字段 setter 方法

    /// 执行请求
    pub async fn execute(self) -> SDKResult<QueryRecentChangeResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        _option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<QueryRecentChangeResponse> {
        // TODO: 实现 API 调用逻辑
        todo!("实现 查询指定时范围内当前版本信息发生变更的岗位 API 调用")
    }
}

/// 查询指定时范围内当前版本信息发生变更的岗位响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryRecentChangeResponse {
    /// 响应数据
    ///
    /// TODO: 根据官方文档添加具体字段
    pub data: Value,
}

impl ApiResponseTrait for QueryRecentChangeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
