//! 创建自定义分组
//!
//! docPath: https://open.feishu.cn/document/task-v2/section/create

use openlark_core::{api::{ApiRequest, ApiResponseTrait, ResponseFormat}, config::Config, http::Transport, SDKResult};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize, Default)] pub struct CreateSectionBody { pub name: Option<String>, pub resource_type: Option<String>, pub resource_id: Option<String> }
#[derive(Debug, Clone, Serialize, Deserialize, Default)] pub struct CreateSectionResponse { pub section: Option<serde_json::Value> }
impl ApiResponseTrait for CreateSectionResponse { fn data_format() -> ResponseFormat { ResponseFormat::Data } }
#[derive(Debug, Clone)] pub struct CreateSectionRequest { config: Arc<Config>, body: CreateSectionBody }
impl CreateSectionRequest { pub fn new(config: Arc<Config>) -> Self { Self { config, body: CreateSectionBody::default() } } pub fn body(mut self, body: CreateSectionBody) -> Self { self.body = body; self } pub async fn execute(self) -> SDKResult<CreateSectionResponse> { self.execute_with_options(openlark_core::req_option::RequestOption::default()).await } pub async fn execute_with_options(self, option: openlark_core::req_option::RequestOption) -> SDKResult<CreateSectionResponse> { let request = ApiRequest::<CreateSectionResponse>::post("/open-apis/task/v2/sections").body(serde_json::to_value(&self.body)?); let response = Transport::request(request, &self.config, Some(option)).await?; response.data.ok_or_else(|| openlark_core::error::validation_error("创建自定义分组", "响应数据为空")) } }
