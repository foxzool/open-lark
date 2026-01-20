//! 获取工单详情

use crate::v1::ticket::models::GetTicketResponse;
use crate::common::{api_endpoints::HelpdeskApiV1, api_utils::*};
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required,
    SDKResult,
};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct GetTicketRequest {
    config: Arc<Config>,
    ticket_id: String,
}

impl GetTicketRequest {
    pub fn new(config: Arc<Config>, ticket_id: String) -> Self {
        Self { config, ticket_id }
    }

    pub async fn execute(self) -> SDKResult<GetTicketResponse> {
        validate_required!(self.ticket_id.trim(), "工单ID不能为空");

        let api_endpoint = HelpdeskApiV1::TicketGet(self.ticket_id.clone());
        let request = ApiRequest::<GetTicketResponse>::get(&api_endpoint.to_url());

        let response = openlark_core::http::Transport::request(request, &self.config, None).await?;
        extract_response_data(response, "获取工单")
    }
}

impl ApiResponseTrait for GetTicketResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
