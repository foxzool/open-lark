//! 查询指定时间范围公司版本
//!
//! docPath: https://open.feishu.cn/document/corehr-v1/organization-management/company/query_multi_timeline

use openlark_core::{api::{ApiRequest, ApiResponseTrait, ResponseFormat}, config::Config, http::Transport, SDKResult};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QueryMultiTimelineBody { pub from_date: Option<String>, pub to_date: Option<String> }
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QueryMultiTimelineResponse { pub items: Vec<serde_json::Value> }
impl ApiResponseTrait for QueryMultiTimelineResponse { fn data_format() -> ResponseFormat { ResponseFormat::Data } }
#[derive(Debug, Clone)] pub struct QueryMultiTimelineRequest { config: Arc<Config>, body: QueryMultiTimelineBody }
impl QueryMultiTimelineRequest { pub fn new(config: Arc<Config>) -> Self { Self { config, body: QueryMultiTimelineBody::default() } } pub fn body(mut self, body: QueryMultiTimelineBody) -> Self { self.body = body; self } pub async fn execute(self) -> SDKResult<QueryMultiTimelineResponse> { self.execute_with_options(openlark_core::req_option::RequestOption::default()).await } pub async fn execute_with_options(self, option: openlark_core::req_option::RequestOption) -> SDKResult<QueryMultiTimelineResponse> { let request = ApiRequest::<QueryMultiTimelineResponse>::post("/open-apis/corehr/v2/companies/query_multi_timeline").body(serde_json::to_value(&self.body)?); let response = Transport::request(request, &self.config, Some(option)).await?; response.data.ok_or_else(|| openlark_core::error::validation_error("查询指定时间范围公司版本", "响应数据为空")) } }
