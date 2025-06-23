// 代码质量检查脚本
//
// 这个脚本检查增强Builder模式实现的代码质量
// 包括：一致性检查、性能分析、最佳实践验证等

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 增强Builder模式代码质量检查");
    println!("{}", "=".repeat(60));
    
    let project_root = std::env::current_dir()?;
    let src_path = project_root.join("src");
    
    let mut stats = QualityStats::new();
    
    // 检查所有服务模块
    check_directory(&src_path, &mut stats)?;
    
    // 生成报告
    stats.generate_report();
    
    Ok(())
}

#[derive(Default)]
struct QualityStats {
    total_files: usize,
    enhanced_builders: usize,
    traditional_builders: usize,
    execute_methods: usize,
    execute_with_options_methods: usize,
    inconsistent_patterns: Vec<String>,
    missing_execute_methods: Vec<String>,
    performance_issues: Vec<String>,
    service_coverage: HashMap<String, BuilderCoverage>,
}

#[derive(Default)]
struct BuilderCoverage {
    total_builders: usize,
    enhanced_builders: usize,
    coverage_percentage: f64,
}

impl QualityStats {
    fn new() -> Self {
        Self::default()
    }
    
    fn generate_report(&self) {
        println!("\n📊 代码质量报告");
        println!("{}", "=".repeat(60));
        
        // 基础统计
        println!("\n📈 基础统计:");
        println!("  总文件数: {}", self.total_files);
        println!("  增强Builder: {}", self.enhanced_builders);
        println!("  传统Builder: {}", self.traditional_builders);
        println!("  execute方法: {}", self.execute_methods);
        println!("  execute_with_options方法: {}", self.execute_with_options_methods);
        
        // 覆盖率分析
        println!("\n📋 覆盖率分析:");
        let total_builders = self.enhanced_builders + self.traditional_builders;
        if total_builders > 0 {
            let coverage = (self.enhanced_builders as f64 / total_builders as f64) * 100.0;
            println!("  增强Builder覆盖率: {:.1}%", coverage);
            
            if coverage >= 90.0 {
                println!("  ✅ 覆盖率优秀");
            } else if coverage >= 70.0 {
                println!("  ⚠️  覆盖率良好，建议继续提升");
            } else {
                println!("  ❌ 覆盖率较低，需要改进");
            }
        }
        
        // 一致性检查
        println!("\n🔍 一致性检查:");
        if self.inconsistent_patterns.is_empty() {
            println!("  ✅ 所有增强Builder模式实现一致");
        } else {
            println!("  ❌ 发现不一致的模式实现:");
            for issue in &self.inconsistent_patterns {
                println!("    - {}", issue);
            }
        }
        
        // 缺失的execute方法
        println!("\n📝 缺失execute方法:");
        if self.missing_execute_methods.is_empty() {
            println!("  ✅ 所有Builder都已实现execute方法");
        } else {
            println!("  ❌ 以下Builder缺少execute方法:");
            for missing in &self.missing_execute_methods {
                println!("    - {}", missing);
            }
        }
        
        // 性能问题
        println!("\n⚡ 性能分析:");
        if self.performance_issues.is_empty() {
            println!("  ✅ 未发现明显的性能问题");
        } else {
            println!("  ⚠️  发现潜在性能问题:");
            for issue in &self.performance_issues {
                println!("    - {}", issue);
            }
        }
        
        // 服务覆盖率详情
        println!("\n🎯 服务模块覆盖率:");
        for (service, coverage) in &self.service_coverage {
            println!("  {}: {}/{} ({:.1}%)", 
                service, 
                coverage.enhanced_builders, 
                coverage.total_builders,
                coverage.coverage_percentage
            );
        }
        
        // 总体评价
        println!("\n🏆 总体评价:");
        let overall_score = self.calculate_overall_score();
        match overall_score {
            90..=100 => println!("  🌟 优秀 ({}分) - 增强Builder模式实现非常完善", overall_score),
            80..=89 => println!("  ✅ 良好 ({}分) - 增强Builder模式实现基本完善", overall_score),
            70..=79 => println!("  ⚠️  一般 ({}分) - 增强Builder模式需要改进", overall_score),
            _ => println!("  ❌ 需要改进 ({}分) - 增强Builder模式实现不足", overall_score),
        }
        
        // 改进建议
        println!("\n💡 改进建议:");
        self.generate_improvement_suggestions();
    }
    
    fn calculate_overall_score(&self) -> u8 {
        let mut score = 0u8;
        
        // 覆盖率权重: 40%
        let total_builders = self.enhanced_builders + self.traditional_builders;
        if total_builders > 0 {
            let coverage = (self.enhanced_builders as f64 / total_builders as f64) * 100.0;
            score += (coverage * 0.4) as u8;
        }
        
        // 一致性权重: 30%
        if self.inconsistent_patterns.is_empty() {
            score += 30;
        } else {
            score += (30 - (self.inconsistent_patterns.len() * 5).min(30)) as u8;
        }
        
        // 完整性权重: 20%
        if self.missing_execute_methods.is_empty() {
            score += 20;
        } else {
            score += (20 - (self.missing_execute_methods.len() * 2).min(20)) as u8;
        }
        
        // 性能权重: 10%
        if self.performance_issues.is_empty() {
            score += 10;
        } else {
            score += (10 - self.performance_issues.len().min(10)) as u8;
        }
        
        score.min(100)
    }
    
    fn generate_improvement_suggestions(&self) {
        let mut suggestions = Vec::new();
        
        // 基于覆盖率的建议
        let total_builders = self.enhanced_builders + self.traditional_builders;
        if total_builders > 0 {
            let coverage = (self.enhanced_builders as f64 / total_builders as f64) * 100.0;
            if coverage < 90.0 {
                suggestions.push(format!("提升增强Builder覆盖率到90%以上 (当前{:.1}%)", coverage));
            }
        }
        
        // 基于一致性的建议
        if !self.inconsistent_patterns.is_empty() {
            suggestions.push("统一所有增强Builder的实现模式".to_string());
        }
        
        // 基于完整性的建议
        if !self.missing_execute_methods.is_empty() {
            suggestions.push("为所有Builder添加execute和execute_with_options方法".to_string());
        }
        
        // 基于性能的建议
        if !self.performance_issues.is_empty() {
            suggestions.push("优化潜在的性能问题".to_string());
        }
        
        // 通用建议
        suggestions.push("添加更多单元测试验证增强Builder的功能".to_string());
        suggestions.push("编写更多文档和示例展示最佳实践".to_string());
        suggestions.push("设置CI/CD检查确保新代码遵循增强Builder模式".to_string());
        
        for (i, suggestion) in suggestions.iter().enumerate() {
            println!("  {}. {}", i + 1, suggestion);
        }
    }
}

fn check_directory(dir: &Path, stats: &mut QualityStats) -> Result<(), Box<dyn std::error::Error>> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                check_directory(&path, stats)?;
            } else if path.extension().map_or(false, |ext| ext == "rs") {
                check_rust_file(&path, stats)?;
            }
        }
    }
    Ok(())
}

fn check_rust_file(file_path: &Path, stats: &mut QualityStats) -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;
    stats.total_files += 1;
    
    // 检查是否是Builder文件
    if !content.contains("Builder") {
        return Ok(());
    }
    
    let file_name = file_path.to_string_lossy().to_string();
    
    // 检查增强Builder模式
    let has_execute = content.contains("pub async fn execute(");
    let has_execute_with_options = content.contains("pub async fn execute_with_options(");
    let has_builder_impl = content.contains("impl ") && content.contains("Builder");
    
    if has_builder_impl {
        if has_execute && has_execute_with_options {
            stats.enhanced_builders += 1;
            stats.execute_methods += 1;
            stats.execute_with_options_methods += 1;
            
            // 检查实现一致性
            check_implementation_consistency(&content, &file_name, stats);
        } else {
            stats.traditional_builders += 1;
            
            if !has_execute {
                stats.missing_execute_methods.push(format!("{} - 缺少execute方法", file_name));
            }
            if !has_execute_with_options {
                stats.missing_execute_methods.push(format!("{} - 缺少execute_with_options方法", file_name));
            }
        }
    }
    
    // 检查性能问题
    check_performance_issues(&content, &file_name, stats);
    
    // 更新服务覆盖率统计
    update_service_coverage(&file_path, has_execute && has_execute_with_options, stats);
    
    Ok(())
}

fn check_implementation_consistency(content: &str, file_name: &str, stats: &mut QualityStats) {
    // 检查execute方法是否有正确的注释
    if !content.contains("/// 直接执行") {
        stats.inconsistent_patterns.push(format!("{} - execute方法缺少标准注释", file_name));
    }
    
    // 检查是否使用了正确的返回类型
    if !content.contains("SDKResult<") {
        stats.inconsistent_patterns.push(format!("{} - 未使用SDKResult返回类型", file_name));
    }
    
    // 检查是否正确调用了build()方法
    if !content.contains("self.build()") {
        stats.inconsistent_patterns.push(format!("{} - execute方法未调用build()", file_name));
    }
}

fn check_performance_issues(content: &str, file_name: &str, stats: &mut QualityStats) {
    // 检查是否有不必要的clone操作
    if content.matches(".clone()").count() > 3 {
        stats.performance_issues.push(format!("{} - 可能存在过多的clone操作", file_name));
    }
    
    // 检查是否有不必要的String分配
    if content.matches("to_string()").count() > 5 {
        stats.performance_issues.push(format!("{} - 可能存在过多的字符串分配", file_name));
    }
    
    // 检查是否有同步的网络调用（不应该出现在增强Builder中）
    if content.contains("reqwest::blocking") {
        stats.performance_issues.push(format!("{} - 使用了同步网络调用", file_name));
    }
}

fn update_service_coverage(file_path: &Path, is_enhanced: bool, stats: &mut QualityStats) {
    // 从文件路径提取服务名称
    let path_str = file_path.to_string_lossy();
    let service_name = if path_str.contains("/sheets/") {
        "Sheets"
    } else if path_str.contains("/drive/") {
        "Drive"
    } else if path_str.contains("/im/") {
        "IM"
    } else if path_str.contains("/permission/") {
        "Permission"
    } else if path_str.contains("/wiki/") {
        "Wiki"
    } else if path_str.contains("/bitable/") {
        "Bitable"
    } else if path_str.contains("/board/") {
        "Board"
    } else {
        "Other"
    };
    
    let coverage = stats.service_coverage.entry(service_name.to_string()).or_default();
    coverage.total_builders += 1;
    
    if is_enhanced {
        coverage.enhanced_builders += 1;
    }
    
    coverage.coverage_percentage = if coverage.total_builders > 0 {
        (coverage.enhanced_builders as f64 / coverage.total_builders as f64) * 100.0
    } else {
        0.0
    };
}