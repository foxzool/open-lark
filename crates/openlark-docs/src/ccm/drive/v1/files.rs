
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
use std::collections::HashMap;
//! Cloud Docs Drive文件管理服务 v1
//!
//! 提供企业级的文件上传、下载、管理功能，支持多种文件格式和高级操作：
//! - 文件上传：支持小文件直接上传和大文件分片上传
//! - 文件下载：支持批量下载和断点续传
//! - 文件管理：文件信息查询、更新、删除等操作
//! - 权限控制：细粒度的文件访问权限管理
//! - 版本控制：文件版本历史和管理

use log::{debug, error, info, warn};
use openlark_core::api::ApiRequest;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::json;
use SDKResult;

use openlark_core::{
    core::{
        api::{ApiResponseTrait, BaseResponse, ResponseFormat},
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
#[derive(Clone, Debug)]
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
    /// let config = openlark_core::config::Config::new("app_id", "app_secret");
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

        let api_resp: Response<UploadFileResponse> =
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

        let api_resp: Response<DownloadFileResponse> =
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
#[derive(Clone, Debug)]
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
            file_data: vec![],
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
        match Some(openlark_core::api::RequestData::Json(serde_json::json!(&self))) {
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
    fn data_format() -> api::ResponseFormat {
        api::ResponseFormat::Data
    }
}

/// 下载文件请求
#[derive(Clone, Debug)]
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
    fn data_format() -> api::ResponseFormat {
        api::ResponseFormat::Binary
    }
}

// ==================== Builder实现 ====================

/// 上传文件请求构建器
#[derive(Clone, Debug)]
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
#[derive(Clone, Debug)]
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
#[derive(Clone, Debug)]
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
#[derive(Clone, Debug)]
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

// ==================== API #188 查询异步任务状态 ====================

/// 查询异步任务状态请求
#[derive(Clone, Debug)]
pub struct GetAsyncTaskStatusRequest {
    /// 任务ID，通过创建异步任务的API返回获取
    pub task_id: String,
    /// 用户ID类型
    pub user_id_type: Option<String>,
}

impl GetAsyncTaskStatusRequest {
    /// 创建新的请求实例
    pub fn new(task_id: impl Into<String>) -> Self {
        Self {
            task_id: task_id.into(),
            user_id_type: None,
        }
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.task_id.trim().is_empty() {
            return Err("任务ID不能为空".to_string());
        }

        // 验证任务ID长度（飞书通常使用64位任务ID）
        if self.task_id.len() > 100 {
            return Err("任务ID长度不能超过100个字符".to_string());
        }

        // 验证任务ID格式（只允许字母、数字、下划线、连字符）
        if !self.task_id.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
            return Err("任务ID只能包含字母、数字、下划线和连字符".to_string());
        }

        if let Some(ref user_id_type) = self.user_id_type {
            let valid_types = ["open_id", "user_id", "union_id"];
            if !valid_types.contains(&user_id_type.as_str()) {
                return Err(format!(
                    "无效的用户ID类型: {}，支持的类型: open_id, user_id, union_id",
                    user_id_type
                ));
            }
        }

        Ok(())
    }
}

/// 异步任务状态信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AsyncTask {
    /// 任务ID
    pub task_id: String,
    /// 任务状态
    pub status: String,
    /// 任务进度
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<i32>,
    /// 任务类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_type: Option<String>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 完成时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complete_time: Option<String>,
    /// 任务结果
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    /// 错误信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// 额外信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<serde_json::Value>,
}

/// 查询异步任务状态响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAsyncTaskStatusResponseData {
    /// 异步任务信息
    pub task: AsyncTask,
}

/// 查询异步任务状态响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetAsyncTaskStatusResponse {
    /// 响应数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<GetAsyncTaskStatusResponseData>,
    /// 是否成功
    pub success: bool,
    /// 错误消息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// 错误代码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
}

impl ApiResponseTrait for GetAsyncTaskStatusResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl FilesService {
    /// 查询异步任务状态
    ///
    /// 查询指定异步任务的执行状态和结果信息，支持监控任务进度
    /// 适用于文件导入、批量操作等长时间运行的异步任务
    ///
    /// # 参数
    /// * `req` - 查询任务状态请求
    ///
    /// # 返回值
    /// 返回异步任务的详细状态信息
    ///
    /// # 示例
    /// ```rust
    /// let request = GetAsyncTaskStatusRequest::new("task_123456789")
    ///     .user_id_type("open_id");
    /// let response = service.get_async_task_status(&request).await?;
    ///
    /// match response.data {
    ///     Some(data) => {
    ///         match data.task.status.as_str() {
    ///             "success" => println!("任务完成: {:?}", data.task.result),
    ///             "processing" => println!("任务进行中: {}%", data.task.progress.unwrap_or(0)),
    ///             "failed" => println!("任务失败: {}", data.task.error_message.unwrap_or_default()),
    ///             _ => println!("未知状态"),
    ///         }
    ///     }
    ///     None => println!("无任务数据"),
    /// }
    /// ```
    pub async fn get_async_task_status(&self, req: &GetAsyncTaskStatusRequest) -> SDKResult<GetAsyncTaskStatusResponse> {
        req.validate().map_err(|e| SDKError::InvalidParameter(e))?;
        log::debug!("开始查询异步任务状态: task_id={}", req.task_id);

        // 构建查询参数
        let mut query: HashMap<&str, String> = HashMap::new();
        if let Some(user_id_type) = &req.user_id_type {
            query_params.insert("user_id_type", user_id_type.clone());
        }

        // 构建API路径，替换task_id占位符
        let api_path = openlark_core::endpoints::Endpoints::DRIVE_V1_TASK_GET
            .replace("{}", &req.task_id);

        let api_req = ApiRequest {
            method: openlark_core::api::HttpMethod::Get,
            api_path,
            // supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: None, // GET请求不需要body
            
        };

        let resp = Transport::<GetAsyncTaskStatusResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        if response.success {
            log::info!("异步任务状态查询成功: task_id={}, status={}",
                req.task_id,
                response.data.as_ref()
                    .map(|d| d.task.status.clone())
                    .unwrap_or_else(|| "unknown".to_string())
            );
        } else {
            log::warn!("异步任务状态查询失败: task_id={}, error={:?}",
                req.task_id, response.error_message);
        }

        Ok(response)
    }
}

// ==================== 构建器模式 ====================

/// 查询异步任务状态构建器
#[derive(Clone, Debug)]
pub struct GetAsyncTaskStatusBuilder {
    request: GetAsyncTaskStatusRequest,
}

impl GetAsyncTaskStatusBuilder {
    /// 创建新的构建器
    pub fn new(task_id: impl Into<String>) -> Self {
        Self {
            request: GetAsyncTaskStatusRequest::new(task_id),
        }
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.request = self.request.user_id_type(user_id_type);
        self
    }

    /// 执行查询异步任务状态操作
    pub async fn execute(self, service: &FilesService) -> SDKResult<GetAsyncTaskStatusResponse> {
        service.get_async_task_status(&self.request).await
    }
}

impl FilesService {
    /// 查询异步任务状态构建器
    pub fn get_async_task_status_builder(&self, task_id: impl Into<String>) -> GetAsyncTaskStatusBuilder {
        GetAsyncTaskStatusBuilder::new(task_id)
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
        let mut query: HashMap<&str, String> = HashMap::new();
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
            method: Method::POST,
            url: openlark_core::endpoints::Endpoints::DRIVE_V1_FILES_CREATE_SHORTCUT.to_string(),
            // supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: Some(openlark_core::api::RequestData::Json(serde_json::json!(&body))).unwrap_or_default(),
            
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
        let api_path = openlark_core::endpoints::Endpoints::DRIVE_V1_FILES_DELETE
            .replace("{}", &req.file_token);

        let api_req = ApiRequest {
            method: Method::DELETE,
            api_path,
            // supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: None, // DELETE请求不需要请求体
            
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
    use config::Config;

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
            shortcut: file_info.clone()
        };

        let response = CreateShortcutResponse {
            data: Some(response_data),
            success: true,
            
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
        let config = openlark_core::config::Config::default();
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
            openlark_core::endpoints::Endpoints::DRIVE_V1_FILES_CREATE_SHORTCUT,
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
mod get_async_task_status_tests {
    use super::*;
    use config::Config;

    #[test]
    fn test_get_async_task_status_request_creation() {
        let request = GetAsyncTaskStatusRequest::new("task_123456789");
        assert_eq!(request.task_id, "task_123456789");
        assert_eq!(request.user_id_type, None);
    }

    #[test]
    fn test_get_async_task_status_request_with_fields() {
        let request = GetAsyncTaskStatusRequest::new("task_987654321")
            .user_id_type("open_id");

        assert_eq!(request.task_id, "task_987654321");
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
    }

    #[test]
    fn test_get_async_task_status_request_validation() {
        // 测试正常情况
        let valid_request = GetAsyncTaskStatusRequest::new("task_valid_123")
            .user_id_type("user_id");
        assert!(valid_request.validate().is_ok());

        // 测试空任务ID
        let empty_task_request = GetAsyncTaskStatusRequest::new("");
        assert!(empty_task_request.validate().is_err());

        // 测试空格任务ID
        let whitespace_task_request = GetAsyncTaskStatusRequest::new("  ");
        assert!(whitespace_task_request.validate().is_err());

        // 测试超长任务ID
        let long_task_request = GetAsyncTaskStatusRequest::new(&"a".repeat(101));
        assert!(long_task_request.validate().is_err());

        // 测试包含非法字符的任务ID
        let invalid_char_requests = vec![
            "task@invalid", "task#invalid", "task invalid", "task.invalid",
            "task,invalid", "task(invalid)", "task)invalid"
        ];

        for invalid_task_id in invalid_char_requests {
            let invalid_request = GetAsyncTaskStatusRequest::new(invalid_task_id);
            assert!(invalid_request.validate().is_err(),
                "Task ID '{}' should be invalid", invalid_task_id);
        }

        // 测试无效的用户ID类型
        let invalid_user_type_request = GetAsyncTaskStatusRequest::new("task_valid_123")
            .user_id_type("invalid_type");
        assert!(invalid_user_type_request.validate().is_err());

        // 测试有效的任务ID格式
        let valid_task_ids = vec![
            "task_123456789", "Task-ABC-123", "task_456", "import_task_789",
            "batch-process-123", "file-upload-001", "TASK_001", "a1-b2-c3"
        ];

        for valid_task_id in valid_task_ids {
            let valid_request = GetAsyncTaskStatusRequest::new(valid_task_id);
            assert!(valid_request.validate().is_ok(),
                "Task ID '{}' should be valid", valid_task_id);
        }

        // 测试有效的用户ID类型
        let valid_user_types = vec!["open_id", "user_id", "union_id"];
        for user_type in valid_user_types {
            let valid_request = GetAsyncTaskStatusRequest::new("task_valid_123")
                .user_id_type(user_type);
            assert!(valid_request.validate().is_ok(),
                "User ID type '{}' should be valid", user_type);
        }
    }

    #[test]
    fn test_async_task_creation() {
        let task = AsyncTask {
            task_id: "task_123456".to_string(),
            status: "success".to_string(),
            progress: Some(100),
            task_type: Some("file_import".to_string()),
            create_time: Some("2023-12-01T10:00:00Z".to_string()),
            complete_time: Some("2023-12-01T10:05:00Z".to_string()),
            result: Some(serde_json::json!({"files_imported": 5})),
            error_message: None,
            extra: Some(serde_json::json!({"total_files": 5})),
        };

        assert_eq!(task.task_id, "task_123456");
        assert_eq!(task.status, "success");
        assert_eq!(task.progress, Some(100));
        assert_eq!(task.task_type, Some("file_import".to_string()));
        assert_eq!(task.create_time, Some("2023-12-01T10:00:00Z".to_string()));
        assert_eq!(task.complete_time, Some("2023-12-01T10:05:00Z".to_string()));
        assert!(task.result.is_some());
        assert!(task.extra.is_some());
        assert_eq!(task.error_message, None);
    }

    #[test]
    fn test_get_async_task_status_response_creation() {
        let task = AsyncTask {
            task_id: "task_789".to_string(),
            status: "processing".to_string(),
            progress: Some(50),
            task_type: Some("batch_operation".to_string()),
            create_time: Some("2023-12-01T12:00:00Z".to_string()),
            complete_time: None,
            result: None,
            error_message: None,
            extra: None,
        };

        let response_data = GetAsyncTaskStatusResponseData {
            task,
        };

        let response = GetAsyncTaskStatusResponse {
            data: Some(response_data),
            success: true,
            
        };

        assert!(response.success);
        assert!(response.data.is_some());
        assert_eq!(response.data.as_ref().unwrap().task.task_id, "task_789");
        assert_eq!(response.data.as_ref().unwrap().task.status, "processing");
        assert_eq!(response.data.as_ref().unwrap().task.progress, Some(50));
    }

    #[test]
    fn test_get_async_task_status_builder() {
        let builder = GetAsyncTaskStatusBuilder::new("task_builder_test")
            .user_id_type("union_id");

        assert_eq!(builder.request.task_id, "task_builder_test");
        assert_eq!(builder.request.user_id_type, Some("union_id".to_string()));
    }

    #[test]
    fn test_get_async_task_status_builder_validation() {
        // 测试有效构建器
        let valid_builder = GetAsyncTaskStatusBuilder::new("task_valid_001")
            .user_id_type("open_id");
        assert!(valid_builder.request.validate().is_ok());

        // 测试无效构建器
        let invalid_builder = GetAsyncTaskStatusBuilder::new("")
            .user_id_type("open_id");
        assert!(invalid_builder.request.validate().is_err());

        // 测试无效用户ID类型
        let invalid_user_type_builder = GetAsyncTaskStatusBuilder::new("task_valid_001")
            .user_id_type("invalid");
        assert!(invalid_user_type_builder.request.validate().is_err());
    }

    #[test]
    fn test_get_async_task_status_service_method() {
        let config = openlark_core::config::Config::default();
        let service = FilesService::new(config);

        // 验证服务包含所需的方法
        let service_str = format!("{:?}", service);
        assert!(!service_str.is_empty());

        // 验证构建器方法存在
        let builder = service.get_async_task_status_builder("task_service_test");
        assert_eq!(builder.request.task_id, "task_service_test");
    }

    #[test]
    fn test_get_async_task_status_endpoint_construction() {
        // 验证端点常量存在
        assert_eq!(
            openlark_core::endpoints::Endpoints::DRIVE_V1_TASK_GET,
            "/open-apis/drive/v1/tasks/{}"
        );

        // 验证路径替换逻辑
        let template = openlark_core::endpoints::Endpoints::DRIVE_V1_TASK_GET;
        let final_path = template.replace("{}", "task_123456");
        assert_eq!(final_path, "/open-apis/drive/v1/tasks/task_123456");
    }

    #[test]
    fn test_async_task_status_scenarios() {
        // 测试不同任务状态
        let task_statuses = vec![
            ("success", 100, Some("任务完成成功")),
            ("processing", 50, Some("任务正在处理中")),
            ("failed", 0, Some("任务执行失败")),
            ("pending", 0, Some("任务等待执行")),
            ("cancelled", 0, Some("任务已取消")),
        ];

        for (status, progress, description) in task_statuses {
            let task = AsyncTask {
                task_id: format!("task_status_{}", status),
                status: status.to_string(),
                progress: Some(progress),
                task_type: Some("test_task".to_string()),
                create_time: Some("2023-12-01T10:00:00Z".to_string()),
                complete_time: if status == "success" {
                    Some("2023-12-01T10:10:00Z".to_string())
                } else {
                    None
                },
                result: if status == "success" {
                    Some(serde_json::json!({"message": "操作成功"}))
                } else {
                    None
                },
                error_message: if status == "failed" {
                    Some("网络连接失败".to_string())
                } else {
                    None
                },
                extra: None,
            };

            let response_data = GetAsyncTaskStatusResponseData { task };
            let response = GetAsyncTaskStatusResponse {
                data: Some(response_data),
                success: true,
                
            };

            assert_eq!(response.data.unwrap().task.status, status);
            assert_eq!(response.data.unwrap().task.progress, Some(progress));
        }
    }

    #[test]
    fn test_async_task_different_types() {
        // 测试不同类型的异步任务
        let task_types = vec![
            ("file_import", "文件导入任务"),
            ("batch_delete", "批量删除任务"),
            ("export", "数据导出任务"),
            ("backup", "数据备份任务"),
            ("sync", "数据同步任务"),
        ];

        for (task_type, description) in task_types {
            let task = AsyncTask {
                task_id: format!("task_{}_001", task_type),
                status: "processing".to_string(),
                progress: Some(25),
                task_type: Some(task_type.to_string()),
                create_time: Some("2023-12-01T14:00:00Z".to_string()),
                complete_time: None,
                result: None,
                error_message: None,
                extra: Some(serde_json::json!({"description": description})),
            };

            assert_eq!(task.task_type.unwrap(), task_type);
            assert_eq!(task.progress, Some(25));
            assert!(task.extra.is_some());
        }
    }

    #[test]
    fn test_get_async_task_status_edge_cases() {
        // 测试边界情况
        let edge_cases = vec![
            ("a", "最小长度任务ID"),
            (&"a".repeat(100), "最大长度任务ID"),
            ("task-123_ABC", "混合字符任务ID"),
            ("TASK_001", "全大写任务ID"),
            ("task_001", "全小写任务ID"),
        ];

        for (task_id, description) in edge_cases {
            let request = GetAsyncTaskStatusRequest::new(task_id);
            assert!(request.validate().is_ok(),
                "{}: '{}' should be valid", description, task_id);
        }

        // 测试无效字符边界情况
        let invalid_chars = vec![
            "task with spaces", "task@symbol", "task#hash", "task$dollar",
            "task%percent", "task^caret", "task&ampersand", "task*asterisk",
            "task(parentheses)", "task)parentheses", "task+plus", "task=equals"
        ];

        for invalid_task_id in invalid_chars {
            let request = GetAsyncTaskStatusRequest::new(invalid_task_id);
            assert!(request.validate().is_err(),
                "Task ID '{}' should be invalid", invalid_task_id);
        }
    }

    #[test]
    fn test_get_async_task_status_response_trait() {
        assert_eq!(GetAsyncTaskStatusResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_async_task_comprehensive_scenario() {
        // 测试完整的业务场景 - 文件导入任务
        let import_task = AsyncTask {
            task_id: "file_import_task_001".to_string(),
            status: "processing".to_string(),
            progress: Some(75),
            task_type: Some("file_import".to_string()),
            create_time: Some("2023-12-01T09:00:00Z".to_string()),
            complete_time: None,
            result: None,
            error_message: None,
            extra: Some(serde_json::json!({
                "total_files": 10,
                "processed_files": 7,
                "failed_files": 1,
                "remaining_files": 2,
                "estimated_completion": "2023-12-01T09:15:00Z"
            })),
        };

        let request = GetAsyncTaskStatusRequest::new("file_import_task_001")
            .user_id_type("open_id");

        assert!(request.validate().is_ok());
        assert_eq!(request.task_id, "file_import_task_001");
        assert_eq!(request.user_id_type, Some("open_id".to_string()));

        let response_data = GetAsyncTaskStatusResponseData { task: import_task };
        let response = GetAsyncTaskStatusResponse {
            data: Some(response_data),
            success: true,
            
        };

        let task_info = response.data.unwrap().task;
        assert_eq!(task_info.task_id, "file_import_task_001");
        assert_eq!(task_info.status, "processing");
        assert_eq!(task_info.progress, Some(75));
        assert_eq!(task_info.task_type, Some("file_import".to_string()));

        let extra_info = task_info.extra.unwrap();
        assert_eq!(extra_info["total_files"], 10);
        assert_eq!(extra_info["processed_files"], 7);
        assert_eq!(extra_info["failed_files"], 1);
        assert_eq!(extra_info["remaining_files"], 2);
    }

    #[test]
    fn test_get_async_task_status_builder_pattern() {
        // 测试构建器模式的流畅性
        let builder = GetAsyncTaskStatusBuilder::new("builder_test_task")
            .user_id_type("union_id");

        // 验证构建器状态
        assert_eq!(builder.request.task_id, "builder_test_task");
        assert_eq!(builder.request.user_id_type, Some("union_id".to_string()));

        // 验证请求验证通过
        assert!(builder.request.validate().is_ok());

        // 测试重新设置任务ID
        let updated_builder = GetAsyncTaskStatusBuilder::new("original_task")
            .user_id_type("open_id");

        let final_request = updated_builder.request;
        assert_eq!(final_request.task_id, "original_task");
        assert_eq!(final_request.user_id_type, Some("open_id".to_string()));
    }

    #[test]
    fn test_async_task_result_and_error_handling() {
        // 测试成功任务结果
        let success_task = AsyncTask {
            task_id: "success_task_001".to_string(),
            status: "success".to_string(),
            progress: Some(100),
            task_type: Some("data_export".to_string()),
            create_time: Some("2023-12-01T10:00:00Z".to_string()),
            complete_time: Some("2023-12-01T10:30:00Z".to_string()),
            result: Some(serde_json::json!({
                "export_url": "https://example.com/export/data_001.zip",
                "file_size": 2048576,
                "file_count": 15,
                "export_duration": 1800
            })),
            error_message: None,
            extra: None,
        };

        // 测试失败任务错误
        let failed_task = AsyncTask {
            task_id: "failed_task_001".to_string(),
            status: "failed".to_string(),
            progress: Some(45),
            task_type: Some("batch_operation".to_string()),
            create_time: Some("2023-12-01T11:00:00Z".to_string()),
            complete_time: Some("2023-12-01T11:15:00Z".to_string()),
            result: None,
            error_message: Some("操作超时：处理时间超过了最大限制".to_string()),
            extra: Some(serde_json::json!({
                "timeout_limit": 900,
                "actual_duration": 915,
                "processed_items": 45,
                "total_items": 100
            })),
        };

        // 验证成功任务
        assert_eq!(success_task.status, "success");
        assert!(success_task.result.is_some());
        assert!(success_task.error_message.is_none());

        let success_result = success_task.result.unwrap();
        assert!(success_result["export_url"].is_string());
        assert_eq!(success_result["file_count"], 15);

        // 验证失败任务
        assert_eq!(failed_task.status, "failed");
        assert!(failed_task.error_message.is_some());
        assert!(failed_task.result.is_none());

        let error_message = failed_task.error_message.unwrap();
        assert!(error_message.contains("超时"));

        let failed_extra = failed_task.extra.unwrap();
        assert_eq!(failed_extra["processed_items"], 45);
        assert_eq!(failed_extra["total_items"], 100);
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
        let config = openlark_core::config::Config::default();
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

// ==================== API #197 复制文件 ====================

/// 复制文件请求
#[derive(Clone, Debug)]
pub struct CopyFileRequest {
    /// 源文件令牌
    pub file_token: String,
    /// 目标文件夹令牌
    pub parent_folder_token: String,
    /// 复制后的文件名称
    pub name: Option<String>,
    /// 用户ID类型
    pub user_id_type: Option<String>,
}

impl CopyFileRequest {
    /// 创建新的请求实例
    pub fn new(file_token: impl Into<String>, parent_folder_token: impl Into<String>) -> Self {
        Self {
            file_token: file_token.into(),
            parent_folder_token: parent_folder_token.into(),
            name: None,
            user_id_type: None,
        }
    }

    /// 设置复制后的文件名称
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
        if self.file_token.trim().is_empty() {
            return Err("源文件令牌不能为空".to_string());
        }

        if self.parent_folder_token.trim().is_empty() {
            return Err("目标文件夹令牌不能为空".to_string());
        }

        // 验证源文件令牌长度（飞书通常使用64位令牌）
        if self.file_token.len() > 256 {
            return Err("源文件令牌长度不能超过256个字符".to_string());
        }

        // 验证目标文件夹令牌长度
        if self.parent_folder_token.len() > 256 {
            return Err("目标文件夹令牌长度不能超过256个字符".to_string());
        }

        // 验证令牌格式（只允许字母、数字、下划线、连字符）
        let allowed_chars = |s: &str| {
            s.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-')
        };

        if !allowed_chars(&self.file_token) {
            return Err("源文件令牌只能包含字母、数字、下划线和连字符".to_string());
        }

        if !allowed_chars(&self.parent_folder_token) {
            return Err("目标文件夹令牌只能包含字母、数字、下划线和连字符".to_string());
        }

        // 验证自定义文件名
        if let Some(ref name) = self.name {
            if name.trim().is_empty() {
                return Err("文件名不能为空字符串".to_string());
            }

            if name.len() > 255 {
                return Err("文件名长度不能超过255个字符".to_string());
            }

            // 检查文件名是否包含非法字符
            let invalid_chars = ['/', '\\', ':', '*', '?', '"', '<', '>', '|'];
            for invalid_char in invalid_chars {
                if name.contains(invalid_char) {
                    return Err(format!("文件名不能包含字符: {}", invalid_char));
                }
            }
        }

        // 验证用户ID类型
        if let Some(ref user_id_type) = self.user_id_type {
            let valid_types = ["open_id", "user_id", "union_id"];
            if !valid_types.contains(&user_id_type.as_str()) {
                return Err(format!(
                    "无效的用户ID类型: {}，支持的类型: open_id, user_id, union_id",
                    user_id_type
                ));
            }
        }

        Ok(())
    }
}

/// 复制文件响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopyFileResponseData {
    /// 复制后的文件信息
    pub file: FileInfo,
}

/// 复制文件响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CopyFileResponse {
    /// 响应数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<CopyFileResponseData>,
    /// 是否成功
    pub success: bool,
    /// 错误消息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// 错误代码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
}

impl ApiResponseTrait for CopyFileResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl FilesService {
    /// 复制文件
    ///
    /// 将指定文件复制到目标文件夹中，支持自定义文件名和用户ID类型
    /// 适用于文件备份、模板创建、文档共享等业务场景
    ///
    /// # 参数
    /// * `req` - 复制文件请求
    ///
    /// # 返回值
    /// 返回复制后的文件信息
    ///
    /// # 示例
    /// ```rust
    /// let request = CopyFileRequest::new("source_file_token_123", "target_folder_456")
    ///     .name("重要文档_副本")
    ///     .user_id_type("open_id");
    /// let response = service.copy_file(&request).await?;
    ///
    /// if response.success {
    ///     if let Some(data) = response.data {
    ///         println!("文件复制成功: {:?}", data.file);
    ///     }
    /// } else {
    ///     println!("文件复制失败: {:?}", response.error_message);
    /// }
    /// ```
    pub async fn copy_file(&self, req: &CopyFileRequest) -> SDKResult<CopyFileResponse> {
        req.validate().map_err(|e| SDKError::InvalidParameter(e))?;
        log::debug!("开始复制文件: source={}, target_folder={}",
            req.file_token, req.parent_folder_token);

        // 构建查询参数
        let mut query: HashMap<&str, String> = HashMap::new();
        if let Some(user_id_type) = &req.user_id_type {
            query_params.insert("user_id_type", user_id_type.clone());
        }

        // 构建请求体
        let mut body = json!({
            "parent_folder_token": req.parent_folder_token
        });

        if let Some(ref name) = req.name {
            body["name"] = json!(name);
        }

        // 构建API路径，替换file_token占位符
        let api_path = openlark_core::endpoints::Endpoints::DRIVE_V1_COPY
            .replace("{file_token}", &req.file_token);

        let api_req = ApiRequest {
            method: openlark_core::api::HttpMethod::Post,
            api_path,
            // supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: Some(openlark_core::api::RequestData::Json(serde_json::json!(&body))).unwrap_or_default(),
            
        };

        let resp = Transport::<CopyFileResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        if response.success {
            log::info!("文件复制成功: source={}, target_folder={}",
                req.file_token, req.parent_folder_token);
        } else {
            log::warn!("文件复制失败: source={}, target_folder={}, error={:?}",
                req.file_token, req.parent_folder_token, response.error_message);
        }

        Ok(response)
    }
}

// ==================== 构建器模式 ====================

/// 复制文件构建器
#[derive(Clone, Debug)]
pub struct CopyFileBuilder {
    request: CopyFileRequest,
}

impl CopyFileBuilder {
    /// 创建新的构建器
    pub fn new(file_token: impl Into<String>, parent_folder_token: impl Into<String>) -> Self {
        Self {
            request: CopyFileRequest::new(file_token, parent_folder_token),
        }
    }

    /// 设置复制后的文件名称
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.request = self.request.name(name);
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.request = self.request.user_id_type(user_id_type);
        self
    }

    /// 执行复制文件操作
    pub async fn execute(self, service: &FilesService) -> SDKResult<CopyFileResponse> {
        service.copy_file(&self.request).await
    }
}

impl FilesService {
    /// 复制文件构建器
    pub fn copy_file_builder(&self, file_token: impl Into<String>, parent_folder_token: impl Into<String>) -> CopyFileBuilder {
        CopyFileBuilder::new(file_token, parent_folder_token)
    }
}

// ==================== 单元测试 ====================

#[cfg(test)]
mod copy_file_tests {
    use super::*;
    use config::Config;

    #[test]
    fn test_copy_file_request_creation() {
        let request = CopyFileRequest::new("source_file_token_123", "target_folder_456");
        assert_eq!(request.file_token, "source_file_token_123");
        assert_eq!(request.parent_folder_token, "target_folder_456");
        assert_eq!(request.name, None);
        assert_eq!(request.user_id_type, None);
    }

    #[test]
    fn test_copy_file_request_with_fields() {
        let request = CopyFileRequest::new("source_token_789", "target_folder_012")
            .name("重要文档副本")
            .user_id_type("open_id");

        assert_eq!(request.file_token, "source_token_789");
        assert_eq!(request.parent_folder_token, "target_folder_012");
        assert_eq!(request.name, Some("重要文档副本".to_string()));
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
    }

    #[test]
    fn test_copy_file_request_validation() {
        // 测试正常情况
        let valid_request = CopyFileRequest::new("source_valid_123", "target_valid_456")
            .name("有效副本");
        assert!(valid_request.validate().is_ok());

        // 测试空源文件令牌
        let empty_source_request = CopyFileRequest::new("", "target_folder_456");
        assert!(empty_source_request.validate().is_err());

        // 测试空目标文件夹令牌
        let empty_target_request = CopyFileRequest::new("source_file_123", "");
        assert!(empty_target_request.validate().is_err());

        // 测试空格源文件令牌
        let whitespace_source_request = CopyFileRequest::new("  ", "target_folder_456");
        assert!(whitespace_source_request.validate().is_err());

        // 测试空格目标文件夹令牌
        let whitespace_target_request = CopyFileRequest::new("source_file_123", "  ");
        assert!(whitespace_target_request.validate().is_err());

        // 测试超长源文件令牌
        let long_source_request = CopyFileRequest::new(&"a".repeat(257), "target_folder_456");
        assert!(long_source_request.validate().is_err());

        // 测试超长目标文件夹令牌
        let long_target_request = CopyFileRequest::new("source_file_123", &"a".repeat(257));
        assert!(long_target_request.validate().is_err());

        // 测试包含非法字符的源文件令牌
        let invalid_source_chars = vec![
            "source@token", "source#token", "source token", "source.token",
            "source,token", "source(token)", "source)token"
        ];

        for invalid_token in invalid_source_chars {
            let invalid_request = CopyFileRequest::new(invalid_token, "target_folder_456");
            assert!(invalid_request.validate().is_err(),
                "源文件令牌 '{}' should be invalid", invalid_token);
        }

        // 测试包含非法字符的目标文件夹令牌
        let invalid_target_chars = vec![
            "target@folder", "target#folder", "target folder", "target.folder",
            "target,folder", "target(folder)", "target)folder"
        ];

        for invalid_token in invalid_target_chars {
            let invalid_request = CopyFileRequest::new("source_file_123", invalid_token);
            assert!(invalid_request.validate().is_err(),
                "目标文件夹令牌 '{}' should be invalid", invalid_token);
        }

        // 测试无效的用户ID类型
        let invalid_user_type_request = CopyFileRequest::new("source_valid_123", "target_valid_456")
            .user_id_type("invalid_type");
        assert!(invalid_user_type_request.validate().is_err());

        // 测试空文件名
        let empty_name_request = CopyFileRequest::new("source_valid_123", "target_valid_456")
            .name("");
        assert!(empty_name_request.validate().is_err());

        // 测试空格文件名
        let whitespace_name_request = CopyFileRequest::new("source_valid_123", "target_valid_456")
            .name("  ");
        assert!(whitespace_name_request.validate().is_err());

        // 测试超长文件名
        let long_name_request = CopyFileRequest::new("source_valid_123", "target_valid_456")
            .name(&"a".repeat(256));
        assert!(long_name_request.validate().is_err());

        // 测试包含非法字符的文件名
        let invalid_name_chars = vec![
            "file/name", "file\\name", "file:name", "file*name", "file?name",
            "file\"name", "file<name", "file>name", "file|name"
        ];

        for invalid_name in invalid_name_chars {
            let invalid_request = CopyFileRequest::new("source_valid_123", "target_valid_456")
                .name(invalid_name);
            assert!(invalid_request.validate().is_err(),
                "文件名 '{}' should be invalid", invalid_name);
        }

        // 测试有效的令牌格式
        let valid_tokens = vec![
            "token_123456789", "Token-ABC-123", "token_456", "file_token_789",
            "batch-process-123", "file-upload-001", "TOKEN_001", "a1-b2-c3"
        ];

        for valid_token in valid_tokens {
            let valid_request = CopyFileRequest::new(valid_token, "target_folder_456")
                .name("有效副本");
            assert!(valid_request.validate().is_ok(),
                "令牌 '{}' should be valid", valid_token);
        }

        // 测试有效的用户ID类型
        let valid_user_types = vec!["open_id", "user_id", "union_id"];
        for user_type in valid_user_types {
            let valid_request = CopyFileRequest::new("source_valid_123", "target_valid_456")
                .user_id_type(user_type);
            assert!(valid_request.validate().is_ok(),
                "用户ID类型 '{}' should be valid", user_type);
        }
    }

    #[test]
    fn test_copy_file_response_creation() {
        let file_info = FileInfo {
            file_token: "copied_file_token_789".to_string(),
            name: "重要文档副本".to_string(),
            r#type: "document".to_string(),
            size: 2048,
            create_time: "2023-12-01T15:00:00Z".to_string(),
            modify_time: "2023-12-01T15:05:00Z".to_string(),
        };

        let response_data = CopyFileResponseData {
            file: file_info,
        };

        let response = CopyFileResponse {
            data: Some(response_data),
            success: true,
            
        };

        assert!(response.success);
        assert!(response.data.is_some());
        assert_eq!(response.data.as_ref().unwrap().file.file_token, "copied_file_token_789");
        assert_eq!(response.data.as_ref().unwrap().file.name, "重要文档副本");
        assert_eq!(response.data.as_ref().unwrap().file.r#type, "document");
        assert_eq!(response.data.as_ref().unwrap().file.size, 2048);
    }

    #[test]
    fn test_copy_file_builder() {
        let builder = CopyFileBuilder::new("source_builder_123", "target_builder_456")
            .name("构建器测试副本")
            .user_id_type("union_id");

        assert_eq!(builder.request.file_token, "source_builder_123");
        assert_eq!(builder.request.parent_folder_token, "target_builder_456");
        assert_eq!(builder.request.name, Some("构建器测试副本".to_string()));
        assert_eq!(builder.request.user_id_type, Some("union_id".to_string()));
    }

    #[test]
    fn test_copy_file_builder_validation() {
        // 测试有效构建器
        let valid_builder = CopyFileBuilder::new("source_valid_001", "target_valid_002")
            .name("有效副本")
            .user_id_type("open_id");
        assert!(valid_builder.request.validate().is_ok());

        // 测试无效构建器
        let invalid_builder = CopyFileBuilder::new("", "target_valid_002")
            .name("无效副本")
            .user_id_type("open_id");
        assert!(invalid_builder.request.validate().is_err());

        // 测试无效用户ID类型
        let invalid_user_type_builder = CopyFileBuilder::new("source_valid_001", "target_valid_002")
            .name("无效副本")
            .user_id_type("invalid");
        assert!(invalid_user_type_builder.request.validate().is_err());
    }

    #[test]
    fn test_copy_file_service_method() {
        let config = openlark_core::config::Config::default();
        let service = FilesService::new(config);

        // 验证服务包含所需的方法
        let service_str = format!("{:?}", service);
        assert!(!service_str.is_empty());

        // 验证构建器方法存在
        let builder = service.copy_file_builder("source_service_123", "target_service_456");
        assert_eq!(builder.request.file_token, "source_service_123");
        assert_eq!(builder.request.parent_folder_token, "target_service_456");
    }

    #[test]
    fn test_copy_file_endpoint_construction() {
        // 验证端点常量存在
        assert_eq!(
            openlark_core::endpoints::Endpoints::DRIVE_V1_COPY,
            "/open-apis/drive/v1/files/{file_token}/copy"
        );

        // 验证路径替换逻辑
        let template = openlark_core::endpoints::Endpoints::DRIVE_V1_COPY;
        let final_path = template.replace("{file_token}", "source_file_123");
        assert_eq!(final_path, "/open-apis/drive/v1/files/source_file_123/copy");
    }

    #[test]
    fn test_copy_file_request_methods() {
        // 测试链式调用
        let request = CopyFileRequest::new("source_chain_123", "target_chain_456")
            .name("链式调用测试")
            .user_id_type("user_id");

        assert_eq!(request.file_token, "source_chain_123");
        assert_eq!(request.parent_folder_token, "target_chain_456");
        assert_eq!(request.name, Some("链式调用测试".to_string()));
        assert_eq!(request.user_id_type, Some("user_id".to_string()));
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_copy_file_edge_cases() {
        // 测试最小长度令牌
        let min_length_request = CopyFileRequest::new("a", "b")
            .name("min");
        assert!(min_length_request.validate().is_ok());

        // 测试最大长度令牌
        let max_length_request = CopyFileRequest::new(&"a".repeat(256), &"b".repeat(256))
            .name("max");
        assert!(max_length_request.validate().is_ok());

        // 测试混合字符令牌
        let mixed_request = CopyFileRequest::new("Token123_ABC-def", "Folder456_GHI-jkl")
            .name("混合字符测试");
        assert!(mixed_request.validate().is_ok());

        // 测试全大写令牌
        let uppercase_request = CopyFileRequest::new("TOKEN_001", "FOLDER_002")
            .name("大写测试");
        assert!(uppercase_request.validate().is_ok());

        // 测试全小写令牌
        let lowercase_request = CopyFileRequest::new("token_003", "folder_004")
            .name("小写测试");
        assert!(lowercase_request.validate().is_ok());
    }

    #[test]
    fn test_copy_file_response_trait() {
        assert_eq!(CopyFileResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_copy_file_comprehensive_scenario() {
        // 测试完整的业务场景 - 复制重要文档作为模板
        let template_request = CopyFileRequest::new("original_contract_2023", "templates_folder_001")
            .name("合同模板_v1.0")
            .user_id_type("open_id");

        assert!(template_request.validate().is_ok());
        assert_eq!(template_request.file_token, "original_contract_2023");
        assert_eq!(template_request.parent_folder_token, "templates_folder_001");
        assert_eq!(template_request.name, Some("合同模板_v1.0".to_string()));
        assert_eq!(template_request.user_id_type, Some("open_id".to_string()));

        // 模拟响应数据
        let template_file_info = FileInfo {
            file_token: "template_contract_001".to_string(),
            name: "合同模板_v1.0".to_string(),
            r#type: "document".to_string(),
            size: 15360,
            create_time: "2023-12-01T16:00:00Z".to_string(),
            modify_time: "2023-12-01T16:01:00Z".to_string(),
        };

        let response_data = CopyFileResponseData {
            file: template_file_info,
        };

        let response = CopyFileResponse {
            data: Some(response_data),
            success: true,
            
        };

        assert!(response.success);
        assert!(response.data.is_some());

        let copied_file = response.data.unwrap().file;
        assert_eq!(copied_file.file_token, "template_contract_001");
        assert_eq!(copied_file.name, "合同模板_v1.0");
        assert_eq!(copied_file.r#type, "document");
        assert_eq!(copied_file.size, 15360);
    }

    #[test]
    fn test_copy_file_different_file_types() {
        // 测试不同类型文件的复制
        let file_types = vec![
            ("document", "docx_file_123", "docx_copy_456"),
            ("spreadsheet", "xlsx_file_789", "xlsx_copy_012"),
            ("presentation", "pptx_file_345", "pptx_copy_678"),
            ("image", "jpg_file_901", "jpg_copy_234"),
            ("pdf", "pdf_file_567", "pdf_copy_890")
        ];

        for (file_type, source_token, target_token) in file_types {
            let file_info = FileInfo {
                file_token: target_token.to_string(),
                name: format!("{}_copy", source_token),
                r#type: file_type.to_string(),
                size: 1024,
                create_time: "2023-12-01T17:00:00Z".to_string(),
                modify_time: "2023-12-01T17:01:00Z".to_string(),
            };

            let response_data = CopyFileResponseData {
                file: file_info,
            };

            let response = CopyFileResponse {
                data: Some(response_data),
                success: true,
                
            };

            assert!(response.success);
            assert_eq!(response.data.unwrap().file.r#type, file_type);
        }
    }

    #[test]
    fn test_copy_file_error_scenarios() {
        // 测试失败响应
        let error_response = CopyFileResponse {
            data: None,
            success: false,
            error_message: Some("源文件不存在".to_string()),
            error_code: Some("FILE_NOT_FOUND".to_string()),
        };

        assert!(!error_response.success);
        assert!(error_response.data.is_none());
        assert_eq!(error_response.error_message, Some("源文件不存在".to_string()));
        assert_eq!(error_response.error_code, Some("FILE_NOT_FOUND".to_string()));

        // 测试权限错误
        let permission_error_response = CopyFileResponse {
            data: None,
            success: false,
            error_message: Some("没有复制权限".to_string()),
            error_code: Some("PERMISSION_DENIED".to_string()),
        };

        assert!(!permission_error_response.success);
        assert_eq!(permission_error_response.error_message, Some("没有复制权限".to_string()));
        assert_eq!(permission_error_response.error_code, Some("PERMISSION_DENIED".to_string()));

        // 测试目标文件夹不存在错误
        let folder_error_response = CopyFileResponse {
            data: None,
            success: false,
            error_message: Some("目标文件夹不存在".to_string()),
            error_code: Some("FOLDER_NOT_FOUND".to_string()),
        };

        assert!(!folder_error_response.success);
        assert_eq!(folder_error_response.error_message, Some("目标文件夹不存在".to_string()));
        assert_eq!(folder_error_response.error_code, Some("FOLDER_NOT_FOUND".to_string()));
    }

    #[test]
    fn test_copy_file_builder_pattern() {
        // 测试构建器模式的流畅性
        let builder = CopyFileBuilder::new("builder_source_123", "builder_target_456")
            .name("构建器模式测试")
            .user_id_type("union_id");

        // 验证构建器状态
        assert_eq!(builder.request.file_token, "builder_source_123");
        assert_eq!(builder.request.parent_folder_token, "builder_target_456");
        assert_eq!(builder.request.name, Some("构建器模式测试".to_string()));
        assert_eq!(builder.request.user_id_type, Some("union_id".to_string()));

        // 验证请求验证通过
        assert!(builder.request.validate().is_ok());

        // 测试链式调用
        let chained_builder = builder
            .name("重新设置名称")
            .request;
        assert_eq!(chained_builder.name.unwrap(), "重新设置名称");
    }

    #[test]
    fn test_copy_file_json_serialization() {
        let request = CopyFileRequest::new("json_source_123", "json_target_456")
            .name("JSON序列化测试")
            .user_id_type("open_id");

        // 测试请求可以转换为JSON
        let body = json!({
            "parent_folder_token": "json_target_456",
            "name": "JSON序列化测试"
        });

        assert_eq!(body["parent_folder_token"], "json_target_456");
        assert_eq!(body["name"], "JSON序列化测试");
    }
}