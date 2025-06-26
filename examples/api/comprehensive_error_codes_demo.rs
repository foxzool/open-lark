/// 飞书SDK扩展错误码系统演示
///
/// 展示如何使用新增的业务错误码和分类系统
use std::collections::HashMap;

use open_lark::core::{error::ErrorSeverity, error_codes::LarkErrorCode};

fn main() {
    println!("🏷️ 飞书SDK扩展错误码系统演示\n");

    // 演示各种业务错误码
    demonstrate_business_error_codes();
    println!();

    // 演示错误分类系统
    demonstrate_error_categorization();
    println!();

    // 演示错误处理策略
    demonstrate_error_handling_strategies();
    println!();

    // 演示错误码统计分析
    demonstrate_error_analysis();
}

/// 演示业务错误码
fn demonstrate_business_error_codes() {
    println!("📋 业务错误码演示:");

    let business_errors = vec![
        (60001, "用户管理模块"),
        (70001, "群组管理模块"),
        (80001, "消息管理模块"),
        (90001, "文件管理模块"),
        (110001, "日历管理模块"),
        (120001, "云文档模块"),
        (130001, "应用商店模块"),
        (999001, "网络连接模块"),
    ];

    for (code, module) in business_errors {
        if let Some(error_code) = LarkErrorCode::from_code(code) {
            println!(
                "   {} - [{}] {} - {}",
                module,
                code,
                error_code.description(),
                error_code.detailed_description()
            );
        }
    }
}

/// 演示错误分类系统
fn demonstrate_error_categorization() {
    println!("🗂️ 错误分类系统演示:");

    // 按类别分组的错误码
    let mut categories = HashMap::new();
    let all_errors = vec![
        LarkErrorCode::AccessTokenInvalid,
        LarkErrorCode::AppPermissionDenied,
        LarkErrorCode::BadRequest,
        LarkErrorCode::UserNotFound,
        LarkErrorCode::TooManyRequests,
        LarkErrorCode::InternalServerError,
        LarkErrorCode::NetworkTimeout,
        LarkErrorCode::DocumentPermissionDenied,
        LarkErrorCode::FileSizeExceeded,
        LarkErrorCode::EventConflict,
        LarkErrorCode::DnsResolutionFailed,
    ];

    // 按类别分组
    for error in all_errors {
        let category = error.category();
        categories
            .entry(category)
            .or_insert_with(Vec::new)
            .push(error);
    }

    // 显示每个类别
    for (category, errors) in categories {
        println!("   📁 {:?}:", category);
        for error in errors {
            let icon = if error.is_retryable() { "🔄" } else { "❌" };
            println!("      {} {} - {}", icon, error, error.description());
        }
        println!();
    }
}

/// 演示错误处理策略
fn demonstrate_error_handling_strategies() {
    println!("🛠️ 错误处理策略演示:");

    let test_errors = vec![
        LarkErrorCode::TooManyRequests,
        LarkErrorCode::NetworkTimeout,
        LarkErrorCode::UserNotFound,
        LarkErrorCode::DocumentPermissionDenied,
        LarkErrorCode::InternalServerError,
    ];

    for error in test_errors {
        println!("   错误: {} - {}", error, error.description());

        // 分析错误特性
        let mut strategies = Vec::new();

        if error.is_retryable() {
            if let Some(delay) = error.suggested_retry_delay() {
                strategies.push(format!("🔄 可重试 (建议延迟{}秒)", delay));
            } else {
                strategies.push("🔄 可重试".to_string());
            }
        }

        if error.is_auth_error() {
            strategies.push("🔐 刷新认证令牌".to_string());
        }

        if error.is_permission_error() {
            strategies.push("🚫 检查权限配置".to_string());
        }

        if error.is_client_error() {
            strategies.push("📝 检查请求参数".to_string());
        }

        if error.is_server_error() {
            strategies.push("🔧 服务器问题，稍后重试".to_string());
        }

        // 显示处理策略
        for strategy in strategies {
            println!("      → {}", strategy);
        }

        // 显示帮助链接
        if let Some(help_url) = error.help_url() {
            println!("      📚 帮助文档: {}", help_url);
        }

        // 显示严重级别
        let severity = error.severity();
        println!(
            "      {} 严重级别: {:?}",
            match severity {
                ErrorSeverity::Info => "ℹ️",
                ErrorSeverity::Warning => "⚠️",
                ErrorSeverity::Error => "❌",
                ErrorSeverity::Critical => "🚨",
            },
            severity
        );

        println!();
    }
}

/// 演示错误分析功能
fn demonstrate_error_analysis() {
    println!("📊 错误码统计分析:");

    // 模拟一批错误
    let error_codes = vec![
        403, 403, 500, 429, 60001, 70001, 999001, 999001, 999001, 120001, 90002, 110003, 130001,
        400, 404, 502,
    ];

    // 统计分析
    let mut category_stats = HashMap::new();
    let mut retryable_count = 0;
    let mut severity_stats = HashMap::new();

    println!("   处理 {} 个错误码...", error_codes.len());

    for code in &error_codes {
        if let Some(error_code) = LarkErrorCode::from_code(*code) {
            // 统计分类
            let category = error_code.category();
            *category_stats.entry(category).or_insert(0) += 1;

            // 统计可重试
            if error_code.is_retryable() {
                retryable_count += 1;
            }

            // 统计严重级别
            let severity = error_code.severity();
            *severity_stats.entry(severity).or_insert(0) += 1;
        }
    }

    // 显示分类统计
    println!("\n   📈 按类别统计:");
    for (category, count) in category_stats {
        println!("      {:?}: {} 次", category, count);
    }

    // 显示重试统计
    println!("\n   🔄 重试分析:");
    println!(
        "      可重试错误: {} / {} ({:.1}%)",
        retryable_count,
        error_codes.len(),
        (retryable_count as f32 / error_codes.len() as f32) * 100.0
    );

    // 显示严重级别统计
    println!("\n   ⚠️ 严重级别统计:");
    for (severity, count) in severity_stats {
        println!(
            "      {} {:?}: {} 次",
            match severity {
                ErrorSeverity::Info => "ℹ️",
                ErrorSeverity::Warning => "⚠️",
                ErrorSeverity::Error => "❌",
                ErrorSeverity::Critical => "🚨",
            },
            severity,
            count
        );
    }

    println!("\n✅ 错误码系统演示完成！");
}
