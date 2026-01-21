//! 获取邮件组详情

use crate::common::{api_endpoints::MailApiV1, api_utils::*};
use crate::v1::mail_group::models::GetMailGroupResponse;
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use std::sync::Arc;

/// 获取邮件组请求
#[derive(Debug, Clone)]
pub struct GetMailGroupRequest {
    config: Arc<Config>,
    mail_group_id: String,
}

impl GetMailGroupRequest {
    pub fn new(config: Arc<Config>, mail_group_id: String) -> Self {
        Self {
            config,
            mail_group_id,
        }
    }

    pub async fn execute(self) -> SDKResult<GetMailGroupResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetMailGroupResponse> {
        validate_required!(self.mail_group_id.trim(), "邮件组ID不能为空");

        let api_endpoint = MailApiV1::MailGroupGet(self.mail_group_id.clone());
        let request = ApiRequest::<GetMailGroupResponse>::get(api_endpoint.to_url());

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "获取邮件组")
    }
}

impl ApiResponseTrait for GetMailGroupResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_mail_group_request() {
        let config = Arc::new(
            openlark_core::config::Config::builder()
                .app_id("test")
                .app_secret("test")
                .build(),
        );

        let request = GetMailGroupRequest::new(config, "group_123".to_string());
        assert_eq!(request.mail_group_id, "group_123");
    }
}
