#!/usr/bin/env cargo run --bin

//! # API设计一致性检查工具
//!
//! 这个工具分析open-lark项目中的API设计一致性，检查：
//! 1. Builder模式的实现一致性
//! 2. 错误处理模式的统一性
//! 3. 命名约定的遵循情况
//! 4. API文档的完整性
//!
//! ## 使用方法
//! ```bash
//! cargo run --bin api_consistency_checker
//! ```

use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiAnalysis {
    pub file_path: String,
    pub service_name: String,
    pub version: String,
    pub methods: Vec<ApiMethod>,
    pub consistency_score: u8,
    pub issues: Vec<ConsistencyIssue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiMethod {
    pub name: String,
    pub has_builder: bool,
    pub uses_standard_response: bool,
    pub has_documentation: bool,
    pub error_handling_pattern: ErrorHandlingPattern,
    pub async_pattern: AsyncPattern,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ErrorHandlingPattern {
    StandardResponse, // 使用 StandardResponse trait
    DirectUnwrap,     // 直接 unwrap_or_default
    ManualMatch,      // 手动匹配 Result
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AsyncPattern {
    Async,
    Sync,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsistencyIssue {
    pub issue_type: IssueType,
    pub severity: Severity,
    pub description: String,
    pub suggestion: String,
    pub line_number: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IssueType {
    MissingBuilder,
    InconsistentErrorHandling,
    MissingDocumentation,
    NamingConvention,
    AsyncInconsistency,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Severity {
    High,
    Medium,
    Low,
}

pub struct ApiConsistencyChecker {
    service_dir: PathBuf,
    analyses: Vec<ApiAnalysis>,
    _builder_pattern: Regex,
    standard_response_pattern: Regex,
    _doc_comment_pattern: Regex,
    error_unwrap_pattern: Regex,
}

impl ApiConsistencyChecker {
    pub fn new<P: AsRef<Path>>(service_dir: P) -> Self {
        let builder_pattern = Regex::new(r"struct\s+(\w+)Builder\s*\{").unwrap();
        let standard_response_pattern = Regex::new(r"\.into_result\(\)").unwrap();
        let doc_comment_pattern = Regex::new(r"///\s+").unwrap();
        let error_unwrap_pattern = Regex::new(r"\.unwrap_or_default\(\)").unwrap();

        Self {
            service_dir: service_dir.as_ref().to_path_buf(),
            analyses: Vec::new(),
            _builder_pattern: builder_pattern,
            standard_response_pattern,
            _doc_comment_pattern: doc_comment_pattern,
            error_unwrap_pattern,
        }
    }

    pub fn analyze_all_services(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🔍 开始分析API设计一致性...");

        for entry in WalkDir::new(&self.service_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().is_some_and(|ext| ext == "rs"))
        {
            if let Some(analysis) = self.analyze_service_file(entry.path())? {
                self.analyses.push(analysis);
            }
        }

        println!("✅ 分析完成，共检查了 {} 个服务文件", self.analyses.len());
        Ok(())
    }

    fn analyze_service_file(
        &self,
        file_path: &Path,
    ) -> Result<Option<ApiAnalysis>, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(file_path)?;

        // 跳过非服务文件
        if !content.contains("impl") || !content.contains("Service") {
            return Ok(None);
        }

        let file_path_str = file_path.to_string_lossy().to_string();
        let (service_name, version) = self.extract_service_info(&file_path_str);

        let methods = self.analyze_methods(&content);
        let issues = self.check_consistency_issues(&content, &methods);
        let consistency_score = self.calculate_consistency_score(&methods, &issues);

        Ok(Some(ApiAnalysis {
            file_path: file_path_str,
            service_name,
            version,
            methods,
            consistency_score,
            issues,
        }))
    }

    fn extract_service_info(&self, file_path: &str) -> (String, String) {
        let path_parts: Vec<&str> = file_path.split('/').collect();

        // 从路径中提取服务名和版本
        // 例如: src/service/contact/v3/user.rs -> (contact, v3)
        let mut service_name = "unknown".to_string();
        let mut version = "unknown".to_string();

        for (i, part) in path_parts.iter().enumerate() {
            if *part == "service" && i + 1 < path_parts.len() {
                service_name = path_parts[i + 1].to_string();
                if i + 2 < path_parts.len() && path_parts[i + 2].starts_with('v') {
                    version = path_parts[i + 2].to_string();
                }
                break;
            }
        }

        (service_name, version)
    }

    fn analyze_methods(&self, content: &str) -> Vec<ApiMethod> {
        let mut methods = Vec::new();
        let lines: Vec<&str> = content.lines().collect();

        // 查找 pub async fn 方法
        for (i, line) in lines.iter().enumerate() {
            if let Some(method_name) = self.extract_method_name(line) {
                let has_builder = self.check_has_builder(content, &method_name);
                let uses_standard_response =
                    self.check_uses_standard_response(content, &method_name);
                let has_documentation = self.check_has_documentation(&lines, i);
                let error_handling_pattern =
                    self.determine_error_handling_pattern(content, &method_name);
                let async_pattern = if line.contains("async") {
                    AsyncPattern::Async
                } else {
                    AsyncPattern::Sync
                };

                methods.push(ApiMethod {
                    name: method_name,
                    has_builder,
                    uses_standard_response,
                    has_documentation,
                    error_handling_pattern,
                    async_pattern,
                });
            }
        }

        methods
    }

    fn extract_method_name(&self, line: &str) -> Option<String> {
        // 匹配 pub async fn method_name 或 pub fn method_name
        let re = Regex::new(r"pub\s+(?:async\s+)?fn\s+(\w+)").unwrap();
        re.captures(line.trim())
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str().to_string())
    }

    fn check_has_builder(&self, content: &str, method_name: &str) -> bool {
        // 检查是否有对应的Builder结构体
        let builder_name = format!("{}Builder", method_name.to_case_camel_case());
        content.contains(&builder_name)
    }

    fn check_uses_standard_response(&self, content: &str, _method_name: &str) -> bool {
        self.standard_response_pattern.is_match(content)
    }

    fn check_has_documentation(&self, lines: &[&str], method_index: usize) -> bool {
        // 检查方法前是否有文档注释
        if method_index == 0 {
            return false;
        }

        for i in (0..method_index).rev() {
            let line = lines[i].trim();
            if line.starts_with("///") {
                return true;
            }
            if line.starts_with("pub") || line.starts_with("impl") {
                break;
            }
        }
        false
    }

    fn determine_error_handling_pattern(
        &self,
        content: &str,
        _method_name: &str,
    ) -> ErrorHandlingPattern {
        if self.standard_response_pattern.is_match(content) {
            ErrorHandlingPattern::StandardResponse
        } else if self.error_unwrap_pattern.is_match(content) {
            ErrorHandlingPattern::DirectUnwrap
        } else if content.contains("match") && content.contains("Result") {
            ErrorHandlingPattern::ManualMatch
        } else {
            ErrorHandlingPattern::Unknown
        }
    }

    fn check_consistency_issues(
        &self,
        _content: &str,
        methods: &[ApiMethod],
    ) -> Vec<ConsistencyIssue> {
        let mut issues = Vec::new();

        for method in methods {
            // 检查Builder模式一致性
            if !method.has_builder && method.name.starts_with("create") {
                issues.push(ConsistencyIssue {
                    issue_type: IssueType::MissingBuilder,
                    severity: Severity::Medium,
                    description: format!("方法 '{}' 缺少Builder模式实现", method.name),
                    suggestion: format!("建议为 '{}' 创建对应的Builder结构体", method.name),
                    line_number: None,
                });
            }

            // 检查错误处理一致性
            if !matches!(
                method.error_handling_pattern,
                ErrorHandlingPattern::StandardResponse
            ) {
                issues.push(ConsistencyIssue {
                    issue_type: IssueType::InconsistentErrorHandling,
                    severity: Severity::High,
                    description: format!(
                        "方法 '{}' 未使用统一的StandardResponse错误处理",
                        method.name
                    ),
                    suggestion: "建议使用 .into_result() 方法进行统一错误处理".to_string(),
                    line_number: None,
                });
            }

            // 检查文档完整性
            if !method.has_documentation {
                issues.push(ConsistencyIssue {
                    issue_type: IssueType::MissingDocumentation,
                    severity: Severity::Low,
                    description: format!("方法 '{}' 缺少文档注释", method.name),
                    suggestion: "建议添加 /// 文档注释描述方法功能和参数".to_string(),
                    line_number: None,
                });
            }
        }

        issues
    }

    fn calculate_consistency_score(
        &self,
        methods: &[ApiMethod],
        issues: &[ConsistencyIssue],
    ) -> u8 {
        if methods.is_empty() {
            return 100;
        }

        let total_checks = methods.len() * 4; // 4 checks per method
        let mut passed_checks = 0;

        for method in methods {
            if method.has_builder {
                passed_checks += 1;
            }
            if method.uses_standard_response {
                passed_checks += 1;
            }
            if method.has_documentation {
                passed_checks += 1;
            }
            if matches!(method.async_pattern, AsyncPattern::Async) {
                passed_checks += 1;
            }
        }

        // 扣除严重问题的分数
        let penalty = issues
            .iter()
            .map(|issue| match issue.severity {
                Severity::High => 10,
                Severity::Medium => 5,
                Severity::Low => 2,
            })
            .sum::<u32>() as usize;

        let score = ((passed_checks * 100) / total_checks).saturating_sub(penalty);
        (score as u8).min(100)
    }

    pub fn generate_report(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut report = String::new();

        report.push_str("# API设计一致性检查报告\n\n");
        report.push_str(&format!(
            "生成时间: {}\n\n",
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
        ));

        // 总体统计
        let total_services = self.analyses.len();
        let avg_score: u8 = if total_services > 0 {
            (self
                .analyses
                .iter()
                .map(|a| a.consistency_score as u32)
                .sum::<u32>()
                / total_services as u32) as u8
        } else {
            0
        };

        let total_issues: usize = self.analyses.iter().map(|a| a.issues.len()).sum();

        report.push_str("## 📊 总体统计\n\n");
        report.push_str(&format!("- 检查的服务文件数: {total_services}\n"));
        report.push_str(&format!("- 平均一致性得分: {avg_score}%\n"));
        report.push_str(&format!("- 发现的问题总数: {total_issues}\n\n"));

        // 按服务分类的详细报告
        let mut services_by_name: HashMap<String, Vec<&ApiAnalysis>> = HashMap::new();
        for analysis in &self.analyses {
            services_by_name
                .entry(analysis.service_name.clone())
                .or_default()
                .push(analysis);
        }

        report.push_str("## 🔍 服务详细分析\n\n");

        for (service_name, analyses) in services_by_name {
            report.push_str(&format!("### {service_name} 服务\n\n"));

            for analysis in analyses {
                report.push_str(&format!(
                    "#### {} - {}\n",
                    analysis.version, analysis.file_path
                ));
                report.push_str(&format!(
                    "**一致性得分**: {}%\n\n",
                    analysis.consistency_score
                ));

                if !analysis.methods.is_empty() {
                    report.push_str("**API方法分析**:\n");
                    for method in &analysis.methods {
                        report.push_str(&format!("- `{}`: ", method.name));
                        let mut features = Vec::new();
                        if method.has_builder {
                            features.push("✅ Builder");
                        } else {
                            features.push("❌ Builder");
                        }
                        if method.uses_standard_response {
                            features.push("✅ StandardResponse");
                        } else {
                            features.push("❌ StandardResponse");
                        }
                        if method.has_documentation {
                            features.push("✅ 文档");
                        } else {
                            features.push("❌ 文档");
                        }
                        report.push_str(&features.join(", "));
                        report.push('\n');
                    }
                    report.push('\n');
                }

                if !analysis.issues.is_empty() {
                    report.push_str("**发现的问题**:\n");
                    for issue in &analysis.issues {
                        let severity_icon = match issue.severity {
                            Severity::High => "🔴",
                            Severity::Medium => "🟡",
                            Severity::Low => "🟢",
                        };
                        report.push_str(&format!("{} {}\n", severity_icon, issue.description));
                        report.push_str(&format!("   💡 {}\n", issue.suggestion));
                    }
                    report.push('\n');
                }
            }
        }

        // 改进建议
        report.push_str("## 🚀 改进建议\n\n");
        report.push_str("### 高优先级\n");
        report.push_str("1. **统一错误处理**: 为所有API方法实现StandardResponse模式\n");
        report.push_str("2. **补充Builder模式**: 为create类型的方法添加Builder支持\n\n");

        report.push_str("### 中优先级\n");
        report.push_str("1. **完善文档**: 为所有公开API添加详细的文档注释\n");
        report.push_str("2. **命名规范**: 确保所有API遵循一致的命名约定\n\n");

        report.push_str("### 低优先级\n");
        report.push_str("1. **代码风格**: 统一代码格式和结构\n");
        report.push_str("2. **性能优化**: 识别和优化潜在的性能瓶颈\n\n");

        Ok(report)
    }

    pub fn save_report<P: AsRef<Path>>(
        &self,
        output_path: P,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let report = self.generate_report()?;
        fs::write(output_path, report)?;
        Ok(())
    }
}

// Helper trait to convert strings to CamelCase
trait ToCamelCase {
    fn to_case_camel_case(&self) -> String;
}

impl ToCamelCase for str {
    fn to_case_camel_case(&self) -> String {
        let mut result = String::new();
        let mut capitalize_next = true;

        for c in self.chars() {
            if c == '_' {
                capitalize_next = true;
            } else if capitalize_next {
                result.push(c.to_uppercase().next().unwrap_or(c));
                capitalize_next = false;
            } else {
                result.push(c);
            }
        }

        result
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔧 API设计一致性检查工具");
    println!("==========================\n");

    let service_dir = "src/service";
    let mut checker = ApiConsistencyChecker::new(service_dir);

    checker.analyze_all_services()?;

    let report_path = "reports/api_consistency_report.md";
    checker.save_report(report_path)?;

    println!("📄 报告已生成: {report_path}");
    println!("\n🎯 总结:");

    let total_files = checker.analyses.len();
    let avg_score: u8 = if total_files > 0 {
        (checker
            .analyses
            .iter()
            .map(|a| a.consistency_score as u32)
            .sum::<u32>()
            / total_files as u32) as u8
    } else {
        0
    };

    println!("   - 检查了 {total_files} 个服务文件");
    println!("   - 平均一致性得分: {avg_score}%");

    if avg_score >= 80 {
        println!("   ✅ API设计一致性良好!");
    } else if avg_score >= 60 {
        println!("   ⚠️  API设计有改进空间");
    } else {
        println!("   🔴 需要重点关注API设计一致性");
    }

    Ok(())
}
