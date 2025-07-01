use std::{sync::Arc, time::Duration};

use crate::{
    core::{config::Config, constants::AppType},
    service::{
        acs::AcsService,
        admin::AdminService,
        ai::AiService,
        aily::AilyService,
        apass::ApassService,
        application::ApplicationService,
        approval::ApprovalService,
        attendance::AttendanceService,
        authentication::AuthenService,
        bot::BotService,
        calendar::CalendarService,
        cardkit::CardkitService,
        cloud_docs::CloudDocsService,
        contact::ContactService,
        corehr::CoreHRService,
        directory::DirectoryService,
        ehr::EhrService,
        elearning::ELearningService,
        group::GroupService,
        helpdesk::HelpdeskService,
        hire::HireService,
        human_authentication::HumanAuthenticationService,
        im::ImService,
        lingo::LingoService,
        mail::MailService,
        mdm::MdmService,
        minutes::MinutesService,
        moments::MomentsService,
        okr::OkrService,
        payroll::PayrollService,
        performance::PerformanceService,
        personal_settings::PersonalSettingsService,
        report::ReportService,
        search::SearchService,
        security_and_compliance::SecurityAndComplianceService,
        task::TaskV2Service,
        tenant::TenantService,
        tenant_tag::TenantTagService,
        trust_party::TrustPartyService,
        vc::VcService,
        verification::VerificationService,
        workplace::WorkplaceService,
        // 向后兼容的导入
        AssistantService,
        BitableService,
        BoardService,
        CommentsService,
        DocsService,
        DriveService,
        PermissionService,
        SheetsService,
        WikiService,
    },
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
///     .with_app_type(AppType::SelfBuilt)
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
    // 核心服务
    pub acs: AcsService,
    pub admin: AdminService,
    pub ai: AiService,
    pub aily: AilyService,
    pub apass: ApassService,
    pub application: ApplicationService,
    pub approval: ApprovalService,
    pub attendance: AttendanceService,
    pub auth: AuthenService,
    pub bot: BotService,
    pub calendar: CalendarService,
    pub cardkit: CardkitService,
    pub contact: ContactService,
    pub corehr: CoreHRService,
    pub directory: DirectoryService,
    pub ehr: EhrService,
    pub elearning: ELearningService,
    pub group: GroupService,
    pub helpdesk: HelpdeskService,
    pub hire: HireService,
    pub human_authentication: HumanAuthenticationService,
    pub im: ImService,
    pub lingo: LingoService,
    pub mail: MailService,
    pub mdm: MdmService,
    pub minutes: MinutesService,
    pub moments: MomentsService,
    pub okr: OkrService,
    pub payroll: PayrollService,
    pub performance: PerformanceService,
    pub personal_settings: PersonalSettingsService,
    pub report: ReportService,
    pub search: SearchService,
    pub security_and_compliance: SecurityAndComplianceService,
    pub task: TaskV2Service,
    pub tenant: TenantService,
    pub tenant_tag: TenantTagService,
    pub trust_party: TrustPartyService,
    pub vc: VcService,
    pub verification: VerificationService,
    pub workplace: WorkplaceService,
    // 云文档服务聚合
    pub cloud_docs: CloudDocsService,
    // 向后兼容的字段
    pub assistant: AssistantService,
    pub docs: DocsService,
    pub drive: DriveService,
    pub sheets: SheetsService,
    pub bitable: BitableService,
    pub wiki: WikiService,
    pub comments: CommentsService,
    pub permission: PermissionService,
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
///     .with_app_type(AppType::SelfBuilt)
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
    /// - `app_type`: 应用类型，`AppType::SelfBuilt`（自建应用）或`AppType::Marketplace`（商店应用）
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

        // 创建云文档服务聚合
        let cloud_docs = CloudDocsService::new(Arc::clone(&config));

        LarkClient {
            config: self.config.clone(),
            // 核心服务
            acs: AcsService::new((*config).clone()),
            admin: AdminService::new((*config).clone()),
            ai: AiService::new((*config).clone()),
            aily: AilyService::new((*config).clone()),
            apass: ApassService::new((*config).clone()),
            application: ApplicationService::new((*config).clone()),
            approval: ApprovalService::new((*config).clone()),
            attendance: AttendanceService::new(Arc::clone(&config)),
            auth: AuthenService::new(Arc::clone(&config)),
            bot: BotService::new((*config).clone()),
            calendar: CalendarService::new((*config).clone()),
            cardkit: CardkitService::new((*config).clone()),
            contact: ContactService::new((*config).clone()),
            corehr: CoreHRService::new((*config).clone()),
            directory: DirectoryService::new((*config).clone()),
            ehr: EhrService::new((*config).clone()),
            elearning: ELearningService::new((*config).clone()),
            group: GroupService::new((*config).clone()),
            helpdesk: HelpdeskService::new((*config).clone()),
            hire: HireService::new((*config).clone()),
            human_authentication: HumanAuthenticationService::new((*config).clone()),
            im: ImService::new(Arc::clone(&config)),
            lingo: LingoService::new((*config).clone()),
            mail: MailService::new((*config).clone()),
            mdm: MdmService::new((*config).clone()),
            minutes: MinutesService::new((*config).clone()),
            moments: MomentsService::new((*config).clone()),
            okr: OkrService::new((*config).clone()),
            payroll: PayrollService::new((*config).clone()),
            performance: PerformanceService::new((*config).clone()),
            personal_settings: PersonalSettingsService::new((*config).clone()),
            report: ReportService::new((*config).clone()),
            search: SearchService::new(Arc::clone(&config)),
            security_and_compliance: SecurityAndComplianceService::new((*config).clone()),
            task: TaskV2Service::new((*config).clone()),
            tenant: TenantService::new((*config).clone()),
            tenant_tag: TenantTagService::new((*config).clone()),
            trust_party: TrustPartyService::new((*config).clone()),
            vc: VcService::new((*config).clone()),
            verification: VerificationService::new((*config).clone()),
            workplace: WorkplaceService::new((*config).clone()),
            // 云文档服务聚合
            cloud_docs,
            // 向后兼容的字段（重新创建实例）
            assistant: AssistantService::new(Arc::clone(&config)),
            docs: DocsService::new(Arc::clone(&config)),
            drive: DriveService::new(Arc::clone(&config)),
            sheets: SheetsService::new(Arc::clone(&config)),
            bitable: BitableService::new(Arc::clone(&config)),
            wiki: WikiService::new(Arc::clone(&config)),
            comments: CommentsService::new(Arc::clone(&config)),
            permission: PermissionService::new(Arc::clone(&config)),
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
    ///     .with_app_type(AppType::SelfBuilt)
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
