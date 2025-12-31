use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

/// 刷新云文档密码
///
/// 刷新云文档的密码保护设置（平台自动生成新密码）。
/// docPath: /document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/permission-public-password/update
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

/// 刷新云文档密码请求
#[derive(Debug, Clone)]
pub struct UpdatePermissionPublicPasswordRequest {
    config: Config,
    /// 云文档 token
    pub token: String,
    /// 云文档类型（需要与 token 匹配）
    pub r#type: String,
}

impl UpdatePermissionPublicPasswordRequest {
    pub fn new(config: Config, token: impl Into<String>, r#type: impl Into<String>) -> Self {
        Self {
            config,
            token: token.into(),
            r#type: r#type.into(),
        }
    }

    pub async fn execute(self) -> SDKResult<UpdatePermissionPublicPasswordResponse> {
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
                "type=minutes 暂不支持刷新云文档密码",
            ));
        }

        let api_endpoint = DriveApi::UpdatePublicPassword(self.token);
        let request =
            ApiRequest::<UpdatePermissionPublicPasswordResponse>::put(&api_endpoint.to_url())
                .query("type", self.r#type);

        let response = Transport::request(request, &self.config, None).await?;
        extract_response_data(response, "刷新云文档密码")
    }
}

/// 刷新云文档密码响应（data）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePermissionPublicPasswordResponse {
    /// 新密码
    pub password: String,
}

impl ApiResponseTrait for UpdatePermissionPublicPasswordResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_permission_public_password_request_builder() {
        let config = Config::default();
        let request = UpdatePermissionPublicPasswordRequest::new(config, "token", "docx");
        assert_eq!(request.token, "token");
        assert_eq!(request.r#type, "docx");
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(
            UpdatePermissionPublicPasswordResponse::data_format(),
            ResponseFormat::Data
        );
    }
}
