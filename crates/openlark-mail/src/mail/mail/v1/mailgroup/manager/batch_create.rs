//! 批量创建邮件组管理员

use crate::common::api_utils::serialize_params;
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
pub struct BatchCreateMailGroupManagerRequest {
    config: Arc<Config>,
    mailgroup_id: String,
    body: BatchCreateMailGroupManagerBody,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BatchCreateMailGroupManagerBody {
    pub manager_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchCreateMailGroupManagerResponse {
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for BatchCreateMailGroupManagerResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl BatchCreateMailGroupManagerRequest {
    pub fn new(config: Arc<Config>, mailgroup_id: impl Into<String>) -> Self {
        Self {
            config,
            mailgroup_id: mailgroup_id.into(),
            body: BatchCreateMailGroupManagerBody::default(),
        }
    }

    pub fn manager_ids(mut self, ids: Vec<String>) -> Self {
        self.body.manager_ids = ids;
        self
    }

    pub async fn execute(self) -> SDKResult<BatchCreateMailGroupManagerResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<BatchCreateMailGroupManagerResponse> {
        let path = format!(
            "/open-apis/mail/v1/mailgroups/{}/managers/batch_create",
            self.mailgroup_id
        );
        let req: ApiRequest<BatchCreateMailGroupManagerResponse> =
            ApiRequest::post(&path).body(serialize_params(&self.body, "批量创建邮件组管理员")?);

        let _resp: openlark_core::api::Response<BatchCreateMailGroupManagerResponse> =
            Transport::request(req, &self.config, Some(option)).await?;
        Ok(BatchCreateMailGroupManagerResponse { data: None })
    }
}
