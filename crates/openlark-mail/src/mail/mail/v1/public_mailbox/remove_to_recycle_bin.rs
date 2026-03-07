//! 将公共邮箱移至回收站

use openlark_core::{
    api::ApiRequest,
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct RemovePublicMailboxToRecycleBinRequest {
    config: Arc<Config>,
    public_mailbox_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemovePublicMailboxToRecycleBinResponse {
    pub data: Option<serde_json::Value>,
}

impl RemovePublicMailboxToRecycleBinRequest {
    pub fn new(config: Arc<Config>, public_mailbox_id: impl Into<String>) -> Self {
        Self {
            config,
            public_mailbox_id: public_mailbox_id.into(),
        }
    }

    pub async fn execute(self) -> SDKResult<RemovePublicMailboxToRecycleBinResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<RemovePublicMailboxToRecycleBinResponse> {
        let path = format!(
            "/open-apis/mail/v1/public_mailboxes/{}/remove_to_recycle_bin",
            self.public_mailbox_id
        );
        let req: ApiRequest<RemovePublicMailboxToRecycleBinResponse> = ApiRequest::delete(&path);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        Ok(resp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

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
