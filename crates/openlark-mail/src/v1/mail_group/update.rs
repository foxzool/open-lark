//! 更新邮件组

use crate::v1::mail_group::models::{UpdateMailGroupBody, UpdateMailGroupResponse};
use crate::common::{api_endpoints::MailApiV1, api_utils::*};
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required,
    SDKResult,
};
use std::sync::Arc;

/// 更新邮件组请求
#[derive(Debug, Clone)]
pub struct UpdateMailGroupRequest {
    config: Arc<Config>,
    mail_group_id: String,
    body: UpdateMailGroupBody,
}

impl UpdateMailGroupRequest {
    pub fn new(config: Arc<Config>, mail_group_id: String) -> Self {
        Self {
            config,
            mail_group_id,
            body: UpdateMailGroupBody::default(),
        }
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.body.description = Some(description.into());
        self
    }

    pub fn only_admins_send(mut self, only_admins_send: bool) -> Self {
        self.body.only_admins_send = Some(only_admins_send);
        self
    }

    pub async fn execute(self) -> SDKResult<UpdateMailGroupResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<UpdateMailGroupResponse> {
        validate_required!(self.mail_group_id.trim(), "邮件组ID不能为空");

        let api_endpoint = MailApiV1::MailGroupUpdate(self.mail_group_id.clone());
        let mut request = ApiRequest::<UpdateMailGroupResponse>::patch(&api_endpoint.to_url());

        let request_body = &self.body;
        request = request.body(serialize_params(request_body, "更新邮件组")?);

        let response = openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "更新邮件组")
    }
}

impl ApiResponseTrait for UpdateMailGroupResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
