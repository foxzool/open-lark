//! HTTP方法检测器
//!
//! 从Rust代码上下文中推断HTTP请求方法，支持多种检测模式：
//! - 链式调用模式（如.post()）
//! - 枚举值模式（如Method::POST）
//! - 变量赋值模式
//! - 字符串字面量模式
//! - 函数名推断模式

use regex::Regex;
use anyhow::{Result, Context};
use tracing::{debug, trace};

use crate::models::{HTTPMethodDetection, HTTPMethod, MethodDetectionSource};

/// HTTP方法检测器
pub struct HTTPMethodDetector {
    /// 链式调用模式映射
    chain_call_patterns: Vec<(Regex, HTTPMethod)>,
    /// 枚举值模式映射
    enum_value_patterns: Vec<(Regex, HTTPMethod)>,
    /// 变量赋值模式映射
    variable_assignment_patterns: Vec<(Regex, HTTPMethod)>,
    /// 字符串字面量模式映射
    string_literal_patterns: Vec<(Regex, HTTPMethod)>,
    /// 默认HTTP方法（大多数飞书API是POST）
    default_method: HTTPMethod,
}

impl HTTPMethodDetector {
    /// 创建新的HTTP方法检测器
    pub fn new() -> Result<Self> {
        let mut chain_call_patterns = Vec::new();
        let mut enum_value_patterns = Vec::new();
        let mut variable_assignment_patterns = Vec::new();
        let mut string_literal_patterns = Vec::new();

        // 链式调用模式
        let chain_patterns = [
            (r"\.post\s*\(", HTTPMethod::Post),
            (r"\.get\s*\(", HTTPMethod::Get),
            (r"\.put\s*\(", HTTPMethod::Put),
            (r"\.patch\s*\(", HTTPMethod::Patch),
            (r"\.delete\s*\(", HTTPMethod::Delete),
            (r"\.head\s*\(", HTTPMethod::Head),
            (r"\.options\s*\(", HTTPMethod::Options),
        ];

        for (pattern, method) in chain_patterns {
            chain_call_patterns.push((
                Regex::new(pattern)
                    .with_context(|| format!("Failed to compile chain pattern: {}", pattern))?,
                method,
            ));
        }

        // 枚举值模式
        let enum_patterns = [
            (r"Method::POST", HTTPMethod::Post),
            (r"Method::GET", HTTPMethod::Get),
            (r"Method::PUT", HTTPMethod::Put),
            (r"Method::PATCH", HTTPMethod::Patch),
            (r"Method::DELETE", HTTPMethod::Delete),
            (r"Method::HEAD", HTTPMethod::Head),
            (r"Method::OPTIONS", HTTPMethod::Options),
        ];

        for (pattern, method) in enum_patterns {
            enum_value_patterns.push((
                Regex::new(pattern)
                    .with_context(|| format!("Failed to compile enum pattern: {}", pattern))?,
                method,
            ));
        }

        // 变量赋值模式
        let assignment_patterns = [
            (r"let\s+method\s*=\s*Method::POST", HTTPMethod::Post),
            (r"let\s+method\s*=\s*Method::GET", HTTPMethod::Get),
            (r"let\s+method\s*=\s*Method::PUT", HTTPMethod::Put),
            (r"let\s+method\s*=\s*Method::PATCH", HTTPMethod::Patch),
            (r"let\s+method\s*=\s*Method::DELETE", HTTPMethod::Delete),
            (r"let\s+method\s*=\s*Method::HEAD", HTTPMethod::Head),
            (r"let\s+method\s*=\s*Method::OPTIONS", HTTPMethod::Options),
            (r"const\s+METHOD\s*=\s*Method::POST", HTTPMethod::Post),
            (r"const\s+METHOD\s*=\s*Method::GET", HTTPMethod::Get),
            (r"METHOD\s*=\s*Method::POST", HTTPMethod::Post),
            (r"METHOD\s*=\s*Method::GET", HTTPMethod::Get),
        ];

        for (pattern, method) in assignment_patterns {
            variable_assignment_patterns.push((
                Regex::new(pattern)
                    .with_context(|| format!("Failed to compile assignment pattern: {}", pattern))?,
                method,
            ));
        }

        // 字符串字面量模式
        let literal_patterns = [
            (r#""POST""#, HTTPMethod::Post),
            (r#""GET""#, HTTPMethod::Get),
            (r#""PUT""#, HTTPMethod::Put),
            (r#""PATCH""#, HTTPMethod::Patch),
            (r#""DELETE""#, HTTPMethod::Delete),
            (r#""HEAD""#, HTTPMethod::Head),
            (r#""OPTIONS""#, HTTPMethod::Options),
            (r"'POST'", HTTPMethod::Post),
            (r"'GET'", HTTPMethod::Get),
            (r"'PUT'", HTTPMethod::Put),
            (r"'PATCH'", HTTPMethod::Patch),
            (r"'DELETE'", HTTPMethod::Delete),
            (r"'HEAD'", HTTPMethod::Head),
            (r"'OPTIONS'", HTTPMethod::Options),
        ];

        for (pattern, method) in literal_patterns {
            string_literal_patterns.push((
                Regex::new(pattern)
                    .with_context(|| format!("Failed to compile literal pattern: {}", pattern))?,
                method,
            ));
        }

        Ok(Self {
            chain_call_patterns,
            enum_value_patterns,
            variable_assignment_patterns,
            string_literal_patterns,
            default_method: HTTPMethod::Post,
        })
    }

    /// 从URL定义的上下文中检测HTTP方法
    pub fn detect_method_from_context(
        &self,
        lines: &[&str],
        url_line_start: usize,
        url_line_end: usize,
    ) -> HTTPMethodDetection {
        // 扩展搜索范围
        let search_start = if url_line_start >= 10 { url_line_start - 10 } else { 0 };
        let search_end = std::cmp::min(lines.len(), url_line_end + 10);

        // 收集所有可能的HTTP方法
        let mut method_candidates = Vec::new();

        for i in search_start..search_end {
            let line = lines[i].trim();

            if let Some(method_info) = self.scan_line_for_method(line, i) {
                method_candidates.push(method_info);
            }
        }

        // 如果找到候选方法，选择置信度最高的
        if let Some(best_candidate) = method_candidates
            .iter()
            .max_by(|a, b| a.confidence.partial_cmp(&b.confidence).unwrap())
        {
            debug!("检测到HTTP方法: {} (置信度: {:.2}, 来源: {:?})",
                   best_candidate.method.as_str(),
                   best_candidate.confidence,
                   best_candidate.source);
            best_candidate.clone()
        } else {
            // 如果没有找到，返回默认方法
            debug!("未检测到明确的HTTP方法，使用默认值: {}", self.default_method.as_str());
            HTTPMethodDetection {
                method: self.default_method,
                confidence: 0.3,
                source: MethodDetectionSource::Default,
                evidence: "No explicit method found, using default".to_string(),
                line_number: url_line_start + 1,
            }
        }
    }

    /// 从函数名推断HTTP方法（启发式方法）
    pub fn detect_method_from_function_name(&self, function_name: &str) -> HTTPMethodDetection {
        let function_lower = function_name.to_lowercase();

        let (method, confidence) = if self.contains_get_keywords(&function_lower) {
            (HTTPMethod::Get, 0.7)
        } else if self.contains_post_keywords(&function_lower) {
            (HTTPMethod::Post, 0.7)
        } else if self.contains_put_keywords(&function_lower) {
            (HTTPMethod::Put, 0.7)
        } else if self.contains_patch_keywords(&function_lower) {
            (HTTPMethod::Patch, 0.7)
        } else if self.contains_delete_keywords(&function_lower) {
            (HTTPMethod::Delete, 0.7)
        } else {
            (self.default_method, 0.4)
        };

        HTTPMethodDetection {
            method,
            confidence,
            source: MethodDetectionSource::FunctionName,
            evidence: format!("Inferred from function name: {}", function_name),
            line_number: 0,
        }
    }

    /// 扫描单行寻找HTTP方法
    fn scan_line_for_method(&self, line: &str, line_num: usize) -> Option<HTTPMethodDetection> {
        // 检查链式调用模式
        for (regex, method) in &self.chain_call_patterns {
            if let Some(cap) = regex.find(line) {
                let confidence = self.calculate_chain_call_confidence(line);
                return Some(HTTPMethodDetection {
                    method: *method,
                    confidence,
                    source: MethodDetectionSource::ChainCall,
                    evidence: cap.as_str().to_string(),
                    line_number: line_num + 1,
                });
            }
        }

        // 检查枚举值模式
        for (regex, method) in &self.enum_value_patterns {
            if let Some(cap) = regex.find(line) {
                return Some(HTTPMethodDetection {
                    method: *method,
                    confidence: 0.95, // 枚举值非常明确
                    source: MethodDetectionSource::EnumValue,
                    evidence: cap.as_str().to_string(),
                    line_number: line_num + 1,
                });
            }
        }

        // 检查变量赋值模式
        for (regex, method) in &self.variable_assignment_patterns {
            if let Some(cap) = regex.find(line) {
                let confidence = self.calculate_assignment_confidence(line);
                return Some(HTTPMethodDetection {
                    method: *method,
                    confidence,
                    source: MethodDetectionSource::VariableAssignment,
                    evidence: cap.as_str().to_string(),
                    line_number: line_num + 1,
                });
            }
        }

        // 检查字符串字面量模式
        for (regex, method) in &self.string_literal_patterns {
            if let Some(cap) = regex.find(line) {
                let confidence = self.calculate_string_literal_confidence(line);
                return Some(HTTPMethodDetection {
                    method: *method,
                    confidence,
                    source: MethodDetectionSource::StringLiteral,
                    evidence: cap.as_str().to_string(),
                    line_number: line_num + 1,
                });
            }
        }

        None
    }

    /// 计算链式调用的置信度
    fn calculate_chain_call_confidence(&self, line: &str) -> f32 {
        let mut confidence: f32 = 0.9; // 基础置信度

        let line_lower = line.to_lowercase();

        // 如果行中包含URL相关关键词，提高置信度
        if line_lower.contains("url") || line_lower.contains("request") || line_lower.contains("http") {
            confidence += 0.05;
        }

        // 如果是明确的传输层调用，提高置信度
        if line_lower.contains("transport") || line_lower.contains("client") {
            confidence += 0.05;
        }

        confidence.min(1.0)
    }

    /// 计算变量赋值的置信度
    fn calculate_assignment_confidence(&self, line: &str) -> f32 {
        let mut confidence: f32 = 0.8; // 基础置信度

        let line_lower = line.to_lowercase();

        // 如果是方法变量赋值，提高置信度
        if line_lower.contains("method") {
            confidence += 0.1;
        }

        // 如果是常量定义，提高置信度
        if line_lower.contains("const") || line_lower.contains("static") {
            confidence += 0.1;
        }

        confidence.min(1.0)
    }

    /// 计算字符串字面量的置信度
    fn calculate_string_literal_confidence(&self, line: &str) -> f32 {
        let mut confidence: f32 = 0.6; // 基础置信度

        let line_lower = line.to_lowercase();

        // 如果行中包含方法相关的上下文
        if line_lower.contains("method") || line_lower.contains("http") || line_lower.contains("request") {
            confidence += 0.1;
        }

        // 如果是变量赋值的值部分
        if line_lower.contains("=") {
            confidence += 0.1;
        }

        confidence.min(1.0)
    }

    /// 检查是否包含GET方法的关键词
    fn contains_get_keywords(&self, function_name: &str) -> bool {
        let keywords = ["get", "fetch", "retrieve", "list", "query", "find", "read", "load"];
        keywords.iter().any(|keyword| function_name.contains(keyword))
    }

    /// 检查是否包含POST方法的关键词
    fn contains_post_keywords(&self, function_name: &str) -> bool {
        let keywords = ["create", "add", "insert", "post", "send", "submit", "append", "write"];
        keywords.iter().any(|keyword| function_name.contains(keyword))
    }

    /// 检查是否包含PUT方法的关键词
    fn contains_put_keywords(&self, function_name: &str) -> bool {
        let keywords = ["update", "replace", "put", "modify", "change", "edit"];
        keywords.iter().any(|keyword| function_name.contains(keyword))
    }

    /// 检查是否包含PATCH方法的关键词
    fn contains_patch_keywords(&self, function_name: &str) -> bool {
        let keywords = ["patch", "partial_update", "modify_partially"];
        keywords.iter().any(|keyword| function_name.contains(keyword))
    }

    /// 检查是否包含DELETE方法的关键词
    fn contains_delete_keywords(&self, function_name: &str) -> bool {
        let keywords = ["delete", "remove", "destroy", "clear"];
        keywords.iter().any(|keyword| function_name.contains(keyword))
    }
}

impl Default for HTTPMethodDetector {
    fn default() -> Self {
        Self::new().expect("Failed to create HTTPMethodDetector")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chain_call_detection() {
        let detector = HTTPMethodDetector::new().unwrap();
        let line = r#"let response = self.config.transport.post(&url).json(&body).send()?;"#;

        let result = detector.scan_line_for_method(line, 1);
        assert!(result.is_some());
        assert_eq!(result.unwrap().method, HTTPMethod::Post);
    }

    #[test]
    fn test_enum_value_detection() {
        let detector = HTTPMethodDetector::new().unwrap();
        let line = "Method::POST,";

        let result = detector.scan_line_for_method(line, 1);
        assert!(result.is_some());
        assert_eq!(result.unwrap().method, HTTPMethod::Post);
        assert_eq!(result.unwrap().confidence, 0.95);
    }

    #[test]
    fn test_function_name_inference() {
        let detector = HTTPMethodDetector::new().unwrap();

        let result = detector.detect_method_from_function_name("get_user_info");
        assert_eq!(result.method, HTTPMethod::Get);
        assert_eq!(result.source, MethodDetectionSource::FunctionName);

        let result = detector.detect_method_from_function_name("create_spreadsheet");
        assert_eq!(result.method, HTTPMethod::Post);
    }

    #[test]
    fn test_context_detection() {
        let detector = HTTPMethodDetector::new().unwrap();
        let lines = vec![
            "let response = self.config.transport.post(&url).json(&body).send()?;",
            "let url = format!(\"{}/open-apis/sheets/v3/spreadsheets\", self.config.base_url);",
        ];

        let result = detector.detect_method_from_context(&lines, 1, 1);
        assert_eq!(result.method, HTTPMethod::Post);
        assert_eq!(result.source, MethodDetectionSource::ChainCall);
    }
}