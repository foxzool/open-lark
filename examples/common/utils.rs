//! 通用工具函数
//!
//! 提供示例程序常用的工具函数，包括日志输出、结果处理等。

use colored::*;
use openlark_client::Error;
use serde_json;

/// 打印成功信息
///
/// # 参数
/// - `message`: 成功信息
pub fn print_success(message: &str) {
    println!("{} {}", "✅".green(), message.green());
}

/// 打印错误信息
///
/// # 参数
/// - `message`: 错误信息
pub fn print_error(message: &str) {
    println!("{} {}", "❌".red(), message.red());
}

/// 打印信息提示
///
/// # 参数
/// - `message`: 提示信息
pub fn print_info(message: &str) {
    println!("{} {}", "ℹ️".blue(), message.blue());
}

/// 打印警告信息
///
/// # 参数
/// - `message`: 警告信息
pub fn print_warning(message: &str) {
    println!("{} {}", "⚠️".yellow(), message.yellow());
}

/// 处理SDK结果
///
/// 统一处理SDK调用的结果，提供标准化的输出格式
///
/// # 参数
/// - `result`: SDK调用的结果
/// - `success_msg`: 成功时的消息
/// - `error_msg`: 失败时的消息前缀
///
/// # 返回值
/// - `Ok(T)`: 成功时返回结果
/// - `Err(Error)`: 失败时返回错误
pub fn handle_result<T>(
    result: std::result::Result<T, Error>,
    success_msg: &str,
    error_msg: &str,
) -> std::result::Result<T, Error> {
    match result {
        Ok(data) => {
            print_success(success_msg);
            Ok(data)
        }
        Err(e) => {
            print_error(&format!("{}: {}", error_msg, e));
            Err(e)
        }
    }
}

/// 格式化JSON输出
///
/// 将数据结构格式化为美观的JSON输出
///
/// # 参数
/// - `data`: 要格式化的数据
/// - `title`: 输出标题
///
/// # 返回值
/// - `Ok(())`: 格式化成功
/// - `Err(serde_json::Error)`: 序列化失败
pub fn print_json<T: serde::Serialize>(data: &T, title: &str) -> Result<(), serde_json::Error> {
    println!("{}", title.bright_cyan().underline());

    match serde_json::to_string_pretty(data) {
        Ok(json) => {
            println!("{}", json.green());
            Ok(())
        }
        Err(e) => {
            print_error(&format!("JSON格式化失败: {}", e));
            Err(e)
        }
    }
}

/// 打印分隔线
///
/// # 参数
/// - `title`: 分隔线标题（可选）
pub fn print_separator(title: Option<&str>) {
    println!();
    if let Some(t) = title {
        println!("{} {}", "═".repeat(20).bright_black(), t.bright_yellow());
    } else {
        println!("{}", "═".repeat(50).bright_black());
    }
    println!();
}

/// 打印步骤信息
///
/// # 参数
/// - `step`: 步骤编号
/// - `description`: 步骤描述
pub fn print_step(step: usize, description: &str) {
    println!("{} {}: {}", "📍".blue(), step, description.bright_white());
}

/// 等待用户确认
///
/// # 参数
/// - `message`: 确认信息
///
/// # 返回值
/// - `bool`: 用户确认返回true，否则返回false
pub fn wait_for_confirmation(message: &str) -> bool {
    print!("{} {} (y/N): ", "❓".yellow(), message);

    use std::io::{self, Write};
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("读取输入失败");

    let input = input.trim().to_lowercase();
    input == "y" || input == "yes"
}

/// 模拟延迟操作
///
/// 在示例中用于模拟网络请求等操作
///
/// # 参数
/// - `seconds`: 延迟秒数
/// - `message`: 延迟期间显示的消息
pub async fn simulate_delay(seconds: u64, message: &str) {
    if !message.is_empty() {
        print_info(message);
    }

    for i in 1..=seconds {
        print!("等待中... {}/{} 秒\r", i, seconds);
        use std::io::{self, Write};
        io::stdout().flush().unwrap();
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
    println!(); // 换行
}

/// 格式化文件大小
///
/// # 参数
/// - `bytes`: 字节数
///
/// # 返回值
/// - 格式化后的文件大小字符串
pub fn format_file_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    format!("{:.2} {}", size, UNITS[unit_index])
}

/// 验证环境变量
///
/// 检查必需的环境变量是否已设置
///
/// # 参数
/// - `var_names`: 环境变量名称列表
///
/// # 返回值
/// - `Ok(())`: 所有环境变量都已设置
/// - `Err(String)`: 缺失的环境变量名称
pub fn check_env_vars(var_names: &[&str]) -> Result<(), String> {
    for &var_name in var_names {
        if std::env::var(var_name).is_err() {
            return Err(format!("环境变量 {} 未设置", var_name));
        }
    }
    Ok(())
}

/// 增强版环境变量检查
///
/// 支持 dotenvy 文件检测和详细诊断
///
/// # 参数
/// - `var_names`: 必需的环境变量名称列表
/// - `env_search_dirs`: 搜索 .env 文件的目录列表
/// - `auto_load`: 是否自动尝试加载 .env 文件
///
/// # 返回值
/// - `Ok(EnvCheckResult)`: 环境变量检查结果
/// - `Err(String)`: 严重错误信息
pub fn check_env_vars_enhanced(
    var_names: &[&str],
    env_search_dirs: &[&str],
    auto_load: bool,
) -> Result<EnvCheckResult, String> {
    use std::path::Path;
    let mut env_file_found = false;
    let mut env_file_loaded = false;
    let mut missing_vars = Vec::new();
    let mut empty_vars = Vec::new();

    // 检查 .env 文件是否存在
    if auto_load {
        for dir in env_search_dirs {
            let env_path = format!("{}/.env", dir);
            if Path::new(&env_path).exists() {
                env_file_found = true;
                print_info(&format!("发现 .env 文件: {}", env_path));

                match dotenvy::from_filename(&env_path) {
                    Ok(_) => {
                        env_file_loaded = true;
                        print_success("✅ .env 文件加载成功");
                        break;
                    }
                    Err(e) => {
                        print_error(&format!("❌ .env 文件加载失败: {}", e));
                    }
                }
            }
        }

        if !env_file_found {
            print_info("ℹ️  未找到 .env 文件，使用系统环境变量");
        }
    }

    // 检查环境变量
    for &var_name in var_names {
        match std::env::var(var_name) {
            Ok(value) if value.trim().is_empty() => {
                empty_vars.push(var_name.to_string());
            }
            Ok(_) => {
                // 变量存在且不为空
            }
            Err(_) => {
                missing_vars.push(var_name.to_string());
            }
        }
    }

    let result = EnvCheckResult {
        env_file_found,
        env_file_loaded,
        total_required: var_names.len(),
        missing_vars,
        empty_vars,
        present_vars: var_names
            .iter()
            .filter(|&&var| std::env::var(var).is_ok())
            .map(|&var| var.to_string())
            .collect(),
    };

    if result.is_complete() {
        print_success("✅ 所有必需的环境变量已正确设置");
    } else {
        print_warning(&format!(
            "⚠️  环境变量检查不完整: {} 个缺失, {} 个为空",
            result.missing_vars.len(),
            result.empty_vars.len()
        ));
    }

    Ok(result)
}

/// 环境变量检查结果
#[derive(Debug, Clone)]
pub struct EnvCheckResult {
    /// 是否找到了 .env 文件
    pub env_file_found: bool,
    /// 是否成功加载了 .env 文件
    pub env_file_loaded: bool,
    /// 总共需要检查的环境变量数量
    pub total_required: usize,
    /// 缺失的环境变量
    pub missing_vars: Vec<String>,
    /// 值为空的环境变量
    pub empty_vars: Vec<String>,
    /// 存在的环境变量
    pub present_vars: Vec<String>,
}

impl EnvCheckResult {
    /// 检查是否所有变量都存在且不为空
    pub fn is_complete(&self) -> bool {
        self.missing_vars.is_empty() && self.empty_vars.is_empty()
    }

    /// 获取所有问题的变量列表
    pub fn get_problematic_vars(&self) -> Vec<String> {
        let mut problems = self.missing_vars.clone();
        problems.extend(self.empty_vars.clone());
        problems
    }

    /// 打印详细的检查结果
    pub fn print_detailed_result(&self) {
        println!();
        println!("📊 环境变量检查详细结果:");
        println!(
            "  📁 .env 文件: {} {}",
            if self.env_file_found { "✅" } else { "❌" },
            if self.env_file_found {
                "找到"
            } else {
                "未找到"
            }
        );

        if self.env_file_found {
            println!(
                "  📖 .env 加载: {} {}",
                if self.env_file_loaded { "✅" } else { "❌" },
                if self.env_file_loaded {
                    "成功"
                } else {
                    "失败"
                }
            );
        }

        println!(
            "  📋 检查进度: {}/{} 已配置",
            self.present_vars.len(),
            self.total_required
        );

        if !self.present_vars.is_empty() {
            println!("  ✅ 已配置: {}", self.present_vars.join(", "));
        }

        if !self.missing_vars.is_empty() {
            println!("  ❌ 缺失: {}", self.missing_vars.join(", "));
        }

        if !self.empty_vars.is_empty() {
            println!("  ⚠️  为空: {}", self.empty_vars.join(", "));
        }

        println!();
    }

    /// 生成修复建议
    pub fn generate_fix_suggestions(&self) -> Vec<String> {
        let mut suggestions = Vec::new();

        if self.missing_vars.is_empty() && self.empty_vars.is_empty() {
            suggestions.push("✅ 配置完美，无需修复".to_string());
            return suggestions;
        }

        if !self.env_file_found {
            suggestions.push("创建 .env 文件并添加以下内容:".to_string());
            for var in &self.get_problematic_vars() {
                suggestions.push(format!(
                    "{}=your_{}_here",
                    var,
                    var.to_lowercase().replace("openlark_", "")
                ));
            }
        } else {
            suggestions.push("在现有的 .env 文件中添加以下变量:".to_string());
            for var in &self.get_problematic_vars() {
                suggestions.push(format!(
                    "{}=your_{}_here",
                    var,
                    var.to_lowercase().replace("openlark_", "")
                ));
            }
        }

        if !self.missing_vars.is_empty() {
            suggestions.push("或者设置系统环境变量:".to_string());
            for var in &self.missing_vars {
                suggestions.push(format!(
                    "export {}=\"your_{}_here\"",
                    var,
                    var.to_lowercase().replace("openlark_", "")
                ));
            }
        }

        suggestions
    }
}

/// 示例程序的标准开头
///
/// 打印示例程序的标准头部信息
///
/// # 参数
/// - `title`: 示例标题
/// - `description`: 示例描述
pub fn print_example_header(title: &str, description: &str) {
    print_separator(None);
    println!("{} {}", "🚀".bright_green(), title.bright_green().bold());
    println!("{}", description.bright_black());
    print_separator(None);
}

/// 示例程序的标准结尾
///
/// 打印示例程序的标准尾部信息
///
/// # 参数
/// - `next_steps`: 下一步建议（可选）
pub fn print_example_footer(next_steps: Option<&str>) {
    print_separator(None);
    print_success("示例执行完成！");

    if let Some(steps) = next_steps {
        println!("{} {}", "💡".bright_blue(), steps.bright_blue());
    }

    print_separator(None);
}
