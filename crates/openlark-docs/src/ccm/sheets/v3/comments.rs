//! Sheets电子表格评论服务 v3
//!
//! 提供飞书电子表格v3版本的评论管理功能，包括：
//! - 创建和删除评论
//! - 回复评论和点赞
//! - 评论查询和编辑
//! - @提及用户和通知

use openlark_core::error::LarkAPIError;

// 使用统一类型定义
use super::Range;

use serde::{Deserialize, Serialize};

/// 评论状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CommentStatus {
    /// 活动
    #[serde(rename = "ACTIVE")]
    Active,
    /// 已解决
    #[serde(rename = "RESOLVED")]
    Resolved,
    /// 已删除
    #[serde(rename = "DELETED")]
    Deleted,
}

impl std::fmt::Display for CommentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommentStatus::Active => write!(f, "ACTIVE"),
            CommentStatus::Resolved => write!(f, "RESOLVED"),
            CommentStatus::Deleted => write!(f, "DELETED"),
        }
    }
}

/// 评论类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CommentType {
    /// 单元格评论
    #[serde(rename = "CELL")]
    Cell,
    /// 范围评论
    #[serde(rename = "RANGE")]
    Range,
}

/// 评论提及的用户
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentMention {
    /// 用户ID
    #[serde(rename = "user_id")]
    pub user_id: String,
    /// 用户名
    #[serde(rename = "user_name")]
    pub user_name: String,
    /// 提及文本
    #[serde(rename = "mention_text")]
    pub mention_text: String,
}

impl CommentMention {
    /// 创建评论提及
    pub fn new(user_id: String, user_name: String, mention_text: String) -> Self {
        Self {
            user_id,
            user_name,
            mention_text,
        }
    }
}

/// 评论内容
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentContent {
    /// 文本内容
    #[serde(rename = "text")]
    pub text: String,
    /// 提及的用户列表
    #[serde(rename = "mentions")]
    pub mentions: Vec<CommentMention>,
}

impl CommentContent {
    /// 创建评论内容
    pub fn new(text: String) -> Self {
        Self {
            text,
            mentions: vec![],
        }
    }

    /// 添加提及用户
    pub fn add_mention(mut self, mention: CommentMention) -> Self {
        self.mentions.push(mention);
        self
    }

    /// 添加用户提及
    pub fn add_user_mention(mut self, user_id: String, user_name: String) -> Self {
        let mention_text = format!("@{}", user_name);
        let mention = CommentMention::new(user_id, user_name, mention_text);
        self.mentions.push(mention);
        self
    }
}

/// 评论回复
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentReply {
    /// 回复ID
    #[serde(rename = "reply_id")]
    pub reply_id: String,
    /// 回复内容
    #[serde(rename = "content")]
    pub content: CommentContent,
    /// 回复者信息
    #[serde(rename = "author")]
    pub author: CommentAuthor,
    /// 创建时间
    #[serde(rename = "created_time")]
    pub created_time: String,
    /// 修改时间
    #[serde(rename = "modified_time")]
    pub modified_time: Option<String>,
}

/// 评论作者
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentAuthor {
    /// 用户ID
    #[serde(rename = "user_id")]
    pub user_id: String,
    /// 用户名
    #[serde(rename = "user_name")]
    pub user_name: String,
    /// 用户头像
    #[serde(rename = "avatar_url")]
    pub avatar_url: Option<String>,
}

impl CommentAuthor {
    /// 创建评论作者
    pub fn new(user_id: String, user_name: String) -> Self {
        Self {
            user_id,
            user_name,
            avatar_url: None,
        }
    }

    /// 设置头像URL
    pub fn avatar_url(mut self, avatar_url: String) -> Self {
        self.avatar_url = Some(avatar_url);
        self
    }
}

/// 评论对象
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    /// 评论ID
    #[serde(rename = "comment_id")]
    pub comment_id: String,
    /// 工作表ID
    #[serde(rename = "sheet_id")]
    pub sheet_id: String,
    /// 评论类型
    #[serde(rename = "comment_type")]
    pub comment_type: CommentType,
    /// 评论范围（仅RANGE类型）
    #[serde(rename = "range")]
    pub range: Option<Range>,
    /// 单元格引用（仅CELL类型）
    #[serde(rename = "cell_reference")]
    pub cell_reference: Option<String>,
    /// 评论内容
    #[serde(rename = "content")]
    pub content: CommentContent,
    /// 评论作者
    #[serde(rename = "author")]
    pub author: CommentAuthor,
    /// 评论状态
    #[serde(rename = "status")]
    pub status: CommentStatus,
    /// 创建时间
    #[serde(rename = "created_time")]
    pub created_time: String,
    /// 修改时间
    #[serde(rename = "modified_time")]
    pub modified_time: Option<String>,
    /// 回复列表
    #[serde(rename = "replies")]
    pub replies: Vec<CommentReply>,
    /// 点赞数量
    #[serde(rename = "like_count")]
    pub like_count: u32,
    /// 当前用户是否点赞
    #[serde(rename = "is_liked")]
    pub is_liked: bool,
}

impl Comment {
    /// 创建评论
    pub fn new(
        comment_id: String,
        sheet_id: String,
        comment_type: CommentType,
        content: CommentContent,
        author: CommentAuthor,
    ) -> Self {
        Self {
            comment_id,
            sheet_id,
            comment_type,
            range: None,
            cell_reference: None,
            content,
            author,
            status: CommentStatus::Active,
            created_time: String::new(),
            modified_time: None,
            replies: vec![],
            like_count: 0,
            is_liked: false,
        }
    }

    /// 设置评论范围
    pub fn range(mut self, range: Range) -> Self {
        self.range = Some(range);
        self
    }

    /// 设置单元格引用
    pub fn cell_reference(mut self, cell_reference: String) -> Self {
        self.cell_reference = Some(cell_reference);
        self
    }

    /// 添加回复
    pub fn add_reply(mut self, reply: CommentReply) -> Self {
        self.replies.push(reply);
        self
    }

    /// 验证评论
    pub fn validate(&self) -> Result<(), LarkAPIError> {
        if self.sheet_id.is_empty() {
            return Err(LarkAPIError::IllegalParamError(
                "工作表ID不能为空".to_string(),
            ));
        }

        if self.content.text.is_empty() {
            return Err(LarkAPIError::IllegalParamError(
                "评论内容不能为空".to_string(),
            ));
        }

        // 验证评论类型和对应字段的匹配
        match self.comment_type {
            CommentType::Cell => {
                if self.cell_reference.is_none() {
                    return Err(LarkAPIError::IllegalParamError(
                        "单元格评论必须提供单元格引用".to_string(),
                    ));
                }
            }
            CommentType::Range => {
                if self.range.is_none() {
                    return Err(LarkAPIError::IllegalParamError(
                        "范围评论必须提供评论范围".to_string(),
                    ));
                }
            }
        }

        Ok(())
    }
}

/// 创建评论请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCommentRequest {
    /// 电子表格ID
    #[serde(rename = "spreadsheet_token")]
    pub spreadsheet_token: String,
    /// 工作表ID
    #[serde(rename = "sheet_id")]
    pub sheet_id: String,
    /// 评论类型
    #[serde(rename = "comment_type")]
    pub comment_type: CommentType,
    /// 评论范围
    #[serde(rename = "range")]
    pub range: Option<Range>,
    /// 单元格引用
    #[serde(rename = "cell_reference")]
    pub cell_reference: Option<String>,
    /// 评论内容
    #[serde(rename = "content")]
    pub content: CommentContent,
}

impl CreateCommentRequest {
    /// 创建创建评论请求
    pub fn new(
        spreadsheet_token: String,
        sheet_id: String,
        comment_type: CommentType,
        content: CommentContent,
    ) -> Self {
        Self {
            spreadsheet_token,
            sheet_id,
            comment_type,
            range: None,
            cell_reference: None,
            content,
        }
    }

    /// 设置评论范围
    pub fn range(mut self, range: Range) -> Self {
        self.range = Some(range);
        self
    }

    /// 设置单元格引用
    pub fn cell_reference(mut self, cell_reference: String) -> Self {
        self.cell_reference = Some(cell_reference);
        self
    }

    /// 构建器模式
    pub fn builder() -> CreateCommentRequestBuilder {
        CreateCommentRequestBuilder::default()
    }

    /// 验证请求
    pub fn validate(&self) -> Result<(), LarkAPIError> {
        if self.spreadsheet_token.is_empty() {
            return Err(LarkAPIError::IllegalParamError(
                "电子表格ID不能为空".to_string(),
            ));
        }

        if self.sheet_id.is_empty() {
            return Err(LarkAPIError::IllegalParamError(
                "工作表ID不能为空".to_string(),
            ));
        }

        if self.content.text.is_empty() {
            return Err(LarkAPIError::IllegalParamError(
                "评论内容不能为空".to_string(),
            ));
        }

        // 验证评论类型和对应字段的匹配
        match self.comment_type {
            CommentType::Cell => {
                if self.cell_reference.is_none() {
                    return Err(LarkAPIError::IllegalParamError(
                        "单元格评论必须提供单元格引用".to_string(),
                    ));
                }
            }
            CommentType::Range => {
                if self.range.is_none() {
                    return Err(LarkAPIError::IllegalParamError(
                        "范围评论必须提供评论范围".to_string(),
                    ));
                }
            }
        }

        Ok(())
    }
}

/// 创建评论请求构建器
#[derive(Debug, Default)]
pub struct CreateCommentRequestBuilder {
    spreadsheet_token: Option<String>,
    sheet_id: Option<String>,
    comment_type: Option<CommentType>,
    range: Option<Range>,
    cell_reference: Option<String>,
    content: Option<CommentContent>,
}

impl CreateCommentRequestBuilder {
    /// 设置电子表格ID
    pub fn spreadsheet_token(mut self, spreadsheet_token: String) -> Self {
        self.spreadsheet_token = Some(spreadsheet_token);
        self
    }

    /// 设置工作表ID
    pub fn sheet_id(mut self, sheet_id: String) -> Self {
        self.sheet_id = Some(sheet_id);
        self
    }

    /// 设置评论类型
    pub fn comment_type(mut self, comment_type: CommentType) -> Self {
        self.comment_type = Some(comment_type);
        self
    }

    /// 设置评论范围
    pub fn range(mut self, range: Range) -> Self {
        self.range = Some(range);
        self
    }

    /// 设置单元格引用
    pub fn cell_reference(mut self, cell_reference: String) -> Self {
        self.cell_reference = Some(cell_reference);
        self
    }

    /// 设置评论内容
    pub fn content(mut self, content: CommentContent) -> Self {
        self.content = Some(content);
        self
    }

    /// 构建请求对象
    pub fn build(self) -> Result<CreateCommentRequest, LarkAPIError> {
        let spreadsheet_token = self
            .spreadsheet_token
            .ok_or_else(|| LarkAPIError::IllegalParamError("电子表格ID不能为空".to_string()))?;

        let sheet_id = self
            .sheet_id
            .ok_or_else(|| LarkAPIError::IllegalParamError("工作表ID不能为空".to_string()))?;

        let comment_type = self
            .comment_type
            .ok_or_else(|| LarkAPIError::IllegalParamError("评论类型不能为空".to_string()))?;

        let content = self
            .content
            .ok_or_else(|| LarkAPIError::IllegalParamError("评论内容不能为空".to_string()))?;

        let mut request =
            CreateCommentRequest::new(spreadsheet_token, sheet_id, comment_type, content);

        if let Some(range) = self.range {
            request = request.range(range);
        }
        if let Some(cell_reference) = self.cell_reference {
            request = request.cell_reference(cell_reference);
        }

        request.validate()?;
        Ok(request)
    }
}

/// 创建评论响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCommentResponse {
    /// 评论信息
    #[serde(rename = "comment")]
    pub comment: Comment,
}

impl openlark_core::api::ApiResponseTrait for CreateCommentResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

/// 查询评论请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCommentsRequest {
    /// 电子表格ID
    #[serde(rename = "spreadsheet_token")]
    pub spreadsheet_token: String,
    /// 工作表ID
    #[serde(rename = "sheet_id")]
    pub sheet_id: String,
    /// 页码（从0开始）
    #[serde(rename = "page")]
    pub page: Option<u32>,
    /// 每页大小
    #[serde(rename = "page_size")]
    pub page_size: Option<u32>,
    /// 评论状态过滤
    #[serde(rename = "status")]
    pub status: Option<CommentStatus>,
    /// 评论类型过滤
    #[serde(rename = "comment_type")]
    pub comment_type: Option<CommentType>,
}

impl GetCommentsRequest {
    /// 创建查询评论请求
    pub fn new(spreadsheet_token: String, sheet_id: String) -> Self {
        Self {
            spreadsheet_token,
            sheet_id,
            page: Some(0),
            page_size: Some(20),
            status: None,
            comment_type: None,
        }
    }

    /// 设置页码
    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    /// 设置每页大小
    pub fn page_size(mut self, page_size: u32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置状态过滤
    pub fn status(mut self, status: CommentStatus) -> Self {
        self.status = Some(status);
        self
    }

    /// 设置评论类型过滤
    pub fn comment_type(mut self, comment_type: CommentType) -> Self {
        self.comment_type = Some(comment_type);
        self
    }

    /// 构建器模式
    pub fn builder() -> GetCommentsRequestBuilder {
        GetCommentsRequestBuilder::default()
    }

    /// 验证请求
    pub fn validate(&self) -> Result<(), LarkAPIError> {
        if self.spreadsheet_token.is_empty() {
            return Err(LarkAPIError::IllegalParamError(
                "电子表格ID不能为空".to_string(),
            ));
        }

        if self.sheet_id.is_empty() {
            return Err(LarkAPIError::IllegalParamError(
                "工作表ID不能为空".to_string(),
            ));
        }

        if let Some(page_size) = self.page_size {
            if page_size == 0 || page_size > 100 {
                return Err(LarkAPIError::IllegalParamError(
                    "每页大小必须在1-100之间".to_string(),
                ));
            }
        }

        Ok(())
    }
}

/// 查询评论请求构建器
#[derive(Debug, Default)]
pub struct GetCommentsRequestBuilder {
    spreadsheet_token: Option<String>,
    sheet_id: Option<String>,
    page: Option<u32>,
    page_size: Option<u32>,
    status: Option<CommentStatus>,
    comment_type: Option<CommentType>,
}

impl GetCommentsRequestBuilder {
    /// 设置电子表格ID
    pub fn spreadsheet_token(mut self, spreadsheet_token: String) -> Self {
        self.spreadsheet_token = Some(spreadsheet_token);
        self
    }

    /// 设置工作表ID
    pub fn sheet_id(mut self, sheet_id: String) -> Self {
        self.sheet_id = Some(sheet_id);
        self
    }

    /// 设置页码
    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    /// 设置每页大小
    pub fn page_size(mut self, page_size: u32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置状态过滤
    pub fn status(mut self, status: CommentStatus) -> Self {
        self.status = Some(status);
        self
    }

    /// 设置评论类型过滤
    pub fn comment_type(mut self, comment_type: CommentType) -> Self {
        self.comment_type = Some(comment_type);
        self
    }

    /// 构建请求对象
    pub fn build(self) -> Result<GetCommentsRequest, LarkAPIError> {
        let spreadsheet_token = self
            .spreadsheet_token
            .ok_or_else(|| LarkAPIError::IllegalParamError("电子表格ID不能为空".to_string()))?;

        let sheet_id = self
            .sheet_id
            .ok_or_else(|| LarkAPIError::IllegalParamError("工作表ID不能为空".to_string()))?;

        let mut request = GetCommentsRequest::new(spreadsheet_token, sheet_id);

        if let Some(page) = self.page {
            request = request.page(page);
        }
        if let Some(page_size) = self.page_size {
            request = request.page_size(page_size);
        }
        if let Some(status) = self.status {
            request = request.status(status);
        }
        if let Some(comment_type) = self.comment_type {
            request = request.comment_type(comment_type);
        }

        request.validate()?;
        Ok(request)
    }
}

/// 查询评论响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCommentsResponse {
    /// 评论列表
    #[serde(rename = "comments")]
    pub comments: Vec<Comment>,
    /// 总数量
    #[serde(rename = "total")]
    pub total: u32,
    /// 是否有下一页
    #[serde(rename = "has_more")]
    pub has_more: bool,
}

impl openlark_core::api::ApiResponseTrait for GetCommentsResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

/// 删除评论请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteCommentRequest {
    /// 电子表格ID
    #[serde(rename = "spreadsheet_token")]
    pub spreadsheet_token: String,
    /// 评论ID
    #[serde(rename = "comment_id")]
    pub comment_id: String,
}

impl DeleteCommentRequest {
    /// 创建删除评论请求
    pub fn new(spreadsheet_token: String, comment_id: String) -> Self {
        Self {
            spreadsheet_token,
            comment_id,
        }
    }

    /// 构建器模式
    pub fn builder() -> DeleteCommentRequestBuilder {
        DeleteCommentRequestBuilder::default()
    }

    /// 验证请求
    pub fn validate(&self) -> Result<(), LarkAPIError> {
        if self.spreadsheet_token.is_empty() {
            return Err(LarkAPIError::IllegalParamError(
                "电子表格ID不能为空".to_string(),
            ));
        }

        if self.comment_id.is_empty() {
            return Err(LarkAPIError::IllegalParamError(
                "评论ID不能为空".to_string(),
            ));
        }

        Ok(())
    }
}

/// 删除评论请求构建器
#[derive(Debug, Default)]
pub struct DeleteCommentRequestBuilder {
    spreadsheet_token: Option<String>,
    comment_id: Option<String>,
}

impl DeleteCommentRequestBuilder {
    /// 设置电子表格ID
    pub fn spreadsheet_token(mut self, spreadsheet_token: String) -> Self {
        self.spreadsheet_token = Some(spreadsheet_token);
        self
    }

    /// 设置评论ID
    pub fn comment_id(mut self, comment_id: String) -> Self {
        self.comment_id = Some(comment_id);
        self
    }

    /// 构建请求对象
    pub fn build(self) -> Result<DeleteCommentRequest, LarkAPIError> {
        let spreadsheet_token = self
            .spreadsheet_token
            .ok_or_else(|| LarkAPIError::IllegalParamError("电子表格ID不能为空".to_string()))?;

        let comment_id = self
            .comment_id
            .ok_or_else(|| LarkAPIError::IllegalParamError("评论ID不能为空".to_string()))?;

        let request = DeleteCommentRequest::new(spreadsheet_token, comment_id);
        request.validate()?;
        Ok(request)
    }
}

/// 删除评论响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteCommentResponse {
    /// 是否成功删除
    #[serde(rename = "success")]
    pub success: bool,
}

impl openlark_core::api::ApiResponseTrait for DeleteCommentResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

/// Sheets电子表格评论服务 v3
#[derive(Clone, Debug)]
pub struct CommentService {
    config: openlark_core::config::Config,
}

impl CommentService {
    /// 创建评论服务实例
    pub fn new(config: openlark_core::config::Config) -> Self {
        Self { config }
    }

    /// 创建评论
    ///
    /// 在指定工作表的单元格或范围中创建评论，支持@提及用户。
    ///
    /// # 参数
    /// - `request`: 创建评论请求
    ///
    /// # 返回
    /// 返回创建后的评论信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::sheets::v3::comments::*;
    /// use open_lark::service::sheets::v3::models::Range;
    ///
    /// // 创建评论内容
    /// let content = CommentContent::new("请检查这个数据".to_string())
    ///     .add_user_mention("user123".to_string(), "张三".to_string());
    ///
    /// // 创建单元格评论
    /// let request = CreateCommentRequest::builder()
    ///     .spreadsheet_token("your_token".to_string())
    ///     .sheet_id("sheet_id".to_string())
    ///     .comment_type(CommentType::Cell)
    ///     .cell_reference("A1".to_string())
    ///     .content(content)
    ///     .build()
    ///     .unwrap();
    ///
    /// let response = service.create_comment(&request).await?;
    /// ```
    pub async fn create_comment(
        &self,
        request: &CreateCommentRequest,
    ) -> openlark_core::error::SDKResult<CreateCommentResponse> {
        use openlark_core::{api::ApiRequest, api::Response, error::LarkAPIError, http::Transport};

        let endpoint = format!(
            "/open-apis/sheets/v3/spreadsheets/{}/comments",
            request.spreadsheet_token
        );

        let mut api_request = ApiRequest::with_method_and_path(openlark_core::api::HttpMethod::Post, &endpoint);
        api_request.body = Some(openlark_core::api::RequestData::Json(request))?;

        let response: Response<CreateCommentResponse> =
            Transport::request(api_request, &self.config, None).await?;

        if response.code() != 0 {
            return Err(LarkAPIError::APIError {
                code: response.code(),
                msg: response.msg().to_string(),
                error: None,
            });
        }

        response.data.ok_or_else(|| LarkAPIError::APIError {
            code: -1,
            msg: "响应数据为空".to_string(),
            error: None,
        })
    }

    /// 查询评论
    ///
    /// 查询指定工作表中的评论列表，支持分页和过滤。
    ///
    /// # 参数
    /// - `request`: 查询评论请求
    ///
    /// # 返回
    /// 返回评论列表和分页信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::sheets::v3::comments::*;
    ///
    /// // 查询活动评论
    /// let request = GetCommentsRequest::builder()
    ///     .spreadsheet_token("your_token".to_string())
    ///     .sheet_id("sheet_id".to_string())
    ///     .page(0)
    ///     .page_size(20)
    ///     .status(CommentStatus::Active)
    ///     .build()
    ///     .unwrap();
    ///
    /// let response = service.get_comments(&request).await?;
    /// println!("找到 {} 条评论", response.total);
    /// ```
    pub async fn get_comments(
        &self,
        request: &GetCommentsRequest,
    ) -> openlark_core::error::SDKResult<GetCommentsResponse> {
        use openlark_core::{api::ApiRequest, api::Response, error::LarkAPIError, http::Transport};

        let endpoint = format!(
            "/open-apis/sheets/v3/spreadsheets/{}/comments",
            request.spreadsheet_token
        );

        // 构建查询参数
        let mut query_params = vec![];
        query_params.push(format!("sheet_id={}", &request.sheet_id));
        if let Some(page_size) = request.page_size {
            query_params.push(format!("page_size={}", page_size));
        }
        if let Some(page) = request.page {
            query_params.push(format!("page={}", page));
        }
        if let Some(status) = &request.status {
            query_params.push(format!("status={}", status));
        }

        let endpoint_with_params = if !query_params.is_empty() {
            format!("{}?{}", endpoint, query_params.join("&"))
        } else {
            endpoint
        };

        let api_request =
            ApiRequest::with_method_and_path(openlark_core::api::HttpMethod::Get, &endpoint_with_params);

        let response: Response<GetCommentsResponse> =
            Transport::request(api_request, &self.config, None).await?;

        if response.code() != 0 {
            return Err(LarkAPIError::APIError {
                code: response.code(),
                msg: response.msg().to_string(),
                error: None,
            });
        }

        response.data.ok_or_else(|| LarkAPIError::APIError {
            code: -1,
            msg: "响应数据为空".to_string(),
            error: None,
        })
    }

    /// 删除评论
    ///
    /// 删除指定的评论。
    ///
    /// # 参数
    /// - `request`: 删除评论请求
    ///
    /// # 返回
    /// 返回删除操作的结果
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::sheets::v3::comments::*;
    ///
    /// // 删除评论
    /// let request = DeleteCommentRequest::builder()
    ///     .spreadsheet_token("your_token".to_string())
    ///     .comment_id("comment_id".to_string())
    ///     .build()
    ///     .unwrap();
    ///
    /// let response = service.delete_comment(&request).await?;
    /// assert!(response.success);
    /// ```
    pub async fn delete_comment(
        &self,
        request: &DeleteCommentRequest,
    ) -> openlark_core::error::SDKResult<DeleteCommentResponse> {
        use openlark_core::{api::ApiRequest, api::Response, error::LarkAPIError, http::Transport};

        let endpoint = format!(
            "/open-apis/sheets/v3/spreadsheets/{}/comments/{}",
            request.spreadsheet_token, request.comment_id
        );

        let api_request = ApiRequest::with_method_and_path(openlark_core::api::HttpMethod::Delete, &endpoint);

        let response: Response<DeleteCommentResponse> =
            Transport::request(api_request, &self.config, None).await?;

        if response.code() != 0 {
            return Err(LarkAPIError::APIError {
                code: response.code(),
                msg: response.msg().to_string(),
                error: None,
            });
        }

        response.data.ok_or_else(|| LarkAPIError::APIError {
            code: -1,
            msg: "响应数据为空".to_string(),
            error: None,
        })
    }

    /// 创建评论构建器
    pub fn create_comment_builder(&self) -> CreateCommentRequestBuilder {
        CreateCommentRequestBuilder::default()
    }

    /// 查询评论构建器
    pub fn get_comments_builder(&self) -> GetCommentsRequestBuilder {
        GetCommentsRequestBuilder::default()
    }

    /// 删除评论构建器
    pub fn delete_comment_builder(&self) -> DeleteCommentRequestBuilder {
        DeleteCommentRequestBuilder::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comment_mention_creation() {
        let mention = CommentMention::new(
            "user123".to_string(),
            "张三".to_string(),
            "@张三".to_string(),
        );

        assert_eq!(mention.user_id, "user123");
        assert_eq!(mention.user_name, "张三");
        assert_eq!(mention.mention_text, "@张三");
    }

    #[test]
    fn test_comment_content_creation() {
        let content = CommentContent::new("请检查这个数据".to_string())
            .add_user_mention("user123".to_string(), "张三".to_string())
            .add_user_mention("user456".to_string(), "李四".to_string());

        assert_eq!(content.text, "请检查这个数据");
        assert_eq!(content.mentions.len(), 2);
        assert_eq!(content.mentions[0].user_name, "张三");
        assert_eq!(content.mentions[1].user_name, "李四");
    }

    #[test]
    fn test_comment_author_creation() {
        let author = CommentAuthor::new("user123".to_string(), "张三".to_string())
            .avatar_url("https://example.com/avatar.jpg".to_string());

        assert_eq!(author.user_id, "user123");
        assert_eq!(author.user_name, "张三");
        assert_eq!(
            author.avatar_url,
            Some("https://example.com/avatar.jpg".to_string())
        );
    }

    #[test]
    fn test_comment_creation() {
        let content = CommentContent::new("测试评论".to_string());
        let author = CommentAuthor::new("user123".to_string(), "张三".to_string());

        let comment = Comment::new(
            "comment123".to_string(),
            "sheet123".to_string(),
            CommentType::Cell,
            content,
            author,
        )
        .cell_reference("A1".to_string());

        assert_eq!(comment.comment_id, "comment123");
        assert_eq!(comment.sheet_id, "sheet123");
        assert_eq!(comment.cell_reference, Some("A1".to_string()));
    }

    #[test]
    fn test_comment_validation() {
        let content = CommentContent::new("测试评论".to_string());
        let author = CommentAuthor::new("user123".to_string(), "张三".to_string());

        // 测试有效评论
        let valid_cell_comment = Comment::new(
            "comment123".to_string(),
            "sheet123".to_string(),
            CommentType::Cell,
            content.clone(),
            author.clone()
        )
        .cell_reference("A1".to_string());
        assert!(valid_cell_comment.validate().is_ok());

        // 测试有效范围评论
        use super::super::models::Range;
        let valid_range_comment = Comment::new(
            "comment124".to_string(),
            "sheet123".to_string(),
            CommentType::Range,
            content.clone(),
            author.clone()
        )
        .range(Range::from("A1".to_string(), "C3".to_string()));
        assert!(valid_range_comment.validate().is_ok());

        // 测试无效的单元格评论（无单元格引用）
        let invalid_cell_comment = Comment::new(
            "comment125".to_string(),
            "sheet123".to_string(),
            CommentType::Cell,
            content.clone(),
            author.clone()
        );
        assert!(invalid_cell_comment.validate().is_err());

        // 测试无效的范围评论（无范围）
        let invalid_range_comment = Comment::new(
            "comment126".to_string(),
            "sheet123".to_string(),
            CommentType::Range,
            content.clone(),
            author.clone()
        );
        assert!(invalid_range_comment.validate().is_err());
    }

    #[test]
    fn test_create_comment_request_creation() {
        let content = CommentContent::new("测试评论".to_string());
        let request = CreateCommentRequest::new(
            "token123".to_string(),
            "sheet123".to_string(),
            CommentType::Cell,
            content,
        )
        .cell_reference("A1".to_string());

        assert_eq!(request.spreadsheet_token, "token123");
        assert_eq!(request.sheet_id, "sheet123");
        assert_eq!(request.cell_reference, Some("A1".to_string()));
    }

    #[test]
    fn test_create_comment_request_validation() {
        use super::super::models::Range;

        // 测试有效请求
        let content = CommentContent::new("测试评论".to_string());
        let valid_request = CreateCommentRequest::new(
            "token123".to_string(),
            "sheet123".to_string(),
            CommentType::Cell,
            content,
        )
        .cell_reference("A1".to_string());
        assert!(valid_request.validate().is_ok());

        // 测试无效请求（空电子表格ID）
        let invalid_request1 = CreateCommentRequest::new(
            "".to_string(),
            "sheet123".to_string(),
            CommentType::Cell,
            CommentContent::new("测试".to_string()),
        )
        .cell_reference("A1".to_string());
        assert!(invalid_request1.validate().is_err());

        // 测试无效请求（空评论内容）
        let invalid_request2 = CreateCommentRequest::new(
            "token123".to_string(),
            "sheet123".to_string(),
            CommentType::Cell,
            CommentContent::new("".to_string()),
        )
        .cell_reference("A1".to_string());
        assert!(invalid_request2.validate().is_err());
    }

    #[test]
    fn test_create_comment_request_builder() {
        use super::super::models::Range;

        let content = CommentContent::new("请检查这个数据".to_string())
            .add_user_mention("user123".to_string(), "张三".to_string());

        let request = CreateCommentRequest::builder()
            .spreadsheet_token("token123".to_string())
            .sheet_id("sheet123".to_string())
            .comment_type(CommentType::Range)
            .range(Range::from("A1".to_string(), "C3".to_string()))
            .content(content)
            .build()
            .unwrap();

        assert_eq!(request.spreadsheet_token, "token123");
        assert_eq!(request.sheet_id, "sheet123");
        assert_eq!(request.comment_type, CommentType::Range);
        assert!(request.range.is_some());
        assert_eq!(request.content.text, "请检查这个数据");
        assert_eq!(request.content.mentions.len(), 1);
    }

    #[test]
    fn test_get_comments_request_creation() {
        let request = GetCommentsRequest::new("token123".to_string(), "sheet123".to_string())
            .page(0)
            .page_size(20)
            .status(CommentStatus::Active)
            .comment_type(CommentType::Cell);

        assert_eq!(request.spreadsheet_token, "token123");
        assert_eq!(request.sheet_id, "sheet123");
        assert_eq!(request.page, Some(0));
        assert_eq!(request.page_size, Some(20));
        assert_eq!(request.status, Some(CommentStatus::Active));
        assert_eq!(request.comment_type, Some(CommentType::Cell));
    }

    #[test]
    fn test_get_comments_request_validation() {
        // 测试有效请求
        let valid_request =
            GetCommentsRequest::new("token123".to_string(), "sheet123".to_string()).page_size(20);
        assert!(valid_request.validate().is_ok());

        // 测试无效页大小
        let invalid_request =
            GetCommentsRequest::new("token123".to_string(), "sheet123".to_string()).page_size(0);
        assert!(invalid_request.validate().is_err());

        let invalid_request2 =
            GetCommentsRequest::new("token123".to_string(), "sheet123".to_string()).page_size(101);
        assert!(invalid_request2.validate().is_err());
    }

    #[test]
    fn test_get_comments_request_builder() {
        let request = GetCommentsRequest::builder()
            .spreadsheet_token("token123".to_string())
            .sheet_id("sheet123".to_string())
            .page(1)
            .page_size(30)
            .status(CommentStatus::Resolved)
            .comment_type(CommentType::Range)
            .build()
            .unwrap();

        assert_eq!(request.spreadsheet_token, "token123");
        assert_eq!(request.sheet_id, "sheet123");
        assert_eq!(request.page, Some(1));
        assert_eq!(request.page_size, Some(30));
        assert_eq!(request.status, Some(CommentStatus::Resolved));
        assert_eq!(request.comment_type, Some(CommentType::Range));
    }

    #[test]
    fn test_delete_comment_request() {
        let request = DeleteCommentRequest::new("token123".to_string(), "comment123".to_string());

        assert_eq!(request.spreadsheet_token, "token123");
        assert_eq!(request.comment_id, "comment123");
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_delete_comment_request_builder() {
        let request = DeleteCommentRequest::builder()
            .spreadsheet_token("token123".to_string())
            .comment_id("comment123".to_string())
            .build()
            .unwrap();

        assert_eq!(request.spreadsheet_token, "token123");
        assert_eq!(request.comment_id, "comment123");
    }

    #[test]
    fn test_comment_service_creation() {
        let config = openlark_core::config::openlark_core::config::Config::default();
        let service = CommentService::new(config);
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_comprehensive_comment_scenarios() {
        use super::super::models::Range;

        // 测试单元格评论场景
        let cell_content = CommentContent::new("这个单元格需要更新".to_string())
            .add_user_mention("user123".to_string(), "张三".to_string());

        let cell_request = CreateCommentRequest::builder()
            .spreadsheet_token("token123".to_string())
            .sheet_id("sheet123".to_string())
            .comment_type(CommentType::Cell)
            .cell_reference("B5".to_string())
            .content(cell_content)
            .build()
            .unwrap();

        assert_eq!(cell_request.comment_type, CommentType::Cell);
        assert_eq!(cell_request.cell_reference, Some("B5".to_string()));

        // 测试范围评论场景
        let range_content = CommentContent::new("这个范围的数据需要审核".to_string())
            .add_user_mention("user456".to_string(), "李四".to_string())
            .add_user_mention("user789".to_string(), "王五".to_string());

        let range_request = CreateCommentRequest::builder()
            .spreadsheet_token("token123".to_string())
            .sheet_id("sheet123".to_string())
            .comment_type(CommentType::Range)
            .range(Range::from("A1".to_string(), "D10".to_string()))
            .content(range_content)
            .build()
            .unwrap();

        assert_eq!(range_request.comment_type, CommentType::Range);
        assert!(range_request.range.is_some());
        assert_eq!(range_request.content.mentions.len(), 2);

        // 测试评论查询场景
        let query_request = GetCommentsRequest::builder()
            .spreadsheet_token("token123".to_string())
            .sheet_id("sheet123".to_string())
            .page(0)
            .page_size(10)
            .status(CommentStatus::Active)
            .build()
            .unwrap();

        assert_eq!(query_request.page_size, Some(10));
        assert_eq!(query_request.status, Some(CommentStatus::Active));
    }

    #[test]
    fn test_comment_types() {
        let comment_types = vec![CommentType::Cell, CommentType::Range];

        for comment_type in comment_types {
            let request = CreateCommentRequest::new(
                "token123".to_string(),
                "sheet123".to_string(),
                comment_type,
                CommentContent::new("test".to_string()),
            );
            assert_eq!(request.sheet_id, "sheet123");
        }
    }

    #[test]
    fn test_comment_statuses() {
        let statuses = vec![
            CommentStatus::Active,
            CommentStatus::Resolved,
            CommentStatus::Deleted,
        ];

        for status in statuses {
            let request = GetCommentsRequest::new("token123".to_string(), "sheet123".to_string())
                .status(status);
            assert_eq!(request.sheet_id, "sheet123");
        }
    }

    #[test]
    fn test_comment_serialization() {
        use serde_json;

        let comment = Comment::new(
            "comment123".to_string(),
            "sheet123".to_string(),
            CommentType::Cell,
            CommentContent::new("test".to_string()),
            CommentAuthor::new("user123".to_string(), "张三".to_string()),
        );
        let json = serde_json::to_string(&comment).unwrap();
        assert!(json.contains("CELL"));
        assert!(json.contains("comment123"));
    }
}
