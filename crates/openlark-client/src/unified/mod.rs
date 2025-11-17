//! OpenLark 统一客户端模块
//!
//! 提供全新的统一客户端接口，所有服务使用相同的调用模式。
//!
//! ## 核心特性
//!
//! - **统一异步特征**：所有服务使用相同的异步接口模式
//! - **类型安全构建器**：编译时验证的服务配置和调用
//! - **零成本抽象**：功能标志在编译时优化，无运行时开销
//! - **服务注册发现**：动态服务加载和依赖注入
//! - **统一API接口**：所有服务使用一致的调用模式

pub mod client;
pub mod config;
pub mod error;
pub mod middleware;
pub mod registry;
pub mod traits;
pub mod transport;
pub mod services;
pub mod api;
pub mod unified_api;
pub mod service_adapter;

// 重新导出核心类型
pub use client::{UnifiedClient, UnifiedClientBuilder};
pub use config::{UnifiedConfig, ConfigBuilder};
pub use error::{UnifiedError, UnifiedResult};
pub use middleware::{Middleware, MiddlewareChain};
pub use registry::{ServiceRegistry, ServiceDescriptor};
pub use traits::{UnifiedService, APICall, ServiceBuilder};
pub use transport::{TransportLayer, RequestExecutor};
pub use services::{
    CommunicationService, HRService, DocsService, AIService, AuthService,
    CommunicationServiceBuilder, HRServiceBuilder, DocsServiceBuilder,
    AIServiceBuilder, AuthServiceBuilder
};
pub use api::{
    APIRequest, APIResponse, APIBuilder, APIExecutor, UnifiedAPIClient,
    APIError, CommunicationAPI, HRAPI, DocsAPI, AIAPI, AuthAPI
};
pub use unified_api::{OpenLarkClient, OpenLarkClientBuilder};
pub use service_adapter::{
    ServiceAdapter, APIDispatcher,
    CommunicationServiceAdapter, HRServiceAdapter, DocsServiceAdapter,
    AIServiceAdapter, AuthServiceAdapter, ServiceAdapterFactory,
};

/// 统一客户端预导出模块
///
/// 提供最常用的类型和函数，简化导入。
pub mod prelude {
    pub use super::{
        // 核心客户端
        OpenLarkClient, OpenLarkClientBuilder,
        UnifiedClient, UnifiedClientBuilder,

        // 配置
        UnifiedConfig, ConfigBuilder,

        // 特征
        UnifiedService, ServiceLifecycle, APIRequest, APIResponse,

        // 错误处理
        UnifiedError, UnifiedResult,

        // API
        UnifiedAPIClient, APIBuilder,
        CommunicationAPI, HRAPI, DocsAPI, AIAPI, AuthAPI,

        // 服务
        CommunicationService, HRService, DocsService, AIService, AuthService,
    };
}