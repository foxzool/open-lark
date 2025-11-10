//! API端点常量定义模块
//!
//! 本模块集中定义飞书开放平台的所有API端点路径常量，旨在：
//! 1. 减少字符串分配 - 避免每次API调用时重新创建路径字符串
//! 2. 防止拼写错误 - 统一管理所有API路径，编译期检查
//! 3. 便于维护升级 - 集中管理，方便API版本升级和路径变更
//!
//! # 性能优化
//!
//! 使用静态字符串常量可以显著减少内存分配：
//! ```rust
//! // 优化前: 每次调用都创建新字符串 (~60字节堆分配)
//! api_path: "/open-apis/workplace/v1/workplace_access_data/search".to_string(),
//!
//! // 优化后: 使用静态常量
//! api_path: Endpoints::WORKPLACE_ACCESS_DATA_SEARCH.to_string(), // 仅在需要时分配
//! // 或者进一步优化为
//! api_path: Endpoints::WORKPLACE_ACCESS_DATA_SEARCH, // 零分配（如果API支持&str）
//! ```
//!
//! # 组织结构
//!
//! API端点按服务模块分组：
//! - `workplace` - 工作台相关API
//! - `vc` - 视频会议相关API  
//! - `im` - 即时消息相关API
//! - `drive` - 云盘相关API
//! - 等等...

/// 飞书API端点路径常量定义
///
/// 所有API端点的静态字符串常量，按服务分组组织。
/// 使用模块结构提供更好的命名空间和组织结构。
pub struct Endpoints;

impl Endpoints {
    // ==================== 工作台服务端点 ====================

    /// 搜索工作台访问数据
    pub const WORKPLACE_ACCESS_DATA_SEARCH: &'static str =
        "/open-apis/workplace/v1/workplace_access_data/search";

    /// 搜索自定义工作台访问数据
    pub const WORKPLACE_CUSTOM_ACCESS_DATA_SEARCH: &'static str =
        "/open-apis/workplace/v1/custom_workplace_access_data/search";

    /// 搜索工作台小部件访问数据
    pub const WORKPLACE_WIDGET_ACCESS_DATA_SEARCH: &'static str =
        "/open-apis/workplace/v1/custom_workplace_widget_access_data/search";