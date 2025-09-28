use std::sync::Arc;
use std::time::Duration;

use crate::core::{
    config::{Config, ConfigBuilder},
    constants::AppType,
};

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
    /// 共享配置（实验性）：单一 `Arc<Config>`，用于内部服务扇出以减少 clone
    #[allow(dead_code)] // Used in constructor and tests
    shared_config: Arc<Config>,
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
    config_builder: ConfigBuilder,
}

impl LarkClientBuilder {
    /// 获取当前配置构建器（仅测试使用）
    #[cfg(test)]
    fn build_config(&self) -> Config {
        self.config_builder.clone().build()
    }

    /// 设置应用类型
    ///
    /// # 参数
    /// - `app_type`: 应用类型，`AppType::SelfBuild`（自建应用）或`AppType::Marketplace`（商店应用）
    pub fn with_app_type(mut self, app_type: AppType) -> Self {
        self.config_builder = self.config_builder.app_type(app_type);
        self
    }

    /// 设置为商店应用（等同于 `with_app_type(AppType::Marketplace)`）
    pub fn with_marketplace_app(mut self) -> Self {
        self.config_builder = self.config_builder.app_type(AppType::Marketplace);
        self
    }

    /// 设置自定义API基础URL
    ///
    /// # 参数
    /// - `base_url`: 自定义的API基础URL，默认为官方地址
    pub fn with_open_base_url(mut self, base_url: String) -> Self {
        self.config_builder = self.config_builder.base_url(base_url);
        self
    }

    /// 启用或禁用令牌缓存
    ///
    /// # 参数
    /// - `enable`: 是否启用令牌缓存，建议启用以提高性能
    pub fn with_enable_token_cache(mut self, enable: bool) -> Self {
        self.config_builder = self.config_builder.enable_token_cache(enable);
        self
    }

    /// 设置请求超时时间
    ///
    /// # 参数
    /// - `timeout`: 超时时间（秒），None表示使用默认值
    pub fn with_req_timeout(mut self, timeout: Option<f32>) -> Self {
        if let Some(timeout) = timeout {
            self.config_builder = self
                .config_builder
                .req_timeout(Duration::from_secs_f32(timeout));
        }
        self
    }

    /// 构建LarkClient实例
    ///
    /// 根据配置的参数创建最终的客户端实例。
    pub fn build(self) -> LarkClient {
        let config = self.config_builder.build();
        let shared_config = Arc::new(config.clone());
        LarkClient {
            config: config.clone(),
            shared_config: shared_config.clone(),
            // 核心服务 - 使用条件编译
            #[cfg(feature = "acs")]
            acs: AcsService::new(config.clone()),
            #[cfg(feature = "admin")]
            admin: AdminService::new(config.clone()),
            #[cfg(feature = "ai")]
            ai: AiService::new(config.clone()),
            #[cfg(feature = "aily")]
            aily: AilyService::new(config.clone()),
            #[cfg(feature = "apass")]
            apass: ApassService::new(config.clone()),
            #[cfg(feature = "application")]
            application: ApplicationService::new_from_shared(shared_config.clone()),
            #[cfg(feature = "approval")]
            approval: ApprovalService::new(config.clone()),
            #[cfg(feature = "attendance")]
            attendance: AttendanceService::new(config.clone()),
            #[cfg(feature = "authentication")]
            auth: AuthenService::new(config.clone()),
            #[cfg(feature = "bot")]
            bot: BotService::new(config.clone()),
            #[cfg(feature = "calendar")]
            calendar: CalendarService::new_from_shared(shared_config.clone()),
            #[cfg(feature = "cardkit")]
            cardkit: CardkitService::new(config.clone()),
            #[cfg(feature = "contact")]
            contact: ContactService::new_from_shared(shared_config.clone()),
            #[cfg(feature = "corehr")]
            corehr: CoreHRService::new(config.clone()),
            #[cfg(feature = "directory")]
            directory: DirectoryService::new_from_shared(shared_config.clone()),
            #[cfg(feature = "ehr")]
            ehr: EhrService::new_from_shared(shared_config.clone()),
            #[cfg(feature = "elearning")]
            elearning: ELearningService::new(config.clone()),
            #[cfg(feature = "group")]
            group: GroupService::new(config.clone()),
            #[cfg(feature = "helpdesk")]
            helpdesk: HelpdeskService::new(config.clone()),
            #[cfg(feature = "hire")]
            hire: HireService::new(config.clone()),
            #[cfg(feature = "human-authentication")]
            human_authentication: HumanAuthenticationService::new(config.clone()),
            #[cfg(feature = "im")]
            im: ImService::new_from_shared(shared_config.clone()),
            #[cfg(feature = "lingo")]
            lingo: LingoService::new(config.clone()),
            #[cfg(feature = "mail")]
            mail: MailService::new(config.clone()),
            #[cfg(feature = "mdm")]
            mdm: MdmService::new(config.clone()),
            #[cfg(feature = "minutes")]
            minutes: MinutesService::new(config.clone()),
            #[cfg(feature = "moments")]
            moments: MomentsService::new(config.clone()),
            #[cfg(feature = "okr")]
            okr: OkrService::new(config.clone()),
            #[cfg(feature = "payroll")]
            payroll: PayrollService::new(config.clone()),
            #[cfg(feature = "performance")]
            performance: PerformanceService::new(config.clone()),
            #[cfg(feature = "personal-settings")]
            personal_settings: PersonalSettingsService::new(config.clone()),
            #[cfg(feature = "report")]
            report: ReportService::new_from_shared(shared_config.clone()),
            #[cfg(feature = "search")]
            search: SearchService::new_from_shared(shared_config.clone()),
            #[cfg(feature = "security-and-compliance")]
            security_and_compliance: SecurityAndComplianceService::new_from_shared(
                shared_config.clone(),
            ),
            #[cfg(feature = "task")]
            task: TaskV2Service::new_from_shared(shared_config.clone()),
            #[cfg(feature = "tenant")]
            tenant: TenantService::new_from_shared(shared_config.clone()),
            #[cfg(feature = "tenant-tag")]
            tenant_tag: TenantTagService::new(config.clone()),
            #[cfg(feature = "trust-party")]
            trust_party: TrustPartyService::new(config.clone()),
            #[cfg(feature = "vc")]
            vc: VcService::new(config.clone()),
            #[cfg(feature = "verification")]
            verification: VerificationService::new(config.clone()),
            #[cfg(feature = "workplace")]
            workplace: WorkplaceService::new(config.clone()),
            // 云文档服务聚合
            #[cfg(feature = "cloud-docs")]
            cloud_docs: CloudDocsService::new_from_shared(shared_config.clone()),
            // 向后兼容的字段（重新创建实例）
            #[cfg(feature = "cloud-docs")]
            assistant: AssistantService::new_from_shared(shared_config.clone()),
            #[cfg(feature = "cloud-docs")]
            docs: DocsService::new(config.clone()),
            #[cfg(feature = "cloud-docs")]
            drive: DriveService::new_from_shared(shared_config.clone()),
            #[cfg(feature = "cloud-docs")]
            sheets: SheetsService::new_from_shared(shared_config.clone()),
            #[cfg(feature = "cloud-docs")]
            bitable: BitableService::new_from_shared(shared_config.clone()),
            #[cfg(feature = "cloud-docs")]
            wiki: WikiService::new_from_shared(shared_config.clone()),
            #[cfg(feature = "cloud-docs")]
            comments: CommentsService::new_from_shared(shared_config.clone()),
            #[cfg(feature = "cloud-docs")]
            permission: PermissionService::new_from_shared(shared_config.clone()),
            #[cfg(feature = "cloud-docs")]
            board: BoardService::new_from_shared(shared_config.clone()),
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
            config_builder: Config::builder().app_id(app_id).app_secret(app_secret),
        }
    }

    /// 获取共享配置（用于内部服务扇出，减少 clone）
    #[allow(dead_code)] // Used by services in constructor
    pub(crate) fn shared_config(&self) -> Arc<Config> {
        self.shared_config.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    fn create_test_builder() -> LarkClientBuilder {
        LarkClient::builder("test_app_id", "test_app_secret")
    }

    #[test]
    fn test_client_builder_creation() {
        let client = LarkClient::builder("test_id", "test_secret").build();
        assert_eq!(client.config.app_id, "test_id");
        assert_eq!(client.config.app_secret, "test_secret");
        assert_eq!(client.config.app_type, AppType::SelfBuild); // Default
    }

    #[test]
    fn test_builder_with_app_type() {
        let client = create_test_builder()
            .with_app_type(AppType::Marketplace)
            .build();
        assert_eq!(client.config.app_type, AppType::Marketplace);
    }

    #[test]
    fn test_builder_with_marketplace_app() {
        let client = create_test_builder().with_marketplace_app().build();
        assert_eq!(client.config.app_type, AppType::Marketplace);
    }

    #[test]
    fn test_builder_with_custom_base_url() {
        let custom_url = "https://custom.api.feishu.cn";
        let client = create_test_builder()
            .with_open_base_url(custom_url.to_string())
            .build();
        assert_eq!(client.config.base_url, custom_url);
    }

    #[test]
    fn test_builder_with_enable_token_cache() {
        let client_enabled = create_test_builder().with_enable_token_cache(true).build();
        assert!(client_enabled.config.enable_token_cache);

        let client_disabled = create_test_builder().with_enable_token_cache(false).build();
        assert!(!client_disabled.config.enable_token_cache);
    }

    #[test]
    fn test_builder_with_req_timeout() {
        let timeout_seconds = 30.0;
        let client = create_test_builder()
            .with_req_timeout(Some(timeout_seconds))
            .build();

        let expected_duration = Duration::from_secs_f32(timeout_seconds);
        assert_eq!(client.config.req_timeout, Some(expected_duration));
    }

    #[test]
    fn test_builder_with_none_timeout() {
        let client = create_test_builder().with_req_timeout(None).build();
        assert_eq!(client.config.req_timeout, None);
    }

    #[test]
    fn test_builder_chaining() {
        let client = create_test_builder()
            .with_app_type(AppType::Marketplace)
            .with_enable_token_cache(true)
            .with_req_timeout(Some(45.0))
            .with_open_base_url("https://test.api.feishu.cn".to_string())
            .build();

        assert_eq!(client.config.app_type, AppType::Marketplace);
        assert!(client.config.enable_token_cache);
        assert_eq!(
            client.config.req_timeout,
            Some(Duration::from_secs_f32(45.0))
        );
        assert_eq!(client.config.base_url, "https://test.api.feishu.cn");
    }

    #[test]
    fn test_client_build() {
        let client = create_test_builder().build();

        assert_eq!(client.config.app_id, "test_app_id");
        assert_eq!(client.config.app_secret, "test_app_secret");
        assert_eq!(client.config.app_type, AppType::SelfBuild);
    }

    #[test]
    fn test_client_build_with_timeout() {
        let client = create_test_builder().with_req_timeout(Some(60.0)).build();

        assert_eq!(
            client.config.req_timeout,
            Some(Duration::from_secs_f32(60.0))
        );
    }

    #[test]
    fn test_client_build_marketplace_app() {
        let client = create_test_builder().with_marketplace_app().build();

        assert_eq!(client.config.app_type, AppType::Marketplace);
    }

    #[test]
    fn test_client_build_with_custom_config() {
        let client = create_test_builder()
            .with_app_type(AppType::Marketplace)
            .with_enable_token_cache(false)
            .with_open_base_url("https://custom.feishu.cn".to_string())
            .with_req_timeout(Some(120.0))
            .build();

        assert_eq!(client.config.app_type, AppType::Marketplace);
        assert!(!client.config.enable_token_cache);
        assert_eq!(client.config.base_url, "https://custom.feishu.cn");
        assert_eq!(
            client.config.req_timeout,
            Some(Duration::from_secs_f32(120.0))
        );
    }

    #[test]
    fn test_builder_empty_credentials() {
        let builder = LarkClient::builder("", "");
        let config = builder.build_config();
        assert_eq!(config.app_id, "");
        assert_eq!(config.app_secret, "");
    }

    #[test]
    fn test_builder_unicode_credentials() {
        let app_id = "测试_app_id_🔑";
        let app_secret = "测试_secret_🔐";
        let builder = LarkClient::builder(app_id, app_secret);
        let config = builder.build_config();

        assert_eq!(config.app_id, app_id);
        assert_eq!(config.app_secret, app_secret);
    }

    #[test]
    fn test_builder_very_long_credentials() {
        let long_id = "a".repeat(1000);
        let long_secret = "b".repeat(1000);
        let builder = LarkClient::builder(&long_id, &long_secret);
        let config = builder.build_config();

        assert_eq!(config.app_id, long_id);
        assert_eq!(config.app_secret, long_secret);
    }

    #[test]
    fn test_builder_special_characters() {
        let special_id = "app-id_123!@#$%^&*()";
        let special_secret = "secret/\\?<>|:\"{}";
        let builder = LarkClient::builder(special_id, special_secret);
        let config = builder.build_config();

        assert_eq!(config.app_id, special_id);
        assert_eq!(config.app_secret, special_secret);
    }

    #[test]
    fn test_builder_extreme_timeout_values() {
        // Very small timeout
        let small_timeout = create_test_builder().with_req_timeout(Some(0.001)).build();
        assert_eq!(
            small_timeout.config.req_timeout,
            Some(Duration::from_secs_f32(0.001))
        );

        // Very large timeout
        let large_timeout = create_test_builder()
            .with_req_timeout(Some(3600.0)) // 1 hour
            .build();
        assert_eq!(
            large_timeout.config.req_timeout,
            Some(Duration::from_secs_f32(3600.0))
        );
    }

    #[test]
    fn test_config_independence() {
        // Test that multiple builders don't interfere with each other
        let builder1 = create_test_builder().with_app_type(AppType::Marketplace);

        let builder2 = create_test_builder().with_app_type(AppType::SelfBuild);

        assert_eq!(builder1.build_config().app_type, AppType::Marketplace);
        assert_eq!(builder2.build_config().app_type, AppType::SelfBuild);
    }

    #[test]
    fn test_builder_default_values() {
        let builder = create_test_builder();
        let config = builder.build_config();

        // Verify default values match Config defaults
        assert_eq!(config.app_type, AppType::SelfBuild);
        assert!(config.enable_token_cache); // Default from Config
        assert_eq!(config.req_timeout, None);
        assert!(!config.base_url.is_empty()); // Should have default URL
    }

    #[cfg(feature = "cloud-docs")]
    #[test]
    fn test_client_cloud_docs_services() {
        let client = create_test_builder().build();

        // Verify cloud docs services are available when feature is enabled
        // This is mainly to check that the struct is properly constructed
        // We can't test much without actual functionality, but we can verify the services exist
        let _assistant = &client.assistant;
        let _drive = &client.drive;
        let _sheets = &client.sheets;
        let _bitable = &client.bitable;
        let _wiki = &client.wiki;
        let _docs = &client.docs;
    }

    #[test]
    fn test_client_builder_multiple_configurations() {
        // Test that the builder can be used to create multiple different clients
        let client1 = create_test_builder().with_marketplace_app().build();

        let client2 = LarkClient::builder("different_id", "different_secret")
            .with_enable_token_cache(false)
            .build();

        assert_eq!(client1.config.app_type, AppType::Marketplace);
        assert_eq!(client1.config.app_id, "test_app_id");

        assert_eq!(client2.config.app_type, AppType::SelfBuild);
        assert_eq!(client2.config.app_id, "different_id");
        assert!(!client2.config.enable_token_cache);
    }
}
