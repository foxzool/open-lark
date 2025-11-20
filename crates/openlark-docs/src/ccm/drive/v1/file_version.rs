use openlark_core::config::Config;
use openlark_core::error::SDKError;
use crate::response::SDKResult;
use crate::service_trait::Service;
use crate::transport::Transport;
use openlark_core::api::ApiRequest;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 获取文档版本信息API端点
pub const ENDPOINT_GET_FILE_VERSION: &str = "/open-apis/drive/v1/files/{}/versions/{}";

/// 删除文档版本API端点
pub const ENDPOINT_DELETE_FILE_VERSION: &str = "/open-apis/drive/v1/files/{}/versions/{}";

/// 获取文档版本信息请求体
///
/// 用于获取文件特定版本的详细信息，包括版本元数据、创建者信息、
/// 修改时间、文件大小等关键信息
#[derive(Clone, Debug)]
pub struct GetFileVersionRequest {
    /// 请求体
    #[serde(skip)]
    pub api_req: ApiRequest,
    /// 文件令牌
    /// 要查询的文件的唯一标识符
    pub file_token: String,
    /// 版本ID
    /// 要查询的特定版本的标识符
    pub version_id: String,
}

impl Default for GetFileVersionRequest {
    fn default() -> Self {
        Self {
            api_req: ApiRequest::default(),
            file_token: String::new(),
            version_id: String::new(),
        }
    }
}

impl GetFileVersionRequest {
    /// 创建新的获取文档版本信息请求
    ///
    /// # 参数
    /// * `file_token` - 文件令牌
    /// * `version_id` - 版本ID
    pub fn new(file_token: impl Into<String>, version_id: impl Into<String>) -> Self {
        Self {
            api_req: ApiRequest::default(),
            file_token: file_token.into(),
            version_id: version_id.into(),
        }
    }

    /// 创建获取文档版本信息请求的构建器
    pub fn builder() -> GetFileVersionBuilder {
        GetFileVersionBuilder::default()
    }

    /// 构建请求验证
    ///
    /// 验证请求参数的有效性
    ///
    /// # 返回
    /// 成功返回空值，失败返回错误信息
    pub fn build(&self) -> SDKResult<()> {
        // 验证文件令牌
        if self.file_token.trim().is_empty() {
            return Err(SDKError::ValidationError("文件令牌不能为空".to_string()));
        }

        // 验证版本ID
        if self.version_id.trim().is_empty() {
            return Err(SDKError::ValidationError("版本ID不能为空".to_string()));
        }

        Ok(())
    }
}

/// 获取文档版本信息请求构建器
///
/// 提供流畅的API来构建获取文档版本信息请求，支持方法链调用
#[derive(Debug, Clone, Default)]
pub struct GetFileVersionBuilder {
    file_token: Option<String>,
    version_id: Option<String>,
}

impl GetFileVersionBuilder {
    /// 创建新的获取文档版本信息构建器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置文件令牌
    ///
    /// # 参数
    /// * `file_token` - 文件令牌
    ///
    /// # 示例
    /// ```rust
    /// let builder = GetFileVersionBuilder::new()
    ///     .file_token("file_token_123");
    /// ```
    pub fn file_token(mut self, file_token: impl Into<String>) -> Self {
        self.file_token = Some(file_token.into());
        self
    }

    /// 设置版本ID
    ///
    /// # 参数
    /// * `version_id` - 版本ID
    ///
    /// # 示例
    /// ```rust
    /// let builder = GetFileVersionBuilder::new()
    ///     .version_id("version_456");
    /// ```
    pub fn version_id(mut self, version_id: impl Into<String>) -> Self {
        self.version_id = Some(version_id.into());
        self
    }

    /// 构建获取文档版本信息请求
    ///
    /// # 返回
    /// 成功返回获取文档版本信息请求，失败返回错误信息
    ///
    /// # 错误
    /// * 如果文件令牌为空，返回错误
    /// * 如果版本ID为空，返回错误
    pub fn build(self) -> SDKResult<GetFileVersionRequest> {
        // 验证必填参数
        if let Some(file_token) = &self.file_token {
            if file_token.trim().is_empty() {
                return Err(SDKError::ValidationError("文件令牌不能为空".to_string()));
            }
        } else {
            return Err(SDKError::ValidationError("文件令牌是必填参数".to_string()));
        }

        if let Some(version_id) = &self.version_id {
            if version_id.trim().is_empty() {
                return Err(SDKError::ValidationError("版本ID不能为空".to_string()));
            }
        } else {
            return Err(SDKError::ValidationError("版本ID是必填参数".to_string()));
        }

        Ok(GetFileVersionRequest {
            api_req: ApiRequest::default(),
            file_token: self.file_token.unwrap(),
            version_id: self.version_id.unwrap(),
        })
    }
}

/// 用户信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    /// 用户ID
    pub user_id: String,
    /// 用户姓名
    pub name: String,
    /// 用户邮箱
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 用户头像
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
}

impl UserInfo {
    /// 创建新的用户信息
    ///
    /// # 参数
    /// * `user_id` - 用户ID
    /// * `name` - 用户姓名
    pub fn new(user_id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            user_id: user_id.into(),
            name: name.into(),
            email: None,
            avatar: None,
        }
    }

    /// 设置邮箱
    pub fn with_email(mut self, email: impl Into<String>) -> Self {
        self.email = Some(email.into());
        self
    }

    /// 设置头像
    pub fn with_avatar(mut self, avatar: impl Into<String>) -> Self {
        self.avatar = Some(avatar.into());
        self
    }
}

/// 删除文档版本请求体
///
/// 用于删除文件的指定版本。删除操作不可逆，请谨慎使用。
/// 删除版本不会影响文件的其他版本，如果删除的是当前版本，
/// 系统会自动将最新版本设为当前版本。
#[derive(Clone, Debug)]
pub struct DeleteFileVersionRequest {
    /// 请求体
    #[serde(skip)]
    pub api_req: ApiRequest,
    /// 文件令牌
    /// 要删除版本的文件的唯一标识符
    pub file_token: String,
    /// 版本ID
    /// 要删除的特定版本的标识符
    pub version_id: String,
    /// 删除确认
    /// 为true时确认删除操作，防止误删除
    pub confirm: bool,
}

impl Default for DeleteFileVersionRequest {
    fn default() -> Self {
        Self {
            api_req: ApiRequest::default(),
            file_token: String::new(),
            version_id: String::new(),
            confirm: false,
        }
    }
}

impl DeleteFileVersionRequest {
    /// 创建新的删除文档版本请求
    ///
    /// # 参数
    /// * `file_token` - 文件令牌
    /// * `version_id` - 版本ID
    /// * `confirm` - 删除确认，必须为true才能执行删除
    pub fn new(file_token: impl Into<String>, version_id: impl Into<String>, confirm: bool) -> Self {
        Self {
            api_req: ApiRequest::default(),
            file_token: file_token.into(),
            version_id: version_id.into(),
            confirm,
        }
    }

    /// 创建删除文档版本请求的构建器
    pub fn builder() -> DeleteFileVersionBuilder {
        DeleteFileVersionBuilder::default()
    }

    /// 构建请求验证
    ///
    /// 验证请求参数的有效性，DELETE操作需要额外的安全验证
    ///
    /// # 返回
    /// 成功返回空值，失败返回错误信息
    pub fn build(&self) -> SDKResult<()> {
        // 验证文件令牌
        if self.file_token.trim().is_empty() {
            return Err(SDKError::ValidationError("文件令牌不能为空".to_string()));
        }

        // 验证版本ID
        if self.version_id.trim().is_empty() {
            return Err(SDKError::ValidationError("版本ID不能为空".to_string()));
        }

        // 验证删除确认 - 安全措施
        if !self.confirm {
            return Err(SDKError::ValidationError(
                "删除操作需要确认，请设置confirm为true".to_string(),
            ));
        }

        // 额外安全检查：防止删除自己
        if self.file_token == "self" || self.version_id == "self" {
            return Err(SDKError::ValidationError(
                "不允许使用'self'作为文件令牌或版本ID".to_string(),
            ));
        }

        Ok(())
    }
}

/// 删除文档版本请求构建器
///
/// 提供流畅的API来构建删除文档版本请求，支持方法链调用
/// 包含额外的安全验证措施防止误删除
#[derive(Debug, Clone, Default)]
pub struct DeleteFileVersionBuilder {
    file_token: Option<String>,
    version_id: Option<String>,
    confirm: Option<bool>,
}

impl DeleteFileVersionBuilder {
    /// 创建新的删除文档版本构建器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置文件令牌
    ///
    /// # 参数
    /// * `file_token` - 文件令牌
    ///
    /// # 示例
    /// ```rust
    /// let builder = DeleteFileVersionBuilder::new()
    ///     .file_token("file_token_123");
    /// ```
    pub fn file_token(mut self, file_token: impl Into<String>) -> Self {
        self.file_token = Some(file_token.into());
        self
    }

    /// 设置版本ID
    ///
    /// # 参数
    /// * `version_id` - 版本ID
    ///
    /// # 示例
    /// ```rust
    /// let builder = DeleteFileVersionBuilder::new()
    ///     .version_id("version_456");
    /// ```
    pub fn version_id(mut self, version_id: impl Into<String>) -> Self {
        self.version_id = Some(version_id.into());
        self
    }

    /// 设置删除确认
    ///
    /// # 参数
    /// * `confirm` - 删除确认，必须为true才能执行删除
    ///
    /// # 示例
    /// ```rust
    /// let builder = DeleteFileVersionBuilder::new()
    ///     .confirm(true);  // 必须显式确认
    /// ```
    pub fn confirm(mut self, confirm: bool) -> Self {
        self.confirm = Some(confirm);
        self
    }

    /// 构建删除文档版本请求
    ///
    /// # 返回
    /// 成功返回删除文档版本请求，失败返回错误信息
    ///
    /// # 错误
    /// * 如果文件令牌为空，返回错误
    /// * 如果版本ID为空，返回错误
    /// * 如果删除确认不为true，返回错误
    pub fn build(self) -> SDKResult<DeleteFileVersionRequest> {
        // 验证必填参数
        if let Some(file_token) = &self.file_token {
            if file_token.trim().is_empty() {
                return Err(SDKError::ValidationError("文件令牌不能为空".to_string()));
            }
        } else {
            return Err(SDKError::ValidationError("文件令牌是必填参数".to_string()));
        }

        if let Some(version_id) = &self.version_id {
            if version_id.trim().is_empty() {
                return Err(SDKError::ValidationError("版本ID不能为空".to_string()));
            }
        } else {
            return Err(SDKError::ValidationError("版本ID是必填参数".to_string()));
        }

        if let Some(confirm) = self.confirm {
            if !confirm {
                return Err(SDKError::ValidationError(
                    "删除操作需要确认，请设置confirm为true".to_string(),
                ));
            }
        } else {
            return Err(SDKError::ValidationError(
                "删除确认是必填参数，请设置confirm为true".to_string(),
            ));
        }

        Ok(DeleteFileVersionRequest {
            api_req: ApiRequest::default(),
            file_token: self.file_token.unwrap(),
            version_id: self.version_id.unwrap(),
            confirm: self.confirm.unwrap(),
        })
    }
}

/// 删除文档版本响应体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteFileVersionResponse {
    /// 是否删除成功
    pub success: bool,
    /// 删除的版本ID
    pub deleted_version_id: String,
    /// 文件令牌
    pub file_token: String,
    /// 删除时间
    pub deleted_at: String,
    /// 消息
    pub message: String,
    /// 当前版本ID（如果删除的是当前版本）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_version_id: Option<String>,
}

impl DeleteFileVersionResponse {
    /// 创建新的删除文档版本响应
    ///
    /// # 参数
    /// * `success` - 是否删除成功
    /// * `deleted_version_id` - 删除的版本ID
    /// * `file_token` - 文件令牌
    /// * `deleted_at` - 删除时间
    /// * `message` - 消息
    pub fn new(
        success: bool,
        deleted_version_id: impl Into<String>,
        file_token: impl Into<String>,
        deleted_at: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            success,
            deleted_version_id: deleted_version_id.into(),
            file_token: file_token.into(),
            deleted_at: deleted_at.into(),
            message: message.into(),
            current_version_id: None,
        }
    }

    /// 获取删除的版本ID
    pub fn deleted_version_id(&self) -> &str {
        &self.deleted_version_id
    }

    /// 获取文件令牌
    pub fn file_token(&self) -> &str {
        &self.file_token
    }

    /// 获取删除时间
    pub fn deleted_at(&self) -> &str {
        &self.deleted_at
    }

    /// 获取消息
    pub fn message(&self) -> &str {
        &self.message
    }

    /// 检查是否删除成功
    pub fn is_success(&self) -> bool {
        self.success
    }

    /// 获取当前版本ID
    pub fn current_version_id(&self) -> Option<&str> {
        self.current_version_id.as_deref()
    }

    /// 设置当前版本ID
    pub fn with_current_version_id(mut self, current_version_id: impl Into<String>) -> Self {
        self.current_version_id = Some(current_version_id.into());
        self
    }
}

/// 获取文档版本信息响应体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFileVersionResponse {
    /// 版本ID
    pub version_id: String,
    /// 文件令牌
    pub file_token: String,
    /// 文件名
    pub name: String,
    /// 文件大小（字节）
    pub size: i64,
    /// 创建时间
    pub created_at: String,
    /// 修改时间
    pub modified_at: String,
    /// 创建者信息
    pub creator: UserInfo,
    /// 修改者信息
    pub modifier: UserInfo,
    /// 版本号
    pub version_number: i32,
    /// 父版本ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_version_id: Option<String>,
    /// 版本描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 文件类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_type: Option<String>,
    /// 文件扩展名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_extension: Option<String>,
    /// MIME类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    /// 文件URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// 预览URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_url: Option<String>,
    /// 缩略图URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_url: Option<String>,
    /// 是否为当前版本
    pub is_current: bool,
    /// 版本状态
    pub status: String,
}

impl GetFileVersionResponse {
    /// 创建新的获取文档版本信息响应
    ///
    /// # 参数
    /// * `version_id` - 版本ID
    /// * `file_token` - 文件令牌
    /// * `name` - 文件名
    /// * `size` - 文件大小
    /// * `created_at` - 创建时间
    /// * `modified_at` - 修改时间
    /// * `creator` - 创建者信息
    /// * `modifier` - 修改者信息
    /// * `version_number` - 版本号
    pub fn new(
        version_id: impl Into<String>,
        file_token: impl Into<String>,
        name: impl Into<String>,
        size: i64,
        created_at: impl Into<String>,
        modified_at: impl Into<String>,
        creator: UserInfo,
        modifier: UserInfo,
        version_number: i32,
    ) -> Self {
        Self {
            version_id: version_id.into(),
            file_token: file_token.into(),
            name: name.into(),
            size,
            created_at: created_at.into(),
            modified_at: modified_at.into(),
            creator,
            modifier,
            version_number,
            parent_version_id: None,
            description: None,
            file_type: None,
            file_extension: None,
            mime_type: None,
            url: None,
            preview_url: None,
            thumbnail_url: None,
            is_current: false,
            status: "active".to_string(),
        }
    }

    /// 获取版本ID
    pub fn version_id(&self) -> &str {
        &self.version_id
    }

    /// 获取文件令牌
    pub fn file_token(&self) -> &str {
        &self.file_token
    }

    /// 获取文件名
    pub fn name(&self) -> &str {
        &self.name
    }

    /// 获取文件大小
    pub fn size(&self) -> i64 {
        self.size
    }

    /// 获取创建时间
    pub fn created_at(&self) -> &str {
        &self.created_at
    }

    /// 获取修改时间
    pub fn modified_at(&self) -> &str {
        &self.modified_at
    }

    /// 获取创建者信息
    pub fn creator(&self) -> &UserInfo {
        &self.creator
    }

    /// 获取修改者信息
    pub fn modifier(&self) -> &UserInfo {
        &self.modifier
    }

    /// 获取版本号
    pub fn version_number(&self) -> i32 {
        self.version_number
    }

    /// 检查是否为当前版本
    pub fn is_current(&self) -> bool {
        self.is_current
    }

    /// 获取版本状态
    pub fn status(&self) -> &str {
        &self.status
    }

    /// 获取文件URL
    pub fn url(&self) -> Option<&str> {
        self.url.as_deref()
    }

    /// 获取预览URL
    pub fn preview_url(&self) -> Option<&str> {
        self.preview_url.as_deref()
    }

    /// 获取缩略图URL
    pub fn thumbnail_url(&self) -> Option<&str> {
        self.thumbnail_url.as_deref()
    }

    /// 格式化文件大小
    ///
    /// # 返回
    /// 格式化后的文件大小字符串（如 "1.5 MB"）
    pub fn formatted_size(&self) -> String {
        const BYTES_PER_KB: f64 = 1024.0;
        const BYTES_PER_MB: f64 = 1024.0 * 1024.0;
        const BYTES_PER_GB: f64 = 1024.0 * 1024.0 * 1024.0;

        let size = self.size as f64;
        if size >= BYTES_PER_GB {
            format!("{:.2} GB", size / BYTES_PER_GB)
        } else if size >= BYTES_PER_MB {
            format!("{:.2} MB", size / BYTES_PER_MB)
        } else if size >= BYTES_PER_KB {
            format!("{:.2} KB", size / BYTES_PER_KB)
        } else {
            format!("{} B", self.size)
        }
    }
}

/// 获取文档版本信息构建器
///
/// 提供流畅的API来获取文档版本信息，支持方法链调用和完整的错误处理
#[derive(Clone, Debug)]
pub struct GetFileVersionBuilder {
    service: Arc<DriveServiceV1>,
    request: GetFileVersionRequest,
}

impl GetFileVersionBuilder {
    /// 创建新的获取文档版本信息构建器
    ///
    /// # 参数
    /// * `service` - 云盘服务实例
    /// * `request` - 获取文档版本信息请求
    pub(crate) fn new(service: Arc<DriveServiceV1>, request: GetFileVersionRequest) -> Self {
        Self { service, request }
    }

    /// 执行获取文档版本信息操作
    ///
    /// 向飞书API发送GET请求来获取文档版本信息
    ///
    /// # 返回
    /// * `Ok(GetFileVersionResponse)` - 获取成功，返回版本信息
    /// * `Err(SDKError)` - 获取失败，返回错误信息
    ///
    /// # 错误类型
    /// * `SDKError::NetworkError` - 网络请求失败
    /// * `SDKError::ApiError` - API调用失败，包含错误码和消息
    /// * `SDKError::SerializationError` - 响应序列化失败
    /// * `SDKError::AuthenticationError` - 身份验证失败
    ///
    /// # 示例
    /// ```rust,no_run
    /// use open_lark::service::cloud_docs::drive::v1::file_version::{GetFileVersionRequest, GetFileVersionResponse};
    ///
    /// async fn get_file_version_example(
    ///     service: Arc<DriveServiceV1>,
    /// ) -> Result<GetFileVersionResponse, Box<dyn std::error::Error>> {
    ///     let request = GetFileVersionRequest::builder()
    ///         .file_token("file_token_123")
    ///         .version_id("version_456")
    ///         .build()?;
    ///
    ///     let response = service
    ///         .get_file_version_builder(request)
    ///         .execute()
    ///         .await?;
    ///
    ///     println!("获取版本信息成功，版本号: {}", response.version_number());
    ///     Ok(response)
    /// }
    /// ```
    pub async fn execute(self) -> SDKResult<GetFileVersionResponse> {
        // 验证请求参数
        self.request.build()?;

        // 构建API请求
        let mut api_req = self.request.api_req;
        api_req.set_http_method(Method::GET);

        // 替换路径参数
        let endpoint = ENDPOINT_GET_FILE_VERSION
            .replace("{}", &self.request.file_token)
            .replace("{}", &self.request.version_id);
        api_req.set_api_path(endpoint);

        // 设置支持的访问令牌类型
        use constants::AccessTokenType;
        api_req.set_supported_access_token_types(vec![
            AccessTokenType::Tenant,
            AccessTokenType::User,
        ]);

        // 发送HTTP GET请求
        let api_resp: api::Response<GetFileVersionResponse> =
            self.service.transport().request(api_req, self.service.config(), None).await?;

        // 解析响应
        match api_resp.into_result() {
            Ok(response) => Ok(response),
            Err(e) => Err(e),
        }
    }
}

/// 删除文档版本构建器
///
/// 提供流畅的API来删除文档版本，支持方法链调用和完整的错误处理
/// 包含额外的安全验证措施防止误删除
#[derive(Clone, Debug)]
pub struct DeleteFileVersionBuilder {
    service: Arc<DriveServiceV1>,
    request: DeleteFileVersionRequest,
}

impl DeleteFileVersionBuilder {
    /// 创建新的删除文档版本构建器
    ///
    /// # 参数
    /// * `service` - 云盘服务实例
    /// * `request` - 删除文档版本请求
    pub(crate) fn new(service: Arc<DriveServiceV1>, request: DeleteFileVersionRequest) -> Self {
        Self { service, request }
    }

    /// 执行删除文档版本操作
    ///
    /// 向飞书API发送DELETE请求来删除文档版本
    ///
    /// # 返回
    /// * `Ok(DeleteFileVersionResponse)` - 删除成功，返回删除结果
    /// * `Err(SDKError)` - 删除失败，返回错误信息
    ///
    /// # 错误类型
    /// * `SDKError::NetworkError` - 网络请求失败
    /// * `SDKError::ApiError` - API调用失败，包含错误码和消息
    /// * `SDKError::SerializationError` - 响应序列化失败
    /// * `SDKError::AuthenticationError` - 身份验证失败
    /// * `SDKError::ValidationError` - 参数验证失败
    ///
    /// # 安全措施
    /// * 需要显式确认删除操作（confirm=true）
    /// * 验证文件令牌和版本ID的有效性
    /// * 防止使用'self'作为参数
    ///
    /// # 示例
    /// ```rust,no_run
    /// use open_lark::service::cloud_docs::drive::v1::file_version::{DeleteFileVersionRequest, DeleteFileVersionResponse};
    ///
    /// async fn delete_file_version_example(
    ///     service: Arc<DriveServiceV1>,
    /// ) -> Result<DeleteFileVersionResponse, Box<dyn std::error::Error>> {
    ///     let request = DeleteFileVersionRequest::builder()
    ///         .file_token("file_token_123")
    ///         .version_id("version_456")
    ///         .confirm(true)  // 必须显式确认删除
    ///         .build()?;
    ///
    ///     let response = service
    ///         .delete_file_version_builder(request)
    ///         .execute()
    ///         .await?;
    ///
    ///     println!("删除版本成功，删除时间: {}", response.deleted_at());
    ///     Ok(response)
    /// }
    /// ```
    pub async fn execute(self) -> SDKResult<DeleteFileVersionResponse> {
        // 验证请求参数（包括安全验证）
        self.request.build()?;

        // 构建API请求
        let mut api_req = self.request.api_req;
        api_req.set_http_method(Method::DELETE);

        // 替换路径参数
        let endpoint = ENDPOINT_DELETE_FILE_VERSION
            .replace("{}", &self.request.file_token)
            .replace("{}", &self.request.version_id);
        api_req.set_api_path(endpoint);

        // 设置支持的访问令牌类型
        use constants::AccessTokenType;
        api_req.set_supported_access_token_types(vec![
            AccessTokenType::Tenant,
            AccessTokenType::User,
        ]);

        // 发送HTTP DELETE请求
        let api_resp: api::Response<DeleteFileVersionResponse> =
            self.service.transport().request(api_req, self.service.config(), None).await?;

        // 解析响应
        match api_resp.into_result() {
            Ok(response) => Ok(response),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_file_version_request_new() {
        let request = GetFileVersionRequest::new("file_token_123", "version_456");

        assert_eq!(request.file_token, "file_token_123");
        assert_eq!(request.version_id, "version_456");
    }

    #[test]
    fn test_get_file_version_request_builder() {
        let request = GetFileVersionRequest::builder()
            .file_token("file_token_123")
            .version_id("version_456")
            .build()
            .unwrap();

        assert_eq!(request.file_token, "file_token_123");
        assert_eq!(request.version_id, "version_456");
    }

    #[test]
    fn test_get_file_version_request_validation() {
        // 测试空文件令牌
        let result = GetFileVersionRequest::builder()
            .file_token("")
            .version_id("version_456")
            .build();
        assert!(result.is_err());

        // 测试空版本ID
        let result = GetFileVersionRequest::builder()
            .file_token("file_token_123")
            .version_id("")
            .build();
        assert!(result.is_err());

        // 测试缺少参数
        let result = GetFileVersionRequest::builder().build();
        assert!(result.is_err());
    }

    #[test]
    fn test_user_info_creation() {
        let user = UserInfo::new("user_123", "张三")
            .with_email("zhangsan@example.com")
            .with_avatar("https://example.com/avatar.jpg");

        assert_eq!(user.user_id, "user_123");
        assert_eq!(user.name, "张三");
        assert_eq!(user.email, Some("zhangsan@example.com".to_string()));
        assert_eq!(user.avatar, Some("https://example.com/avatar.jpg".to_string()));
    }

    #[test]
    fn test_get_file_version_response_creation() {
        let creator = UserInfo::new("user_123", "张三");
        let modifier = UserInfo::new("user_456", "李四");

        let response = GetFileVersionResponse::new(
            "version_123",
            "file_token_456",
            "测试文档.pdf",
            1024 * 1024, // 1MB
            "2024-01-01T00:00:00Z",
            "2024-01-02T00:00:00Z",
            creator,
            modifier,
            1,
        );

        assert_eq!(response.version_id(), "version_123");
        assert_eq!(response.file_token(), "file_token_456");
        assert_eq!(response.name(), "测试文档.pdf");
        assert_eq!(response.size(), 1024 * 1024);
        assert_eq!(response.version_number(), 1);
        assert!(!response.is_current());
        assert_eq!(response.status(), "active");
    }

    #[test]
    fn test_formatted_size() {
        let response = GetFileVersionResponse::new(
            "version_123",
            "file_token_456",
            "test.txt",
            0, // size will be overridden
            "2024-01-01T00:00:00Z",
            "2024-01-02T00:00:00Z",
            UserInfo::new("user_123", "张三"),
            UserInfo::new("user_456", "李四"),
            1,
        );

        // 测试字节
        let response_bytes = GetFileVersionResponse {
            size: 512,
            ..response.clone()
        };
        assert_eq!(response_bytes.formatted_size(), "512 B");

        // 测试KB
        let response_kb = GetFileVersionResponse {
            size: 1024 * 2,
            ..response.clone()
        };
        assert_eq!(response_kb.formatted_size(), "2.00 KB");

        // 测试MB
        let response_mb = GetFileVersionResponse {
            size: 1024 * 1024 * 3,
            ..response.clone()
        };
        assert_eq!(response_mb.formatted_size(), "3.00 MB");

        // 测试GB
        let response_gb = GetFileVersionResponse {
            size: 1024 * 1024 * 1024 * 2,
            ..response.clone()
        };
        assert_eq!(response_gb.formatted_size(), "2.00 GB");
    }

    #[test]
    fn test_get_file_version_builder_defaults() {
        let builder = GetFileVersionBuilder::default();
        assert!(builder.file_token.is_none());
        assert!(builder.version_id.is_none());
    }

    #[test]
    fn test_valid_request_combinations() {
        let test_cases = vec![
            ("file_token_1", "version_1"),
            ("file_token_2", "version_2"),
            ("file_token_3", "version_3"),
        ];

        for (file_token, version_id) in test_cases {
            let request = GetFileVersionRequest::builder()
                .file_token(file_token)
                .version_id(version_id)
                .build()
                .unwrap();

            assert_eq!(request.file_token, file_token);
            assert_eq!(request.version_id, version_id);
        }
    }

    #[test]
    fn test_edge_cases() {
        // 测试长字符串
        let long_file_token = "a".repeat(1000);
        let long_version_id = "b".repeat(500);

        let request = GetFileVersionRequest::builder()
            .file_token(&long_file_token)
            .version_id(&long_version_id)
            .build();
        assert!(request.is_ok());

        // 测试特殊字符
        let special_chars = "!@#$%^&*()_+-=[]{}|;':\",./<>?";
        let request = GetFileVersionRequest::builder()
            .file_token(special_chars)
            .version_id(special_chars)
            .build();
        assert!(request.is_ok());
    }

    #[test]
    fn test_user_info_edge_cases() {
        // 测试空姓名
        let user = UserInfo::new("user_123", "");
        assert_eq!(user.name, "");

        // 测试特殊字符用户名
        let special_name = "用户@#$%";
        let user = UserInfo::new("user_123", special_name);
        assert_eq!(user.name, special_name);
    }

    #[test]
    fn test_response_optional_fields() {
        let creator = UserInfo::new("user_123", "张三");
        let modifier = UserInfo::new("user_456", "李四");

        let mut response = GetFileVersionResponse::new(
            "version_123",
            "file_token_456",
            "test.pdf",
            1024,
            "2024-01-01T00:00:00Z",
            "2024-01-02T00:00:00Z",
            creator,
            modifier,
            1,
        );

        // 设置可选字段
        response.parent_version_id = Some("parent_123".to_string());
        response.description = Some("测试版本".to_string());
        response.file_type = Some("pdf".to_string());
        response.url = Some("https://example.com/file.pdf".to_string());

        assert_eq!(response.parent_version_id, Some("parent_123".to_string()));
        assert_eq!(response.description, Some("测试版本".to_string()));
        assert_eq!(response.file_type, Some("pdf".to_string()));
        assert_eq!(response.url(), Some("https://example.com/file.pdf"));
    }

    #[test]
    fn test_endpoint_constant() {
        assert_eq!(
            ENDPOINT_GET_FILE_VERSION,
            "/open-apis/drive/v1/files/{}/versions/{}"
        );
    }

    #[test]
    fn test_request_build_validation() {
        let request = GetFileVersionRequest::new("file_token_123", "version_456");
        assert!(request.build().is_ok());

        let empty_request = GetFileVersionRequest::new("", "");
        assert!(empty_request.build().is_err());
    }

    #[test]
    fn test_response_status_methods() {
        let creator = UserInfo::new("user_123", "张三");
        let modifier = UserInfo::new("user_456", "李四");

        let mut response = GetFileVersionResponse::new(
            "version_123",
            "file_token_456",
            "test.pdf",
            1024,
            "2024-01-01T00:00:00Z",
            "2024-01-02T00:00:00Z",
            creator,
            modifier,
            1,
        );

        // 测试当前版本状态
        assert!(!response.is_current());
        response.is_current = true;
        assert!(response.is_current());

        // 测试状态
        assert_eq!(response.status(), "active");
        response.status = "archived".to_string();
        assert_eq!(response.status(), "archived");
    }

    #[test]
    fn test_builder_error_messages() {
        // 测试空文件令牌错误消息
        let result = GetFileVersionRequest::builder()
            .file_token("")
            .version_id("version_456")
            .build();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("文件令牌不能为空"));

        // 测试空版本ID错误消息
        let result = GetFileVersionRequest::builder()
            .file_token("file_token_123")
            .version_id("")
            .build();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("版本ID不能为空"));

        // 测试缺少参数错误消息
        let result = GetFileVersionRequest::builder().build();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("必填参数"));
    }

    // ========== DELETE功能测试 ==========

    #[test]
    fn test_delete_file_version_request_new() {
        let request = DeleteFileVersionRequest::new("file_token_123", "version_456", true);

        assert_eq!(request.file_token, "file_token_123");
        assert_eq!(request.version_id, "version_456");
        assert_eq!(request.confirm, true);
    }

    #[test]
    fn test_delete_file_version_request_builder() {
        let request = DeleteFileVersionRequest::builder()
            .file_token("file_token_123")
            .version_id("version_456")
            .confirm(true)
            .build()
            .unwrap();

        assert_eq!(request.file_token, "file_token_123");
        assert_eq!(request.version_id, "version_456");
        assert_eq!(request.confirm, true);
    }

    #[test]
    fn test_delete_file_version_request_validation() {
        // 测试空文件令牌
        let result = DeleteFileVersionRequest::builder()
            .file_token("")
            .version_id("version_456")
            .confirm(true)
            .build();
        assert!(result.is_err());

        // 测试空版本ID
        let result = DeleteFileVersionRequest::builder()
            .file_token("file_token_123")
            .version_id("")
            .confirm(true)
            .build();
        assert!(result.is_err());

        // 测试未确认删除
        let result = DeleteFileVersionRequest::builder()
            .file_token("file_token_123")
            .version_id("version_456")
            .confirm(false)
            .build();
        assert!(result.is_err());

        // 测试缺少确认参数
        let result = DeleteFileVersionRequest::builder()
            .file_token("file_token_123")
            .version_id("version_456")
            .build();
        assert!(result.is_err());

        // 测试缺少参数
        let result = DeleteFileVersionRequest::builder().build();
        assert!(result.is_err());
    }

    #[test]
    fn test_delete_file_version_request_security_validation() {
        // 测试安全检查：不允许使用'self'
        let result = DeleteFileVersionRequest::builder()
            .file_token("self")
            .version_id("version_456")
            .confirm(true)
            .build();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("不允许使用'self'"));

        let result = DeleteFileVersionRequest::builder()
            .file_token("file_token_123")
            .version_id("self")
            .confirm(true)
            .build();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("不允许使用'self'"));
    }

    #[test]
    fn test_delete_file_version_response_creation() {
        let response = DeleteFileVersionResponse::new(
            true,
            "version_123",
            "file_token_456",
            "2024-01-01T00:00:00Z",
            "删除成功",
        );

        assert!(response.is_success());
        assert_eq!(response.deleted_version_id(), "version_123");
        assert_eq!(response.file_token(), "file_token_456");
        assert_eq!(response.deleted_at(), "2024-01-01T00:00:00Z");
        assert_eq!(response.message(), "删除成功");
        assert_eq!(response.current_version_id(), None);
    }

    #[test]
    fn test_delete_file_version_response_with_current_version() {
        let response = DeleteFileVersionResponse::new(
            true,
            "version_123",
            "file_token_456",
            "2024-01-01T00:00:00Z",
            "删除成功，已更新当前版本",
        )
        .with_current_version_id("version_789");

        assert!(response.is_success());
        assert_eq!(response.current_version_id(), Some("version_789"));
    }

    #[test]
    fn test_delete_file_version_builder_defaults() {
        let builder = DeleteFileVersionBuilder::default();
        assert!(builder.file_token.is_none());
        assert!(builder.version_id.is_none());
        assert!(builder.confirm.is_none());
    }

    #[test]
    fn test_delete_file_version_builder_chaining() {
        let request = DeleteFileVersionBuilder::new()
            .file_token("file_token_123")
            .version_id("version_456")
            .confirm(true)
            .build()
            .unwrap();

        assert_eq!(request.file_token, "file_token_123");
        assert_eq!(request.version_id, "version_456");
        assert_eq!(request.confirm, true);
    }

    #[test]
    fn test_delete_file_version_error_messages() {
        // 测试空文件令牌错误消息
        let result = DeleteFileVersionRequest::builder()
            .file_token("")
            .version_id("version_456")
            .confirm(true)
            .build();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("文件令牌不能为空"));

        // 测试空版本ID错误消息
        let result = DeleteFileVersionRequest::builder()
            .file_token("file_token_123")
            .version_id("")
            .confirm(true)
            .build();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("版本ID不能为空"));

        // 测试删除确认错误消息
        let result = DeleteFileVersionRequest::builder()
            .file_token("file_token_123")
            .version_id("version_456")
            .confirm(false)
            .build();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("删除操作需要确认"));

        // 测试缺少确认参数错误消息
        let result = DeleteFileVersionRequest::builder()
            .file_token("file_token_123")
            .version_id("version_456")
            .build();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("删除确认是必填参数"));
    }

    #[test]
    fn test_delete_file_version_edge_cases() {
        // 测试长字符串
        let long_file_token = "a".repeat(1000);
        let long_version_id = "b".repeat(500);

        let request = DeleteFileVersionRequest::builder()
            .file_token(&long_file_token)
            .version_id(&long_version_id)
            .confirm(true)
            .build();
        assert!(request.is_ok());

        // 测试特殊字符
        let special_chars = "!@#$%^&*()_+-=[]{}|;':\",./<>?";
        let request = DeleteFileVersionRequest::builder()
            .file_token(special_chars)
            .version_id(special_chars)
            .confirm(true)
            .build();
        assert!(request.is_ok());
    }

    #[test]
    fn test_delete_endpoint_constant() {
        assert_eq!(
            ENDPOINT_DELETE_FILE_VERSION,
            "/open-apis/drive/v1/files/{}/versions/{}"
        );
    }

    #[test]
    fn test_delete_request_build_validation() {
        let request = DeleteFileVersionRequest::new("file_token_123", "version_456", true);
        assert!(request.build().is_ok());

        let empty_request = DeleteFileVersionRequest::new("", "", false);
        assert!(empty_request.build().is_err());

        let no_confirm_request = DeleteFileVersionRequest::new("file_token_123", "version_456", false);
        assert!(no_confirm_request.build().is_err());
    }

    #[test]
    fn test_delete_response_methods() {
        let mut response = DeleteFileVersionResponse::new(
            false,
            "version_123",
            "file_token_456",
            "2024-01-01T00:00:00Z",
            "删除失败",
        );

        // 测试失败状态
        assert!(!response.is_success());
        assert_eq!(response.message(), "删除失败");

        // 测试成功状态
        response.success = true;
        response.message = "删除成功".to_string();
        assert!(response.is_success());
        assert_eq!(response.message(), "删除成功");
    }

    #[test]
    fn test_valid_delete_request_combinations() {
        let test_cases = vec![
            ("file_token_1", "version_1"),
            ("file_token_2", "version_2"),
            ("file_token_3", "version_3"),
        ];

        for (file_token, version_id) in test_cases {
            let request = DeleteFileVersionRequest::builder()
                .file_token(file_token)
                .version_id(version_id)
                .confirm(true)
                .build()
                .unwrap();

            assert_eq!(request.file_token, file_token);
            assert_eq!(request.version_id, version_id);
            assert!(request.confirm);
        }
    }

    #[test]
    fn test_delete_request_default() {
        let request = DeleteFileVersionRequest::default();
        assert!(request.file_token.is_empty());
        assert!(request.version_id.is_empty());
        assert!(!request.confirm);
    }
}