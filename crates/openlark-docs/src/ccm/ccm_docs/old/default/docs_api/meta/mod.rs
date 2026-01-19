/// 获取元数据请求 - 已废弃
#[deprecated(note = "使用新架构替代")]
pub struct GetMetaRequest {
    _config: openlark_core::config::Config,
}

impl GetMetaRequest {
    #[deprecated(note = "此 API 已废弃")]
    pub fn new(config: openlark_core::config::Config) -> Self {
        Self { _config: config }
    }
}
