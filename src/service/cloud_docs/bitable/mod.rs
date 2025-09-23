pub mod v1;

pub struct BitableService {
    pub v1: v1::V1,
}

impl BitableService {
    pub fn new(config: crate::core::config::Config) -> Self {
        Self {
            v1: v1::V1::new(config),
        }
    }

    /// 使用共享配置（实验性）
    pub fn new_from_shared(shared: std::sync::Arc<crate::core::config::Config>) -> Self {
        Self { v1: v1::V1::new(shared.as_ref().clone()) }
    }
}
