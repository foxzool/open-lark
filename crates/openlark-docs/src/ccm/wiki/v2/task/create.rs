/// 创建异步任务
///
/// 此接口用于创建异步任务，如文档导出、导入等。
/// docPath: https://open.feishu.cn/document/server-docs/docs/wiki-v2/task/create

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat, Response},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 创建异步任务请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTaskRequest {
    /// 任务类型：export(导出)、import(导入)
    pub task_type: String,
    /// 任务参数
    pub task_params: serde_json::Value,
    /// 回调URL
    pub callback_url: Option<String>,
}

impl CreateTaskRequest {
    /// 创建新的任务请求
    pub fn new(task_type: impl Into<String>, task_params: serde_json::Value) -> Self {
        Self {
            task_type: task_type.into(),
            task_params,
            callback_url: None,
        }
    }

    /// 设置回调URL
    pub fn callback_url(mut self, url: impl Into<String>) -> Self {
        self.callback_url = Some(url.into());
        self
    }
}

/// 创建的任务信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatedTask {
    /// 任务ID
    pub task_id: String,
    /// 任务类型
    pub task_type: String,
    /// 任务状态
    pub status: String,
    /// 创建时间
    pub create_time: Option<i64>,
}

/// 创建异步任务响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTaskResponse {
    /// 创建的任务信息
    pub data: Option<CreatedTask>,
}

impl ApiResponseTrait for CreateTaskResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建异步任务
///
/// 此接口用于创建异步任务，如文档导出、导入等。
/// docPath: https://open.feishu.cn/document/server-docs/docs/wiki-v2/task/create
pub async fn create_task(
    request: CreateTaskRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<CreateTaskResponse>> {
    // 构建请求体
    let mut body = serde_json::json!({
        "task_type": request.task_type,
        "task_params": request.task_params
    });

    if let Some(callback_url) = request.callback_url {
        body["callback_url"] = serde_json::json!(callback_url);
    }

    // 创建API请求
    let mut api_request: ApiRequest<CreateTaskResponse> =
        ApiRequest::post("/open-apis/wiki/v2/tasks")
            .body(body);

    // 如果有请求选项，应用它们
    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    // 发送请求
    Transport::request(api_request, config, None).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_task_request() {
        let task_params = serde_json::json!({
            "file_id": "file_123",
            "format": "pdf"
        });

        let request = CreateTaskRequest {
            task_type: "export".to_string(),
            task_params,
            callback_url: Some("https://example.com/callback".to_string()),
        };

        assert_eq!(request.task_type, "export");
        assert_eq!(request.callback_url, Some("https://example.com/callback".to_string()));
    }

    #[test]
    fn test_created_task() {
        let task = CreatedTask {
            task_id: "task_123".to_string(),
            task_type: "export".to_string(),
            status: "processing".to_string(),
            create_time: Some(1609459200),
        };

        assert_eq!(task.task_id, "task_123");
        assert_eq!(task.task_type, "export");
        assert_eq!(task.status, "processing");
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(CreateTaskResponse::data_format(), ResponseFormat::Data);
    }
}
