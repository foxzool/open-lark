//! 创建邮箱文件夹

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
pub struct CreateMailboxFolderRequest {
    config: Arc<Config>,
    user_mailbox_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMailboxFolderResponse {
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for CreateMailboxFolderResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl CreateMailboxFolderRequest {
    pub fn new(config: Arc<Config>, user_mailbox_id: impl Into<String>) -> Self {
        Self {
            config,
            user_mailbox_id: user_mailbox_id.into(),
        }
    }

    pub async fn execute(self) -> SDKResult<CreateMailboxFolderResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<CreateMailboxFolderResponse> {
        let path = format!(
            "/open-apis/mail/v1/user_mailboxes/{}/folders",
            self.user_mailbox_id
        );
        let req: ApiRequest<CreateMailboxFolderResponse> = ApiRequest::post(&path);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("创建邮箱文件夹", "响应数据为空"))
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
        let request = CreateMailboxFolderRequest::new(arc_config.clone(), "test".to_string());
        let _ = request;
    }
}
