//! 获取工单列表

use crate::common::{api_endpoints::HelpdeskApiV1, api_utils::*};
use crate::helpdesk::helpdesk::v1::ticket::models::TicketListResponse;
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
        let mut request = ApiRequest::<TicketListResponse>::get(api_endpoint.to_url());

        if let Some(page_size) = self.page_size {
            request = request.query("page_size", page_size.to_string().as_str());
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

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn test_builder_basic() {
        let arc_config = Arc::new(
            openlark_core::config::Config::builder()
                .app_id("test_app")
                .app_secret("test_secret")
                .build(),
        );
        let _config = openlark_core::config::Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build();
        let request = TicketListRequest::new(arc_config.clone())
            .page_size(1)
            .page_token("test".to_string());
        let _ = request;
    }
}
