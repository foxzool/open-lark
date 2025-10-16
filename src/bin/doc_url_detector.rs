use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;

/// API文档URL检测工具
///
/// 自动扫描代码库中的所有API方法，识别缺少文档URL的API，
/// 并生成详细的报告，为批量添加文档URL提供依据。

#[derive(Debug, Clone)]
struct ApiMethod {
    name: String,
    service: String,
    version: String,
    file_path: String,
    line_number: usize,
    has_doc_url: bool,
    signature: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let root_path = if args.len() > 1 {
        &args[1]
    } else {
        "."
    };

    println!("🔍 开始扫描项目: {}", root_path);
    println!();

    let mut apis = Vec::new();
    let service_path = format!("{}/src/service", root_path);

    if Path::new(&service_path).exists() {
        scan_directory(&service_path, &mut apis);
    } else {
        eprintln!("❌ 服务目录不存在: {}", service_path);
        std::process::exit(1);
    }

    generate_report(&apis);
}

fn scan_directory(dir_path: &str, apis: &mut Vec<ApiMethod>) {
    if let Ok(entries) = fs::read_dir(dir_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                scan_directory(&path.to_string_lossy(), apis);
            } else if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                scan_file(&path.to_string_lossy(), apis);
            }
        }
    }
}

fn scan_file(file_path: &str, apis: &mut Vec<ApiMethod>) {
    if let Ok(content) = fs::read_to_string(file_path) {
        let (service, version) = extract_service_info_from_path(file_path).unwrap_or_else(|_| {
            ("unknown".to_string(), "v1".to_string())
        });

        let lines: Vec<&str> = content.lines().collect();

        for (line_num, line) in lines.iter().enumerate() {
            if is_api_method(line) {
                let method_name = extract_method_name(line);
                let signature = extract_method_signature(line, &lines, line_num);
                let has_doc_url = check_has_documentation_url(&content, line_num);

                let api_method = ApiMethod {
                    name: method_name,
                    service: service.clone(),
                    version: version.clone(),
                    file_path: file_path.to_string(),
                    line_number: line_num + 1,
                    has_doc_url,
                    signature,
                };

                apis.push(api_method);
            }
        }
    }
}

fn extract_service_info_from_path(file_path: &str) -> Result<(String, String), String> {
    let path_parts: Vec<&str> = file_path.split('/').collect();

    let service_index = path_parts.iter()
        .position(|&part| part == "service")
        .ok_or("找不到service目录")?;

    if service_index + 3 >= path_parts.len() {
        return Err("路径格式不正确".to_string());
    }

    let service = path_parts[service_index + 1].to_string();
    let version_file = path_parts[service_index + 2];

    let version = if version_file.starts_with('v') {
        version_file.to_string()
    } else {
        "v1".to_string()
    };

    Ok((service, version))
}

fn is_api_method(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.starts_with("pub async fn") ||
    trimmed.starts_with("pub fn") ||
    (trimmed.starts_with("async fn") && trimmed.contains("-> SDKResult"))
}

fn extract_method_name(line: &str) -> String {
    let line = line.trim();
    if let Some(fn_pos) = line.find("fn ") {
        let after_fn = &line[fn_pos + 3..];
        if let Some(paren_pos) = after_fn.find('(') {
            return after_fn[..paren_pos].trim().to_string();
        }
    }
    "unknown_method".to_string()
}

fn extract_method_signature(line: &str, all_lines: &[&str], line_num: usize) -> String {
    let mut signature = line.trim().to_string();

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

fn check_has_documentation_url(content: &str, method_line_num: usize) -> bool {
    let lines: Vec<&str> = content.lines().collect();

    for i in (0..method_line_num).rev() {
        let line = lines[i].trim();
        if line.is_empty() {
            break;
        }

        if line.starts_with("///") &&
           (line.contains("open.feishu.cn") || line.contains("open.larksuite.com")) {
            return true;
        }
    }

    false
}

fn generate_report(apis: &[ApiMethod]) {
    let total_apis = apis.len();
    let apis_with_docs = apis.iter().filter(|api| api.has_doc_url).count();
    let apis_without_docs = total_apis - apis_with_docs;
    let coverage_percentage = if total_apis > 0 {
        (apis_with_docs as f64 / total_apis as f64) * 100.0
    } else {
        0.0
    };

    println!("# 飞书API文档URL检测报告");
    println!();

    println!("## 总体统计");
    println!("- 总API数量: {}", total_apis);
    println!("- 有文档URL的API: {}", apis_with_docs);
    println!("- 缺少文档URL的API: {}", apis_without_docs);
    println!("- 文档覆盖率: {:.1}%", coverage_percentage);
    println!();

    let mut service_stats: HashMap<String, Vec<&ApiMethod>> = HashMap::new();
    for api in apis {
        service_stats.entry(api.service.clone()).or_default().push(api);
    }

    println!("## 按服务统计");
    let mut services: Vec<_> = service_stats.values().collect();
    services.sort_by(|a, b| b.len().cmp(&a.len()));

    for service_apis in services {
        let service_name = &service_apis[0].service;
        let service_total = service_apis.len();
        let service_with_docs = service_apis.iter().filter(|api| api.has_doc_url).count();
        let service_without_docs = service_total - service_with_docs;
        let service_coverage = if service_total > 0 {
            (service_with_docs as f64 / service_total as f64) * 100.0
        } else {
            0.0
        };

        println!("### {} ({})", service_name, service_apis[0].version);
        println!("- 总API: {}", service_total);
        println!("- 有文档: {}", service_with_docs);
        println!("- 缺少文档: {}", service_without_docs);
        println!("- 覆盖率: {:.1}%", service_coverage);
        println!();
    }

    let missing_apis: Vec<&ApiMethod> = apis.iter()
        .filter(|api| !api.has_doc_url)
        .collect();

    println!("## 缺少文档URL的API列表");
    if missing_apis.is_empty() {
        println!("🎉 所有API都有文档URL！");
    } else {
        println!("总共发现 {} 个API缺少文档URL:", missing_apis.len());
        println!();

        let mut missing_by_service: HashMap<String, Vec<&ApiMethod>> = HashMap::new();
        for api in &missing_apis {
            missing_by_service.entry(api.service.clone()).or_default().push(api);
        }

        let mut services: Vec<_> = missing_by_service.keys().collect();
        services.sort();

        for service in services {
            let service_apis = &missing_by_service[service];
            println!("### {}", service);

            for api in service_apis {
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
    if apis_without_docs > 0 {
        println!("1. 使用 `doc_url!` 宏为核心服务API添加文档URL");
        println!("2. 优先为高频使用的服务添加文档（IM、云文档、通讯录）");
        println!("3. 建立自动化CI检查，确保新API都有文档URL");
        println!("4. 定期运行此检测工具监控文档覆盖率");

        // 设置退出码为1，表示有API缺少文档URL
        std::process::exit(1);
    } else {
        println!("✅ 文档URL覆盖率已达到100%！");
    }
}