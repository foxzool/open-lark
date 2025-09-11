/// 用户身份验证和信息获取示例
///
/// 这个示例演示如何使用飞书SDK获取当前登录用户的详细信息。
///
/// 使用方法：
/// cargo run --example core_refresh_token
///
/// 环境变量：
/// APP_ID=your_app_id
/// APP_SECRET=your_app_secret  
/// USER_ACCESS_TOKEN=your_user_access_token (必需，用于获取用户信息)
use open_lark::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载环境变量
    dotenvy::dotenv().ok();

    let app_id = std::env::var("APP_ID").expect("APP_ID environment variable not set");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET environment variable not set");
    let user_access_token =
        std::env::var("USER_ACCESS_TOKEN").expect("USER_ACCESS_TOKEN environment variable not set");

    // 创建客户端
    let client = LarkClient::builder(&app_id, &app_secret)
        .with_enable_token_cache(true)
        .build();

    println!("🔐 飞书用户身份验证示例");
    println!("{}", "=".repeat(50));

    // 获取当前用户信息
    get_current_user_info(&client, &user_access_token).await?;

    // 演示令牌管理功能
    demonstrate_token_management(&client).await?;

    // 演示用户身份验证流程
    demonstrate_auth_workflow(&client).await?;

    Ok(())
}

/// 获取当前用户信息
async fn get_current_user_info(
    client: &LarkClient,
    user_access_token: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n👤 获取当前用户信息...");

    match client.auth.v1.user_info.get(user_access_token).await {
        Ok(user_info) => {
            println!("✅ 用户信息获取成功!");

            println!("\n📋 用户详细信息:");
            println!("   姓名: {}", user_info.name);
            println!("   英文名: {}", user_info.en_name);
            println!("   员工工号: {}", user_info.employee_no);
            println!("   用户ID: {}", user_info.user_id);
            println!("   OpenID: {}", user_info.open_id);
            println!("   UnionID: {}", user_info.union_id);
            println!("   企业标识: {}", user_info.tenant_key);

            // 联系方式
            println!("\n📞 联系方式:");
            if let Some(email) = &user_info.email {
                println!("   个人邮箱: {email}");
            } else {
                println!("   个人邮箱: 未设置");
            }

            if let Some(enterprise_email) = &user_info.enterprise_email {
                println!("   企业邮箱: {enterprise_email}");
            } else {
                println!("   企业邮箱: 未设置");
            }

            if let Some(mobile) = &user_info.mobile {
                println!("   手机号: {mobile}");
            } else {
                println!("   手机号: 未设置");
            }

            // 头像信息
            println!("\n🖼️ 头像信息:");
            println!("   头像URL: {}", user_info.avatar_url);
            println!("   头像(72x72): {}", user_info.avatar_thumb);
            println!("   头像(240x240): {}", user_info.avatar_middle);
            println!("   头像(640x640): {}", user_info.avatar_big);

            // 验证用户身份信息完整性
            validate_user_info(&user_info).await?;
        }
        Err(e) => {
            println!("❌ 获取用户信息失败: {e:?}");
            println!("\n💡 常见错误解决方案:");
            println!("   1. 检查USER_ACCESS_TOKEN是否有效");
            println!("   2. 确认用户访问令牌未过期");
            println!("   3. 验证应用权限配置");
            println!("   4. 检查网络连接状态");
            return Err(e.into());
        }
    }

    Ok(())
}

/// 验证用户信息完整性
async fn validate_user_info(
    user_info: &open_lark::service::authentication::v1::UserInfo,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 验证用户信息完整性...");

    let mut warnings = Vec::new();
    let mut missing_fields = Vec::new();

    // 检查必填字段
    if user_info.name.is_empty() {
        missing_fields.push("用户姓名");
    }

    if user_info.open_id.is_empty() {
        missing_fields.push("OpenID");
    }

    if user_info.union_id.is_empty() {
        missing_fields.push("UnionID");
    }

    if user_info.user_id.is_empty() {
        missing_fields.push("用户ID");
    }

    if user_info.tenant_key.is_empty() {
        missing_fields.push("企业标识");
    }

    // 检查可选字段
    if user_info.email.is_none() {
        warnings.push("个人邮箱未设置");
    }

    if user_info.enterprise_email.is_none() {
        warnings.push("企业邮箱未设置");
    }

    if user_info.mobile.is_none() {
        warnings.push("手机号未设置");
    }

    // 输出验证结果
    if missing_fields.is_empty() {
        println!("✅ 必填字段验证通过");
    } else {
        println!("❌ 缺少必填字段: {}", missing_fields.join(", "));
    }

    if !warnings.is_empty() {
        println!("⚠️ 注意事项:");
        for warning in warnings {
            println!("   - {warning}");
        }
    }

    // 检查头像URL有效性
    println!("\n🖼️ 头像URL验证:");
    let avatar_urls = vec![
        ("标准头像", &user_info.avatar_url),
        ("缩略图", &user_info.avatar_thumb),
        ("中等尺寸", &user_info.avatar_middle),
        ("大尺寸", &user_info.avatar_big),
    ];

    for (name, url) in avatar_urls {
        if url.starts_with("http") {
            println!("   ✅ {name}: URL格式正确");
        } else {
            println!("   ⚠️ {name}: URL格式异常");
        }
    }

    Ok(())
}

/// 演示令牌管理功能
async fn demonstrate_token_management(
    client: &LarkClient,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔑 令牌管理功能演示...");

    // 获取当前令牌状态
    println!("📊 当前令牌状态:");
    println!("   应用ID: {}", client.config.app_id);
    println!(
        "   令牌缓存: {}",
        if client.config.enable_token_cache {
            "已启用"
        } else {
            "已禁用"
        }
    );

    // 演示令牌类型说明
    println!("\n🔐 令牌类型说明:");
    println!("   1. App Access Token (应用令牌):");
    println!("      - 用于调用大部分开放平台API");
    println!("      - 有效期约2小时，SDK自动刷新");
    println!("      - 基于app_id和app_secret获取");

    println!("   2. Tenant Access Token (企业令牌):");
    println!("      - 用于获取企业信息和管理功能");
    println!("      - 有效期约2小时，SDK自动刷新");
    println!("      - 需要企业管理员授权");

    println!("   3. User Access Token (用户令牌):");
    println!("      - 用于访问用户个人数据");
    println!("      - 需要用户手动授权获取");
    println!("      - 有效期由用户授权范围决定");

    // 演示令牌缓存机制
    println!("\n💾 令牌缓存机制:");
    if client.config.enable_token_cache {
        println!("   ✅ 缓存已启用 - 减少令牌获取API调用");
        println!("   📈 性能优势:");
        println!("      - 避免重复获取令牌");
        println!("      - 降低API调用频率");
        println!("      - 提高请求响应速度");
    } else {
        println!("   ⚠️ 缓存未启用 - 每次请求都会获取新令牌");
        println!("   💡 建议启用缓存以提高性能");
    }

    Ok(())
}

/// 演示用户身份验证流程
async fn demonstrate_auth_workflow(_client: &LarkClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔄 用户身份验证流程演示...");

    println!("📋 完整身份验证流程:");
    println!("   1. 用户授权阶段:");
    println!("      a) 引导用户到飞书授权页面");
    println!("      b) 用户确认授权范围");
    println!("      c) 获取授权code");

    println!("   2. 令牌获取阶段:");
    println!("      a) 使用授权code获取access_token");
    println!("      b) 获取refresh_token用于令牌刷新");
    println!("      c) 保存令牌信息到安全存储");

    println!("   3. API调用阶段:");
    println!("      a) 使用access_token调用API");
    println!("      b) 处理令牌过期情况");
    println!("      c) 使用refresh_token刷新令牌");

    println!("   4. 用户信息验证:");
    println!("      a) 调用用户信息API验证身份");
    println!("      b) 检查用户权限和企业归属");
    println!("      c) 记录用户活动日志");

    // 演示最佳实践
    println!("\n💡 身份验证最佳实践:");
    println!("   🔒 安全建议:");
    println!("      - 使用HTTPS传输令牌");
    println!("      - 令牌加密存储");
    println!("      - 定期检查令牌有效性");
    println!("      - 实现令牌自动刷新机制");

    println!("   ⚡ 性能优化:");
    println!("      - 启用令牌缓存");
    println!("      - 批量处理API请求");
    println!("      - 合理设置请求超时");
    println!("      - 实现请求重试机制");

    println!("   🛡️ 错误处理:");
    println!("      - 区分不同类型的认证错误");
    println!("      - 提供友好的错误提示");
    println!("      - 记录认证失败日志");
    println!("      - 实现降级处理策略");

    Ok(())
}

/// 演示用户权限检查
#[allow(dead_code)]
async fn demonstrate_permission_check(
    user_info: &open_lark::service::authentication::v1::UserInfo,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🛡️ 用户权限检查演示...");

    // 基于用户信息进行权限判断
    println!("📋 权限检查项目:");

    // 检查企业归属
    if !user_info.tenant_key.is_empty() {
        println!("   ✅ 企业归属验证: 通过 ({})", user_info.tenant_key);
    } else {
        println!("   ❌ 企业归属验证: 失败");
    }

    // 检查用户身份完整性
    if !user_info.user_id.is_empty() && !user_info.open_id.is_empty() {
        println!("   ✅ 用户身份验证: 通过");
    } else {
        println!("   ❌ 用户身份验证: 失败");
    }

    // 检查联系方式
    let has_contact = user_info.email.is_some() || user_info.mobile.is_some();
    if has_contact {
        println!("   ✅ 联系方式验证: 通过");
    } else {
        println!("   ⚠️ 联系方式验证: 无有效联系方式");
    }

    // 检查员工状态
    if !user_info.employee_no.is_empty() {
        println!("   ✅ 员工状态验证: 通过 (工号: {})", user_info.employee_no);
    } else {
        println!("   ⚠️ 员工状态验证: 无员工工号");
    }

    println!("\n💡 权限检查用途:");
    println!("   - 确保用户归属正确企业");
    println!("   - 验证用户身份真实性");
    println!("   - 检查功能访问权限");
    println!("   - 记录用户行为审计");

    Ok(())
}

/// 演示错误处理和重试机制
#[allow(dead_code)]
async fn demonstrate_error_handling(
    _client: &LarkClient,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔧 错误处理和重试机制演示...");

    println!("📋 常见认证错误类型:");

    // 模拟不同类型的错误
    let error_scenarios = vec![
        ("无效令牌", "Token无效或已过期"),
        ("权限不足", "用户权限不足或应用权限未授权"),
        ("网络错误", "网络连接超时或不稳定"),
        ("服务器错误", "飞书服务器临时不可用"),
        ("参数错误", "请求参数格式错误"),
    ];

    for (error_type, description) in error_scenarios {
        println!("   🔍 {error_type}: {description}");

        match error_type {
            "无效令牌" => {
                println!("      处理策略: 使用refresh_token刷新令牌");
            }
            "权限不足" => {
                println!("      处理策略: 引导用户重新授权或联系管理员");
            }
            "网络错误" => {
                println!("      处理策略: 实现指数退避重试机制");
            }
            "服务器错误" => {
                println!("      处理策略: 稍后重试或使用缓存数据");
            }
            "参数错误" => {
                println!("      处理策略: 验证参数格式并提示用户");
            }
            _ => {}
        }
    }

    println!("\n🔄 重试机制实现:");
    println!("   1. 指数退避算法: 1s, 2s, 4s, 8s...");
    println!("   2. 最大重试次数: 3-5次");
    println!("   3. 错误分类处理: 可重试vs不可重试错误");
    println!("   4. 熔断机制: 连续失败后暂停请求");

    Ok(())
}
