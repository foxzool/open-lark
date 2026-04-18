//! 搜索地理库信息
//!
//! docPath: https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/district/search

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct SearchDistrictsBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub district_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyword: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SearchDistrictsResponse {
    #[serde(default)]
    pub items: Vec<super::list::DistrictItem>,
}

impl ApiResponseTrait for SearchDistrictsResponse {
    fn data_format() -> ResponseFormat { ResponseFormat::Data }
}

#[derive(Debug, Clone)]
pub struct SearchDistrictsRequest {
    config: Arc<Config>,
    locale: Option<String>,
    body: SearchDistrictsBody,
}

impl SearchDistrictsRequest {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config, locale: None, body: SearchDistrictsBody::default() }
    }
    pub fn locale(mut self, locale: impl Into<String>) -> Self { self.locale = Some(locale.into()); self }
    pub fn district_ids(mut self, district_ids: Vec<String>) -> Self { self.body.district_ids = Some(district_ids); self }
    pub fn keyword(mut self, keyword: impl Into<String>) -> Self { self.body.keyword = Some(keyword.into()); self }
    pub async fn execute(self) -> SDKResult<SearchDistrictsResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default()).await
    }
    pub async fn execute_with_options(self, option: openlark_core::req_option::RequestOption) -> SDKResult<SearchDistrictsResponse> {
        let mut request = ApiRequest::<SearchDistrictsResponse>::post("/open-apis/approval/v4/districts/search");
        if let Some(locale) = self.locale { request = request.query("locale", locale); }
        request = request.body(serde_json::to_value(&self.body)?);
        let response = Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| openlark_core::error::validation_error("搜索地理库信息", "响应数据为空"))
    }
}
