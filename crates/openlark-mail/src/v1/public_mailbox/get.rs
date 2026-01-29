//! 获取公共邮箱详情

use crate::common::{api_endpoints::MailApiV1, api_utils::*};
use crate::v1::public_mailbox::models::GetPublicMailboxResponse;
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use std::sync::Arc;

/// 获取公共邮箱请求
#[derive(Debug, Clone)]
pub struct GetPublicMailboxRequest {
    config: Arc<Config>,
    mailbox_id: String,
}

impl GetPublicMailboxRequest {
    pub fn new(config: Arc<Config>, mailbox_id: String) -> Self {
        Self { config, mailbox_id }
    }

    pub async fn execute(self) -> SDKResult<GetPublicMailboxResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetPublicMailboxResponse> {
        validate_required!(self.mailbox_id.trim(), "公共邮箱ID不能为空");

        let api_endpoint = MailApiV1::PublicMailboxGet(self.mailbox_id.clone());
        let request = ApiRequest::<GetPublicMailboxResponse>::get(api_endpoint.to_url());

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "获取公共邮箱")
    }
}

impl ApiResponseTrait for GetPublicMailboxResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_public_mailbox_request() {
        let config = Arc::new(
            openlark_core::config::Config::builder()
                .app_id("test")
                .app_secret("test")
                .build(),
        );

        let request = GetPublicMailboxRequest::new(config, "mailbox_123".to_string());
        assert_eq!(request.mailbox_id, "mailbox_123");
    }
}
