/// 获取文件夹中的文件清单
///
/// 获取用户云空间中指定文件夹下的文件清单。清单类型包括文件、各种在线文档（文档、电子表格、多维表格、思维笔记）、文件夹和快捷方式。
/// 该接口支持分页，但是不会递归的获取子文件夹的清单。
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/folder/list

use serde::{Deserialize, Serialize};
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

/// 获取文件夹中的文件清单请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListFilesRequest {
    /// 文件夹token，不填则获取根目录
    pub parent_folder_token: Option<String>,
    /// 分页大小，默认50，最大1000
    pub page_size: Option<i32>,
    /// 分页标记，第一页不填，后续页面使用上一页返回的page_token
    pub page_token: Option<String>,
    /// 排序字段：name|modified_time|created_time|size
    pub order_by: Option<String>,
    /// 排序方向：asc|desc
    pub direction: Option<String>,
    /// 搜索关键字
    pub search_key: Option<String>,
    /// 文件类型过滤
    pub file_type: Option<String>,
}

/// 文件信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    /// 文件token
    pub file_token: String,
    /// 文件名称
    pub name: String,
    /// 文件类型
    pub r#type: String,
    /// 文件大小
    pub size: i64,
    /// 创建时间
    pub created_time: String,
    /// 修改时间
    pub modified_time: String,
    /// 创建者信息
    pub creator: Option<serde_json::Value>,
    /// 父文件夹token
    pub parent_folder_token: String,
}

/// 获取文件夹中的文件清单响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListFilesResponse {
    /// 文件列表信息
    pub data: Option<ListFilesData>,
}

/// 文件列表数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListFilesData {
    /// 文件列表
    pub files: Vec<FileInfo>,
    /// 分页token
    pub page_token: Option<String>,
    /// 是否有更多
    pub has_more: bool,
}

impl ApiResponseTrait for ListFilesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取文件夹中的文件清单
///
/// 获取用户云空间中指定文件夹下的文件清单。
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/folder/list
pub async fn list_files(
    config: &Config,
    params: ListFilesRequest,
) -> SDKResult<ListFilesResponse> {
    // 使用DriveApi枚举生成API端点
    let api_endpoint = DriveApi::ListFiles;

    // 创建API请求
    let api_request: ApiRequest<ListFilesResponse> =
        ApiRequest::get(&api_endpoint.to_url())
            .body(serde_json::json!(&params));

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "获取文件清单")
}