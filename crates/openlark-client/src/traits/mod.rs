//! OpenLark Client 核心特征定义
//!
//! 定义客户端和服务的核心抽象接口

mod client;
mod service;

// 重新导出核心特征 - 仅保留实际实现的特征
pub use client::LarkClient;
pub use service::{ServiceLifecycle, ServiceTrait};