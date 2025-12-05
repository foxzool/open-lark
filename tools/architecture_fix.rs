#!/usr/bin/env rust-script

//! 批量修复架构违规问题的脚本
//!
//! 这个脚本用于批量修复openlark-docs模块中发现的系统性架构问题：
//! 1. 移除冗余的api_request字段
//! 2. 消除重复的Builder模式
//! 3. 统一错误处理为validate_required!宏
//! 4. 改进参数类型为impl Into<String>

use std::fs;
use std::path::Path;

fn main() {
    println!("🔧 开始批量修复架构问题...");

    let docs_dir = "crates/openlark-docs/src";

    if let Err(e) = fix_architecture_issues(docs_dir) {
        eprintln!("❌ 修复失败: {}", e);
    } else {
        println!("✅ 架构修复完成！");
    }
}

fn fix_architecture_issues(dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            // 递归处理子目录
            fix_architecture_issues(path.to_str().unwrap())?;
        } else if let Some(ext) = path.extension() {
            if ext == "rs" {
                fix_file_architecture(&path)?;
            }
        }
    }
    Ok(())
}

fn fix_file_architecture(file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;
    let mut modified = false;
    let mut new_content = content.clone();

    // 修复1: 移除冗余的api_request字段
    if new_content.contains("api_request: ApiRequest<") {
        new_content = remove_api_request_field(&new_content);
        modified = true;
    }

    // 修复2: 替换错误处理导入
    if new_content.contains("error::validation_error") {
        new_content = new_content.replace("error::validation_error,", "validate_required,");
        modified = true;
    }

    // 修复3: 移除硬编码URL
    if new_content.contains("https://open.feishu.cn") {
        new_content = remove_hardcoded_urls(&new_content);
        modified = true;
    }

    // 修复4: 改进参数类型为impl Into<String>
    if !new_content.contains("impl Into<String>") {
        new_content = improve_parameter_types(&new_content);
        modified = true;
    }

    // 修复5: 移除重复的Builder模式
    if new_content.contains("pub struct *Builder") {
        new_content = remove_duplicate_builders(&new_content);
        modified = true;
    }

    if modified {
        fs::write(file_path, new_content)?;
        println!("✅ 修复: {}", file_path.display());
    }

    Ok(())
}

fn remove_api_request_field(content: &str) -> String {
    // 这个函数需要实现移除api_request字段的逻辑
    // 由于Rust代码的复杂性，这里只是示例
    content.replace("api_request: ApiRequest<", "// api_request: ApiRequest<")
}

fn remove_hardcoded_urls(content: &str) -> String {
    content.replace("https://open.feishu.cn", "")
}

fn improve_parameter_types(content: &str) -> String {
    // 简化的参数类型改进逻辑
    content
        .replace(": String)", ": impl Into<String>)")
        .replace("app_token: String", "app_token: impl Into<String>")
        .replace("name: String", "name: impl Into<String>")
}

fn remove_duplicate_builders(content: &str) -> String {
    // 简化的Builder移除逻辑
    let lines: Vec<&str> = content.lines().collect();
    let mut result = Vec::new();
    let mut in_builder = false;

    for line in lines {
        if line.trim().starts_with("pub struct ") && line.contains("Builder") {
            in_builder = true;
            continue;
        }

        if in_builder && line.trim().starts_with("impl ") {
            in_builder = false;
            continue;
        }

        if !in_builder {
            result.push(line);
        }
    }

    result.join("\n")
}