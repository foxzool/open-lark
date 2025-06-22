use std::{sync::Arc, time::Duration};

use crate::{
    core::{config::Config, constants::AppType},
    service::{
        assistant::AssistantService, attendance::AttendanceService, authentication::AuthenService,
        bitable::BitableService, board::BoardService, comments::CommentsService, docs::DocsService,
        drive::DriveService, im::ImService, permission::PermissionService, search::SearchService,
        sheets::SheetsService, wiki::WikiService,
    },
};

#[cfg(feature = "websocket")]
pub mod ws_client;

/// 飞书开放平台SDK client
pub struct LarkClient {
    pub config: Config,
    pub assistant: AssistantService,
    pub attendance: AttendanceService,
    pub auth: AuthenService,
    pub docs: DocsService,
    pub im: ImService,
    pub drive: DriveService,
    pub search: SearchService,
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

        LarkClient {
            config: self.config,
            assistant: AssistantService::new(Arc::clone(&config)),
            attendance: AttendanceService::new(Arc::clone(&config)),
            auth: AuthenService::new(Arc::clone(&config)),
            docs: DocsService::new(Arc::clone(&config)),
            im: ImService::new(Arc::clone(&config)),
            drive: DriveService::new(Arc::clone(&config)),
            search: SearchService::new(Arc::clone(&config)),
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
