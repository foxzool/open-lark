//! 创建服务台对话
//!
//! 创建一个新的服务台对话（工单）。

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

/// 创建服务台对话请求
#[derive(Debug, Clone)]
pub struct StartServiceRequest {
    config: Arc<Config>,
    body: StartServiceBody,
}

/// 创建服务台对话请求体
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StartServiceBody {
    /// 用户 ID
    pub user_id: String,
    /// 服务台 ID
    pub service_id: String,
    /// 问题描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub question: Option<String>,
}

impl StartServiceBody {
    fn validate(&self) -> SDKResult<()> {
        if self.user_id.trim().is_empty() {
            return Err(openlark_core::error::validation_error("用户ID不能为空", ""));
        }
        if self.service_id.trim().is_empty() {
            return Err(openlark_core::error::validation_error("服务台ID不能为空", ""));
        }
        Ok(())
    }
}

/// 创建服务台对话响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartServiceResponse {
    pub data: Option<StartServiceData>,
}

impl ApiResponseTrait for StartServiceResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建服务台对话数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartServiceData {
    /// 工单 ID
    pub ticket_id: String,
}

impl StartServiceRequest {
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            body: StartServiceBody::default(),
        }
    }

    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.body.user_id = user_id.into();
        self
    }

    pub fn service_id(mut self, service_id: impl Into<String>) -> Self {
        self.body.service_id = service_id.into();
        self
    }

    pub fn question(mut self, question: impl Into<String>) -> Self {
        self.body.question = Some(question.into());
        self
    }

    pub async fn execute(self) -> SDKResult<StartServiceResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<StartServiceResponse> {
        self.body.validate()?;

        let req: ApiRequest<StartServiceResponse> =
            ApiRequest::post(HelpdeskApiV1::TicketStartService.to_url())
                .body(serialize_params(&self.body, "创建服务台对话")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("创建服务台对话", "响应数据为空")
        })
    }
}
