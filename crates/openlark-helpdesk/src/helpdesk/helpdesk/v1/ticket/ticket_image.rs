//! 获取工单内图像

use crate::common::{api_endpoints::HelpdeskApiV1, api_utils::*};
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct GetTicketImageRequest {
    config: Arc<Config>,
    ticket_id: String,
    image_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTicketImageResponse {
    pub data: Option<GetTicketImageData>,
}

impl ApiResponseTrait for GetTicketImageResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTicketImageData {
    pub image_url: String,
}

impl GetTicketImageRequest {
    pub fn new(
        config: Arc<Config>,
        ticket_id: impl Into<String>,
        image_key: impl Into<String>,
    ) -> Self {
        Self {
            config,
            ticket_id: ticket_id.into(),
            image_key: image_key.into(),
        }
    }

    pub async fn execute(self) -> SDKResult<GetTicketImageResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetTicketImageResponse> {
        let path = format!(
            "{}/{}/images/{}",
            "/open-apis/helpdesk/v1/tickets",
            self.ticket_id,
            self.image_key
        );
        let req: ApiRequest<GetTicketImageResponse> = ApiRequest::get(&path);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("获取工单内图像", "响应数据为空")
        })
    }
}
