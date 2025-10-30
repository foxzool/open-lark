use std::sync::Arc;

use crate::core::{
    config::{Config, ConfigBuilder},
    constants::AppType,
};

// 条件导入服务
#[cfg(feature = "acs")]
use crate::service::acs::AcsService;
#[cfg(feature = "board")]
use open_lark_extensions::board::BoardService;
#[cfg(feature = "event")]
use open_lark_extensions::event::EventService;
#[cfg(feature = "admin")]
use crate::service::admin::AdminService;
#[cfg(feature = "auth")]
use crate::service::auth::AuthService;
#[cfg(feature = "ai")]
use crate::service::ai::AiService;
#[cfg(feature = "aily")]
use crate::service::aily::AilyService;
#[cfg(feature = "app_engine")]
use crate::service::app_engine::AppengineService;
#[cfg(feature = "authentication")]
use crate::service::authentication::AuthenticationService;
#[cfg(feature = "bot")]
use crate::service::bot::BotService;
#[cfg(feature = "calendar")]
use crate::service::calendar::CalendarService;
#[cfg(feature = "ccm")]
use crate::service::ccm::CcmService;
#[cfg(feature = "cloud-docs")]
use crate::service::cloud_docs::ClouddocsService;
#[cfg(feature = "contact")]
use crate::service::contact::ContactService;
#[cfg(feature = "group")]
use crate::service::group::GroupService;
#[cfg(feature = "im")]
use crate::service::im::ImService;
#[cfg(feature = "search")]
use crate::service::search::SearchService;
#[cfg(feature = "compensation-management")]
use open_lark_hr_suite::compensation_management::CompensationManagementService;

/// 飞书开放平台SDK主客户端
///
/// 提供对所有飞书开放平台API的统一访问接口。支持自建应用和商店应用两种类型，
/// 自动处理认证、令牌管理、请求重试等核心功能。
#[derive(Debug)]
pub struct LarkClient {
    pub config: Config,
    /// 共享配置（实验性）：单一 `Arc<Config>`，用于内部服务扇出以减少 clone
    #[allow(dead_code)] // Used in constructor and tests
    shared_config: Arc<Config>,

    // 核心服务 - 使用条件编译
    #[cfg(feature = "acs")]
    pub acs: AcsService,
    #[cfg(feature = "admin")]
    pub admin: AdminService,
    #[cfg(feature = "board")]
    pub board: BoardService,
    #[cfg(feature = "event")]
    pub event: EventService,
    #[cfg(feature = "auth")]
    pub auth: AuthService,
    #[cfg(feature = "ai")]
    pub ai: AiService,
    #[cfg(feature = "aily")]
    pub aily: AilyService,
    #[cfg(feature = "app_engine")]
    pub app_engine: AppengineService,
    #[cfg(feature = "authentication")]
    pub auth: AuthenticationService,
    #[cfg(feature = "bot")]
    pub bot: BotService,
    #[cfg(feature = "calendar")]
    pub calendar: CalendarService,
    #[cfg(feature = "ccm")]
    pub ccm: CcmService,
    #[cfg(feature = "cloud-docs")]
    pub cloud_docs: ClouddocsService,
    #[cfg(feature = "contact")]
    pub contact: ContactService,
    #[cfg(feature = "group")]
    pub group: GroupService,
    #[cfg(feature = "im")]
    pub im: ImService,
    #[cfg(feature = "search")]
    pub search: SearchService,
    #[cfg(feature = "compensation-management")]
    pub compensation_management: CompensationManagementService,
}

impl LarkClient {
    /// 从配置创建客户端
    pub fn new(config: Config) -> Self {
        let shared_config = Arc::new(config.clone());

        Self {
            config: config.clone(),
            shared_config,
            #[cfg(feature = "acs")]
            acs: AcsService::new(config.clone()),
            #[cfg(feature = "admin")]
            admin: AdminService::new(config.clone()),
            #[cfg(feature = "board")]
            board: BoardService::new(unsafe { std::mem::transmute_copy(&config) }),
            #[cfg(feature = "event")]
            event: EventService::new(unsafe { std::mem::transmute_copy(&config) }),
            #[cfg(feature = "auth")]
            auth: AuthService::new(config.clone()),
            #[cfg(feature = "ai")]
            ai: AiService::new(config.clone()),
            #[cfg(feature = "aily")]
            aily: AilyService::new(config.clone()),
            #[cfg(feature = "app_engine")]
            app_engine: AppengineService::new(config.clone()),
            #[cfg(feature = "authentication")]
            auth: AuthenticationService::new(config.clone()),
            #[cfg(feature = "bot")]
            bot: BotService::new(config.clone()),
            #[cfg(feature = "calendar")]
            calendar: CalendarService::new(config.clone()),
            #[cfg(feature = "ccm")]
            ccm: CcmService::new(config.clone()),
            #[cfg(feature = "cloud-docs")]
            cloud_docs: ClouddocsService::new(config.clone()),
            #[cfg(feature = "contact")]
            contact: ContactService::new(config.clone()),
            #[cfg(feature = "group")]
            group: GroupService::new(config.clone()),
            #[cfg(feature = "im")]
            im: ImService::new(config.clone()),
            #[cfg(feature = "search")]
            search: SearchService::new(config.clone()),
            #[cfg(feature = "compensation-management")]
            compensation_management: CompensationManagementService::new(config),
        }
    }

    /// 创建客户端构建器
    pub fn builder(app_id: impl Into<String>, app_secret: impl Into<String>) -> LarkClientBuilder {
        LarkClientBuilder::new(app_id, app_secret)
    }
}

/// 飞书客户端构建器
///
/// 使用构建器模式配置和创建LarkClient实例。
pub struct LarkClientBuilder {
    config_builder: ConfigBuilder,
}

impl LarkClientBuilder {
    /// 创建新的构建器实例
    pub fn new(app_id: impl Into<String>, app_secret: impl Into<String>) -> Self {
        Self {
            config_builder: ConfigBuilder::default()
                .app_id(app_id)
                .app_secret(app_secret),
        }
    }

    /// 设置应用类型
    pub fn with_app_type(mut self, app_type: AppType) -> Self {
        self.config_builder = self.config_builder.app_type(app_type);
        self
    }

    /// 构建客户端实例
    pub fn build(self) -> LarkClient {
        let config = self.config_builder.build();
        LarkClient::new(config)
    }
}
