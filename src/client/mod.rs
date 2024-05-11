use std::time::Duration;

use crate::core::config::Config;
use crate::core::constants::AppType;

pub struct LarkClient {
    pub config: Config,
}

impl LarkClient {
    pub fn new(app_id: &str, app_secret: &str) -> Self {
        let mut config = Config {
            app_id: app_id.to_string(),
            app_secret: app_secret.to_string(),
            ..Default::default()
        };

        Self { config }
    }

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
        self.config.req_timeout = timeout.map(|t| Duration::from_secs_f32(t));
        self.config.http_client = reqwest::blocking::Client::builder()
            .timeout(self.config.req_timeout)
            .build()
            .unwrap();
        self
    }
}
