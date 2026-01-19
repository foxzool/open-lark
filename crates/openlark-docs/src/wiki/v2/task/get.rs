/// 获取任务结果
///
/// 获取异步任务的执行结果。
/// docPath: https://open.feishu.cn/document/server-docs/docs/wiki-v2/task/get
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use serde::{Deserialize, Serialize};
use crate::common::api_endpoints::WikiApiV2;

/// 任务状态
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    /// 进行中
    Processing,
    /// 成功
    Success,
    /// 失败
    Failed,
}

/// 获取任务结果响应
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetTaskResponse {
    /// 任务ID
    pub task_id: String,
    /// 任务状态
    pub task_status: TaskStatus,
    /// 任务结果
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    /// 错误信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<TaskError>,
}

/// 任务错误
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TaskError {
    /// 错误码
    pub code: String,
    /// 错误消息
    pub message: String,
}

impl ApiResponseTrait for GetTaskResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取任务结果
///
/// 获取异步任务的执行结果。
///
/// # 参数
/// * `config` - SDK 配置
/// * `task_id` - 任务ID
///
/// # 返回
/// 返回任务执行结果
pub async fn get_task(
    config: &Config,
    task_id: &str,
) -> SDKResult<GetTaskResponse> {
    // 使用 ApiEndpoint 枚举生成 URL
    let api_endpoint = WikiApiV2::TaskGet(task_id.to_string());

    // 创建 API 请求
    let api_request: ApiRequest<GetTaskResponse> = ApiRequest::get(&api_endpoint.to_url());

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    Ok(response.data.ok_or_else(|| {
        openlark_core::error::CoreError::validation_msg("响应数据为空")
    })?)
}
