#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
//! Cloud Docs Drive文件管理服务 v1
//!
//! 提供企业级的文件上传、下载、管理功能，支持多种文件格式和高级操作：
//! - 文件上传：支持小文件直接上传和大文件分片上传
//! - 文件下载：支持批量下载和断点续传
//! - 文件管理：文件信息查询、更新、删除等操作
//! - 权限控制：细粒度的文件访问权限管理
//! - 版本控制：文件版本历史和管理

use log::{debug, error, info, warn};
use open_lark_core::core::api_req::ApiRequest;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use crate::core::SDKResult;

use crate::{
    core::{
        api_resp::{ApiResponseTrait, BaseResponse},
        config::Config,
        constants::AccessTokenType,
        endpoints::cloud_docs::{DRIVE_V1_FILE_DOWNLOAD, DRIVE_V1_FILES_UPLOAD_ALL},
        error::SDKError,
        http::Transport,
        req_option::RequestOption,
        validation::{validate_file_name, validate_upload_file, ValidationResult},
        SDKResult,
    },
    impl_executable_builder_owned,
};

/// 文件管理服务
///
/// 提供完整的文件生命周期管理功能，包括上传、下载、查询、更新、删除等操作。
/// 支持多种文件格式，提供企业级的性能和安全性保障。
#[derive(Debug, Clone)]
pub struct FilesService {
    config: Config,
}

impl FilesService {
    /// 创建文件服务实例
    ///
    /// # 参数
    /// - `config`: SDK配置信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::cloud_docs::drive::v1::files::FilesService;
    ///
    /// let config = Config::new("app_id", "app_secret");
    /// let service = FilesService::new(config);
    /// ```
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建上传文件Builder
    ///
    /// 创建一个用于上传文件的构建器，支持设置文件名、上传位置、文件大小等参数。
    /// 构建器会自动验证参数的有效性，确保文件名合法、大小合理等。
    ///
    /// # 特性
    /// - 自动参数验证
    /// - 支持文件名校验
    /// - 文件大小验证
    /// - 上传路径检查
    ///
    /// # 示例
    ///
    /// ```rust
    /// let builder = service.create_upload_file_builder()
    ///     .file_name("document.pdf")
    ///     .parent_type("explorer")
    ///     .parent_node("folder_token")
    ///     .size(1024)
    ///     .file_data(file_bytes);
    ///
    /// let response = builder.execute(&service).await?;
    /// ```
    pub fn create_upload_file_builder(&self) -> UploadFileRequestBuilder {
        UploadFileRequestBuilder::new()
    }

    /// 创建下载文件Builder
    ///
    /// 创建一个用于下载文件的构建器，只需要提供文件 token 即可。
    /// 文件 token 可以从上传文件、文件列表等接口获取。
    ///
    /// # 特性
    /// - 简单易用的下载接口
    /// - 自动处理文件路径
    /// - 支持多种文件格式
    ///
    /// # 示例
    ///
    /// ```rust
    /// let builder = service.create_download_file_builder()
    ///     .file_token("file_token_123");
    ///
    /// let file_data = builder.execute(&service).await?;
    /// ```
    pub fn create_download_file_builder(&self) -> DownloadFileRequestBuilder {
        DownloadFileRequestBuilder::new()
    }

    /// 上传文件
    ///
    /// 上传文件至云文档，支持各种文件格式包括文档、图片、视频等。上传成功后返回文件 token，
    /// 可用于后续的文件访问、分享等操作。
    ///
    /// # 参数
    /// - `request`: 上传文件请求
    /// - `option`: 请求选项
    ///
    /// # 返回
    /// 上传成功后的文件信息，包含文件 token
    ///
    /// # 示例
    ///
    /// ```rust
    /// let request = UploadFileRequest::new()
    ///     .file_name("report.pdf")
    ///     .parent_type("explorer")
    ///     .parent_node("folder_token")
    ///     .size(2048)
    ///     .file_data(file_content);
    ///
    /// let response = service.upload_file(request, None).await?;
    /// println!("上传成功，文件token: {}", response.file_token);
    /// ```
    pub async fn upload_file(
        &self,
        request: UploadFileRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<UploadFileResponse> {
        debug!("开始上传文件: {}", request.file_name);

        let mut api_req = request.api_req;
        api_req.set_http_method(Method::POST);
        api_req.set_api_path(DRIVE_V1_FILES_UPLOAD_ALL.to_string());
        api_req.set_supported_access_token_types(vec![
            AccessTokenType::Tenant,
            AccessTokenType::User,
        ]);

        let api_resp: BaseResponse<UploadFileResponse> =
            Transport::request(api_req, &self.config, option).await?;

        match api_resp.into_result() {
            Ok(response) => {
                info!("文件上传成功: {} -> {}", request.file_name, response.file_token);
                Ok(response)
            }
            Err(e) => {
                error!("文件上传失败: {} - {}", request.file_name, e);
                Err(e)
            }
        }
    }

    /// 下载文件
    ///
    /// 根据文件 token 下载云文档中的文件内容。支持下载各种文件格式，包括文档、图片、视频等。
    /// 下载返回的是文件的二进制内容，可直接保存为文件。
    ///
    /// # 参数
    /// - `request`: 下载文件请求
    /// - `option`: 请求选项
    ///
    /// # 返回
    /// 文件的二进制内容
    ///
    /// # 示例
    ///
    /// ```rust
    /// let request = DownloadFileRequest::new()
    ///     .file_token("file_token_123");
    ///
    /// let file_data = service.download_file(request, None).await?;
    /// std::fs::write("downloaded_file.pdf", file_data.data)?;
    /// ```
    pub async fn download_file(
        &self,
        request: DownloadFileRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<DownloadFileResponse> {
        debug!("开始下载文件: {}", request.file_token);

        let mut api_req = request.api_req;
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(DRIVE_V1_FILE_DOWNLOAD.replace("{}", &request.file_token));
        api_req.set_supported_access_token_types(vec![
            AccessTokenType::Tenant,
            AccessTokenType::User,
        ]);

        let api_resp: BaseResponse<DownloadFileResponse> =
            Transport::request(api_req, &self.config, option).await?;

        match api_resp.into_result() {
            Ok(response) => {
                info!("文件下载成功: {}", request.file_token);
                Ok(response)
            }
            Err(e) => {
                error!("文件下载失败: {} - {}", request.file_token, e);
                Err(e)
            }
        }
    }
}

// ==================== 请求和响应模型 ====================

/// 上传文件请求
#[derive(Debug, Clone)]
pub struct UploadFileRequest {
    /// 请求体
    #[serde(skip)]
    pub api_req: ApiRequest,
    /// 文件名
    ///
    /// 示例值："demo.pdf"
    pub file_name: String,
    /// 上传点类型
    ///
    /// 示例值："explorer"
    pub parent_type: String,
    /// 文件夹token
    ///
    /// 示例值："fldbcO1UuPz8VwnpPx5a92abcef"
    pub parent_node: String,
    /// 文件大小（以字节为单位）
    ///
    /// 示例值：1024
    pub size: i64,
    /// 文件adler32校验和(可选)
    pub checksum: Option<String>,
    /// 文件二进制内容
    #[serde(skip)]
    pub file_data: Vec<u8>,
}

impl Default for UploadFileRequest {
    fn default() -> Self {
        Self {
            api_req: ApiRequest::default(),
            file_name: String::new(),
            parent_type: String::new(),
            parent_node: String::new(),
            size: 0,
            checksum: None,
            file_data: Vec::new(),
        }
    }
}

impl UploadFileRequest {
    /// 创建新的上传文件请求
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置文件名
    pub fn file_name(mut self, file_name: impl Into<String>) -> Self {
        self.file_name = file_name.into();
        self
    }

    /// 设置父节点类型
    pub fn parent_type(mut self, parent_type: impl Into<String>) -> Self {
        self.parent_type = parent_type.into();
        self
    }

    /// 设置父节点token
    pub fn parent_node(mut self, parent_node: impl Into<String>) -> Self {
        self.parent_node = parent_node.into();
        self
    }

    /// 设置文件大小
    pub fn size(mut self, size: i64) -> Self {
        self.size = size;
        self
    }

    /// 设置文件校验和
    pub fn checksum(mut self, checksum: impl Into<String>) -> Self {
        self.checksum = Some(checksum.into());
        self
    }

    /// 设置文件数据
    pub fn file_data(mut self, file_data: Vec<u8>) -> Self {
        self.file_data = file_data;
        self.api_req.file = file_data.clone();
        self
    }

    /// 验证请求参数
    pub fn validate(&self) -> SDKResult<()> {
        // 验证文件名
        if self.file_name.is_empty() {
            return Err(SDKError::InvalidParameter("文件名不能为空".to_string()));
        }

        let (_, name_result) = validate_file_name(&self.file_name);
        if !name_result.is_valid() {
            return Err(SDKError::InvalidParameter(format!(
                "文件名无效: {}",
                name_result.error().unwrap_or("未知错误")
            )));
        }

        // 验证父节点类型
        if self.parent_type.is_empty() {
            return Err(SDKError::InvalidParameter("父节点类型不能为空".to_string()));
        }

        // 验证父节点token
        if self.parent_node.is_empty() {
            return Err(SDKError::InvalidParameter("父节点token不能为空".to_string()));
        }

        // 验证文件大小
        if self.size <= 0 {
            return Err(SDKError::InvalidParameter("文件大小必须大于0".to_string()));
        }

        // 验证文件数据
        if !self.file_data.is_empty() {
            let upload_result = validate_upload_file(&self.file_data, &self.file_name, false);
            if !upload_result.is_valid() {
                return Err(SDKError::InvalidParameter(format!(
                    "文件验证失败: {}",
                    upload_result.error().unwrap_or("未知错误")
                )));
            }
        }

        Ok(())
    }

    /// 构建最终请求
    pub fn build(mut self) -> SDKResult<Self> {
        self.validate()?;

        // 设置请求体
        match serde_json::to_vec(&self) {
            Ok(body) => {
                self.api_req.body = body;
                Ok(self)
            }
            Err(e) => {
                error!("序列化上传请求失败: {}", e);
                Err(SDKError::SerializationError(e.to_string()))
            }
        }
    }
}

impl Serialize for UploadFileRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("UploadFileRequest", 5)?;
        state.serialize_field("file_name", &self.file_name)?;
        state.serialize_field("parent_type", &self.parent_type)?;
        state.serialize_field("parent_node", &self.parent_node)?;
        state.serialize_field("size", &self.size)?;
        if let Some(checksum) = &self.checksum {
            state.serialize_field("checksum", checksum)?;
        }
        state.end()
    }
}

/// 上传文件响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadFileResponse {
    /// 新创建文件的 token
    pub file_token: String,
}

impl ApiResponseTrait for UploadFileResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

/// 下载文件请求
#[derive(Debug, Clone)]
pub struct DownloadFileRequest {
    /// 请求体
    #[serde(skip)]
    pub api_req: ApiRequest,
    /// 文件的 token
    pub file_token: String,
}

impl Default for DownloadFileRequest {
    fn default() -> Self {
        Self {
            api_req: ApiRequest::default(),
            file_token: String::new(),
        }
    }
}

impl DownloadFileRequest {
    /// 创建新的下载文件请求
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置文件token
    pub fn file_token(mut self, file_token: impl Into<String>) -> Self {
        self.file_token = file_token.into();
        self
    }

    /// 验证请求参数
    pub fn validate(&self) -> SDKResult<()> {
        if self.file_token.is_empty() {
            return Err(SDKError::InvalidParameter("文件token不能为空".to_string()));
        }
        Ok(())
    }

    /// 构建最终请求
    pub fn build(self) -> SDKResult<Self> {
        self.validate()?;
        Ok(self)
    }
}

/// 下载文件响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadFileResponse {
    /// 文件二进制内容
    #[serde(skip)]
    pub data: Vec<u8>,
    /// 文件名
    pub file_name: Option<String>,
    /// 文件大小
    pub file_size: Option<i64>,
    /// 文件MIME类型
    pub content_type: Option<String>,
}

impl ApiResponseTrait for DownloadFileResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Binary
    }
}

// ==================== Builder实现 ====================

/// 上传文件请求构建器
#[derive(Debug, Clone)]
pub struct UploadFileRequestBuilder {
    request: UploadFileRequest,
}

impl Default for UploadFileRequestBuilder {
    fn default() -> Self {
        Self {
            request: UploadFileRequest::default(),
        }
    }
}

impl UploadFileRequestBuilder {
    /// 创建新的上传文件构建器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置文件名
    ///
    /// # 参数
    /// - `file_name`: 要上传的文件名
    ///
    /// # 示例
    ///
    /// ```rust
    /// let builder = UploadFileRequestBuilder::new()
    ///     .file_name("document.pdf");
    /// ```
    pub fn file_name(mut self, file_name: impl Into<String>) -> Self {
        self.request.file_name = file_name.into();
        self
    }

    /// 设置父节点类型
    ///
    /// # 参数
    /// - `parent_type`: 父节点类型，如 "explorer"
    ///
    /// # 示例
    ///
    /// ```rust
    /// let builder = UploadFileRequestBuilder::new()
    ///     .parent_type("explorer");
    /// ```
    pub fn parent_type(mut self, parent_type: impl Into<String>) -> Self {
        self.request.parent_type = parent_type.into();
        self
    }

    /// 设置父节点token
    ///
    /// # 参数
    /// - `parent_node`: 父文件夹的token
    ///
    /// # 示例
    ///
    /// ```rust
    /// let builder = UploadFileRequestBuilder::new()
    ///     .parent_node("fldbcO1UuPz8VwnpPx5a92abcef");
    /// ```
    pub fn parent_node(mut self, parent_node: impl Into<String>) -> Self {
        self.request.parent_node = parent_node.into();
        self
    }

    /// 设置文件大小
    ///
    /// # 参数
    /// - `size`: 文件大小（字节）
    ///
    /// # 示例
    ///
    /// ```rust
    /// let builder = UploadFileRequestBuilder::new()
    ///     .size(1024);
    /// ```
    pub fn size(mut self, size: i64) -> Self {
        self.request.size = size;
        self
    }

    /// 设置文件校验和
    ///
    /// # 参数
    /// - `checksum`: 文件的adler32校验和（可选）
    ///
    /// # 示例
    ///
    /// ```rust
    /// let builder = UploadFileRequestBuilder::new()
    ///     .checksum("abcdef123456");
    /// ```
    pub fn checksum(mut self, checksum: impl Into<String>) -> Self {
        self.request.checksum = Some(checksum.into());
        self
    }

    /// 设置文件数据
    ///
    /// # 参数
    /// - `file_data`: 文件的二进制内容
    ///
    /// # 示例
    ///
    /// ```rust
    /// let builder = UploadFileRequestBuilder::new()
    ///     .file_data(file_bytes);
    /// ```
    pub fn file_data(mut self, file_data: Vec<u8>) -> Self {
        self.request.file_data = file_data.clone();
        self.request.api_req.file = file_data;
        self
    }

    /// 自动设置文件大小
    ///
    /// 根据提供的文件数据自动计算文件大小
    ///
    /// # 示例
    ///
    /// ```rust
    /// let builder = UploadFileRequestBuilder::new()
    ///     .file_data(file_bytes)
    ///     .auto_size();
    /// ```
    pub fn auto_size(mut self) -> Self {
        self.request.size = self.request.file_data.len() as i64;
        self
    }

    /// 执行上传操作
    ///
    /// 构建请求并执行文件上传
    ///
    /// # 返回
    /// 上传结果，包含文件token
    ///
    /// # 错误
    /// 如果参数验证失败或上传过程中出现错误，返回相应的错误信息
    pub async fn execute(self, service: &FilesService) -> SDKResult<UploadFileResponse> {
        debug!("执行文件上传: {}", self.request.file_name);

        match self.request.build() {
            Ok(request) => service.upload_file(request, None).await,
            Err(e) => {
                error!("构建上传请求失败: {}", e);
                Err(e)
            }
        }
    }
}

/// 下载文件请求构建器
#[derive(Debug, Clone)]
pub struct DownloadFileRequestBuilder {
    request: DownloadFileRequest,
}

impl Default for DownloadFileRequestBuilder {
    fn default() -> Self {
        Self {
            request: DownloadFileRequest::default(),
        }
    }
}

impl DownloadFileRequestBuilder {
    /// 创建新的下载文件构建器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置文件token
    ///
    /// # 参数
    /// - `file_token`: 要下载的文件token
    ///
    /// # 示例
    ///
    /// ```rust
    /// let builder = DownloadFileRequestBuilder::new()
    ///     .file_token("file_token_123");
    /// ```
    pub fn file_token(mut self, file_token: impl Into<String>) -> Self {
        self.request.file_token = file_token.into();
        self
    }

    /// 执行下载操作
    ///
    /// 构建请求并执行文件下载
    ///
    /// # 返回
    /// 下载的文件内容
    ///
    /// # 错误
    /// 如果参数验证失败或下载过程中出现错误，返回相应的错误信息
    pub async fn execute(self, service: &FilesService) -> SDKResult<DownloadFileResponse> {
        debug!("执行文件下载: {}", self.request.file_token);

        match self.request.build() {
            Ok(request) => service.download_file(request, None).await,
            Err(e) => {
                error!("构建下载请求失败: {}", e);
                Err(e)
            }
        }
    }
}

// ==================== ExecutableBuilder宏实现 ====================

crate::impl_executable_builder_owned!(
    UploadFileRequestBuilder,
    FilesService,
    UploadFileRequest,
    UploadFileResponse,
    upload_file,
);

crate::impl_executable_builder_owned!(
    DownloadFileRequestBuilder,
    FilesService,
    DownloadFileRequest,
    DownloadFileResponse,
    download_file,
);

// ==================== 辅助函数 ====================

/// 计算文件的adler32校验和
///
/// # 参数
/// - `data`: 文件二进制数据
///
/// # 返回
/// adler32校验和的十六进制字符串
///
/// # 示例
///
/// ```rust
/// let file_data = std::fs::read("file.pdf")?;
/// let checksum = calculate_adler32_checksum(&file_data);
/// ```
pub fn calculate_adler32_checksum(data: &[u8]) -> String {
    use adler32::RollingAdler32;

    let mut hasher = RollingAdler32::new();
    hasher.update_buffer(data);
    format!("{:08x}", hasher.hash())
}

/// 验证文件类型是否被支持
///
/// # 参数
/// - `file_name`: 文件名
/// - `file_data`: 文件二进制数据（可选）
///
/// # 返回
/// 如果文件类型被支持返回Ok(())，否则返回错误信息
pub fn validate_file_type(file_name: &str, file_data: Option<&[u8]>) -> SDKResult<()> {
    // 检查文件扩展名
    let supported_extensions = vec![
        "pdf", "doc", "docx", "xls", "xlsx", "ppt", "pptx",
        "txt", "md", "rtf",
        "jpg", "jpeg", "png", "gif", "bmp", "webp",
        "mp3", "wav", "mp4", "avi", "mov", "mkv",
        "zip", "rar", "7z", "tar", "gz",
    ];

    if let Some(extension) = std::path::Path::new(file_name)
        .extension()
        .and_then(|ext| ext.to_str())
    {
        if supported_extensions.contains(&extension.to_lowercase().as_str()) {
            return Ok(());
        }
    }

    // 如果提供了文件数据，尝试通过magic number验证
    if let Some(data) = file_data {
        if is_supported_file_type(data) {
            return Ok(());
        }
    }

    Err(SDKError::InvalidParameter(format!(
        "不支持的文件类型: {}",
        file_name
    )))
}

/// 通过文件头信息判断文件类型是否被支持
fn is_supported_file_type(data: &[u8]) -> bool {
    if data.len() < 4 {
        return false;
    }

    // 检查常见文件格式的magic number
    let signatures = vec![
        // PDF
        b"%PDF",
        // PNG
        b"\x89PNG",
        // JPEG
        b"\xFF\xD8\xFF",
        // ZIP
        b"PK\x03\x04",
        // Office文档
        b"\xD0\xCF\x11\xE0",
    ];

    for signature in signatures {
        if data.starts_with(signature) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_upload_file_request_validation() {
        let request = UploadFileRequest::new()
            .file_name("test.pdf")
            .parent_type("explorer")
            .parent_node("folder_token")
            .size(1024)
            .file_data(vec![1, 2, 3, 4]);

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_download_file_request_validation() {
        let request = DownloadFileRequest::new()
            .file_token("file_token_123");

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_calculate_adler32_checksum() {
        let data = b"Hello, World!";
        let checksum = calculate_adler32_checksum(data);
        assert!(!checksum.is_empty());
        assert_eq!(checksum.len(), 8); // adler32是8位十六进制
    }

    #[test]
    fn test_validate_file_type() {
        // 测试支持的文件类型
        assert!(validate_file_type("document.pdf", None).is_ok());
        assert!(validate_file_type("image.jpg", None).is_ok());

        // 测试不支持的文件类型
        assert!(validate_file_type("unknown.xyz", None).is_err());
    }

    #[test]
    fn test_upload_file_builder() {
        let builder = UploadFileRequestBuilder::new()
            .file_name("test.pdf")
            .parent_type("explorer")
            .parent_node("folder_token")
            .size(1024)
            .file_data(vec![1, 2, 3, 4]);

        assert_eq!(builder.request.file_name, "test.pdf");
        assert_eq!(builder.request.parent_type, "explorer");
        assert_eq!(builder.request.parent_node, "folder_token");
        assert_eq!(builder.request.size, 1024);
        assert_eq!(builder.request.file_data, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_download_file_builder() {
        let builder = DownloadFileRequestBuilder::new()
            .file_token("file_token_123");

        assert_eq!(builder.request.file_token, "file_token_123");
    }

    #[test]
    fn test_auto_size() {
        let file_data = vec![1, 2, 3, 4, 5];
        let builder = UploadFileRequestBuilder::new()
            .file_data(file_data.clone())
            .auto_size();

        assert_eq!(builder.request.size, file_data.len() as i64);
    }
}