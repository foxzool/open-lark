//! 功能标志详细报告生成器
//!
//! 基于验证结果生成详细的功能标志实施报告

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

/// 验证结果（重用之前的数据结构）
#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationResult {
    pub total_apis: usize,
    pub valid_mappings: usize,
    pub invalid_mappings: usize,
    pub missing_features: usize,
    pub issues: Vec<ValidationIssue>,
}

/// 验证问题（重用之前的数据结构）
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

/// 服务汇总信息
#[derive(Debug)]
pub struct ServiceSummary {
    pub name: String,
    pub total_apis: usize,
    pub implemented_apis: usize,
    pub missing_apis: usize,
    pub implementation_rate: f64,
    pub priority: Priority,
}

/// 优先级
#[derive(Debug, Clone)]
pub enum Priority {
    High,
    Medium,
    Low,
}

/// 报告生成器
pub struct FeatureReportGenerator {
    validation_result: ValidationResult,
    services: HashMap<String, ServiceSummary>,
}

impl Default for FeatureReportGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl FeatureReportGenerator {
    /// 创建新的报告生成器
    pub fn new() -> Self {
        Self {
            validation_result: ValidationResult {
                total_apis: 0,
                valid_mappings: 0,
                invalid_mappings: 0,
                missing_features: 0,
                issues: Vec::new(),
            },
            services: HashMap::new(),
        }
    }

    /// 从文件加载验证结果
    pub fn load_validation_result<P: AsRef<std::path::Path>>(
        &mut self,
        file_path: P,
    ) -> Result<()> {
        let contents = fs::read_to_string(file_path)?;
        self.validation_result = serde_json::from_str(&contents)?;
        self.analyze_services();
        Ok(())
    }

    /// 分析服务实现情况
    fn analyze_services(&mut self) {
        let mut service_stats: HashMap<String, (usize, usize)> = HashMap::new();

        for issue in &self.validation_result.issues {
            let entry = service_stats
                .entry(issue.expected_feature.clone())
                .or_insert((0, 0));
            entry.0 += 1; // 总API数量

            if issue.current_feature != "未实现" {
                entry.1 += 1; // 已实现数量
            }
        }

        // 计算有效映射的服务
        // 暂时跳过复杂计算，使用默认值

        // 生成服务汇总
        for (service_name, (total, implemented)) in service_stats {
            let implementation_rate = if total > 0 {
                implemented as f64 / total as f64 * 100.0
            } else {
                0.0
            };

            let priority = self.determine_priority(&service_name, implementation_rate);

            let summary = ServiceSummary {
                name: service_name.clone(),
                total_apis: total,
                implemented_apis: implemented,
                missing_apis: total - implemented,
                implementation_rate,
                priority,
            };

            self.services.insert(service_name, summary);
        }
    }

    /// 确定服务优先级
    fn determine_priority(&self, service_name: &str, implementation_rate: f64) -> Priority {
        // 高优先级服务
        let high_priority_services = vec![
            "auth",
            "contact",
            "im",
            "group",
            "cloud-docs",
            "ai",
            "sheets",
            "bitable",
            "drive",
            "docx",
        ];

        // 中优先级服务
        let medium_priority_services = vec![
            "calendar",
            "meeting",
            "approval",
            "attendance",
            "hire",
            "search",
            "wiki",
            "email",
            "ehr",
            "admin",
        ];

        if high_priority_services.contains(&service_name) {
            if implementation_rate < 50.0 {
                Priority::High
            } else if implementation_rate < 80.0 {
                Priority::Medium
            } else {
                Priority::Low
            }
        } else if medium_priority_services.contains(&service_name) {
            if implementation_rate < 30.0 {
                Priority::High
            } else if implementation_rate < 70.0 {
                Priority::Medium
            } else {
                Priority::Low
            }
        } else if implementation_rate < 20.0 {
            Priority::Medium
        } else {
            Priority::Low
        }
    }

    /// 生成Markdown报告
    pub fn generate_markdown_report(&self) -> String {
        let mut report = String::new();

        // 标题和摘要
        report.push_str("# open-lark 功能标志实施报告\n\n");
        report.push_str(&format!(
            "生成时间: {}\n\n",
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
        ));

        // 总体统计
        report.push_str("## 📊 总体统计\n\n");
        report.push_str(&format!(
            "- **总API数量**: {}\n",
            self.validation_result.total_apis
        ));
        report.push_str(&format!(
            "- **有效映射**: {} ({:.1}%)\n",
            self.validation_result.valid_mappings,
            self.get_success_rate()
        ));
        report.push_str(&format!(
            "- **需要实现的功能**: {}\n",
            self.validation_result.missing_features
        ));
        report.push_str(&format!(
            "- **需要修正的映射**: {}\n",
            self.validation_result.invalid_mappings
        ));

        // 服务实施状况
        report.push_str("\n## 🏗️ 服务实施状况\n\n");
        report.push_str("| 服务名称 | 总API数 | 已实现 | 缺失 | 完成率 | 优先级 |\n");
        report.push_str("|---------|--------|--------|------|--------|--------|\n");

        let mut services: Vec<_> = self.services.values().collect();
        services.sort_by(|a, b| match (a.priority.clone(), b.priority.clone()) {
            (Priority::High, Priority::Medium | Priority::Low) => std::cmp::Ordering::Less,
            (Priority::Medium, Priority::Low) => std::cmp::Ordering::Less,
            (Priority::High, Priority::High) => b
                .implementation_rate
                .partial_cmp(&a.implementation_rate)
                .unwrap_or(std::cmp::Ordering::Equal),
            (Priority::Medium, Priority::Medium) => b
                .implementation_rate
                .partial_cmp(&a.implementation_rate)
                .unwrap_or(std::cmp::Ordering::Equal),
            (Priority::Low, Priority::Low) => b
                .implementation_rate
                .partial_cmp(&a.implementation_rate)
                .unwrap_or(std::cmp::Ordering::Equal),
            _ => std::cmp::Ordering::Greater,
        });

        for service in &services {
            let priority_icon = match service.priority {
                Priority::High => "🔴",
                Priority::Medium => "🟡",
                Priority::Low => "🟢",
            };

            report.push_str(&format!(
                "| {} {} | {} | {} | {} | {:.1}% | {:?} |\n",
                service.name,
                priority_icon,
                service.total_apis,
                service.implemented_apis,
                service.missing_apis,
                service.implementation_rate,
                service.priority
            ));
        }

        // 高优先级任务
        report.push_str("\n## 🔥 高优先级任务\n\n");

        let high_priority_services: Vec<_> = services
            .iter()
            .filter(|s| matches!(s.priority, Priority::High))
            .collect();

        if !high_priority_services.is_empty() {
            report.push_str("以下服务需要优先实施：\n\n");
            for service in high_priority_services {
                report.push_str(&format!(
                    "### {}\n- **状态**: {:.1}% 完成 ({}/{})\n- **建议**: 立即开始实施剩余 {} 个API\n\n",
                    service.name,
                    service.implementation_rate,
                    service.implemented_apis,
                    service.total_apis,
                    service.missing_apis
                ));
            }
        }

        // 实施计划
        report.push_str("## 📅 实施计划建议\n\n");
        report.push_str("### 第一阶段（1-2周）：核心服务完善\n");

        let phase1_services: Vec<_> = services
            .iter()
            .filter(|s| matches!(s.priority, Priority::High) && s.name != "auth")
            .take(5)
            .map(|s| s.name.as_str())
            .collect();

        if !phase1_services.is_empty() {
            report.push_str(&format!(
                "1. 完成 {} 服务的完整实现\n",
                phase1_services.join("、")
            ));
        }
        report.push_str("2. 修复 authen → auth 的命名不匹配\n");
        report.push_str("3. 统一 docx/drive 到 cloud-docs 功能标志\n\n");

        report.push_str("### 第二阶段（3-4周）：扩展服务覆盖\n");

        let phase2_services: Vec<_> = services
            .iter()
            .filter(|s| matches!(s.priority, Priority::Medium))
            .take(8)
            .map(|s| s.name.as_str())
            .collect();

        if !phase2_services.is_empty() {
            report.push_str(&format!("1. 实施 {} 服务\n", phase2_services.join("、")));
        }
        report.push_str("2. 完善测试覆盖\n");
        report.push_str("3. 更新文档和示例\n\n");

        report.push_str("### 第三阶段（5-6周）：全面覆盖和优化\n");
        report.push_str("1. 完成所有剩余服务\n");
        report.push_str("2. 性能优化和代码重构\n");
        report.push_str("3. 完善开发者工具\n\n");

        // 技术建议
        report.push_str("## 💡 技术建议\n\n");
        report.push_str("### 代码组织\n");
        report.push_str("- 使用共享数据模型减少重复代码\n");
        report.push_str("- 建立统一的API实现模式\n");
        report.push_str("- 完善错误处理和日志记录\n\n");

        report.push_str("### 质量保证\n");
        report.push_str("- 为每个新功能编写单元测试\n");
        report.push_str("- 使用自动化工具验证API一致性\n");
        report.push_str("- 定期运行完整测试套件\n\n");

        report.push_str("### 文档和示例\n");
        report.push_str("- 为每个功能标志提供使用示例\n");
        report.push_str("- 维护API映射文档的最新状态\n");
        report.push_str("- 提供迁移指南和最佳实践\n\n");

        // 风险评估
        report.push_str("## ⚠️ 风险评估\n\n");
        report.push_str("### 高风险项目\n");
        report.push_str("- **authen → auth 重命名**: 可能影响现有用户代码\n");
        report.push_str("- **云文档服务统一**: 需要仔细处理API兼容性\n\n");

        report.push_str("### 缓解措施\n");
        report.push_str("- 提供向后兼容的别名\n");
        report.push_str("- 分阶段实施，确保每个阶段都能正常工作\n");
        report.push_str("- 保持完整的变更日志\n\n");

        report
    }

    /// 获取成功率
    fn get_success_rate(&self) -> f64 {
        if self.validation_result.total_apis > 0 {
            self.validation_result.valid_mappings as f64 / self.validation_result.total_apis as f64
                * 100.0
        } else {
            0.0
        }
    }

    /// 保存报告
    pub fn save_report<P: AsRef<std::path::Path>>(&self, file_path: P) -> Result<()> {
        let report = self.generate_markdown_report();
        fs::write(file_path, report)?;
        Ok(())
    }
}

fn main() -> Result<()> {
    println!("📝 生成功能标志实施报告...");

    let mut generator = FeatureReportGenerator::new();

    // 加载验证结果
    match generator.load_validation_result("reports/feature_flag_validation_result.json") {
        Ok(()) => println!("✅ 验证结果加载成功"),
        Err(e) => {
            println!("❌ 加载验证结果失败: {}", e);
            println!("请先运行功能标志验证工具");
            return Err(e);
        }
    }

    // 生成并保存报告
    let report_file = "reports/feature_flag_implementation_report.md";
    generator.save_report(report_file)?;

    println!("✅ 报告生成完成！");
    println!("📄 报告已保存到: {}", report_file);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority_determination() {
        let generator = FeatureReportGenerator::new();

        // 高优先级服务，低完成率
        assert!(matches!(
            generator.determine_priority("auth", 30.0),
            Priority::High
        ));

        // 高优先级服务，高完成率
        assert!(matches!(
            generator.determine_priority("auth", 90.0),
            Priority::Low
        ));

        // 低优先级服务
        assert!(matches!(
            generator.determine_priority("some_service", 50.0),
            Priority::Low
        ));
    }
}
