//! 版本信息模块

/// OpenLark Core版本信息
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// 版本信息结构
#[derive(Debug, Clone)]
pub struct VersionInfo {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub pre_release: Option<String>,
    pub build_metadata: Option<String>,
}

impl VersionInfo {
    /// 创建新的版本信息
    pub fn new() -> Self {
        let version = VERSION;
        parse_version(version)
    }

    /// 获取完整版本字符串
    pub fn full_version(&self) -> String {
        VERSION.to_string()
    }
}

/// 解析版本字符串
fn parse_version(version: &str) -> VersionInfo {
    let parts: Vec<&str> = version.split('.').collect();
    let major = parts.get(0).and_then(|s| s.parse().ok()).unwrap_or(0);
    let minor = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
    let patch = parts.get(2)
        .and_then(|s| s.split('-').next())
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);

    VersionInfo {
        major,
        minor,
        patch,
        pre_release: None,
        build_metadata: None,
    }
}

impl Default for VersionInfo {
    fn default() -> Self {
        Self::new()
    }
}