//! 批量API改进工具
//! 
//! 此工具基于workplace模块的成功经验，批量为其他核心模块应用StandardResponse改进

use std::fs;
use std::path::{Path, PathBuf};

/// 批量API改进器
pub struct BatchApiImprover {
    /// 源代码根目录
    source_root: PathBuf,
    /// 改进统计
    stats: ImprovementStats,
}

/// 改进统计信息
#[derive(Debug, Default)]
pub struct ImprovementStats {
    pub files_processed: u32,
    pub methods_improved: u32,
    pub imports_added: u32,
    pub files_skipped: u32,
    pub errors: Vec<String>,
}

impl BatchApiImprover {
    pub fn new<P: AsRef<Path>>(source_root: P) -> Self {
        Self {
            source_root: source_root.as_ref().to_path_buf(),
            stats: ImprovementStats::default(),
        }
    }

    /// 批量改进指定模块
    pub fn improve_modules(&mut self, module_names: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
        println!("🚀 开始批量API改进...");
        
        for module_name in module_names {
            println!("📁 处理模块: {}", module_name);
            self.improve_module(module_name)?;
        }
        
        self.print_summary();
        Ok(())
    }

    /// 改进单个模块
    fn improve_module(&mut self, module_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let module_path = self.source_root.join("service").join(module_name);
        
        if !module_path.exists() {
            self.stats.errors.push(format!("模块路径不存在: {:?}", module_path));
            return Ok(());
        }

        self.process_directory(&module_path)?;
        Ok(())
    }

    /// 递归处理目录
    fn process_directory(&mut self, dir_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        for entry in fs::read_dir(dir_path)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                self.process_directory(&path)?;
            } else if path.extension().map_or(false, |ext| ext == "rs") {
                self.process_rust_file(&path)?;
            }
        }
        Ok(())
    }

    /// 处理单个Rust文件
    fn process_rust_file(&mut self, file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let content = fs::read_to_string(file_path)?;
        
        // 跳过不需要改进的文件
        if self.should_skip_file(&content) {
            self.stats.files_skipped += 1;
            return Ok(());
        }

        let mut modified_content = content.clone();
        let mut file_changed = false;

        // 1. 添加StandardResponse导入（如果需要）
        if self.needs_standard_response_import(&content) {
            modified_content = self.add_standard_response_import(modified_content);
            self.stats.imports_added += 1;
            file_changed = true;
        }

        // 2. 改进API方法
        let method_improvements = self.improve_api_methods(&modified_content);
        if !method_improvements.is_empty() {
            for improvement in method_improvements {
                modified_content = self.apply_method_improvement(modified_content, &improvement);
                self.stats.methods_improved += 1;
                file_changed = true;
            }
        }

        // 3. 写回文件（如果有改动）
        if file_changed {
            fs::write(file_path, modified_content)?;
            self.stats.files_processed += 1;
            println!("   ✅ 改进文件: {:?}", file_path.file_name().unwrap());
        }

        Ok(())
    }

    /// 检查是否应该跳过此文件
    fn should_skip_file(&self, content: &str) -> bool {
        // 跳过已经使用StandardResponse的文件
        content.contains("api_resp.into_result()") ||
        // 跳过不包含API方法的文件
        !content.contains("pub async fn") ||
        // 跳过不使用Transport::request的文件
        !content.contains("Transport::request")
    }

    /// 检查是否需要添加StandardResponse导入
    fn needs_standard_response_import(&self, content: &str) -> bool {
        content.contains("Transport::request") && 
        !content.contains("standard_response::StandardResponse") &&
        content.contains("pub async fn")
    }

    /// 添加StandardResponse导入
    fn add_standard_response_import(&self, content: String) -> String {
        // 检查是否已经有standard_response导入
        if content.contains("standard_response::StandardResponse") {
            return content;
        }

        // 查找use crate::core块并添加standard_response导入
        if let Some(core_start) = content.find("use crate::core::{") {
            // 找到import块的结束位置
            let mut brace_count = 0;
            let mut import_end = core_start + 18; // 跳过 "use crate::core::{"
            let chars: Vec<char> = content.chars().collect();
            
            for i in import_end..chars.len() {
                match chars[i] {
                    '{' => brace_count += 1,
                    '}' => {
                        if brace_count == 0 {
                            import_end = i;
                            break;
                        } else {
                            brace_count -= 1;
                        }
                    },
                    _ => {}
                }
            }
            
            // 提取现有的导入内容
            let import_content = &content[core_start + 18..import_end];
            if !import_content.contains("standard_response::StandardResponse") {
                let new_import_content = format!("{},\n    standard_response::StandardResponse", import_content.trim_end_matches(',').trim());
                let replacement = format!("use crate::core::{{\n    {}\n}}", new_import_content);
                
                let mut result = content.clone();
                result.replace_range(core_start..import_end + 2, &replacement); // +2 for "};"
                return result;
            }
        }
        
        // 如果没有找到标准的导入格式，返回原内容
        content
    }

    /// 识别需要改进的API方法
    fn improve_api_methods(&self, content: &str) -> Vec<MethodImprovement> {
        let mut improvements = Vec::new();
        
        // 查找所有包含BaseResponse的方法签名
        let lines: Vec<&str> = content.lines().collect();
        for (i, line) in lines.iter().enumerate() {
            if line.contains("pub async fn") && line.contains("-> SDKResult<BaseResponse<") {
                // 提取方法名
                if let Some(fn_start) = line.find("pub async fn ") {
                    if let Some(fn_name_start) = line[fn_start + 13..].find(|c: char| c.is_alphabetic()) {
                        let abs_start = fn_start + 13 + fn_name_start;
                        if let Some(paren) = line[abs_start..].find('(') {
                            let method_name = &line[abs_start..abs_start + paren];
                            
                            // 提取响应类型
                            if let Some(response_start) = line.find("-> SDKResult<BaseResponse<") {
                                let type_start = response_start + 26; // "-> SDKResult<BaseResponse<"的长度
                                if let Some(type_end) = line[type_start..].find(">>") {
                                    let response_type = &line[type_start..type_start + type_end];
                                    
                                    improvements.push(MethodImprovement {
                                        method_name: method_name.to_string(),
                                        response_type: response_type.to_string(),
                                        old_return_type: format!("SDKResult<BaseResponse<{}>>", response_type),
                                        new_return_type: format!("SDKResult<{}>", response_type),
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
        
        improvements
    }

    /// 应用方法改进
    fn apply_method_improvement(&self, content: String, improvement: &MethodImprovement) -> String {
        let mut result = content;
        
        // 1. 替换返回类型
        let old_signature = format!("-> SDKResult<BaseResponse<{}>>", improvement.response_type);
        let new_signature = format!("-> SDKResult<{}>", improvement.response_type);
        result = result.replace(&old_signature, &new_signature);

        // 2. 查找并替换Transport::request调用
        if result.contains("Transport::request") && !result.contains("api_resp.into_result()") {
            // 简化的方法：查找Transport::request行并替换
            let lines: Vec<&str> = result.lines().collect();
            let mut modified_lines = Vec::new();
            let mut in_target_method = false;
            
            for line in lines {
                if line.contains(&format!("pub async fn {}", improvement.method_name)) {
                    in_target_method = true;
                }
                
                if in_target_method && line.contains("Transport::request") && !line.contains("api_resp.into_result()") {
                    // 替换Transport::request调用
                    let indent = line.len() - line.trim_start().len();
                    let spaces = " ".repeat(indent);
                    modified_lines.push(format!("{}let api_resp: BaseResponse<{}> = ", spaces, improvement.response_type));
                    modified_lines.push(format!("{}    Transport::request(api_req, &self.config, option).await?;", spaces));
                    modified_lines.push(format!("{}api_resp.into_result()", spaces));
                } else {
                    modified_lines.push(line.to_string());
                }
                
                if in_target_method && line.trim() == "}" {
                    in_target_method = false;
                }
            }
            
            result = modified_lines.join("\n");
        }

        result
    }

    /// 打印改进摘要
    fn print_summary(&self) {
        println!("\n📊 批量改进摘要:");
        println!("   文件处理数: {}", self.stats.files_processed);
        println!("   方法改进数: {}", self.stats.methods_improved);  
        println!("   导入添加数: {}", self.stats.imports_added);
        println!("   文件跳过数: {}", self.stats.files_skipped);
        
        if !self.stats.errors.is_empty() {
            println!("   错误数: {}", self.stats.errors.len());
            for error in &self.stats.errors {
                println!("     ❌ {}", error);
            }
        }
    }
}

/// 方法改进信息
#[derive(Debug)]
struct MethodImprovement {
    method_name: String,
    response_type: String,
    old_return_type: String,
    new_return_type: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut improver = BatchApiImprover::new("src");
    
    // 定义要改进的核心模块（按优先级）
    let priority_modules = vec![
        "contact",      // 通讯录服务 - 高使用率
        "im",          // 即时消息服务 - 核心功能
        "authentication", // 认证服务 - 基础服务
        "search",       // 搜索服务 - 常用功能
    ];
    
    println!("🎯 批量改进目标模块: {:?}", priority_modules);
    println!("📋 改进内容:");
    println!("   - 添加StandardResponse导入");
    println!("   - 修改返回类型：SDKResult<BaseResponse<T>> → SDKResult<T>");
    println!("   - 添加.into_result()调用");
    println!();
    
    improver.improve_modules(&priority_modules)?;
    
    println!("\n✅ 批量改进完成！");
    println!("💡 建议：运行 cargo check 验证改进结果");
    
    Ok(())
}