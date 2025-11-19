//! Open-Lark Admin Module
//!
//! 飞书管理后台服务模块，提供完整的系统管理和治理功能。
//!
//! ## 主要功能
//!
//! - **ACS门禁系统**: 设备管理、访客管理、门禁记录、访问控制规则
//! - **后台管理**: 密码管理、勋章系统、数据报表、用户治理
//! - **MDM主数据**: 国家地区管理、用户认证数据关系
//! - **安全合规**: 审计数据、开放接口日志、合规管理
//! - **租户管理**: 企业信息、产品分配、租户配置
//! - **可信第三方**: 第三方服务集成和信任管理
//! - **工作台管理**: 工作台配置、应用推荐、访问数据分析
//!
//! ## 使用示例
//!
//! ```rust
//! use openlark_admin::endpoints::*;
//!
//! // 使用端点常量
//! let devices_endpoint = ACS_V1_DEVICES;
//! let badges_endpoint = ADMIN_V1_BADGES_LIST;
//! let tenant_endpoint = TENANT_V2_QUERY;
//! println!("ACS设备端点: {}", devices_endpoint);
//! println!("管理勋章端点: {}", badges_endpoint);
//! println!("租户查询端点: {}", tenant_endpoint);
//! ```
//!
//! ## 端点组织
//!
//! - `acs`: ACS门禁控制端点
//! - `admin`: 后台管理端点
//! - `mdm`: 主数据管理端点
//! - `security`: 安全合规端点
//! - `tenant`: 租户管理端点
//! - `workplace`: 工作台管理端点

#![allow(missing_docs)]

// Include macros first
#[macro_use]
mod macros;

// 导入端点模块
pub mod endpoints;

// 重新导出端点常量，方便外部使用
pub use endpoints::*;

/// Re-exports from openlark-core for convenience.
pub mod prelude {
    pub use openlark_core::SDKResult;
}
