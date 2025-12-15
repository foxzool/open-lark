/// 获取任务列表
///
/// 此接口用于获取异步任务列表，如文档导出、导入等任务状态。
/// docPath: https://open.feishu.cn/document/server-docs/docs/wiki-v2/task/list

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat, Response},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 获取任务列表请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListTasksRequest {
    /// 任务类型：export(导出)、import(导入)
    pub task_type: Option<String>,
    /// 任务状态：processing(处理中)、success(成功)、failed(失败)
    pub status: Option<String>,
    /// 页面大小，默认20，最大100
    pub page_size: Option<i32>,
    /// 页码，从1开始
    pub page_token: Option<String>,
}

/// 任务信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskInfo {
    /// 任务ID
    pub task_id: String,
    /// 任务类型
    pub task_type: String,
    /// 任务状态
    pub status: String,
    /// 进度百分比
    pub progress: Option<i32>,
    /// 创建时间
    pub create_time: Option<i64>,
    /// 更新时间
    pub update_time: Option<i64>,
    /// 完成时间
    pub complete_time: Option<i64>,
    /// 错误信息
    pub error_message: Option<String>,
}

/// 分页信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskPageToken {
    /// 是否还有下一页
    pub has_more: Option<bool>,
    /// 页码
    pub page_token: Option<String>,
}

/// 获取任务列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListTasksResponse {
    /// 任务列表数据
    pub data: Option<ListTasksData>,
}

/// 任务列表数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListTasksData {
    /// 任务列表
    pub items: Option<Vec<TaskInfo>>,
    /// 分页信息
    pub page_token: Option<TaskPageToken>,
}

impl ApiResponseTrait for ListTasksResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取任务列表
///
/// 此接口用于获取异步任务列表，如文档导出、导入等任务状态。
/// docPath: https://open.feishu.cn/document/server-docs/docs/wiki-v2/task/list
pub async fn list_tasks(
    request: ListTasksRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<ListTasksResponse>> {
    // 构建请求体
    let mut body = serde_json::json!({});

    if let Some(task_type) = request.task_type {
        body["task_type"] = serde_json::json!(task_type);
    }
    if let Some(status) = request.status {
        body["status"] = serde_json::json!(status);
    }
    if let Some(page_size) = request.page_size {
        body["page_size"] = serde_json::json!(page_size);
    }
    if let Some(page_token) = request.page_token {
        body["page_token"] = serde_json::json!(page_token);
    }

    // 创建API请求
    let mut api_request: ApiRequest<ListTasksResponse> =
        ApiRequest::get("/open-apis/wiki/v2/tasks")
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
    fn test_list_tasks_request() {
        let request = ListTasksRequest {
            task_type: Some("export".to_string()),
            status: Some("processing".to_string()),
            page_size: Some(20),
            page_token: Some("token_456".to_string()),
        };

        assert_eq!(request.task_type, Some("export"));
        assert_eq!(request.status, Some("processing"));
        assert_eq!(request.page_size, Some(20));
    }

    #[test]
    fn test_task_info() {
        let task = TaskInfo {
            task_id: "task_123".to_string(),
            task_type: "export".to_string(),
            status: "processing".to_string(),
            progress: Some(50),
            create_time: Some(1609459200),
            update_time: Some(1609459260),
            complete_time: None,
            error_message: None,
        };

        assert_eq!(task.task_id, "task_123");
        assert_eq!(task.task_type, "export");
        assert_eq!(task.status, "processing");
        assert_eq!(task.progress, Some(50));
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(ListTasksResponse::data_format(), ResponseFormat::Data);
    }
}
