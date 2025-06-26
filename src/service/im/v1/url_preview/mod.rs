use crate::core::config::Config;

// TODO: 以下功能待实现
// pub mod batch_update;

/// URL预览服务
///
/// 提供URL预览的批量更新功能
pub struct UrlPreviewService {
    pub config: Config,
}

impl UrlPreviewService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}