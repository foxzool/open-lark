#!/usr/bin/env cargo run --bin

//! # 简化版API设计一致性检查工具
//!
//! 快速检查API设计一致性的工具

use std::{collections::HashMap, fs, path::Path};
use walkdir::WalkDir;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔧 API设计一致性简化检查工具");
    println!("===============================\n");

    let service_dir = "src/service";
    let mut results = HashMap::new();

    println!("🔍 扫描服务目录: {service_dir}");

    for entry in WalkDir::new(service_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "rs"))
    // 移除文件数量限制以确保扫描所有服务文件
    {
        let path = entry.path();
        if let Ok(content) = fs::read_to_string(path) {
            let analysis = analyze_file(&content, path);
            results.insert(path.to_string_lossy().to_string(), analysis);
        }
    }

    println!("📊 分析结果:");
    println!("检查了 {} 个文件\n", results.len());

    // 统计结果
    let mut total_methods = 0;
    let mut builder_count = 0;
    let mut standard_response_count = 0;
    let mut documented_count = 0;

    for (file_path, analysis) in &results {
        println!(
            "📁 {}",
            file_path.split('/').next_back().unwrap_or(file_path)
        );
        println!("   方法数: {}", analysis.method_count);
        println!("   Builder支持: {}", analysis.builder_patterns);
        println!("   StandardResponse: {}", analysis.standard_response_usage);
        println!("   文档注释: {}", analysis.documentation_count);
        println!();

        total_methods += analysis.method_count;
        builder_count += analysis.builder_patterns;
        standard_response_count += analysis.standard_response_usage;
        documented_count += analysis.documentation_count;
    }

    println!("🎯 总结统计:");
    println!("   总方法数: {total_methods}");
    println!(
        "   Builder模式覆盖率: {:.1}%",
        if total_methods > 0 {
            (builder_count as f32 / total_methods as f32) * 100.0
        } else {
            0.0
        }
    );
    println!(
        "   StandardResponse覆盖率: {:.1}%",
        if total_methods > 0 {
            (standard_response_count as f32 / total_methods as f32) * 100.0
        } else {
            0.0
        }
    );
    println!(
        "   文档覆盖率: {:.1}%",
        if total_methods > 0 {
            (documented_count as f32 / total_methods as f32) * 100.0
        } else {
            0.0
        }
    );

    // 生成简化报告
    let report_path = "reports/simple_api_consistency_report.md";
    generate_simple_report(&results, report_path)?;
    println!("\n📄 报告已生成: {report_path}");

    Ok(())
}

#[derive(Debug)]
struct FileAnalysis {
    method_count: u32,
    builder_patterns: u32,
    standard_response_usage: u32,
    documentation_count: u32,
}

fn analyze_file(content: &str, _path: &Path) -> FileAnalysis {
    // 计算方法数（统计 pub fn 和 pub async fn）
    let method_count =
        content.matches("pub fn").count() as u32 + content.matches("pub async fn").count() as u32;

    // 计算Builder模式（寻找 Builder 结构体和Builder方法）
    let builder_patterns = content.matches("Builder {").count() as u32
        + content.matches("Builder::").count() as u32
        + content.matches("_builder()").count() as u32;

    // 计算StandardResponse使用（寻找 into_result 调用和 StandardResponse import）
    let standard_response_usage = content.matches(".into_result()").count() as u32
        + content.matches("StandardResponse").count() as u32;

    // 计算文档注释
    let documentation_count = content.matches("///").count() as u32;

    FileAnalysis {
        method_count,
        builder_patterns,
        standard_response_usage,
        documentation_count,
    }
}

fn generate_simple_report(
    results: &HashMap<String, FileAnalysis>,
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut report = String::new();

    report.push_str("# API设计一致性简化检查报告\n\n");
    report.push_str(&format!(
        "生成时间: {}\n\n",
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
    ));

    // 总体统计
    let total_files = results.len();
    let total_methods: u32 = results.values().map(|a| a.method_count).sum();
    let total_builders: u32 = results.values().map(|a| a.builder_patterns).sum();
    let total_standard_response: u32 = results.values().map(|a| a.standard_response_usage).sum();
    let total_docs: u32 = results.values().map(|a| a.documentation_count).sum();

    report.push_str("## 📊 总体统计\n\n");
    report.push_str(&format!("- 检查的服务文件数: {total_files}\n"));
    report.push_str(&format!("- 总方法数: {total_methods}\n"));
    report.push_str(&format!("- Builder模式数: {total_builders}\n"));
    report.push_str(&format!(
        "- StandardResponse使用数: {total_standard_response}\n"
    ));
    report.push_str(&format!("- 文档注释数: {total_docs}\n\n"));

    // 覆盖率统计
    report.push_str("## 📈 覆盖率分析\n\n");
    if total_methods > 0 {
        let builder_rate = (total_builders as f32 / total_methods as f32) * 100.0;
        let response_rate = (total_standard_response as f32 / total_methods as f32) * 100.0;
        let doc_rate = (total_docs as f32 / total_methods as f32) * 100.0;

        report.push_str(&format!("- Builder模式覆盖率: {builder_rate:.1}%\n"));
        report.push_str(&format!("- StandardResponse覆盖率: {response_rate:.1}%\n"));
        report.push_str(&format!("- 文档覆盖率: {doc_rate:.1}%\n\n"));

        // 评级
        let avg_rate = (builder_rate + response_rate + doc_rate) / 3.0;
        report.push_str("## 🎯 整体评级\n\n");
        if avg_rate >= 80.0 {
            report.push_str("✅ **优秀** - API设计一致性良好\n\n");
        } else if avg_rate >= 60.0 {
            report.push_str("⚠️ **良好** - API设计有改进空间\n\n");
        } else {
            report.push_str("🔴 **需要改进** - 需要重点关注API设计一致性\n\n");
        }
    }

    // 详细文件分析
    report.push_str("## 📋 文件详细分析\n\n");
    for (file_path, analysis) in results {
        let file_name = file_path.split('/').next_back().unwrap_or(file_path);
        report.push_str(&format!("### {file_name}\n"));
        report.push_str(&format!("- 方法数: {}\n", analysis.method_count));
        report.push_str(&format!("- Builder模式: {}\n", analysis.builder_patterns));
        report.push_str(&format!(
            "- StandardResponse: {}\n",
            analysis.standard_response_usage
        ));
        report.push_str(&format!("- 文档注释: {}\n\n", analysis.documentation_count));
    }

    // 改进建议
    report.push_str("## 🚀 改进建议\n\n");
    report.push_str("1. **标准化错误处理**: 在所有API方法中使用StandardResponse.into_result()\n");
    report.push_str("2. **完善Builder模式**: 为复杂的创建方法添加Builder支持\n");
    report.push_str("3. **增加文档**: 为所有公开API添加详细的文档注释\n");
    report.push_str("4. **代码一致性**: 保持命名约定和结构的一致性\n\n");

    fs::write(output_path, report)?;
    Ok(())
}
