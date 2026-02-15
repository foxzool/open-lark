//! 转移应用所有者

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
pub struct TransferAppOwnerRequest {
    config: Arc<Config>,
    app_id: String,
    body: TransferAppOwnerBody,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferAppOwnerBody {
    pub new_owner_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferAppOwnerResponse {
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for TransferAppOwnerResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl TransferAppOwnerRequest {
    pub fn new(config: Arc<Config>, app_id: impl Into<String>) -> Self {
        Self {
            config,
            app_id: app_id.into(),
            body: TransferAppOwnerBody::default(),
        }
    }

    pub fn new_owner_id(mut self, id: impl Into<String>) -> Self {
        self.body.new_owner_id = id.into();
        self
    }

    pub async fn execute(self) -> SDKResult<TransferAppOwnerResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<TransferAppOwnerResponse> {
        let path = format!("/open-apis/application/v6/applications/{}/owner/transfer", self.app_id);
        let req: ApiRequest<TransferAppOwnerResponse> =
            ApiRequest::post(&path).json(&self.body).map_err(|e| {
                openlark_core::error::CoreError::Serialization(e.to_string())
            })?;

        let _resp: openlark_core::api::Response<TransferAppOwnerResponse> =
            Transport::request(req, &self.config, Some(option)).await?;
        Ok(TransferAppOwnerResponse { data: None })
    }
}
