//! API匹配引擎
//!
//! 负责将提取的URL定义与API列表进行匹配。

use anyhow::{Result, Context};
use csv::Reader;
use std::collections::HashMap;
use std::fs::File;
use tracing::{info, debug};

use crate::models::{URLDefinition, APIInfo, APIMatch, MatchStatus, HTTPMethod};
use crate::normalizer::ParameterNormalizer;

/// API匹配器
pub struct APIMatcher {
    /// API名称到URL的映射
    api_name_to_url: HashMap<String, APIInfo>,
    /// 参数标准化器
    parameter_normalizer: ParameterNormalizer,
}

impl APIMatcher {
    /// 创建新的API匹配器
    pub async fn new(api_list_path: &str) -> Result<Self> {
        let mut api_name_to_url = HashMap::new();

        // 读取CSV文件
        let file = File::open(api_list_path)
            .with_context(|| format!("Failed to open API list file: {}", api_list_path))?;

        let mut reader = Reader::from_reader(file);
        let headers = reader.headers()?;

        info!("开始加载API列表: {}", api_list_path);

        let mut api_count = 0;
        for result in reader.records() {
            let record = result?;

            if record.len() >= 7 {
                let name = record.get(0).unwrap_or("").to_string();
                let method_str = record.get(1).unwrap_or("POST");
                let path = record.get(2).unwrap_or("");
                let description = record.get(3).unwrap_or("").to_string();
                let self_build = record.get(4).unwrap_or("").to_string();
                let store_app = record.get(5).unwrap_or("").to_string();
                let doc_link = record.get(6).unwrap_or("").to_string();

                // 解析HTTP方法
                let method = HTTPMethod::from_str(method_str).unwrap_or_default();

                // 标准化路径
                let normalized_path = Self::normalize_url_path(path);

                // 提取服务和版本信息
                let (service, version) = Self::extract_service_info(&normalized_path);

                let api_info = APIInfo {
                    name,
                    method,
                    path: normalized_path,
                    description,
                    self_build,
                    store_app,
                    doc_link,
                    service,
                    version,
                };

                // 创建唯一键
                let api_key = format!("{}:{}", api_info.method.as_str(), api_info.path);
                api_name_to_url.insert(api_key, api_info);

                api_count += 1;
            }
        }

        info!("加载完成，共 {} 个API定义", api_count);

        Ok(Self {
            api_name_to_url,
            parameter_normalizer: ParameterNormalizer::default(),
        })
    }

    /// 获取API数量
    pub fn api_count(&self) -> usize {
        self.api_name_to_url.len()
    }

    /// 执行API匹配
    pub async fn match_apis(&self, url_definitions: &[URLDefinition]) -> Result<Vec<APIMatch>> {
        info!("开始API匹配，URL定义数: {}, API总数: {}",
              url_definitions.len(), self.api_name_to_url.len());

        // 建立标准化后的URL到函数的映射
        let mut url_function_map = HashMap::new();
        for url_def in url_definitions {
            // 对代码中的URL进行参数标准化
            let normalized_code_url = self.parameter_normalizer
                .normalize_url_with_variables(&url_def.url, &url_def.variables)
                .unwrap_or_else(|_| url_def.url.clone());

            let url_key = format!("{}:{}",
                                  url_def.method_detection.method.as_str(),
                                  normalized_code_url);
            url_function_map.insert(url_key, url_def);
        }

        let mut results = Vec::new();
        let mut matched_count = 0;

        for (api_key, api_info) in &self.api_name_to_url {
            // 对API列表中的URL也进行标准化，确保格式一致
            let normalized_api_url = self.parameter_normalizer
                .normalize_url_with_variables(&api_info.path, &[])
                .unwrap_or_else(|_| api_info.path.clone());

            let normalized_api_key = format!("{}:{}",
                                            api_info.method.as_str(),
                                            normalized_api_url);

            if let Some(url_def) = url_function_map.get(&normalized_api_key) {
                // 找到匹配
                let match_confidence = self.calculate_match_confidence(api_info, url_def);

                let result = APIMatch {
                    api_info: api_info.clone(),
                    implementation: Some(URLDefinition {
                    url: url_def.url.clone(),
                    method_detection: url_def.method_detection.clone(),
                    line_start: url_def.line_start,
                    line_end: url_def.line_end,
                    raw_format: url_def.raw_format.clone(),
                    variables: url_def.variables.clone(),
                    extraction_type: url_def.extraction_type.clone(),
                    file_path: url_def.file_path.clone(),
                }),
                    function_info: None, // TODO: 实现函数信息提取
                    status: MatchStatus::Found,
                    match_confidence,
                    implementation_preview: format!("{}:{}", url_def.file_path.display(), url_def.line_start),
                };

                matched_count += 1;
                results.push(result);
            } else {
                // 未找到匹配
                let result = APIMatch {
                    api_info: api_info.clone(),
                    implementation: None,
                    function_info: None,
                    status: MatchStatus::NotFound,
                    match_confidence: 0.0,
                    implementation_preview: "-".to_string(),
                };

                results.push(result);
            }
        }

        let match_rate = (matched_count as f64 / results.len() as f64) * 100.0;

        info!("匹配完成！");
        info!("  总API数: {}", results.len());
        info!("  成功匹配: {}", matched_count);
        info!("  匹配率: {:.1}%", match_rate);

        Ok(results)
    }

    /// 标准化URL路径格式
    fn normalize_url_path(path: &str) -> String {
        let normalizer = ParameterNormalizer::default();
        // 使用标准化器处理路径，但不传入变量信息
        normalizer.normalize_url_with_variables(path, &[]).unwrap_or_else(|_| path.to_string())
    }

    /// 从路径提取服务和版本信息
    fn extract_service_info(path: &str) -> (String, String) {
        let parts: Vec<&str> = path.trim_start_matches('/').split('/').collect();

        if parts.len() >= 3 && parts[0] == "open-apis" {
            let service = parts[1].to_string();
            let version = if parts[2].starts_with('v') {
                parts[2].to_string()
            } else {
                "unknown".to_string()
            };
            return (service, version);
        }

        ("unknown".to_string(), "unknown".to_string())
    }

    /// 计算匹配置信度
    fn calculate_match_confidence(&self, api_info: &APIInfo, url_def: &URLDefinition) -> f32 {
        let mut confidence = 1.0;

        // HTTP方法匹配权重
        if api_info.method == url_def.method_detection.method {
            confidence += url_def.method_detection.confidence * 0.2;
        }

        // URL路径匹配权重
        let (url_matches, _) = self.parameter_normalizer
            .validate_url_mapping(&api_info.path, &url_def.url)
            .unwrap_or((false, "验证失败".to_string()));

        if url_matches {
            confidence += 0.5;
        } else {
            confidence -= 0.3;
        }

        // HTTP方法检测置信度权重
        confidence += url_def.method_detection.confidence * 0.1;

        // 变量信息完整性权重
        if !url_def.variables.is_empty() {
            confidence += 0.1;
        }

        confidence.max(0.0).min(1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn test_api_matcher_creation() {
        // 创建临时CSV文件
        let mut temp_file = NamedTempFile::new().unwrap();
        let csv_content = r#"name,method,path,description,self_build,store_app,doc_link
Test API,GET,/open-apis/test/v1/resource,Test API,yes,yes,https://example.com
"#;
        temp_file.write_all(csv_content.as_bytes()).unwrap();

        let matcher = APIMatcher::new(temp_file.path().to_str().unwrap()).await.unwrap();
        assert_eq!(matcher.api_count(), 1);
    }

    #[test]
    fn test_extract_service_info() {
        let (service, version) = APIMatcher::extract_service_info("/open-apis/sheets/v3/spreadsheets");
        assert_eq!(service, "sheets");
        assert_eq!(version, "v3");

        let (service, version) = APIMatcher::extract_service_info("/other/path");
        assert_eq!(service, "unknown");
        assert_eq!(version, "unknown");
    }

    #[test]
    fn test_normalize_url_path() {
        let normalized = APIMatcher::normalize_url_path("/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/values");
        // 应该被标准化器处理
        assert!(normalized.contains("/open-apis/sheets/v2/spreadsheets/"));
    }
}