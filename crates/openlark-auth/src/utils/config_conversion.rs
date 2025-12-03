//! 配置转换工具
//!
//! 提供 AuthConfig 与 openlark-core Config 之间的转换功能

use crate::models::AuthConfig;
use openlark_core::config::Config;

/// 将 AuthConfig 转换为 openlark-core 的 Config
pub fn auth_config_to_core_config(auth_config: AuthConfig, app_type: openlark_core::constants::AppType) -> Config {
    Config::builder()
        .app_id(auth_config.app_id)
        .app_secret(auth_config.app_secret)
        .base_url(&auth_config.base_url)
        .app_type(app_type)
        .build()
}

/// 将商店应用 AuthConfig 转换为 openlark-core 的 Config
pub fn marketplace_auth_config_to_core(auth_config: AuthConfig) -> Config {
    auth_config_to_core_config(auth_config, openlark_core::constants::AppType::Marketplace)
}

/// 将自建应用 AuthConfig 转换为 openlark-core 的 Config
pub fn self_build_auth_config_to_core(auth_config: AuthConfig) -> Config {
    auth_config_to_core_config(auth_config, openlark_core::constants::AppType::SelfBuild)
}

/// 将 AuthConfig 转换为 user_access_token 认证专用的 Config
/// user_access_token 认证只需要 base_url，不需要应用凭证
pub fn authen_auth_config_to_core(auth_config: AuthConfig) -> Config {
    Config::builder()
        .base_url(&auth_config.base_url)
        // user_access_token 认证不需要应用凭证，但设置空值以符合Config要求
        .app_id("")
        .app_secret("")
        .app_type(openlark_core::constants::AppType::Marketplace) // 默认类型，实际不使用
        .build()
}