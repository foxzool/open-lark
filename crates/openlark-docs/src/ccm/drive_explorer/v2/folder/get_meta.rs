/// 获取文件夹元数据
///
/// 获取指定文件夹的详细信息，包括名称、创建时间、权限等。
/// docPath: https://open.feishu.cn/open-apis/drive/explorer/v2/folder/:folderToken/meta
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::CcmDriveExplorerApiOld, api_utils::*};

/// 获取文件夹元数据请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFolderMetaRequest {
    /// 文件夹token
    pub folder_token: String,
}

impl GetFolderMetaRequest {
    /// 创建获取文件夹元数据请求
    ///
    /// # 参数
    /// * `folder_token` - 文件夹token
    pub fn new(folder_token: impl Into<String>) -> Self {
        Self {
            folder_token: folder_token.into(),
        }
    }

    /// 设置文件夹token
    pub fn folder_token(mut self, folder_token: impl Into<String>) -> Self {
        self.folder_token = folder_token.into();
        self
    }
}

/// 获取文件夹元数据响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFolderMetaResponse {
    /// 文件夹信息
    pub data: Option<FolderMeta>,
}

impl ApiResponseTrait for GetFolderMetaResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 文件夹元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderMeta {
    /// 文件夹token
    pub folder_token: String,
    /// 文件夹名称
    pub name: String,
    /// 文件夹类型
    pub r#type: String,
    /// 创建时间
    pub create_time: String,
    /// 修改时间
    pub update_time: String,
    /// 创建者信息
    pub creator: CreatorInfo,
    /// 父文件夹token
    pub parent_token: Option<String>,
}

/// 创建者信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatorInfo {
    /// 用户ID
    pub user_id: String,
    /// 用户名
    pub name: String,
}

/// 获取文件夹元数据
///
/// 获取指定文件夹的详细信息，包括名称、创建时间、权限等。
/// docPath: https://open.feishu.cn/open-apis/drive/explorer/v2/folder/:folderToken/meta
pub async fn get_folder_meta(
    request: GetFolderMetaRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<GetFolderMetaResponse>> {
    // 使用CcmDriveExplorerApiOld枚举生成API端点
    let api_endpoint = CcmDriveExplorerApiOld::FolderMeta(request.folder_token);

    // 创建API请求
    let mut api_request: ApiRequest<GetFolderMetaResponse> =
        ApiRequest::get(&api_endpoint.to_url());

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
    fn test_get_folder_meta_request_builder() {
        let request = GetFolderMetaRequest::new("folder_token");

        assert_eq!(request.folder_token, "folder_token");
    }

    #[test]
    fn test_get_folder_meta_request_builder_chain() {
        let request = GetFolderMetaRequest::new("folder_token_1")
            .folder_token("folder_token_2");

        assert_eq!(request.folder_token, "folder_token_2");
    }

    #[test]
    fn test_folder_meta_structure() {
        let creator = CreatorInfo {
            user_id: "user_id".to_string(),
            name: "用户名".to_string(),
        };

        let folder_meta = FolderMeta {
            folder_token: "folder_token".to_string(),
            name: "文件夹名称".to_string(),
            r#type: "folder".to_string(),
            create_time: "2023-01-01T00:00:00Z".to_string(),
            update_time: "2023-01-02T00:00:00Z".to_string(),
            creator: creator.clone(),
            parent_token: Some("parent_token".to_string()),
        };

        assert_eq!(folder_meta.folder_token, "folder_token");
        assert_eq!(folder_meta.name, "文件夹名称");
        assert_eq!(folder_meta.creator.name, "用户名");
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(GetFolderMetaResponse::data_format(), ResponseFormat::Data);
    }
}