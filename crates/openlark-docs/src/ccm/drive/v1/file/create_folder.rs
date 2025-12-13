/// 新建文件夹
///
/// 在用户云空间的指定文件夹中创建一个新的空文件夹。
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/folder/create_folder

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use serde_json::json;


// 导入序列化支持
use serde::{Deserialize, Serialize};
use crate::common::{api_endpoints::DriveApi, api_utils::*};

/// 新建文件夹请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFolderRequest {
    /// 文件夹名称
    pub name: String,
    /// 父文件夹token
    pub parent_folder_token: String,
}

/// 文件夹信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderInfo {
    /// 文件夹token
    pub file_token: String,
    /// 文件夹名称
    pub name: String,
    /// 文件夹类型
    pub r#type: String,
    /// 创建时间
    pub created_time: String,
    /// 修改时间
    pub modified_time: String,
    /// 父文件夹token
    pub parent_folder_token: String,
}

/// 新建文件夹响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFolderResponse {
    /// 文件夹信息
    pub data: Option<FolderInfo>,
}

impl ApiResponseTrait for CreateFolderResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 新建文件夹
///
/// 在用户云空间的指定文件夹中创建一个新的空文件夹。
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/folder/create_folder
pub async fn create_folder(
    config: &Config,
    params: CreateFolderRequest,
) -> SDKResult<CreateFolderResponse> {
    // 使用DriveApi枚举生成API端点
    let api_endpoint = DriveApi::CreateFolder;

    // 创建API请求
    let api_request: ApiRequest<CreateFolderResponse> =
        ApiRequest::post(&api_endpoint.to_url())
            .body(json!(params));

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "新建文件夹")
}