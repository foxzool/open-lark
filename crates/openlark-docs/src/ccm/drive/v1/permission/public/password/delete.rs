/// 关闭云文档密码
///
/// 该接口用于根据 filetoken 关闭云文档密码。
/// docPath: https://open.feishu.cn/document/server-docs/docs/permission/permission-public-password/delete
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletePermissionPublicPasswordRequest {
    #[serde(skip)]
    config: Config,
    pub token: String,
    pub r#type: String,
}

impl DeletePermissionPublicPasswordRequest {
    pub fn new(config: Config, token: impl Into<String>, r#type: impl Into<String>) -> Self {
        Self {
            config,
            token: token.into(),
            r#type: r#type.into(),
        }
    }

    pub async fn execute(self) -> SDKResult<Response<DeletePermissionPublicPasswordResponse>> {
        let url = format!("/open-apis/drive/v1/permissions/{}/public/password", self.token);
        let mut api_request: ApiRequest<DeletePermissionPublicPasswordResponse> = ApiRequest::delete(&url);
        api_request = api_request.query("type", &self.r#type);
        
        Transport::request(api_request, &self.config, None).await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletePermissionPublicPasswordResponse {
    // Empty response body usually implies success
}

impl ApiResponseTrait for DeletePermissionPublicPasswordResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_permission_public_password_request_builder() {
        let config = Config::default();
        let request = DeletePermissionPublicPasswordRequest::new(config, "token", "type");
        assert_eq!(request.token, "token");
        assert_eq!(request.r#type, "type");
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(DeletePermissionPublicPasswordResponse::data_format(), ResponseFormat::Data);
    }
}
