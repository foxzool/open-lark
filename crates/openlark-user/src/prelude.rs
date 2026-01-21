//! 用户设置预导入模块
//!
//! 导入用户设置常用的类型和特征。

// 重新导出核心类型
pub use crate::UserService;
pub use crate::UserConfig;

// 重新导出 openlark-core 核心类型
pub use openlark_core::{config::Config, SDKResult, CoreError};

// 重新导出常用特征
pub use openlark_core::api::ApiRequest;
