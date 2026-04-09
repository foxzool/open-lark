//! OpenLark 官方入口 crate。
//!
//! 普通用户应优先使用 `openlark`，通过业务 feature 开启所需能力：
//!
//! ```toml
//! [dependencies]
//! openlark = { version = "0.15.0", default-features = false, features = ["auth", "docs-drive", "docs-bitable", "webhook-signature"] }
//! ```
//!
//! - 统一客户端入口：[`Client`]
//! - 高级业务模块入口：[`auth`]、[`communication`]、[`docs`]、[`workflow`] 等
//! - 统一预导出：[`prelude`]
//!
//! 若只想要单一业务域的最小依赖，再直接使用 `openlark-{domain}` 子 crate。
//!
//! Canonical public API 入口规则见 `docs/PUBLIC_REEXPORT_POLICY.md`。
//!
//! 推荐顺序：
//!
//! - 运行时入口：[`Client`] / [`ClientBuilder`]
//! - 导入入口：[`prelude`]
//! - 业务命名空间：`open_lark::auth`、`open_lark::communication`、`open_lark::docs`
//! - 最小依赖场景：直接依赖对应 `openlark-{domain}` crate
//!
//! 顶层 `AuthClient`、`DocsClient`、`HrClient` 等类型 re-export 会继续保留，
//! 但它们的定位是 compatibility alias，而不是普通用户的主入口。
//!
//! 根 crate `prelude` 有意不暴露客户端实现层细节：
//!
//! ```rust,compile_fail
//! use open_lark::prelude::*;
//!
//! let _registry: ServiceRegistry;
//! ```

// 允许测试模块中的未使用导入（测试桩代码常见模式）
#![allow(unused_imports)]

// 原始 crate-name passthrough re-export 仅用于兼容历史路径，
// 不再作为根 crate 的推荐公开入口出现在文档中。
#[doc(hidden)]
pub use openlark_client;
pub use openlark_client::{Client, ClientBuilder, Config, Error, Result};
#[doc(hidden)]
pub use openlark_core;
#[doc(hidden)]
pub use openlark_core as core;
pub use openlark_core::config::Config as CoreConfig;
pub use openlark_core::error::{CoreError, ErrorCode, ErrorSeverity, ErrorTrait, ErrorType};
pub use openlark_core::req_option::RequestOption;
pub use openlark_core::SDKResult;

#[cfg(feature = "websocket")]
/// WebSocket 客户端相关类型导出。
pub mod ws_client {
    pub use openlark_client::ws_client::*;
}

#[cfg(feature = "auth")]
#[doc(hidden)]
pub use openlark_auth;

#[cfg(feature = "auth")]
pub use openlark_auth as auth;

#[cfg(feature = "communication")]
#[doc(hidden)]
pub use openlark_communication;

#[cfg(feature = "communication")]
pub use openlark_communication as communication;

#[cfg(any(
    feature = "docs",
    feature = "docs-ccm",
    feature = "docs-base",
    feature = "docs-bitable",
    feature = "docs-drive",
    feature = "docs-explorer",
    feature = "docs-sheets",
    feature = "docs-sheets-v2",
    feature = "docs-sheets-v3",
    feature = "docs-full"
))]
#[doc(hidden)]
pub use openlark_docs;

#[cfg(any(
    feature = "docs",
    feature = "docs-ccm",
    feature = "docs-base",
    feature = "docs-bitable",
    feature = "docs-drive",
    feature = "docs-explorer",
    feature = "docs-sheets",
    feature = "docs-sheets-v2",
    feature = "docs-sheets-v3",
    feature = "docs-full"
))]
pub use openlark_docs as docs;

#[cfg(feature = "hr")]
pub use openlark_hr as hr;

#[cfg(feature = "ai")]
pub use openlark_ai as ai;

#[cfg(feature = "helpdesk")]
pub use openlark_helpdesk as helpdesk;

#[cfg(feature = "mail")]
pub use openlark_mail as mail;

#[cfg(feature = "meeting")]
pub use openlark_meeting as meeting;

#[cfg(feature = "application")]
pub use openlark_application as application;

#[cfg(feature = "security")]
pub use openlark_security as security;

#[cfg(feature = "workflow")]
pub use openlark_workflow as workflow;

#[cfg(feature = "platform")]
pub use openlark_platform as platform;

#[cfg(feature = "analytics")]
pub use openlark_analytics as analytics;

#[cfg(feature = "user")]
pub use openlark_user as user;

#[cfg(feature = "webhook")]
#[doc(hidden)]
pub use openlark_webhook;

#[cfg(feature = "webhook")]
pub use openlark_webhook as webhook;

#[cfg(feature = "cardkit")]
pub use openlark_cardkit as cardkit;

// 顶层 meta client 类型 re-export 保留为兼容层；
// 普通 `openlark` 用户优先通过 `Client` 字段访问业务能力。
#[cfg(feature = "auth")]
#[doc(hidden)]
pub use openlark_client::AuthClient;

#[cfg(any(
    feature = "docs",
    feature = "docs-ccm",
    feature = "docs-base",
    feature = "docs-bitable",
    feature = "docs-drive",
    feature = "docs-explorer",
    feature = "docs-sheets",
    feature = "docs-sheets-v2",
    feature = "docs-sheets-v3",
    feature = "docs-full"
))]
#[doc(hidden)]
pub use openlark_client::DocsClient;

#[cfg(feature = "communication")]
#[doc(hidden)]
pub use openlark_client::CommunicationClient;

#[cfg(feature = "hr")]
#[doc(hidden)]
pub use openlark_client::HrClient;

#[cfg(feature = "meeting")]
#[doc(hidden)]
pub use openlark_client::MeetingClient;

#[cfg(feature = "cardkit")]
#[doc(hidden)]
pub use openlark_client::CardkitClient;

#[cfg(feature = "ai")]
#[doc(hidden)]
pub use openlark_client::AiClient;

#[cfg(feature = "workflow")]
#[doc(hidden)]
pub use openlark_client::WorkflowClient;

#[cfg(feature = "platform")]
#[doc(hidden)]
pub use openlark_client::PlatformClient;

#[cfg(feature = "application")]
#[doc(hidden)]
pub use openlark_client::ApplicationClient;

#[cfg(feature = "helpdesk")]
#[doc(hidden)]
pub use openlark_client::HelpdeskClient;

#[cfg(feature = "mail")]
#[doc(hidden)]
pub use openlark_client::MailClient;

#[cfg(feature = "analytics")]
#[doc(hidden)]
pub use openlark_client::AnalyticsClient;

#[cfg(feature = "user")]
#[doc(hidden)]
pub use openlark_client::UserClient;

#[cfg(feature = "security")]
#[doc(hidden)]
pub use openlark_client::SecurityServices;

/// 面向 `openlark` 用户的统一预导出。
///
/// 该模块只导出“创建客户端 + 顶层业务入口”所需的稳定公共类型。
/// registry / feature loader / traits 等高级客户端层能力保留在 `openlark-client`。
pub mod prelude {
    #[cfg(feature = "auth")]
    #[doc(hidden)]
    pub use crate::AuthClient;
    #[cfg(feature = "communication")]
    #[doc(hidden)]
    pub use crate::CommunicationClient;
    #[cfg(any(
        feature = "docs",
        feature = "docs-ccm",
        feature = "docs-base",
        feature = "docs-bitable",
        feature = "docs-drive",
        feature = "docs-explorer",
        feature = "docs-sheets",
        feature = "docs-sheets-v2",
        feature = "docs-sheets-v3",
        feature = "docs-full"
    ))]
    #[doc(hidden)]
    pub use crate::DocsClient;
    #[cfg(feature = "hr")]
    #[doc(hidden)]
    pub use crate::HrClient;
    #[cfg(feature = "meeting")]
    #[doc(hidden)]
    pub use crate::MeetingClient;
    #[cfg(feature = "ai")]
    #[doc(hidden)]
    pub use crate::AiClient;
    #[cfg(feature = "workflow")]
    #[doc(hidden)]
    pub use crate::WorkflowClient;
    #[cfg(feature = "platform")]
    #[doc(hidden)]
    pub use crate::PlatformClient;
    #[cfg(feature = "application")]
    #[doc(hidden)]
    pub use crate::ApplicationClient;
    #[cfg(feature = "helpdesk")]
    #[doc(hidden)]
    pub use crate::HelpdeskClient;
    #[cfg(feature = "mail")]
    #[doc(hidden)]
    pub use crate::MailClient;
    #[cfg(feature = "analytics")]
    #[doc(hidden)]
    pub use crate::AnalyticsClient;
    #[cfg(feature = "user")]
    #[doc(hidden)]
    pub use crate::UserClient;
    #[cfg(feature = "security")]
    #[doc(hidden)]
    pub use crate::SecurityServices;
    pub use crate::SDKResult;
    pub use crate::{Client, ClientBuilder, Config, CoreConfig, Error, Result};
    pub use crate::{CoreError, ErrorCode, ErrorSeverity, ErrorTrait, ErrorType, RequestOption};
    pub use openlark_core::prelude::*;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_test_client() -> Result<Client> {
        Client::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build()
    }

    #[test]
    fn root_prelude_exposes_canonical_core_entrypoints() {
        use crate::prelude::*;

        let _builder: ClientBuilder = Client::builder();
        let _config: Config = Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build()
            .expect("test config should build");
        let _request_option: Option<RequestOption> = None;
    }

    #[cfg(feature = "auth")]
    #[test]
    fn root_client_exposes_auth_entrypoint() {
        let client = build_test_client().expect("client should build with auth feature");

        let _auth = &client.auth;
        let alias = std::any::type_name::<crate::AuthClient>();
        let compat = std::any::type_name::<openlark_client::AuthClient>();

        assert_eq!(alias, compat);
    }

    #[cfg(feature = "communication")]
    #[test]
    fn root_client_exposes_communication_namespace() {
        let client = build_test_client().expect("client should build with communication feature");

        let _communication = &client.communication;
        let _endpoint = crate::communication::endpoints::IM_V1_MESSAGES;
    }

    #[cfg(any(
        feature = "docs",
        feature = "docs-ccm",
        feature = "docs-base",
        feature = "docs-bitable",
        feature = "docs-drive",
        feature = "docs-explorer",
        feature = "docs-sheets",
        feature = "docs-sheets-v2",
        feature = "docs-sheets-v3",
        feature = "docs-full"
    ))]
    #[test]
    fn root_client_exposes_docs_namespace() {
        let client = build_test_client().expect("client should build with docs feature");

        let _docs = &client.docs;
        let alias = std::any::type_name::<crate::DocsClient>();
        let compat = std::any::type_name::<openlark_client::DocsClient>();

        assert_eq!(alias, compat);
    }

    #[cfg(feature = "hr")]
    #[test]
    fn root_client_exposes_hr_entrypoint() {
        let client = build_test_client().expect("client should build with hr feature");

        let _hr = &client.hr;
        let alias = std::any::type_name::<crate::HrClient>();
        let compat = std::any::type_name::<openlark_client::HrClient>();

        assert_eq!(alias, compat);
    }
}
