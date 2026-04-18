//! 更新自定义分组
//!
//! docPath: https://open.feishu.cn/document/task-v2/section/patch

use openlark_core::{api::{ApiRequest, ApiResponseTrait, ResponseFormat}, config::Config, http::Transport, validate_required, SDKResult};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[derive(Debug, Clone, Serialize, Deserialize, Default)] pub struct UpdateSectionBody { pub name: Option<String> }
#[derive(Debug, Clone, Serialize, Deserialize, Default)] pub struct UpdateSectionResponse { pub section: Option<serde_json::Value> }
impl ApiResponseTrait for UpdateSectionResponse { fn data_format() -> ResponseFormat { ResponseFormat::Data } }
#[derive(Debug, Clone)] pub struct UpdateSectionRequest { config: Arc<Config>, section_guid: String, body: UpdateSectionBody }
impl UpdateSectionRequest { pub fn new(config: Arc<Config>, section_guid: impl Into<String>) -> Self { Self { config, section_guid: section_guid.into(), body: UpdateSectionBody::default() } } pub fn body(mut self, body: UpdateSectionBody) -> Self { self.body = body; self } pub async fn execute(self) -> SDKResult<UpdateSectionResponse> { self.execute_with_options(openlark_core::req_option::RequestOption::default()).await } pub async fn execute_with_options(self, option: openlark_core::req_option::RequestOption) -> SDKResult<UpdateSectionResponse> { validate_required!(self.section_guid.trim(), "分组 GUID 不能为空"); let path = format!("/open-apis/task/v2/sections/{}", self.section_guid); let request = ApiRequest::<UpdateSectionResponse>::patch(&path).body(serde_json::to_value(&self.body)?); let response = Transport::request(request, &self.config, Some(option)).await?; response.data.ok_or_else(|| openlark_core::error::validation_error("更新自定义分组", "响应数据为空")) } }
