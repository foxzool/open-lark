use crate::core::config::Config;

pub mod models;
pub mod tag;
pub mod tag_binding;

use tag::TagService;
use tag_binding::TagBindingService;

/// 企业自定义群标签服务
pub struct TenantTagService {
    /// 标签管理服务
    pub tag: TagService,
    /// 标签绑定服务  
    pub tag_binding: TagBindingService,
}

impl TenantTagService {
    pub fn new(config: Config) -> Self {
        Self {
            tag: TagService::new(config.clone()),
            tag_binding: TagBindingService::new(config),
        }
    }
}