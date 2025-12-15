/// 搜索文件
///
/// 根据关键词和过滤条件搜索文件。
/// docPath: https://open.feishu.cn/open-apis/drive/v1/files/search
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::common::{api_endpoints::DriveApi, api_utils::*};

/// 搜索文件请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchFilesRequest {
    /// 搜索关键词
    pub query: String,
    /// 搜索范围
    pub search_type: Option<String>,
    /// 文件类型过滤
    pub file_type: Option<String>,
    /// 父目录token
    pub parent_token: Option<String>,
    /// 创建者ID
    pub creator_id: Option<String>,
    /// 开始时间
    pub start_time: Option<String>,
    /// 结束时间
    pub end_time: Option<String>,
    /// 分页大小
    pub page_size: Option<u32>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 排序字段
    pub order_by: Option<String>,
    /// 排序方向
    pub direction: Option<String>,
}

impl SearchFilesRequest {
    /// 创建搜索文件请求
    ///
    /// # 参数
    /// * `query` - 搜索关键词
    pub fn new(query: impl Into<String>) -> Self {
        Self {
            query: query.into(),
            search_type: None,
            file_type: None,
            parent_token: None,
            creator_id: None,
            start_time: None,
            end_time: None,
            page_size: None,
            page_token: None,
            order_by: None,
            direction: None,
        }
    }

    // Builder 方法
    pub fn search_type(mut self, search_type: impl Into<String>) -> Self { self.search_type = Some(search_type.into()); self }
    pub fn file_type(mut self, file_type: impl Into<String>) -> Self { self.file_type = Some(file_type.into()); self }
    pub fn parent_token(mut self, parent_token: impl Into<String>) -> Self { self.parent_token = Some(parent_token.into()); self }
    pub fn creator_id(mut self, creator_id: impl Into<String>) -> Self { self.creator_id = Some(creator_id.into()); self }
    pub fn start_time(mut self, start_time: impl Into<String>) -> Self { self.start_time = Some(start_time.into()); self }
    pub fn end_time(mut self, end_time: impl Into<String>) -> Self { self.end_time = Some(end_time.into()); self }
    pub fn page_size(mut self, page_size: u32) -> Self { self.page_size = Some(page_size); self }
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self { self.page_token = Some(page_token.into()); self }
    pub fn order_by(mut self, order_by: impl Into<String>) -> Self { self.order_by = Some(order_by.into()); self }
    pub fn direction(mut self, direction: impl Into<String>) -> Self { self.direction = Some(direction.into()); self }
}

/// 搜索文件响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchFilesResponse {
    /// 搜索结果
    pub data: Option<SearchResultData>,
}

impl ApiResponseTrait for SearchFilesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 搜索结果数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResultData {
    /// 文件列表
    pub files: Vec<FileInfo>,
    /// 是否有更多数据
    pub has_more: bool,
    /// 分页标记
    pub page_token: Option<String>,
}

/// 文件信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    /// 文件token
    pub file_token: String,
    /// 文件名
    pub name: String,
    /// 文件类型
    pub r#type: String,
    /// 文件大小
    pub size: u64,
    /// 创建时间
    pub created_at: String,
    /// 修改时间
    pub modified_at: String,
    /// 创建者信息
    pub creator: CreatorInfo,
    /// 父目录token
    pub parent_token: String,
    /// 文件URL
    pub url: Option<String>,
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

/// 搜索文件
///
/// 根据关键词和过滤条件搜索文件。
/// docPath: https://open.feishu.cn/open-apis/drive/v1/files/search
pub async fn search_files(
    request: SearchFilesRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<SearchFilesResponse>> {
    // 创建API请求
    let url = "/open-apis/drive/v1/files/search".to_string();
    let mut api_request: ApiRequest<SearchFilesResponse> =
        ApiRequest::get(&url)
            .query_param("query", &request.query);

    // 添加查询参数
    if let Some(search_type) = &request.search_type {
        api_request = api_request.query_param("search_type", search_type);
    }
    if let Some(file_type) = &request.file_type {
        api_request = api_request.query_param("file_type", file_type);
    }
    if let Some(parent_token) = &request.parent_token {
        api_request = api_request.query_param("parent_token", parent_token);
    }
    if let Some(creator_id) = &request.creator_id {
        api_request = api_request.query_param("creator_id", creator_id);
    }
    if let Some(start_time) = &request.start_time {
        api_request = api_request.query_param("start_time", start_time);
    }
    if let Some(end_time) = &request.end_time {
        api_request = api_request.query_param("end_time", end_time);
    }
    if let Some(page_size) = request.page_size {
        api_request = api_request.query_param("page_size", &page_size.to_string());
    }
    if let Some(page_token) = &request.page_token {
        api_request = api_request.query_param("page_token", page_token);
    }
    if let Some(order_by) = &request.order_by {
        api_request = api_request.query_param("order_by", order_by);
    }
    if let Some(direction) = &request.direction {
        api_request = api_request.query_param("direction", direction);
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
    fn test_search_files_request_builder() {
        let request = SearchFilesRequest::new("测试文档")
            .search_type("file")
            .file_type("doc")
            .page_size(20)
            .order_by("created_at")
            .direction("desc");

        assert_eq!(request.query, "测试文档");
        assert_eq!(request.search_type, Some("file".to_string()));
        assert_eq!(request.file_type, Some("doc".to_string()));
        assert_eq!(request.page_size, Some(20));
    }

    #[test]
    fn test_search_result_data_structure() {
        let files = vec![
            FileInfo {
                file_token: "file_123".to_string(),
                name: "测试文档.doc".to_string(),
                r#type: "doc".to_string(),
                size: 1024,
                created_at: "2023-01-01T00:00:00Z".to_string(),
                modified_at: "2023-01-02T00:00:00Z".to_string(),
                creator: CreatorInfo {
                    user_id: "user_456".to_string(),
                    name: "张三".to_string(),
                    avatar: None,
                },
                parent_token: "folder_789".to_string(),
                url: Some("https://example.com/file".to_string()),
            }
        ];

        let data = SearchResultData {
            files: files.clone(),
            has_more: true,
            page_token: Some("next_token".to_string()),
        };

        assert_eq!(data.files.len(), 1);
        assert_eq!(data.has_more, true);
        assert_eq!(data.files[0].name, "测试文档.doc");
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(SearchFilesResponse::data_format(), ResponseFormat::Data);
    }
}