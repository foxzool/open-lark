//! 更新清单
//!
//! docPath: https://open.feishu.cn/document/task-v2/tasklist/patch

use crate::v2::tasklist::models::{UpdateTasklistBody, UpdateTasklistResponse};
use openlark_core::{api::{ApiRequest, ApiResponseTrait, ResponseFormat}, config::Config, http::Transport, validate_required, SDKResult};
use std::sync::Arc;
#[derive(Debug, Clone)] pub struct UpdateTasklistRequest { config: Arc<Config>, tasklist_guid: String, body: UpdateTasklistBody }
impl UpdateTasklistRequest { pub fn new(config: Arc<Config>, tasklist_guid: impl Into<String>) -> Self { Self { config, tasklist_guid: tasklist_guid.into(), body: UpdateTasklistBody::default() } } pub fn body(mut self, body: UpdateTasklistBody) -> Self { self.body = body; self } pub async fn execute(self) -> SDKResult<UpdateTasklistResponse> { self.execute_with_options(openlark_core::req_option::RequestOption::default()).await } pub async fn execute_with_options(self, option: openlark_core::req_option::RequestOption) -> SDKResult<UpdateTasklistResponse> { validate_required!(self.tasklist_guid.trim(), "任务清单 GUID 不能为空"); let path = format!("/open-apis/task/v2/tasklists/{}", self.tasklist_guid); let request = ApiRequest::<UpdateTasklistResponse>::patch(&path).body(serde_json::to_value(&self.body)?); let response = Transport::request(request, &self.config, Some(option)).await?; response.data.ok_or_else(|| openlark_core::error::validation_error("更新清单", "响应数据为空")) } }
impl ApiResponseTrait for UpdateTasklistResponse { fn data_format() -> ResponseFormat { ResponseFormat::Data } }
