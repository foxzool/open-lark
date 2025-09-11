#!/usr/bin/env cargo run --bin

//! # 增强版API设计一致性检查工具
//!
//! 提供详细的API分析、问题识别和改进建议生成功能

use std::{collections::HashMap, fs, path::Path};
use walkdir::WalkDir;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔧 增强版API设计一致性检查工具");
    println!("=====================================\n");

    let service_dir = "src/service";
    let mut results = HashMap::new();

    println!("🔍 扫描服务目录: {service_dir}");

    // 扫描所有服务文件
    for entry in WalkDir::new(service_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "rs"))
    {
        let path = entry.path();
        if let Ok(content) = fs::read_to_string(path) {
            let analysis = analyze_file_enhanced(&content, path);
            results.insert(path.to_string_lossy().to_string(), analysis);
        }
    }

    println!("📊 分析结果:");
    println!("检查了 {} 个文件\n", results.len());

    // 生成详细统计
    generate_detailed_statistics(&results);

    // 识别问题和生成建议
    let issues = identify_issues(&results);
    display_issues(&issues);

    // 生成增强报告
    let report_path = "reports/enhanced_api_consistency_report.md";
    generate_enhanced_report(&results, &issues, report_path)?;
    println!("\n📄 详细报告已生成: {report_path}");

    Ok(())
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct EnhancedFileAnalysis {
    file_path: String,
    method_count: u32,
    builder_patterns: u32,
    standard_response_usage: u32,
    documentation_count: u32,
    // 新增字段
    method_details: Vec<MethodInfo>,
    has_proper_error_handling: bool,
    api_patterns: Vec<APIPattern>,
    complexity_score: u32,
    improvement_priority: Priority,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct MethodInfo {
    name: String,
    line_number: usize,
    is_async: bool,
    has_builder: bool,
    uses_standard_response: bool,
    parameter_count: u32,
    return_type: String,
    has_documentation: bool,
}

#[derive(Debug, Clone)]
enum APIPattern {
    Crud,
    Query,
    Upload,
    Batch,
    Stream,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Priority {
    Critical, // 需要立即修复
    High,     // 高优先级改进
    Medium,   // 中等优先级
    Low,      // 低优先级
}

#[derive(Debug, Clone)]
struct Issue {
    file_path: String,
    issue_type: IssueType,
    description: String,
    severity: Priority,
    suggestion: String,
    example_fix: Option<String>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
enum IssueType {
    MissingStandardResponse,
    MissingBuilderPattern,
    PoorDocumentation,
    InconsistentNaming,
    ComplexParameters,
}

fn analyze_file_enhanced(content: &str, path: &Path) -> EnhancedFileAnalysis {
    let file_path = path.to_string_lossy().to_string();

    // 基础统计（保持与简化版兼容）
    let method_count =
        content.matches("pub fn").count() as u32 + content.matches("pub async fn").count() as u32;
    let builder_patterns = count_builder_patterns(content);
    let standard_response_usage = count_standard_response_usage(content);
    let documentation_count = content.matches("///").count() as u32;

    // 增强分析
    let method_details = extract_method_details(content);
    let has_proper_error_handling = check_error_handling(content);
    let api_patterns = identify_api_patterns(content, path);
    let complexity_score = calculate_complexity_score(&method_details, content);
    let improvement_priority =
        determine_priority(method_count, standard_response_usage, &method_details);

    EnhancedFileAnalysis {
        file_path,
        method_count,
        builder_patterns,
        standard_response_usage,
        documentation_count,
        method_details,
        has_proper_error_handling,
        api_patterns,
        complexity_score,
        improvement_priority,
    }
}

fn count_builder_patterns(content: &str) -> u32 {
    let mut count = 0;

    // 更精确的Builder模式检测
    if content.contains("Builder {") || content.contains("Builder<") {
        count += content.matches("Builder").count() as u32;
    }

    // 检查builder()方法
    count += content.matches("pub fn builder()").count() as u32;
    count += content.matches(".builder()").count() as u32;

    // 检查典型的Builder方法
    for method in ["build()", "with_", "set_"] {
        if content.contains(method) {
            count += 1;
            break;
        }
    }

    count
}

fn count_standard_response_usage(content: &str) -> u32 {
    let mut count = 0;

    // 精确检测.into_result()调用
    count += content.matches(".into_result()").count() as u32;

    // 检查StandardResponse trait使用
    if content.contains("use crate::core::standard_response::StandardResponse")
        || content.contains("StandardResponse")
    {
        count += 1;
    }

    count
}

fn extract_method_details(content: &str) -> Vec<MethodInfo> {
    let mut methods = Vec::new();
    let lines: Vec<&str> = content.lines().collect();

    for (line_num, line) in lines.iter().enumerate() {
        if line.trim_start().starts_with("pub async fn")
            || (line.trim_start().starts_with("pub fn") && !line.contains(" new("))
        {
            let method_info = parse_method_signature(line, line_num + 1, &lines);
            methods.push(method_info);
        }
    }

    methods
}

fn parse_method_signature(line: &str, line_number: usize, lines: &[&str]) -> MethodInfo {
    let is_async = line.contains("async");
    let name = extract_method_name(line);

    // 检查方法是否有builder支持（简化检查）
    let has_builder = line.contains("builder")
        || lines
            .iter()
            .any(|l| l.contains(&format!("{}Builder", name)));

    // 检查是否使用StandardResponse
    let uses_standard_response = lines
        .iter()
        .skip(line_number.saturating_sub(1))
        .take(20) // 检查方法体前20行
        .any(|l| l.contains(".into_result()"));

    // 估算参数数量（简化实现）
    let parameter_count = count_parameters(line);

    // 提取返回类型
    let return_type = extract_return_type(line);

    // 检查文档注释（检查方法前的注释）
    let has_documentation = line_number > 1
        && lines
            .get(line_number.saturating_sub(2))
            .is_some_and(|prev_line| prev_line.trim_start().starts_with("///"));

    MethodInfo {
        name: name.to_string(),
        line_number,
        is_async,
        has_builder,
        uses_standard_response,
        parameter_count,
        return_type,
        has_documentation,
    }
}

fn extract_method_name(line: &str) -> &str {
    if let Some(start) = line.find("fn ") {
        let after_fn = &line[start + 3..];
        if let Some(end) = after_fn.find('(') {
            return after_fn[..end].trim();
        }
    }
    "unknown"
}

fn count_parameters(line: &str) -> u32 {
    // 简化的参数计数，通过逗号数量估算
    let param_part = if let Some(start) = line.find('(') {
        if let Some(end) = line.find(')') {
            &line[start + 1..end]
        } else {
            ""
        }
    } else {
        ""
    };

    if param_part.trim().is_empty() || param_part.trim() == "&self" {
        0
    } else {
        // 排除&self，计算其他参数
        let params = param_part.split(',').count() as u32;
        if param_part.contains("&self") {
            params.saturating_sub(1)
        } else {
            params
        }
    }
}

fn extract_return_type(line: &str) -> String {
    if let Some(arrow_pos) = line.find("-> ") {
        let return_part = &line[arrow_pos + 3..];
        if let Some(brace_pos) = return_part.find(" {") {
            return_part[..brace_pos].trim().to_string()
        } else {
            return_part.trim().to_string()
        }
    } else {
        "()".to_string()
    }
}

fn check_error_handling(content: &str) -> bool {
    // 检查是否有适当的错误处理
    content.contains("SDKResult")
        || content.contains("Result<")
        || content.contains(".into_result()")
        || content.contains("LarkAPIError")
}

fn identify_api_patterns(content: &str, path: &Path) -> Vec<APIPattern> {
    let mut patterns = Vec::new();
    let path_str = path.to_string_lossy().to_lowercase();

    // 根据文件名和内容识别API模式
    if path_str.contains("create") || content.contains("pub fn create") || content.contains("POST")
    {
        patterns.push(APIPattern::Crud);
    }

    if path_str.contains("list")
        || path_str.contains("search")
        || content.contains("pub fn list")
        || content.contains("pub fn search")
    {
        patterns.push(APIPattern::Query);
    }

    if path_str.contains("upload") || content.contains("multipart") || content.contains("file") {
        patterns.push(APIPattern::Upload);
    }

    if content.contains("batch") || path_str.contains("batch") {
        patterns.push(APIPattern::Batch);
    }

    if content.contains("stream") || content.contains("WebSocket") {
        patterns.push(APIPattern::Stream);
    }

    patterns
}

fn calculate_complexity_score(methods: &[MethodInfo], content: &str) -> u32 {
    let mut score = 0;

    // 方法数量贡献
    score += methods.len() as u32;

    // 复杂参数贡献
    for method in methods {
        score += method.parameter_count;
    }

    // 内容复杂度
    score += (content.len() / 1000) as u32; // 每1000字符+1分

    // 嵌套结构复杂度
    score += content.matches("impl").count() as u32;
    score += content.matches("struct").count() as u32;
    score += content.matches("enum").count() as u32;

    score
}

fn determine_priority(
    method_count: u32,
    standard_response_usage: u32,
    _methods: &[MethodInfo],
) -> Priority {
    let sr_coverage = if method_count > 0 {
        (standard_response_usage as f32 / method_count as f32) * 100.0
    } else {
        0.0
    };

    // 根据StandardResponse覆盖率和方法数量确定优先级
    if method_count > 0 && sr_coverage < 10.0 {
        if method_count > 20 {
            Priority::Critical
        } else if method_count > 10 {
            Priority::High
        } else {
            Priority::Medium
        }
    } else if sr_coverage < 50.0 {
        Priority::Medium
    } else {
        Priority::Low
    }
}

fn identify_issues(results: &HashMap<String, EnhancedFileAnalysis>) -> Vec<Issue> {
    let mut issues = Vec::new();

    for analysis in results.values() {
        // 检查StandardResponse使用情况
        if analysis.method_count > 0 {
            let sr_coverage =
                (analysis.standard_response_usage as f32 / analysis.method_count as f32) * 100.0;

            if sr_coverage < 80.0 {
                issues.push(Issue {
                    file_path: analysis.file_path.clone(),
                    issue_type: IssueType::MissingStandardResponse,
                    description: format!(
                        "StandardResponse覆盖率仅{:.1}%，应该使用.into_result()统一错误处理",
                        sr_coverage
                    ),
                    severity: if sr_coverage < 20.0 {
                        Priority::Critical
                    } else {
                        Priority::High
                    },
                    suggestion: "为所有API方法添加.into_result()调用，统一响应处理".to_string(),
                    example_fix: Some(generate_standard_response_example()),
                });
            }
        }

        // 检查Builder模式
        let complex_methods = analysis
            .method_details
            .iter()
            .filter(|m| m.parameter_count > 3)
            .count();

        if complex_methods > 0 && analysis.builder_patterns == 0 {
            issues.push(Issue {
                file_path: analysis.file_path.clone(),
                issue_type: IssueType::MissingBuilderPattern,
                description: format!("有{}个复杂参数方法未使用Builder模式", complex_methods),
                severity: Priority::Medium,
                suggestion: "为复杂参数的方法添加Builder支持，提升易用性".to_string(),
                example_fix: Some(generate_builder_example()),
            });
        }

        // 检查文档覆盖率
        let documented_methods = analysis
            .method_details
            .iter()
            .filter(|m| m.has_documentation)
            .count();
        let doc_coverage = if analysis.method_count > 0 {
            (documented_methods as f32 / analysis.method_count as f32) * 100.0
        } else {
            0.0
        };

        if doc_coverage < 80.0 && analysis.method_count > 0 {
            issues.push(Issue {
                file_path: analysis.file_path.clone(),
                issue_type: IssueType::PoorDocumentation,
                description: format!("文档覆盖率仅{:.1}%，需要改进", doc_coverage),
                severity: Priority::Low,
                suggestion: "为所有公共API方法添加详细的文档注释".to_string(),
                example_fix: Some(generate_documentation_example()),
            });
        }
    }

    issues
}

fn generate_standard_response_example() -> String {
    r#"// 改进前
pub async fn search(&self, req: Request) -> SDKResult<BaseResponse<Response>> {
    Transport::request(api_req, &self.config, None).await
}

// 改进后  
pub async fn search(&self, req: Request) -> SDKResult<Response> {
    let api_resp: BaseResponse<Response> = 
        Transport::request(api_req, &self.config, None).await?;
    api_resp.into_result()
}"#
    .to_string()
}

fn generate_builder_example() -> String {
    r#"// 为复杂请求结构添加Builder
impl SearchRequest {
    pub fn builder() -> SearchRequestBuilder {
        SearchRequestBuilder::default()
    }
}

impl SearchRequestBuilder {
    pub fn page_size(mut self, size: i32) -> Self {
        self.inner.page_size = Some(size);
        self
    }
    
    pub fn build(self) -> SearchRequest {
        self.inner
    }
}"#
    .to_string()
}

fn generate_documentation_example() -> String {
    r#"/// 搜索工作台访问数据
///
/// 获取指定时间范围内的工作台访问统计信息
///
/// # Arguments
/// * `request` - 搜索请求参数
/// * `option` - 可选的请求配置
///
/// # Returns
/// 返回访问数据列表
pub async fn search(&self, request: SearchRequest) -> SDKResult<SearchResponse>"#
        .to_string()
}

fn generate_detailed_statistics(results: &HashMap<String, EnhancedFileAnalysis>) {
    let total_files = results.len();
    let total_methods: u32 = results.values().map(|a| a.method_count).sum();
    let total_builders: u32 = results.values().map(|a| a.builder_patterns).sum();
    let total_standard_response: u32 = results.values().map(|a| a.standard_response_usage).sum();

    println!("🎯 详细统计:");
    println!("   总文件数: {total_files}");
    println!("   总方法数: {total_methods}");
    println!(
        "   Builder模式覆盖率: {:.1}%",
        if total_methods > 0 {
            (total_builders as f32 / total_methods as f32) * 100.0
        } else {
            0.0
        }
    );
    println!(
        "   StandardResponse覆盖率: {:.1}%",
        if total_methods > 0 {
            (total_standard_response as f32 / total_methods as f32) * 100.0
        } else {
            0.0
        }
    );

    // 按优先级分类统计
    let mut priority_counts = HashMap::new();
    for analysis in results.values() {
        *priority_counts
            .entry(analysis.improvement_priority.clone())
            .or_insert(0u32) += 1;
    }

    println!("\n📋 改进优先级分布:");
    for (priority, count) in priority_counts {
        println!("   {:?}: {} 个文件", priority, count);
    }
}

fn display_issues(issues: &[Issue]) {
    println!("\n🚨 发现的问题:");
    let mut by_severity: HashMap<Priority, Vec<&Issue>> = HashMap::new();

    for issue in issues {
        by_severity
            .entry(issue.severity.clone())
            .or_default()
            .push(issue);
    }

    for severity in [
        Priority::Critical,
        Priority::High,
        Priority::Medium,
        Priority::Low,
    ] {
        if let Some(issues) = by_severity.get(&severity) {
            println!("\n{:?} 级问题 ({} 个):", severity, issues.len());
            for issue in issues.iter().take(3) {
                // 只显示前3个
                let file_name = Path::new(&issue.file_path)
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy();
                println!("   📁 {}: {}", file_name, issue.description);
            }
            if issues.len() > 3 {
                println!("   ... 还有 {} 个类似问题", issues.len() - 3);
            }
        }
    }
}

fn generate_enhanced_report(
    results: &HashMap<String, EnhancedFileAnalysis>,
    issues: &[Issue],
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // 确保目录存在
    if let Some(parent) = Path::new(output_path).parent() {
        fs::create_dir_all(parent)?;
    }

    let mut report = String::new();

    report.push_str("# API设计一致性增强检查报告\n\n");
    report.push_str(&format!(
        "生成时间: {}\n\n",
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
    ));

    // 执行摘要
    report.push_str("## 📋 执行摘要\n\n");
    let total_files = results.len();
    let total_methods: u32 = results.values().map(|a| a.method_count).sum();
    let total_issues = issues.len();

    report.push_str(&format!(
        "- **检查范围**: {} 个服务文件，{} 个API方法\n",
        total_files, total_methods
    ));
    report.push_str(&format!(
        "- **发现问题**: {} 个需要改进的项目\n",
        total_issues
    ));

    let critical_issues = issues
        .iter()
        .filter(|i| i.severity == Priority::Critical)
        .count();
    let high_issues = issues
        .iter()
        .filter(|i| i.severity == Priority::High)
        .count();

    report.push_str(&format!(
        "- **严重问题**: {} 个 Critical，{} 个 High 优先级\n\n",
        critical_issues, high_issues
    ));

    // 详细统计分析
    report.push_str("## 📊 详细统计分析\n\n");
    generate_statistics_section(&mut report, results);

    // 问题分析和建议
    report.push_str("## 🔍 问题分析和改进建议\n\n");
    generate_issues_section(&mut report, issues);

    // 改进路线图
    report.push_str("## 🗺️ 改进路线图\n\n");
    generate_roadmap_section(&mut report, results, issues);

    fs::write(output_path, report)?;
    Ok(())
}

fn generate_statistics_section(
    report: &mut String,
    results: &HashMap<String, EnhancedFileAnalysis>,
) {
    let total_methods: u32 = results.values().map(|a| a.method_count).sum();
    let total_builders: u32 = results.values().map(|a| a.builder_patterns).sum();
    let total_standard_response: u32 = results.values().map(|a| a.standard_response_usage).sum();

    let sr_coverage = if total_methods > 0 {
        (total_standard_response as f32 / total_methods as f32) * 100.0
    } else {
        0.0
    };

    let builder_coverage = if total_methods > 0 {
        (total_builders as f32 / total_methods as f32) * 100.0
    } else {
        0.0
    };

    report.push_str(&format!(
        "| 指标 | 当前值 | 目标值 | 达成率 |\n\
         |------|--------|--------|--------|\n\
         | StandardResponse覆盖率 | {:.1}% | 80% | {:.1}% |\n\
         | Builder模式覆盖率 | {:.1}% | 60% | {:.1}% |\n\
         | 总API方法数 | {} | - | - |\n\n",
        sr_coverage,
        (sr_coverage / 80.0) * 100.0,
        builder_coverage,
        (builder_coverage / 60.0) * 100.0,
        total_methods
    ));
}

fn generate_issues_section(report: &mut String, issues: &[Issue]) {
    let mut by_type: HashMap<String, Vec<&Issue>> = HashMap::new();

    for issue in issues {
        let type_name = format!("{:?}", issue.issue_type);
        by_type.entry(type_name).or_default().push(issue);
    }

    for (issue_type, issues) in by_type {
        report.push_str(&format!("### {}\n\n", issue_type));
        report.push_str(&format!("发现 {} 个相关问题:\n\n", issues.len()));

        // 显示前5个问题的详细信息
        for (i, issue) in issues.iter().take(5).enumerate() {
            let file_name = Path::new(&issue.file_path)
                .file_name()
                .unwrap_or_default()
                .to_string_lossy();

            report.push_str(&format!(
                "{}. **{}** ({:?})\n",
                i + 1,
                file_name,
                issue.severity
            ));
            report.push_str(&format!("   - 问题: {}\n", issue.description));
            report.push_str(&format!("   - 建议: {}\n", issue.suggestion));

            if let Some(ref example) = issue.example_fix {
                report.push_str("   - 示例修复:\n");
                report.push_str("   ```rust\n");
                report.push_str(example);
                report.push_str("\n   ```\n\n");
            }
        }

        if issues.len() > 5 {
            report.push_str(&format!("... 还有 {} 个类似问题\n\n", issues.len() - 5));
        }
    }
}

fn generate_roadmap_section(
    report: &mut String,
    results: &HashMap<String, EnhancedFileAnalysis>,
    _issues: &[Issue],
) {
    report.push_str("根据分析结果，建议按以下顺序进行改进:\n\n");

    // 按优先级分组文件
    let mut critical_files = Vec::new();
    let mut high_files = Vec::new();
    let mut medium_files = Vec::new();

    for analysis in results.values() {
        match analysis.improvement_priority {
            Priority::Critical => critical_files.push(&analysis.file_path),
            Priority::High => high_files.push(&analysis.file_path),
            Priority::Medium => medium_files.push(&analysis.file_path),
            Priority::Low => {}
        }
    }

    if !critical_files.is_empty() {
        report.push_str("### 🚨 第一阶段: Critical 优先级文件\n\n");
        for file in critical_files.iter().take(10) {
            let file_name = Path::new(file)
                .file_name()
                .unwrap_or_default()
                .to_string_lossy();
            report.push_str(&format!("- {}\n", file_name));
        }
        report.push('\n');
    }

    if !high_files.is_empty() {
        report.push_str("### ⚡ 第二阶段: High 优先级文件\n\n");
        for file in high_files.iter().take(10) {
            let file_name = Path::new(file)
                .file_name()
                .unwrap_or_default()
                .to_string_lossy();
            report.push_str(&format!("- {}\n", file_name));
        }
        report.push('\n');
    }

    if !medium_files.is_empty() {
        report.push_str("### 📋 第三阶段: Medium 优先级文件\n\n");
        report.push_str(&format!(
            "剩余 {} 个文件可以在后续阶段逐步改进。\n\n",
            medium_files.len()
        ));
    }

    report.push_str("### 🎯 成功标准\n\n");
    report.push_str("- StandardResponse覆盖率达到80%\n");
    report.push_str("- Builder模式覆盖率达到60%\n");
    report.push_str("- 所有Critical和High优先级问题得到解决\n");
    report.push_str("- 保持现有测试通过率100%\n\n");
}
