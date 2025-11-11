//! Rust API Mapper - 高性能API实现映射工具
//!
//! 这个工具用于从Open Lark项目的Rust代码中提取API实现信息，
//! 并与官方API列表进行匹配，生成详细的实现状态报告。
//!
//! 主要功能：
//! - 提取format!宏中的URL定义
//! - 智能检测HTTP方法
//! - 精确定位函数定义
//! - 类型安全的参数标准化
//! - 高性能并行处理

use clap::Parser;
use anyhow::Result;
use tracing::{info, error};
use tracing_subscriber;

mod parser;
mod detector;
mod normalizer;
mod matcher;
mod models;
mod scanner;
mod reporter;

use scanner::ServiceScanner;
use matcher::APIMatcher;
use reporter::ReportGenerator;

#[derive(Parser)]
#[command(name = "api_mapper")]
#[command(about = "A high-performance API implementation mapping tool", long_about = None)]
#[command(version)]
struct Args {
    /// 服务源代码目录路径
    #[arg(short, long, default_value = "../src/service")]
    service_dir: String,

    /// API列表CSV文件路径
    #[arg(short, long, default_value = "../server_api_list.csv")]
    api_list: String,

    /// 输出Markdown报告文件路径
    #[arg(long, default_value = "../complete_all_api_implementation_map_rust.md")]
    markdown_output: String,

    /// 输出JSON数据文件路径
    #[arg(long, default_value = "../api_implementation_data_rust.json")]
    json_output: String,

    /// 启用详细日志输出
    #[arg(short, long)]
    verbose: bool,

    /// 并行处理的线程数 (0表示使用所有CPU核心)
    #[arg(long, default_value = "0")]
    threads: usize,

    /// 启用性能分析模式
    #[arg(long)]
    profile: bool,
}

fn init_logging(verbose: bool) -> Result<()> {
    let filter = if verbose {
        "debug"
    } else {
        "info"
    };

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .with_thread_ids(false)
        .with_thread_names(false)
        .init();

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // 初始化日志
    init_logging(args.verbose)?;

    info!("🚀 启动Rust版本API映射工具");
    info!("📂 服务目录: {}", args.service_dir);
    info!("📋 API列表: {}", args.api_list);

    // 设置并行处理线程数
    if args.threads > 0 {
        rayon::ThreadPoolBuilder::new()
            .num_threads(args.threads)
            .build_global()?;
    }

    let start_time = std::time::Instant::now();

    // Phase 1: 扫描服务文件
    info!("🔍 Phase 1: 扫描服务文件...");
    let scanner = ServiceScanner::new(&args.service_dir);
    let url_definitions = scanner.scan_all_services().await?;
    info!("✅ 扫描完成，找到 {} 个URL定义", url_definitions.len());

    // Phase 2: 加载API列表
    info!("📋 Phase 2: 加载API列表...");
    let matcher = APIMatcher::new(&args.api_list).await?;
    info!("✅ 加载完成，共 {} 个API定义", matcher.api_count());

    // Phase 3: 执行匹配
    info!("🔗 Phase 3: 执行API匹配...");
    let match_results = matcher.match_apis(&url_definitions).await?;

    let matched_count = match_results.iter()
        .filter(|r| r.implementation.is_some())
        .count();

    let match_rate = (matched_count as f64 / match_results.len() as f64) * 100.0;

    info!("✅ 匹配完成！");
    info!("  总API数: {}", match_results.len());
    info!("  成功匹配: {}", matched_count);
    info!("  匹配率: {:.1}%", match_rate);

    // Phase 4: 生成报告
    info!("📝 Phase 4: 生成报告...");
    let reporter = ReportGenerator::new();
    reporter.generate_markdown_report(&match_results, &args.markdown_output)?;
    reporter.generate_json_report(&url_definitions, &match_results, &args.json_output)?;

    let elapsed = start_time.elapsed();
    info!("🎉 处理完成！耗时: {:?}", elapsed);
    info!("📄 Markdown报告: {}", args.markdown_output);
    info!("📄 JSON数据: {}", args.json_output);

    // 性能统计
    if args.profile {
        info!("📊 性能统计:");
        info!("  处理速度: {:.1} 文件/秒", url_definitions.len() as f64 / elapsed.as_secs_f64());
        info!("  匹配速度: {:.1} API/秒", match_results.len() as f64 / elapsed.as_secs_f64());
        info!("  内存使用: {} MB", get_memory_usage());
    }

    Ok(())
}

/// 获取当前进程的内存使用量（MB）
fn get_memory_usage() -> usize {
    #[cfg(unix)]
    {
        use std::fs;
        let status = fs::read_to_string("/proc/self/status").unwrap_or_default();
        for line in status.lines() {
            if line.starts_with("VmRSS:") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    return parts[1].parse::<usize>().unwrap_or(0) / 1024;
                }
            }
        }
    }
    0
}