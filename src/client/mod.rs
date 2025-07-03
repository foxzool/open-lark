use std::{sync::Arc, time::Duration};

use crate::core::{config::Config, constants::AppType};

// 条件导入服务
#[cfg(feature = "acs")]
use crate::service::acs::AcsService;
#[cfg(feature = "admin")]
use crate::service::admin::AdminService;
#[cfg(feature = "ai")]
use crate::service::ai::AiService;
#[cfg(feature = "aily")]
use crate::service::aily::AilyService;
#[cfg(feature = "apass")]
use crate::service::apass::ApassService;
#[cfg(feature = "application")]
use crate::service::application::ApplicationService;
#[cfg(feature = "approval")]
use crate::service::approval::ApprovalService;
#[cfg(feature = "attendance")]
use crate::service::attendance::AttendanceService;
#[cfg(feature = "authentication")]
use crate::service::authentication::AuthenService;
#[cfg(feature = "bot")]
use crate::service::bot::BotService;
#[cfg(feature = "calendar")]
use crate::service::calendar::CalendarService;
#[cfg(feature = "cardkit")]
use crate::service::cardkit::CardkitService;
#[cfg(feature = "cloud-docs")]
use crate::service::cloud_docs::CloudDocsService;
#[cfg(feature = "contact")]
use crate::service::contact::ContactService;
#[cfg(feature = "corehr")]
use crate::service::corehr::CoreHRService;
#[cfg(feature = "directory")]
use crate::service::directory::DirectoryService;
#[cfg(feature = "ehr")]
use crate::service::ehr::EhrService;
#[cfg(feature = "elearning")]
use crate::service::elearning::ELearningService;
#[cfg(feature = "group")]
use crate::service::group::GroupService;
#[cfg(feature = "helpdesk")]
use crate::service::helpdesk::HelpdeskService;
#[cfg(feature = "hire")]
use crate::service::hire::HireService;
#[cfg(feature = "human-authentication")]
use crate::service::human_authentication::HumanAuthenticationService;
#[cfg(feature = "im")]
use crate::service::im::ImService;
#[cfg(feature = "lingo")]
use crate::service::lingo::LingoService;
#[cfg(feature = "mail")]
use crate::service::mail::MailService;
#[cfg(feature = "mdm")]
use crate::service::mdm::MdmService;
#[cfg(feature = "minutes")]
use crate::service::minutes::MinutesService;
#[cfg(feature = "moments")]
use crate::service::moments::MomentsService;
#[cfg(feature = "okr")]
use crate::service::okr::OkrService;
#[cfg(feature = "payroll")]
use crate::service::payroll::PayrollService;
#[cfg(feature = "performance")]
use crate::service::performance::PerformanceService;
#[cfg(feature = "personal-settings")]
use crate::service::personal_settings::PersonalSettingsService;
#[cfg(feature = "report")]
use crate::service::report::ReportService;
#[cfg(feature = "search")]
use crate::service::search::SearchService;
#[cfg(feature = "security-and-compliance")]
use crate::service::security_and_compliance::SecurityAndComplianceService;
#[cfg(feature = "task")]
use crate::service::task::TaskV2Service;
#[cfg(feature = "tenant")]
use crate::service::tenant::TenantService;
#[cfg(feature = "tenant-tag")]
use crate::service::tenant_tag::TenantTagService;
#[cfg(feature = "trust-party")]
use crate::service::trust_party::TrustPartyService;
#[cfg(feature = "vc")]
use crate::service::vc::VcService;
#[cfg(feature = "verification")]
use crate::service::verification::VerificationService;
#[cfg(feature = "workplace")]
use crate::service::workplace::WorkplaceService;

// 向后兼容的导入
#[cfg(feature = "cloud-docs")]
use crate::service::{
    AssistantService, BitableService, BoardService, CommentsService, DocsService, DriveService,
    PermissionService, SheetsService, WikiService,
};

#[cfg(feature = "websocket")]
pub mod ws_client;

/// 飞书开放平台SDK主客户端
///
/// 提供对所有飞书开放平台API的统一访问接口。支持自建应用和商店应用两种类型，
/// 自动处理认证、令牌管理、请求重试等核心功能。
///
/// # 主要功能
///
/// - 🔐 自动令牌管理和刷新
/// - 🚀 支持所有飞书开放平台API
/// - 🔄 内置请求重试机制
/// - 📡 WebSocket长连接支持（需开启websocket特性）
/// - 🎯 类型安全的API调用
///
/// # 快速开始
///
/// ```rust
/// use open_lark::prelude::*;
///
/// // 创建自建应用客户端
/// let client = LarkClient::builder("your_app_id", "your_app_secret")
///     .with_app_type(AppType::SelfBuild)
///     .with_enable_token_cache(true)
///     .build();
///
/// // 发送文本消息
/// let message = CreateMessageRequestBody::builder()
///     .receive_id("ou_xxx")
///     .msg_type("text")
///     .content("{\"text\":\"Hello from Rust!\"}")
///     .build();
///
/// let request = CreateMessageRequest::builder()
///     .receive_id_type("open_id")
///     .request_body(message)
///     .build();
///
/// // let result = client.im.message.create(request, None).await?;
/// ```
///
/// # 服务模块
///
/// 客户端包含以下主要服务模块：
/// - `im`: 即时消息
/// - `drive`: 云盘文件
/// - `sheets`: 电子表格
/// - `calendar`: 日历
/// - `contact`: 通讯录
/// - `hire`: 招聘
/// - 更多服务请参考各字段文档
pub struct LarkClient {
    pub config: Config,
    // 核心服务 - 使用条件编译
    #[cfg(feature = "acs")]
    pub acs: AcsService,
    #[cfg(feature = "admin")]
    pub admin: AdminService,
    #[cfg(feature = "ai")]
    pub ai: AiService,
    #[cfg(feature = "aily")]
    pub aily: AilyService,
    #[cfg(feature = "apass")]
    pub apass: ApassService,
    #[cfg(feature = "application")]
    pub application: ApplicationService,
    #[cfg(feature = "approval")]
    pub approval: ApprovalService,
    #[cfg(feature = "attendance")]
    pub attendance: AttendanceService,
    #[cfg(feature = "authentication")]
    pub auth: AuthenService,
    #[cfg(feature = "bot")]
    pub bot: BotService,
    #[cfg(feature = "calendar")]
    pub calendar: CalendarService,
    #[cfg(feature = "cardkit")]
    pub cardkit: CardkitService,
    #[cfg(feature = "contact")]
    pub contact: ContactService,
    #[cfg(feature = "corehr")]
    pub corehr: CoreHRService,
    #[cfg(feature = "directory")]
    pub directory: DirectoryService,
    #[cfg(feature = "ehr")]
    pub ehr: EhrService,
    #[cfg(feature = "elearning")]
    pub elearning: ELearningService,
    #[cfg(feature = "group")]
    pub group: GroupService,
    #[cfg(feature = "helpdesk")]
    pub helpdesk: HelpdeskService,
    #[cfg(feature = "hire")]
    pub hire: HireService,
    #[cfg(feature = "human-authentication")]
    pub human_authentication: HumanAuthenticationService,
    #[cfg(feature = "im")]
    pub im: ImService,
    #[cfg(feature = "lingo")]
    pub lingo: LingoService,
    #[cfg(feature = "mail")]
    pub mail: MailService,
    #[cfg(feature = "mdm")]
    pub mdm: MdmService,
    #[cfg(feature = "minutes")]
    pub minutes: MinutesService,
    #[cfg(feature = "moments")]
    pub moments: MomentsService,
    #[cfg(feature = "okr")]
    pub okr: OkrService,
    #[cfg(feature = "payroll")]
    pub payroll: PayrollService,
    #[cfg(feature = "performance")]
    pub performance: PerformanceService,
    #[cfg(feature = "personal-settings")]
    pub personal_settings: PersonalSettingsService,
    #[cfg(feature = "report")]
    pub report: ReportService,
    #[cfg(feature = "search")]
    pub search: SearchService,
    #[cfg(feature = "security-and-compliance")]
    pub security_and_compliance: SecurityAndComplianceService,
    #[cfg(feature = "task")]
    pub task: TaskV2Service,
    #[cfg(feature = "tenant")]
    pub tenant: TenantService,
    #[cfg(feature = "tenant-tag")]
    pub tenant_tag: TenantTagService,
    #[cfg(feature = "trust-party")]
    pub trust_party: TrustPartyService,
    #[cfg(feature = "vc")]
    pub vc: VcService,
    #[cfg(feature = "verification")]
    pub verification: VerificationService,
    #[cfg(feature = "workplace")]
    pub workplace: WorkplaceService,
    // 云文档服务聚合
    #[cfg(feature = "cloud-docs")]
    pub cloud_docs: CloudDocsService,
    // 向后兼容的字段
    #[cfg(feature = "cloud-docs")]
    pub assistant: AssistantService,
    #[cfg(feature = "cloud-docs")]
    pub docs: DocsService,
    #[cfg(feature = "cloud-docs")]
    pub drive: DriveService,
    #[cfg(feature = "cloud-docs")]
    pub sheets: SheetsService,
    #[cfg(feature = "cloud-docs")]
    pub bitable: BitableService,
    #[cfg(feature = "cloud-docs")]
    pub wiki: WikiService,
    #[cfg(feature = "cloud-docs")]
    pub comments: CommentsService,
    #[cfg(feature = "cloud-docs")]
    pub permission: PermissionService,
    #[cfg(feature = "cloud-docs")]
    pub board: BoardService,
}

/// 飞书客户端构建器
///
/// 使用构建器模式配置和创建LarkClient实例。支持链式调用配置各种选项。
///
/// # 示例
///
/// ```rust
/// use open_lark::prelude::*;
///
/// let client = LarkClient::builder("app_id", "app_secret")
///     .with_app_type(AppType::SelfBuild)
///     .with_enable_token_cache(true)
///     .with_req_timeout(Some(30.0))
///     .build();
/// ```
pub struct LarkClientBuilder {
    config: Config,
}

impl LarkClientBuilder {
    /// 设置应用类型
    ///
    /// # 参数
    /// - `app_type`: 应用类型，`AppType::SelfBuild`（自建应用）或`AppType::Marketplace`（商店应用）
    pub fn with_app_type(mut self, app_type: AppType) -> Self {
        self.config.app_type = app_type;
        self
    }

    /// 设置为商店应用（等同于 `with_app_type(AppType::Marketplace)`）
    pub fn with_marketplace_app(mut self) -> Self {
        self.config.app_type = AppType::Marketplace;
        self
    }

    /// 设置自定义API基础URL
    ///
    /// # 参数
    /// - `base_url`: 自定义的API基础URL，默认为官方地址
    pub fn with_open_base_url(mut self, base_url: String) -> Self {
        self.config.base_url = base_url;
        self
    }

    /// 启用或禁用令牌缓存
    ///
    /// # 参数
    /// - `enable`: 是否启用令牌缓存，建议启用以提高性能
    pub fn with_enable_token_cache(mut self, enable: bool) -> Self {
        self.config.enable_token_cache = enable;
        self
    }

    /// 设置请求超时时间
    ///
    /// # 参数
    /// - `timeout`: 超时时间（秒），None表示使用默认值
    pub fn with_req_timeout(mut self, timeout: Option<f32>) -> Self {
        self.config.req_timeout = timeout.map(Duration::from_secs_f32);
        self
    }

    /// 构建LarkClient实例
    ///
    /// 根据配置的参数创建最终的客户端实例。
    pub fn build(mut self) -> LarkClient {
        if let Some(req_timeout) = self.config.req_timeout {
            self.config.http_client = reqwest::Client::builder()
                .timeout(req_timeout)
                .build()
                .expect("Failed to build HTTP client with timeout")
        }

        // 创建单个 Arc<Config> 并在所有服务间共享
        let config = Arc::new(self.config.clone());

        LarkClient {
            config: self.config.clone(),
            // 核心服务 - 使用条件编译
            #[cfg(feature = "acs")]
            acs: AcsService::new((*config).clone()),
            #[cfg(feature = "admin")]
            admin: AdminService::new((*config).clone()),
            #[cfg(feature = "ai")]
            ai: AiService::new((*config).clone()),
            #[cfg(feature = "aily")]
            aily: AilyService::new((*config).clone()),
            #[cfg(feature = "apass")]
            apass: ApassService::new((*config).clone()),
            #[cfg(feature = "application")]
            application: ApplicationService::new((*config).clone()),
            #[cfg(feature = "approval")]
            approval: ApprovalService::new((*config).clone()),
            #[cfg(feature = "attendance")]
            attendance: AttendanceService::new(Arc::clone(&config)),
            #[cfg(feature = "authentication")]
            auth: AuthenService::new(Arc::clone(&config)),
            #[cfg(feature = "bot")]
            bot: BotService::new((*config).clone()),
            #[cfg(feature = "calendar")]
            calendar: CalendarService::new((*config).clone()),
            #[cfg(feature = "cardkit")]
            cardkit: CardkitService::new((*config).clone()),
            #[cfg(feature = "contact")]
            contact: ContactService::new((*config).clone()),
            #[cfg(feature = "corehr")]
            corehr: CoreHRService::new((*config).clone()),
            #[cfg(feature = "directory")]
            directory: DirectoryService::new((*config).clone()),
            #[cfg(feature = "ehr")]
            ehr: EhrService::new((*config).clone()),
            #[cfg(feature = "elearning")]
            elearning: ELearningService::new((*config).clone()),
            #[cfg(feature = "group")]
            group: GroupService::new((*config).clone()),
            #[cfg(feature = "helpdesk")]
            helpdesk: HelpdeskService::new((*config).clone()),
            #[cfg(feature = "hire")]
            hire: HireService::new((*config).clone()),
            #[cfg(feature = "human-authentication")]
            human_authentication: HumanAuthenticationService::new((*config).clone()),
            #[cfg(feature = "im")]
            im: ImService::new(Arc::clone(&config)),
            #[cfg(feature = "lingo")]
            lingo: LingoService::new((*config).clone()),
            #[cfg(feature = "mail")]
            mail: MailService::new((*config).clone()),
            #[cfg(feature = "mdm")]
            mdm: MdmService::new((*config).clone()),
            #[cfg(feature = "minutes")]
            minutes: MinutesService::new((*config).clone()),
            #[cfg(feature = "moments")]
            moments: MomentsService::new((*config).clone()),
            #[cfg(feature = "okr")]
            okr: OkrService::new((*config).clone()),
            #[cfg(feature = "payroll")]
            payroll: PayrollService::new((*config).clone()),
            #[cfg(feature = "performance")]
            performance: PerformanceService::new((*config).clone()),
            #[cfg(feature = "personal-settings")]
            personal_settings: PersonalSettingsService::new((*config).clone()),
            #[cfg(feature = "report")]
            report: ReportService::new((*config).clone()),
            #[cfg(feature = "search")]
            search: SearchService::new(Arc::clone(&config)),
            #[cfg(feature = "security-and-compliance")]
            security_and_compliance: SecurityAndComplianceService::new((*config).clone()),
            #[cfg(feature = "task")]
            task: TaskV2Service::new((*config).clone()),
            #[cfg(feature = "tenant")]
            tenant: TenantService::new((*config).clone()),
            #[cfg(feature = "tenant-tag")]
            tenant_tag: TenantTagService::new((*config).clone()),
            #[cfg(feature = "trust-party")]
            trust_party: TrustPartyService::new((*config).clone()),
            #[cfg(feature = "vc")]
            vc: VcService::new((*config).clone()),
            #[cfg(feature = "verification")]
            verification: VerificationService::new((*config).clone()),
            #[cfg(feature = "workplace")]
            workplace: WorkplaceService::new((*config).clone()),
            // 云文档服务聚合
            #[cfg(feature = "cloud-docs")]
            cloud_docs: CloudDocsService::new(Arc::clone(&config)),
            // 向后兼容的字段（重新创建实例）
            #[cfg(feature = "cloud-docs")]
            assistant: AssistantService::new(Arc::clone(&config)),
            #[cfg(feature = "cloud-docs")]
            docs: DocsService::new(Arc::clone(&config)),
            #[cfg(feature = "cloud-docs")]
            drive: DriveService::new(Arc::clone(&config)),
            #[cfg(feature = "cloud-docs")]
            sheets: SheetsService::new(Arc::clone(&config)),
            #[cfg(feature = "cloud-docs")]
            bitable: BitableService::new(Arc::clone(&config)),
            #[cfg(feature = "cloud-docs")]
            wiki: WikiService::new(Arc::clone(&config)),
            #[cfg(feature = "cloud-docs")]
            comments: CommentsService::new(Arc::clone(&config)),
            #[cfg(feature = "cloud-docs")]
            permission: PermissionService::new(Arc::clone(&config)),
            #[cfg(feature = "cloud-docs")]
            board: BoardService::new(Arc::clone(&config)),
        }
    }
}

impl LarkClient {
    /// 创建客户端构建器
    ///
    /// # 参数
    /// - `app_id`: 应用ID，从飞书开放平台获取
    /// - `app_secret`: 应用密钥，从飞书开放平台获取
    ///
    /// # 示例
    /// ```rust
    /// use open_lark::prelude::*;
    ///
    /// let client = LarkClient::builder("cli_xxx", "xxx")
    ///     .with_app_type(AppType::SelfBuild)
    ///     .build();
    /// ```
    pub fn builder(app_id: &str, app_secret: &str) -> LarkClientBuilder {
        LarkClientBuilder {
            config: Config {
                app_id: app_id.to_string(),
                app_secret: app_secret.to_string(),
                ..Default::default()
            },
        }
    }
}
