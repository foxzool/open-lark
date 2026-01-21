//! 更新工单

use crate::common::{api_endpoints::HelpdeskApiV1, api_utils::*};
use crate::v1::ticket::models::{UpdateTicketBody, UpdateTicketResponse};
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct UpdateTicketRequest {
    config: Arc<Config>,
    ticket_id: String,
    body: UpdateTicketBody,
}

impl UpdateTicketRequest {
    pub fn new(config: Arc<Config>, ticket_id: String) -> Self {
        Self {
            config,
            ticket_id,
            body: UpdateTicketBody::default(),
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.body.title = Some(title.into());
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.body.description = Some(description.into());
        self
    }

    pub fn status(mut self, status: impl Into<String>) -> Self {
        self.body.status = Some(status.into());
        self
    }

    pub async fn execute(self) -> SDKResult<UpdateTicketResponse> {
        validate_required!(self.ticket_id.trim(), "工单ID不能为空");

        let api_endpoint = HelpdeskApiV1::TicketUpdate(self.ticket_id.clone());
        let mut request = ApiRequest::<UpdateTicketResponse>::patch(&api_endpoint.to_url());

        request = request.body(serialize_params(&self.body, "更新工单")?);

        let response = openlark_core::http::Transport::request(request, &self.config, None).await?;
        extract_response_data(response, "更新工单")
    }
}

impl ApiResponseTrait for UpdateTicketResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
