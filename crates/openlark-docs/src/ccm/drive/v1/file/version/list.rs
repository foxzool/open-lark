/// 获取文件版本列表
///
/// 获取指定文件的所有版本列表。
/// docPath: https://open.feishu.cn/open-apis/drive/v1/files/:file_token/versions
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

/// 获取文件版本列表请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListFileVersionsRequest {
    /// 文件token
    pub file_token: String,
    /// 分页大小
    pub page_size: Option<u32>,
    /// 分页标记
    pub page_token: Option<String>,
}

impl ListFileVersionsRequest {
    /// 创建获取文件版本列表请求
    ///
    /// # 参数
    /// * `file_token` - 文件token
    pub fn new(file_token: impl Into<String>) -> Self {
        Self {
            file_token: file_token.into(),
            page_size: None,
            page_token: None,
        }
    }

    /// 设置分页大小
    pub fn page_size(mut self, page_size: u32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }
}

/// 获取文件版本列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListFileVersionsResponse {
    /// 版本列表信息
    pub data: Option<VersionListData>,
}

impl ApiResponseTrait for ListFileVersionsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 版本列表数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionListData {
    /// 版本列表
    pub versions: Vec<VersionInfo>,
    /// 是否有更多数据
    pub has_more: bool,
    /// 分页标记
    pub page_token: Option<String>,
}

/// 版本信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionInfo {
    /// 版本ID
    pub version_id: String,
    /// 版本号
    pub version_number: i32,
    /// 版本名称
    pub name: String,
    /// 创建时间
    pub created_at: String,
    /// 创建者信息
    pub creator: CreatorInfo,
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
}

/// 创建者信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatorInfo {
    /// 用户ID
    pub user_id: String,
    /// 用户名
    pub name: String,
    /// 头像
    pub avatar: Option<AvatarInfo>,
}

/// 头像信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvatarInfo {
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

/// 获取文件版本列表
///
/// 获取指定文件的所有版本列表。
/// docPath: https://open.feishu.cn/open-apis/drive/v1/files/:file_token/versions
pub async fn list_file_versions(
    request: ListFileVersionsRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<ListFileVersionsResponse>> {
    // 创建API请求
    let url = DriveApi::ListFileVersions(request.file_token.clone()).to_url();
    let mut api_request: ApiRequest<ListFileVersionsResponse> =
        ApiRequest::get(&url);

    // 添加查询参数
    if let Some(page_size) = request.page_size {
        api_request = api_request.query_param("page_size", &page_size.to_string());
    }
    if let Some(page_token) = &request.page_token {
        api_request = api_request.query_param("page_token", page_token);
    }

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
    fn test_list_file_versions_request_builder() {
        let request = ListFileVersionsRequest::new("file_token")
            .page_size(20)
            .page_token("token123");

        assert_eq!(request.file_token, "file_token");
        assert_eq!(request.page_size, Some(20));
        assert_eq!(request.page_token, Some("token123".to_string()));
    }

    #[test]
    fn test_version_list_data_structure() {
        let versions = vec![
            VersionInfo {
                version_id: "v1".to_string(),
                version_number: 1,
                name: "版本1".to_string(),
                created_at: "2023-01-01T00:00:00Z".to_string(),
                creator: CreatorInfo {
                    user_id: "user1".to_string(),
                    name: "张三".to_string(),
                    avatar: None,
                },
                size: 1024,
                r#type: "docx".to_string(),
                is_major: true,
                remarks: None,
                changes: None,
            }
        ];

        let data = VersionListData {
            versions: versions.clone(),
            has_more: true,
            page_token: Some("next_token".to_string()),
        };

        assert_eq!(data.versions.len(), 1);
        assert_eq!(data.has_more, true);
        assert_eq!(data.page_token, Some("next_token".to_string()));
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(ListFileVersionsResponse::data_format(), ResponseFormat::Data);
    }
}