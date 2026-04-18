//! 更新任务
//!
//! docPath: https://open.feishu.cn/document/task-v2/task/patch

use crate::v2::task::models::{UpdateTaskBody, UpdateTaskResponse};
use openlark_core::{api::{ApiRequest, ApiResponseTrait, ResponseFormat}, config::Config, http::Transport, validate_required, SDKResult};
use std::sync::Arc;
#[derive(Debug, Clone)] pub struct UpdateTaskRequest { config: Arc<Config>, task_guid: String, body: UpdateTaskBody }
impl UpdateTaskRequest { pub fn new(config: Arc<Config>, task_guid: impl Into<String>) -> Self { Self { config, task_guid: task_guid.into(), body: UpdateTaskBody::default() } } pub fn body(mut self, body: UpdateTaskBody) -> Self { self.body = body; self } pub async fn execute(self) -> SDKResult<UpdateTaskResponse> { self.execute_with_options(openlark_core::req_option::RequestOption::default()).await } pub async fn execute_with_options(self, option: openlark_core::req_option::RequestOption) -> SDKResult<UpdateTaskResponse> { validate_required!(self.task_guid.trim(), "任务 GUID 不能为空"); let path = format!("/open-apis/task/v2/tasks/{}", self.task_guid); let request = ApiRequest::<UpdateTaskResponse>::patch(&path).body(serde_json::to_value(&self.body)?); let response = Transport::request(request, &self.config, Some(option)).await?; response.data.ok_or_else(|| openlark_core::error::validation_error("更新任务", "响应数据为空")) } }
impl ApiResponseTrait for UpdateTaskResponse { fn data_format() -> ResponseFormat { ResponseFormat::Data } }
