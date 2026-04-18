//! 列取自定义分组中的任务
//!
//! docPath: https://open.feishu.cn/document/task-v2/section/tasks

use crate::common::api_utils::*;
use crate::v2::section::tasks::ListSectionTasksResponse;
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use std::sync::Arc;

/// 列取自定义分组中的任务请求。
#[derive(Debug, Clone)]
pub struct GetSectionTasksRequest {
    config: Arc<Config>,
    section_guid: String,
    page_size: Option<i32>,
    page_token: Option<String>,
    completed: Option<bool>,
    created_from: Option<String>,
    created_to: Option<String>,
}

impl GetSectionTasksRequest {
    pub fn new(config: Arc<Config>, section_guid: impl Into<String>) -> Self {
        Self {
            config,
            section_guid: section_guid.into(),
            page_size: None,
            page_token: None,
            completed: None,
            created_from: None,
            created_to: None,
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

    pub async fn execute(self) -> SDKResult<ListSectionTasksResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListSectionTasksResponse> {
        validate_required!(self.section_guid.trim(), "分组 GUID 不能为空");

        let path = format!("/open-apis/task/v2/sections/{}/tasks", self.section_guid);
        let mut request = ApiRequest::<ListSectionTasksResponse>::get(&path);
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

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "列取自定义分组中的任务")
    }
}

impl ApiResponseTrait for ListSectionTasksResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
