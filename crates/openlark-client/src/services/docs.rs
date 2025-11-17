//! 文档服务适配器
//!
//! 将openlark-docs服务适配到统一客户端接口。

use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;

use crate::unified::{
    traits::{UnifiedService, ServiceDescriptor, ServiceStatus, ServiceLifecycle},
    config::{UnifiedConfig, DocsConfig},
    error::{UnifiedError, UnifiedResult},
};

/// 文档服务适配器
///
/// 将openlark-docs的功能适配到统一客户端接口。
#[derive(Debug, Clone)]
pub struct DocsService {
    /// 服务配置
    config: Option<DocsConfig>,
    /// 服务状态
    status: ServiceStatus,
    /// 核心客户端（用于实际API调用）
    core_client: Option<Arc<openlark_core::client::LarkClient>>,
    /// 服务元数据
    metadata: HashMap<String, String>,
}

impl DocsService {
    /// 创建新的文档服务适配器
    pub fn new() -> Self {
        Self {
            config: None,
            status: ServiceStatus::Uninitialized,
            core_client: None,
            metadata: HashMap::new(),
        }
    }

    /// 从配置创建服务
    pub fn with_config(mut self, config: DocsConfig) -> Self {
        self.config = Some(config);
        self
    }

    /// 从核心客户端创建服务
    pub fn with_core_client(mut self, core_client: Arc<openlark_core::client::LarkClient>) -> Self {
        self.core_client = Some(core_client);
        self
    }

    /// 检查服务是否可用
    pub fn is_enabled(&self) -> bool {
        self.config
            .as_ref()
            .map(|config| config.enabled)
            .unwrap_or(false)
    }

    /// 创建电子表格
    pub async fn create_spreadsheet(
        &self,
        title: &str,
        folder_token: Option<&str>,
    ) -> UnifiedResult<Spreadsheet> {
        self.ensure_available()?;

        let create_request = serde_json::json!({
            "title": title,
            "folder_token": folder_token
        });

        tracing::info!("创建电子表格: {:?}", create_request);

        // 模拟返回表格数据
        Ok(Spreadsheet {
            spreadsheet_token: "mock_spreadsheet_token".to_string(),
            title: title.to_string(),
            url: "https://docs.feishu.cn/sheets/mock".to_string(),
            create_time: chrono::Utc::now(),
            creator: "Mock User".to_string(),
        })
    }

    /// 获取电子表格信息
    pub async fn get_spreadsheet(&self, spreadsheet_token: &str) -> UnifiedResult<Spreadsheet> {
        self.ensure_available()?;

        tracing::info!("获取电子表格信息: {}", spreadsheet_token);

        Ok(Spreadsheet {
            spreadsheet_token: spreadsheet_token.to_string(),
            title: "Mock Spreadsheet".to_string(),
            url: "https://docs.feishu.cn/sheets/mock".to_string(),
            create_time: chrono::Utc::now(),
            creator: "Mock User".to_string(),
        })
    }

    /// 读取工作表范围
    pub async fn read_range(
        &self,
        spreadsheet_token: &str,
        range: &str,
        value_render_option: Option<&str>,
    ) -> UnifiedResult<RangeReadResult> {
        self.ensure_available()?;

        let read_request = serde_json::json!({
            "spreadsheet_token": spreadsheet_token,
            "range": range,
            "value_render_option": value_render_option.unwrap_or("DisplayValue")
        });

        tracing::info!("读取工作表范围: {:?}", read_request);

        Ok(RangeReadResult {
            range: range.to_string(),
            values: vec![vec![["Mock Data"]]],
            updated_time: chrono::Utc::now(),
        })
    }

    /// 写入工作表范围
    pub async fn write_range(
        &self,
        spreadsheet_token: &str,
        range: &str,
        values: Vec<Vec<String>>,
    ) -> UnifiedResult<WriteResult> {
        self.ensure_available()?;

        let write_request = serde_json::json!({
            "spreadsheet_token": spreadsheet_token,
            "range": range,
            "values": values
        });

        tracing::info!("写入工作表范围: {:?}", write_request);

        Ok(WriteResult {
            range: range.to_string(),
            updated_rows: values.len(),
            updated_time: chrono::Utc::now(),
        })
    }

    /// 上传文件
    pub async fn upload_file(
        &self,
        file_path: &str,
        parent_type: &str,
        parent_node: Option<&str>,
    ) -> UnifiedResult<FileUploadResult> {
        self.ensure_available()?;

        let upload_request = serde_json::json!({
            "file_path": file_path,
            "parent_type": parent_type,
            "parent_node": parent_node
        });

        tracing::info!("上传文件: {:?}", upload_request);

        Ok(FileUploadResult {
            file_token: "mock_file_token".to_string(),
            file_name: std::path::Path::new(file_path)
                .file_name()
                .unwrap_or("unknown")
                .to_string(),
            file_type: "mock_type".to_string(),
            upload_time: chrono::Utc::now(),
        })
    }

    /// 下载文件
    pub async fn download_file(
        &self,
        file_token: &str,
    ) -> UnifiedResult<FileDownloadResult> {
        self.ensure_available()?;

        tracing::info!("下载文件: {}", file_token);

        Ok(FileDownloadResult {
            file_token: file_token.to_string(),
            file_name: "mock_file.ext".to_string(),
            file_size: 1024,
            download_url: "https://example.com/download".to_string(),
        })
    }

    /// 确保服务可用
    fn ensure_available(&self) -> UnifiedResult<()> {
        if !self.is_enabled() {
            return Err(UnifiedError::ServiceNotAvailable("docs".to_string()));
        }

        if self.status != ServiceStatus::Running {
            return Err(UnifiedError::ServiceNotAvailable(
                "docs service not running".to_string(),
            ));
        }

        Ok(())
    }
}

/// 电子表格信息
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Spreadsheet {
    /// 电子表格Token
    pub spreadsheet_token: String,
    /// 标题
    pub title: String,
    /// 访问URL
    pub url: String,
    /// 创建时间
    pub create_time: chrono::DateTime<chrono::Utc>,
    /// 创建者
    pub creator: String,
}

/// 范围读取结果
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RangeReadResult {
    /// 范围
    pub range: String,
    /// 单元格值
    pub values: Vec<Vec<String>>,
    /// 更新时间
    pub updated_time: chrono::DateTime<chrono::Utc>,
}

/// 写入结果
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WriteResult {
    /// 写入范围
    pub range: String,
    /// 更新的行数
    pub updated_rows: usize,
    /// 更新时间
    pub updated_time: chrono::DateTime<chrono::Utc>,
}

/// 文件上传结果
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FileUploadResult {
    /// 文件Token
    pub file_token: String,
    /// 文件名
    pub file_name: String,
    /// 文件类型
    pub file_type: String,
    /// 上传时间
    pub upload_time: chrono::DateTime<chrono::Utc>,
}

/// 文件下载结果
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FileDownloadResult {
    /// 文件Token
    pub file_token: String,
    /// 文件名
    pub file_name: String,
    /// 文件大小
    pub file_size: u64,
    /// 下载URL
    pub download_url: String,
}

impl Default for DocsService {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl UnifiedService for DocsService {
    type Config = DocsConfig;
    type Error = UnifiedError;

    fn name(&self) -> &'static str {
        "docs"
    }

    fn version(&self) -> &'static str {
        "1.0.0"
    }

    async fn configure(&mut self, config: Self::Config) -> UnifiedResult<()> {
        if !config.enabled {
            self.status = ServiceStatus::Stopped;
            return Ok(());
        }

        self.config = Some(config);

        // 创建核心客户端
        let core_config = self.config.as_ref().map(|config| {
            openlark_core::config::ConfigBuilder::new()
                .base_url(&config.api_url)
                .build()
                .unwrap_or_else(|_| openlark_core::config::Config::default())
        });

        if let Some(core_config) = core_config {
            match openlark_core::client::LarkClient::new(
                core_config.app_id.clone(),
                core_config.app_secret.clone(),
            ) {
                Ok(client) => {
                    self.core_client = Some(Arc::new(client));
                    self.status = ServiceStatus::Running;
                    tracing::info!("文档服务配置成功");
                    Ok(())
                }
                Err(e) => {
                    self.status = ServiceStatus::Error;
                    Err(UnifiedError::ConfigurationError(
                        format!("创建核心客户端失败: {}", e),
                    ))
                }
            }
        } else {
            self.status = ServiceStatus::Error;
            Err(UnifiedError::ConfigurationError("文档配置无效".to_string()))
        }
    }

    fn is_available(&self) -> bool {
        self.is_enabled() && self.status == ServiceStatus::Running && self.core_client.is_some()
    }

    fn status(&self) -> ServiceStatus {
        self.status
    }

    fn descriptor(&self) -> ServiceDescriptor {
        let mut descriptor = ServiceDescriptor::new(
            "docs",
            "1.0.0",
            "飞书文档服务，提供云文档、表格、知识库等功能",
        )
        .with_tag("documents")
        .with_tag("collaboration")
        .with_dependency("openlark-core");

        if let Some(config) = &self.config {
            descriptor = descriptor
                .with_metadata("api_url", config.api_url.clone())
                .with_metadata("enabled", config.enabled.to_string())
                .with_metadata(
                    "max_file_size",
                    config.upload.max_file_size.to_string(),
                )
                .with_metadata(
                    "allowed_types",
                    format!("{:?}", config.upload.allowed_types),
                );
        }

        descriptor
    }
}

#[async_trait]
impl ServiceLifecycle for DocsService {
    async fn start(&mut self) -> SDKResult<()> {
        if let Some(config) = self.config.clone() {
            self.configure(config).await?;
        } else {
            tracing::warn!("文档服务配置未设置，服务将处于未初始化状态");
        }
        Ok(())
    }

    async fn stop(&mut self) -> SDKResult<()> {
        self.status = ServiceStatus::Stopped;
        self.core_client = None;
        tracing::info!("文档服务已停止");
        Ok(())
    }

    async fn health_check(&self) -> SDKResult<bool> {
        Ok(self.is_available())
    }
}

/// 文档服务构建器
pub struct DocsServiceBuilder {
    config: Option<DocsConfig>,
    core_client: Option<Arc<openlark_core::client::LarkClient>>,
}

impl DocsServiceBuilder {
    /// 创建新的构建器
    pub fn new() -> Self {
        Self {
            config: None,
            core_client: None,
        }
    }

    /// 设置配置
    pub fn config(mut self, config: DocsConfig) -> Self {
        self.config = Some(config);
        self
    }

    /// 设置核心客户端
    pub fn core_client(mut self, core_client: Arc<openlark_core::client::LarkClient>) -> Self {
        self.core_client = Some(core_client);
        self
    }

    /// 构建服务
    pub fn build(self) -> UnifiedResult<DocsService> {
        let mut service = DocsService::new();

        if let Some(config) = self.config {
            service = service.with_config(config);
        }

        if let Some(core_client) = self.core_client {
            service = service.with_core_client(core_client);
        }

        Ok(service)
    }
}

impl Default for DocsServiceBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_docs_service_creation() {
        let service = DocsService::new();
        assert_eq!(service.name(), "docs");
        assert_eq!(service.version(), "1.0.0");
    }

    #[test]
    fn test_docs_service_builder() {
        let config = DocsConfig::default();
        let service = DocsServiceBuilder::new()
            .config(config)
            .build()
            .unwrap();

        assert!(service.is_enabled());
    }

    #[tokio::test]
    async fn test_service_lifecycle() {
        let mut service = DocsService::new();

        // 测试启动
        service.start().await.unwrap();
        // 由于没有配置，服务应该是未初始化状态
        assert_eq!(service.status(), ServiceStatus::Stopped);

        // 测试停止
        service.stop().await.unwrap();
        assert_eq!(service.status(), ServiceStatus::Stopped);
    }

    #[tokio::test]
    async fn test_spreadsheet_operations() {
        let service = DocsService::new();

        // 测试创建表格
        let result = service.create_spreadsheet("Test Sheet", None).await;
        assert!(result.is_ok());

        // 测试获取表格信息
        let result = service.get_spreadsheet("mock_token").await;
        assert!(result.is_ok());

        // 测试读取范围
        let result = service.read_range("mock_token", "A1:B2", None).await;
        assert!(result.is_ok());

        // 测试写入范围
        let result = service.write_range("mock_token", "A1:B2", vec![["1", "2"], ["3", "4"]]).await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_service_descriptors() {
        let service = DocsService::new();
        let descriptor = service.descriptor();

        assert_eq!(descriptor.name, "docs");
        assert_eq!(descriptor.version, "1.0.0");
        assert!(descriptor.tags.contains(&"documents".to_string()));
    }
}