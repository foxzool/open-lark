//! 回复用户在工单里的提问

use crate::common::{api_endpoints::HelpdeskApiV1, api_utils::*};
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct AnswerUserQueryRequest {
    config: Arc<Config>,
    ticket_id: String,
    body: AnswerUserQueryBody,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AnswerUserQueryBody {
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
}

impl AnswerUserQueryBody {
    fn validate(&self) -> SDKResult<()> {
        if self.content.trim().is_empty() {
            return Err(openlark_core::error::validation_error(
                "回复内容不能为空",
                "",
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnswerUserQueryResponse {
    pub data: Option<AnswerUserQueryData>,
}

impl ApiResponseTrait for AnswerUserQueryResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnswerUserQueryData {
    pub message_id: String,
}

impl AnswerUserQueryRequest {
    pub fn new(config: Arc<Config>, ticket_id: impl Into<String>) -> Self {
        Self {
            config,
            ticket_id: ticket_id.into(),
            body: AnswerUserQueryBody::default(),
        }
    }

    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.body.content = content.into();
        self
    }

    pub fn content_type(mut self, content_type: impl Into<String>) -> Self {
        self.body.content_type = Some(content_type.into());
        self
    }

    pub async fn execute(self) -> SDKResult<AnswerUserQueryResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<AnswerUserQueryResponse> {
        self.body.validate()?;

        let path = HelpdeskApiV1::TicketAnswerUserQuery(self.ticket_id.clone()).to_url();
        let req: ApiRequest<AnswerUserQueryResponse> =
            ApiRequest::post(&path).body(serialize_params(&self.body, "回复用户在工单里的提问")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("回复用户在工单里的提问", "响应数据为空")
        })
    }
}
