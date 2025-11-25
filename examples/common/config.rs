//! 配置管理模块
//!
//! 提供统一的环境变量配置加载、验证和管理功能。
//! 支持 .env 文件加载，并提供详细的错误诊断。

use crate::utils::print_info;
use openlark_client::Client;
use openlark_client::Error;
use std::env;
use std::path::Path;

/// 配置加载结果
#[derive(Debug, Clone)]
pub struct ConfigLoadResult {
    /// 是否成功从 .env 文件加载配置
    pub env_loaded: bool,
    /// .env 文件路径（如果存在）
    pub env_file_path: Option<String>,
    /// 配置完整性检查结果
    pub config_complete: bool,
    /// 缺失的环境变量
    pub missing_vars: Vec<String>,
}

impl ConfigLoadResult {
    /// 创建成功的配置加载结果
    pub fn success(env_file_path: Option<String>) -> Self {
        Self {
            env_loaded: env_file_path.is_some(),
            env_file_path,
            config_complete: true,
            missing_vars: Vec::new(),
        }
    }

    /// 创建部分成功的配置加载结果
    pub fn partial(env_file_path: Option<String>, missing_vars: Vec<String>) -> Self {
        Self {
            env_loaded: env_file_path.is_some(),
            env_file_path,
            config_complete: missing_vars.is_empty(),
            missing_vars,
        }
    }

    /// 是否完全成功
    pub fn is_success(&self) -> bool {
        self.config_complete
    }

    /// 打印配置加载结果
    pub fn print_result(&self) {
        if let Some(env_path) = &self.env_file_path {
            print_info(&format!("✅ 已加载配置文件: {}", env_path));
        } else {
            print_info("ℹ️  未找到 .env 文件，使用系统环境变量");
        }

        if self.config_complete {
            print_info("✅ 配置完整性检查通过");
        } else {
            print_info(&format!("⚠️  缺失配置: {}", self.missing_vars.join(", ")));
        }
    }
}

/// 配置诊断信息
#[derive(Debug)]
pub struct ConfigDiagnostics {
    /// 配置状态
    pub status: ConfigStatus,
    /// 详细的错误信息和建议
    pub details: Vec<String>,
    /// 推荐的修复步骤
    pub recommendations: Vec<String>,
}

/// 配置状态
#[derive(Debug, PartialEq)]
pub enum ConfigStatus {
    /// 配置完整且有效
    Complete,
    /// 配置部分缺失但可运行
    Partial,
    /// 配置关键缺失无法运行
    Critical,
    /// 配置验证失败
    Invalid,
}

impl ConfigDiagnostics {
    /// 创建成功诊断
    pub fn success() -> Self {
        Self {
            status: ConfigStatus::Complete,
            details: vec!["所有必需的环境变量已正确配置".to_string()],
            recommendations: Vec::new(),
        }
    }

    /// 创建部分缺失诊断
    pub fn partial(missing_vars: Vec<String>) -> Self {
        let details = vec![
            format!("发现 {} 个缺失的环境变量", missing_vars.len()),
            "某些功能可能无法正常使用".to_string(),
        ];

        let recommendations = vec![
            "创建 .env 文件并添加缺失的环境变量".to_string(),
            "或设置系统环境变量".to_string(),
            format!("缺失变量: {}", missing_vars.join(", ")),
        ];

        Self {
            status: ConfigStatus::Partial,
            details,
            recommendations,
        }
    }

    /// 创建关键错误诊断
    pub fn critical(missing_vars: Vec<String>) -> Self {
        let details = vec![
            "关键环境变量缺失，无法正常运行示例".to_string(),
            format!("缺失变量: {}", missing_vars.join(", ")),
        ];

        let recommendations = vec![
            "立即设置以下环境变量:".to_string(),
            format!("export OPENLARK_APP_ID=\"your_app_id\""),
            format!("export OPENLARK_APP_SECRET=\"your_app_secret\""),
            "或创建包含这些变量的 .env 文件".to_string(),
        ];

        Self {
            status: ConfigStatus::Critical,
            details,
            recommendations,
        }
    }

    /// 打印诊断结果
    pub fn print_diagnostics(&self) {
        match self.status {
            ConfigStatus::Complete => {
                crate::utils::print_success("配置诊断: 完整有效");
            }
            ConfigStatus::Partial => {
                crate::utils::print_warning("配置诊断: 部分缺失");
            }
            ConfigStatus::Critical => {
                crate::utils::print_error("配置诊断: 关键缺失");
            }
            ConfigStatus::Invalid => {
                crate::utils::print_error("配置诊断: 验证失败");
            }
        }

        // 打印详细信息
        for detail in &self.details {
            println!("  📝 {}", detail);
        }

        // 打印推荐修复步骤
        if !self.recommendations.is_empty() {
            println!();
            println!("💡 推荐修复步骤:");
            for (i, recommendation) in self.recommendations.iter().enumerate() {
                println!("  {}. {}", i + 1, recommendation);
            }
        }
    }
}

/// 加载配置并自动检测 .env 文件
///
/// # 参数
/// - `env_search_dirs`: 搜索 .env 文件的目录列表
///
/// # 返回值
/// - `ConfigLoadResult`: 配置加载结果
///
/// # 示例
/// ```rust
/// let result = load_config_with_diagnostics(&["examples", "."]);
/// result.print_result();
/// ```
pub fn load_config_with_diagnostics(env_search_dirs: &[&str]) -> ConfigLoadResult {
    let mut env_loaded = false;
    let mut env_file_path = None;

    // 尝试从搜索目录加载 .env 文件
    for dir in env_search_dirs {
        let env_path = format!("{}/.env", dir);
        if Path::new(&env_path).exists() {
            match dotenvy::from_filename(&env_path) {
                Ok(_) => {
                    env_loaded = true;
                    env_file_path = Some(env_path);
                    break;
                }
                Err(e) => {
                    crate::utils::print_error(&format!("加载 .env 文件失败: {}", e));
                    // 尝试下一个目录
                    continue;
                }
            }
        }
    }

    // 检查必需的环境变量
    let required_vars = ["OPENLARK_APP_ID", "OPENLARK_APP_SECRET"];
    let mut missing_vars = Vec::new();

    for var in &required_vars {
        if env::var(var).is_err() {
            missing_vars.push(var.to_string());
        }
    }

    // 检查可选的环境变量
    let optional_vars = ["OPENLARK_BASE_URL"];
    for var in &optional_vars {
        if env::var(var).is_ok() {
            crate::utils::print_info(&format!("✅ 检测到可选配置: {}", var));
        }
    }

    if missing_vars.is_empty() {
        ConfigLoadResult::success(env_file_path)
    } else {
        ConfigLoadResult::partial(env_file_path, missing_vars)
    }
}

/// 执行完整的配置诊断
///
/// # 参数
/// - `strict_mode`: 是否启用严格模式（要求所有必需变量存在）
///
/// # 返回值
/// - `ConfigDiagnostics`: 详细的诊断信息
pub fn run_config_diagnostics(strict_mode: bool) -> ConfigDiagnostics {
    let required_vars = ["OPENLARK_APP_ID", "OPENLARK_APP_SECRET"];
    let mut missing_vars = Vec::new();
    let mut critical_missing = Vec::new();

    for var in &required_vars {
        match env::var(var) {
            Ok(value) if value.trim().is_empty() => {
                missing_vars.push(format!("{} (值为空)", var));
                if strict_mode {
                    critical_missing.push(var.to_string());
                }
            }
            Ok(_) => {
                // 变量存在且不为空
            }
            Err(_) => {
                missing_vars.push(var.to_string());
                if strict_mode || *var != "OPENLARK_BASE_URL" {
                    critical_missing.push(var.to_string());
                }
            }
        }
    }

    // 验证配置值的有效性
    if let Ok(app_id) = env::var("OPENLARK_APP_ID") {
        if app_id.len() < 8 {
            missing_vars.push("OPENLARK_APP_ID (长度可能不足)".to_string());
            if strict_mode {
                critical_missing.push("OPENLARK_APP_ID".to_string());
            }
        }
    }

    if critical_missing.is_empty() && missing_vars.is_empty() {
        ConfigDiagnostics::success()
    } else if critical_missing.is_empty() {
        ConfigDiagnostics::partial(missing_vars)
    } else {
        ConfigDiagnostics::critical(critical_missing)
    }
}

/// 创建并配置客户端（增强版本）
///
/// 自动加载配置、验证完整性、提供详细错误信息
///
/// # 参数
/// - `env_search_dirs`: 搜索 .env 文件的目录列表
///
/// # 返回值
/// - `Result<Client, ConfigError>`: 客户端实例或配置错误
///
/// # 示例
/// ```rust
/// let client = create_client_with_config(&["examples", "."])?;
/// println!("客户端创建成功");
/// ```
pub fn create_client_with_config(env_search_dirs: &[&str]) -> Result<Client, ConfigError> {
    // 加载配置
    let load_result = load_config_with_diagnostics(env_search_dirs);
    load_result.print_result();

    // 运行诊断
    let diagnostics = run_config_diagnostics(true);

    match diagnostics.status {
        ConfigStatus::Complete => {
            // 配置完整，创建客户端
            Client::from_env().map_err(|e| ConfigError::ClientCreationFailed(e.to_string()))
        }
        ConfigStatus::Partial => {
            // 配置部分缺失，询问是否继续
            println!();
            crate::utils::print_warning("配置不完整，某些功能可能受限");
            if crate::utils::wait_for_confirmation("是否继续创建客户端？") {
                Client::from_env().map_err(|e| ConfigError::ClientCreationFailed(e.to_string()))
            } else {
                Err(ConfigError::IncompleteConfiguration)
            }
        }
        ConfigStatus::Critical => {
            // 关键配置缺失
            diagnostics.print_diagnostics();
            Err(ConfigError::CriticalMissing)
        }
        ConfigStatus::Invalid => {
            diagnostics.print_diagnostics();
            Err(ConfigError::InvalidConfiguration)
        }
    }
}

/// 配置错误类型
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("客户端创建失败: {0}")]
    ClientCreationFailed(String),

    #[error("配置不完整，无法继续")]
    IncompleteConfiguration,

    #[error("关键环境变量缺失")]
    CriticalMissing,

    #[error("配置验证失败")]
    InvalidConfiguration,

    #[error(".env 文件加载失败: {0}")]
    EnvFileLoadFailed(String),

    #[error("IO 错误: {0}")]
    IoError(#[from] std::io::Error),
}

/// 生成 .env 文件模板
///
/// # 参数
/// - `output_path`: 输出文件路径
/// - `include_comments`: 是否包含详细注释
///
/// # 示例
/// ```rust
/// generate_env_template("examples/.env.example", true)?;
/// ```
pub fn generate_env_template(output_path: &str, include_comments: bool) -> Result<(), ConfigError> {
    use std::fs::File;
    use std::io::Write;

    let mut file = File::create(output_path)?;

    if include_comments {
        writeln!(file, "# Open-Lark SDK 示例配置文件")?;
        writeln!(file, "#")?;
        writeln!(file, "# 请将此文件重命名为 .env 并填入您的真实配置信息")?;
        writeln!(
            file,
            "# 注意：请勿将包含敏感信息的 .env 文件提交到版本控制系统"
        )?;
        writeln!(file, "")?;
        writeln!(file, "# 飞书应用配置")?;
    }

    writeln!(file, "OPENLARK_APP_ID=\"your_app_id_here\"")?;
    writeln!(file, "OPENLARK_APP_SECRET=\"your_app_secret_here\"")?;

    if include_comments {
        writeln!(file, "")?;
        writeln!(file, "# 可选配置")?;
        writeln!(file, "# OPENLARK_BASE_URL=\"https://open.feishu.cn\"")?;
        writeln!(file, "# OPENLARK_TIMEOUT=30")?;
        writeln!(file, "# OPENLARK_RETRY_COUNT=3")?;
    }

    Ok(())
}

/// 验证 .env 文件格式
///
/// # 参数
/// - `env_path`: .env 文件路径
///
/// # 返回值
/// - `Result<(), Vec<String>>`: 成功或错误列表
pub fn validate_env_file(env_path: &str) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();

    // 检查文件是否存在
    if !Path::new(env_path).exists() {
        errors.push(format!("文件不存在: {}", env_path));
        return Err(errors);
    }

    // 尝试加载文件
    if let Err(e) = dotenvy::from_filename(env_path) {
        errors.push(format!("文件格式错误: {}", e));
        return Err(errors);
    }

    // 验证关键变量
    let original_vars = std::env::vars().collect::<std::collections::HashMap<_, _>>();

    // 加载后再验证
    dotenvy::from_filename(env_path).ok(); // 重新加载

    let required_vars = ["OPENLARK_APP_ID", "OPENLARK_APP_SECRET"];
    for var in &required_vars {
        if std::env::var(var).is_err() {
            errors.push(format!("缺失必需变量: {}", var));
        }
    }

    // 恢复原始环境变量（仅验证，不修改实际配置）
    for (key, value) in original_vars {
        if std::env::var(&key).is_err() {
            std::env::set_var(&key, value);
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
