#![allow(deprecated)]

/// 搜索对象请求 - 已废弃
#[deprecated(note = "使用新架构替代")]
pub struct SearchObjectRequest {
    _config: openlark_core::config::Config,
}

impl SearchObjectRequest {
    #[deprecated(note = "此 API 已废弃")]
    pub fn new(config: openlark_core::config::Config) -> Self {
        Self { _config: config }
    }
}
