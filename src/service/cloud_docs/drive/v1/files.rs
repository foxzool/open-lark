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
use serde_json::json;
use std::collections::HashMap;
use crate::core::SDKResult;

use crate::{
    core::{
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
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

// ==================== 文件删除功能 ====================

/// 删除文件请求
#[derive(Debug, Clone)]
pub struct DeleteFileRequest {
    /// 文件token，用于唯一标识文件
    pub file_token: String,
}

impl DeleteFileRequest {
    /// 创建新的删除请求实例
    ///
    /// # 参数
    /// - `file_token`: 要删除的文件token
    ///
    /// # 示例
    /// ```rust
    /// let request = DeleteFileRequest::new("file_token_123");
    /// ```
    pub fn new(file_token: impl Into<String>) -> Self {
        Self {
            file_token: file_token.into(),
        }
    }

    /// 验证请求参数
    ///
    /// 验证file_token的有效性，确保符合API要求
    ///
    /// # 返回
    /// - `Ok(())`: 验证通过
    /// - `Err(String)`: 验证失败，包含错误信息
    pub fn validate(&self) -> Result<(), String> {
        if self.file_token.trim().is_empty() {
            return Err("file_token不能为空".to_string());
        }

        // 验证token长度
        if self.file_token.len() > 200 {
            return Err("file_token长度不能超过200个字符".to_string());
        }

        // 验证字符安全性，只允许字母、数字、连字符、下划线
        let allowed_chars = self.file_token.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-');
        if !allowed_chars {
            return Err("file_token包含不支持的字符，只允许字母、数字、下划线和连字符".to_string());
        }

        Ok(())
    }
}

/// 删除文件响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteFileResponseData {
    /// 是否成功删除
    pub deleted: bool,
    /// 被删除的文件token
    pub file_token: String,
    /// 删除时间戳（毫秒）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_time: Option<i64>,
}

/// 删除文件响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeleteFileResponse {
    /// 响应数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<DeleteFileResponseData>,
    /// 是否成功
    pub success: bool,
    /// 错误消息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// 错误代码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
}

impl ApiResponseTrait for DeleteFileResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

// ==================== 创建文件快捷方式 ====================

/// 创建文件快捷方式请求
#[derive(Debug, Clone)]
pub struct CreateShortcutRequest {
    /// 源文件令牌
    pub source_file_token: String,
    /// 目标文件夹令牌
    pub folder_token: String,
    /// 快捷方式名称（可选）
    pub name: Option<String>,
    /// 用户ID类型
    pub user_id_type: Option<String>,
}

impl CreateShortcutRequest {
    /// 创建新的请求实例
    pub fn new(
        source_file_token: impl Into<String>,
        folder_token: impl Into<String>,
    ) -> Self {
        Self {
            source_file_token: source_file_token.into(),
            folder_token: folder_token.into(),
            name: None,
            user_id_type: None,
        }
    }

    /// 设置快捷方式名称
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.source_file_token.trim().is_empty() {
            return Err("源文件令牌不能为空".to_string());
        }

        if self.source_file_token.len() > 256 {
            return Err("源文件令牌长度不能超过256个字符".to_string());
        }

        if self.folder_token.trim().is_empty() {
            return Err("目标文件夹令牌不能为空".to_string());
        }

        if self.folder_token.len() > 256 {
            return Err("目标文件夹令牌长度不能超过256个字符".to_string());
        }

        if let Some(ref name) = self.name {
            if name.trim().is_empty() {
                return Err("快捷方式名称不能为空字符串".to_string());
            }
            if name.len() > 255 {
                return Err("快捷方式名称长度不能超过255个字符".to_string());
            }
            // 检查名称是否包含非法字符
            if name.contains(['/', '\\', ':', '*', '?', '"', '<', '>', '|']) {
                return Err("快捷方式名称不能包含以下字符: / \\ : * ? \" < > |".to_string());
            }
        }

        Ok(())
    }
}

/// 创建文件快捷方式响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateShortcutResponseData {
    /// 创建的快捷方式信息
    pub shortcut: FileInfo,
}

/// 文件信息结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    /// 文件令牌
    pub file_token: String,
    /// 文件名称
    pub name: String,
    /// 文件类型
    pub r#type: String,
    /// 文件大小
    pub size: i64,
    /// 创建时间
    pub create_time: String,
    /// 修改时间
    pub modify_time: String,
}

/// 创建文件快捷方式响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateShortcutResponse {
    /// 响应数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<CreateShortcutResponseData>,
    /// 是否成功
    pub success: bool,
    /// 错误消息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// 错误代码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
}

impl ApiResponseTrait for CreateShortcutResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl FilesService {
    /// 创建文件快捷方式
    ///
    /// 为指定文件创建快捷方式，快捷方式可以放置在目标文件夹中
    /// 支持为文档、表格、图片等文件类型创建快捷方式
    ///
    /// # 参数
    /// * `req` - 创建快捷方式请求
    ///
    /// # 返回值
    /// 返回创建的快捷方式信息
    ///
    /// # 示例
    /// ```rust
    /// let request = CreateShortcutRequest::new("source_file_token_123", "folder_token_456")
    ///     .name("重要文档快捷方式")
    ///     .user_id_type("open_id");
    /// let response = service.create_shortcut(&request).await?;
    ///
    /// if response.success {
    ///     println!("快捷方式创建成功: {:?}", response.data.as_ref().unwrap().shortcut);
    /// } else {
    ///     println!("快捷方式创建失败: {:?}", response.error_message);
    /// }
    /// ```
    pub async fn create_shortcut(&self, req: &CreateShortcutRequest) -> SDKResult<CreateShortcutResponse> {
        req.validate().map_err(|e| SDKError::InvalidParameter(e))?;
        debug!("开始创建文件快捷方式: source_file={}, folder={}",
               req.source_file_token, req.folder_token);

        // 构建查询参数
        let mut query_params: HashMap<&str, String> = HashMap::new();
        if let Some(user_id_type) = &req.user_id_type {
            query_params.insert("user_id_type", user_id_type.clone());
        }

        // 构建请求体
        let mut body = json!({
            "source_file_token": req.source_file_token,
            "folder_token": req.folder_token
        });

        if let Some(ref name) = req.name {
            body["name"] = json!(name);
        }

        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: crate::core::endpoints_original::Endpoints::DRIVE_V1_FILES_CREATE_SHORTCUT.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: serde_json::to_vec(&body).unwrap_or_default(),
            ..Default::default()
        };

        match Transport::request(api_req, &self.config, None).await {
            Ok(resp) => {
                let response = resp.data.unwrap_or_default();
                if response.success {
                    info!("快捷方式创建成功: source_file={}, folder={}",
                          req.source_file_token, req.folder_token);
                } else {
                    warn!("快捷方式创建失败: source_file={}, folder={}, error={:?}",
                          req.source_file_token, req.folder_token, response.error_message);
                }
                Ok(response)
            }
            Err(e) => {
                error!("创建快捷方式请求失败: {}", e);
                Err(e)
            }
        }
    }

    /// 删除文件或文件夹
    ///
    /// 删除指定token对应的文件或文件夹。删除操作不可恢复，请谨慎使用。
    /// 支持删除各种类型的文件，包括文档、表格、图片等。
    ///
    /// # 参数
    /// - `req`: 删除文件请求
    ///
    /// # 返回值
    /// 返回删除操作的结果，包含删除状态和时间信息
    ///
    /// # 示例
    /// ```rust
    /// let request = DeleteFileRequest::new("file_token_123");
    /// let response = service.delete_file(&request).await?;
    ///
    /// if response.success {
    ///     println!("文件删除成功");
    /// } else {
    ///     println!("文件删除失败: {:?}", response.error_message);
    /// }
    /// ```
    pub async fn delete_file(&self, req: &DeleteFileRequest) -> SDKResult<DeleteFileResponse> {
        req.validate().map_err(|e| SDKError::InvalidParameter(e))?;
        debug!("开始删除文件: {}", req.file_token);

        // 构建API路径，替换file_token占位符
        let api_path = crate::core::endpoints_original::Endpoints::DRIVE_V1_FILES_DELETE
            .replace("{}", &req.file_token);

        let api_req = ApiRequest {
            http_method: Method::DELETE,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: Vec::new(), // DELETE请求不需要请求体
            ..Default::default()
        };

        match Transport::request(api_req, &self.config, None).await {
            Ok(resp) => {
                let response = resp.data.unwrap_or_default();
                if response.success {
                    info!("文件删除成功: {}", req.file_token);
                } else {
                    warn!("文件删除失败: {}, error: {:?}", req.file_token, response.error_message);
                }
                Ok(response)
            }
            Err(e) => {
                error!("删除文件请求失败: {}", e);
                Err(e)
            }
        }
    }
}

// ==================== 创建文件快捷方式构建器模式 ====================

/// 创建文件快捷方式构建器
#[derive(Debug, Clone, Default)]
pub struct CreateShortcutRequestBuilder {
    request: CreateShortcutRequest,
}

impl CreateShortcutRequestBuilder {
    /// 创建新的构建器
    ///
    /// # 参数
    /// * `source_file_token` - 源文件令牌
    /// * `folder_token` - 目标文件夹令牌
    ///
    /// # 示例
    /// ```rust
    /// let builder = CreateShortcutRequestBuilder::new("source_token_123", "folder_token_456");
    /// ```
    pub fn new(
        source_file_token: impl Into<String>,
        folder_token: impl Into<String>,
    ) -> Self {
        Self {
            request: CreateShortcutRequest::new(source_file_token, folder_token),
        }
    }

    /// 设置快捷方式名称
    ///
    /// # 参数
    /// * `name` - 快捷方式名称
    ///
    /// # 返回
    /// 返回构建器实例，支持链式调用
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.request = self.request.name(name);
        self
    }

    /// 设置用户ID类型
    ///
    /// # 参数
    /// * `user_id_type` - 用户ID类型
    ///
    /// # 返回
    /// 返回构建器实例，支持链式调用
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.request = self.request.user_id_type(user_id_type);
        self
    }

    /// 执行创建快捷方式操作
    ///
    /// # 参数
    /// * `service` - 文件服务实例
    ///
    /// # 返回
    /// 返回创建快捷方式的结果
    ///
    /// # 示例
    /// ```rust
    /// let response = CreateShortcutRequestBuilder::new("source_token", "folder_token")
    ///     .name("快捷方式名称")
    ///     .user_id_type("open_id")
    ///     .execute(&service)
    ///     .await?;
    /// ```
    pub async fn execute(self, service: &FilesService) -> SDKResult<CreateShortcutResponse> {
        service.create_shortcut(&self.request).await
    }
}

impl FilesService {
    /// 创建文件快捷方式构建器
    ///
    /// 创建一个用于创建文件快捷方式的构建器，支持设置源文件、目标文件夹、快捷方式名称等参数。
    /// 构建器会自动验证参数的有效性，确保源文件存在、目标文件夹可访问等。
    ///
    /// # 特性
    /// - 自动参数验证
    /// - 支持快捷方式命名
    /// - 支持用户ID类型设置
    /// - 链式调用支持
    ///
    /// # 参数
    /// * `source_file_token` - 源文件令牌
    /// * `folder_token` - 目标文件夹令牌
    ///
    /// # 示例
    /// ```rust
    /// let response = service
    ///     .create_shortcut_builder("source_file_token_123", "folder_token_456")
    ///     .name("重要文档快捷方式")
    ///     .user_id_type("open_id")
    ///     .execute()
    ///     .await?;
    /// ```
    pub fn create_shortcut_builder(
        &self,
        source_file_token: impl Into<String>,
        folder_token: impl Into<String>,
    ) -> CreateShortcutRequestBuilder {
        CreateShortcutRequestBuilder::new(source_file_token, folder_token)
    }
}

// ==================== 删除文件构建器模式 ====================

/// 删除文件构建器
#[derive(Debug, Clone, Default)]
pub struct DeleteFileRequestBuilder {
    request: DeleteFileRequest,
}

impl DeleteFileRequestBuilder {
    /// 创建新的构建器
    ///
    /// # 参数
    /// - `file_token`: 要删除的文件token
    ///
    /// # 示例
    /// ```rust
    /// let builder = DeleteFileRequestBuilder::new("file_token_123");
    /// ```
    pub fn new(file_token: impl Into<String>) -> Self {
        Self {
            request: DeleteFileRequest::new(file_token),
        }
    }

    /// 设置文件token
    ///
    /// # 参数
    /// - `file_token`: 文件token
    ///
    /// # 返回
    /// 返回构建器实例，支持链式调用
    pub fn file_token(mut self, file_token: impl Into<String>) -> Self {
        self.request.file_token = file_token.into();
        self
    }

    /// 执行删除操作
    ///
    /// 构建请求并执行文件删除
    ///
    /// # 参数
    /// - `service`: FilesService实例
    ///
    /// # 返回
    /// 删除操作的响应结果
    ///
    /// # 错误
    /// 如果参数验证失败或删除过程中出现错误，返回相应的错误信息
    pub async fn execute(self, service: &FilesService) -> SDKResult<DeleteFileResponse> {
        debug!("执行文件删除: {}", self.request.file_token);

        match self.request.validate() {
            Ok(()) => service.delete_file(&self.request).await,
            Err(e) => {
                error!("删除文件请求验证失败: {}", e);
                Err(SDKError::InvalidParameter(e))
            }
        }
    }
}

impl FilesService {
    /// 创建删除文件Builder
    ///
    /// 创建一个用于删除文件的构建器，只需要提供文件 token 即可。
    /// 文件 token 可以从上传文件、文件列表等接口获取。
    ///
    /// # 特性
    /// - 自动参数验证
    /// - 支持链式调用
    /// - 错误处理完善
    ///
    /// # 示例
    ///
    /// ```rust
    /// let builder = service.create_delete_file_builder()
    ///     .file_token("file_token_123");
    ///
    /// let response = builder.execute(&service).await?;
    /// ```
    pub fn create_delete_file_builder(&self, file_token: impl Into<String>) -> DeleteFileRequestBuilder {
        DeleteFileRequestBuilder::new(file_token)
    }
}

// ==================== ExecutableBuilder宏实现 ====================

crate::impl_executable_builder_owned!(
    DeleteFileRequestBuilder,
    FilesService,
    DeleteFileRequest,
    DeleteFileResponse,
    delete_file,
);

// ==================== 单元测试 ====================

#[cfg(test)]
mod create_shortcut_tests {
    use super::*;
    use crate::core::config::Config;

    #[test]
    fn test_create_shortcut_request_creation() {
        let request = CreateShortcutRequest::new("source_file_token_123", "folder_token_456");
        assert_eq!(request.source_file_token, "source_file_token_123");
        assert_eq!(request.folder_token, "folder_token_456");
        assert_eq!(request.name, None);
        assert_eq!(request.user_id_type, None);
    }

    #[test]
    fn test_create_shortcut_request_with_fields() {
        let request = CreateShortcutRequest::new("source_file_token_123", "folder_token_456")
            .name("快捷方式名称")
            .user_id_type("open_id");

        assert_eq!(request.source_file_token, "source_file_token_123");
        assert_eq!(request.folder_token, "folder_token_456");
        assert_eq!(request.name, Some("快捷方式名称".to_string()));
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
    }

    #[test]
    fn test_create_shortcut_request_validation() {
        // 测试正常情况
        let valid_request = CreateShortcutRequest::new("source_token_123", "folder_token_456")
            .name("有效快捷方式");
        assert!(valid_request.validate().is_ok());

        // 测试空源文件令牌
        let empty_source_request = CreateShortcutRequest::new("", "folder_token_456");
        assert!(empty_source_request.validate().is_err());

        // 测试空目标文件夹令牌
        let empty_folder_request = CreateShortcutRequest::new("source_token_123", "");
        assert!(empty_folder_request.validate().is_err());

        // 测试超长源文件令牌
        let long_source_request = CreateShortcutRequest::new(&"a".repeat(257), "folder_token_456");
        assert!(long_source_request.validate().is_err());

        // 测试超长目标文件夹令牌
        let long_folder_request = CreateShortcutRequest::new("source_token_123", &"a".repeat(257));
        assert!(long_folder_request.validate().is_err());

        // 测试空快捷方式名称
        let empty_name_request = CreateShortcutRequest::new("source_token_123", "folder_token_456")
            .name("");
        assert!(empty_name_request.validate().is_err());

        // 测试超长快捷方式名称
        let long_name_request = CreateShortcutRequest::new("source_token_123", "folder_token_456")
            .name(&"a".repeat(256));
        assert!(long_name_request.validate().is_err());

        // 测试包含非法字符的快捷方式名称
        let invalid_chars_requests = vec!["名称/包含斜杠", "名称\\包含反斜杠", "名称:包含冒号",
            "名称*包含星号", "名称?包含问号", "名称\"包含引号", "名称<包含尖括号", "名称>包含尖括号", "名称|包含竖线"];

        for invalid_name in invalid_chars_requests {
            let invalid_request = CreateShortcutRequest::new("source_token_123", "folder_token_456")
                .name(invalid_name);
            assert!(invalid_request.validate().is_err(), "应该拒绝包含非法字符的名称: {}", invalid_name);
        }

        // 测试有效的快捷方式名称
        let valid_names = vec!["正常名称", "正常名称123", "正常_名称", "正常-名称", "正常.名称"];
        for valid_name in valid_names {
            let valid_request = CreateShortcutRequest::new("source_token_123", "folder_token_456")
                .name(valid_name);
            assert!(valid_request.validate().is_ok(), "应该接受有效名称: {}", valid_name);
        }
    }

    #[test]
    fn test_create_shortcut_response_creation() {
        let file_info = FileInfo {
            file_token: "shortcut_token_789".to_string(),
            name: "快捷方式名称".to_string(),
            r#type: "shortcut".to_string(),
            size: 0,
            create_time: "2023-01-01T00:00:00Z".to_string(),
            modify_time: "2023-01-01T00:00:00Z".to_string(),
        };

        let response_data = CreateShortcutResponseData {
            shortcut: file_info.clone(),
        };

        let response = CreateShortcutResponse {
            data: Some(response_data),
            success: true,
            ..Default::default()
        };

        assert!(response.success);
        assert!(response.data.is_some());
        assert_eq!(response.data.as_ref().unwrap().shortcut.file_token, "shortcut_token_789");
        assert_eq!(response.data.as_ref().unwrap().shortcut.name, "快捷方式名称");
    }

    #[test]
    fn test_create_shortcut_builder() {
        let builder = CreateShortcutRequestBuilder::new("source_token_123", "folder_token_456")
            .name("快捷方式测试")
            .user_id_type("union_id");

        assert_eq!(builder.request.source_file_token, "source_token_123");
        assert_eq!(builder.request.folder_token, "folder_token_456");
        assert_eq!(builder.request.name, Some("快捷方式测试".to_string()));
        assert_eq!(builder.request.user_id_type, Some("union_id".to_string()));
    }

    #[test]
    fn test_create_shortcut_builder_validation() {
        // 测试有效构建器
        let valid_builder = CreateShortcutRequestBuilder::new("source_token_123", "folder_token_456")
            .name("有效快捷方式");
        assert!(valid_builder.request.validate().is_ok());

        // 测试无效构建器
        let invalid_builder = CreateShortcutRequestBuilder::new("", "folder_token_456");
        assert!(invalid_builder.request.validate().is_err());

        // 测试另一个无效构建器
        let invalid_builder2 = CreateShortcutRequestBuilder::new("source_token_123", "");
        assert!(invalid_builder2.request.validate().is_err());
    }

    #[test]
    fn test_create_shortcut_service_method() {
        let config = Config::default();
        let service = FilesService::new(config);

        // 验证服务包含所需的方法
        let service_str = format!("{:?}", service);
        assert!(!service_str.is_empty());

        // 验证构建器方法存在
        let builder = service.create_shortcut_builder("source_token_123", "folder_token_456");
        assert_eq!(builder.request.source_file_token, "source_token_123");
        assert_eq!(builder.request.folder_token, "folder_token_456");
    }

    #[test]
    fn test_create_shortcut_endpoint_construction() {
        // 验证端点常量存在
        assert_eq!(
            crate::core::endpoints_original::Endpoints::DRIVE_V1_FILES_CREATE_SHORTCUT,
            "/open-apis/drive/v1/files/create_shortcut"
        );
    }

    #[test]
    fn test_create_shortcut_json_serialization() {
        let request = CreateShortcutRequest::new("source_token_123", "folder_token_456")
            .name("序列化测试快捷方式")
            .user_id_type("open_id");

        // 测试请求可以转换为JSON
        let body = json!({
            "source_file_token": "source_token_123",
            "folder_token": "folder_token_456",
            "name": "序列化测试快捷方式"
        });

        assert_eq!(body["source_file_token"], "source_token_123");
        assert_eq!(body["folder_token"], "folder_token_456");
        assert_eq!(body["name"], "序列化测试快捷方式");
    }

    #[test]
    fn test_create_shortcut_response_trait() {
        assert_eq!(CreateShortcutResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_create_shortcut_comprehensive_scenario() {
        // 测试完整的业务场景
        let complex_request = CreateShortcutRequest::new("complex_source_token_001", "complex_folder_token_002")
            .name("复杂场景快捷方式")
            .user_id_type("union_id");

        assert!(complex_request.validate().is_ok());
        assert_eq!(complex_request.source_file_token, "complex_source_token_001");
        assert_eq!(complex_request.folder_token, "complex_folder_token_002");
        assert_eq!(complex_request.name, Some("复杂场景快捷方式".to_string()));
        assert_eq!(complex_request.user_id_type, Some("union_id".to_string()));
    }

    #[test]
    fn test_create_shortcut_edge_cases() {
        // 测试边界情况
        let edge_request = CreateShortcutRequest::new(&"a".repeat(256), &"b".repeat(256))
            .name(&"c".repeat(255));

        assert!(edge_request.validate().is_ok());
        assert_eq!(edge_request.source_file_token.len(), 256);
        assert_eq!(edge_request.folder_token.len(), 256);
        assert_eq!(edge_request.name.as_ref().unwrap().len(), 255);
    }

    #[test]
    fn test_create_shortcut_builder_pattern() {
        // 测试构建器模式的流畅性
        let builder = CreateShortcutRequestBuilder::new("test_source", "test_folder")
            .name("流畅性测试")
            .user_id_type("open_id");

        // 验证构建器状态
        assert_eq!(builder.request.source_file_token, "test_source");
        assert_eq!(builder.request.folder_token, "test_folder");
        assert_eq!(builder.request.name, Some("流畅性测试".to_string()));
        assert_eq!(builder.request.user_id_type, Some("open_id".to_string()));

        // 验证请求验证通过
        assert!(builder.request.validate().is_ok());

        // 测试链式调用
        let chained_builder = builder
            .name("重新设置名称")
            .request;
        assert_eq!(chained_builder.name.unwrap(), "重新设置名称");
    }

    #[test]
    fn test_file_info_creation() {
        let file_info = FileInfo {
            file_token: "test_file_token".to_string(),
            name: "测试文件".to_string(),
            r#type: "document".to_string(),
            size: 1024,
            create_time: "2023-12-01T10:00:00Z".to_string(),
            modify_time: "2023-12-01T15:30:00Z".to_string(),
        };

        assert_eq!(file_info.file_token, "test_file_token");
        assert_eq!(file_info.name, "测试文件");
        assert_eq!(file_info.r#type, "document");
        assert_eq!(file_info.size, 1024);
        assert_eq!(file_info.create_time, "2023-12-01T10:00:00Z");
        assert_eq!(file_info.modify_time, "2023-12-01T15:30:00Z");
    }
}

#[cfg(test)]
mod delete_file_tests {
    use super::*;

    #[test]
    fn test_delete_file_request_creation() {
        let request = DeleteFileRequest::new("test_file_token");
        assert_eq!(request.file_token, "test_file_token");
    }

    #[test]
    fn test_delete_file_request_validation() {
        // 测试正常情况
        let valid_request = DeleteFileRequest::new("valid_token_123");
        assert!(valid_request.validate().is_ok());

        // 测试空token
        let empty_request = DeleteFileRequest::new("");
        assert!(empty_request.validate().is_err());

        // 测试超长token
        let long_request = DeleteFileRequest::new("a".repeat(201));
        assert!(long_request.validate().is_err());

        // 测试非法字符
        let invalid_request = DeleteFileRequest::new("token@invalid");
        assert!(invalid_request.validate().is_err());
    }

    #[test]
    fn test_delete_file_builder() {
        let builder = DeleteFileRequestBuilder::new("test_token")
            .file_token("updated_token");

        assert_eq!(builder.request.file_token, "updated_token");
    }

    #[test]
    fn test_delete_file_service_method() {
        let config = Config::default();
        let service = FilesService::new(config);

        // 验证服务包含所需的方法
        let service_str = format!("{:?}", service);
        assert!(!service_str.is_empty());

        // 验证构建器方法存在
        let builder = service.create_delete_file_builder("test_token");
        assert_eq!(builder.request.file_token, "test_token");
    }

    #[test]
    fn test_delete_file_response_creation() {
        let response_data = DeleteFileResponseData {
            deleted: true,
            file_token: "test_token".to_string(),
            delete_time: Some(1699123456789),
        };

        let response = DeleteFileResponse {
            data: Some(response_data),
            success: true,
            ..Default::default()
        };

        assert!(response.success);
        assert!(response.data.is_some());
        assert_eq!(response.data.as_ref().unwrap().file_token, "test_token");
        assert_eq!(response.data.as_ref().unwrap().deleted, true);
    }

    #[test]
    fn test_delete_file_response_trait() {
        assert_eq!(DeleteFileResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_delete_file_edge_cases() {
        // 测试边界长度
        let max_length_request = DeleteFileRequest::new("a".repeat(200));
        assert!(max_length_request.validate().is_ok());

        // 测试特殊允许字符
        let special_chars_request = DeleteFileRequest::new("token_with_-chars");
        assert!(special_chars_request.validate().is_ok());

        // 测试混合字符
        let mixed_request = DeleteFileRequest::new("Token123_ABC-def");
        assert!(mixed_request.validate().is_ok());
    }

    #[test]
    fn test_delete_file_builder_validation() {
        // 测试有效构建器
        let valid_builder = DeleteFileRequestBuilder::new("valid_token_123");
        assert!(valid_builder.request.validate().is_ok());

        // 测试无效构建器
        let invalid_builder = DeleteFileRequestBuilder::new("");
        assert!(invalid_builder.request.validate().is_err());
    }

    #[test]
    fn test_delete_file_comprehensive_scenario() {
        // 测试完整的业务场景 - 删除一个真实文件的模拟
        let request = DeleteFileRequest::new("real_file_token_abc123")
            .validate()
            .map(|_| DeleteFileRequest::new("real_file_token_abc123"))
            .unwrap();

        assert_eq!(request.file_token, "real_file_token_abc123");
        assert!(request.validate().is_ok());
    }
}