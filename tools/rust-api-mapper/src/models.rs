//! 核心数据结构定义
//!
//! 定义了API映射工具中使用的所有核心数据结构，
//! 包括URL定义、函数信息、API信息、匹配结果等。

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::collections::HashMap;

/// HTTP请求方法枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HTTPMethod {
    #[serde(rename = "GET")]
    Get,
    #[serde(rename = "POST")]
    Post,
    #[serde(rename = "PUT")]
    Put,
    #[serde(rename = "PATCH")]
    Patch,
    #[serde(rename = "DELETE")]
    Delete,
    #[serde(rename = "HEAD")]
    Head,
    #[serde(rename = "OPTIONS")]
    Options,
}

impl HTTPMethod {
    /// 从字符串解析HTTP方法
    pub fn from_str(method: &str) -> Option<Self> {
        match method.to_uppercase().as_str() {
            "GET" => Some(HTTPMethod::Get),
            "POST" => Some(HTTPMethod::Post),
            "PUT" => Some(HTTPMethod::Put),
            "PATCH" => Some(HTTPMethod::Patch),
            "DELETE" => Some(HTTPMethod::Delete),
            "HEAD" => Some(HTTPMethod::Head),
            "OPTIONS" => Some(HTTPMethod::Options),
            _ => None,
        }
    }

    /// 转换为字符串
    pub fn as_str(&self) -> &'static str {
        match self {
            HTTPMethod::Get => "GET",
            HTTPMethod::Post => "POST",
            HTTPMethod::Put => "PUT",
            HTTPMethod::Patch => "PATCH",
            HTTPMethod::Delete => "DELETE",
            HTTPMethod::Head => "HEAD",
            HTTPMethod::Options => "OPTIONS",
        }
    }
}

impl Default for HTTPMethod {
    fn default() -> Self {
        HTTPMethod::Post // 大多数飞书API都是POST
    }
}

/// 函数类型枚举
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FunctionType {
    /// 公共异步函数
    #[serde(rename = "pub_async")]
    PubAsync,
    /// 公共同步函数
    #[serde(rename = "pub_sync")]
    PubSync,
    /// 私有异步函数
    #[serde(rename = "async")]
    Async,
    /// 私有同步函数
    #[serde(rename = "sync")]
    Sync,
}

/// URL提取类型
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum URLExtractionType {
    /// 单行format!
    #[serde(rename = "single_line")]
    SingleLine,
    /// 多行format!
    #[serde(rename = "multiline")]
    MultiLine,
    /// ApiRequest模式
    #[serde(rename = "api_request")]
    ApiRequest,
}

/// HTTP方法检测结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HTTPMethodDetection {
    /// 检测到的HTTP方法
    pub method: HTTPMethod,
    /// 置信度 (0.0 - 1.0)
    pub confidence: f32,
    /// 检测来源
    pub source: MethodDetectionSource,
    /// 检测证据（匹配到的文本）
    pub evidence: String,
    /// 检测到的行号
    pub line_number: usize,
}

/// HTTP方法检测来源
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MethodDetectionSource {
    /// 链式调用（如.post()）
    #[serde(rename = "chain_call")]
    ChainCall,
    /// 枚举值（如Method::POST）
    #[serde(rename = "enum_value")]
    EnumValue,
    /// 变量赋值
    #[serde(rename = "variable_assignment")]
    VariableAssignment,
    /// 字符串字面量
    #[serde(rename = "string_literal")]
    StringLiteral,
    /// 函数名推断
    #[serde(rename = "function_name")]
    FunctionName,
    /// 默认值
    #[serde(rename = "default")]
    Default,
}

/// 从代码中提取的URL定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct URLDefinition {
    /// 标准化后的URL路径
    pub url: String,
    /// HTTP方法信息
    pub method_detection: HTTPMethodDetection,
    /// URL定义的开始行号
    pub line_start: usize,
    /// URL定义的结束行号
    pub line_end: usize,
    /// 原始的format!宏内容
    pub raw_format: String,
    /// 使用的变量名列表
    pub variables: Vec<String>,
    /// URL提取类型
    pub extraction_type: URLExtractionType,
    /// 文件路径
    pub file_path: PathBuf,
}

/// 函数信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionInfo {
    /// 函数名
    pub name: String,
    /// 函数定义的行号
    pub line_number: usize,
    /// 完整的函数签名
    pub signature: String,
    /// 函数类型
    pub function_type: FunctionType,
    /// 搜索距离（从URL行到函数定义的行数）
    pub search_distance: usize,
}

/// API列表中的API信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct APIInfo {
    /// API名称
    pub name: String,
    /// HTTP方法
    pub method: HTTPMethod,
    /// API路径
    pub path: String,
    /// API描述
    pub description: String,
    /// 是否自建
    pub self_build: String,
    /// 店铺应用
    pub store_app: String,
    /// 文档链接
    pub doc_link: String,
    /// 所属服务
    pub service: String,
    /// API版本
    pub version: String,
}

/// API匹配结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct APIMatch {
    /// API信息
    pub api_info: APIInfo,
    /// 对应的实现（如果有）
    pub implementation: Option<URLDefinition>,
    /// 函数信息（如果找到）
    pub function_info: Option<FunctionInfo>,
    /// 匹配状态
    pub status: MatchStatus,
    /// 匹配置信度
    pub match_confidence: f32,
    /// 实现预览文本
    pub implementation_preview: String,
}

/// 匹配状态
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MatchStatus {
    /// 找到实现
    #[serde(rename = "found")]
    Found,
    /// 未找到实现
    #[serde(rename = "not_found")]
    NotFound,
    /// 部分匹配
    #[serde(rename = "partial")]
    Partial,
}

/// 服务统计信息
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ServiceStats {
    /// 总API数
    pub total: usize,
    /// 已实现API数
    pub found: usize,
    /// 实现率百分比
    pub implementation_rate: f32,
}

/// 汇总统计信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummaryStats {
    /// 总API数
    pub total_apis: usize,
    /// 已实现API数
    pub found_apis: usize,
    /// 实现率百分比
    pub implementation_rate: f32,
    /// 生成时间
    pub generation_time: String,
    /// 匹配方法
    pub method: String,
    /// 发现的URL定义数量
    pub url_definitions_found: usize,
    /// 按服务分组的统计
    pub service_stats: HashMap<String, ServiceStats>,
}

/// 最终的映射报告
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MappingReport {
    /// 汇总统计
    pub summary: SummaryStats,
    /// URL到函数的映射关系
    pub url_function_map: HashMap<String, FunctionInfo>,
    /// 所有匹配结果
    pub apis: Vec<APIMatch>,
}

impl MappingReport {
    /// 创建新的映射报告
    pub fn new() -> Self {
        Self {
            summary: SummaryStats {
                total_apis: 0,
                found_apis: 0,
                implementation_rate: 0.0,
                generation_time: chrono::Utc::now().to_rfc3339(),
                method: "rust_based_exact_matching".to_string(),
                url_definitions_found: 0,
                service_stats: HashMap::new(),
            },
            url_function_map: HashMap::new(),
            apis: Vec::new(),
        }
    }

    /// 计算汇总统计信息
    pub fn calculate_summary(&mut self) {
        self.summary.total_apis = self.apis.len();
        self.summary.found_apis = self.apis.iter()
            .filter(|api| api.status == MatchStatus::Found)
            .count();

        if self.summary.total_apis > 0 {
            self.summary.implementation_rate =
                (self.summary.found_apis as f64 / self.summary.total_apis as f64 * 100.0) as f32;
        }

        // 计算各服务统计
        let mut service_stats = HashMap::new();
        for api in &self.apis {
            let stats = service_stats.entry(api.api_info.service.clone())
                .or_insert_with(ServiceStats::default);

            stats.total += 1;
            if api.status == MatchStatus::Found {
                stats.found += 1;
            }
        }

        // 计算实现率
        for stats in service_stats.values_mut() {
            if stats.total > 0 {
                stats.implementation_rate = (stats.found as f32 / stats.total as f32) * 100.0;
            }
        }

        self.summary.service_stats = service_stats;
    }
}