use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

/// 启用云文档密码
///
/// 为云文档启用密码保护功能（平台自动生成密码）。
/// docPath: /document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/permission-public-password/create
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

/// 启用云文档密码请求
#[derive(Debug, Clone)]
pub struct CreatePermissionPublicPasswordRequest {
    config: Config,
    /// 云文档 token
    pub token: String,
    /// 云文档类型（需要与 token 匹配）
    pub r#type: String,
}

impl CreatePermissionPublicPasswordRequest {
    pub fn new(config: Config, token: impl Into<String>, r#type: impl Into<String>) -> Self {
        Self {
            config,
            token: token.into(),
            r#type: r#type.into(),
        }
    }

    pub async fn execute(self) -> SDKResult<CreatePermissionPublicPasswordResponse> {
        if self.token.is_empty() {
            return Err(openlark_core::error::validation_error(
                "token",
                "token 不能为空",
            ));
        }
        if self.r#type.is_empty() {
            return Err(openlark_core::error::validation_error(
                "type",
                "type 不能为空",
            ));
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
                "type=minutes 暂不支持启用云文档密码",
            ));
        }

        let api_endpoint = DriveApi::CreatePublicPassword(self.token);
        let request =
            ApiRequest::<CreatePermissionPublicPasswordResponse>::post(&api_endpoint.to_url())
                .query("type", self.r#type);

        let response = Transport::request(request, &self.config, None).await?;
        extract_response_data(response, "启用云文档密码")
    }
}

/// 启用云文档密码响应（data）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePermissionPublicPasswordResponse {
    /// 密码
    pub password: String,
}

impl ApiResponseTrait for CreatePermissionPublicPasswordResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_permission_public_password_request_builder() {
        let config = Config::default();
        let request = CreatePermissionPublicPasswordRequest::new(config, "token", "docx");
        assert_eq!(request.token, "token");
        assert_eq!(request.r#type, "docx");
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(
            CreatePermissionPublicPasswordResponse::data_format(),
            ResponseFormat::Data
        );
    }
}
