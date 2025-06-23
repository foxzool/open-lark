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

/// 解决/恢复评论请求
#[derive(Debug, Serialize, Default)]
pub struct PatchCommentRequest {
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
    /// 是否解决
    is_solved: bool,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    user_id_type: Option<String>,
}

impl PatchCommentRequest {
    pub fn builder() -> PatchCommentRequestBuilder {
        PatchCommentRequestBuilder::default()
    }

    pub fn new(
        file_token: impl ToString,
        file_type: impl ToString,
        comment_id: impl ToString,
        is_solved: bool,
    ) -> Self {
        Self {
            file_token: file_token.to_string(),
            file_type: file_type.to_string(),
            comment_id: comment_id.to_string(),
            is_solved,
            ..Default::default()
        }
    }

    /// 创建解决评论的请求
    pub fn solve(
        file_token: impl ToString,
        file_type: impl ToString,
        comment_id: impl ToString,
    ) -> Self {
        Self::new(file_token, file_type, comment_id, true)
    }

    /// 创建恢复评论的请求
    pub fn restore(
        file_token: impl ToString,
        file_type: impl ToString,
        comment_id: impl ToString,
    ) -> Self {
        Self::new(file_token, file_type, comment_id, false)
    }
}

#[derive(Default)]
pub struct PatchCommentRequestBuilder {
    request: PatchCommentRequest,
}

impl PatchCommentRequestBuilder {
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

    /// 是否解决
    pub fn is_solved(mut self, is_solved: bool) -> Self {
        self.request.is_solved = is_solved;
        self
    }

    /// 解决评论
    pub fn solve_comment(mut self) -> Self {
        self.request.is_solved = true;
        self
    }

    /// 恢复评论
    pub fn restore_comment(mut self) -> Self {
        self.request.is_solved = false;
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

    pub fn build(mut self) -> PatchCommentRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }

    /// 执行请求
    pub async fn execute(
        mut self,
        service: &crate::service::cloud_docs::comments::CommentsService,
    ) -> SDKResult<BaseResponse<PatchCommentResponse>> {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        service.patch(self.request, None).await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        mut self,
        service: &crate::service::cloud_docs::comments::CommentsService,
        option: RequestOption,
    ) -> SDKResult<BaseResponse<PatchCommentResponse>> {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        service.patch(self.request, Some(option)).await
    }
}

/// 解决/恢复评论响应
#[derive(Debug, Deserialize)]
pub struct PatchCommentResponse {
    /// 评论ID
    pub comment_id: String,
    /// 是否解决
    pub is_solved: bool,
    /// 解决时间（毫秒时间戳）
    pub solved_time: Option<i64>,
    /// 解决者用户ID
    pub solver_user_id: Option<String>,
}

impl ApiResponseTrait for PatchCommentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 解决/恢复评论
pub async fn patch_comment(
    request: PatchCommentRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<PatchCommentResponse>> {
    let mut api_req = request.api_request;
    api_req.http_method = Method::PATCH;
    api_req.api_path = format!(
        "/open-apis/comment/v1/comments/{}?file_type={}&file_token={}",
        request.comment_id, request.file_type, request.file_token
    );

    // 添加用户ID类型查询参数
    if let Some(user_id_type) = request.user_id_type {
        api_req.api_path = format!("{}&user_id_type={}", api_req.api_path, user_id_type);
    }

    api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

    let api_resp = Transport::request(api_req, config, option).await?;
    Ok(api_resp)
}

impl PatchCommentResponse {
    /// 是否已解决
    pub fn is_solved(&self) -> bool {
        self.is_solved
    }

    /// 是否被恢复
    pub fn is_restored(&self) -> bool {
        !self.is_solved
    }

    /// 是否有解决时间
    pub fn has_solved_time(&self) -> bool {
        self.solved_time.is_some()
    }

    /// 是否有解决者
    pub fn has_solver(&self) -> bool {
        self.solver_user_id.is_some()
    }

    /// 获取解决时间的格式化字符串
    pub fn solved_time_formatted(&self) -> Option<String> {
        self.solved_time.map(|timestamp| {
            // 这里可以根据需要格式化时间戳
            format!("解决时间: {}", timestamp)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_patch_comment_request_builder() {
        let request = PatchCommentRequest::builder()
            .file_token("doccnxxxxxx")
            .as_doc()
            .comment_id("comment123")
            .solve_comment()
            .with_open_id()
            .build();

        assert_eq!(request.file_token, "doccnxxxxxx");
        assert_eq!(request.file_type, "doc");
        assert_eq!(request.comment_id, "comment123");
        assert_eq!(request.is_solved, true);
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
    }

    #[test]
    fn test_patch_comment_convenience_methods() {
        let solve_request = PatchCommentRequest::solve("doccnxxxxxx", "doc", "comment123");
        assert_eq!(solve_request.is_solved, true);

        let restore_request = PatchCommentRequest::restore("doccnxxxxxx", "doc", "comment123");
        assert_eq!(restore_request.is_solved, false);
    }
}
