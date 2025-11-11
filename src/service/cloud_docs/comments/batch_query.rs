use open_lark_core::SDKResult;use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, ResponseFormat},
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
///
/// 用于批量获取指定云文档中的多个评论的详细信息。
/// 支持一次请求获取多个评论，提高API调用效率。
///
/// # 示例
/// ```
/// use open_lark::service::cloud_docs::comments::batch_query::BatchQueryCommentsRequest;
///
/// let request = BatchQueryCommentsRequest::builder()
///     .file_token("doccnxxxxxx")
///     .with_doc_type()
///     .comment_ids(vec!["comment_123", "comment_456"])
///     .with_open_id()
///     .build();
/// ```
#[derive(Debug, Clone, Serialize)]
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
    /// 创建新的批量查询评论请求
    ///
    /// # 参数
    /// - `file_token`: 文档标识符
    /// - `file_type`: 文档类型
    /// - `comment_ids`: 评论ID列表
    ///
    /// # 示例
    /// ```
    /// use open_lark::service::cloud_docs::comments::batch_query::BatchQueryCommentsRequest;
    ///
    /// let request = BatchQueryCommentsRequest::new(
    ///     "doccnxxxxxx",
    ///     "doc",
    ///     vec!["comment_123", "comment_456"]
    /// );
    /// ```
    pub fn new(
        file_token: impl ToString,
        file_type: impl ToString,
        comment_ids: Vec<impl ToString>,
    ) -> Self {
        let mut api_request = ApiRequest::new();
        api_request.api_path = COMMENT_V1_BATCH_QUERY.to_string();

        Self {
            api_request,
            file_token: file_token.to_string(),
            file_type: file_type.to_string(),
            comment_ids: comment_ids.into_iter().map(|id| id.to_string()).collect(),
            user_id_type: None,
        }
    }

    /// 创建构建器实例
    pub fn builder() -> BatchQueryCommentsRequestBuilder {
        BatchQueryCommentsRequestBuilder::default()
    }

    /// 验证请求参数
    ///
    /// # 返回
    /// 如果请求参数有效则返回Ok(())，否则返回错误信息
    pub fn validate(&self) -> Result<(), String> {
        if self.comment_ids.is_empty() {
            return Err("评论ID列表不能为空".to_string());
        }

        if self.comment_ids.len() > 50 {
            return Err("单次批量查询最多支持50个评论ID".to_string());
        }

        if self.comment_ids.iter().any(|id| id.is_empty()) {
            return Err("评论ID不能为空".to_string());
        }

        Ok(())
    }
}

/// 批量获取评论请求构建器
///
/// 提供流式API来构建BatchQueryCommentsRequest，支持链式调用。
#[derive(Debug, Clone, Default)]
pub struct BatchQueryCommentsRequestBuilder {
    file_token: Option<String>,
    file_type: Option<String>,
    comment_ids: Option<Vec<String>>,
    user_id_type: Option<String>,
}

impl BatchQueryCommentsRequestBuilder {
    /// 设置文档token
    ///
    /// # 参数
    /// - `file_token`: 文档标识符
    pub fn file_token(mut self, file_token: impl ToString) -> Self {
        self.file_token = Some(file_token.to_string());
        self
    }

    /// 设置文档类型为doc
    pub fn with_doc_type(self) -> Self {
        self.file_type("doc")
    }

    /// 设置文档类型为docx
    pub fn with_docx_type(self) -> Self {
        self.file_type("docx")
    }

    /// 设置文档类型为sheet
    pub fn with_sheet_type(self) -> Self {
        self.file_type("sheet")
    }

    /// 设置文档类型为bitable
    pub fn with_bitable_type(self) -> Self {
        self.file_type("bitable")
    }

    /// 设置文档类型
    ///
    /// # 参数
    /// - `file_type`: 文档类型（doc、docx、sheet、bitable）
    pub fn file_type(mut self, file_type: impl ToString) -> Self {
        self.file_type = Some(file_type.to_string());
        self
    }

    /// 设置评论ID列表
    ///
    /// # 参数
    /// - `comment_ids`: 评论ID列表
    ///
    /// # 示例
    /// ```
    /// use open_lark::service::cloud_docs::comments::batch_query::BatchQueryCommentsRequest;
    ///
    /// let builder = BatchQueryCommentsRequest::builder()
    ///     .comment_ids(vec!["comment_123", "comment_456"]);
    /// ```
    pub fn comment_ids(mut self, comment_ids: Vec<impl ToString>) -> Self {
        self.comment_ids = Some(comment_ids.into_iter().map(|id| id.to_string()).collect());
        self
    }

    /// 添加单个评论ID
    ///
    /// # 参数
    /// - `comment_id`: 评论ID
    pub fn add_comment_id(mut self, comment_id: impl ToString) -> Self {
        if self.comment_ids.is_none() {
            self.comment_ids = Some(vec![]);
        }
        if let Some(ref mut ids) = self.comment_ids {
            ids.push(comment_id.to_string());
        }
        self
    }

    /// 添加多个评论ID
    ///
    /// # 参数
    /// - `comment_ids`: 评论ID列表
    pub fn add_comment_ids(mut self, comment_ids: Vec<impl ToString>) -> Self {
        if self.comment_ids.is_none() {
            self.comment_ids = Some(vec![]);
        }
        if let Some(ref mut ids) = self.comment_ids {
            ids.extend(comment_ids.into_iter().map(|id| id.to_string()));
        }
        self
    }

    /// 设置用户ID类型
    ///
    /// # 参数
    /// - `user_id_type`: 用户ID类型
    pub fn user_id_type(mut self, user_id_type: impl ToString) -> Self {
        self.user_id_type = Some(user_id_type.to_string());
        self
    }

    /// 使用OpenID作为用户ID类型
    pub fn with_open_id(self) -> Self {
        self.user_id_type("open_id")
    }

    /// 使用UserID作为用户ID类型
    pub fn with_user_id(self) -> Self {
        self.user_id_type("user_id")
    }

    /// 使用UnionID作为用户ID类型
    pub fn with_union_id(self) -> Self {
        self.user_id_type("union_id")
    }

    /// 构建BatchQueryCommentsRequest实例
    ///
    /// # 返回
    /// 返回配置好的BatchQueryCommentsRequest实例
    ///
    /// # 示例
    /// ```
    /// use open_lark::service::cloud_docs::comments::batch_query::BatchQueryCommentsRequest;
    ///
    /// let request = BatchQueryCommentsRequest::builder()
    ///     .file_token("doccnxxxxxx")
    ///     .with_doc_type()
    ///     .comment_ids(vec!["comment_123", "comment_456"])
    ///     .with_open_id()
    ///     .build();
    /// ```
    pub fn build(self) -> BatchQueryCommentsRequest {
        let file_token = self.file_token.expect("file_token is required");
        let file_type = self.file_type.expect("file_type is required");
        let comment_ids = self.comment_ids.expect("comment_ids is required");

        let mut request = BatchQueryCommentsRequest::new(&file_token, &file_type, comment_ids);
        request.user_id_type = self.user_id_type;

        // 自动验证请求参数
        if let Err(err) = request.validate() {
            panic!("批量查询请求参数验证失败: {}", err);
        }

        request
    }
}

// 应用ExecutableBuilder trait到BatchQueryCommentsRequestBuilder
impl_executable_builder_owned!(
    BatchQueryCommentsRequestBuilder,
    super::CommentsService,
    BatchQueryCommentsRequest,
    crate::core::api_resp::BaseResponse<BatchQueryCommentsResponse>,
    batch_query,
);

/// 批量获取评论响应
///
/// 包含批量查询的评论信息
#[derive(Debug, Clone, Deserialize)]
pub struct BatchQueryCommentsResponse {
    /// 评论列表
    pub items: Vec<Comment>,
    /// 请求的评论ID总数
    pub total_requested: i32,
    /// 成功获取的评论数量
    pub successful_count: i32,
    /// 失败的评论ID列表
    pub failed_comment_ids: Option<Vec<String>>,
}

impl BatchQueryCommentsResponse {
    /// 创建新的批量查询响应
    ///
    /// # 参数
    /// - `items`: 评论列表
    /// - `total_requested`: 请求的评论ID总数
    /// - `successful_count`: 成功获取的评论数量
    /// - `failed_comment_ids`: 失败的评论ID列表
    pub fn new(
        items: Vec<Comment>,
        total_requested: i32,
        successful_count: i32,
        failed_comment_ids: Option<Vec<String>>,
    ) -> Self {
        Self {
            items,
            total_requested,
            successful_count,
            failed_comment_ids,
        }
    }

    /// 获取成功获取的评论数量
    pub fn success_count(&self) -> usize {
        self.items.len()
    }

    /// 获取失败的评论数量
    pub fn failed_count(&self) -> i32 {
        self.total_requested - self.successful_count
    }

    /// 检查是否所有评论都成功获取
    pub fn is_all_successful(&self) -> bool {
        self.successful_count == self.total_requested
    }

    /// 获取成功率
    ///
    /// # 返回
    /// 返回成功率（0.0-1.0之间）
    pub fn success_rate(&self) -> f64 {
        if self.total_requested == 0 {
            1.0
        } else {
            self.successful_count as f64 / self.total_requested as f64
        }
    }

    /// 获取失败的评论ID列表
    pub fn failed_ids(&self) -> &[String] {
        self.failed_comment_ids.as_ref().map_or(&[], |ids| ids.as_slice())
    }

    /// 根据评论ID查找评论
    ///
    /// # 参数
    /// - `comment_id`: 评论ID
    ///
    /// # 返回
    /// 如果找到则返回评论，否则返回None
    pub fn find_comment_by_id(&self, comment_id: &str) -> Option<&Comment> {
        self.items.iter().find(|comment| comment.comment_id == comment_id)
    }

    /// 获取已解决的评论数量
    pub fn solved_comment_count(&self) -> usize {
        self.items.iter().filter(|comment| comment.is_solved).count()
    }

    /// 获取未解决的评论数量
    pub fn unsolved_comment_count(&self) -> usize {
        self.items.iter().filter(|comment| !comment.is_solved).count()
    }

    /// 获取全文评论数量
    pub fn whole_comment_count(&self) -> usize {
        self.items
            .iter()
            .filter(|comment| comment.is_whole_comment())
            .count()
    }

    /// 获取部分评论数量
    pub fn partial_comment_count(&self) -> usize {
        self.items
            .iter()
            .filter(|comment| !comment.is_whole_comment())
            .count()
    }

    /// 按创建时间排序评论（最新的在前）
    pub fn sorted_by_create_time_desc(&self) -> Vec<&Comment> {
        let mut comments: Vec<&Comment> = self.items.iter().collect();
        comments.sort_by(|a, b| b.create_time.cmp(&a.create_time));
        comments
    }

    /// 按创建时间排序评论（最旧的在前）
    pub fn sorted_by_create_time_asc(&self) -> Vec<&Comment> {
        let mut comments: Vec<&Comment> = self.items.iter().collect();
        comments.sort_by(|a, b| a.create_time.cmp(&b.create_time));
        comments
    }

    /// 获取指定用户创建的评论
    ///
    /// # 参数
    /// - `user_id`: 用户ID
    ///
    /// # 返回
    /// 返回指定用户创建的评论列表
    pub fn comments_by_user(&self, user_id: &str) -> Vec<&Comment> {
        self.items
            .iter()
            .filter(|comment| comment.user_id == user_id)
            .collect()
    }
}

impl ApiResponseTrait for BatchQueryCommentsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量获取评论
///
/// 批量获取指定云文档中的多个评论的详细信息。
///
/// # API文档
/// <https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/cloud_docs-v1/comment/batch_query>
///
/// # 参数
/// - `request`: 批量查询评论请求
/// - `config`: SDK配置
/// - `option`: 可选请求参数
///
/// # 返回
/// 包含批量评论信息的响应
///
/// # 示例
/// ```
/// use open_lark::service::cloud_docs::comments::batch_query::{batch_query_comments, BatchQueryCommentsRequest};
/// use open_lark::core::config::Config;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let config = Config::new("app_id", "app_secret");
///     let request = BatchQueryCommentsRequest::builder()
///         .file_token("doccnxxxxxx")
///         .with_doc_type()
///         .comment_ids(vec!["comment_123", "comment_456"])
///         .with_open_id()
///         .build();
///
///     let response = batch_query_comments(request, &config, None).await?;
///     println!("成功获取 {} 条评论", response.data.success_count());
///     println!("成功率: {:.2}%", response.data.success_rate() * 100.0);
///
///     Ok(())
/// }
/// ```
pub async fn batch_query_comments(
    request: BatchQueryCommentsRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<crate::core::api_resp::BaseResponse<BatchQueryCommentsResponse>> {
    // 验证请求参数
    request.validate().map_err(|err| {
        crate::core::error::SDKError::InvalidRequest(err)
    })?;

    let mut api_req = request.api_request;
    api_req.set_http_method(Method::POST);
    api_req.api_path = format!(
        "{}?file_type={}&file_token={}",
        COMMENT_V1_BATCH_QUERY, request.file_type, request.file_token,
    );

    // 添加用户ID类型查询参数
    if let Some(user_id_type) = request.user_id_type {
        api_req.api_path = format!(
            "{}&user_id_type={}",
            api_req.api_path, user_id_type,
        );
    }

    // 设置请求体
    api_req.body = Some(serde_json::to_vec(&request)?);

    api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

    let api_resp = Transport::request(api_req, config, option).await?;
    Ok(api_resp)
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
            .comment_ids(vec!["comment_123", "comment_456"])
            .with_open_id()
            .build();

        assert_eq!(request.file_token, "doccnxxxxxx");
        assert_eq!(request.file_type, "doc");
        assert_eq!(request.comment_ids.len(), 2);
        assert_eq!(request.comment_ids[0], "comment_123");
        assert_eq!(request.comment_ids[1], "comment_456");
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
    }

    #[test]
    fn test_batch_query_comments_request_new() {
        let request = BatchQueryCommentsRequest::new(
            "doccnxxxxxx",
            "doc",
            vec!["comment_123", "comment_456", "comment_789"],
        );

        assert_eq!(request.file_token, "doccnxxxxxx");
        assert_eq!(request.file_type, "doc");
        assert_eq!(request.comment_ids.len(), 3);
        assert_eq!(request.comment_ids[0], "comment_123");
        assert_eq!(request.comment_ids[1], "comment_456");
        assert_eq!(request.comment_ids[2], "comment_789");
        assert_eq!(request.user_id_type, None);
    }

    #[test]
    fn test_batch_query_request_validation() {
        // 测试空评论ID列表
        let request = BatchQueryCommentsRequest::new("doc_token", "doc", vec![]);
        assert!(request.validate().is_err());

        // 测试超过50个评论ID
        let too_many_ids: Vec<String> = (0..51).map(|i| format!("comment_{}", i)).collect();
        let request = BatchQueryCommentsRequest::new("doc_token", "doc", too_many_ids);
        assert!(request.validate().is_err());

        // 测试包含空字符串的评论ID
        let request = BatchQueryCommentsRequest::new("doc_token", "doc", vec!["", "valid_id"]);
        assert!(request.validate().is_err());

        // 测试有效的请求
        let request = BatchQueryCommentsRequest::new("doc_token", "doc", vec!["valid_id"]);
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_builder_with_add_comment_id() {
        let request = BatchQueryCommentsRequest::builder()
            .file_token("doccnxxxxxx")
            .with_doc_type()
            .add_comment_id("comment_123")
            .add_comment_id("comment_456")
            .add_comment_ids(vec!["comment_789", "comment_abc"])
            .build();

        assert_eq!(request.comment_ids.len(), 4);
        assert_eq!(request.comment_ids[0], "comment_123");
        assert_eq!(request.comment_ids[1], "comment_456");
        assert_eq!(request.comment_ids[2], "comment_789");
        assert_eq!(request.comment_ids[3], "comment_abc");
    }

    #[test]
    fn test_all_file_types() {
        let doc_request = BatchQueryCommentsRequest::builder()
            .file_token("doc_token")
            .with_doc_type()
            .comment_ids(vec!["comment_123"])
            .build();

        let docx_request = BatchQueryCommentsRequest::builder()
            .file_token("docx_token")
            .with_docx_type()
            .comment_ids(vec!["comment_123"])
            .build();

        let sheet_request = BatchQueryCommentsRequest::builder()
            .file_token("sheet_token")
            .with_sheet_type()
            .comment_ids(vec!["comment_123"])
            .build();

        let bitable_request = BatchQueryCommentsRequest::builder()
            .file_token("bitable_token")
            .with_bitable_type()
            .comment_ids(vec!["comment_123"])
            .build();

        assert_eq!(doc_request.file_type, "doc");
        assert_eq!(docx_request.file_type, "docx");
        assert_eq!(sheet_request.file_type, "sheet");
        assert_eq!(bitable_request.file_type, "bitable");
    }

    #[test]
    fn test_all_user_id_types() {
        let open_id_request = BatchQueryCommentsRequest::builder()
            .file_token("token")
            .with_doc_type()
            .comment_ids(vec!["comment_123"])
            .with_open_id()
            .build();

        let user_id_request = BatchQueryCommentsRequest::builder()
            .file_token("token")
            .with_doc_type()
            .comment_ids(vec!["comment_123"])
            .with_user_id()
            .build();

        let union_id_request = BatchQueryCommentsRequest::builder()
            .file_token("token")
            .with_doc_type()
            .comment_ids(vec!["comment_123"])
            .with_union_id()
            .build();

        assert_eq!(open_id_request.user_id_type, Some("open_id".to_string()));
        assert_eq!(user_id_request.user_id_type, Some("user_id".to_string()));
        assert_eq!(union_id_request.user_id_type, Some("union_id".to_string()));
    }

    #[test]
    fn test_batch_query_comments_response() {
        let comment1 = Comment::new("comment1", "user1", 1000, 1001, true)
            .with_whole(true);
        let comment2 = Comment::new("comment2", "user2", 1002, 1003, false)
            .with_whole(false);

        let response = BatchQueryCommentsResponse::new(
            vec![comment1, comment2],
            3,
            2,
            Some(vec!["comment_failed".to_string()]),
        );

        assert_eq!(response.success_count(), 2);
        assert_eq!(response.failed_count(), 1);
        assert_eq!(response.success_rate(), 2.0 / 3.0);
        assert!(!response.is_all_successful());
        assert_eq!(response.failed_ids().len(), 1);
        assert_eq!(response.failed_ids()[0], "comment_failed");
    }

    #[test]
    fn test_batch_query_response_methods() {
        let comment1 = Comment::new("comment1", "user1", 1000, 1001, true)
            .with_whole(true);
        let comment2 = Comment::new("comment2", "user1", 1002, 1003, false)
            .with_whole(false);

        let response = BatchQueryCommentsResponse::new(
            vec![comment1, comment2],
            2,
            2,
            None,
        );

        assert_eq!(response.solved_comment_count(), 1);
        assert_eq!(response.unsolved_comment_count(), 1);
        assert_eq!(response.whole_comment_count(), 1);
        assert_eq!(response.partial_comment_count(), 1);
        assert_eq!(response.comments_by_user("user1").len(), 2);
        assert_eq!(response.comments_by_user("user2").len(), 0);

        // 测试查找评论
        assert!(response.find_comment_by_id("comment1").is_some());
        assert!(response.find_comment_by_id("nonexistent").is_none());
    }

    #[test]
    fn test_request_validation_edge_cases() {
        // 测试刚好50个评论ID（应该成功）
        let fifty_ids: Vec<String> = (0..50).map(|i| format!("comment_{}", i)).collect();
        let request = BatchQueryCommentsRequest::new("doc_token", "doc", fifty_ids);
        assert!(request.validate().is_ok());

        // 测试51个评论ID（应该失败）
        let fifty_one_ids: Vec<String> = (0..51).map(|i| format!("comment_{}", i)).collect();
        let request = BatchQueryCommentsRequest::new("doc_token", "doc", fifty_one_ids);
        assert!(request.validate().is_err());

        // 测试重复的评论ID（应该成功，API会去重）
        let duplicate_ids = vec!["comment_1", "comment_1", "comment_2"];
        let request = BatchQueryCommentsRequest::new("doc_token", "doc", duplicate_ids);
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_request_validation_panic() {
        // 测试构建器会在build时验证参数，不合法的参数会panic
        let result = std::panic::catch_unwind(|| {
            BatchQueryCommentsRequest::builder()
                .file_token("doc_token")
                .with_doc_type()
                // 缺少comment_ids
                .build();
        });
        assert!(result.is_err());

        let result = std::panic::catch_unwind(|| {
            BatchQueryCommentsRequest::builder()
                .file_token("doc_token")
                .with_doc_type()
                .comment_ids(vec![]) // 空列表
                .build();
        });
        assert!(result.is_err());
    }

    #[test]
    fn test_response_sorting() {
        let comment1 = Comment::new("comment1", "user1", 3000, 3001, false);
        let comment2 = Comment::new("comment2", "user2", 1000, 1001, false);
        let comment3 = Comment::new("comment3", "user3", 2000, 2001, false);

        let response = BatchQueryCommentsResponse::new(
            vec![comment1, comment2, comment3],
            3,
            3,
            None,
        );

        // 测试降序排序（最新的在前）
        let desc_sorted = response.sorted_by_create_time_desc();
        assert_eq!(desc_sorted[0].comment_id, "comment1");
        assert_eq!(desc_sorted[1].comment_id, "comment3");
        assert_eq!(desc_sorted[2].comment_id, "comment2");

        // 测试升序排序（最旧的在前）
        let asc_sorted = response.sorted_by_create_time_asc();
        assert_eq!(asc_sorted[0].comment_id, "comment2");
        assert_eq!(asc_sorted[1].comment_id, "comment3");
        assert_eq!(asc_sorted[2].comment_id, "comment1");
    }
}