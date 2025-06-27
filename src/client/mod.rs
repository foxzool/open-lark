use std::{sync::Arc, time::Duration};

use crate::{
    core::{config::Config, constants::AppType},
    service::{
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
        corehr::CoreHRService,
        directory::DirectoryService,
        ehr::EhrService,
        group::GroupService,
        helpdesk::HelpdeskService,
        hire::HireService,
        im::ImService,
        mail::MailService,
        minutes::MinutesService,
        moments::MomentsService,
        payroll::PayrollService,
        personal_settings::PersonalSettingsService,
        search::SearchService,
        task::TaskV2Service,
        tenant::TenantService,
        tenant_tag::TenantTagService,
        vc::VcService,
        verification::VerificationService,
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

/// 飞书开放平台SDK client
pub struct LarkClient {
    pub config: Config,
    // 核心服务
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
    pub corehr: CoreHRService,
    pub directory: DirectoryService,
    pub ehr: EhrService,
    pub group: GroupService,
    pub helpdesk: HelpdeskService,
    pub hire: HireService,
    pub im: ImService,
    pub mail: MailService,
    pub minutes: MinutesService,
    pub moments: MomentsService,
    pub payroll: PayrollService,
    pub personal_settings: PersonalSettingsService,
    pub search: SearchService,
    pub task: TaskV2Service,
    pub tenant: TenantService,
    pub tenant_tag: TenantTagService,
    pub vc: VcService,
    pub verification: VerificationService,
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

pub struct LarkClientBuilder {
    config: Config,
}

impl LarkClientBuilder {
    pub fn with_app_type(mut self, app_type: AppType) -> Self {
        self.config.app_type = app_type;
        self
    }

    pub fn with_marketplace_app(mut self) -> Self {
        self.config.app_type = AppType::Marketplace;
        self
    }

    pub fn with_open_base_url(mut self, base_url: String) -> Self {
        self.config.base_url = base_url;
        self
    }

    pub fn with_enable_token_cache(mut self, enable: bool) -> Self {
        self.config.enable_token_cache = enable;
        self
    }

    pub fn with_req_timeout(mut self, timeout: Option<f32>) -> Self {
        self.config.req_timeout = timeout.map(Duration::from_secs_f32);
        self
    }

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
            corehr: CoreHRService::new((*config).clone()),
            directory: DirectoryService::new((*config).clone()),
            ehr: EhrService::new((*config).clone()),
            group: GroupService::new((*config).clone()),
            helpdesk: HelpdeskService::new((*config).clone()),
            hire: HireService::new((*config).clone()),
            im: ImService::new(Arc::clone(&config)),
            mail: MailService::new((*config).clone()),
            minutes: MinutesService::new((*config).clone()),
            moments: MomentsService::new((*config).clone()),
            payroll: PayrollService::new((*config).clone()),
            personal_settings: PersonalSettingsService::new((*config).clone()),
            search: SearchService::new(Arc::clone(&config)),
            task: TaskV2Service::new((*config).clone()),
            tenant: TenantService::new((*config).clone()),
            tenant_tag: TenantTagService::new((*config).clone()),
            vc: VcService::new((*config).clone()),
            verification: VerificationService::new((*config).clone()),
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
