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

/// 获取云文档所有评论请求
#[derive(Debug, Serialize, Default, Clone)]
pub struct ListCommentsRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 文档token
    #[serde(skip)]
    file_token: String,
    /// 文档类型：doc、docx、sheet、bitable
    #[serde(skip)]
    file_type: String,
    /// 是否是全文评论，不传该参数则返回全部评论
    #[serde(skip_serializing_if = "Option::is_none")]
    is_whole: Option<bool>,
    /// 是否获取已解决的评论
    #[serde(skip_serializing_if = "Option::is_none")]
    is_solved: Option<bool>,
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

impl ListCommentsRequest {
    pub fn builder() -> ListCommentsRequestBuilder {
        ListCommentsRequestBuilder::default()
    }

    pub fn new(file_token: impl ToString, file_type: impl ToString) -> Self {
        Self {
            file_token: file_token.to_string(),
            file_type: file_type.to_string(),
            ..Default::default()
        }
    }
}

#[derive(Default)]
pub struct ListCommentsRequestBuilder {
    request: ListCommentsRequest,
}

impl ListCommentsRequestBuilder {
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

    /// 是否是全文评论
    pub fn set_whole(mut self, is_whole: bool) -> Self {
        self.request.is_whole = Some(is_whole);
        self
    }

    /// 只获取全文评论
    pub fn whole_comments_only(mut self) -> Self {
        self.request.is_whole = Some(true);
        self
    }

    /// 获取所有类型评论
    pub fn all_comment_types(mut self) -> Self {
        self.request.is_whole = None;
        self
    }

    /// 是否获取已解决的评论
    pub fn set_solved(mut self, is_solved: bool) -> Self {
        self.request.is_solved = Some(is_solved);
        self
    }

    /// 只获取已解决的评论
    pub fn solved_comments_only(mut self) -> Self {
        self.request.is_solved = Some(true);
        self
    }

    /// 只获取未解决的评论
    pub fn unsolved_comments_only(mut self) -> Self {
        self.request.is_solved = Some(false);
        self
    }

    /// 获取所有评论（无论是否解决）
    pub fn all_comments(mut self) -> Self {
        self.request.is_solved = None;
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

    pub fn build(mut self) -> ListCommentsRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

// 应用ExecutableBuilder trait到ListCommentsRequestBuilder
impl_executable_builder_owned!(
    ListCommentsRequestBuilder,
    super::CommentsService,
    ListCommentsRequest,
    BaseResponse<ListCommentsResponse>,
    list
);

/// 评论信息
#[derive(Debug, Deserialize)]
pub struct Comment {
    /// 评论ID
    pub comment_id: String,
    /// 用户ID
    pub user_id: String,
    /// 创建时间（毫秒时间戳）
    pub create_time: i64,
    /// 更新时间（毫秒时间戳）
    pub update_time: i64,
    /// 是否解决
    pub is_solved: bool,
    /// 已解决时间（毫秒时间戳）
    pub solved_time: Option<i64>,
    /// 解决者用户ID
    pub solver_user_id: Option<String>,
    /// 是否有更多回复
    pub has_more: bool,
    /// 分页标记
    pub page_token: Option<String>,
    /// 回复列表
    pub reply_list: Option<Vec<Reply>>,
    /// 是否是全文评论
    pub is_whole: Option<bool>,
    /// 引用内容
    pub quote: Option<String>,
}

/// 回复信息
#[derive(Debug, Deserialize)]
pub struct Reply {
    /// 回复ID
    pub reply_id: String,
    /// 用户ID
    pub user_id: String,
    /// 创建时间（毫秒时间戳）
    pub create_time: i64,
    /// 更新时间（毫秒时间戳）
    pub update_time: i64,
    /// 回复内容
    pub content: ReplyContent,
    /// 额外字段
    pub extra: Option<serde_json::Value>,
}

/// 回复内容
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct ReplyContent {
    /// 元素列表
    pub elements: Vec<ContentElement>,
}

/// 内容元素
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContentElement {
    /// 元素类型
    #[serde(rename = "type")]
    pub element_type: String,
    /// 文本内容
    pub text_run: Option<TextRun>,
}

/// 文本内容
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TextRun {
    /// 文本内容
    pub text: String,
    /// 样式
    pub style: Option<serde_json::Value>,
}

/// 获取云文档所有评论响应
#[derive(Debug, Deserialize)]
pub struct ListCommentsResponse {
    /// 评论列表
    pub items: Vec<Comment>,
    /// 是否还有更多项
    pub has_more: bool,
    /// 分页标记
    pub page_token: Option<String>,
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
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<ListCommentsResponse>> {
    let mut api_req = request.api_request;
    api_req.http_method = Method::GET;
    api_req.api_path = format!(
        "{}?file_type={}&file_token={}",
        COMMENT_V1_COMMENTS, request.file_type, request.file_token
    );

    // 构建查询参数
    let mut query_params = Vec::new();
    if let Some(is_whole) = request.is_whole {
        query_params.push(format!("is_whole={is_whole}"));
    }
    if let Some(is_solved) = request.is_solved {
        query_params.push(format!("is_solved={is_solved}"));
    }
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

impl Comment {
    /// 获取评论的文本内容
    pub fn get_text_content(&self) -> String {
        if let Some(replies) = &self.reply_list {
            replies
                .iter()
                .map(|reply| reply.get_text_content())
                .collect::<Vec<_>>()
                .join("\n")
        } else {
            String::new()
        }
    }

    /// 是否有回复
    pub fn has_replies(&self) -> bool {
        self.reply_list
            .as_ref()
            .is_some_and(|replies| !replies.is_empty())
    }

    /// 获取回复数量
    pub fn reply_count(&self) -> usize {
        self.reply_list.as_ref().map_or(0, |replies| replies.len())
    }
}

impl Reply {
    /// 获取回复的文本内容
    pub fn get_text_content(&self) -> String {
        self.content
            .elements
            .iter()
            .filter_map(|element| {
                element
                    .text_run
                    .as_ref()
                    .map(|text_run| text_run.text.clone())
            })
            .collect::<Vec<_>>()
            .join("")
    }
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;

    #[test]
    fn test_list_comments_request_builder() {
        let request = ListCommentsRequest::builder()
            .file_token("doccnxxxxxx")
            .with_doc_type()
            .whole_comments_only()
            .unsolved_comments_only()
            .page_size(20)
            .user_id_type("open_id")
            .build();

        assert_eq!(request.file_token, "doccnxxxxxx");
        assert_eq!(request.file_type, "doc");
        assert_eq!(request.is_whole, Some(true));
        assert_eq!(request.is_solved, Some(false));
        assert_eq!(request.page_size, Some(20));
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
    }
}
