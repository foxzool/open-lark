use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
/// 获取根目录元数据
///
/// 此接口用于获取用户云盘根目录的元数据信息，包括目录属性、权限等。
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/explorer/root_folder_meta
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::CcmDriveExplorerApi, api_utils::*};

use serde_json::json;

/// 获取根目录元数据请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RootFolderMetaRequest {
    /// 用户ID
    pub user_id: Option<String>,
    /// 用户类型
    pub user_id_type: Option<String>,
}

impl RootFolderMetaRequest {
    /// 创建获取根目录元数据请求
    pub fn new() -> Self {
        Self {
            user_id: None,
            user_id_type: None,
        }
    }

    /// 设置用户ID
    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = Some(user_id.into());
        self
    }

    /// 设置用户类型
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }
}

/// 根目录元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RootFolderMetaData {
    /// 文件夹token
    pub folder_token: String,
    /// 文件夹名称
    pub name: String,
    /// 文件夹类型
    pub obj_type: String,
    /// 文件夹URL
    pub url: String,
    /// 创建时间
    pub create_time: String,
    /// 更新时间
    pub update_time: String,
    /// 所有者
    pub owner: Option<UserInfo>,
    /// 权限
    pub permissions: Option<serde_json::Value>,
    /// 扩展信息
    pub extra: Option<serde_json::Value>,
}

/// 用户信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    /// 用户ID
    pub user_id: String,
    /// 用户名称
    pub name: String,
    /// 用户邮箱
    pub email: Option<String>,
}

/// 获取根目录元数据响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RootFolderMetaResponse {
    /// 根目录元数据
    pub data: Option<RootFolderMetaData>,
}

impl ApiResponseTrait for RootFolderMetaResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取根目录元数据
///
/// 获取用户云盘根目录的元数据信息，包括目录属性、权限等。
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/explorer/root_folder_meta
pub async fn root_folder_meta(
    request: RootFolderMetaRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<RootFolderMetaResponse> {
    // 使用CcmDriveExplorerApi枚举生成API端点
    let api_endpoint = CcmDriveExplorerApi::RootFolderMeta;

    // 创建查询参数
    let mut query_params = Vec::new();

    if let Some(user_id) = &request.user_id {
        query_params.push(("user_id", user_id.clone()));
    }
    if let Some(user_id_type) = &request.user_id_type {
        query_params.push(("user_id_type", user_id_type.clone()));
    }

    // 创建API请求
    let mut api_request: ApiRequest<RootFolderMetaResponse> =
        ApiRequest::get(&api_endpoint.to_url_with_params(&query_params));

    // 如果有请求选项，应用它们
    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    // 发送请求
    let response = Transport::request(api_request, config, None).await?;

    // 返回数据
    response.data.ok_or_else(|| {
        openlark_core::error::validation_error("response_data", "Response data is missing")
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_root_folder_meta_request_builder() {
        let request = RootFolderMetaRequest::new()
            .user_id("user_id")
            .user_id_type("open_id");

        assert_eq!(request.user_id, Some("user_id".to_string()));
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
    }

    #[test]
    fn test_root_folder_meta_request_minimal() {
        let request = RootFolderMetaRequest::new();

        assert_eq!(request.user_id, None);
        assert_eq!(request.user_id_type, None);
    }

    #[test]
    fn test_root_folder_meta_data_structure() {
        let user_info = UserInfo {
            user_id: "user_id".to_string(),
            name: "用户名".to_string(),
            email: Some("email@example.com".to_string()),
        };

        let meta_data = RootFolderMetaData {
            folder_token: "folder_token".to_string(),
            name: "我的云盘".to_string(),
            obj_type: "folder".to_string(),
            url: "https://example.com".to_string(),
            create_time: "2023-01-01T00:00:00Z".to_string(),
            update_time: "2023-01-01T00:00:00Z".to_string(),
            owner: Some(user_info.clone()),
            permissions: Some(json!({"read": true})),
            extra: Some(json!({"custom": "value"})),
        };

        assert_eq!(meta_data.folder_token, "folder_token");
        assert_eq!(meta_data.name, "我的云盘");
        assert_eq!(meta_data.owner.as_ref().unwrap().name, "用户名");
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(RootFolderMetaResponse::data_format(), ResponseFormat::Data);
    }
}
