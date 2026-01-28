//! 获取假期类型列表
//!
//! docPath: https://open.feishu.cn/document/server-docs/corehr-v1/leave/leave_types

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 获取假期类型列表请求
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct LeaveTypesRequest {
    /// 配置信息
    config: Config,
    // TODO: 添加请求字段
}

impl LeaveTypesRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            // TODO: 初始化字段
        }
    }

    // TODO: 添加字段 setter 方法

    /// 执行请求
    pub async fn execute(self) -> SDKResult<LeaveTypesResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<LeaveTypesResponse> {
        // TODO: 实现 API 调用逻辑
        todo!("实现 获取假期类型列表 API 调用")
    }
}

/// 获取假期类型列表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LeaveTypesResponse {
    /// 响应数据
    ///
    /// TODO: 根据官方文档添加具体字段
    pub data: Value,
}

impl ApiResponseTrait for LeaveTypesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
