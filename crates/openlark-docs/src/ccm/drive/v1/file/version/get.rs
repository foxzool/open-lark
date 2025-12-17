/// 获取文件版本信息
///
/// 获取指定文件的特定版本详细信息。
/// docPath: https://open.feishu.cn/open-apis/drive/v1/files/:file_token/versions/:version_id
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::DriveApi;

/// 获取文件版本信息请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFileVersionRequest {
    /// 文件token
    pub file_token: String,
    /// 版本ID
    pub version_id: String,
}

impl GetFileVersionRequest {
    /// 创建获取文件版本信息请求
    ///
    /// # 参数
    /// * `file_token` - 文件token
    /// * `version_id` - 版本ID
    pub fn new(
        file_token: impl Into<String>,
        version_id: impl Into<String>,
    ) -> Self {
        Self {
            file_token: file_token.into(),
            version_id: version_id.into(),
        }
    }
}

/// 获取文件版本信息响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFileVersionResponse {
    /// 版本信息
    pub data: Option<GetVersionInfo>,
}

impl ApiResponseTrait for GetFileVersionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 版本信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetVersionInfo {
    /// 版本ID
    pub version_id: String,
    /// 版本号
    pub version_number: i32,
    /// 版本名称
    pub name: String,
    /// 创建时间
    pub created_at: String,
    /// 创建者信息
    pub creator: GetVersionCreatorInfo,
    /// 文件大小
    pub size: u64,
    /// 版本类型
    pub r#type: String,
    /// 是否为主要版本
    pub is_major: bool,
    /// 版本备注
    pub remarks: Option<String>,
    /// 修改历史
    pub changes: Option<Vec<ChangeInfo>>,
    /// 下载链接
    pub download_url: Option<String>,
    /// 预览链接
    pub preview_url: Option<String>,
}

/// 创建者信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetVersionCreatorInfo {
    /// 用户ID
    pub user_id: String,
    /// 用户名
    pub name: String,
    /// 头像
    pub avatar: Option<GetVersionAvatarInfo>,
}

/// 头像信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetVersionAvatarInfo {
    /// 头像URL
    pub url: String,
    /// 头像大小
    pub size: Option<u32>,
}

/// 修改信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeInfo {
    /// 修改类型
    pub r#type: String,
    /// 修改描述
    pub description: String,
}

/// 获取文件版本信息
///
/// 获取指定文件的特定版本详细信息。
/// docPath: https://open.feishu.cn/open-apis/drive/v1/files/:file_token/versions/:version_id
pub async fn get_file_version(
    request: GetFileVersionRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<GetFileVersionResponse>> {
    // 创建API请求
    let url = DriveApi::GetFileVersion(request.file_token.clone(), request.version_id.clone()).to_url();
    let mut api_request: ApiRequest<GetFileVersionResponse> =
        ApiRequest::get(&url);

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
    fn test_get_file_version_request_builder() {
        let request = GetFileVersionRequest::new("file_token", "version_id");
        assert_eq!(request.file_token, "file_token");
        assert_eq!(request.version_id, "version_id");
    }

    #[test]
    fn test_version_info_structure() {
        let creator = CreatorInfo {
            user_id: "user_123".to_string(),
            name: "张三".to_string(),
            avatar: Some(AvatarInfo {
                url: "https://example.com/avatar.jpg".to_string(),
                size: Some(128),
            }),
        };

        let version_info = VersionInfo {
            version_id: "version_456".to_string(),
            version_number: 1,
            name: "第一版".to_string(),
            created_at: "2023-01-01T00:00:00Z".to_string(),
            creator,
            size: 1024,
            r#type: "docx".to_string(),
            is_major: true,
            remarks: Some("初始版本".to_string()),
            changes: None,
            download_url: Some("https://example.com/download".to_string()),
            preview_url: Some("https://example.com/preview".to_string()),
        };

        assert_eq!(version_info.version_id, "version_456");
        assert_eq!(version_info.version_number, 1);
        assert_eq!(version_info.is_major, true);
        assert_eq!(version_info.creator.name, "张三");
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(GetFileVersionResponse::data_format(), ResponseFormat::Data);
    }
}