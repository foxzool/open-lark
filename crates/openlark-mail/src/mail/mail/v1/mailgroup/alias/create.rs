//! 创建邮件组别名

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
pub struct CreateMailGroupAliasRequest {
    config: Arc<Config>,
    mailgroup_id: String,
    body: CreateMailGroupAliasBody,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateMailGroupAliasBody {
    pub alias: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMailGroupAliasResponse {
    pub data: Option<MailGroupAliasData>,
}

impl ApiResponseTrait for CreateMailGroupAliasResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MailGroupAliasData {
    pub alias_id: String,
}

impl CreateMailGroupAliasRequest {
    pub fn new(config: Arc<Config>, mailgroup_id: impl Into<String>) -> Self {
        Self {
            config,
            mailgroup_id: mailgroup_id.into(),
            body: CreateMailGroupAliasBody::default(),
        }
    }

    pub fn alias(mut self, alias: impl Into<String>) -> Self {
        self.body.alias = alias.into();
        self
    }

    pub async fn execute(self) -> SDKResult<CreateMailGroupAliasResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<CreateMailGroupAliasResponse> {
        let path = format!(
            "/open-apis/mail/v1/mailgroups/{}/aliases",
            self.mailgroup_id
        );
        let req: ApiRequest<CreateMailGroupAliasResponse> =
            ApiRequest::post(&path).body(serialize_params(&self.body, "创建邮件组别名")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("创建邮件组别名", "响应数据为空"))
    }
}

#[cfg(test)]
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
        let request = CreateMailGroupAliasRequest::new(arc_config.clone(), "test".to_string())
            .alias("test".to_string());
        let _ = request;
    }
}
