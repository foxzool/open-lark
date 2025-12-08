//! 获取任务结果
//!
//! 该方法用于获取wiki异步任务的结果。
//! 文档参考：https://open.feishu.cn/document/server-docs/docs/wiki-v2/task/get

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use super::super::models::WikiTask;
use crate::common::api_endpoints::WikiApiV2;

/// 获取任务结果请求
pub struct GetWikiTaskRequest {
    task_id: String,
    config: Config,
}

/// 获取任务结果响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetWikiTaskResponse {
    /// 任务信息
    pub data: Option<WikiTask>,
}

impl ApiResponseTrait for GetWikiTaskResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl GetWikiTaskRequest {
    /// 创建获取任务结果请求
    pub fn new(config: Config) -> Self {
        Self {
            task_id: String::new(),
            config,
        }
    }

    /// 设置任务ID
    pub fn task_id(mut self, task_id: impl Into<String>) -> Self {
        self.task_id = task_id.into();
        self
    }

    /// 执行请求
    ///
    /// API文档: https://open.feishu.cn/document/server-docs/docs/wiki-v2/task/get
    pub async fn execute(self) -> SDKResult<GetWikiTaskResponse> {
        // 验证必填字段
        validate_required!(self.task_id, "任务ID不能为空");

        // 使用新的enum+builder系统生成API端点
        let api_endpoint = WikiApiV2::TaskGet(self.task_id.clone());

        // 创建API请求 - 使用类型安全的URL生成
        let api_request: ApiRequest<GetWikiTaskResponse> =
            ApiRequest::get(&api_endpoint.to_url());

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}