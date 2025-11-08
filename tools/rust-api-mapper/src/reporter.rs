//! 报告生成器
//!
//! 负责生成Markdown和JSON格式的API实现映射报告。

use anyhow::{Result, Context};
use std::fs::File;
use std::io::Write;
use serde_json;
use tracing::{info, debug};

use crate::models::{URLDefinition, APIMatch, MappingReport};

/// 报告生成器
pub struct ReportGenerator {
    // 可以在这里添加配置选项
}

impl ReportGenerator {
    /// 创建新的报告生成器
    pub fn new() -> Self {
        Self {}
    }

    /// 生成Markdown报告
    pub fn generate_markdown_report(&self, match_results: &[APIMatch], output_path: &str) -> Result<()> {
        info!("生成Markdown报告: {}", output_path);

        let total_apis = match_results.len();
        let found_apis = match_results.iter()
            .filter(|r| r.status == crate::models::MatchStatus::Found)
            .count();

        let implementation_rate = if total_apis > 0 {
            (found_apis as f64 / total_apis as f64 * 100.0)
        } else {
            0.0
        };

        // 计算服务统计
        let mut service_stats = std::collections::HashMap::new();
        for result in match_results {
            let stats = service_stats
                .entry(result.api_info.service.clone())
                .or_insert((0, 0)); // (total, found)

            stats.0 += 1;
            if result.status == crate::models::MatchStatus::Found {
                stats.1 += 1;
            }
        }

        let mut content = String::new();

        // 报告标题和概述
        content.push_str("# Rust版本API实现映射表\n\n");
        content.push_str(&format!("**生成时间**: {}\n", chrono::Utc::now().format("%Y年%m月%d日 %H:%M:%S")));
        content.push_str("**匹配方法**: Rust版本高精度URL提取与匹配\n");
        content.push_str(&format!("**总API数**: {}\n", total_apis));
        content.push_str(&format!("**已实现**: {}\n", found_apis));
        content.push_str(&format!("**实现率**: {:.1}%\n\n", implementation_rate));

        // 服务统计
        content.push_str("## 按服务统计\n\n");
        content.push_str("| 服务 | 总数 | 已实现 | 实现率 |\n");
        content.push_str("|------|------|--------|--------|\n");

        let mut services: Vec<_> = service_stats.iter().collect();
        services.sort_by_key(|(service, _)| *service);

        for (service, (total, found)) in services {
            if service != "unknown" {
                let rate = if *total > 0 {
                    (*found as f64 / *total as f64 * 100.0)
                } else {
                    0.0
                };
                content.push_str(&format!("| {} | {} | {} | {:.1}% |\n", service, total, found, rate));
            }
        }

        // 详细映射表
        content.push_str("\n## 详细映射表\n\n");
        content.push_str("| 序号 | API名称 | HTTP方法 | 路径 | 状态 | 函数名 | 文件路径 | 行号 |\n");
        content.push_str("|------|---------|----------|------|------|--------|----------|------|\n");

        for (i, result) in match_results.iter().enumerate() {
            let status_emoji = if result.status == crate::models::MatchStatus::Found {
                "✅"
            } else {
                "❌"
            };

            let file_path = if let Some(impl_def) = &result.implementation {
                impl_def.file_path.display().to_string()
                    .replace("../src/", "")
            } else {
                "未找到".to_string()
            };

            let line_number = if let Some(impl_def) = &result.implementation {
                impl_def.line_start.to_string()
            } else {
                "-".to_string()
            };

            content.push_str(&format!(
                "| {} | {} | {} | {} | {} {} | {} | {} | {} |\n",
                i + 1,
                result.api_info.name,
                result.api_info.method.as_str(),
                result.api_info.path,
                status_emoji,
                format!("{:?}", result.status),
                result.function_info.as_ref()
                    .map(|f| f.name.as_str())
                    .unwrap_or("-"),
                file_path,
                line_number
            ));
        }

        // 写入文件
        let mut file = File::create(output_path)
            .with_context(|| format!("Failed to create markdown file: {}", output_path))?;

        file.write_all(content.as_bytes())
            .with_context(|| format!("Failed to write markdown file: {}", output_path))?;

        info!("Markdown报告生成完成");
        Ok(())
    }

    /// 生成JSON报告
    pub fn generate_json_report(
        &self,
        url_definitions: &[URLDefinition],
        match_results: &[APIMatch],
        output_path: &str,
    ) -> Result<()> {
        info!("生成JSON报告: {}", output_path);

        let total_apis = match_results.len();
        let found_apis = match_results.iter()
            .filter(|r| r.status == crate::models::MatchStatus::Found)
            .count();

        let implementation_rate = if total_apis > 0 {
            (found_apis as f64 / total_apis as f64 * 100.0) as f32
        } else {
            0.0
        };

        // 计算服务统计
        let mut service_stats = std::collections::HashMap::new();
        for result in match_results {
            let stats = service_stats
                .entry(result.api_info.service.clone())
                .or_insert(crate::models::ServiceStats::default());

            stats.total += 1;
            if result.status == crate::models::MatchStatus::Found {
                stats.found += 1;
            }
        }

        // 计算实现率
        for stats in service_stats.values_mut() {
            if stats.total > 0 {
                stats.implementation_rate = (stats.found as f32 / stats.total as f32) * 100.0;
            }
        }

        // 创建URL到函数的映射
        let mut url_function_map = std::collections::HashMap::new();
        for url_def in url_definitions {
            let url_key = format!("{}:{}", url_def.method_detection.method.as_str(), url_def.url);
            // 这里简化处理，实际应该包含完整的函数信息
            url_function_map.insert(url_key, crate::models::FunctionInfo {
                name: "TODO".to_string(),
                line_number: url_def.line_start,
                signature: url_def.raw_format.clone(),
                function_type: crate::models::FunctionType::PubAsync,
                search_distance: 0,
            });
        }

        let report = MappingReport {
            summary: crate::models::SummaryStats {
                total_apis,
                found_apis,
                implementation_rate,
                generation_time: chrono::Utc::now().to_rfc3339(),
                method: "rust_based_exact_matching".to_string(),
                url_definitions_found: url_definitions.len(),
                service_stats,
            },
            url_function_map,
            apis: match_results.to_vec(),
        };

        // 序列化为JSON
        let json_content = serde_json::to_string_pretty(&report)
            .context("Failed to serialize report to JSON")?;

        // 写入文件
        let mut file = File::create(output_path)
            .with_context(|| format!("Failed to create JSON file: {}", output_path))?;

        file.write_all(json_content.as_bytes())
            .with_context(|| format!("Failed to write JSON file: {}", output_path))?;

        info!("JSON报告生成完成");
        Ok(())
    }
}

impl Default for ReportGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use tempfile::NamedTempFile;

    #[test]
    fn test_markdown_report_generation() {
        let reporter = ReportGenerator::new();
        let match_results = vec![
            crate::models::APIMatch {
                api_info: crate::models::APIInfo {
                    name: "Test API".to_string(),
                    method: crate::models::HTTPMethod::Get,
                    path: "/open-apis/test/v1/resource".to_string(),
                    description: "Test API".to_string(),
                    self_build: "yes".to_string(),
                    store_app: "yes".to_string(),
                    doc_link: "https://example.com".to_string(),
                    service: "test".to_string(),
                    version: "v1".to_string(),
                },
                implementation: Some(crate::models::URLDefinition {
                    url: "/open-apis/test/v1/resource".to_string(),
                    method_detection: crate::models::HTTPMethodDetection {
                        method: crate::models::HTTPMethod::Get,
                        confidence: 0.9,
                        source: crate::models::MethodDetectionSource::ChainCall,
                        evidence: ".get()".to_string(),
                        line_number: 10,
                    },
                    line_start: 10,
                    line_end: 10,
                    raw_format: "format!(\"/open-apis/test/v1/resource\")".to_string(),
                    variables: vec![],
                    extraction_type: crate::models::URLExtractionType::SingleLine,
                    file_path: PathBuf::from("src/test.rs"),
                }),
                function_info: None,
                status: crate::models::MatchStatus::Found,
                match_confidence: 0.9,
                implementation_preview: "src/test.rs:10".to_string(),
            }
        ];

        let temp_file = NamedTempFile::new().unwrap();
        let result = reporter.generate_markdown_report(&match_results, temp_file.path().to_str().unwrap());
        assert!(result.is_ok());

        // 验证文件内容
        let content = std::fs::read_to_string(temp_file.path()).unwrap();
        assert!(content.contains("Rust版本API实现映射表"));
        assert!(content.contains("Test API"));
        assert!(content.contains("✅"));
    }

    #[test]
    fn test_json_report_generation() {
        let reporter = ReportGenerator::new();
        let url_definitions = vec![];
        let match_results = vec![];

        let temp_file = NamedTempFile::new().unwrap();
        let result = reporter.generate_json_report(&url_definitions, &match_results, temp_file.path().to_str().unwrap());
        assert!(result.is_ok());

        // 验证JSON格式
        let content = std::fs::read_to_string(temp_file.path()).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&content).unwrap();
        assert!(parsed.get("summary").is_some());
        assert!(parsed.get("apis").is_some());
    }
}