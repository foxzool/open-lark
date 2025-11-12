//! Sheets电子表格服务
//!
//! 提供飞书电子表格的基础管理功能。
//! 注意：当前为迁移过渡状态，仅包含基础架构。

// 暂时注释掉有问题的模块
// pub mod v2;
// pub mod v3;
// pub use v2::*;
// pub use v3::*;

use openlark_core::{
    config::Config,
    trait_system::Service,
};


/// Sheets电子表格服务
///
/// 基础服务架构，具体功能在后续版本中实现。
#[derive(Debug, Clone)]
pub struct SheetsService {
    config: Config,
}

impl SheetsService {
    /// 创建Sheets服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

impl openlark_core::trait_system::Service for SheetsService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str
    where
        Self: Sized,
    {
        "SheetsService"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::trait_system::Service;

    #[test]
    fn test_sheets_service_creation() {
        let config = openlark_core::config::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let service = SheetsService::new(config);
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_service_trait_implementation() {
        let config = openlark_core::config::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let service = SheetsService::new(config);

        let config_ref = service.config();
        assert_eq!(config_ref.app_id, "test_app_id");
    }
}
