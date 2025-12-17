use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
/// 创建文档版本
///
/// 为文件创建新版本
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/file-version/create
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::DriveApi;

/// 创建文档版本请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFileVersionRequest {
    /// 文件token
    pub file_token: String,
    /// 版本名称，可选
    pub name: Option<String>,
    /// 版本描述，可选
    pub description: Option<String>,
}

impl CreateFileVersionRequest {
    /// 创建文档版本请求
    ///
    /// # 参数
    /// * `file_token` - 文件token
    pub fn new(file_token: impl Into<String>) -> Self {
        Self {
            file_token: file_token.into(),
            name: None,
            description: None,
        }
    }

    /// 设置版本名称
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// 设置版本描述
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
}

/// 版本信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateVersionInfo {
    /// 版本ID
    pub version_id: String,
    /// 版本名称
    pub name: Option<String>,
    /// 版本描述
    pub description: Option<String>,
    /// 创建时间
    pub create_time: i64,
    /// 创建者信息
    pub creator: CreateVersionCreatorInfo,
}

/// 创建者信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateVersionCreatorInfo {
    /// 用户ID
    pub user_id: String,
    /// 用户名称
    pub name: String,
}

/// 创建文档版本响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFileVersionResponse {
    /// 版本信息
    pub data: Option<CreateVersionInfo>,
}

impl ApiResponseTrait for CreateFileVersionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建文档版本
///
/// 为文件创建新版本
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/file-version/create
pub async fn create_file_version(
    request: CreateFileVersionRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<CreateFileVersionResponse>> {
    // 使用DriveApi枚举生成API端点
    let api_endpoint = DriveApi::CreateFileVersion(request.file_token.clone());

    // 创建API请求
    let mut api_request: ApiRequest<CreateFileVersionResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(serde_json::json!({
            "name": request.name,
            "description": request.description
        }));

    // 如果有请求选项，应用它们
    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    // 发送请求
    Transport::request(api_request, config, None).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_file_version_request_builder() {
        let request = CreateFileVersionRequest::new("file_token");

        assert_eq!(request.file_token, "file_token");
        assert!(request.name.is_none());
        assert!(request.description.is_none());
    }

    #[test]
    fn test_create_file_version_request_with_params() {
        let request = CreateFileVersionRequest::new("file_token")
            .name("版本1.0")
            .description("初始版本");

        assert_eq!(request.file_token, "file_token");
        assert_eq!(request.name.unwrap(), "版本1.0");
        assert_eq!(request.description.unwrap(), "初始版本");
    }

    #[test]
    fn test_version_info_structure() {
        let creator = CreateVersionCreatorInfo {
            user_id: "user_id".to_string(),
            name: "用户名".to_string(),
        };

        let version = CreateVersionInfo {
            version_id: "version_id".to_string(),
            name: Some("版本1.0".to_string()),
            description: Some("初始版本".to_string()),
            create_time: 1640995200,
            creator,
        };

        assert_eq!(version.version_id, "version_id");
        assert_eq!(version.name.unwrap(), "版本1.0");
        assert_eq!(version.description.unwrap(), "初始版本");
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(
            CreateFileVersionResponse::data_format(),
            ResponseFormat::Data
        );
    }
}
