//! 服务文件扫描器
//!
//! 负责扫描服务目录中的所有Rust文件，提取URL定义。

use anyhow::{Result, Context};
use std::path::Path;
use tracing::{info, debug};
use walkdir::WalkDir;
use rayon::prelude::*;

use crate::parser::FormatParser;
use crate::detector::HTTPMethodDetector;
use crate::models::URLDefinition;

/// 服务文件扫描器
pub struct ServiceScanner {
    /// 服务目录路径
    service_dir: String,
    /// Format解析器
    format_parser: FormatParser,
    /// HTTP方法检测器
    method_detector: HTTPMethodDetector,
}

impl ServiceScanner {
    /// 创建新的服务扫描器
    pub fn new(service_dir: &str) -> Self {
        Self {
            service_dir: service_dir.to_string(),
            format_parser: FormatParser::default(),
            method_detector: HTTPMethodDetector::default(),
        }
    }

    /// 扫描所有服务文件，提取URL定义
    pub async fn scan_all_services(&self) -> Result<Vec<URLDefinition>> {
        let service_path = Path::new(&self.service_dir);
        if !service_path.exists() {
            return Err(anyhow::anyhow!("Service directory does not exist: {}", self.service_dir));
        }

        info!("开始扫描服务目录: {}", self.service_dir);

        // 收集所有Rust文件
        let rust_files: Vec<std::path::PathBuf> = WalkDir::new(service_path)
            .into_iter()
            .filter_map(|entry| entry.ok())
            .filter(|e| {
                e.file_type().is_file() &&
                e.path().extension().map_or(false, |ext| ext == "rs")
            })
            .map(|e| e.path().to_path_buf())
            .collect();

        info!("发现 {} 个Rust文件", rust_files.len());

        // 并行处理文件
        let url_definitions: Result<Vec<_>> = rust_files
            .par_iter()
            .map(|file_path| self.process_file(file_path))
            .collect();

        let mut all_urls = Vec::new();
        for file_urls in url_definitions? {
            all_urls.extend(file_urls);
        }

        // 更新HTTP方法检测
        for url_def in &mut all_urls {
            if let Ok(content) = std::fs::read_to_string(&url_def.file_path) {
                let lines: Vec<&str> = content.lines().collect();
                url_def.method_detection = self.method_detector
                    .detect_method_from_context(&lines, url_def.line_start - 1, url_def.line_end - 1);
            }
        }

        info!("扫描完成，共提取 {} 个URL定义", all_urls.len());
        Ok(all_urls)
    }

    /// 处理单个文件
    fn process_file(&self, file_path: &std::path::Path) -> Result<Vec<URLDefinition>> {
        let content = std::fs::read_to_string(file_path)
            .with_context(|| format!("Failed to read file: {}", file_path.display()))?;

        // 如果文件不包含format!宏，跳过处理
        if !self.format_parser.contains_format_macro(&content) {
            return Ok(Vec::new());
        }

        debug!("处理文件: {}", file_path.display());

        // 提取URL定义
        let mut url_definitions = self.format_parser
            .extract_format_urls(file_path, &content)?;

        // 检测函数信息（这里先留空，后面由专门的函数定位器处理）
        for url_def in &mut url_definitions {
            // TODO: 实现函数定位逻辑
        }

        Ok(url_definitions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use tempfile::TempDir;

    #[test]
    fn test_scanner_creation() {
        let scanner = ServiceScanner::new("../src/service");
        assert_eq!(scanner.service_dir, "../src/service");
    }

    #[test]
    fn test_file_processing() {
        // 创建临时目录和文件
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test_service.rs");

        let test_content = r#"
use crate::prelude::*;

pub struct TestService {
    pub config: std::sync::Arc<Config>,
}

impl TestService {
    pub async fn create(&self, request: CreateRequest) -> SDKResult<CreateResponse> {
        let url = format!("{}/open-apis/test/v1/resource", self.config.base_url);
        // 更多代码...
    }
}
"#;

        fs::write(&file_path, test_content).unwrap();

        let scanner = ServiceScanner::new("../src/service");
        let urls = scanner.process_file(&file_path).unwrap();

        assert_eq!(urls.len(), 1);
        assert!(urls[0].url.contains("/open-apis/test/v1/resource"));
    }
}