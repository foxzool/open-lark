#!/usr/bin/env cargo script

//! 增强Builder模式项目最终验收脚本
//! 
//! 功能：
//! - 全面验证项目完成状态
//! - 检查代码质量和覆盖率
//! - 验证示例代码完整性
//! - 生成最终验收报告
//! 
//! 运行：
//! cargo script scripts/project_validation.rs

use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Debug, Default)]
struct ValidationReport {
    total_builders: usize,
    enhanced_builders: usize,
    missing_execute: Vec<String>,
    compilation_errors: Vec<String>,
    examples_validated: usize,
    docs_completed: usize,
    performance_benchmarks: usize,
    overall_score: f64,
}

fn main() {
    println!("🔍 增强Builder模式项目最终验收");
    println!("{}", "=".repeat(80));
    
    let mut report = ValidationReport::default();
    
    println!("\n📊 步骤1: 代码覆盖率检查");
    check_builder_coverage(&mut report);
    
    println!("\n🏗️ 步骤2: 编译验证");
    check_compilation(&mut report);
    
    println!("\n📚 步骤3: 示例验证");
    check_examples(&mut report);
    
    println!("\n📝 步骤4: 文档完整性");
    check_documentation(&mut report);
    
    println!("\n⚡ 步骤5: 性能基准测试");
    check_performance_benchmarks(&mut report);
    
    println!("\n📋 步骤6: 生成最终报告");
    generate_final_report(&mut report);
    
    println!("\n🎯 验收结论");
    print_conclusion(&report);
}

fn check_builder_coverage(report: &mut ValidationReport) {
    println!("  🔍 扫描Builder实现...");
    
    let service_dir = Path::new("src/service");
    scan_builders(service_dir, report);
    
    let coverage = if report.total_builders > 0 {
        (report.enhanced_builders as f64 / report.total_builders as f64) * 100.0
    } else {
        0.0
    };
    
    println!("  📊 总Builder数: {}", report.total_builders);
    println!("  ✅ 增强Builder数: {}", report.enhanced_builders);
    println!("  📈 覆盖率: {:.1}%", coverage);
    
    if !report.missing_execute.is_empty() {
        println!("  ⚠️  缺少execute方法的文件: {}", report.missing_execute.len());
        for missing in &report.missing_execute[..5.min(report.missing_execute.len())] {
            println!("     - {}", missing);
        }
        if report.missing_execute.len() > 5 {
            println!("     ... 及其他{}个文件", report.missing_execute.len() - 5);
        }
    }
}

fn scan_builders(dir: &Path, report: &mut ValidationReport) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                scan_builders(&path, report);
            } else if path.extension().map_or(false, |ext| ext == "rs") {
                check_builder_file(&path, report);
            }
        }
    }
}

fn check_builder_file(file_path: &Path, report: &mut ValidationReport) {
    if let Ok(content) = fs::read_to_string(file_path) {
        if content.contains("RequestBuilder") && content.contains("impl") && content.contains("pub fn build(") {
            report.total_builders += 1;
            
            if content.contains("pub async fn execute(") || content.contains("pub async fn execute_with_options(") {
                report.enhanced_builders += 1;
            } else {
                report.missing_execute.push(
                    file_path.strip_prefix("src/").unwrap_or(file_path)
                        .to_string_lossy().to_string()
                );
            }
        }
    }
}

fn check_compilation(report: &mut ValidationReport) {
    println!("  🔨 检查编译状态...");
    
    // 检查主项目编译
    let output = Command::new("cargo")
        .args(&["check", "--all-features"])
        .output()
        .expect("Failed to run cargo check");
    
    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        report.compilation_errors.push(format!("Main project: {}", error));
        println!("  ❌ 主项目编译失败");
    } else {
        println!("  ✅ 主项目编译成功");
    }
    
    // 检查示例编译
    let examples = ["enterprise_scenario_with_enhanced_builder", 
                   "data_processing_with_enhanced_builder",
                   "multi_service_integration_enhanced"];
    
    for example in &examples {
        let output = Command::new("cargo")
            .args(&["check", "--example", example])
            .output()
            .expect("Failed to check example");
            
        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            report.compilation_errors.push(format!("Example {}: {}", example, error));
            println!("  ❌ 示例 {} 编译失败", example);
        } else {
            println!("  ✅ 示例 {} 编译成功", example);
        }
    }
}

fn check_examples(report: &mut ValidationReport) {
    println!("  📚 验证示例完整性...");
    
    let examples_dir = Path::new("examples/api");
    let expected_examples = [
        "enterprise_scenario_with_enhanced_builder.rs",
        "data_processing_with_enhanced_builder.rs", 
        "multi_service_integration_enhanced.rs",
        "enhanced_drive_operations.rs",
    ];
    
    for example in &expected_examples {
        let example_path = examples_dir.join(example);
        if example_path.exists() {
            if let Ok(content) = fs::read_to_string(&example_path) {
                if content.contains(".execute(") && content.len() > 1000 {
                    report.examples_validated += 1;
                    println!("  ✅ {} - 完整且包含增强Builder使用", example);
                } else {
                    println!("  ⚠️  {} - 内容不完整或缺少增强Builder示例", example);
                }
            }
        } else {
            println!("  ❌ {} - 文件不存在", example);
        }
    }
    
    println!("  📊 验证通过的示例: {}/{}", report.examples_validated, expected_examples.len());
}

fn check_documentation(report: &mut ValidationReport) {
    println!("  📝 检查文档完整性...");
    
    let docs = [
        ("docs/enhanced-builder-final-summary.md", "最终项目总结"),
        ("docs/enhanced-builder-pattern-summary.md", "Builder模式总结"),
        ("docs/performance-optimization-guide.md", "性能优化指南"),
        ("benches/enhanced_builder_performance.rs", "性能基准测试"),
        ("scripts/code_quality_check.rs", "代码质量检查"),
    ];
    
    for (doc_path, description) in &docs {
        let path = Path::new(doc_path);
        if path.exists() {
            if let Ok(content) = fs::read_to_string(path) {
                if content.len() > 500 {
                    report.docs_completed += 1;
                    println!("  ✅ {} - {}", description, doc_path);
                } else {
                    println!("  ⚠️  {} - 内容太少: {}", description, doc_path);
                }
            }
        } else {
            println!("  ❌ {} - 文件不存在: {}", description, doc_path);
        }
    }
    
    println!("  📊 完成的文档: {}/{}", report.docs_completed, docs.len());
}

fn check_performance_benchmarks(report: &mut ValidationReport) {
    println!("  ⚡ 检查性能基准测试...");
    
    let bench_dir = Path::new("benches");
    if bench_dir.exists() {
        if let Ok(entries) = fs::read_dir(bench_dir) {
            for entry in entries.flatten() {
                if entry.path().extension().map_or(false, |ext| ext == "rs") {
                    report.performance_benchmarks += 1;
                    println!("  ✅ 发现基准测试: {}", entry.file_name().to_string_lossy());
                }
            }
        }
    }
    
    // 检查Cargo.toml中的benchmark配置
    if let Ok(cargo_content) = fs::read_to_string("Cargo.toml") {
        if cargo_content.contains("[[bench]]") {
            println!("  ✅ Cargo.toml包含基准测试配置");
        } else {
            println!("  ⚠️  Cargo.toml缺少基准测试配置");
        }
    }
    
    println!("  📊 基准测试文件数: {}", report.performance_benchmarks);
}

fn generate_final_report(report: &mut ValidationReport) {
    println!("  📋 计算项目完成度...");
    
    // 计算各项得分
    let coverage_score = if report.total_builders > 0 {
        (report.enhanced_builders as f64 / report.total_builders as f64) * 30.0
    } else {
        0.0
    };
    
    let compilation_score = if report.compilation_errors.is_empty() {
        25.0
    } else {
        25.0 * (1.0 - (report.compilation_errors.len() as f64 / 10.0).min(1.0))
    };
    
    let examples_score = (report.examples_validated as f64 / 4.0) * 20.0;
    let docs_score = (report.docs_completed as f64 / 5.0) * 15.0;
    let performance_score = if report.performance_benchmarks > 0 { 10.0 } else { 0.0 };
    
    report.overall_score = coverage_score + compilation_score + examples_score + docs_score + performance_score;
    
    println!("  🎯 评分明细:");
    println!("    - Builder覆盖率: {:.1}/30.0", coverage_score);
    println!("    - 编译质量: {:.1}/25.0", compilation_score);
    println!("    - 示例完整性: {:.1}/20.0", examples_score);
    println!("    - 文档完整性: {:.1}/15.0", docs_score);
    println!("    - 性能基准: {:.1}/10.0", performance_score);
    println!("    - 总分: {:.1}/100.0", report.overall_score);
}

fn print_conclusion(report: &ValidationReport) {
    println!("{}", "=".repeat(80));
    println!("🎯 最终验收结果");
    println!("{}", "=".repeat(80));
    
    let grade = match report.overall_score {
        score if score >= 95.0 => ("🏆 优秀", "生产就绪"),
        score if score >= 85.0 => ("🥇 良好", "基本就绪"),
        score if score >= 75.0 => ("🥈 合格", "需要改进"),
        _ => ("🥉 待改进", "需要重大修改"),
    };
    
    println!("\n📊 项目完成度: {:.1}%", report.overall_score);
    println!("🏅 评级: {}", grade.0);
    println!("📋 状态: {}", grade.1);
    
    println!("\n📈 关键指标:");
    println!("  • Builder增强覆盖率: {:.1}%", 
        if report.total_builders > 0 {
            (report.enhanced_builders as f64 / report.total_builders as f64) * 100.0
        } else { 0.0 }
    );
    println!("  • 编译成功率: {:.1}%", 
        if report.compilation_errors.is_empty() { 100.0 } else { 
            100.0 * (1.0 - (report.compilation_errors.len() as f64 / 10.0).min(1.0))
        }
    );
    println!("  • 示例完整度: {}/4", report.examples_validated);
    println!("  • 文档完成度: {}/5", report.docs_completed);
    
    if report.overall_score >= 90.0 {
        println!("\n🎉 恭喜！增强Builder模式项目已成功完成，达到生产级别标准！");
        println!("✅ 建议立即发布并推广使用");
    } else if report.overall_score >= 80.0 {
        println!("\n👍 项目基本完成，但仍有改进空间");
        println!("🔧 建议修复剩余问题后发布");
    } else {
        println!("\n⚠️  项目需要进一步完善才能发布");
        println!("🛠️  请优先解决高优先级问题");
    }
    
    if !report.missing_execute.is_empty() && report.missing_execute.len() > 10 {
        println!("\n💡 建议：优先为剩余{}个Builder添加execute()方法", report.missing_execute.len());
    }
    
    if !report.compilation_errors.is_empty() {
        println!("\n🔥 紧急：修复{}个编译错误", report.compilation_errors.len());
    }
}