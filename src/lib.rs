#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]
#![allow(rustdoc::broken_intra_doc_links)]

/// 飞书卡片组件和工具
pub mod card;
/// 客户端实现和WebSocket支持
pub mod client;
/// 核心功能：HTTP传输、配置、错误处理等
pub mod core;
// pub mod message;
/// 自定义机器人相关功能
pub mod custom_bot;
/// 事件处理和分发器
pub mod event;
/// 常用类型和trait的重导出，方便用户使用
pub mod prelude;
/// 所有飞书开放平台API服务的实现
pub mod service;
