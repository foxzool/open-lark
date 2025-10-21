use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::cloud_docs::*,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    impl_executable_builder_owned,
};

use super::list::Comment;

/// 批量获取评论请求
#[derive(Debug, Serialize, Default, Clone)]
pub struct BatchQueryCommentsRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 文档token
    #[serde(skip)]
    file_token: String,
    /// 文档类型：doc、docx、sheet、bitable
    #[serde(skip)]
    file_type: String,
    /// 评论ID列表
    comment_ids: Vec<String>,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    user_id_type: Option<String>,
}

impl BatchQueryCommentsRequest {
    pub fn builder() -> BatchQueryCommentsRequestBuilder {
        BatchQueryCommentsRequestBuilder::default()
    }

    pub fn new(
        file_token: impl ToString,
        file_type: impl ToString,
        comment_ids: Vec<String>,
    ) -> Self {
        Self {
            file_token: file_token.to_string(),
            file_type: file_type.to_string(),
            comment_ids,
            ..Default::default()
        }
    }
}

#[derive(Default)]
pub struct BatchQueryCommentsRequestBuilder {
    request: BatchQueryCommentsRequest,
}

impl BatchQueryCommentsRequestBuilder {
    /// 文档token
    pub fn file_token(mut self, file_token: impl ToString) -> Self {
        self.request.file_token = file_token.to_string();
        self
    }

    /// 文档类型
    pub fn file_type(mut self, file_type: impl ToString) -> Self {
        self.request.file_type = file_type.to_string();
        self
    }

    /// 设置为文档类型
    pub fn with_doc_type(mut self) -> Self {
        self.request.file_type = "doc".to_string();
        self
    }

    /// 设置为docx类型
    pub fn with_docx_type(mut self) -> Self {
        self.request.file_type = "docx".to_string();
        self
    }

    /// 设置为电子表格类型
    pub fn with_sheet_type(mut self) -> Self {
        self.request.file_type = "sheet".to_string();
        self
    }

    /// 设置为多维表格类型
    pub fn with_bitable_type(mut self) -> Self {
        self.request.file_type = "bitable".to_string();
        self
    }

    /// 评论ID列表
    pub fn comment_ids(mut self, comment_ids: Vec<String>) -> Self {
        self.request.comment_ids = comment_ids;
        self
    }

    /// 添加单个评论ID
    pub fn add_comment_id(mut self, comment_id: impl ToString) -> Self {
        self.request.comment_ids.push(comment_id.to_string());
        self
    }

    /// 批量添加评论ID
    pub fn add_comment_ids(mut self, comment_ids: Vec<impl ToString>) -> Self {
        for comment_id in comment_ids {
            self.request.comment_ids.push(comment_id.to_string());
        }
        self
    }

    /// 用户ID类型
    pub fn user_id_type(mut self, user_id_type: impl ToString) -> Self {
        self.request.user_id_type = Some(user_id_type.to_string());
        self
    }

    /// 使用OpenID
    pub fn with_open_id(mut self) -> Self {
        self.request.user_id_type = Some("open_id".to_string());
        self
    }

    /// 使用UserID
    pub fn with_user_id(mut self) -> Self {
        self.request.user_id_type = Some("user_id".to_string());
        self
    }

    /// 使用UnionID
    pub fn with_union_id(mut self) -> Self {
        self.request.user_id_type = Some("union_id".to_string());
        self
    }

    pub fn build(mut self) -> BatchQueryCommentsRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

// 应用ExecutableBuilder trait到BatchQueryCommentsRequestBuilder
impl_executable_builder_owned!(
    BatchQueryCommentsRequestBuilder,
    super::CommentsService,
    BatchQueryCommentsRequest,
    BaseResponse<BatchQueryCommentsResponse>,
    batch_query
);

/// 批量获取评论响应
#[derive(Debug, Deserialize)]
pub struct BatchQueryCommentsResponse {
    /// 评论列表
    pub items: Vec<Comment>,
}

impl ApiResponseTrait for BatchQueryCommentsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量获取评论
pub async fn batch_query_comments(
    request: BatchQueryCommentsRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<BatchQueryCommentsResponse>> {
    let mut api_req = request.api_request;
    api_req.http_method = Method::POST;
    api_req.api_path = format!(
        "{}?file_type={}&file_token={}",
        COMMENT_V1_COMMENTS_BATCH_QUERY, request.file_type, request.file_token
    );

    // 添加用户ID类型查询参数
    if let Some(user_id_type) = request.user_id_type {
        api_req.api_path = format!("{}&user_id_type={}", api_req.api_path, user_id_type);
    }

    api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

    let api_resp = Transport::request(api_req, config, option).await?;
    Ok(api_resp)
}

impl BatchQueryCommentsResponse {
    /// 获取评论数量
    pub fn count(&self) -> usize {
        self.items.len()
    }

    /// 是否为空
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// 获取已解决的评论
    pub fn solved_comments(&self) -> Vec<&Comment> {
        self.items
            .iter()
            .filter(|comment| comment.is_solved)
            .collect()
    }

    /// 获取未解决的评论
    pub fn unsolved_comments(&self) -> Vec<&Comment> {
        self.items
            .iter()
            .filter(|comment| !comment.is_solved)
            .collect()
    }

    /// 获取全文评论
    pub fn whole_comments(&self) -> Vec<&Comment> {
        self.items
            .iter()
            .filter(|comment| comment.is_whole.unwrap_or(false))
            .collect()
    }

    /// 获取非全文评论
    pub fn non_whole_comments(&self) -> Vec<&Comment> {
        self.items
            .iter()
            .filter(|comment| !comment.is_whole.unwrap_or(false))
            .collect()
    }

    /// 根据用户ID筛选评论
    pub fn comments_by_user(&self, user_id: &str) -> Vec<&Comment> {
        self.items
            .iter()
            .filter(|comment| comment.user_id == user_id)
            .collect()
    }

    /// 获取有回复的评论
    pub fn comments_with_replies(&self) -> Vec<&Comment> {
        self.items
            .iter()
            .filter(|comment| comment.has_replies())
            .collect()
    }
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_query_comments_request_builder() {
        let request = BatchQueryCommentsRequest::builder()
            .file_token("doccnxxxxxx")
            .with_doc_type()
            .add_comment_id("comment1")
            .add_comment_id("comment2")
            .add_comment_ids(vec!["comment3", "comment4"])
            .with_open_id()
            .build();

        assert_eq!(request.file_token, "doccnxxxxxx");
        assert_eq!(request.file_type, "doc");
        assert_eq!(request.comment_ids.len(), 4);
        assert_eq!(request.comment_ids[0], "comment1");
        assert_eq!(request.comment_ids[3], "comment4");
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
    }
}
