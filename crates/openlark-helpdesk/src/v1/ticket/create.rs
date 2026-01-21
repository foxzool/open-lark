//! 创建工单

use crate::common::{api_endpoints::HelpdeskApiV1, api_utils::*};
use crate::v1::ticket::models::{CreateTicketBody, CreateTicketResponse};
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct CreateTicketRequest {
    config: Arc<Config>,
    body: CreateTicketBody,
}

impl CreateTicketRequest {
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            body: CreateTicketBody::default(),
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.body.title = title.into();
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.body.description = Some(description.into());
        self
    }

    pub fn priority(mut self, priority: impl Into<String>) -> Self {
        self.body.priority = Some(priority.into());
        self
    }

    pub async fn execute(self) -> SDKResult<CreateTicketResponse> {
        validate_required!(self.body.title.trim(), "工单标题不能为空");

        let api_endpoint = HelpdeskApiV1::TicketCreate;
        let mut request = ApiRequest::<CreateTicketResponse>::post(api_endpoint.to_url());

        request = request.body(serialize_params(&self.body, "创建工单")?);

        let response = openlark_core::http::Transport::request(request, &self.config, None).await?;
        extract_response_data(response, "创建工单")
    }
}

impl ApiResponseTrait for CreateTicketResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
