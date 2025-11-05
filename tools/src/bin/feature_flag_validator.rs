//! 功能标志验证工具
//!
//! 用于验证API路径到功能标志的映射是否符合技术规范

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

/// API映射记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiMapping {
    pub name: String,
    pub method: String,
    pub path: String,
    pub description: String,
    pub self_build: String,
    pub store_app: String,
    pub doc_link: String,
}

/// 验证结果
#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationResult {
    pub total_apis: usize,
    pub valid_mappings: usize,
    pub invalid_mappings: usize,
    pub missing_features: usize,
    pub issues: Vec<ValidationIssue>,
}

/// 验证问题
#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationIssue {
    pub api_name: String,
    pub api_path: String,
    pub method: String,
    pub current_feature: String,
    pub expected_feature: String,
    pub issue_type: IssueType,
    pub description: String,
}

/// 问题类型
#[derive(Debug, Serialize, Deserialize)]
pub enum IssueType {
    FeatureNotImplemented,
    NamingMismatch,
    SpecialMappingRequired,
    MissingFeature,
}

/// 功能标志验证器
pub struct FeatureFlagValidator {
    api_mappings: Vec<ApiMapping>,
    special_mappings: HashMap<String, String>,
}

impl Default for FeatureFlagValidator {
    fn default() -> Self {
        Self::new()
    }
}

impl FeatureFlagValidator {
    /// 创建新的验证器
    pub fn new() -> Self {
        let mut special_mappings = HashMap::new();

        // 特殊映射规则
        special_mappings.insert("authen".to_string(), "auth".to_string());
        special_mappings.insert("docx".to_string(), "cloud-docs".to_string());
        special_mappings.insert("drive".to_string(), "cloud-docs".to_string());
        special_mappings.insert(
            "personal_settings".to_string(),
            "personal-settings".to_string(),
        );
        special_mappings.insert("speech_to_text".to_string(), "speech-to-text".to_string());
        special_mappings.insert("optical_char_recognition".to_string(), "ocr".to_string());

        Self {
            api_mappings: Vec::new(),
            special_mappings,
        }
    }

    /// 从文件加载API映射数据
    pub fn load_from_file<P: AsRef<std::path::Path>>(
        &mut self,
        file_path: P,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        // 假设是CSV格式，简单解析
        self.parse_csv_data(&contents)?;
        Ok(())
    }

    /// 解析CSV数据
    fn parse_csv_data(&mut self, contents: &str) -> Result<(), Box<dyn std::error::Error>> {
        let lines: Vec<&str> = contents.lines().collect();

        for (i, line) in lines.iter().enumerate() {
            if i == 0 || line.trim().is_empty() {
                continue; // 跳过标题行和空行
            }

            // 简单处理CSV引号和逗号
            let fields = self.parse_csv_line(line);
            if fields.len() >= 7 {
                let mapping = ApiMapping {
                    name: fields.first().unwrap_or(&String::new()).clone(),
                    method: fields.get(1).unwrap_or(&String::new()).clone(),
                    path: fields.get(2).unwrap_or(&String::new()).clone(),
                    description: fields.get(3).unwrap_or(&String::new()).clone(),
                    self_build: fields.get(4).unwrap_or(&String::new()).clone(),
                    store_app: fields.get(5).unwrap_or(&String::new()).clone(),
                    doc_link: fields.get(6).unwrap_or(&String::new()).clone(),
                };
                self.api_mappings.push(mapping);
            }
        }

        Ok(())
    }

    /// 简单的CSV行解析（处理引号包围的字段）
    fn parse_csv_line(&self, line: &str) -> Vec<String> {
        let mut fields = Vec::new();
        let mut current = String::new();
        let mut in_quotes = false;
        let mut chars = line.chars().peekable();

        while let Some(ch) = chars.next() {
            match ch {
                '"' => {
                    if in_quotes && chars.peek() == Some(&'"') {
                        // 转义的引号
                        chars.next(); // 消耗下一个引号
                        current.push('"');
                    } else {
                        // 开启或关闭引号
                        in_quotes = !in_quotes;
                    }
                }
                ',' if !in_quotes => {
                    // 字段分隔符
                    fields.push(current.clone());
                    current.clear();
                }
                _ => {
                    current.push(ch);
                }
            }
        }

        // 添加最后一个字段
        fields.push(current);
        fields
    }

    /// 从API路径提取服务名称
    fn extract_service_from_path(&self, api_path: &str) -> Option<String> {
        // 解析路径1: /open-apis/{service}/{version}/{endpoint}
        if let Some(start) = api_path.find("/open-apis/") {
            let after_start = &api_path[start + 11..]; // 去掉 "/open-apis/"
            if let Some(end) = after_start.find('/') {
                return Some(after_start[..end].to_string());
            }
        }

        // 解析路径2: /approval/openapi/vX/{endpoint}
        if let Some(start) = api_path.find("/") {
            if let Some(end) = api_path.find("/openapi/") {
                let service_part = &api_path[start + 1..end]; // 提取服务名
                return Some(service_part.to_string());
            }
        }

        None
    }

    /// 获取期望的功能标志
    fn get_expected_feature(&self, service: &str) -> String {
        // 应用特殊映射规则
        if let Some(mapped) = self.special_mappings.get(service) {
            return mapped.clone();
        }
        service.to_string()
    }

    /// 验证所有API映射
    pub fn validate(&self) -> ValidationResult {
        let mut issues = Vec::new();
        let mut valid_count = 0;
        let mut missing_count = 0;

        for mapping in &self.api_mappings {
            let service_name = match self.extract_service_from_path(&mapping.path) {
                Some(name) => name,
                None => {
                    issues.push(ValidationIssue {
                        api_name: mapping.name.clone(),
                        api_path: mapping.path.clone(),
                        method: mapping.method.clone(),
                        current_feature: "未实现".to_string(),
                        expected_feature: "unknown".to_string(),
                        issue_type: IssueType::MissingFeature,
                        description: "无法从API路径提取服务名称".to_string(),
                    });
                    continue;
                }
            };

            let expected_feature = self.get_expected_feature(&service_name);

            // 检查该功能是否已经实现（通过检查现有的功能标志）
            let current_feature = self.get_current_feature_for_service(&service_name);

            if current_feature == expected_feature {
                valid_count += 1;
            } else {
                let issue_type = if current_feature.is_empty() {
                    IssueType::FeatureNotImplemented
                } else if current_feature != expected_feature {
                    IssueType::NamingMismatch
                } else {
                    IssueType::MissingFeature
                };

                if current_feature.is_empty() {
                    missing_count += 1;
                }

                issues.push(ValidationIssue {
                    api_name: mapping.name.clone(),
                    api_path: mapping.path.clone(),
                    method: mapping.method.clone(),
                    current_feature: if current_feature.is_empty() {
                        "未实现".to_string()
                    } else {
                        current_feature.clone()
                    },
                    expected_feature: expected_feature.clone(),
                    issue_type,
                    description: format!(
                        "API '{}' (路径: {}) 需要功能标志 '{}', 当前状态: '{}'",
                        mapping.name,
                        mapping.path,
                        expected_feature,
                        if current_feature.is_empty() {
                            "未实现"
                        } else {
                            &current_feature
                        }
                    ),
                });
            }
        }

        ValidationResult {
            total_apis: self.api_mappings.len(),
            valid_mappings: valid_count,
            invalid_mappings: issues.len() - missing_count,
            missing_features: missing_count,
            issues,
        }
    }

    /// 获取服务当前的功能标志（通过检查Cargo.toml等）
    fn get_current_feature_for_service(&self, service: &str) -> String {
        // 这里应该检查项目中实际的功能标志实现
        // 基于当前项目结构，更新已实现的功能列表
        let implemented_features = vec![
            "im",
            "contact",
            "group",
            "authentication",
            "search",
            "cloud-docs",
            "ai",
            "hire",
            "attendance",
            "approval",
            "calendar",
            "drive",
            "sheets",
            "bitable",
            "wiki",
            "meeting",
            "email",
            "ehr",
            "code",
            "finance",
            "admin",
            "report",
            "integration",
            "auth",
            "authen",
            "passport",
            "event",
            "interactive",
            "ocr",
            "verification",
            "task",
            "speech-to-text",
            "personal-settings",
            "cardkit",
            "corehr",
            "doc",
            "ephemeral",
            "security_and_compliance",
            "helpdesk",
            "translation",
            "aily",
            "board",
            "base",
            "vc",
            "user",
            "tenant",
            "minutes",
            "lingo",
            "moments",
            "application",
            "apaas",
            "human_authentication",
            "okr",
            "payroll",
            "pay",
            "face_verify",
            "acs",
            "directory",
            "performance",
            "docs",
            "workplace",
            "mdm",
            "suite",
            "compensation",
            "trust_party",
            "mail",
            "document_ai",
            "message",
            "meeting_room",
            "baike",
            // 新添加的云文档别名
            "docx",
            "drive",
        ];

        let expected_feature = self.get_expected_feature(service);
        if implemented_features.contains(&expected_feature.as_str()) {
            expected_feature
        } else {
            String::new()
        }
    }

    /// 生成修复建议
    pub fn generate_fix_suggestions(&self, validation_result: &ValidationResult) -> Vec<String> {
        let mut suggestions = Vec::new();

        if validation_result.missing_features > 0 {
            suggestions.push(format!(
                "需要为 {} 个API实现缺失的功能标志",
                validation_result.missing_features
            ));
        }

        if validation_result.invalid_mappings > 0 {
            suggestions.push(format!(
                "需要修正 {} 个API的功能标志映射",
                validation_result.invalid_mappings
            ));
        }

        // 统计需要特殊处理的映射
        let mut special_cases = std::collections::HashMap::new();
        for issue in &validation_result.issues {
            if let IssueType::NamingMismatch = issue.issue_type {
                *special_cases
                    .entry(issue.expected_feature.clone())
                    .or_insert(0) += 1;
            }
        }

        for (feature, count) in special_cases {
            suggestions.push(format!(
                "功能标志 '{}' 需要重命名或创建别名，涉及 {} 个API",
                feature, count
            ));
        }

        suggestions
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 开始验证功能标志映射...");

    let mut validator = FeatureFlagValidator::new();

    // 加载API映射数据
    let csv_file = "api_mapping_tools/server_api_list.csv";
    println!("📊 加载API映射数据: {}", csv_file);

    match validator.load_from_file(csv_file) {
        Ok(()) => println!("✅ API映射数据加载成功"),
        Err(e) => {
            println!("❌ 加载API映射数据失败: {}", e);
            return Err(e);
        }
    }

    // 执行验证
    println!("🔍 执行功能标志验证...");
    let result = validator.validate();

    // 输出结果
    println!("\n📈 验证结果:");
    println!("总API数量: {}", result.total_apis);
    println!("有效映射: {}", result.valid_mappings);
    println!("无效映射: {}", result.invalid_mappings);
    println!("缺失功能: {}", result.missing_features);

    let success_rate = if result.total_apis > 0 {
        (result.valid_mappings as f64 / result.total_apis as f64) * 100.0
    } else {
        0.0
    };

    println!("成功率: {:.1}%", success_rate);

    // 显示问题详情
    if !result.issues.is_empty() {
        println!("\n⚠️  发现的问题:");

        // 按问题类型分组
        let mut by_type = std::collections::HashMap::new();
        for issue in &result.issues {
            by_type
                .entry(format!("{:?}", issue.issue_type))
                .or_insert_with(Vec::new)
                .push(issue);
        }

        for (type_name, issues) in by_type {
            println!("\n  {}: {} 个问题", type_name, issues.len());
            for issue in issues.iter().take(5) {
                println!(
                    "    - {} ({} -> {})",
                    issue.api_name, issue.current_feature, issue.expected_feature
                );
            }
            if issues.len() > 5 {
                println!("    - ... 还有 {} 个类似问题", issues.len() - 5);
            }
        }

        // 生成修复建议
        let suggestions = validator.generate_fix_suggestions(&result);
        if !suggestions.is_empty() {
            println!("\n💡 修复建议:");
            for suggestion in suggestions {
                println!("  - {}", suggestion);
            }
        }
    }

    // 保存详细结果
    let output_file = "reports/feature_flag_validation_result.json";
    std::fs::create_dir_all("reports")?;
    let output_json = serde_json::to_string_pretty(&result)?;
    std::fs::write(output_file, output_json)?;

    println!("\n✅ 验证完成！详细结果已保存到: reports/feature_flag_validation_result.json");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_service_from_path() {
        let validator = FeatureFlagValidator::new();

        assert_eq!(
            validator.extract_service_from_path("/open-apis/authen/v1/user_info"),
            Some("authen".to_string())
        );
        assert_eq!(
            validator.extract_service_from_path("/open-apis/contact/v3/users"),
            Some("contact".to_string())
        );
        assert_eq!(validator.extract_service_from_path("/invalid/path"), None);
    }

    #[test]
    fn test_special_mappings() {
        let validator = FeatureFlagValidator::new();

        assert_eq!(validator.get_expected_feature("authen"), "auth");
        assert_eq!(validator.get_expected_feature("docx"), "cloud-docs");
        assert_eq!(validator.get_expected_feature("drive"), "cloud-docs");
        assert_eq!(validator.get_expected_feature("contact"), "contact");
    }
}
