use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

/// 获取文档版本信息
///
/// 获取指定源文档的指定版本信息。
/// docPath: /document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file-version/get
use crate::common::{api_endpoints::DriveApi, api_utils::*};

use super::models::FileVersionInfo;

/// 获取文档版本信息请求
#[derive(Debug, Clone)]
pub struct GetFileVersionRequest {
    config: Config,
    /// 源文档 token
    pub file_token: String,
    /// 版本文档的版本标识
    pub version_id: String,
    /// 源文档类型（docx/sheet）
    pub obj_type: String,
    /// 用户 ID 类型
    pub user_id_type: Option<String>,
}

impl GetFileVersionRequest {
    pub fn new(
        config: Config,
        file_token: impl Into<String>,
        version_id: impl Into<String>,
        obj_type: impl Into<String>,
    ) -> Self {
        Self {
            config,
            file_token: file_token.into(),
            version_id: version_id.into(),
            obj_type: obj_type.into(),
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    pub async fn execute(self) -> SDKResult<GetFileVersionResponse> {
        if self.file_token.is_empty() {
            return Err(openlark_core::error::validation_error(
                "file_token",
                "file_token 不能为空",
            ));
        }
        if self.version_id.is_empty() {
            return Err(openlark_core::error::validation_error(
                "version_id",
                "version_id 不能为空",
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

        let api_endpoint = DriveApi::GetFileVersion(self.file_token, self.version_id);
        let request = ApiRequest::<GetFileVersionResponse>::get(&api_endpoint.to_url())
            .query("obj_type", self.obj_type)
            .query_opt("user_id_type", self.user_id_type);

        let response = Transport::request(request, &self.config, None).await?;
        extract_response_data(response, "获取文档版本信息")
    }
}

/// 获取文档版本信息响应（data）
pub type GetFileVersionResponse = FileVersionInfo;

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::api::ApiResponseTrait;

    #[test]
    fn test_get_file_version_request_builder() {
        let config = Config::default();
        let request = GetFileVersionRequest::new(config, "file_token", "fnJfyX", "docx")
            .user_id_type("open_id");

        assert_eq!(request.file_token, "file_token");
        assert_eq!(request.version_id, "fnJfyX");
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
