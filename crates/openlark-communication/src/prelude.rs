//! OpenLark Communication Crate 预导出模块
//!
//! 集中导出最常用的类型和特征，简化导入。

// 重新导出核心类型
pub use openlark_core::{
    config::Config,
    SDKResult,
};

// 重新导出通用工具
pub use crate::common::chain::CommunicationClient;

// 重新导出端点常量
pub use crate::endpoints::{
    im::*,
    contact::*,
    aily::*,
    moments::*,
    mail::*,
    vc::*,
    event::*,
};

// 重新导出请求选项
pub use openlark_core::req_option::RequestOption;

// 重新导出常用的 API 类型（按 feature gate）
#[cfg(feature = "im")]
pub use crate::im::im::v1::message::models::ReceiveIdType;

#[cfg(feature = "contact")]
pub use crate::contact::contact::v3::user::models::UserIdType;

#[cfg(feature = "contact")]
pub use crate::contact::contact::v3::user::models::DepartmentIdType;
