//! 获取工单列表

use crate::v1::ticket::models::TicketListResponse;
use crate::common::{api_endpoints::HelpdeskApiV1, api_utils::*};
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    SDKResult,
};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct TicketListRequest {
    config: Arc<Config>,
    page_size: Option<i32>,
    page_token: Option<String>,
}

impl TicketListRequest {
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

    pub async fn execute(self) -> SDKResult<TicketListResponse> {
        let api_endpoint = HelpdeskApiV1::TicketList;
        let mut request = ApiRequest::<TicketListResponse>::get(&api_endpoint.to_url());

        if let Some(page_size) = self.page_size {
            request = request.query("page_size", &page_size.to_string());
        }

        if let Some(ref page_token) = self.page_token {
            request = request.query("page_token", page_token);
        }

        let response = openlark_core::http::Transport::request(request, &self.config, None).await?;
        extract_response_data(response, "获取工单列表")
    }
}

impl ApiResponseTrait for TicketListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
