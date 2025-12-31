use openlark_core::{
    api::ApiRequest,
    config::Config,
    http::Transport,
    SDKResult,
};

/// 创建文档版本
///
/// 为源文档创建版本文档。
/// docPath: /document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file-version/create
use serde::Serialize;

use crate::common::{api_endpoints::DriveApi, api_utils::*};

use super::models::FileVersionInfo;

/// 创建文档版本请求
#[derive(Debug, Clone)]
pub struct CreateFileVersionRequest {
    config: Config,
    /// 源文档 token
    pub file_token: String,
    /// 用户 ID 类型
    pub user_id_type: Option<String>,
    /// 版本文档标题
    pub name: String,
    /// 源文档类型（docx/sheet）
    pub obj_type: String,
}

impl CreateFileVersionRequest {
    pub fn new(
        config: Config,
        file_token: impl Into<String>,
        name: impl Into<String>,
        obj_type: impl Into<String>,
    ) -> Self {
        Self {
            config,
            file_token: file_token.into(),
            user_id_type: None,
            name: name.into(),
            obj_type: obj_type.into(),
        }
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    pub async fn execute(self) -> SDKResult<CreateFileVersionResponse> {
        if self.file_token.is_empty() {
            return Err(openlark_core::error::validation_error(
                "file_token",
                "file_token 不能为空",
            ));
        }
        if self.name.is_empty() {
            return Err(openlark_core::error::validation_error("name", "name 不能为空"));
        }
        if self.name.chars().count() > 1024 {
            return Err(openlark_core::error::validation_error(
                "name",
                "name 最大长度为 1024 个 Unicode 码点",
            ));
        }
        match self.obj_type.as_str() {
            "docx" | "sheet" => {}
            _ => {
                return Err(openlark_core::error::validation_error(
                    "obj_type",
                    "obj_type 仅支持 docx/sheet",
                ))
            }
        }

        #[derive(Serialize)]
        struct VersionPayload {
            name: String,
            obj_type: String,
        }

        let api_endpoint = DriveApi::CreateFileVersion(self.file_token);
        let payload = VersionPayload {
            name: self.name,
            obj_type: self.obj_type,
        };

        let request = ApiRequest::<CreateFileVersionResponse>::post(&api_endpoint.to_url())
            .query_opt("user_id_type", self.user_id_type)
            .body(serialize_params(&payload, "创建文档版本")?);

        let response = Transport::request(request, &self.config, None).await?;
        extract_response_data(response, "创建文档版本")
    }
}

/// 创建文档版本响应（data）
pub type CreateFileVersionResponse = FileVersionInfo;

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::api::ApiResponseTrait;

    #[test]
    fn test_create_file_version_request_builder() {
        let config = Config::default();
        let request =
            CreateFileVersionRequest::new(config, "file_token", "项目文档 第 1 版", "docx")
                .user_id_type("open_id");

        assert_eq!(request.file_token, "file_token");
        assert_eq!(request.name, "项目文档 第 1 版");
        assert_eq!(request.obj_type, "docx");
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(
            <FileVersionInfo as ApiResponseTrait>::data_format(),
            openlark_core::api::ResponseFormat::Data
        );
    }
}
