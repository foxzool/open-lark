/// 关闭云文档密码
///
/// 该接口用于根据 filetoken 关闭云文档密码。
/// docPath: /document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/permission-public-password/delete
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

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

    pub async fn execute(self) -> SDKResult<DeletePermissionPublicPasswordResponse> {
        if self.token.is_empty() {
            return Err(openlark_core::error::validation_error("token", "token 不能为空"));
        }
        if self.r#type.is_empty() {
            return Err(openlark_core::error::validation_error("type", "type 不能为空"));
        }
        match self.r#type.as_str() {
            "doc" | "sheet" | "file" | "wiki" | "bitable" | "docx" | "mindnote" | "minutes"
            | "slides" => {}
            _ => {
                return Err(openlark_core::error::validation_error(
                    "type",
                    "type 必须为 doc/sheet/file/wiki/bitable/docx/mindnote/minutes/slides",
                ));
            }
        }
        if self.r#type == "minutes" {
            return Err(openlark_core::error::validation_error(
                "type",
                "type=minutes 暂不支持停用云文档密码",
            ));
        }

        let api_endpoint = DriveApi::DeletePublicPassword(self.token);
        let api_request: ApiRequest<DeletePermissionPublicPasswordResponse> =
            ApiRequest::delete(&api_endpoint.to_url()).query("type", self.r#type);

        let response = Transport::request(api_request, &self.config, None).await?;
        extract_response_data(response, "停用云文档密码")
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
        assert_eq!(
            DeletePermissionPublicPasswordResponse::data_format(),
            ResponseFormat::Data
        );
    }
}
