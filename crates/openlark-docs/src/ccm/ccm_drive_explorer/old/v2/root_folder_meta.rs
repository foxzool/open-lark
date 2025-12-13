/// 获取我的空间（根文件夹）元数据
///
/// 获取 "我的空间" 的元信息。
/// API文档: https://open.feishu.cn/document/server-docs/docs/drive-v1/folder/get-root-folder-meta
/// 对应CSV记录: https://open.feishu.cn/document/server-docs/docs/drive-v1/folder/get-root-folder-meta

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmDriveExplorerApiOld;

/// 获取我的空间（根文件夹）元数据响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRootFolderMetaResponse {
    /// 根文件夹元数据
    pub data: Option<FolderMeta>,
}

/// 文件夹元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderMeta {
    /// 文件夹token
    #[serde(rename = "folder_token")]
    pub folder_token: String,
    /// 文件夹名称
    pub name: String,
    /// 创建时间
    #[serde(rename = "create_time")]
    pub create_time: i64,
    /// 更新时间
    #[serde(rename = "update_time")]
    pub update_time: i64,
    /// 创建者信息
    pub creator: Option<UserInfo>,
    /// 子文件夹和文件数量
    #[serde(rename = "child_count")]
    pub child_count: i32,
}

/// 用户信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    /// 用户ID
    #[serde(rename = "user_id")]
    pub user_id: String,
    /// 用户名称
    pub name: String,
}

impl ApiResponseTrait for GetRootFolderMetaResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取我的空间（根文件夹）元数据请求
pub struct GetRootFolderMetaRequest {
    config: Config,
}

impl GetRootFolderMetaRequest {
    /// 创建获取我的空间（根文件夹）元数据请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// API文档: https://open.feishu.cn/document/server-docs/docs/drive-v1/folder/get-root-folder-meta
    /// 对应CSV记录: https://open.feishu.cn/document/server-docs/docs/drive-v1/folder/get-root-folder-meta
    pub async fn execute(self) -> SDKResult<GetRootFolderMetaResponse> {
        // 使用enum+builder系统生成API端点
        let api_endpoint = CcmDriveExplorerApiOld::RootFolderMeta;

        // 创建API请求 - 使用类型安全的URL生成
        let api_request: ApiRequest<GetRootFolderMetaResponse> =
            ApiRequest::get(&api_endpoint.to_url());

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}