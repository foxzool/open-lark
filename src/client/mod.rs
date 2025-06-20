use std::time::Duration;

use crate::{
    core::{config::Config, constants::AppType},
    service::{
        attendance::AttendanceService, authentication::AuthenService, bitable::BitableService,
        drive::DriveService, im::ImService, search::SearchService, sheets::SheetsService,
    },
};

#[cfg(feature = "websocket")]
pub mod ws_client;

/// 飞书开放平台SDK client
pub struct LarkClient {
    pub config: Config,
    pub auth: AuthenService,
    pub im: ImService,
    pub drive: DriveService,
    pub search: SearchService,
    pub sheets: SheetsService,
    pub bitable: BitableService,
    pub attendance: AttendanceService,
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
                .unwrap()
        }

        LarkClient {
            config: self.config.clone(),
            auth: AuthenService::new(self.config.clone()),
            im: ImService::new(self.config.clone()),
            drive: DriveService::new(self.config.clone()),
            search: SearchService::new(self.config.clone()),
            sheets: SheetsService::new(self.config.clone()),
            bitable: BitableService::new(self.config.clone()),
            attendance: AttendanceService::new(self.config),
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
