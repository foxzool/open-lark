use crate::{
    config::{Config, ConfigBuilder},
    constants::AppType,
};
use openlark_core::{config, constants};

// 条件导入服务
#[cfg(feature = "acs")]
use crate::service::acs::AcsService;

// 云文档服务从 openlark-docs crate 导入
#[cfg(feature = "docs")]
use openlark_docs::docs::DocxService;
#[cfg(feature = "sheet")]
use openlark_docs::sheet::SheetsService;
#[cfg(feature = "bitable")]
use openlark_docs::bitable::BitableService;
#[cfg(feature = "wiki")]
use openlark_docs::wiki::WikiService;
#[cfg(feature = "drive")]
use openlark_docs::drive::DriveService;
#[cfg(feature = "ccm")]
use openlark_docs::ccm::CcmService;
#[cfg(feature = "admin")]
use crate::service::admin::AdminService;
#[cfg(feature = "ai")]
use crate::service::ai::AiService;
#[cfg(feature = "aily")]
use crate::service::aily::AilyService;
#[cfg(feature = "analytics")]
use crate::service::analytics::AnalyticsService;
#[cfg(feature = "apaas")]
use crate::service::apaas::ApaasServiceV1;
#[cfg(feature = "app_engine")]
use crate::service::app_engine::AppengineService;
#[cfg(feature = "approval")]
use crate::service::approval::ApprovalService;
#[cfg(feature = "attendance")]
use crate::service::attendance::AttendanceService;
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
#[cfg(feature = "event")]
use crate::service::event::EventService;
#[cfg(feature = "feishu_people")]
use crate::service::feishu_people::FeishuPeopleService;
#[cfg(feature = "group")]
use crate::service::group::GroupService;
#[cfg(feature = "im")]
use crate::service::im::ImService;
#[cfg(feature = "okr")]
use crate::service::okr::OkrService;
#[cfg(feature = "passport")]
use crate::service::passport::PassportService;
#[cfg(feature = "performance")]
use crate::service::performance::PerformanceService;
#[cfg(feature = "search")]
use crate::service::search::SearchService;
#[cfg(feature = "security_and_compliance")]
use crate::service::security_and_compliance::SecurityAndComplianceService;
#[cfg(feature = "task")]
use crate::service::task::TaskService;
#[cfg(feature = "board")]
use open_lark_extensions::board::BoardService;

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
    // TODO: Fix config type mismatch for extension services
    // #[cfg(feature = "board")]
    // pub board: BoardService,
    #[cfg(feature = "event")]
    pub event: EventService,
    #[cfg(feature = "ai")]
    pub ai: AiService,
    #[cfg(feature = "aily")]
    pub aily: AilyService,
    #[cfg(feature = "analytics")]
    pub analytics: AnalyticsService,
    #[cfg(feature = "apaas")]
    pub apaas: ApaasServiceV1,
    #[cfg(feature = "app_engine")]
    pub app_engine: AppengineService,
    #[cfg(feature = "attendance")]
    pub attendance: AttendanceService,
    #[cfg(feature = "approval")]
    pub approval: ApprovalService,
    #[cfg(feature = "authentication")]
    pub authentication: AuthenticationService,
    #[cfg(feature = "bot")]
    pub bot: BotService,
    #[cfg(feature = "calendar")]
    pub calendar: CalendarService,
    // 云文档服务 - 从 openlark-docs crate
    #[cfg(feature = "docs")]
    pub docs: DocxService,
    #[cfg(feature = "sheet")]
    pub sheet: SheetsService,
    #[cfg(feature = "bitable")]
    pub bitable: BitableService,
    #[cfg(feature = "wiki")]
    pub wiki: WikiService,
    #[cfg(feature = "drive")]
    pub drive: DriveService,
    #[cfg(feature = "ccm")]
    pub ccm: CcmService,
    #[cfg(feature = "contact")]
    pub contact: ContactService,
    #[cfg(feature = "group")]
    pub group: GroupService,
    #[cfg(feature = "im")]
    pub im: ImService,
    #[cfg(feature = "search")]
    pub search: SearchService,
    #[cfg(feature = "compensation-management")]
    /* pub compensation_management: CompensationManagementService, // Temporarily disabled due to config type mismatch */
    #[cfg(feature = "task")]
    pub task: TaskService,
    #[cfg(feature = "okr")]
    pub okr: OkrService,
    #[cfg(feature = "passport")]
    pub passport: PassportService,
    #[cfg(feature = "feishu_people")]
    pub feishu_people: FeishuPeopleService,
    #[cfg(feature = "performance")]
    pub performance: PerformanceService,
    #[cfg(feature = "security_and_compliance")]
    pub security_and_compliance: SecurityAndComplianceService,
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
            // TODO: Fix config type mismatch for extension services
            // #[cfg(feature = "board")]
            // board: BoardService::new(config.clone()),
            #[cfg(feature = "event")]
            event: EventService::new(config.clone()),
            #[cfg(feature = "ai")]
            ai: AiService::new(config.clone()),
            #[cfg(feature = "aily")]
            aily: AilyService::new(config.clone()),
            #[cfg(feature = "analytics")]
            analytics: AnalyticsService::new(config.clone()),
            #[cfg(feature = "apaas")]
            apaas: ApaasServiceV1::new(config.clone()),
            #[cfg(feature = "app_engine")]
            app_engine: AppengineService::new(config.clone()),
            #[cfg(feature = "attendance")]
            attendance: AttendanceService::new(config.clone()),
            #[cfg(feature = "approval")]
            approval: ApprovalService::new(config.clone()),
            #[cfg(feature = "authentication")]
            authentication: AuthenticationService::new(config.clone()),
            #[cfg(feature = "bot")]
            bot: BotService::new(config.clone()),
            #[cfg(feature = "calendar")]
            calendar: CalendarService::new(config.clone()),
            // 云文档服务 - 从 openlark-docs crate
            #[cfg(feature = "docs")]
            docs: DocxService::new(config.clone()),
            #[cfg(feature = "sheet")]
            sheet: SheetsService::new(config.clone()),
            #[cfg(feature = "bitable")]
            bitable: BitableService::new(config.clone()),
            #[cfg(feature = "wiki")]
            wiki: WikiService::new(config.clone()),
            #[cfg(feature = "drive")]
            drive: DriveService::new(config.clone()),
            #[cfg(feature = "ccm")]
            ccm: CcmService::new(config.clone()),
            #[cfg(feature = "contact")]
            contact: ContactService::new(config.clone()),
            #[cfg(feature = "group")]
            group: GroupService::new(config.clone()),
            #[cfg(feature = "im")]
            im: ImService::new(config.clone()),
            #[cfg(feature = "search")]
            search: SearchService::new(config.clone()),
            #[cfg(feature = "compensation-management")]
            /* compensation_management: CompensationManagementService::new(config), // Temporarily disabled due to config type mismatch */
            #[cfg(feature = "task")]
            task: TaskService::new(config.clone()),

            #[cfg(feature = "okr")]
            okr: OkrService::new(config.clone()),
            #[cfg(feature = "passport")]
            passport: PassportService::new(config.clone()),
            #[cfg(feature = "feishu_people")]
            feishu_people: FeishuPeopleService::new(config.clone()),
            #[cfg(feature = "performance")]
            performance: PerformanceService::new(config.clone()),

            #[cfg(feature = "security_and_compliance")]
            security_and_compliance: SecurityAndComplianceService::new(config.clone()),
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

    /// 设置是否启用令牌缓存
    pub fn with_enable_token_cache(mut self, enable: bool) -> Self {
        self.config_builder = self.config_builder.enable_token_cache(enable);
        self
    }

    /// 构建客户端实例
    pub fn build(self) -> LarkClient {
        let config = self.config_builder.build();
        LarkClient::new(config)
    }
}
