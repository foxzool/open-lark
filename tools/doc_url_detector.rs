#!/usr/bin/env rust-script

use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use std::process;

/// API文档URL检测工具
///
/// 自动扫描代码库中的所有API方法，识别缺少文档URL的API，
/// 并生成详细的报告，为批量添加文档URL提供依据。

#[derive(Debug, Clone)]
struct ApiMethod {
    /// 方法名称
    name: String,
    /// 服务名称
    service: String,
    /// 版本
    version: String,
    /// 文件路径
    file_path: String,
    /// 行号
    line_number: usize,
    /// 是否已有文档URL
    has_doc_url: bool,
    /// 文档URL（如果存在）
    doc_url: Option<String>,
    /// 方法签名
    signature: String,
}

#[derive(Debug)]
struct DocUrlReport {
    /// 总API数量
    total_apis: usize,
    /// 有文档URL的API数量
    apis_with_docs: usize,
    /// 缺少文档URL的API数量
    apis_without_docs: usize,
    /// 覆盖率
    coverage_percentage: f64,
    /// 按服务分组的统计
    service_stats: HashMap<String, ServiceStats>,
    /// 缺少文档URL的API列表
    missing_apis: Vec<ApiMethod>,
}

#[derive(Debug)]
struct ServiceStats {
    /// 服务名称
    name: String,
    /// 总API数量
    total_apis: usize,
    /// 有文档URL的API数量
    apis_with_docs: usize,
    /// 缺少文档URL的API数量
    apis_without_docs: usize,
    /// 覆盖率
    coverage_percentage: f64,
}

/// API文档URL检测器
struct DocUrlDetector {
    /// 项目根目录
    root_path: String,
    /// 扫描结果
    apis: Vec<ApiMethod>,
}

impl DocUrlDetector {
    /// 创建新的检测器
    fn new(root_path: String) -> Self {
        Self {
            root_path,
            apis: Vec::new(),
        }
    }

    /// 扫描所有API方法
    fn scan_all_apis(&mut self) -> Result<(), String> {
        let service_path = format!("{}/src/service", self.root_path);

        if !Path::new(&service_path).exists() {
            return Err("服务目录不存在".to_string());
        }

        self.scan_directory(&service_path)?;
        Ok(())
    }

    /// 递归扫描目录
    fn scan_directory(&mut self, dir_path: &str) -> Result<(), String> {
        let entries = fs::read_dir(dir_path)
            .map_err(|e| format!("读取目录失败: {}", e))?;

        for entry in entries {
            let entry = entry.map_err(|e| format!("读取目录条目失败: {}", e))?;
            let path = entry.path();

            if path.is_dir() {
                let path_str = path.to_string_lossy();
                self.scan_directory(&path_str)?;
            } else if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                self.scan_file(&path.to_string_lossy())?;
            }
        }

        Ok(())
    }

    /// 扫描单个文件
    fn scan_file(&mut self, file_path: &str) -> Result<(), String> {
        let content = fs::read_to_string(file_path)
            .map_err(|e| format!("读取文件失败 {}: {}", file_path, e))?;

        let (service, version) = self.extract_service_info_from_path(file_path)?;

        let lines: Vec<&str> = content.lines().collect();

        for (line_num, line) in lines.iter().enumerate() {
            // 查找API方法定义
            if self.is_api_method(line) {
                let method_name = self.extract_method_name(line);
                let signature = self.extract_method_signature(line, &lines, line_num);
                let has_doc_url = self.check_has_documentation_url(&content, line_num);
                let doc_url = if has_doc_url {
                    self.extract_documentation_url(&content, line_num)
                } else {
                    None
                };

                let api_method = ApiMethod {
                    name: method_name,
                    service: service.clone(),
                    version: version.clone(),
                    file_path: file_path.to_string(),
                    line_number: line_num + 1,
                    has_doc_url,
                    doc_url,
                    signature,
                };

                self.apis.push(api_method);
            }
        }

        Ok(())
    }

    /// 从文件路径提取服务信息
    fn extract_service_info_from_path(&self, file_path: &str) -> Result<(String, String), String> {
        let path_parts: Vec<&str> = file_path.split('/').collect();

        let service_index = path_parts.iter()
            .position(|&part| part == "service")
            .ok_or("找不到service目录")?;

        if service_index + 3 >= path_parts.len() {
            return Err("路径格式不正确".to_string());
        }

        let service = path_parts[service_index + 1].to_string();
        let version_file = path_parts[service_index + 2];

        // 从版本目录中提取版本号（如 v1, v2 等）
        let version = if version_file.starts_with('v') {
            version_file.to_string()
        } else {
            // 如果不是版本目录，可能是其他文件，使用默认版本
            "v1".to_string()
        };

        Ok((service, version))
    }

    /// 判断是否为API方法
    fn is_api_method(&self, line: &str) -> bool {
        let trimmed = line.trim();
        trimmed.starts_with("pub async fn") ||
        trimmed.starts_with("pub fn") ||
        (trimmed.starts_with("async fn") && trimmed.contains("-> SDKResult"))
    }

    /// 提取方法名称
    fn extract_method_name(&self, line: &str) -> String {
        let line = line.trim();

        // 查找 "fn " 关键字后的方法名
        if let Some(fn_pos) = line.find("fn ") {
            let after_fn = &line[fn_pos + 3..];
            if let Some paren_pos) = after_fn.find('(') {
                return after_fn[..paren_pos].trim().to_string();
            }
        }

        "unknown_method".to_string()
    }

    /// 提取方法签名
    fn extract_method_signature(&self, line: &str, all_lines: &[&str], line_num: usize) -> String {
        let mut signature = line.trim().to_string();

        // 如果当前行不以 '{' 结尾，需要继续读取直到找到完整的签名
        if !signature.ends_with('{') && line_num + 1 < all_lines.len() {
            for i in (line_num + 1)..all_lines.len() {
                let next_line = all_lines[i].trim();
                signature.push_str(" ");
                signature.push_str(next_line);

                if next_line.ends_with('{') || next_line.ends_with(';') {
                    break;
                }
            }
        }

        signature
    }

    /// 检查是否有文档URL
    fn check_has_documentation_url(&self, content: &str, method_line_num: usize) -> bool {
        let lines: Vec<&str> = content.lines().collect();

        // 向上查找文档注释
        for i in (0..method_line_num).rev() {
            let line = lines[i].trim();

            // 如果遇到空行，停止搜索
            if line.is_empty() {
                break;
            }

            // 如果是文档注释且包含URL，返回true
            if line.starts_with("///") &&
               (line.contains("open.feishu.cn") || line.contains("open.larksuite.com")) {
                return true;
            }
        }

        false
    }

    /// 提取文档URL
    fn extract_documentation_url(&self, content: &str, method_line_num: usize) -> Option<String> {
        let lines: Vec<&str> = content.lines().collect();

        // 向上查找包含URL的文档注释
        for i in (0..method_line_num).rev() {
            let line = lines[i].trim();

            // 如果遇到空行，停止搜索
            if line.is_empty() {
                break;
            }

            // 如果是文档注释且包含URL，提取URL
            if line.starts_with("///") {
                if let Some(start) = line.find("https://") {
                    let url_part = &line[start..];
                    if let Some(end) = url_part.find(|c| c == ' ' || c == '>' || c == ')') {
                        return Some(url_part[..end].to_string());
                    } else {
                        return Some(url_part.to_string());
                    }
                }
            }
        }

        None
    }

    /// 生成报告
    fn generate_report(&self) -> DocUrlReport {
        let total_apis = self.apis.len();
        let apis_with_docs = self.apis.iter().filter(|api| api.has_doc_url).count();
        let apis_without_docs = total_apis - apis_with_docs;
        let coverage_percentage = if total_apis > 0 {
            (apis_with_docs as f64 / total_apis as f64) * 100.0
        } else {
            0.0
        };

        // 按服务分组统计
        let mut service_stats: HashMap<String, Vec<&ApiMethod>> = HashMap::new();
        for api in &self.apis {
            service_stats.entry(api.service.clone()).or_default().push(api);
        }

        let mut service_report = HashMap::new();
        for (service_name, apis) in service_stats {
            let service_total = apis.len();
            let service_with_docs = apis.iter().filter(|api| api.has_doc_url).count();
            let service_without_docs = service_total - service_with_docs;
            let service_coverage = if service_total > 0 {
                (service_with_docs as f64 / service_total as f64) * 100.0
            } else {
                0.0
            };

            service_report.insert(service_name, ServiceStats {
                name: service_name,
                total_apis: service_total,
                apis_with_docs: service_with_docs,
                apis_without_docs: service_without_docs,
                coverage_percentage: service_coverage,
            });
        }

        let missing_apis: Vec<ApiMethod> = self.apis.iter()
            .filter(|api| !api.has_doc_url)
            .cloned()
            .collect();

        DocUrlReport {
            total_apis,
            apis_with_docs,
            apis_without_docs,
            coverage_percentage,
            service_stats: service_report,
            missing_apis,
        }
    }
}

/// 打印报告
fn print_report(report: &DocUrlReport) {
    println!("# 飞书API文档URL检测报告");
    println!();

    println!("## 总体统计");
    println!("- 总API数量: {}", report.total_apis);
    println!("- 有文档URL的API: {}", report.apis_with_docs);
    println!("- 缺少文档URL的API: {}", report.apis_without_docs);
    println!("- 文档覆盖率: {:.1}%", report.coverage_percentage);
    println!();

    println!("## 按服务统计");
    let mut services: Vec<_> = report.service_stats.values().collect();
    services.sort_by(|a, b| b.total_apis.cmp(&a.total_apis));

    for service in services {
        println!("### {} (v{})", service.name,
                if service.name == "im" || service.name == "contact" { "v1/v3" } else { "v1" });
        println!("- 总API: {}", service.total_apis);
        println!("- 有文档: {}", service.apis_with_docs);
        println!("- 缺少文档: {}", service.apis_without_docs);
        println!("- 覆盖率: {:.1}%", service.coverage_percentage);
        println!();
    }

    println!("## 缺少文档URL的API列表");
    if report.missing_apis.is_empty() {
        println!("🎉 所有API都有文档URL！");
    } else {
        println!("总共发现 {} 个API缺少文档URL:", report.missing_apis.len());
        println!();

        // 按服务分组显示
        let mut missing_by_service: HashMap<String, Vec<&ApiMethod>> = HashMap::new();
        for api in &report.missing_apis {
            missing_by_service.entry(api.service.clone()).or_default().push(api);
        }

        let mut services: Vec<_> = missing_by_service.keys().collect();
        services.sort();

        for service in services {
            let apis = &missing_by_service[service];
            println!("### {}", service);

            for api in apis {
                println!("**{}** `{}::{}` (行 {})",
                        api.name, api.service, api.version, api.line_number);
                println!("```rust");
                println!("{}", api.signature);
                println!("```");
                println!("文件: `{}`", api.file_path);
                println!();
            }
        }
    }

    println!("## 建议操作");
    if report.apis_without_docs > 0 {
        println!("1. 使用 `doc_url!` 宏为核心服务API添加文档URL");
        println!("2. 优先为高频使用的服务添加文档（IM、云文档、通讯录）");
        println!("3. 建立自动化CI检查，确保新API都有文档URL");
        println!("4. 定期运行此检测工具监控文档覆盖率");
    } else {
        println!("✅ 文档URL覆盖率已达到100%！");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("用法: {} <项目根目录>", args[0]);
        eprintln!("示例: {} /path/to/open-lark", args[0]);
        process::exit(1);
    }

    let root_path = &args[1];

    println!("🔍 开始扫描项目: {}", root_path);
    println!();

    let mut detector = DocUrlDetector::new(root_path.clone());

    match detector.scan_all_apis() {
        Ok(()) => {
            let report = detector.generate_report();
            print_report(&report);

            // 如果有API缺少文档URL，设置退出码为1
            if report.apis_without_docs > 0 {
                process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("❌ 扫描失败: {}", e);
            process::exit(1);
        }
    }
}