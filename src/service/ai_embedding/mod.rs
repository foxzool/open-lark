pub use v1::*;

mod v1;

/// AI嵌入服务
pub struct AIEmbeddingService {
    config: crate::core::config::Config,
}

impl AIEmbeddingService {
    pub fn new(config: crate::core::config::Config) -> Self {
        Self { config }
    }
}

impl crate::core::service_traits::Service for AIEmbeddingService {
    fn config(&self) -> &crate::core::config::Config {
        &self.config
    }
}