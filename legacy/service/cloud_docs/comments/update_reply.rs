use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::{cloud_docs::*, EndpointBuilder},
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    impl_executable_builder_owned,
};

use super::list::ReplyContent;

/// 更新回复的内容请求
#[derive(Debug, Serialize, Default, Clone)]
pub struct UpdateReplyRequest {
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
    /// 回复ID
    #[serde(skip)]
    reply_id: String,
    /// 回复内容
    content: ReplyContent,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    user_id_type: Option<String>,
}

impl UpdateReplyRequest {
    pub fn builder() -> UpdateReplyRequestBuilder {
        UpdateReplyRequestBuilder::default()
    }

    pub fn new(
        file_token: impl ToString,
        file_type: impl ToString,
        comment_id: impl ToString,
        reply_id: impl ToString,
        content: ReplyContent,
    ) -> Self {
        Self {
            file_token: file_token.to_string(),
            file_type: file_type.to_string(),
            comment_id: comment_id.to_string(),
            reply_id: reply_id.to_string(),
            content,
            ..Default::default()
        }
    }
}

#[derive(Default)]
pub struct UpdateReplyRequestBuilder {
    request: UpdateReplyRequest,
}

impl UpdateReplyRequestBuilder {
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

    /// 回复ID
    pub fn reply_id(mut self, reply_id: impl ToString) -> Self {
        self.request.reply_id = reply_id.to_string();
        self
    }

    /// 回复内容
    pub fn content(mut self, content: ReplyContent) -> Self {
        self.request.content = content;
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

    pub fn build(mut self) -> UpdateReplyRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

/// 更新后的回复信息
#[derive(Debug, Deserialize)]
pub struct UpdatedReply {
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
}

// 应用ExecutableBuilder trait到UpdateReplyRequestBuilder
impl_executable_builder_owned!(
    UpdateReplyRequestBuilder,
    super::CommentsService,
    UpdateReplyRequest,
    BaseResponse<UpdateReplyResponse>,
    update_reply
);

/// 更新回复的内容响应
#[derive(Debug, Deserialize)]
pub struct UpdateReplyResponse {
    /// 更新后的回复信息
    pub reply: UpdatedReply,
}

impl ApiResponseTrait for UpdateReplyResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新回复的内容
pub async fn update_reply(
    request: UpdateReplyRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<UpdateReplyResponse>> {
    let mut api_req = request.api_request;
    api_req.http_method = Method::PUT;
    api_req.api_path = format!(
        "{}?file_type={}&file_token={}",
        EndpointBuilder::replace_params_from_array(
            COMMENT_V1_COMMENT_REPLY_UPDATE,
            &[
                ("comment_id", &request.comment_id),
                ("reply_id", &request.reply_id)
            ]
        ),
        request.file_type,
        request.file_token
    );

    // 添加用户ID类型查询参数
    if let Some(user_id_type) = request.user_id_type {
        api_req.api_path = format!("{}&user_id_type={}", api_req.api_path, user_id_type);
    }

    api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

    let api_resp = Transport::request(api_req, config, option).await?;
    Ok(api_resp)
}

impl UpdatedReply {
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

    /// 是否已更新
    pub fn is_updated(&self) -> bool {
        self.update_time > self.create_time
    }

    /// 获取更新时间与创建时间的差值（毫秒）
    pub fn time_since_creation(&self) -> i64 {
        self.update_time - self.create_time
    }

    /// 获取更新时间的格式化字符串
    pub fn updated_at_formatted(&self) -> String {
        format!("更新时间: {}", self.update_time)
    }

    /// 获取回复摘要信息
    pub fn summary(&self) -> String {
        format!(
            "回复ID: {}, 用户: {}, 内容: {}, 更新时间: {}",
            self.reply_id,
            self.user_id,
            self.get_text_content(),
            self.update_time
        )
    }
}

impl UpdateReplyResponse {
    /// 获取回复ID
    pub fn reply_id(&self) -> &str {
        &self.reply.reply_id
    }

    /// 获取用户ID
    pub fn user_id(&self) -> &str {
        &self.reply.user_id
    }

    /// 获取回复的文本内容
    pub fn get_text_content(&self) -> String {
        self.reply.get_text_content()
    }

    /// 是否已更新
    pub fn is_updated(&self) -> bool {
        self.reply.is_updated()
    }

    /// 获取创建时间
    pub fn create_time(&self) -> i64 {
        self.reply.create_time
    }

    /// 获取更新时间
    pub fn update_time(&self) -> i64 {
        self.reply.update_time
    }

    /// 获取更新摘要
    pub fn update_summary(&self) -> String {
        format!(
            "回复更新成功 - ID: {}, 新内容: \"{}\"",
            self.reply_id(),
            self.get_text_content()
        )
    }
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
    use crate::service::comments::create::ContentBuilder;

    #[test]
    fn test_update_reply_request_builder() {
        let content = ContentBuilder::new().add_text("更新后的回复内容").build();

        let request = UpdateReplyRequest::builder()
            .file_token("doccnxxxxxx")
            .with_doc_type()
            .comment_id("comment123")
            .reply_id("reply456")
            .content(content)
            .with_open_id()
            .build();

        assert_eq!(request.file_token, "doccnxxxxxx");
        assert_eq!(request.file_type, "doc");
        assert_eq!(request.comment_id, "comment123");
        assert_eq!(request.reply_id, "reply456");
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
    }
}
