//! 获取邮件组列表

use crate::common::{api_endpoints::MailApiV1, api_utils::*};
use crate::v1::mail_group::models::MailGroupListResponse;
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    SDKResult,
};
use std::sync::Arc;

/// 邮件组列表请求
#[derive(Debug, Clone)]
pub struct MailGroupListRequest {
    config: Arc<Config>,
    page_size: Option<i32>,
    page_token: Option<String>,
}

impl MailGroupListRequest {
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            page_size: None,
            page_token: None,
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

    pub async fn execute(self) -> SDKResult<MailGroupListResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<MailGroupListResponse> {
        let api_endpoint = MailApiV1::MailGroupList;
        let mut request = ApiRequest::<MailGroupListResponse>::get(&api_endpoint.to_url());

        if let Some(page_size) = self.page_size {
            request = request.query("page_size", &page_size.to_string());
        }

        if let Some(ref page_token) = self.page_token {
            request = request.query("page_token", page_token);
        }

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "获取邮件组列表")
    }
}

impl ApiResponseTrait for MailGroupListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
