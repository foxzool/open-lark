//! 获取云文档权限设置
//!
//! 获取指定云文档的公共访问与协作权限设置。
//!
//! docPath: https://open.feishu.cn/document/server-docs/docs/permission/permission-public/get

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

use super::models::PermissionPublic;

/// 获取云文档权限设置请求
#[derive(Debug, Clone)]
pub struct GetPublicPermissionRequest {
    config: Config,
    /// 云文档 token
    pub token: String,
    /// 云文档类型（需要与 token 匹配）
    pub r#type: String,
}

impl GetPublicPermissionRequest {
    pub fn new(config: Config, token: impl Into<String>, r#type: impl Into<String>) -> Self {
        Self {
            config,
            token: token.into(),
            r#type: r#type.into(),
        }
    }

    pub async fn execute(self) -> SDKResult<GetPublicPermissionResponse> {
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

        let api_endpoint = DriveApi::GetPublicPermission(self.token);
        let request = ApiRequest::<GetPublicPermissionResponse>::get(&api_endpoint.to_url())
            .query("type", self.r#type);

        let response = Transport::request(request, &self.config, None).await?;
        extract_response_data(response, "获取云文档权限设置")
    }
}

/// 获取云文档权限设置响应（data）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPublicPermissionResponse {
    /// 返回的文档公共访问和协作权限设置
    pub permission_public: PermissionPublic,
}

impl ApiResponseTrait for GetPublicPermissionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_public_permission_request_builder() {
        let config = Config::default();
        let request = GetPublicPermissionRequest::new(config, "doc_token", "docx");

        assert_eq!(request.token, "doc_token");
        assert_eq!(request.r#type, "docx");
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(
            GetPublicPermissionResponse::data_format(),
            ResponseFormat::Data
        );
    }
}
