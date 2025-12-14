use serde::{Deserialize, Serialize};
use openlark_core::{
    api:: ApiResponseTrait,
    models::{OpenLarkConfig, OpenLarkRequest},
    OpenLarkClient, SDKResult,
};

/// 搜索文件请求
#[derive(Debug, Serialize, Default)]
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

/// 搜索文件响应
#[derive(Debug, Deserialize, Default)]
pub struct SearchFilesResponse {
    /// 文件列表
    pub files: Vec<FileInfo>,
    /// 是否有更多数据
    pub has_more: bool,
    /// 分页标记
    pub page_token: Option<String>,
    /// 操作结果
    pub result: String,
}

/// 文件信息
#[derive(Debug, Deserialize, Default)]
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
#[derive(Debug, Deserialize, Default)]
pub struct CreatorInfo {
    /// 用户ID
    pub user_id: String,
    /// 用户名
    pub name: String,
    /// 头像
    pub avatar: Option<AvatarInfo>,
}

/// 头像信息
#[derive(Debug, Deserialize, Default)]
pub struct AvatarInfo {
    /// 头像URL
    pub url: String,
    /// 头像大小
    pub size: Option<u32>,
}

/// 搜索文件
/// docPath: https://open.feishu.cn/open-apis/drive/v1/files/search
pub async fn search_files(
    request: SearchFilesRequest,
    config: &OpenLarkConfig,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<SearchFilesResponse>> {
    let url = format!(
        "{}/open-apis/drive/v1/files/search",
        config.base_url
    );

    let mut query_params = vec![("query".to_string(), request.query)];

    if let Some(search_type) = &request.search_type {
        query_params.push(("search_type".to_string(), search_type.clone()));
    }

    if let Some(file_type) = &request.file_type {
        query_params.push(("file_type".to_string(), file_type.clone()));
    }

    if let Some(parent_token) = &request.parent_token {
        query_params.push(("parent_token".to_string(), parent_token.clone()));
    }

    if let Some(creator_id) = &request.creator_id {
        query_params.push(("creator_id".to_string(), creator_id.clone()));
    }

    if let Some(start_time) = &request.start_time {
        query_params.push(("start_time".to_string(), start_time.clone()));
    }

    if let Some(end_time) = &request.end_time {
        query_params.push(("end_time".to_string(), end_time.clone()));
    }

    if let Some(page_size) = request.page_size {
        query_params.push(("page_size".to_string(), page_size.to_string()));
    }

    if let Some(page_token) = &request.page_token {
        query_params.push(("page_token".to_string(), page_token.clone()));
    }

    if let Some(order_by) = &request.order_by {
        query_params.push(("order_by".to_string(), order_by.clone()));
    }

    if let Some(direction) = &request.direction {
        query_params.push(("direction".to_string(), direction.clone()));
    }

    let req = OpenLarkRequest {
        url,
        method: http::Method::GET,
        headers: vec![],
        query_params,
        body: None,
    };

    OpenLarkClient::request(req, config, option).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_search_files() {
        let config = OpenLarkConfig {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            base_url: "https://open.feishu.cn".to_string(),
            ..Default::default()
        };

        let request = SearchFilesRequest {
            query: "测试文档".to_string(),
            search_type: Some("file".to_string()),
            file_type: Some("doc".to_string()),
            parent_token: None,
            creator_id: None,
            start_time: None,
            end_time: None,
            page_size: Some(20),
            page_token: None,
            order_by: Some("created_at".to_string()),
            direction: Some("desc".to_string()),
        };

        let result = search_files(request, &config, None).await;
        assert!(result.is_ok());
    }
}