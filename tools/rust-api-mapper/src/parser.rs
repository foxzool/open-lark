//! Format!宏解析器
//!
//! 负责从Rust代码中提取format!宏中的URL定义，
//! 支持单行、多行和ApiRequest等多种模式。

use regex::Regex;
use anyhow::{Result, Context};
use tracing::{debug, trace};

use crate::models::{URLDefinition, HTTPMethodDetection, HTTPMethod, MethodDetectionSource, URLExtractionType};

/// Format!宏解析器
pub struct FormatParser {
    /// 基本的单行format!模式
    single_line_pattern: Regex,
    /// 多行format!开始模式
    multiline_start_pattern: Regex,
    /// URL在format!第一行的模式
    url_first_line_pattern: Regex,
    /// ApiRequest模式
    api_request_pattern: Regex,
    /// 参数占位符模式
    placeholder_pattern: Regex,
    /// Base URL占位符模式 {}/open-apis/
    base_url_placeholder_pattern: Regex,
    /// 变量提取模式
    variable_patterns: Vec<Regex>,
}

impl FormatParser {
    /// 创建新的格式解析器
    pub fn new() -> Result<Self> {
        Ok(Self {
            // 改进的正则表达式，支持 {}/open-apis/ 格式
            single_line_pattern: Regex::new(r#"format!\s*\(\s*"([^"]*(?:/open-apis/|/[^"]*open-apis/)[^"]*)"[^)]*\)"#)
                .context("Failed to compile single line format pattern")?,

            multiline_start_pattern: Regex::new(r"format!\s*\(\s*$")
                .context("Failed to compile multiline start pattern")?,

            // 改进的多行URL模式，支持 {}/open-apis/ 格式
            url_first_line_pattern: Regex::new(r#"format!\s*\(\s*"([^"]*(?:/open-apis/|/[^"]*open-apis/)[^"]*)""#)
                .context("Failed to compile URL first line pattern")?,

            // 改进的ApiRequest模式
            api_request_pattern: Regex::new(
                r#"ApiRequest::with_method_and_path\s*\([^,]*,\s*format!\s*\(\s*"([^"]*(?:/open-apis/|/[^"]*open-apis/)[^"]*)"[^)]*\)"#
            ).context("Failed to compile API request pattern")?,

            placeholder_pattern: Regex::new(r"\{\s*([^}]*)\s*\}")
                .context("Failed to compile placeholder pattern")?,

            // 新增：专门匹配 {}/open-apis/ 格式的模式
            base_url_placeholder_pattern: Regex::new(
                r#"format!\s*\(\s*"\{[^}]*\}(/open-apis/[^"]+)"[^)]*\)"#
            ).context("Failed to compile base URL placeholder pattern")?,

            variable_patterns: vec![
                Regex::new(r"&([a-zA-Z_][a-zA-Z0-9_]*)")
                    .context("Failed to compile reference variable pattern")?,
                Regex::new(r"request\.([a-zA-Z_][a-zA-Z0-9_]*)")
                    .context("Failed to compile request field pattern")?,
                Regex::new(r"([a-zA-Z_][a-zA-Z0-9_]*)\.")
                    .context("Failed to compile object field pattern")?,
                Regex::new(r"&self\.([a-zA-Z_][a-zA-Z0-9_]*)")
                    .context("Failed to compile self field pattern")?,
            ],
        })
    }

    /// 从文件内容中提取所有URL定义
    pub fn extract_format_urls(&self, file_path: &std::path::Path, content: &str) -> Result<Vec<URLDefinition>> {
        let lines: Vec<&str> = content.lines().collect();
        let mut urls_found = Vec::new();

        let mut i = 0;
        while i < lines.len() {
            let line = lines[i].trim();

            // 检查是否是多行format!的开始
            if self.multiline_start_pattern.is_match(line) {
                if let Some(url_info) = self.extract_multiline_format(&lines, i, file_path)? {
                    let line_span = url_info.line_end - url_info.line_start;
                    urls_found.push(url_info);
                    i += line_span;
                    continue;
                }
            }

            // 检查ApiRequest模式
            if let Some(url_info) = self.extract_api_request_format(line, i, file_path)? {
                urls_found.push(url_info);
            }

            // 检查单行format!模式
            let single_line_matches = self.extract_single_line_format(line, i, file_path)?;
            urls_found.extend(single_line_matches);

            i += 1;
        }

        debug!("从文件 {} 中提取了 {} 个URL定义",
               file_path.display(), urls_found.len());

        Ok(urls_found)
    }

    /// 提取多行format!宏中的URL
    fn extract_multiline_format(
        &self,
        lines: &[&str],
        start_line: usize,
        file_path: &std::path::Path,
    ) -> Result<Option<URLDefinition>> {
        let mut format_lines = Vec::new();
        let mut i = start_line;

        // 收集format!宏的所有行，直到找到结束的')'
        while i < lines.len() {
            let line = lines[i].trim();
            format_lines.push(line);

            if line.contains(')') {
                break;
            }
            i += 1;
        }

        if i >= lines.len() {
            return Ok(None); // 没有找到结束
        }

        // 将收集的行合并为完整字符串
        let full_format = format_lines.join(" ");

        // 尝试提取URL
        if let Some(url_match) = self.extract_url_from_format_string(&full_format)? {
            // 提取变量
            let variables = self.extract_variables_from_format(&full_format);

            let url_def = URLDefinition {
                url: url_match,
                method_detection: HTTPMethodDetection {
                    method: HTTPMethod::default(),
                    confidence: 0.3,
                    source: MethodDetectionSource::Default,
                    evidence: "No explicit method found, using default".to_string(),
                    line_number: start_line + 1,
                },
                line_start: start_line + 1,
                line_end: i + 1,
                raw_format: full_format,
                variables,
                extraction_type: URLExtractionType::MultiLine,
                file_path: file_path.to_path_buf(),
            };

            trace!("提取多行format! URL: {}", url_def.url);
            return Ok(Some(url_def));
        }

        Ok(None)
    }

    /// 提取单行format!中的URL
    fn extract_single_line_format(
        &self,
        line: &str,
        line_num: usize,
        file_path: &std::path::Path,
    ) -> Result<Vec<URLDefinition>> {
        let mut results = Vec::new();

        // 首先匹配 {}/open-apis/ 格式（最常见的情况）
        if let Some(cap) = self.base_url_placeholder_pattern.captures(line) {
            if let Some(url_match) = cap.get(1) {
                let url = url_match.as_str().to_string();

                // 添加前导斜杠以确保格式一致
                let normalized_url = if url.starts_with('/') {
                    url
                } else {
                    format!("/{}", url)
                };

                let url_def = URLDefinition {
                    url: normalized_url,
                    method_detection: HTTPMethodDetection {
                        method: HTTPMethod::default(),
                        confidence: 0.4,
                        source: MethodDetectionSource::Default,
                        evidence: "Base URL placeholder format".to_string(),
                        line_number: line_num + 1,
                    },
                    line_start: line_num + 1,
                    line_end: line_num + 1,
                    raw_format: line.to_string(),
                    variables: Vec::new(), // 将在后续处理中提取
                    extraction_type: URLExtractionType::SingleLine,
                    file_path: file_path.to_path_buf(),
                };

                trace!("提取base_url占位符格式URL: {}", url_def.url);
                results.push(url_def);
            }
        }

        // 匹配包含open-apis的format!模式（原有的模式）
        let captures: Vec<_> = self.single_line_pattern.captures_iter(line).collect();

        for cap in captures {
            if let Some(url_match) = cap.get(1) {
                let url = url_match.as_str().to_string();

                let url_def = URLDefinition {
                    url,
                    method_detection: HTTPMethodDetection {
                        method: HTTPMethod::default(),
                        confidence: 0.3,
                        source: MethodDetectionSource::Default,
                        evidence: "Single line format without explicit method".to_string(),
                        line_number: line_num + 1,
                    },
                    line_start: line_num + 1,
                    line_end: line_num + 1,
                    raw_format: line.to_string(),
                    variables: Vec::new(), // 单行格式暂时不提取变量
                    extraction_type: URLExtractionType::SingleLine,
                    file_path: file_path.to_path_buf(),
                };

                trace!("提取单行format! URL: {}", url_def.url);
                results.push(url_def);
            }
        }

        Ok(results)
    }

    /// 提取ApiRequest模式中的URL
    fn extract_api_request_format(
        &self,
        line: &str,
        line_num: usize,
        file_path: &std::path::Path,
    ) -> Result<Option<URLDefinition>> {
        if let Some(cap) = self.api_request_pattern.captures(line) {
            if let Some(url_match) = cap.get(1) {
                let url = url_match.as_str().to_string();

                let url_def = URLDefinition {
                    url,
                    method_detection: HTTPMethodDetection {
                        method: HTTPMethod::default(),
                        confidence: 0.6, // ApiRequest模式置信度稍高
                        source: MethodDetectionSource::Default,
                        evidence: "ApiRequest pattern detected".to_string(),
                        line_number: line_num + 1,
                    },
                    line_start: line_num + 1,
                    line_end: line_num + 1,
                    raw_format: line.to_string(),
                    variables: Vec::new(),
                    extraction_type: URLExtractionType::ApiRequest,
                    file_path: file_path.to_path_buf(),
                };

                trace!("提取ApiRequest URL: {}", url_def.url);
                return Ok(Some(url_def));
            }
        }

        Ok(None)
    }

    /// 从format字符串中提取URL
    fn extract_url_from_format_string(&self, format_str: &str) -> Result<Option<String>> {
        // 首先匹配base_url占位符格式: {}/open-apis/...
        if let Some(cap) = self.base_url_placeholder_pattern.captures(format_str) {
            if let Some(url_match) = cap.get(1) {
                let url = url_match.as_str().to_string();
                // 添加前导斜杠以确保格式一致
                let normalized_url = if url.starts_with('/') {
                    url
                } else {
                    format!("/{}", url)
                };
                return Ok(Some(normalized_url));
            }
        }

        // 匹配format!中的URL部分
        if let Some(cap) = self.url_first_line_pattern.captures(format_str) {
            if let Some(url_match) = cap.get(1) {
                return Ok(Some(url_match.as_str().to_string()));
            }
        }

        // 直接匹配包含open-apis的字符串
        let url_pattern = Regex::new(r#""([^"]*(/open-apis/[^"]+)")"#)
            .context("Failed to compile URL extraction pattern")?;

        if let Some(cap) = url_pattern.captures(format_str) {
            if let Some(url_match) = cap.get(1) {
                return Ok(Some(url_match.as_str().to_string()));
            }
        }

        Ok(None)
    }

    /// 从format!中提取使用的变量
    fn extract_variables_from_format(&self, format_str: &str) -> Vec<String> {
        let mut variables = Vec::new();

        for pattern in &self.variable_patterns {
            for cap in pattern.captures_iter(format_str) {
                // 提取匹配的组
                for group in cap.iter().skip(1).flatten() {
                    let var_name = group.as_str().to_string();

                    // 过滤常见的关键词
                    if !["self", "config", "base_url", "request"].contains(&var_name.as_str()) {
                        variables.push(var_name);
                    }
                }
            }
        }

        // 去重
        variables.sort();
        variables.dedup();

        trace!("从format中提取变量: {:?}", variables);
        variables
    }

    /// 获取所有参数占位符
    pub fn extract_placeholders(&self, url: &str) -> Vec<String> {
        self.placeholder_pattern
            .captures_iter(url)
            .map(|cap| cap.get(1).unwrap().as_str().to_string())
            .collect()
    }

    /// 检查字符串是否包含format!宏
    pub fn contains_format_macro(&self, content: &str) -> bool {
        content.contains("format!") && content.contains("/open-apis/")
    }
}

impl Default for FormatParser {
    fn default() -> Self {
        Self::new().expect("Failed to create FormatParser")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_extract_single_line_format() {
        let parser = FormatParser::new().unwrap();
        let line = r#"let url = format!("{}/open-apis/sheets/v3/spreadsheets", self.config.base_url);"#;

        let results = parser.extract_single_line_format(line, 1, &PathBuf::from("test.rs")).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].url, "{}/open-apis/sheets/v3/spreadsheets");
        assert_eq!(results[0].line_start, 2);
        assert_eq!(results[0].line_end, 2);
    }

    #[test]
    fn test_extract_variables() {
        let parser = FormatParser::new().unwrap();
        let format_str = r#"format!("{}/open-apis/sheets/v2/spreadsheets/{}/merge_cells", &request.spreadsheet_token)"#;

        let variables = parser.extract_variables_from_format(format_str);
        assert!(variables.contains(&"spreadsheet_token".to_string()));
    }

    #[test]
    fn test_extract_placeholders() {
        let parser = FormatParser::new().unwrap();
        let url = "/open-apis/sheets/v2/spreadsheets/{}/merge_cells";

        let placeholders = parser.extract_placeholders(url);
        assert_eq!(placeholders.len(), 1);
        assert_eq!(placeholders[0], ""); // 空的占位符
    }
}