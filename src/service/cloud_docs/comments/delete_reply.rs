use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::core::{
    api_req::ApiRequest,
    api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};

/// 删除回复请求
#[derive(Debug, Serialize, Default)]
pub struct DeleteReplyRequest {
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
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    user_id_type: Option<String>,
}

impl DeleteReplyRequest {
    pub fn builder() -> DeleteReplyRequestBuilder {
        DeleteReplyRequestBuilder::default()
    }

    pub fn new(
        file_token: impl ToString,
        file_type: impl ToString,
        comment_id: impl ToString,
        reply_id: impl ToString,
    ) -> Self {
        Self {
            file_token: file_token.to_string(),
            file_type: file_type.to_string(),
            comment_id: comment_id.to_string(),
            reply_id: reply_id.to_string(),
            ..Default::default()
        }
    }
}

#[derive(Default)]
pub struct DeleteReplyRequestBuilder {
    request: DeleteReplyRequest,
}

impl DeleteReplyRequestBuilder {
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
    pub fn as_doc(mut self) -> Self {
        self.request.file_type = "doc".to_string();
        self
    }

    /// 设置为docx类型
    pub fn as_docx(mut self) -> Self {
        self.request.file_type = "docx".to_string();
        self
    }

    /// 设置为电子表格类型
    pub fn as_sheet(mut self) -> Self {
        self.request.file_type = "sheet".to_string();
        self
    }

    /// 设置为多维表格类型
    pub fn as_bitable(mut self) -> Self {
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

    pub fn build(mut self) -> DeleteReplyRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }

    /// 执行请求
    pub async fn execute(
        mut self,
        service: &crate::service::cloud_docs::comments::CommentsService,
    ) -> SDKResult<BaseResponse<DeleteReplyResponse>> {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        service.delete_reply(self.request, None).await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        mut self,
        service: &crate::service::cloud_docs::comments::CommentsService,
        option: RequestOption,
    ) -> SDKResult<BaseResponse<DeleteReplyResponse>> {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        service.delete_reply(self.request, Some(option)).await
    }
}

/// 删除的回复信息
#[derive(Debug, Deserialize)]
pub struct DeletedReply {
    /// 回复ID
    pub reply_id: String,
    /// 评论ID
    pub comment_id: String,
    /// 删除时间（毫秒时间戳）
    pub delete_time: Option<i64>,
    /// 删除者用户ID
    pub deleter_user_id: Option<String>,
}

/// 删除回复响应
#[derive(Debug, Deserialize)]
pub struct DeleteReplyResponse {
    /// 删除的回复信息
    pub reply: DeletedReply,
}

impl ApiResponseTrait for DeleteReplyResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除回复
pub async fn delete_reply(
    request: DeleteReplyRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<DeleteReplyResponse>> {
    let mut api_req = request.api_request;
    api_req.http_method = Method::DELETE;
    api_req.api_path = format!(
        "/open-apis/comment/v1/comments/{}/replies/{}?file_type={}&file_token={}",
        request.comment_id, request.reply_id, request.file_type, request.file_token
    );

    // 添加用户ID类型查询参数
    if let Some(user_id_type) = request.user_id_type {
        api_req.api_path = format!("{}&user_id_type={}", api_req.api_path, user_id_type);
    }

    api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

    let api_resp = Transport::request(api_req, config, option).await?;
    Ok(api_resp)
}

impl DeletedReply {
    /// 是否有删除时间
    pub fn has_delete_time(&self) -> bool {
        self.delete_time.is_some()
    }

    /// 是否有删除者信息
    pub fn has_deleter(&self) -> bool {
        self.deleter_user_id.is_some()
    }

    /// 获取删除时间的格式化字符串
    pub fn delete_time_formatted(&self) -> Option<String> {
        self.delete_time
            .map(|timestamp| format!("删除时间: {}", timestamp))
    }

    /// 获取删除摘要信息
    pub fn summary(&self) -> String {
        let delete_info = if let Some(time) = self.delete_time {
            format!("删除时间: {}", time)
        } else {
            "删除时间: 未知".to_string()
        };

        let deleter_info = if let Some(deleter) = &self.deleter_user_id {
            format!("删除者: {}", deleter)
        } else {
            "删除者: 未知".to_string()
        };

        format!(
            "回复ID: {}, 评论ID: {}, {}, {}",
            self.reply_id, self.comment_id, delete_info, deleter_info
        )
    }
}

impl DeleteReplyResponse {
    /// 获取回复ID
    pub fn reply_id(&self) -> &str {
        &self.reply.reply_id
    }

    /// 获取评论ID
    pub fn comment_id(&self) -> &str {
        &self.reply.comment_id
    }

    /// 获取删除时间
    pub fn delete_time(&self) -> Option<i64> {
        self.reply.delete_time
    }

    /// 获取删除者用户ID
    pub fn deleter_user_id(&self) -> Option<&str> {
        self.reply.deleter_user_id.as_deref()
    }

    /// 是否成功删除
    pub fn is_deleted(&self) -> bool {
        // 如果有回复信息返回，说明删除操作已执行
        true
    }

    /// 获取删除成功的摘要信息
    pub fn success_summary(&self) -> String {
        format!(
            "回复删除成功 - 回复ID: {}, 评论ID: {}",
            self.reply_id(),
            self.comment_id()
        )
    }

    /// 获取详细的删除信息
    pub fn detailed_info(&self) -> String {
        self.reply.summary()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_reply_request_builder() {
        let request = DeleteReplyRequest::builder()
            .file_token("doccnxxxxxx")
            .as_doc()
            .comment_id("comment123")
            .reply_id("reply456")
            .with_open_id()
            .build();

        assert_eq!(request.file_token, "doccnxxxxxx");
        assert_eq!(request.file_type, "doc");
        assert_eq!(request.comment_id, "comment123");
        assert_eq!(request.reply_id, "reply456");
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
    }

    #[test]
    fn test_delete_reply_new() {
        let request = DeleteReplyRequest::new("doccnxxxxxx", "doc", "comment123", "reply456");
        assert_eq!(request.file_token, "doccnxxxxxx");
        assert_eq!(request.file_type, "doc");
        assert_eq!(request.comment_id, "comment123");
        assert_eq!(request.reply_id, "reply456");
    }
}
