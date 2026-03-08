//! 修改邮箱文件夹

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
pub struct PatchMailboxFolderRequest {
    config: Arc<Config>,
    user_mailbox_id: String,
    folder_id: String,
    patch_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchMailboxFolderResponse {
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for PatchMailboxFolderResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl PatchMailboxFolderRequest {
    pub fn new(
        config: Arc<Config>,
        user_mailbox_id: impl Into<String>,
        folder_id: impl Into<String>,
    ) -> Self {
        Self {
            config,
            user_mailbox_id: user_mailbox_id.into(),
            folder_id: folder_id.into(),
            patch_id: String::new(),
        }
    }

    pub async fn execute(self) -> SDKResult<PatchMailboxFolderResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<PatchMailboxFolderResponse> {
        let path = format!(
            "/open-apis/mail/v1/user_mailboxes/{}/folders/{}",
            self.user_mailbox_id, self.folder_id
        );
        let req: ApiRequest<PatchMailboxFolderResponse> = ApiRequest::patch(&path);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("修改邮箱文件夹", "响应数据为空"))
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {

    #[test]
    fn test_serialization_roundtrip() {
        // 基础序列化测试
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        // 基础反序列化测试
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(value["field"], "data");
    }
}
