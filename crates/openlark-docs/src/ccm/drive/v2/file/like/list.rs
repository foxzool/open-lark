use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
/// 获取云文档的点赞者列表
///
/// 获取文件或文档的点赞者列表
/// docPath: https://open.feishu.cn/document/docs/drive-v1/like/list
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::DriveApi;

/// 获取点赞者列表请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListFileLikesRequest {
    /// 文件token
    pub file_token: String,
    /// 页码，从0开始
    pub page_size: Option<i32>,
    /// 分页标记
    pub page_token: Option<String>,
}

impl ListFileLikesRequest {
    /// 创建获取点赞者列表请求
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

    /// 设置页面大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }
}

/// 点赞者信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LikeInfo {
    /// 用户ID
    pub user_id: String,
    /// 用户名称
    pub name: String,
    /// 头像URL
    pub avatar: Option<String>,
    /// 点赞时间
    pub like_time: i64,
}

/// 获取点赞者列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListFileLikesResponse {
    /// 点赞者列表数据
    pub data: Option<FileLikesData>,
}

/// 文件点赞数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileLikesData {
    /// 点赞者列表
    pub items: Option<Vec<LikeInfo>>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 是否有更多
    pub has_more: Option<bool>,
    /// 总点赞数
    pub total_count: Option<i32>,
}

impl ApiResponseTrait for ListFileLikesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取云文档的点赞者列表
///
/// 获取文件或文档的点赞者列表
/// docPath: https://open.feishu.cn/document/docs/drive-v1/like/list
pub async fn list_file_likes(
    request: ListFileLikesRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<ListFileLikesResponse>> {
    // 使用DriveApi枚举生成API端点
    let api_endpoint = DriveApi::ListFileLikes(request.file_token.clone());

    // 创建API请求
    let mut api_request: ApiRequest<ListFileLikesResponse> =
        ApiRequest::get(&api_endpoint.to_url());

    // 添加查询参数
    if let Some(page_size) = request.page_size {
        api_request = api_request.query("page_size", &page_size.to_string());
    }
    if let Some(page_token) = &request.page_token {
        api_request = api_request.query("page_token", page_token);
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
    fn test_list_file_likes_request_builder() {
        let request = ListFileLikesRequest::new("file_token");

        assert_eq!(request.file_token, "file_token");
        assert!(request.page_size.is_none());
    }

    #[test]
    fn test_list_file_likes_request_with_params() {
        let request = ListFileLikesRequest::new("file_token")
            .page_size(20)
            .page_token("next_page_token");

        assert_eq!(request.file_token, "file_token");
        assert_eq!(request.page_size.unwrap(), 20);
        assert_eq!(request.page_token.unwrap(), "next_page_token");
    }

    #[test]
    fn test_like_info_structure() {
        let like = LikeInfo {
            user_id: "user_id".to_string(),
            name: "用户名".to_string(),
            avatar: Some("https://example.com/avatar.jpg".to_string()),
            like_time: 1640995200,
        };

        assert_eq!(like.user_id, "user_id");
        assert_eq!(like.name, "用户名");
        assert_eq!(like.avatar.unwrap(), "https://example.com/avatar.jpg");
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(
            ListFileLikesResponse::data_format(),
            ResponseFormat::Data
        );
    }
}