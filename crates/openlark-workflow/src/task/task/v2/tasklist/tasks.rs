//! 获取清单任务列表
//!
//! docPath: https://open.feishu.cn/document/task-v2/tasklist/tasks

use crate::common::{api_endpoints::TaskApiV2, api_utils::*};
use crate::v2::tasklist::tasks::ListTasklistTasksResponse;
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use std::sync::Arc;

/// 获取清单任务列表请求。
#[derive(Debug, Clone)]
pub struct GetTasklistTasksRequest {
    /// 配置信息。
    config: Arc<Config>,
    /// 清单 GUID。
    tasklist_guid: String,
    /// 分页大小。
    page_size: Option<i32>,
    /// 分页标记。
    page_token: Option<String>,
    /// 完成状态过滤。
    completed: Option<bool>,
    /// 创建时间起点。
    created_from: Option<String>,
    /// 创建时间终点。
    created_to: Option<String>,
    /// 用户 ID 类型。
    user_id_type: Option<String>,
}

impl GetTasklistTasksRequest {
    pub fn new(config: Arc<Config>, tasklist_guid: impl Into<String>) -> Self {
        Self {
            config,
            tasklist_guid: tasklist_guid.into(),
            page_size: None,
            page_token: None,
            completed: None,
            created_from: None,
            created_to: None,
            user_id_type: None,
        }
    }

    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    pub fn completed(mut self, completed: bool) -> Self {
        self.completed = Some(completed);
        self
    }

    pub fn created_from(mut self, created_from: impl Into<String>) -> Self {
        self.created_from = Some(created_from.into());
        self
    }

    pub fn created_to(mut self, created_to: impl Into<String>) -> Self {
        self.created_to = Some(created_to.into());
        self
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    pub async fn execute(self) -> SDKResult<ListTasklistTasksResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListTasklistTasksResponse> {
        validate_required!(self.tasklist_guid.trim(), "任务清单 GUID 不能为空");

        let api_endpoint = TaskApiV2::TasklistGetTasks(self.tasklist_guid.clone());
        let mut request = ApiRequest::<ListTasklistTasksResponse>::get(api_endpoint.to_url());
        if let Some(page_size) = self.page_size {
            request = request.query("page_size", page_size.to_string());
        }
        if let Some(page_token) = self.page_token {
            request = request.query("page_token", page_token);
        }
        if let Some(completed) = self.completed {
            request = request.query("completed", completed.to_string());
        }
        if let Some(created_from) = self.created_from {
            request = request.query("created_from", created_from);
        }
        if let Some(created_to) = self.created_to {
            request = request.query("created_to", created_to);
        }
        if let Some(user_id_type) = self.user_id_type {
            request = request.query("user_id_type", user_id_type);
        }

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "获取清单任务列表")
    }
}

impl ApiResponseTrait for ListTasklistTasksResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
