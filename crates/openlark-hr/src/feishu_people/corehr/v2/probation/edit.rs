//! 编辑试用期
//!
//! docPath: https://open.feishu.cn/document/corehr-v1/probation/edit

use openlark_core::{api::{ApiRequest, ApiResponseTrait, ResponseFormat}, config::Config, http::Transport, SDKResult};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize, Default)] pub struct EditProbationBody { pub employee_id: Option<String> }
#[derive(Debug, Clone, Serialize, Deserialize, Default)] pub struct EditProbationResponse { pub probation: Option<serde_json::Value> }
impl ApiResponseTrait for EditProbationResponse { fn data_format() -> ResponseFormat { ResponseFormat::Data } }
#[derive(Debug, Clone)] pub struct EditProbationRequest { config: Arc<Config>, body: EditProbationBody }
impl EditProbationRequest { pub fn new(config: Arc<Config>) -> Self { Self { config, body: EditProbationBody::default() } } pub fn body(mut self, body: EditProbationBody) -> Self { self.body = body; self } pub async fn execute(self) -> SDKResult<EditProbationResponse> { self.execute_with_options(openlark_core::req_option::RequestOption::default()).await } pub async fn execute_with_options(self, option: openlark_core::req_option::RequestOption) -> SDKResult<EditProbationResponse> { let request = ApiRequest::<EditProbationResponse>::post("/open-apis/corehr/v2/probation/edit").body(serde_json::to_value(&self.body)?); let response = Transport::request(request, &self.config, Some(option)).await?; response.data.ok_or_else(|| openlark_core::error::validation_error("编辑试用期", "响应数据为空")) } }
