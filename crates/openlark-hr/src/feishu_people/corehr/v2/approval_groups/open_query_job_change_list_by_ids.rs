//! 批量查询人员调整内容
//!
//! docPath: https://open.feishu.cn/document/server-docs/corehr-v2/approval_groups/open_query_job_change_list_by_ids

use openlark_core::{
    api::{ApiResponseTrait, ResponseFormat},
    config::Config, SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 批量查询人员调整内容请求
#[derive(Debug, Clone)]
pub struct OpenQueryJobChangeListByIdsRequest {
    /// 配置信息
    config: Config,
    // TODO: 添加请求字段
}

impl OpenQueryJobChangeListByIdsRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            // TODO: 初始化字段
        }
    }

    // TODO: 添加字段 setter 方法

    /// 执行请求
    pub async fn execute(self) -> SDKResult<OpenQueryJobChangeListByIdsResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        _option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<OpenQueryJobChangeListByIdsResponse> {
        // TODO: 实现 API 调用逻辑
        todo!("实现 批量查询人员调整内容 API 调用")
    }
}

/// 批量查询人员调整内容响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OpenQueryJobChangeListByIdsResponse {
    /// 响应数据
    ///
    /// TODO: 根据官方文档添加具体字段
    pub data: Value,
}

impl ApiResponseTrait for OpenQueryJobChangeListByIdsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
