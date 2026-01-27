//! 查询归档报表表头
//!
//! docPath: https://open.feishu.cn/document/server-docs/attendance-v1/archive_rule/user_stats_fields_query

use openlark_core::{
    api::{ApiResponseTrait, ResponseFormat},
    config::Config, SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 查询归档报表表头请求
#[derive(Debug, Clone)]
pub struct UserStatsFieldsQueryRequest {
    /// 配置信息
    config: Config,
    // TODO: 添加请求字段
}

impl UserStatsFieldsQueryRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            // TODO: 初始化字段
        }
    }

    // TODO: 添加字段 setter 方法

    /// 执行请求
    pub async fn execute(self) -> SDKResult<UserStatsFieldsQueryResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        _option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<UserStatsFieldsQueryResponse> {
        // TODO: 实现 API 调用逻辑
        todo!("实现 查询归档报表表头 API 调用")
    }
}

/// 查询归档报表表头响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserStatsFieldsQueryResponse {
    /// 响应数据
    ///
    /// TODO: 根据官方文档添加具体字段
    pub data: Value,
}

impl ApiResponseTrait for UserStatsFieldsQueryResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
