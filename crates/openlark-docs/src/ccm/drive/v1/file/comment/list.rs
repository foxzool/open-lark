//! 获取云文档所有评论

//!

//! docPath: https://open.feishu.cn/document/server-docs/docs/CommentAPI/list

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

use super::models::Comment;

/// 获取评论列表请求

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct ListCommentsRequest {
    /// 文件 token
    pub file_token: String,

    /// 文件类型（必填）
    pub file_type: String,

    /// 是否是全文评论

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_whole: Option<bool>,

    /// 是否已解决

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_solved: Option<bool>,

    /// 页大小（1~100）

    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,

    /// 分页标记

    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,

    /// 用户 ID 类型（默认 open_id）

    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

impl ListCommentsRequest {
    pub fn new(file_token: impl Into<String>, file_type: impl Into<String>) -> Self {
        Self {
            file_token: file_token.into(),

            file_type: file_type.into(),

            is_whole: None,

            is_solved: None,

            page_size: None,

            page_token: None,

            user_id_type: None,
        }
    }

    pub fn is_whole(mut self, is_whole: bool) -> Self {
        self.is_whole = Some(is_whole);

        self
    }

    pub fn is_solved(mut self, is_solved: bool) -> Self {
        self.is_solved = Some(is_solved);

        self
    }

    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);

        self
    }

    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());

        self
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());

        self
    }
}

/// 获取评论列表响应（data）

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct ListCommentsResponse {
    #[serde(default)]
    pub has_more: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,

    #[serde(default)]
    pub items: Vec<Comment>,
}

impl ApiResponseTrait for ListCommentsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取云文档所有评论
pub async fn list_comments(
    request: ListCommentsRequest,

    config: &Config,

    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<ListCommentsResponse> {
    if request.file_token.trim().is_empty() {
        return Err(openlark_core::error::validation_error(
            "file_token",
            "file_token 不能为空",
        ));
    }

    if request.file_type.trim().is_empty() {
        return Err(openlark_core::error::validation_error(
            "file_type",
            "file_type 不能为空",
        ));
    }

    super::validate_comment_file_type_for_list_like(&request.file_type)?;

    let api_endpoint = DriveApi::ListFileComments(request.file_token.clone());

    let mut api_request: ApiRequest<ListCommentsResponse> = ApiRequest::get(&api_endpoint.to_url());

    api_request = api_request.query("file_type", &request.file_type);

    if let Some(is_whole) = request.is_whole {
        api_request = api_request.query("is_whole", &is_whole.to_string());
    }

    if let Some(is_solved) = request.is_solved {
        api_request = api_request.query("is_solved", &is_solved.to_string());
    }

    if let Some(page_size) = request.page_size {
        if !(1..=100).contains(&page_size) {
            return Err(openlark_core::error::validation_error(
                "page_size",
                "page_size 必须在 1~100 之间",
            ));
        }

        api_request = api_request.query("page_size", &page_size.to_string());
    }

    if let Some(page_token) = &request.page_token {
        api_request = api_request.query("page_token", page_token);
    }

    if let Some(user_id_type) = &request.user_id_type {
        api_request = api_request.query("user_id_type", user_id_type);
    }

    let response = Transport::request(api_request, config, option).await?;

    extract_response_data(response, "获取云文档所有评论")
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]

    fn test_list_comments_request_builder() {
        let request = ListCommentsRequest::new("file_token", "docx");

        assert_eq!(request.file_token, "file_token");

        assert_eq!(request.file_type, "docx");

        assert!(request.page_size.is_none());
    }

    #[test]

    fn test_list_comments_request_with_params() {
        let request = ListCommentsRequest::new("file_token", "docx")
            .is_whole(true)
            .is_solved(false)
            .page_size(20)
            .page_token("next_page_token")
            .user_id_type("open_id");

        assert_eq!(request.file_token, "file_token");

        assert_eq!(
            request
                .page_size
                .expect("page_size should be set when .page_size() is called"),
            20
        );

        assert_eq!(
            request
                .page_token
                .expect("page_token should be set when .page_token() is called"),
            "next_page_token"
        );

        assert_eq!(
            request
                .user_id_type
                .expect("user_id_type should be set when .user_id_type() is called"),
            "open_id"
        );

        assert!(request
            .is_whole
            .expect("is_whole should be set when .is_whole() is called"));

        assert!(!request
            .is_solved
            .expect("is_solved should be set when .is_solved() is called"));
    }

    #[test]

    fn test_response_trait() {
        assert_eq!(ListCommentsResponse::data_format(), ResponseFormat::Data);
    }
}
