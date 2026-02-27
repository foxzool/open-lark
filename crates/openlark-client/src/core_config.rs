//! openlark-client → openlark-core 配置转换
//!
//! 目标：
//! - 单一来源：避免在多个位置重复拼装 core Config（DRY）
//! - 行为一致：meta 单入口的所有业务入口使用同一套映射（KISS）

use crate::Config;

#[cfg(feature = "auth")]
use openlark_auth::AuthTokenProvider;

/// 基础 core Config（不注入 TokenProvider）
///
/// 说明：
/// - 该 Config 适合用于构建 TokenProvider 本身（避免递归持有）
/// - 也适合用于「auth 模块」这类不应依赖 token 自动获取的 API（如获取 app/tenant token）
pub(crate) fn build_base_core_config(config: &Config) -> openlark_core::config::Config {
    openlark_core::config::Config::builder()
        .app_id(config.app_id.clone())
        .app_secret(config.app_secret.clone())
        .base_url(config.base_url.clone())
        .app_type(config.app_type)
        .enable_token_cache(config.enable_token_cache)
        .req_timeout(config.timeout)
        .header(config.headers.clone())
        .build()
}

/// 业务调用用的 core Config（默认注入 openlark-auth 的 TokenProvider）
///
/// 说明：
/// - 当 `enable_token_cache=false` 时，即使注入 provider，core 也不会自动拉取 token（由 core 控制）
/// - provider 内部持有 base_core_config（不含 provider），避免潜在递归依赖
pub(crate) fn build_core_config_with_default_token_provider(
    config: &Config,
) -> openlark_core::config::Config {
    let base_core_config = build_base_core_config(config);

    #[cfg(feature = "auth")]
    {
        let provider = AuthTokenProvider::new(base_core_config.clone());
        return base_core_config.with_token_provider(provider);
    }

    #[cfg(not(feature = "auth"))]
    {
        base_core_config
    }
}
