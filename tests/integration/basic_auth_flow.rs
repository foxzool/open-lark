#![cfg(feature = "integration-tests")]

use std::env;

use open_lark::prelude::*;

// 受环境与特性门控的集成测试示例：
// - 仅在启用 feature `integration-tests` 时编译
// - 若 APP_ID/APP_SECRET 缺失，则跳过（返回 Ok(())）
// - 不进行真实网络调用，避免在 CI/沙箱环境失败

#[test]
fn test_build_client_from_env() -> Result<(), Box<dyn std::error::Error>> {
    let _ = dotenvy::dotenv();

    let app_id = env::var("APP_ID").ok();
    let app_secret = env::var("APP_SECRET").ok();

    if app_id.is_none() || app_secret.is_none() {
        eprintln!(
            "[integration-tests] 跳过：未设置 APP_ID/APP_SECRET 环境变量"
        );
        return Ok(());
    }

    let app_id = app_id.unwrap();
    let app_secret = app_secret.unwrap();

    let client = LarkClient::builder(&app_id, &app_secret)
        .with_app_type(AppType::SelfBuild)
        .build();

    assert_eq!(client.config.app_id, app_id);
    assert_eq!(client.config.app_secret, app_secret);

    // 如需真实 API 验证，可在本地解除注释并配置网络与凭证：
    // let _ = client.auth.v3.get_app_access_token(None).await?;

    Ok(())
}

