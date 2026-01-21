//! 数据分析预导入模块
//!
//! 导入数据分析常用的类型和特征。

// 重新导出核心类型
pub use crate::AnalyticsService;
pub use crate::AnalyticsConfig;

// 重新导出 openlark-core 核心类型
pub use openlark_core::{config::Config, SDKResult, CoreError};

// 重新导出常用特征
pub use openlark_core::api::ApiRequest;
