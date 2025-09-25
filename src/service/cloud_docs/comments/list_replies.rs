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

use super::list::Reply;

/// 获取回复信息请求
#[derive(Debug, Serialize, Default, Clone)]
pub struct ListRepliesRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 文档token
    #[serde(skip)]
    file_token: String,
    /// 文档类型：doc、docx、sheet、bitable
    #[serde(skip)]
    file_type: String,
    /// 评论ID
    #[serde(skip)]
    comment_id: String,
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    page_token: Option<String>,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    user_id_type: Option<String>,
}

impl ListRepliesRequest {
    pub fn builder() -> ListRepliesRequestBuilder {
        ListRepliesRequestBuilder::default()
    }

    pub fn new(
        file_token: impl ToString,
        file_type: impl ToString,
        comment_id: impl ToString,
    ) -> Self {
        Self {
            file_token: file_token.to_string(),
            file_type: file_type.to_string(),
            comment_id: comment_id.to_string(),
            ..Default::default()
        }
    }
}

#[derive(Default)]
pub struct ListRepliesRequestBuilder {
    request: ListRepliesRequest,
}

impl ListRepliesRequestBuilder {
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

    /// 评论ID
    pub fn comment_id(mut self, comment_id: impl ToString) -> Self {
        self.request.comment_id = comment_id.to_string();
        self
    }

    /// 分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request.page_size = Some(page_size);
        self
    }

    /// 分页标记
    pub fn page_token(mut self, page_token: impl ToString) -> Self {
        self.request.page_token = Some(page_token.to_string());
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

    pub fn build(mut self) -> ListRepliesRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

// 应用ExecutableBuilder trait到ListRepliesRequestBuilder
impl_executable_builder_owned!(
    ListRepliesRequestBuilder,
    super::CommentsService,
    ListRepliesRequest,
    BaseResponse<ListRepliesResponse>,
    list_replies
);

/// 获取回复信息响应
#[derive(Debug, Deserialize)]
pub struct ListRepliesResponse {
    /// 回复列表
    pub items: Vec<Reply>,
    /// 是否还有更多项
    pub has_more: bool,
    /// 分页标记
    pub page_token: Option<String>,
}

impl ApiResponseTrait for ListRepliesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取回复信息
pub async fn list_replies(
    request: ListRepliesRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<ListRepliesResponse>> {
    let mut api_req = request.api_request;
    api_req.http_method = Method::GET;
    api_req.api_path = format!(
        "{}?file_type={}&file_token={}",
        COMMENT_V1_COMMENT_REPLIES.replace("{}", &request.comment_id),
        request.file_type,
        request.file_token
    );

    // 构建查询参数
    let mut query_params = Vec::new();
    if let Some(page_size) = request.page_size {
        query_params.push(format!("page_size={page_size}"));
    }
    if let Some(page_token) = request.page_token {
        query_params.push(format!("page_token={page_token}"));
    }
    if let Some(user_id_type) = request.user_id_type {
        query_params.push(format!("user_id_type={user_id_type}"));
    }

    if !query_params.is_empty() {
        api_req.api_path = format!("{}&{}", api_req.api_path, query_params.join("&"));
    }

    api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

    let api_resp = Transport::request(api_req, config, option).await?;
    Ok(api_resp)
}

impl ListRepliesResponse {
    /// 获取回复数量
    pub fn count(&self) -> usize {
        self.items.len()
    }

    /// 是否为空
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// 根据用户ID筛选回复
    pub fn replies_by_user(&self, user_id: &str) -> Vec<&Reply> {
        self.items
            .iter()
            .filter(|reply| reply.user_id == user_id)
            .collect()
    }

    /// 获取最新的回复
    pub fn latest_reply(&self) -> Option<&Reply> {
        self.items.iter().max_by_key(|reply| reply.create_time)
    }

    /// 获取最早的回复
    pub fn earliest_reply(&self) -> Option<&Reply> {
        self.items.iter().min_by_key(|reply| reply.create_time)
    }

    /// 按创建时间排序的回复列表
    pub fn sorted_by_time(&self) -> Vec<&Reply> {
        let mut replies: Vec<&Reply> = self.items.iter().collect();
        replies.sort_by_key(|reply| reply.create_time);
        replies
    }

    /// 按创建时间倒序排序的回复列表
    pub fn sorted_by_time_desc(&self) -> Vec<&Reply> {
        let mut replies: Vec<&Reply> = self.items.iter().collect();
        replies.sort_by_key(|reply| std::cmp::Reverse(reply.create_time));
        replies
    }

    /// 获取所有回复的文本内容
    pub fn get_all_text_content(&self) -> Vec<String> {
        self.items
            .iter()
            .map(|reply| reply.get_text_content())
            .collect()
    }

    /// 获取回复摘要信息
    pub fn summary(&self) -> String {
        format!(
            "回复总数: {}, 是否有更多: {}, 最新回复时间: {}",
            self.count(),
            self.has_more,
            self.latest_reply()
                .map(|r| r.create_time.to_string())
                .unwrap_or_else(|| "无".to_string())
        )
    }
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;

    #[test]
    fn test_list_replies_request_builder() {
        let request = ListRepliesRequest::builder()
            .file_token("doccnxxxxxx")
            .with_doc_type()
            .comment_id("comment123")
            .page_size(20)
            .with_open_id()
            .build();

        assert_eq!(request.file_token, "doccnxxxxxx");
        assert_eq!(request.file_type, "doc");
        assert_eq!(request.comment_id, "comment123");
        assert_eq!(request.page_size, Some(20));
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
    }

    #[test]
    fn test_list_replies_new() {
        let request = ListRepliesRequest::new("doccnxxxxxx", "doc", "comment123");
        assert_eq!(request.file_token, "doccnxxxxxx");
        assert_eq!(request.file_type, "doc");
        assert_eq!(request.comment_id, "comment123");
    }
}
