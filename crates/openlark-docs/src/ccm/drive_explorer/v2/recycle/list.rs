/// 获取回收站列表
///
/// 获取云盘回收站中的文件列表，支持分页和过滤。
/// docPath: https://open.feishu.cn/open-apis/drive/explorer/v2/recycle/list
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::CcmDriveExplorerApiOld, api_utils::*};

/// 获取回收站列表请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRecycleListRequest {
    /// 分页大小
    pub page_size: Option<u32>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 文件类型过滤
    pub file_type: Option<String>,
}

impl GetRecycleListRequest {
    /// 创建获取回收站列表请求
    pub fn new() -> Self {
        Self {
            page_size: None,
            page_token: None,
            file_type: None,
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

    /// 设置文件类型过滤
    pub fn file_type(mut self, file_type: impl Into<String>) -> Self {
        self.file_type = Some(file_type.into());
        self
    }
}

/// 获取回收站列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRecycleListResponse {
    /// 回收站数据
    pub data: Option<RecycleData>,
}

impl ApiResponseTrait for GetRecycleListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 回收站数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecycleData {
    /// 文件列表
    pub files: Vec<FileMeta>,
    /// 是否有更多数据
    pub has_more: bool,
    /// 分页标记
    pub page_token: Option<String>,
}

/// 文件元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMeta {
    /// 文件token
    pub file_token: String,
    /// 文件名称
    pub name: String,
    /// 文件类型
    pub r#type: String,
    /// 创建时间
    pub create_time: String,
    /// 修改时间
    pub update_time: String,
    /// 删除时间
    pub delete_time: String,
    /// 创建者信息
    pub creator: CreatorInfo,
}

/// 创建者信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatorInfo {
    /// 用户ID
    pub user_id: String,
    /// 用户名
    pub name: String,
}

/// 获取回收站列表
///
/// 获取云盘回收站中的文件列表，支持分页和过滤。
/// docPath: https://open.feishu.cn/open-apis/drive/explorer/v2/recycle/list
pub async fn get_recycle_list(
    request: GetRecycleListRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<GetRecycleListResponse>> {
    // 使用CcmDriveExplorerApiOld枚举生成API端点
    let api_endpoint = CcmDriveExplorerApiOld::FolderChildren("recycle".to_string());

    // 构建查询参数
    let mut api_request: ApiRequest<GetRecycleListResponse> =
        ApiRequest::get(&api_endpoint.to_url());

    // 添加查询参数
    if let Some(page_size) = request.page_size {
        api_request = api_request.query("page_size", &page_size.to_string());
    }

    if let Some(page_token) = &request.page_token {
        api_request = api_request.query("page_token", page_token);
    }

    if let Some(file_type) = &request.file_type {
        api_request = api_request.query("file_type", file_type);
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
    fn test_get_recycle_list_request_builder() {
        let request = GetRecycleListRequest::new();

        assert!(request.page_size.is_none());
        assert!(request.page_token.is_none());
        assert!(request.file_type.is_none());
    }

    #[test]
    fn test_get_recycle_list_request_with_params() {
        let request = GetRecycleListRequest::new()
            .page_size(20)
            .page_token("token123")
            .file_type("folder");

        assert_eq!(request.page_size, Some(20));
        assert_eq!(request.page_token, Some("token123".to_string()));
        assert_eq!(request.file_type, Some("folder".to_string()));
    }

    #[test]
    fn test_file_meta_structure() {
        let creator = CreatorInfo {
            user_id: "user_id".to_string(),
            name: "用户名".to_string(),
        };

        let file_meta = FileMeta {
            file_token: "file_token".to_string(),
            name: "文件名".to_string(),
            r#type: "doc".to_string(),
            create_time: "2023-01-01T00:00:00Z".to_string(),
            update_time: "2023-01-02T00:00:00Z".to_string(),
            delete_time: "2023-01-03T00:00:00Z".to_string(),
            creator: creator.clone(),
        };

        assert_eq!(file_meta.file_token, "file_token");
        assert_eq!(file_meta.name, "文件名");
        assert_eq!(file_meta.r#type, "doc");
        assert_eq!(file_meta.creator.name, "用户名");
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(GetRecycleListResponse::data_format(), ResponseFormat::Data);
    }
}